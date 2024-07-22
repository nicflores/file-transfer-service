use super::models::{Downloader, Status, Uploader};
use crate::api::models::{AwsS3, AzureBlob, Sftp};
use crate::errors::models::AppError;
use crate::streaming::models::{ByteStream, StreamDownloader, StreamUploader};
use async_trait::async_trait;

#[async_trait]
impl Downloader for AzureBlob {
    async fn download(&self) -> Result<ByteStream, AppError> {
        todo!("Implement the logic to download the file from Azure Blob");
    }

    async fn status(&self) -> Result<Status, AppError> {
        // Implement the logic to check the status of the Azure Blob.
        // Placeholder implementation:
        Ok(Status { available: true }) // Replace with actual implementation
    }
}

#[async_trait]
impl Downloader for AwsS3 {
    async fn download(&self) -> Result<ByteStream, AppError> {
        self.download_stream().await
    }

    async fn status(&self) -> Result<Status, AppError> {
        // Implement the logic to check the status of the AWS S3.
        // Placeholder implementation:
        Ok(Status { available: true }) // Replace with actual implementation
    }
}

#[async_trait]
impl Uploader for Sftp {
    async fn upload(&self, stream: ByteStream) -> Result<(), AppError> {
        self.upload_stream(stream).await
    }

    async fn status(&self) -> Result<Status, AppError> {
        // Implement the logic to check the status of the SFTP.
        // Placeholder implementation:
        Ok(Status { available: true }) // Replace with actual implementation
    }
}
