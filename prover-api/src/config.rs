use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(rename = "port")]
    pub server_port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ::config::ConfigError> {
        dotenvy::dotenv().ok();

        ::config::Config::builder()
            .set_default("port", 8000)?
            .add_source(::config::Environment::default().try_parsing(true))
            .build()?
            .try_deserialize()
    }
}
