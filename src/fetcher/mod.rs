pub mod web;

use axum::body::Bytes;
use image::ImageFormat;

pub struct FetchResult {
    pub bytes: Bytes,
    pub filename: Option<String>,
    pub image_format: Option<ImageFormat>,
}

pub trait Fetcher {
    fn fetch(&self, url: &str) -> impl Future<Output = anyhow::Result<FetchResult>> + Send;
}
