use async_trait::async_trait;
use futures::stream::Stream;
use std::pin::Pin;

use crate::errors::models::AppError;

pub type ByteStream = Pin<Box<dyn Stream<Item = Result<bytes::Bytes, AppError>> + Send>>;

#[async_trait]
pub trait StreamDownloader: Send + Sync {
    async fn download_stream(&self) -> Result<ByteStream, AppError>;
}

#[async_trait]
pub trait StreamUploader: Send + Sync {
    async fn upload_stream(&self, stream: ByteStream) -> Result<(), AppError>;
}
