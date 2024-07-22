use std::{collections::HashMap, sync::Arc};

use crate::errors::models::AppError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct AzureBlob {
    pub container: String,
    pub blob: String,
    pub connection_string: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Sftp {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub ssh_key: String,
    pub remote_path: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct HttpApi {
    pub url: String,
    pub body: String,
    pub token: Option<String>,
    pub headers: Option<Vec<(String, String)>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AwsS3 {
    pub bucket: String,
    pub key: String,
    pub region: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Source {
    AwsS3(AwsS3),
    AzureBlob(AzureBlob),
    Sftp(Sftp),
    HttpApi(HttpApi),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Destination {
    AwsS3(AwsS3),
    AzureBlob(AzureBlob),
    Sftp(Sftp),
    HttpApi(HttpApi),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TransferRequest {
    pub source: Source,
    pub destination: Destination,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TransferStatus {
    pub source: String,
    pub filename: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct TransferResponse {
    pub status: String,
    pub message: String,
}

#[async_trait]
pub trait FileService: Send + Sync + Clone + 'static {
    async fn transfer(&self, request: TransferRequest) -> Result<TransferResponse, AppError>;
    async fn status(&self, request: TransferStatus) -> Result<TransferResponse, AppError>;
}

/// The Transfer trait is defined with an async function to handle the transfer.
/// This trait can be implemented for each specific source and destination combination.
#[async_trait]
pub trait Transfer: Send + Sync {
    async fn execute_transfer(
        &self,
        source: &Source,
        destination: &Destination,
    ) -> Result<TransferResponse, AppError>;
    fn supports(&self, source: &Source, destination: &Destination) -> bool;
}

/// This struct contains a HashMap of TransferKey -> Arc<dyn Transfer> to hold different
/// implementations of the Transfer trait. It provides a method to add new
/// implementations and a method to transfer files using the appropriate
/// implementation based on the source and destination types.
#[derive(Clone)]
pub struct FileTransferService {
    pub transfer_impls: HashMap<TransferKey, Arc<dyn Transfer>>,
}

/// The TransferKey struct is used as a key of the HashMap above to store Trnasfers.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TransferKey {
    pub source_type: &'static str,
    pub destination_type: &'static str,
}
