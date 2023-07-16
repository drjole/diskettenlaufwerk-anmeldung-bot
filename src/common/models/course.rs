use crate::http::request_document;
use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::Europe::Berlin;
use color_eyre::{eyre::eyre, Result};
use sqlx::{Pool, Postgres};
use std::{collections::HashMap, fmt::Display};
use url::Url;

#[derive(Debug)]
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
        let document = request_document(request).await?;

        let table_header_cells_selector = scraper::Selector::parse("thead > tr:first-of-type > th")
            .map_err(|err| eyre!("{err}"))?;
        let table_body_rows_selector =
            scraper::Selector::parse("tbody > tr").map_err(|err| eyre!("{err}"))?;
        let table_cells_selector = scraper::Selector::parse("td").map_err(|err| eyre!("{err}"))?;
        let a_tag_selector = scraper::Selector::parse("a").map_err(|err| eyre!("{err}"))?;

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
            .ok_or_else(|| eyre!("Header 'Anmeldung' is missing"))?;

        for table_row in document.select(&table_body_rows_selector) {
            let table_cells: Result<HashMap<usize, String>> = table_row
                .select(&table_cells_selector)
                .enumerate()
                .map(|(i, e)| {
                    if i == *url_column {
                        let a_tag = e
                            .select(&a_tag_selector)
                            .next()
                            .ok_or_else(|| eyre!("a-tag for course URL is missing"))?;
                        let href = a_tag
                            .value()
                            .attr("href")
                            .ok_or_else(|| eyre!("a-tag for course URL has no href attribute"))?;
                        Ok((i, href.to_string()))
                    } else {
                        Ok((i, e.text().collect()))
                    }
                })
                .collect();
            let table_cells = table_cells?;
            let date = &table_cells[table_headers
                .get("Zeitraum")
                .ok_or_else(|| eyre!("Header 'Zeitraum' is missing"))?];
            let mut date_components = date.split('.');
            let date = format!(
                "{}.{}.20{}",
                date_components
                    .next()
                    .ok_or_else(|| eyre!("Day is missing in date"))?,
                date_components
                    .next()
                    .ok_or_else(|| eyre!("Month is missing in date"))?,
                date_components
                    .next()
                    .ok_or_else(|| eyre!("Year is missing in date"))?
            );
            let time = &table_cells[table_headers
                .get("Zeit")
                .ok_or_else(|| eyre!("Header 'Zeit' is missing"))?];
            let (start_time_of_day, end_time_of_day) = time
                .split_once('-')
                .ok_or_else(|| eyre!("'Zeit' field cannot be split at '-'"))?;

            let url = Url::parse(
                &table_cells[table_headers
                    .get("Anmeldung")
                    .ok_or_else(|| eyre!("Header 'Anmeldung' is missing"))?],
            )?;
            let query_params: HashMap<_, _> = url.query_pairs().into_owned().collect();
            let id_string = query_params
                .get("Kursid")
                .ok_or_else(|| eyre!("'Kursid' missing in URL"))?;
            let id: i64 = id_string.parse()?;
            let start_time = Berlin
                .datetime_from_str(
                    &format!("{date} {start_time_of_day}:00"),
                    "%d.%m.%Y %H:%M:%S",
                )?
                .naive_utc();
            let end_time = Berlin
                .datetime_from_str(&format!("{date} {end_time_of_day}:00"), "%d.%m.%Y %H:%M:%S")?
                .naive_utc();
            let level = table_cells[table_headers
                .get("Bezeichnung")
                .ok_or_else(|| eyre!("Header 'Bezeichnung' is missing"))?]
            .clone();
            let location = table_cells[table_headers
                .get("Ort")
                .ok_or_else(|| eyre!("Header 'Ort' is missing"))?]
            .clone();
            let trainer = table_cells[table_headers
                .get("Kursleiter/In")
                .ok_or_else(|| eyre!("Header 'Kursleiter/In' is missing"))?]
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
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Von: {}
Bis: {}
Bezeichnung: {}
Ort: {}
Kursleiter/In: {}"#,
            self.start_time.format("%H:%M"),
            self.end_time.format("%H:%M"),
            self.level,
            self.location,
            self.trainer
        )
    }
}
