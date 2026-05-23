#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use std::{fs::read_to_string, future::Future, path::Path};

use log::error;
use vetis::{
    config::server::{virtual_host::VirtualHostConfig, ListenerConfig, ServerConfig},
    errors::VetisError,
    server::virtual_host::{handler_fn, path::HandlerPath, VirtualHost},
    Vetis,
};

use crate::{config::Config, errors::SofieError};

/// Configuration file name
pub static CONFIG: &str = "sofie.toml";

/// Configuration module
pub mod config;
/// Error types module
pub mod errors;
/// Tests module
mod tests;

/// Request type alias
pub type Request = vetis::server::http::Request;
/// Response type alias
pub type Response = vetis::server::http::Response;

/// Application struct
pub struct App {
    config: Config,
    server: vetis::Vetis,
}

impl Default for App {
    fn default() -> Self {
        let config = if Path::exists(Path::new(CONFIG)) {
            let file = read_to_string(CONFIG);
            if let Ok(file) = file {
                let config = toml::from_str(&file);
                if let Ok(config) = config {
                    config
                } else {
                    error!("Failed to parse config file");
                    Config::default()
                }
            } else {
                Config::default()
            }
        } else {
            Config::default()
        };

        let port = config.port();
        let interface = config.interface();

        let listener_config = ListenerConfig::builder()
            .port(port)
            .interface(interface)
            .build()
            .expect("Failed to build listener config");

        let server_config = ServerConfig::builder()
            .add_listener(listener_config)
            .build()
            .expect("Failed to build server config");

        App { config, server: Vetis::new(server_config) }
    }
}

impl App {
    /// Start the server with the given handler
    pub async fn serve<H, Fut>(&mut self, handler: H) -> Result<(), SofieError>
    where
        H: Fn(Request) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, VetisError>> + Send + Sync + 'static,
    {
        let localhost_config = VirtualHostConfig::builder()
            .hostname("localhost")
            .port(self.config.port())
            .build()
            .map_err(|e| SofieError::ServerStart(e.to_string()))?;

        let mut virtual_host = VirtualHost::new(localhost_config);
        virtual_host.add_path(
            HandlerPath::builder()
                .uri("/")
                .handler(handler_fn(move |_req| {
                    let value = handler.clone();
                    async move { value(_req).await }
                }))
                .build()
                .map_err(|e| SofieError::ServerStart(e.to_string()))?,
        );

        self.server
            .add_virtual_host(virtual_host)
            .await;

        let result = self
            .server
            .run()
            .await;

        if let Err(e) = result {
            error!("Failed to start server: {}", e);
            return Err(SofieError::ServerStart(e.to_string()));
        }

        Ok(())
    }
}
