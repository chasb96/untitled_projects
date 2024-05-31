use std::{env, sync::OnceLock};
use log_unwrap::LogUnwrap;
use serde::Deserialize;

static CONFIGURATION: OnceLock<Configuration> = OnceLock::new();

#[derive(Deserialize)]
pub struct Configuration {
    pub database_url: String,
}

impl Default for &Configuration {
    fn default() -> Self {
        CONFIGURATION
            .get_or_init(|| {
                let database_url = env::var("PROJECTS_DATABASE_URL").log_unwrap();

                Configuration {
                    database_url,
                }
            })
    }
}