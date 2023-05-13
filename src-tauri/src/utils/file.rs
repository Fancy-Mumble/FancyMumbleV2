use std::error::Error;

use tokio::fs::{self, File};
use tokio::io::AsyncReadExt;

// we check that the file is not too large
#[allow(clippy::cast_possible_truncation)]
pub async fn get_file_as_byte_vec(filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(&filename).await?;
    let metadata = fs::metadata(&filename).await?;
    let mut buffer = vec![0; metadata.len() as usize];
    if buffer.len() > u32::MAX as usize {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "File too large",
        )));
    }

    match f.read(&mut buffer).await {
        Ok(read) => {
            if read == metadata.len() as usize {
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
