use strum::{Display, EnumIter, EnumProperty, EnumString};

#[derive(Clone, Debug, Display, EnumString, EnumProperty, EnumIter, sqlx::Type)]
#[sqlx(type_name = "participant_status")]
pub enum Status {
    #[strum(props(pretty = "Stud. Uni Köln"))]
    StudentUniKoeln,
    #[strum(props(pretty = "Stud. DSHS Köln"))]
    StudentDSHSKoeln,
    #[strum(props(pretty = "Stud. TH Köln"))]
    StudentTHKoeln,
    #[strum(props(pretty = "Stud. Macromedia Köln"))]
    StudentMacromediaKoeln,
    #[strum(props(pretty = "Stud. KHM Köln"))]
    StudentKunsthochschuleFuerMedien,
    #[strum(props(pretty = "Stud. HMKW Köln"))]
    StudentHochschuleFuerMedienKommunikationUndWirtschaft,
    #[strum(props(pretty = "Stud. HfMT Köln"))]
    StudentHochschuleFuerMusikKoeln,
    #[strum(props(pretty = "Stud. anderer Hochschulen"))]
    StudentAndereHochschulen,
    #[strum(props(pretty = "Beschäft. staatl. Kölner Hochschulen"))]
    BeschaeftigteStaatlicherKoelnerHochschulen,
    #[strum(props(pretty = "Beschäft. UniKlinik Köln"))]
    BeschaeftigteUniKlinikKoeln,
    #[strum(props(pretty = "Beschäft. Kölner Studierendenwerk"))]
    BeschaeftigteKoelnerStudierendenwerk,
    #[strum(props(pretty = "Mitglied von KölnAlumni"))]
    MitgliedKoelnAlumni,
    #[strum(props(pretty = "Azubi Uni Köln"))]
    AzubiUniKoeln,
    #[strum(props(pretty = "Azubi UniKlinik Köln"))]
    AzubiUniKlinik,
    #[strum(props(pretty = "Azubi Kölner Studierendenwerk"))]
    AzubiKoelnerStudierendenwerk,
    #[strum(props(pretty = "Gast"))]
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
