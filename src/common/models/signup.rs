use crate::{http::request_document, models::participant::Participant};
use anyhow::{anyhow, Result};
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
    let response = request_document(request).await?;
    // We need a scope here... https://github.com/causal-agent/scraper/issues/75#issuecomment-1076997293
    let body = {
        let document = scraper::Html::parse_document(response.as_str());
        let form = parse_form(&document);
        let mut params = params_from_form(form, false);
        let participant_params = participant.as_payload();
        for (key, value) in participant_params {
            params.push((key, value));
        }
        request_body_from_params(params)?
    };

    // Step 2: Submit the initial form and get the user confirmation page in response
    let mut request = client
        .post(SIGNUP_URL)
        .header("Referer", &form_url)
        .body(body);
    request = add_headers(request);
    let response = request_document(request).await?;
    // We need a scope here... https://github.com/causal-agent/scraper/issues/75#issuecomment-1076997293
    let body = {
        let document = scraper::Html::parse_document(response.as_str());
        let form = parse_form(&document);
        let mut params = params_from_form(form, true);
        // Add this parameter to "confirm" the signup
        params.push(("submit".into(), "verbindliche Buchung".into()));
        request_body_from_params(params)?
    };

    // Step 3: Finalize the signup
    dbg!(&body);
    let mut request = client
        .post(SIGNUP_URL)
        .header("Referer", SIGNUP_URL)
        .body(body);
    request = add_headers(request);
    let response = request_document(request).await?;
    dbg!(response);

    Ok(())
}

fn parse_form(document: &Html) -> ElementRef {
    let form_selector = scraper::Selector::parse("form").unwrap();
    let form_element = document.select(&form_selector).next().unwrap();
    form_element
}

fn add_headers(request: RequestBuilder) -> RequestBuilder {
    request
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Host", "isis.verw.uni-koeln.de")
        .header("Origin", "https://isis.verw.uni-koeln.de")
}

fn params_from_form(form: ElementRef<'_>, keep_user_params: bool) -> Vec<(String, String)> {
    let inputs_selector = scraper::Selector::parse("input").unwrap();
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
                element.value().attr("name").unwrap().to_string(),
                element.value().attr("value").unwrap().to_string(),
            )
        })
        .filter(|(name, _)| name != "reset")
        .filter(|(name, _)| name != "back")
        .filter(|(name, _)| keep_user_params || !user_params.contains(name))
        .collect::<Vec<(String, String)>>()
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
                .map_err(|e| anyhow!("failed to encode to ISO 8859-1: {e}"))?,
        )
        .collect();
    }
    Ok(())
}
