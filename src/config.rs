use serde::Deserialize;

/// Configuration for the Sophia server
#[derive(Deserialize)]
pub struct Config {
    /// Port to listen on
    port: u16,
    /// Interface to bind to
    interface: String,
    /// Optional security configuration
    security: Option<SecurityConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config { port: 5000, interface: "0.0.0.0".to_string(), security: None }
    }
}

impl Config {
    /// Get the port to listen on
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the interface to bind to
    pub fn interface(&self) -> &str {
        &self.interface
    }

    /// Get the security configuration
    pub fn security(&self) -> Option<&SecurityConfig> {
        self.security
            .as_ref()
    }
}

#[derive(Deserialize)]
/// Security configuration for the Sophia server
pub struct SecurityConfig {
    /// Path to the certificate file
    cert_path: String,
    /// Path to the key file
    key_path: String,
}

impl SecurityConfig {
    /// Get the path to the certificate file
    pub fn cert_path(&self) -> &str {
        &self.cert_path
    }

    /// Get the path to the key file
    pub fn key_path(&self) -> &str {
        &self.key_path
    }
}
