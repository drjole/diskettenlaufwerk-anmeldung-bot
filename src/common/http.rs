use color_eyre::Result;
use reqwest::RequestBuilder;
use tokio::time::{sleep, Duration};

pub async fn request_document(builder: RequestBuilder) -> Result<String> {
    let response = builder.send().await?;
    dbg!(&response);
    sleep(Duration::from_secs(3)).await;
    let text = response.text().await?;
    println!("{text}");
    Ok(text)
}
