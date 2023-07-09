use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug)]
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
    fn as_params(&self) -> Vec<(String, String)> {
        vec![
            (
                "Geschlecht".into(),
                self.gender
                    .clone()
                    .map_or(String::from(""), |g| g.to_string()),
            ),
            (
                "Vorname".into(),
                self.given_name.clone().unwrap_or(String::from("")),
            ),
            (
                "Name".into(),
                self.last_name.clone().unwrap_or(String::from("")),
            ),
            (
                "Strasse".into(),
                self.street.clone().unwrap_or(String::from("")),
            ),
            ("Ort".into(), self.city.clone().unwrap_or(String::from(""))),
            (
                "Statusorig".into(),
                self.status
                    .clone()
                    .map_or(String::from(""), |s| s.as_str().to_string()),
            ),
            (
                "Matnr".into(),
                self.matriculation_number
                    .clone()
                    .unwrap_or(String::from("")),
            ),
            (
                "Institut".into(),
                self.business_phone.clone().unwrap_or(String::from("")),
            ),
            (
                "Mail".into(),
                self.email.clone().unwrap_or(String::from("")),
            ),
            ("Tel".into(), self.phone.clone().unwrap_or(String::from(""))),
        ]
    }
}

impl std::fmt::Display for Participant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"{} {}
{}
{}
{}"#,
            self.given_name.clone().unwrap_or_default(),
            self.last_name.clone().unwrap_or_default(),
            self.street.clone().unwrap_or_default(),
            self.city.clone().unwrap_or_default(),
            self.status.clone().map_or("", |s| s.to_string())
        )
    }
}

#[derive(Clone, Debug, EnumIter, sqlx::Type)]
#[sqlx(type_name = "gender")]
pub enum Gender {
    Male,
    Female,
    Diverse,
}

impl FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" => Ok(Gender::Male),
            "W" => Ok(Gender::Female),
            "D" => Ok(Gender::Diverse),
            _ => Err("illegal gender".into()),
        }
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "M"),
            Gender::Female => write!(f, "W"),
            Gender::Diverse => write!(f, "D"),
        }
    }
}

#[derive(Clone, Debug, EnumIter, sqlx::Type)]
#[sqlx(type_name = "status")]
pub enum Status {
    StudentUniKoeln,
    StudentDSHSKoeln,
    StudentTHKoeln,
    StudentMacromediaKoeln,
    StudentKunsthochschuleFuerMedien,
    StudentHochschuleFuerMedienKommunikationUndWirtschaft,
    StudentHochschuleFuerMusikKoeln,
    StudentAndereHochschulen,
    BeschaeftigteStaatlicherKoelnerHochschulen,
    BeschaeftigteUniKlinikKoeln,
    BeschaeftigteKoelnerStudierendenwerk,
    MitgliedKoelnAlumni,
    AzubiUniKoeln,
    AzubiUniKlinik,
    AzubiKoelnerStudierendenwerk,
    Gast,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::StudentUniKoeln => "S-UNI",
            Status::StudentDSHSKoeln => "S-DSHS",
            Status::StudentTHKoeln => "S-TH",
            Status::StudentMacromediaKoeln => "S-MAC",
            Status::StudentKunsthochschuleFuerMedien => "S-KHSM",
            Status::StudentHochschuleFuerMedienKommunikationUndWirtschaft => "S-HMKW",
            Status::StudentHochschuleFuerMusikKoeln => "S-MH",
            Status::StudentAndereHochschulen => "S-aH",
            Status::BeschaeftigteStaatlicherKoelnerHochschulen => "B-SFH",
            Status::BeschaeftigteUniKlinikKoeln => "B-UK",
            Status::BeschaeftigteKoelnerStudierendenwerk => "B-KStW",
            Status::MitgliedKoelnAlumni => "Alumni",
            Status::AzubiUniKoeln => "A-Uni",
            Status::AzubiUniKlinik => "A-UK",
            Status::AzubiKoelnerStudierendenwerk => "A-KSTW",
            Status::Gast => "Extern",
        }
    }

    pub fn is_student(&self) -> bool {
        matches!(
            self,
            Status::StudentUniKoeln
                | Status::StudentDSHSKoeln
                | Status::StudentTHKoeln
                | Status::StudentMacromediaKoeln
                | Status::StudentKunsthochschuleFuerMedien
                | Status::StudentHochschuleFuerMedienKommunikationUndWirtschaft
                | Status::StudentHochschuleFuerMusikKoeln
                | Status::StudentAndereHochschulen
        )
    }

    pub fn is_employed_at_cgn_uni_related_thing(&self) -> bool {
        matches!(
            self,
            Status::BeschaeftigteStaatlicherKoelnerHochschulen
                | Status::BeschaeftigteUniKlinikKoeln
                | Status::BeschaeftigteKoelnerStudierendenwerk
        )
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S-UNI" => Ok(Status::StudentUniKoeln),
            "S-DSHS" => Ok(Status::StudentDSHSKoeln),
            "S-TH" => Ok(Status::StudentTHKoeln),
            "S-MAC" => Ok(Status::StudentMacromediaKoeln),
            "S-KHSM" => Ok(Status::StudentKunsthochschuleFuerMedien),
            "S-HMKW" => Ok(Status::StudentHochschuleFuerMedienKommunikationUndWirtschaft),
            "S-MH" => Ok(Status::StudentHochschuleFuerMusikKoeln),
            "S-aH" => Ok(Status::StudentAndereHochschulen),
            "B-SFH" => Ok(Status::BeschaeftigteStaatlicherKoelnerHochschulen),
            "B-UK" => Ok(Status::BeschaeftigteUniKlinikKoeln),
            "B-KStW" => Ok(Status::BeschaeftigteKoelnerStudierendenwerk),
            "Alumni" => Ok(Status::MitgliedKoelnAlumni),
            "A-Uni" => Ok(Status::AzubiUniKoeln),
            "A-UK" => Ok(Status::AzubiUniKlinik),
            "A-KSTW" => Ok(Status::AzubiKoelnerStudierendenwerk),
            "Extern" => Ok(Status::Gast),
            _ => Err("illegal status".into()),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::StudentUniKoeln => write!(f, "Stud. Uni Köln"),
            Status::StudentDSHSKoeln => write!(f, "Stud. DSHS Köln"),
            Status::StudentTHKoeln => write!(f, "Stud. TH Köln"),
            Status::StudentMacromediaKoeln => write!(f, "Stud. Macromedia Köln"),
            Status::StudentKunsthochschuleFuerMedien => {
                write!(f, "Stud. Kunsthochschule für Medien")
            }
            Status::StudentHochschuleFuerMedienKommunikationUndWirtschaft => write!(
                f,
                "Stud. Hochschule für Medien, Kommunikation und Wirtschaft"
            ),
            Status::StudentHochschuleFuerMusikKoeln => write!(f, "Stud. Hochschule für Musik Köln"),
            Status::StudentAndereHochschulen => write!(f, "Stud. anderer Hochschulen"),
            Status::BeschaeftigteStaatlicherKoelnerHochschulen => {
                write!(f, "Beschäft. staatl. Kölner Hochschulen")
            }
            Status::BeschaeftigteUniKlinikKoeln => write!(f, "Beschäft. UniKlinik Köln"),
            Status::BeschaeftigteKoelnerStudierendenwerk => {
                write!(f, "Beschäft. KölnerStudierendenwerk")
            }
            Status::MitgliedKoelnAlumni => write!(f, "Mitglied von KölnAlumni"),
            Status::AzubiUniKoeln => write!(f, "Azubi Uni Köln"),
            Status::AzubiUniKlinik => write!(f, "Azubi UniKlinik"),
            Status::AzubiKoelnerStudierendenwerk => write!(f, "Azubi KölnerStudierendenwerk"),
            Status::Gast => write!(f, "Gast"),
        }
    }
}
