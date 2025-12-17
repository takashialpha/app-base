use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub network: NetworkConfig,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub target: String,
    pub timeout_ms: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                target: "8.8.8.8".to_string(),
                timeout_ms: 1000,
            },
        }
    }
}
