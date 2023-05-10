extern crate encoding;

use encoding::{all::ISO_8859_1, Encoding};
use form_urlencoded::byte_serialize;

const FORM_URL: &str = "https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi?Kursid=245617";
const SIGNUP_URL: &str = "https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let form_selector = scraper::Selector::parse("form").unwrap();
    let inputs_selector = scraper::Selector::parse("input").unwrap();

    // Step 1: Get the signup form page that contains some session specific data
    let form_page_response = client.get(FORM_URL).send().await?;
    std::thread::sleep(std::time::Duration::from_secs(3));
    let form_page = form_page_response.text().await?;

    // Store the response text on disk
    std::fs::write("form.html", &form_page)?;

    // Parse the form
    let form_page_document = scraper::Html::parse_document(&form_page);
    let form_element = form_page_document
        .select(&form_selector)
        .next()
        .ok_or(String::from("could not find form element"))?;

    // Create a map of parameters
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
    let mut params: Vec<(String, String)> = form_element
        .select(&inputs_selector)
        .map(|element| {
            (
                element.value().attr("name").unwrap().into(),
                element.value().attr("value").unwrap().into(),
            )
        })
        .filter(|(name, _)| name != "reset")
        .filter(|(name, _)| !user_params.contains(name))
        .collect();

    // This is the user's data
    params.push(("Geschlecht".into(), "M".into()));
    params.push(("Vorname".into(), "Jonas".into()));
    params.push(("Name".into(), "Weber".into()));
    params.push(("Strasse".into(), "Eichstr. 60".into()));
    params.push(("Ort".into(), "50733 Köln".into()));
    params.push(("Statusorig".into(), "S-TH".into()));
    params.push(("Matnr".into(), "11134601".into()));
    params.push(("Mail".into(), "jonaslevinweber+uni@gmail.com".into()));
    params.push(("Tel".into(), "01719692309".into()));

    // We have to encode the form data to ISO-8859-1 because why not
    for (_, value) in params.iter_mut() {
        *value =
            byte_serialize(&ISO_8859_1.encode(value, encoding::EncoderTrap::Strict)?).collect();
    }

    // Create the form body by concatenating the params with &
    let body = params
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<String>>()
        .join("&");

    // Step 2: Get the user confirmation page
    let request = client
        .post(SIGNUP_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Host", "isis.verw.uni-koeln.de")
        .header("Origin", "https://isis.verw.uni-koeln.de")
        .header("Referer", FORM_URL)
        .body(body);
    let user_confirmation_page_response = request.send().await?;
    std::thread::sleep(std::time::Duration::from_secs(3));
    let user_confirmation_page = user_confirmation_page_response.text().await?;

    // Store the response text on disk
    std::fs::write("user_confirmation.html", &user_confirmation_page)?;

    // Parse the form
    let user_confirmation_page_document = scraper::Html::parse_document(&user_confirmation_page);
    let form_element = user_confirmation_page_document
        .select(&form_selector)
        .next()
        .ok_or(String::from("could not find form element"))?;

    // Create a map of parameters
    let mut params: Vec<(String, String)> = form_element
        .select(&inputs_selector)
        .map(|element| {
            (
                element.value().attr("name").unwrap().into(),
                element.value().attr("value").unwrap().into(),
            )
        })
        .filter(|(name, _)| name != "reset")
        .filter(|(name, _)| name != "back")
        .collect();

    // We have to encode the form data to ISO-8859-1 because why not
    for (_, value) in params.iter_mut() {
        *value =
            byte_serialize(&ISO_8859_1.encode(value, encoding::EncoderTrap::Strict)?).collect();
    }

    // We need to set the submit parameter
    params.push(("submit".into(), "verbindliche Buchung".into()));

    // Create the form body by concatenating the params with &
    let body = params
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<String>>()
        .join("&");

    // Step 3: Finalize the signup
    let request = client
        .post(SIGNUP_URL)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Host", "isis.verw.uni-koeln.de")
        .header("Origin", "https://isis.verw.uni-koeln.de")
        .header("Referer", SIGNUP_URL)
        .body(body);
    let confirmation_response = request.send().await?;
    let confirmation_page = confirmation_response.text().await?;

    // Store the response text on disk
    std::fs::write("confirmation.html", confirmation_page)?;

    Ok(())
}
