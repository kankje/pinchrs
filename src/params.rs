use crate::operation::{Operation, Rotation};
use anyhow::{anyhow, bail};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::*;
use image::{EncodableLayout, ImageFormat};
use std::str;

pub struct Params {
    pub url: String,
    pub operations: Vec<Operation>,
}

pub fn parse_params(path_without_signature: &str) -> Result<Params, anyhow::Error> {
    let segments: Vec<_> = path_without_signature.split('/').collect();
    let [filters @ .., encoded_url] = segments.as_slice() else {
        bail!("Image URL missing");
    };

    let mut operations = Vec::new();
    for filter in filters {
        let parts: Vec<_> = filter.split(":").collect();
        match parts.as_slice() {
            ["format", format] => {
                operations.push(Operation::Format(
                    ImageFormat::from_extension(format).ok_or(anyhow!("Invalid format"))?,
                ));
            }

            ["speed", speed] => {
                operations.push(Operation::Speed(speed.parse::<u8>()?));
            }

            ["quality", quality] => {
                operations.push(Operation::Quality(quality.parse::<u8>()?));
            }

            ["resize", width, height] => {
                operations.push(Operation::Resize(
                    width.parse::<u32>()?,
                    height.parse::<u32>()?,
                ));
            }

            ["rotate", degrees] => {
                let normalized_degrees = degrees.parse::<u32>()? % 360;
                if normalized_degrees > 0 {
                    operations.push(Operation::Rotate(match normalized_degrees {
                        90 => Rotation::Rotate90,
                        180 => Rotation::Rotate180,
                        270 => Rotation::Rotate270,
                        _ => {
                            bail!("Invalid rotation");
                        }
                    }));
                }
            }

            _ => {
                bail!("Invalid filter");
            }
        }
    }

    let url = str::from_utf8(
        URL_SAFE_NO_PAD
            .decode(encoded_url.replace('=', ""))?
            .as_bytes(),
    )?
    .to_string();

    Ok(Params { url, operations })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_params() {
        let result =
            parse_params("resize:800:600/format:webp/quality:85/rotate:90/cGF0aA").unwrap();
        assert_eq!(result.url, "path");
        assert_eq!(result.operations.len(), 4);
    }

    #[test]
    fn test_fails_parsing_due_to_missing_image_url() {
        let result = parse_params("resize:800:600");
        assert!(result.is_err());
    }

    #[test]
    fn test_fails_parsing_due_to_invalid_image_url() {
        let result = parse_params("resize:800:600/!!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_fails_parsing_due_to_invalid_filter() {
        let result = parse_params("invalidfilter/cGF0aA");
        assert!(result.is_err());
    }

    #[test]
    fn test_fails_parsing_due_to_invalid_rotation() {
        let result = parse_params("rotate:45/cGF0aA");
        assert!(result.is_err());
    }

    #[test]
    fn test_fails_parsing_due_to_invalid_number() {
        let result = parse_params("quality:high/cGF0aA");
        assert!(result.is_err());
    }
}
