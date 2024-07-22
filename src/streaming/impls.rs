use std::io::Write;
use std::path::Path;

use async_trait::async_trait;
use aws_sdk_s3::Client;
use bytes::Buf;
use futures::StreamExt;
use hyper::StatusCode;
use ssh2::Session;
use tokio::net::TcpStream;
use tokio_util::io::ReaderStream;

use crate::api::models::Sftp;
use crate::{api::models::AwsS3, errors::models::AppError};

use super::models::StreamUploader;
use super::models::{ByteStream, StreamDownloader};

#[async_trait]
impl StreamDownloader for AwsS3 {
    async fn download_stream(&self) -> Result<ByteStream, AppError> {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);

        let resp = client
            .get_object()
            .bucket(&self.bucket)
            .key(&self.key)
            .send()
            .await?;

        let byte_stream =
            ReaderStream::new(resp.body.into_async_read()).map(|result| match result {
                Ok(bytes) => Ok(bytes),
                Err(err) => Err(AppError::new(
                    &format!("Error streaming object: {}", err),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )),
            });

        Ok(Box::pin(byte_stream))
    }
}

/// TODO: Deal with expected errors.
impl Sftp {
    async fn connect(&self) -> Result<Session, AppError> {
        let tcp = TcpStream::connect(format!("{}:{}", self.host, self.port))
            .await
            .expect("Failed to connect to SFTP server");
        let mut session = Session::new().expect("Failed to create SSH session");
        session.set_tcp_stream(tcp);
        session
            .handshake()
            .expect("Failed to handshake with SSH server");
        session
            .userauth_pubkey_memory(&self.username, None, &self.ssh_key, None)
            .expect("Failed to authenticate with SSH server");
        if !session.authenticated() {
            return Err(AppError::new(
                "Sftp authentication failed",
                StatusCode::UNAUTHORIZED,
            ));
        }

        Ok(session)
    }
}

/// TODO: Deal with expected errors.
#[async_trait]
impl StreamUploader for Sftp {
    async fn upload_stream(&self, mut stream: ByteStream) -> Result<(), AppError> {
        let sess = self
            .connect()
            .await
            .expect("Failed to connect to SFTP server");
        let sftp = sess.sftp().expect("Failed to create SFTP session");
        let mut remote_file = sftp
            .create(Path::new(&self.remote_path))
            .expect("Failed to create remote file");

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.expect("Failed to read chunk");
            remote_file
                .write_all(chunk.chunk())
                .expect("Failed to write chunk");
        }

        remote_file.close().expect("Failed to close remote file");
        Ok(())
    }
}
