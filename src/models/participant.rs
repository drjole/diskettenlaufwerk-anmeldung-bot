use color_eyre::eyre::Result;
use sqlx::{Pool, Postgres};

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
    pub matriculation_number: Option<String>,
    pub business_phone: Option<String>,
}

impl Participant {
    pub async fn find_by_chat_id(pool: &Pool<Postgres>, chat_id: i64) -> Result<Participant> {
        let participant = sqlx::query_as!(Participant,
            r#"
            SELECT chat_id, given_name, last_name, gender as "gender: _", street, city, phone, email, status as "status: _", matriculation_number, business_phone
            FROM participants
            WHERE chat_id = $1
            "#,
            chat_id,
        )
        .fetch_one(pool)
        .await?;
        Ok(participant)
    }

    pub async fn insert(self, pool: &Pool<Postgres>) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO participants(chat_id, given_name, last_name, gender, street, city, phone, email, status, matriculation_number, business_phone)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            self.chat_id,
            self.given_name.unwrap_or_default(),
            self.last_name.unwrap_or_default(),
            self.gender as Option<Gender>,
            self.street.unwrap_or_default(),
            self.city.unwrap_or_default(),
            self.phone.unwrap_or_default(),
            self.email.unwrap_or_default(),
            self.status as Option<Status>,
            self.matriculation_number.unwrap_or_default(),
            self.business_phone.unwrap_or_default()
        )
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update(self, pool: &Pool<Postgres>) -> Result<()> {
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
                matriculation_number = $9,
                business_phone = $10
            WHERE chat_id = $11
            "#,
            self.given_name.unwrap_or_default(),
            self.last_name.unwrap_or_default(),
            self.gender as Option<Gender>,
            self.street.unwrap_or_default(),
            self.city.unwrap_or_default(),
            self.phone.unwrap_or_default(),
            self.email.unwrap_or_default(),
            self.status as Option<Status>,
            self.matriculation_number.unwrap_or_default(),
            self.business_phone.unwrap_or_default(),
            self.chat_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(self, pool: &Pool<Postgres>) -> Result<()> {
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
                self.matriculation_number.clone().unwrap_or_default(),
            ),
            (
                "Institut".into(),
                self.business_phone.clone().unwrap_or_default(),
            ),
            ("Mail".into(), self.email.clone().unwrap_or_default()),
            ("Tel".into(), self.phone.clone().unwrap_or_default()),
        ]
    }

    fn status_related_info(&self) -> Option<String> {
        self.status.clone().and_then(|status| {
            if status.is_student() {
                self.matriculation_number.clone()
            } else if status.is_employed_at_cgn_uni_related_thing() {
                self.business_phone.clone()
            } else {
                None
            }
        })
    }
}

impl std::fmt::Display for Participant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{} {}
{}
{}
{}
{}
{}
{}"#,
            self.given_name.clone().unwrap_or_default(),
            self.last_name.clone().unwrap_or_default(),
            self.street.clone().unwrap_or_default(),
            self.city.clone().unwrap_or_default(),
            self.phone.clone().unwrap_or_default(),
            self.email.clone().unwrap_or_default(),
            self.status.clone().map_or(String::new(), |s| s.to_string()),
            self.status_related_info().unwrap_or_default()
        )
    }
}
