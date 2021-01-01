use config::{Config, ConfigError, Environment};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub log_level: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::default();
        c.merge(Environment::with_prefix("TODO"))?;

        c.try_into::<Settings>()
    }
}