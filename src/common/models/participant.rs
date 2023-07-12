use color_eyre::Result;
use sqlx::{Pool, Postgres};
use strum::EnumProperty;

use crate::models::gender::Gender;
use crate::models::status::Status;

#[derive(Debug, Default)]
pub struct Participant {
    pub chat_id: i64,
    pub given_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<Gender>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<Status>,
    pub status_related_info: Option<String>,
}

impl Participant {
    pub async fn all(pool: &Pool<Postgres>) -> Result<Vec<Self>> {
        let participants = sqlx::query_as!(Participant,
        r#"
        SELECT chat_id, given_name, last_name, gender as "gender: _", street, city, phone, email, status as "status: _", status_related_info
        FROM participants
        "#).fetch_all(pool).await?;
        Ok(participants)
    }

    pub async fn find_by_chat_id(pool: &Pool<Postgres>, chat_id: i64) -> Result<Self> {
        let participant = sqlx::query_as!(Participant,
            r#"
            SELECT chat_id, given_name, last_name, gender as "gender: _", street, city, phone, email, status as "status: _", status_related_info
            FROM participants
            WHERE chat_id = $1
            "#,
            chat_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(participant)
    }

    pub async fn insert(&self, pool: &Pool<Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO participants(chat_id, given_name, last_name, gender, street, city, phone, email, status, status_related_info)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            self.chat_id,
            self.given_name.clone().unwrap_or_default(),
            self.last_name.clone().unwrap_or_default(),
            self.gender.clone() as Option<Gender>,
            self.street.clone().unwrap_or_default(),
            self.city.clone().unwrap_or_default(),
            self.phone.clone().unwrap_or_default(),
            self.email.clone().unwrap_or_default(),
            self.status.clone() as Option<Status>,
            self.status_related_info.clone().unwrap_or_default(),
        )
            .execute(pool)
            .await?;
        Ok(())
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
                status_related_info = $9
            WHERE chat_id = $10
            "#,
            self.given_name.clone().unwrap_or_default(),
            self.last_name.clone().unwrap_or_default(),
            self.gender.clone() as Option<Gender>,
            self.street.clone().unwrap_or_default(),
            self.city.clone().unwrap_or_default(),
            self.phone.clone().unwrap_or_default(),
            self.email.clone().unwrap_or_default(),
            self.status.clone() as Option<Status>,
            self.status_related_info.clone().unwrap_or_default(),
            self.chat_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, pool: &Pool<Postgres>) -> Result<()> {
        sqlx::query!(
            r#"DELETE FROM participants WHERE chat_id = $1"#,
            self.chat_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    fn as_payload(&self) -> Vec<(String, String)> {
        vec![
            (
                "Geschlecht".into(),
                self.gender.clone().map_or(String::new(), |g| g.to_string()),
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
            (
                "Matnr".into(),
                self.status_related_info.clone().unwrap_or_default(),
            ),
            (
                "Institut".into(),
                self.status_related_info.clone().unwrap_or_default(),
            ),
            ("Mail".into(), self.email.clone().unwrap_or_default()),
            ("Tel".into(), self.phone.clone().unwrap_or_default()),
        ]
    }

    pub fn is_student(&self) -> bool {
        self.status
            .clone()
            .map_or(false, |status| status.is_student())
    }

    pub fn is_employed_at_cgn_uni_related_thing(&self) -> bool {
        self.status.clone().map_or(false, |status| {
            status.is_employed_at_cgn_uni_related_thing()
        })
    }

    pub fn status_related_info_name(&self) -> Option<String> {
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
    #[allow(clippy::expect_used)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Vorname: {} (/edit_given_name)
Nachname: {} (/edit_last_name)
Geschlecht: {} (/edit_gender)
Straße: {} (/edit_street)
Ort: {} (/edit_city)
Telefonnummer: {} (/edit_phone)
E-Mail-Adresse: {} (/edit_email)
Status: {} (/edit_status)
{}: {} (/edit_status_related_info)"#,
            self.given_name.clone().unwrap_or_default(),
            self.last_name.clone().unwrap_or_default(),
            self.gender.clone().map_or(String::new(), |g| g
                .get_str("pretty")
                .expect("Better add that enum prop")
                .to_string()),
            self.street.clone().unwrap_or_default(),
            self.city.clone().unwrap_or_default(),
            self.phone.clone().unwrap_or_default(),
            self.email.clone().unwrap_or_default(),
            self.status.clone().map_or(String::new(), |s| s
                .get_str("pretty")
                .expect("Better add that enum prop")
                .to_string()),
            self.status_related_info_name().unwrap_or_default(),
            self.status_related_info.clone().unwrap_or_default()
        )
    }
}
