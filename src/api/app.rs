use super::{
    handlers::{data_transfer, transfer_status},
    models::FileService,
};
use axum::{routing::post, Router};

pub fn router<T: FileService>(repo: T) -> Router {
    Router::new()
        .route("/transfer", post(data_transfer::<T>))
        .route("/status", post(transfer_status::<T>))
        .with_state(repo)
}
