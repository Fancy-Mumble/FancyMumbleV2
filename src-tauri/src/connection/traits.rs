use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait Shutdown {
    async fn shutdown(&mut self) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait HandleMessage {}
