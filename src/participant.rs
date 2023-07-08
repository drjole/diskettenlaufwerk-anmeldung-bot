use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Participant {
    pub given_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub street: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub status: Status,
    pub matriculation_number: String,
    pub business_phone: String,
}

impl Participant {
    fn as_params(&self) -> Vec<(String, String)> {
        vec![
            ("Geschlecht".into(), self.gender.to_string()),
            ("Vorname".into(), self.given_name.clone()),
            ("Name".into(), self.last_name.clone()),
            ("Strasse".into(), self.street.clone()),
            ("Ort".into(), self.city.clone()),
            ("Statusorig".into(), self.status.as_str().to_string()),
            ("Matnr".into(), self.matriculation_number.clone()),
            ("Institut".into(), self.business_phone.clone()),
            ("Mail".into(), self.email.clone()),
            ("Tel".into(), self.phone.clone()),
        ]
    }
}

#[derive(Clone, Debug, EnumIter, sqlx::Type)]
#[sqlx(type_name = "gender")]
#[sqlx(rename_all = "lowercase")]
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
#[sqlx(rename_all = "lowercase")]
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
