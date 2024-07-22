use crate::{
    api::models::{Destination, Source, Transfer, TransferResponse},
    errors::models::AppError,
    ops::models::{Downloader, Uploader},
};
use async_trait::async_trait;
use hyper::StatusCode;

#[derive(Debug, Clone)]
pub struct AwsS3ToSftp;

impl AwsS3ToSftp {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Transfer for AwsS3ToSftp {
    async fn execute_transfer(
        &self,
        source: &Source,
        destination: &Destination,
    ) -> Result<TransferResponse, AppError> {
        if let (Source::AwsS3(s3), Destination::Sftp(sftp)) = (source, destination) {
            let stream = s3.download().await?;
            sftp.upload(stream).await?;

            return Ok(TransferResponse {
                status: "success".into(),
                message: format!(
                    "File from bucket {} with key {} transferred to SFTP at {}{}",
                    s3.bucket, s3.key, sftp.host, sftp.remote_path
                ),
            });
        }
        Err(AppError::new(
            format!("Error executing transfer between awss3 -> sftp").as_str(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }

    fn supports(&self, source: &Source, destination: &Destination) -> bool {
        matches!(
            (source, destination),
            (Source::AwsS3 { .. }, Destination::Sftp { .. })
        )
    }
}
