use crate::{errors::models::AppError, streaming::models::ByteStream};
use async_trait::async_trait;

#[async_trait]
pub trait Downloader: Send + Sync {
    async fn download(&self) -> Result<ByteStream, AppError>;
    async fn status(&self) -> Result<Status, AppError>;
}

#[async_trait]
pub trait Uploader: Send + Sync {
    async fn upload(&self, stream: ByteStream) -> Result<(), AppError>;
    async fn status(&self) -> Result<Status, AppError>;
}

pub struct Status {
    pub available: bool,
}
