use crate::encode::EncodeOptions;
use image::imageops::FilterType as ImageFilterType;
use image::{DynamicImage, ImageFormat};

#[derive(Clone, Copy)]
pub enum Rotation {
    Rotate90,
    Rotate180,
    Rotate270,
}

pub enum Operation {
    Format(ImageFormat),
    Speed(u8),
    Quality(u8),
    Resize(u32, u32),
    Rotate(Rotation),
}

pub fn apply_operations(
    image: DynamicImage,
    input_format: ImageFormat,
    operations: &[Operation],
) -> (DynamicImage, EncodeOptions) {
    let mut image = image;
    let mut output_options = EncodeOptions {
        format: input_format,
        speed: None,
        quality: None,
    };

    for operation in operations {
        match *operation {
            Operation::Format(format) => {
                output_options.format = format;
            }

            Operation::Speed(speed) => {
                output_options.speed = Some(speed);
            }

            Operation::Quality(quality) => {
                output_options.quality = Some(quality);
            }

            Operation::Resize(width, height) => {
                image = image.resize(width, height, ImageFilterType::Lanczos3);
            }

            Operation::Rotate(rotation) => {
                image = match rotation {
                    Rotation::Rotate90 => image.rotate90(),
                    Rotation::Rotate180 => image.rotate180(),
                    Rotation::Rotate270 => image.rotate270(),
                }
            }
        }
    }

    (image, output_options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageFormat, RgbaImage};

    fn create_test_image() -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::from_fn(64, 64, |_, _| {
            image::Rgba([255, 255, 255, 255])
        }))
    }

    #[test]
    fn test_applies_operations() {
        let image = create_test_image();
        let operations = vec![
            Operation::Resize(48, 32),
            Operation::Rotate(Rotation::Rotate180),
            Operation::Quality(90),
            Operation::Speed(2),
            Operation::Format(ImageFormat::Jpeg),
        ];

        let (output_image, options) = apply_operations(image, ImageFormat::Png, &operations);

        assert_eq!(output_image.width(), 32);
        assert_eq!(output_image.height(), 32);
        assert_eq!(options.format, ImageFormat::Jpeg);
        assert_eq!(options.quality, Some(90));
        assert_eq!(options.speed, Some(2));
    }
}
