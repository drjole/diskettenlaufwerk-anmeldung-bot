use crate::models::{
    gender::Gender,
    signup::{self, Signup},
    status::Status,
};
use color_eyre::Result;
use sqlx::{Pool, Postgres};
use strum::EnumProperty;

#[derive(Debug, Default)]
pub struct Participant {
    pub id: i64,
    pub given_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<Gender>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<Status>,
    pub status_info: Option<String>,
    pub signup_always: bool,
}

impl Participant {
    pub async fn create(&self, pool: &Pool<Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO participants(id, given_name, last_name, gender, street, city, phone, email, status, status_info)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            self.id,
            self.given_name,
            self.last_name,
            self.gender.clone() as Option<Gender>,
            self.street,
            self.city,
            self.phone,
            self.email,
            self.status.clone() as Option<Status>,
            self.status_info,
        )
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn find_by_id(pool: &Pool<Postgres>, id: i64) -> Result<Self> {
        let participant = sqlx::query_as!(
            Participant,
            r#"
            SELECT id, given_name, last_name, gender as "gender: _", street, city, phone, email, status as "status: _", status_info, signup_always
            FROM participants
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(pool)
        .await?;
        Ok(participant)
    }

    pub async fn uninformed(pool: &Pool<Postgres>, course_id: i64) -> Result<Vec<Self>> {
        let participants = sqlx::query_as!(
            Participant,
            r#"
            SELECT id, given_name, last_name, gender as "gender: _", street, city, phone, email, participants.status as "status: _", status_info, signup_always
            FROM participants
            WHERE NOT EXISTS (
                SELECT 1
                FROM signups
                WHERE participants.id = signups.participant_id AND signups.course_id = $1
            )
            "#,
            course_id,
        ).fetch_all(pool).await?;
        Ok(participants)
    }

    pub async fn signup(&self, pool: &Pool<Postgres>, course_id: i64) -> Result<Option<Signup>> {
        let signup = sqlx::query_as!(
            Signup,
            r#"
            SELECT participant_id, course_id, status as "status: _"
            FROM signups
            WHERE participant_id = $1 AND course_id = $2
            "#,
            self.id,
            course_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(signup)
    }

    pub async fn update(&self, pool: &Pool<Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE participants
            SET given_name = $1,
                last_name = $2,
                gender = $3,
                street = $4,
                city = $5,
                phone = $6,
                email = $7,
                status = $8,
                status_info = $9
            WHERE id = $10
            "#,
            self.given_name,
            self.last_name,
            self.gender.clone() as Option<Gender>,
            self.street,
            self.city,
            self.phone,
            self.email,
            self.status.clone() as Option<Status>,
            self.status_info,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn set_signup_status(
        &self,
        pool: &Pool<Postgres>,
        course_id: i64,
        status: signup::Status,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO signups(participant_id, course_id, status)
            VALUES ($1, $2, $3)
            ON CONFLICT (participant_id, course_id)
            DO
                UPDATE SET status = $3
            "#,
            self.id,
            course_id,
            status as signup::Status,
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&mut self, pool: &Pool<Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM participants WHERE id = $1
            "#,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub fn as_payload(&self) -> Vec<(String, String)> {
        vec![
            (
                "Geschlecht".into(),
                self.gender
                    .clone()
                    .map_or(String::new(), |g| g.as_payload().to_string()),
            ),
            (
                "Vorname".into(),
                self.given_name.clone().unwrap_or_default(),
            ),
            ("Name".into(), self.last_name.clone().unwrap_or_default()),
            ("Strasse".into(), self.street.clone().unwrap_or_default()),
            ("Ort".into(), self.city.clone().unwrap_or_default()),
            (
                "Statusorig".into(),
                self.status
                    .clone()
                    .map_or(String::new(), |s| s.as_payload().to_string()),
            ),
            ("Matnr".into(), self.status_info.clone().unwrap_or_default()),
            (
                "Institut".into(),
                self.status_info.clone().unwrap_or_default(),
            ),
            ("Mail".into(), self.email.clone().unwrap_or_default()),
            ("Tel".into(), self.phone.clone().unwrap_or_default()),
        ]
    }

    pub fn is_student(&self) -> bool {
        self.status
            .clone()
            .is_some_and(|status| status.is_student())
    }

    pub fn is_employed_at_cgn_uni_related_thing(&self) -> bool {
        self.status
            .clone()
            .is_some_and(|status| status.is_employed_at_cgn_uni_related_thing())
    }

    pub fn status_info_name(&self) -> Option<String> {
        self.status.clone().and_then(|status| {
            if status.is_student() {
                Some(String::from("Matrikelnummer"))
            } else if status.is_employed_at_cgn_uni_related_thing() {
                Some(String::from("Dienstliche Telefonnummer"))
            } else {
                None
            }
        })
    }
}

impl std::fmt::Display for Participant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_info = match &self.status {
            Some(Status::Gast) | None => String::new(),
            Some(_) => format!(
                "\n{}: {} (/edit_status_info)",
                self.status_info_name().unwrap_or_default(),
                self.status_info
                    .as_ref()
                    .map_or("<i>leer</i>", String::as_str)
            ),
        };

        write!(
            f,
            "Vorname: {} (/edit_given_name)
Nachname: {} (/edit_last_name)
Geschlecht: {} (/edit_gender)
Straße: {} (/edit_street)
Ort: {} (/edit_city)
Telefonnummer: {} (/edit_phone)
E-Mail-Adresse: {} (/edit_email)
Status: {} (/edit_status){}",
            self.given_name
                .as_ref()
                .map_or("<i>leer</i>", String::as_str),
            self.last_name
                .as_ref()
                .map_or("<i>leer</i>", String::as_str),
            self.gender.as_ref().map_or("<i>leer</i>", |g| g
                .get_str("pretty")
                .unwrap_or("Better set that enum prop")),
            self.street.as_ref().map_or("<i>leer</i>", String::as_str),
            self.city.as_ref().map_or("<i>leer</i>", String::as_str),
            self.phone.as_ref().map_or("<i>leer</i>", String::as_str),
            self.email.as_ref().map_or("<i>leer</i>", String::as_str),
            self.status.as_ref().map_or("<i>leer</i>", |s| s
                .get_str("pretty")
                .unwrap_or("Better set that enum prop")),
            status_info,
        )
    }
}
