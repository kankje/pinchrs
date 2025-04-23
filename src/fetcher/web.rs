use crate::fetcher::{FetchResult, Fetcher};
use crate::util::format;
use content_disposition::parse_content_disposition;

pub struct WebFetcher {}

impl WebFetcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl Fetcher for WebFetcher {
    async fn fetch(&self, url: &str) -> anyhow::Result<FetchResult> {
        let client = reqwest::Client::builder()
            .user_agent(format!(
                "{}/{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;
        let response = client.get(url).send().await?;
        let headers = response.headers();

        let filename = headers
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|header| header.to_str().ok())
            .map(parse_content_disposition)
            .and_then(|content_disposition| content_disposition.filename_full());

        let image_format = headers
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(format::parse_image_format_from_content_type)
            .or_else(|| match &filename {
                Some(filename) => format::parse_image_format_from_filename(filename.as_str()),
                None => None,
            });

        Ok(FetchResult {
            bytes: response.bytes().await?,
            filename,
            image_format,
        })
    }
}
