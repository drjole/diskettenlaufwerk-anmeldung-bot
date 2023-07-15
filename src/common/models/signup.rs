use crate::http::request_document;
use crate::models::participant::Participant;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use encoding::{all::ISO_8859_1, Encoding};
use form_urlencoded::byte_serialize;
use reqwest::RequestBuilder;
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Debug, Default, Display, EnumString, sqlx::Type)]
#[sqlx(type_name = "signup_status")]
pub enum SignupStatus {
    #[default]
    Uninformed,
    Notified,
    SignedUp,
    Rejected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignupRequest {
    pub course_id: i64,
    pub answer: bool,
}

const SIGNUP_URL: &str = "https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi";

pub async fn signup(participant: &Participant, course_id: i64) -> Result<()> {
    let client = reqwest::Client::new();
    let form_url = format!("https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi?Kursid={course_id}");

    // Step 1: Get the signup page that contains session specific data
    let request = client.get(&form_url);
    let document = request_document(request).await?;
    let form = parse_form(&document).await?;
    let mut params = params_from_form(form, false);
    let participant_params = participant.as_payload();
    for (key, value) in participant_params {
        params.push((key, value));
    }
    let body = request_body_from_params(params)?;

    // Step 2: Submit the initial form and get the user confirmation page in response
    let mut request = client
        .post(SIGNUP_URL)
        .header("Referer", &form_url)
        .body(body);
    request = add_headers(request);
    let document = request_document(request).await?;
    let form = parse_form(&document).await?;
    let mut params = params_from_form(form, true);
    // Add this parameter to "confirm" the signup
    params.push(("submit".into(), "verbindliche Buchung".into()));
    let body = request_body_from_params(params)?;

    // Step 3: Finalize the signup
    let mut request = client
        .post(SIGNUP_URL)
        .header("Referer", SIGNUP_URL)
        .body(body);
    request = add_headers(request);
    let document = request_document(request).await?;

    dbg!(document);

    Ok(())
}

async fn parse_form(document: &Html) -> Result<ElementRef> {
    let form_selector = scraper::Selector::parse("form").unwrap();
    let form_element = document.select(&form_selector).next().unwrap();
    Ok(form_element)
}

fn add_headers(request: RequestBuilder) -> RequestBuilder {
    request
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Host", "isis.verw.uni-koeln.de")
        .header("Origin", "https://isis.verw.uni-koeln.de")
}

fn params_from_form(form: ElementRef, keep_user_params: bool) -> Vec<(String, String)> {
    let inputs_selector = scraper::Selector::parse("input").unwrap_or_else(|_| panic!());
    let user_params: Vec<String> = vec![
        "Geschlecht".to_string(),
        "Vorname".to_string(),
        "Name".to_string(),
        "Strasse".to_string(),
        "Ort".to_string(),
        "Statusorig".to_string(),
        "Matnr".to_string(),
        "Mail".to_string(),
        "Tel".to_string(),
    ];
    form.select(&inputs_selector)
        .map(|element| {
            (
                element.value().attr("name").unwrap().into(),
                element.value().attr("value").unwrap().into(),
            )
        })
        .filter(|(name, _)| name != "reset")
        .filter(|(name, _)| name != "back")
        .filter(|(name, _)| keep_user_params || !user_params.contains(name))
        .collect()
}

fn request_body_from_params(mut params: Vec<(String, String)>) -> Result<String> {
    encode_params(&mut params)?;
    Ok(params
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<String>>()
        .join("&"))
}

fn encode_params(params: &mut [(String, String)]) -> Result<()> {
    for (_, value) in params.iter_mut() {
        *value = byte_serialize(
            &ISO_8859_1
                .encode(value, encoding::EncoderTrap::Strict)
                .map_err(|e| eyre!("failed to encode to ISO 8859-1: {e}"))?,
        )
        .collect();
    }
    Ok(())
}
