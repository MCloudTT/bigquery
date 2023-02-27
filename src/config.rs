use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Redis {
    pub(crate) host: String,
    pub(crate) port: u16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BigQuery {
    pub(crate) project_id: String,
    pub(crate) dataset_id: String,
    pub(crate) table_id: String,
    pub(crate) credentials_path: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Configuration {
    pub(crate) bigquery: BigQuery,
    pub(crate) redis: Redis,
}

impl Configuration {
    pub(crate) fn load() -> Result<Self, config::ConfigError> {
        Ok(Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()?
            .try_deserialize()?)
    }
}
