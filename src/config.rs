//! # Config
//!
//! App configuration

use std::path::PathBuf;

/// App configuration read from environment
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub email_sender: String,
    pub listen_url: String,
    #[cfg(target_family = "unix")]
    pub pidfile: Option<PathBuf>,
}

impl Config {
    pub fn try_from_env() -> anyhow::Result<Self> {
        envy::from_env()
            .map_err(|e| anyhow::anyhow!("could not load config from environment: {}", e))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_parse_config_from_env() {
        assert!(Config::try_from_env().is_ok());
    }
}
