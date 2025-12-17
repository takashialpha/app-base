use std::fmt;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

mod settings;
pub use settings::Settings;

#[derive(Debug)]
pub enum ConfigError {
    ConfigDirNotFound,
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ConfigDirNotFound => {
                write!(f, "configuration directory not found")
            }
            ConfigError::Io(err) => {
                write!(f, "I/O error while handling configuration: {}", err)
            }
            ConfigError::Parse(err) => {
                write!(f, "failed to parse configuration file: {}", err)
            }
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(err) => Some(err),
            ConfigError::Parse(err) => Some(err),
            _ => None,
        }
    }
}

pub fn load(config_arg_path: Option<PathBuf>) -> Result<Settings, ConfigError> {
    let default_path = config_path()?;
    if !default_path.exists() {
        create_default_config(&default_path)?;
    }

    let path = config_arg_path.unwrap_or(default_path);

    let contents = fs::read_to_string(&path).map_err(ConfigError::Io)?;
    let settings = toml::from_str(&contents).map_err(ConfigError::Parse)?;
    Ok(settings)
}

fn config_path() -> Result<PathBuf, ConfigError> {
    let base = dirs::config_dir().ok_or(ConfigError::ConfigDirNotFound)?;
    Ok(base.join("net-probe").join("config.toml"))
}

fn create_default_config(path: &PathBuf) -> Result<(), ConfigError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(ConfigError::Io)?;
    }

    let defaults = Settings::default();
    let mut file = fs::File::create(path).map_err(ConfigError::Io)?;

    let contents = format!(
        r#"# net-probe configuration file
#
# This file was generated automatically.
# Edit values carefully. Invalid TOML will cause startup failure.
#
# Network probe settings
[network]
# Target host or IP address to probe
target = "{target}"

# Timeout for the probe in milliseconds
timeout_ms = {timeout}
"#,
        target = defaults.network.target,
        timeout = defaults.network.timeout_ms
    );

    file.write_all(contents.as_bytes())
        .map_err(ConfigError::Io)?;

    Ok(())
}
