use anyhow::bail;
use image::codecs::avif::AvifEncoder;
use image::codecs::bmp::BmpEncoder;
use image::codecs::farbfeld::FarbfeldEncoder;
use image::codecs::gif::GifEncoder;
use image::codecs::hdr::HdrEncoder;
use image::codecs::ico::IcoEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::openexr::OpenExrEncoder;
use image::codecs::png::{CompressionType, FilterType as PngFilterType, PngEncoder};
use image::codecs::pnm::PnmEncoder;
use image::codecs::qoi::QoiEncoder;
use image::codecs::tga::TgaEncoder;
use image::codecs::tiff::TiffEncoder;
use image::codecs::webp::WebPEncoder;
use image::{DynamicImage, ImageEncoder, ImageFormat};

use std::io::Cursor;

pub struct EncodeOptions {
    pub format: ImageFormat,
    pub speed: Option<u8>,
    pub quality: Option<u8>,
}

pub fn encode_image(
    image: DynamicImage,
    options: EncodeOptions,
) -> anyhow::Result<(Cursor<Vec<u8>>, ImageFormat)> {
    let mut buffer = Cursor::new(Vec::new());

    let result = match options.format {
        ImageFormat::Png => PngEncoder::new_with_quality(
            &mut buffer,
            CompressionType::Default,
            PngFilterType::Adaptive,
        )
        .write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Jpeg => {
            JpegEncoder::new_with_quality(&mut buffer, options.quality.unwrap_or(80)).write_image(
                image.as_bytes(),
                image.width(),
                image.height(),
                image.color().into(),
            )
        }

        ImageFormat::Gif => {
            GifEncoder::new_with_speed(&mut buffer, options.speed.unwrap_or(10) as i32).encode(
                image.as_bytes(),
                image.width(),
                image.height(),
                image.color().into(),
            )
        }

        ImageFormat::WebP => WebPEncoder::new_lossless(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Pnm => PnmEncoder::new(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Tiff => TiffEncoder::new(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Tga => TgaEncoder::new(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Bmp => BmpEncoder::new(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Ico => IcoEncoder::new(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Hdr => HdrEncoder::new(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::OpenExr => OpenExrEncoder::new(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Farbfeld => FarbfeldEncoder::new(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Avif => AvifEncoder::new_with_speed_quality(
            &mut buffer,
            options.speed.unwrap_or(8),
            options.quality.unwrap_or(80),
        )
        .write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        ImageFormat::Qoi => QoiEncoder::new(&mut buffer).write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        ),

        _ => {
            bail!("Unsupported output format");
        }
    };
    if result.is_err() {
        bail!("Encoding image failed");
    };

    Ok((buffer, options.format))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageFormat, RgbaImage};

    #[test]
    fn test_encodes_image() {
        let image = DynamicImage::ImageRgba8(RgbaImage::from_fn(64, 64, |_, _| {
            image::Rgba([255, 255, 255, 255])
        }));
        let options = EncodeOptions {
            format: ImageFormat::Avif,
            speed: Some(4),
            quality: Some(60),
        };

        let result = encode_image(image, options);
        assert!(result.is_ok());
        let (buffer, format) = result.unwrap();
        assert!(!buffer.get_ref().is_empty());
        assert_eq!(format, ImageFormat::Avif);
    }
}
