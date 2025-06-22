use teloxide::types::EffectId;

pub enum MessageEffect {
    Heart,
    Celebration,
    Fire,
    ThumbsUp,
    ThumbsDown,
    Poop,
}

impl MessageEffect {
    pub fn id(&self) -> EffectId {
        match self {
            MessageEffect::Heart => "5159385139981059251",
            MessageEffect::Celebration => "5046509860389126442",
            MessageEffect::Fire => "5104841245755180586",
            MessageEffect::ThumbsUp => "5107584321108051014",
            MessageEffect::ThumbsDown => "5104858069142078462",
            MessageEffect::Poop => "5046589136895476101",
        }
        .into()
    }
}
