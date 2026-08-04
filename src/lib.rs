#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::{config::Config, errors::SofieError};
use log::error;
use std::{fs::read_to_string, future::Future, path::Path};
use vetis_tokio::{ListenerConfig, ServerConfig, Vetis};

/// Configuration file name
pub static CONFIG: &str = "sofie.toml";

/// Configuration module
pub mod config;
/// Error types module
pub mod errors;

/// Application struct
pub struct App {
    config: Config,
}

impl App {}
