use super::models::{FileService, TransferRequest, TransferResponse, TransferStatus};
use crate::errors::models::AppError;
use axum::{extract::State, Json};

pub async fn data_transfer<T: FileService>(
    State(repo): State<T>,
    Json(data): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, AppError> {
    let transfer_response = repo.transfer(data).await?;
    Ok(Json(transfer_response))
}

pub async fn transfer_status<T: FileService>(
    State(repo): State<T>,
    Json(data): Json<TransferStatus>,
) -> Result<Json<TransferResponse>, AppError> {
    let status_response = repo.status(data).await?;
    Ok(Json(status_response))
}
