use chrono::NaiveDateTime;
use chrono_tz::Europe;
use color_eyre::{
    eyre::{eyre, OptionExt},
    Result,
};
use sqlx::{Pool, Postgres};
use std::{collections::HashMap, fmt::Display};
use url::Url;

use crate::utils::http::request_document;

use super::signup::parse_form;

#[derive(Debug, Clone)]
pub struct Course {
    pub id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub level: String,
    pub location: String,
    pub trainer: String,
}

impl Course {
    pub async fn create(&self, pool: &Pool<Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO courses (id, start_time, end_time, level, location, trainer)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            self.id,
            self.start_time,
            self.end_time,
            self.level,
            self.location,
            self.trainer
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_id(pool: &Pool<Postgres>, id: i64) -> Result<Option<Self>> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT id, start_time, end_time, level, location, trainer
            FROM courses
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(course)
    }

    pub async fn today(pool: &Pool<Postgres>) -> Result<Option<Self>> {
        let course = sqlx::query_as!(
            Course,
            r#"
            SELECT id, start_time, end_time, level, location, trainer
            FROM courses
            WHERE date(start_time) = current_date
            "#
        )
        .fetch_optional(pool)
        .await?;
        Ok(course)
    }

    pub async fn fetch(pool: &Pool<Postgres>) -> Result<()> {
        log::info!("fetching courses");
        let courses = Self::download().await?;
        if courses.is_empty() {
            log::info!("no courses found");
            return Ok(());
        }
        for course in &courses {
            if course.exists(pool).await? {
                log::info!("course {} already exists", course.id);
            } else {
                log::info!("inserting new course {}", course.id);
                course.create(pool).await?;
            }
        }
        Ok(())
    }

    async fn exists(&self, pool: &Pool<Postgres>) -> Result<bool> {
        let record = sqlx::query!(r#"SELECT id FROM courses WHERE id = $1 LIMIT 1"#, self.id)
            .fetch_optional(pool)
            .await?;
        Ok(record.is_some())
    }

    async fn download() -> Result<Vec<Self>> {
        const COURSES_URL: &str =
            "https://unisport.koeln/e65/e41657/e41692/k_content41702/publicGetData";
        let client = reqwest::Client::new();
        let request = client.get(COURSES_URL);
        let response = request_document(request).await?;
        let document = scraper::Html::parse_document(response.as_str());

        let table_header_cells_selector = scraper::Selector::parse("thead > tr:first-of-type > th")
            .map_err(|e| eyre!("scraper error: {e}"))?;
        let table_body_rows_selector =
            scraper::Selector::parse("tbody > tr").map_err(|e| eyre!("scraper error: {e}"))?;
        let table_cells_selector =
            scraper::Selector::parse("td").map_err(|e| eyre!("scraper error: {e}"))?;
        let a_tag_selector =
            scraper::Selector::parse("a").map_err(|e| eyre!("scraper error: {e}"))?;

        let table_headers: HashMap<String, usize> = document
            .select(&table_header_cells_selector)
            .enumerate()
            .map(|(i, e)| (e.text().collect(), i))
            .collect();

        if table_headers.is_empty() {
            return Ok(vec![]);
        }

        let mut courses = vec![];

        let url_column = table_headers
            .get("Anmeldung")
            .ok_or_else(|| eyre!("no header 'Anmeldung'"))?;

        for table_row in document.select(&table_body_rows_selector) {
            let table_cells = table_row
                .select(&table_cells_selector)
                .enumerate()
                .map(|(i, e)| {
                    if i == *url_column {
                        let a_tag = e
                            .select(&a_tag_selector)
                            .next()
                            .ok_or_else(|| eyre!("no a tag found in table row"))?;
                        let href = a_tag
                            .value()
                            .attr("href")
                            .ok_or_else(|| eyre!("no href attribute on a tag"))?;
                        Ok((i, href.to_string()))
                    } else {
                        Ok((i, e.text().collect()))
                    }
                })
                .collect::<Result<HashMap<usize, String>>>()?;
            let date = &table_cells[table_headers
                .get("Zeitraum")
                .ok_or_else(|| eyre!("no header 'Zeitraum'"))?];
            let mut date_components = date.split('.');
            let date = format!(
                "{}.{}.20{}",
                date_components
                    .next()
                    .ok_or_else(|| eyre!("no first value for date component"))?,
                date_components
                    .next()
                    .ok_or_else(|| eyre!("no second value for date component"))?,
                date_components
                    .next()
                    .ok_or_else(|| eyre!("no third value for date component"))?,
            );
            let date: String = date.chars().take(10).collect();
            let time = &table_cells[table_headers
                .get("Zeit")
                .ok_or_else(|| eyre!("no header 'Zeit'"))?];
            let (start_time_of_day, end_time_of_day) = time
                .split_once('-')
                .ok_or_else(|| eyre!("could not split time at '-'"))?;

            let url = Url::parse(
                &table_cells[table_headers
                    .get("Anmeldung")
                    .ok_or_else(|| eyre!("no header 'Anmeldung'"))?],
            )?;
            if url.path() == "/buchsys/meldungen/keine_anmeldung_kurs.html" {
                continue;
            }
            let query_params: HashMap<_, _> = url.query_pairs().into_owned().collect();
            let id_string = query_params
                .get("Kursid")
                .ok_or_else(|| eyre!("no query param 'Kursid'"))?;
            let id: i64 = id_string.parse()?;
            let start_time = NaiveDateTime::parse_from_str(
                &format!("{date} {start_time_of_day}:00"),
                "%d.%m.%Y %H:%M:%S",
            )?
            .and_local_timezone(Europe::Berlin)
            .single()
            .ok_or_eyre("could not convert to local timezone")?
            .naive_utc();
            let end_time = NaiveDateTime::parse_from_str(
                &format!("{date} {end_time_of_day}:00"),
                "%d.%m.%Y %H:%M:%S",
            )?
            .and_local_timezone(Europe::Berlin)
            .single()
            .ok_or_eyre("could not convert to local timezone")?
            .naive_utc();
            let level = table_cells[table_headers
                .get("Bezeichnung")
                .ok_or_else(|| eyre!("no header 'Bezeichnung'"))?]
            .clone();
            let location = table_cells[table_headers
                .get("Ort")
                .ok_or_else(|| eyre!("no header 'Ort'"))?]
            .clone();
            let trainer = table_cells[table_headers
                .get("Kursleiter/In")
                .ok_or_else(|| eyre!("no header 'Kursleiter/In'"))?]
            .clone();

            let course = Self {
                id,
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

    pub async fn is_signup_available(&self) -> bool {
        let client = reqwest::Client::new();
        let form_url = format!(
            "https://isis.verw.uni-koeln.de/cgi/anmeldung.fcgi?Kursid={}",
            self.id
        );
        let request = client.get(&form_url);
        let Ok(response) = request_document(request).await else {
            return false;
        };
        let document = scraper::Html::parse_document(response.as_str());
        parse_form(&document).is_ok()
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Von: {}
Bis: {}
Bezeichnung: {}
Ort: {}
Kursleiter/In: {}",
            self.start_time
                .and_utc()
                .with_timezone(&Europe::Berlin)
                .format("%H:%M"),
            self.end_time
                .and_utc()
                .with_timezone(&Europe::Berlin)
                .format("%H:%M"),
            self.level,
            self.location,
            self.trainer
        )
    }
}
