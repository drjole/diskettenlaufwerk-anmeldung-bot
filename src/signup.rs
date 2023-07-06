use encoding::{all::ISO_8859_1, Encoding};
use form_urlencoded::byte_serialize;
use reqwest::RequestBuilder;
use scraper::{ElementRef, Html};

use crate::{http::request_document, Error, HandlerResult};

const FORM_URL: &str = "https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi?Kursid=245802";
const SIGNUP_URL: &str = "https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi";

async fn signup() -> HandlerResult {
    let client = reqwest::Client::new();

    // Step 1: Get the signup page that contains session specific data
    let request = client.get(FORM_URL);
    let document = request_document(request).await?;
    let form = parse_form(&document).await?;
    let mut params = params_from_form(form, false);
    // add_user_data(&mut params); TODO: Load participant, add params from 'as_params()'
    let body = request_body_from_params(params)?;

    // Step 2: Submit the initial form and get the user confirmation page in response
    let mut request = client
        .post(SIGNUP_URL)
        .header("Referer", FORM_URL)
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

async fn parse_form(document: &Html) -> Result<ElementRef, Error> {
    let form_selector = scraper::Selector::parse("form").unwrap();
    document
        .select(&form_selector)
        .next()
        .ok_or(String::from("could not find form element").into())
}

fn add_headers(request: RequestBuilder) -> RequestBuilder {
    request
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Host", "isis.verw.uni-koeln.de")
        .header("Origin", "https://isis.verw.uni-koeln.de")
}

fn params_from_form(form: ElementRef, keep_user_params: bool) -> Vec<(String, String)> {
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
                element.value().attr("name").unwrap().into(),
                element.value().attr("value").unwrap().into(),
            )
        })
        .filter(|(name, _)| name != "reset")
        .filter(|(name, _)| name != "back")
        .filter(|(name, _)| keep_user_params || !user_params.contains(name))
        .collect()
}

fn request_body_from_params(mut params: Vec<(String, String)>) -> Result<String, Error> {
    encode_params(&mut params)?;
    Ok(params
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<String>>()
        .join("&"))
}

fn encode_params(params: &mut [(String, String)]) -> HandlerResult {
    for (_, value) in params.iter_mut() {
        *value =
            byte_serialize(&ISO_8859_1.encode(value, encoding::EncoderTrap::Strict)?).collect();
    }
    Ok(())
}
