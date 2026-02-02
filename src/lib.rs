use std::{env, future::Future};

use log::error;
use vetis::{
    config::{ListenerConfig, ServerConfig, VirtualHostConfig},
    errors::VetisError,
    server::{
        path::HandlerPath,
        virtual_host::{handler_fn, VirtualHost},
    },
    Request, Response, Vetis,
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
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, VetisError>> + Send + Sync + 'static,
    {
        let port = env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse::<u16>()
            .unwrap();
        let interface = env::var("INTERFACE").unwrap_or("0.0.0.0".to_string());

        let listener_config = ListenerConfig::builder()
            .port(port)
            .interface(&interface)
            .build();

        let config = ServerConfig::builder()
            .add_listener(listener_config)
            .build();

        let localhost_config = VirtualHostConfig::builder()
            .hostname("localhost")
            .build()
            .map_err(|e| SofieError::ServerStart(e.to_string()))?;

        let mut virtual_host = VirtualHost::new(localhost_config);
        virtual_host.add_path(HandlerPath::new_host_path("/", handler_fn(handler)));

        let mut server = Vetis::new(config);
        server.add_virtual_host(virtual_host);

        let result = server.run().await;

        if let Err(e) = result {
            error!("Failed to start server: {}", e);
            return Err(SofieError::ServerStart(e.to_string()));
        }

        Ok(())
    }
}
