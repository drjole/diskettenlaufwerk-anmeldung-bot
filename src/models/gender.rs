use strum::{EnumIter, EnumProperty, EnumString};

#[derive(Debug, Clone, EnumString, EnumProperty, EnumIter, sqlx::Type)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    #[strum(props(pretty = "Männlich"))]
    Male,
    #[strum(props(pretty = "Weiblich"))]
    Female,
    #[strum(props(pretty = "Divers"))]
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
