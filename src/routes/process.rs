use crate::AppState;
use crate::encode::encode_image;
use crate::fetcher::Fetcher;
use crate::operation::apply_operations;
use crate::params::parse_params;
use crate::signature::verify_signature;
use crate::util::error::AppError;
use crate::util::format;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use image::ImageReader;
use std::io::Cursor;
use tokio::task;

pub async fn process(
    State(state): State<AppState>,
    Path((signature, rest)): Path<(String, String)>,
) -> Result<impl IntoResponse, AppError> {
    if let Some(ref key) = state.key {
        verify_signature(rest.as_str(), signature.as_str(), key)
            .map_err(|_| AppError::Forbidden("Invalid signature".to_string()))?;
    }

    let params = parse_params(rest.as_str())
        .map_err(|_| AppError::UnprocessableEntity("Invalid params".to_string()))?;

    // Fetch remote image
    let fetcher =
        state
            .resolve_fetcher(params.url.as_str())
            .ok_or(AppError::UnprocessableEntity(
                "Unsupported protocol for remote image".to_string(),
            ))?;
    let fetch_result = fetcher
        .fetch(params.url.as_str())
        .await
        .map_err(|_| AppError::NotFound("Fetching remote image failed".to_string()))?;

    // Create reader for appropriate image format
    let reader = if let Some(image_format) = fetch_result.image_format {
        let mut reader = ImageReader::new(Cursor::new(fetch_result.bytes));
        reader.set_format(image_format);
        reader
    } else {
        ImageReader::new(Cursor::new(fetch_result.bytes))
            .with_guessed_format()
            .map_err(|_| {
                AppError::UnprocessableEntity("Unable to determine image format".to_string())
            })?
    };
    let input_format = reader.format().ok_or(AppError::UnprocessableEntity(
        "Unable to determine image format".to_string(),
    ))?;

    // Decode, apply operations and encode
    let join = task::spawn_blocking(move || {
        let decoded_image = reader.decode()?;

        let (image, output_options) =
            apply_operations(decoded_image, input_format, params.operations.as_slice());

        encode_image(image, output_options)
    });
    let (buffer, format) = join
        .await
        .map_err(|_| AppError::UnprocessableEntity("Processing image failed".to_string()))?
        .map_err(|_| AppError::UnprocessableEntity("Decoding image failed".to_string()))?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "Cache-Control",
        HeaderValue::from_static("max-age=31536000, public"),
    );
    headers.insert(
        "Content-Disposition",
        fetch_result
            .filename
            .as_deref()
            .map(|filename| format!("inline; filename=\"{}\"", filename))
            .and_then(|value| HeaderValue::from_str(&value).ok())
            .unwrap_or_else(|| HeaderValue::from_static("inline")),
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_static(format::resolve_content_type(format)),
    );

    Ok((headers, buffer.into_inner()))
}
