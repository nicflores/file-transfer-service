use super::models::{
    Destination, FileService, FileTransferService, Source, Transfer, TransferKey, TransferRequest,
    TransferResponse, TransferStatus,
};
use crate::errors::models::AppError;
use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};

impl FileTransferService {
    pub fn new() -> Self {
        Self {
            transfer_impls: HashMap::new(),
        }
    }

    pub fn add_transfer_impl(&mut self, key: TransferKey, transfer_impl: Arc<dyn Transfer>) {
        self.transfer_impls.insert(key, transfer_impl);
    }

    pub fn get_transfer_impl(&self, key: &TransferKey) -> Option<&Arc<dyn Transfer>> {
        self.transfer_impls.get(key)
    }

    pub fn list_transfer_impls(&self) -> Vec<TransferKey> {
        self.transfer_impls.keys().cloned().collect()
    }
}

#[async_trait]
impl FileService for FileTransferService {
    async fn transfer(&self, req: TransferRequest) -> Result<TransferResponse, AppError> {
        let source_type = match &req.source {
            Source::AwsS3(_) => "aws_s3",
            Source::AzureBlob(_) => "azure_blob",
            Source::Sftp(_) => "sftp",
            Source::HttpApi(_) => "http_api",
        };

        let destination_type = match &req.destination {
            Destination::AwsS3(_) => "aws_s3",
            Destination::AzureBlob(_) => "azure_blob",
            Destination::Sftp(_) => "sftp",
            Destination::HttpApi(_) => "http_api",
        };

        let key = TransferKey::new(source_type, destination_type);

        match self.get_transfer_impl(&key) {
            Some(impl_trait) => {
                impl_trait
                    .execute_transfer(&req.source, &req.destination)
                    .await
            }
            None => Err(AppError::from(format!(
                "Unsupported transfer combination {} -> {}",
                source_type, destination_type
            ))),
        }
    }

    async fn status(&self, req: TransferStatus) -> Result<TransferResponse, AppError> {
        // If the source of the TransferStatus is something we can check the status of, like sftp.
        // Maybe sources should come with their own status check impls?
        todo!("Implement status check")
    }
}

impl TransferKey {
    pub fn new(source: &'static str, destination: &'static str) -> Self {
        Self {
            source_type: source,
            destination_type: destination,
        }
    }
}
