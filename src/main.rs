extern crate encoding;

use chrono_tz::{Europe::Berlin, Tz};
use std::collections::HashMap;

use chrono::prelude::*;
use encoding::{all::ISO_8859_1, Encoding};
use form_urlencoded::byte_serialize;
use reqwest::RequestBuilder;
use scraper::{ElementRef, Html};
use url::Url;

type Error = Box<dyn std::error::Error>;

const FORM_URL: &str = "https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi?Kursid=245802";
const SIGNUP_URL: &str = "https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi";

#[tokio::main]
async fn main() -> Result<(), Error> {
    // signup().await?;
    load_courses().await?;
    Ok(())
}

#[derive(Debug)]
struct Course {
    url: Url,
    start_time: DateTime<Tz>,
    end_time: DateTime<Tz>,
    level: String,
    location: String,
    trainer: String,
}

impl Course {
    fn open(&self) -> bool {
        self.url.path() == "/cgi/anmeldung.fcgi"
    }
}

async fn load_courses() -> Result<Vec<Course>, Error> {
    const COURSES_URL: &str =
        "https://unisport.koeln/e65/e41657/e41692/k_content41702/publicGetData";
    let client = reqwest::Client::new();
    let request = client.get(COURSES_URL);
    let document = request_document(request).await?;

    let table_header_cells_selector =
        scraper::Selector::parse("thead > tr:first-of-type > th").unwrap();
    let table_body_rows_selector = scraper::Selector::parse("tbody > tr").unwrap();
    let table_cells_selector = scraper::Selector::parse("td").unwrap();
    let a_tag_selector = scraper::Selector::parse("a").unwrap();

    let table_headers: HashMap<String, usize> = document
        .select(&table_header_cells_selector)
        .enumerate()
        .map(|(i, e)| (e.text().collect(), i))
        .collect();

    let url_column = table_headers.get("Anmeldung").unwrap();

    for table_row in document.select(&table_body_rows_selector) {
        let table_cells: HashMap<usize, String> = table_row
            .select(&table_cells_selector)
            .enumerate()
            .map(|(i, e)| {
                if i == *url_column {
                    (
                        i,
                        e.select(&a_tag_selector)
                            .next()
                            .unwrap()
                            .value()
                            .attr("href")
                            .unwrap()
                            .to_string(),
                    )
                } else {
                    (i, e.text().collect())
                }
            })
            .collect();
        let date = &table_cells[table_headers.get("Zeitraum").unwrap()];
        let mut date_components = date.split(".");
        let date = format!(
            "{}.{}.20{}",
            date_components.nth(0).unwrap(),
            date_components.nth(0).unwrap(),
            date_components.nth(0).unwrap()
        );
        let time = &table_cells[table_headers.get("Zeit").unwrap()];
        let (start_time_of_day, end_time_of_day) = time.split_once("-").unwrap();

        let url = Url::parse(&table_cells[table_headers.get("Anmeldung").unwrap()])?;
        let start_time = Berlin
            .datetime_from_str(
                &format!("{} {}:00", date, start_time_of_day),
                "%d.%m.%Y %H:%M:%S",
            )
            .unwrap();
        let end_time = Berlin
            .datetime_from_str(
                &format!("{} {}:00", date, end_time_of_day),
                "%d.%m.%Y %H:%M:%S",
            )
            .unwrap();
        let level = table_cells[table_headers.get("Bezeichnung").unwrap()].clone();
        let location = table_cells[table_headers.get("Ort").unwrap()].clone();
        let trainer = table_cells[table_headers.get("Kursleiter/In").unwrap()].clone();

        let course = Course {
            url,
            start_time,
            end_time,
            level,
            location,
            trainer,
        };

        dbg!(course);
    }

    let courses = vec![];
    Ok(courses)
}

async fn signup() -> Result<(), Error> {
    let client = reqwest::Client::new();

    // Step 1: Get the signup page that contains session specific data
    let request = client.get(FORM_URL);
    let document = request_document(request).await?;
    let form = parse_form(&document).await?;
    let mut params = params_from_form(form, false);
    add_user_data(&mut params);
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

async fn request_document(builder: RequestBuilder) -> Result<Html, Error> {
    let response = builder.send().await?;
    std::thread::sleep(std::time::Duration::from_secs(3));
    let text = response.text().await?;
    let document = scraper::Html::parse_document(&text);
    Ok(document)
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

fn add_user_data(params: &mut Vec<(String, String)>) {
    params.push(("Geschlecht".into(), "M".into()));
    params.push(("Vorname".into(), "Jonas".into()));
    params.push(("Name".into(), "Weber".into()));
    params.push(("Strasse".into(), "Eichstr. 60".into()));
    params.push(("Ort".into(), "50733 Köln".into()));
    params.push(("Statusorig".into(), "S-TH".into()));
    params.push(("Matnr".into(), "11134601".into()));
    params.push(("Mail".into(), "jonaslevinweber+uni@gmail.com".into()));
    params.push(("Tel".into(), "01719692309".into()));
}

fn request_body_from_params(mut params: Vec<(String, String)>) -> Result<String, Error> {
    encode_params(&mut params)?;
    Ok(params
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<String>>()
        .join("&"))
}

fn encode_params(params: &mut [(String, String)]) -> Result<(), Error> {
    for (_, value) in params.iter_mut() {
        *value =
            byte_serialize(&ISO_8859_1.encode(value, encoding::EncoderTrap::Strict)?).collect();
    }
    Ok(())
}
