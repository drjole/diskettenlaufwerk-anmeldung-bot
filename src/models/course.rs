use chrono::DateTime;
use chrono_tz::Tz;
use url::Url;

#[derive(Debug)]
pub struct Course {
    url: Url,
    start_time: DateTime<Tz>,
    end_time: DateTime<Tz>,
    level: String,
    location: String,
    trainer: String,
}

impl Course {
    fn open(&self) -> bool {
        self.url.path() == "/cgi/anmeldung.fcgi"
    }
}
