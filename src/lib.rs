use std::future::Future;

use log::error;
use vetis::{
    server::{config::ServerConfig, errors::VetisError},
    RequestType, ResponseType, Vetis,
};

use crate::errors::SofieError;

pub mod errors;
mod tests;

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub async fn serve<F, Fut>(&mut self, handler: F) -> Result<(), SofieError>
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

        let result = server
            .run(handler)
            .await;

        if let Err(e) = result {
            error!("Failed to start server: {}", e);
            return Err(SofieError::ServerStart(e.to_string()));
        }

        Ok(())
    }
}
