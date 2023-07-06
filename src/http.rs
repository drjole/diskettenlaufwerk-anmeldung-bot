use crate::Error;
use reqwest::RequestBuilder;
use scraper::Html;

pub async fn request_document(builder: RequestBuilder) -> Result<Html, Error> {
    let response = builder.send().await?;
    std::thread::sleep(std::time::Duration::from_secs(3));
    let text = response.text().await?;
    let document = scraper::Html::parse_document(&text);
    Ok(document)
}
