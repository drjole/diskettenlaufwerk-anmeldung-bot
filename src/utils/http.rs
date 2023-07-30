use color_eyre::{eyre::eyre, Result};
use reqwest::RequestBuilder;

pub async fn request_document(builder: RequestBuilder) -> Result<String> {
    let response = builder.send().await.map_err(|err| {
        log::error!("request error: {err}");
        eyre!("Verbindungsfehler: {err}")
    })?;
    if response.status() != 200 {
        log::error!("got response with code {}", response.status());
        return Err(eyre!(
            "Server hat mit Code {} geantwortet",
            response.status()
        ));
    }
    Ok(response.text().await?)
}
