use strum::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, Display, EnumString, EnumIter, sqlx::Type)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    Male,
    Female,
    Diverse,
}

impl Gender {
    pub const fn as_payload(&self) -> &'static str {
        match self {
            Self::Male => "M",
            Self::Female => "W",
            Self::Diverse => "D",
        }
    }
}
