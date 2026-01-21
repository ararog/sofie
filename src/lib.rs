use std::future::Future;

use log::error;
use vetis::{RequestType, ResponseType, Vetis, server::{config::ServerConfig, errors::VetisError}};

use crate::errors::SophiaError;

pub mod errors;

pub struct Sophia {}

impl Sophia {
    pub fn new() -> Sophia {
        Sophia {}
    }

    pub async fn serve<F, Fut>(&mut self, handler: F) -> Result<(), SophiaError>
    where
        F: Fn(RequestType) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<ResponseType, VetisError>> + Send + 'static,
    {
        let port = 8080;
        let interface = "::".to_string();

        let config = ServerConfig::builder()
            .port(port)
            .interface(interface)
            .build();

        let mut server = Vetis::new(config);

        let result =server
            .run(handler)
            .await;

        if let Err(e) = result {
            error!("Failed to start server: {}", e);
            return Err(SophiaError::ServerStart(e.to_string()));
        }

        Ok(())
    }
}
