use color_eyre::{eyre::eyre, Result};
use reqwest::RequestBuilder;

pub async fn request_document(builder: RequestBuilder) -> Result<String> {
    let response = match builder.send().await {
        Ok(r) => {
            if r.status() == 200 {
                r
            } else {
                log::error!("got response with code {}", r.status());
                return Err(eyre!("Server hat mit Code {} geantwortet", r.status()));
            }
        }
        Err(err) => {
            log::error!("request error: {err}");
            return Err(eyre!("Verbindungsfehler: {err}"));
        }
    };
    Ok(response.text().await?)
}
