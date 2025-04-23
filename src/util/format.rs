use axum::http::HeaderValue;
use image::ImageFormat;
use std::path::Path;

pub fn parse_image_format_from_content_type(content_type: &HeaderValue) -> Option<ImageFormat> {
    let Ok(content_type) = content_type.to_str() else {
        return None;
    };

    match content_type.to_ascii_lowercase().as_str() {
        "image/png" => Some(ImageFormat::Png),
        "image/jpeg" | "image/jpg" => Some(ImageFormat::Jpeg),
        "image/gif" => Some(ImageFormat::Gif),
        "image/webp" => Some(ImageFormat::WebP),
        "image/x-portable-anymap" => Some(ImageFormat::Pnm),
        "image/tiff" => Some(ImageFormat::Tiff),
        "image/x-tga" => Some(ImageFormat::Tga),
        "image/vnd.ms-dds" => Some(ImageFormat::Dds),
        "image/bmp" => Some(ImageFormat::Bmp),
        "image/vnd.microsoft.icon" | "image/x-icon" => Some(ImageFormat::Ico),
        "image/vnd.radiance" => Some(ImageFormat::Hdr),
        "image/aces" | "image/exr" => Some(ImageFormat::OpenExr),
        "image/farbfeld" => Some(ImageFormat::Farbfeld),
        "image/avif" => Some(ImageFormat::Avif),
        "image/qoi" => Some(ImageFormat::Qoi),
        "image/x-pcx" => Some(ImageFormat::Pcx),
        _ => None,
    }
}

pub fn parse_image_format_from_filename(filename: &str) -> Option<ImageFormat> {
    let extension: &str = Path::new(filename)
        .extension()
        .and_then(|extension| extension.to_str())?;
    ImageFormat::from_extension(extension)
}

pub fn resolve_content_type(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Png => "image/png",
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Gif => "image/gif",
        ImageFormat::WebP => "image/webp",
        ImageFormat::Pnm => "image/x-portable-anymap",
        ImageFormat::Tiff => "image/tiff",
        ImageFormat::Tga => "image/x-tga",
        ImageFormat::Dds => "image/vnd.ms-dds",
        ImageFormat::Bmp => "image/bmp",
        ImageFormat::Ico => "image/x-icon",
        ImageFormat::Hdr => "image/vnd.radiance",
        ImageFormat::OpenExr => "image/exr",
        ImageFormat::Farbfeld => "image/farbfeld",
        ImageFormat::Avif => "image/avif",
        ImageFormat::Qoi => "image/qoi",
        ImageFormat::Pcx => "image/x-pcx",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_from_content_type() {
        let header = HeaderValue::from_str("image/avif").unwrap();
        let result = parse_image_format_from_content_type(&header);
        assert_eq!(result, Some(ImageFormat::Avif));

        let header = HeaderValue::from_str("foo/bar").unwrap();
        let result = parse_image_format_from_content_type(&header);
        assert_eq!(result, None);
    }

    #[test]
    fn parses_from_filename() {
        let result = parse_image_format_from_filename("image.jpg");
        assert_eq!(result, Some(ImageFormat::Jpeg));

        let result = parse_image_format_from_filename("foo.bar");
        assert_eq!(result, None);
    }

    #[test]
    fn resolves_content_type() {
        let result = resolve_content_type(ImageFormat::Jpeg);
        assert_eq!(result, "image/jpeg");
    }
}
