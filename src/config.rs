use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    port: u16,
    interface: String,
    security: Option<SecurityConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config { port: 5000, interface: "0.0.0.0".to_string(), security: None }
    }
}

impl Config {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn interface(&self) -> &str {
        &self.interface
    }

    pub fn security(&self) -> Option<&SecurityConfig> {
        self.security
            .as_ref()
    }
}

#[derive(Deserialize)]
pub struct SecurityConfig {
    cert_path: String,
    key_path: String,
}
