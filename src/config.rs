use anyhow::Result;
use config::File;
use serde::Deserialize;

const DEFAULT_CSV_NAME: &str = "_logs.csv";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub es_url: String,
    pub es_id: String,
    pub es_pw: String,
    pub indices: Vec<String>,
    pub start_time: String,
    pub end_time: String,
    pub query_size: usize,
    pub save_location: String,
    pub csv_name: String,
}

impl Config {
    pub fn new(path: Option<&str>) -> Result<Self> {
        let builder = config::Config::builder().set_default("csv_name", DEFAULT_CSV_NAME)?;

        let config: Config = if let Some(path) = path {
            builder.add_source(File::with_name(path))
        } else {
            builder
        }
        .build()?
        .try_deserialize()?;

        Ok(Self {
            es_url: config.es_url,
            es_id: config.es_id,
            es_pw: config.es_pw,
            indices: config.indices,
            start_time: config.start_time,
            end_time: config.end_time,
            query_size: config.query_size,
            save_location: config.save_location,
            csv_name: config.csv_name,
        })
    }
}
