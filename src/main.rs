use axum::middleware;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use file_transfer_service::api::app;
use file_transfer_service::api::models::{FileTransferService, TransferKey};
use file_transfer_service::capabilities::s3tosftp::AwsS3ToSftp;
use file_transfer_service::config::models::AppConfig;
use file_transfer_service::shutdown::shutdown_signal;
use file_transfer_service::utils::auth::auth;
use std::sync::Arc;
use tower::ServiceBuilder;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// The main function is the entry point of the application.
#[tokio::main]
async fn main() {
    // Load the application configuration.
    let cfg = AppConfig::new().unwrap();

    // Setup the tracing subscriber to log to stdout.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    //let file_ops = FileTransferService::new();
    let mut transfer_service = FileTransferService::new();

    // Add the AwsS3ToSftp transfer implementation
    transfer_service.add_transfer_impl(
        TransferKey::new("aws_s3", "sftp"),
        Arc::new(AwsS3ToSftp::new()),
    );

    // Setup the routers for the various parts of the application.
    let app_router = app::router(transfer_service);

    // Setup the auth layer.
    let token = Arc::new(cfg.api_key.clone());
    let auth_layer = ServiceBuilder::new()
        .layer(middleware::from_fn(move |req, next| {
            let token = token.clone();
            async move { auth(req, next, token).await }
        }))
        .into_inner();

    // Merge all the routers into a single app.
    let app = app_router
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(auth_layer);

    // Create a TCP listener and serve the app on the listener.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    // This is the main event loop that listens for incoming requests.
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
