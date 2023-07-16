use color_eyre::Result;
use reqwest::RequestBuilder;
use scraper::Html;
use tokio::time::{sleep, Duration};

pub async fn request_document(builder: RequestBuilder) -> Result<Html> {
    let response = builder.send().await?;
    sleep(Duration::from_secs(3)).await;
    let text = response.text().await?;
    let document = scraper::Html::parse_document(&text);
    Ok(document)
}
