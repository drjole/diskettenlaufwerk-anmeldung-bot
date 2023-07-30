use crate::models::{course::Course, participant::Participant};
use std::fmt::Display;

#[derive(Debug)]
pub enum TextMessage {
    Start,
    Cancel,
    ShowData(Participant),
    EnterDataComplete,
    SignupResponse(Course),
}

impl Display for TextMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(
                f,
                "Hey!

Ich helfe dir dabei, dich für die Ultimate-Frisbee Kurse des UniSport Köln anzumelden.

Um loszulegen, nutze den /enter_data Befehl.

Keine Sorge! Solltest du bei der Eingabe deiner Daten einen Fehler machen, kannst du deine Daten später ändern.
Fahre dafür zunächst mit der Eingabe deiner Daten fort und nutze dann die in /help angezeigten Befehle, um deine Daten zu ändern.

Warnung: Ich überprüfe deine Daten in keinster Weise auf Echtheit oder Korrektheit, sondern schicke diese so, wie du sie eingibts, an den UniSport weiter."
            ),
            Self::Cancel => write!(f, "Aktion abgebrochen."),
            Self::ShowData(participant) => write!(
                f,
                "Ich habe folgende Informationen über dich gespeichert. Nutze die angezeigten Befehle, um deine Daten zu ändern.

{participant}"
            ),
            Self::EnterDataComplete => write!(
                f,
                "Super {}

Damit habe ich alle Daten, die ich brauche.

Wenn du deine Daten ändern willst, nutze die /edit... Befehle. Diese findest du auch, wenn du dir deine Daten mittels /show_data anzeigen lässt.

Wenn Trainings anstehen, wirst du von mir benachrichtigt. Du kannst dann antworten und dich anmelden lassen.",
                emojis::get_by_shortcode("tada").ok_or(std::fmt::Error)?
            ),
            Self::SignupResponse(course) => write!(
                f,
                "Heute ist Frisbee-Zeit! {}

{course}

Soll ich dich anmelden?",
                emojis::get_by_shortcode("flying_disc").ok_or(std::fmt::Error)?
            ),
        }
    }
}
