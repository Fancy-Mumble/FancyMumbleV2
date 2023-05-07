use std::error::Error;

use tokio::fs::{self, File};
use tokio::io::AsyncReadExt;

pub async fn get_file_as_byte_vec(filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut f = File::open(&filename).await?;
    let metadata = fs::metadata(&filename).await?;
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).await?;

    Ok(buffer)
}
