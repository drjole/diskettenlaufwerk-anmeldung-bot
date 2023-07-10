use crate::{http::request_document, Error};
use chrono::TimeZone;
use chrono_tz::Europe::Berlin;
use std::collections::HashMap;
use url::Url;

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

    let mut courses = vec![];

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
        let mut date_components = date.split('.');
        let date = format!(
            "{}.{}.20{}",
            date_components.next().unwrap(),
            date_components.next().unwrap(),
            date_components.next().unwrap()
        );
        let time = &table_cells[table_headers.get("Zeit").unwrap()];
        let (start_time_of_day, end_time_of_day) = time.split_once('-').unwrap();

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
        courses.push(course);
    }
    Ok(courses)
}
