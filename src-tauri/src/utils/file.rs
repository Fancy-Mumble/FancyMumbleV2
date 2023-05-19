use std::fmt::Display;

use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, GenericImageView};
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
    let mut buffer = vec![];
    buffer.reserve(metadata.len() as usize);

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
    let image_type = image::ImageFormat::from_path(filename)?;

    if image_type == image::ImageFormat::Gif {
        let file = std::fs::File::open(filename)?;
        let reader = std::io::BufReader::new(file);
        let gif_decoder = GifDecoder::new(reader)?;

        let frame_iter = gif_decoder.into_frames().filter_map(Result::ok);
        {
            let mut encoder = image::codecs::gif::GifEncoder::new(buf_writer);
            encoder.set_repeat(image::codecs::gif::Repeat::Infinite)?;
            encoder.encode_frames(frame_iter)?;
        }
    } else {
        let mut image = image::open(filename)?;
        image = image.thumbnail(max_size, max_size);

        let (width, height) = image.dimensions();
        let color_type = image.color();

        let image = image.into_bytes();

        image::codecs::jpeg::JpegEncoder::new(buf_writer)
            .encode(&image, width, height, color_type)?;
    }

    Ok(ImageInfo {
        data: output,
        format: ImageFormat { format: image_type },
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
