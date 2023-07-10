use strum::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, Display, EnumString, EnumIter, sqlx::Type)]
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
    pub const fn as_payload(&self) -> &'static str {
        match self {
            Self::StudentUniKoeln => "S-UNI",
            Self::StudentDSHSKoeln => "S-DSHS",
            Self::StudentTHKoeln => "S-TH",
            Self::StudentMacromediaKoeln => "S-MAC",
            Self::StudentKunsthochschuleFuerMedien => "S-KHSM",
            Self::StudentHochschuleFuerMedienKommunikationUndWirtschaft => "S-HMKW",
            Self::StudentHochschuleFuerMusikKoeln => "S-MH",
            Self::StudentAndereHochschulen => "S-aH",
            Self::BeschaeftigteStaatlicherKoelnerHochschulen => "B-SFH",
            Self::BeschaeftigteUniKlinikKoeln => "B-UK",
            Self::BeschaeftigteKoelnerStudierendenwerk => "B-KStW",
            Self::MitgliedKoelnAlumni => "Alumni",
            Self::AzubiUniKoeln => "A-Uni",
            Self::AzubiUniKlinik => "A-UK",
            Self::AzubiKoelnerStudierendenwerk => "A-KSTW",
            Self::Gast => "Extern",
        }
    }

    pub const fn is_student(&self) -> bool {
        matches!(
            self,
            Self::StudentUniKoeln
                | Self::StudentDSHSKoeln
                | Self::StudentTHKoeln
                | Self::StudentMacromediaKoeln
                | Self::StudentKunsthochschuleFuerMedien
                | Self::StudentHochschuleFuerMedienKommunikationUndWirtschaft
                | Self::StudentHochschuleFuerMusikKoeln
                | Self::StudentAndereHochschulen
        )
    }

    pub const fn is_employed_at_cgn_uni_related_thing(&self) -> bool {
        matches!(
            self,
            Self::BeschaeftigteStaatlicherKoelnerHochschulen
                | Self::BeschaeftigteUniKlinikKoeln
                | Self::BeschaeftigteKoelnerStudierendenwerk
        )
    }
}
