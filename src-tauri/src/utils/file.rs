use std::fmt::Display;

use image::imageops::FilterType;
use image::GenericImageView;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, BufReader};
use tracing::debug;

use crate::errors::AnyError;

pub struct ImageInfo {
    pub data: Vec<u8>,
    pub format: ImageFormat,
}

pub struct ImageFormat {
    pub format: image::ImageFormat,
}

// we check that the file is not too large
#[allow(clippy::cast_possible_truncation)]
#[allow(unused)]
pub async fn get_file_as_byte_vec(filename: &str) -> AnyError<Vec<u8>> {
    let mut f = BufReader::new(File::open(&filename).await?);
    let metadata = fs::metadata(&filename).await?;
    let mut buffer = vec![0; metadata.len() as usize];
    if buffer.len() > u32::MAX as usize {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "File too large",
        )));
    }

    match f.read_to_end(&mut buffer).await {
        Ok(read) => {
            if read == metadata.len() as usize {
                debug!("Read {} bytes from {}", read, filename);
                Ok(buffer)
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to read all bytes",
                )))
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn read_image_as_thumbnail(filename: &str, max_size: u32) -> AnyError<ImageInfo> {
    let mut output = Vec::new();
    let buf_writer = std::io::BufWriter::new(&mut output);
    let image = image::open(filename)?;

    let image = image.resize(max_size, max_size, FilterType::Lanczos3);
    let (width, height) = image.dimensions();
    let color_type = image.color();

    let image = image.into_bytes();

    image::codecs::jpeg::JpegEncoder::new_with_quality(buf_writer, 80)
        .encode(&image, width, height, color_type)?;

    Ok(ImageInfo {
        data: output,
        format: ImageFormat {
            format: image::ImageFormat::Jpeg,
        },
    })
}

impl Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.format {
            image::ImageFormat::Png => write!(f, "png"),
            image::ImageFormat::Jpeg => write!(f, "jpeg"),
            image::ImageFormat::Gif => write!(f, "gif"),
            image::ImageFormat::WebP => write!(f, "webp"),
            image::ImageFormat::Pnm => write!(f, "pnm"),
            image::ImageFormat::Tiff => write!(f, "tiff"),
            image::ImageFormat::Tga => write!(f, "tga"),
            image::ImageFormat::Dds => write!(f, "dds"),
            image::ImageFormat::Bmp => write!(f, "bmp"),
            image::ImageFormat::Ico => write!(f, "ico"),
            image::ImageFormat::Hdr => write!(f, "hdr"),
            image::ImageFormat::Farbfeld => write!(f, "farbfeld"),
            image::ImageFormat::Avif => write!(f, "avif"),
            image::ImageFormat::OpenExr => write!(f, "exr"),
            image::ImageFormat::Qoi => write!(f, "qoi"),
            _ => todo!(),
        }
    }
}

impl From<image::ImageFormat> for ImageFormat {
    fn from(format: image::ImageFormat) -> Self {
        Self { format }
    }
}
