use std::time::Duration;

use aws_config::meta::region::RegionProviderChain;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::error::Error;

use super::config::Settings;

pub struct AppData {
    pub settings: Settings,
    pub s3_client: aws_sdk_s3::Client,
    pub pool: DatabaseConnection,
}

impl AppData {
    pub async fn create(settings: &Settings) -> Result<Self, Error> {
        let credentials = aws_credential_types::Credentials::from_keys(
            settings.aws_access_key.clone(),
            settings.aws_secret_key.clone(),
            None,
        );
        let s3_config = aws_config::from_env()
            .region(RegionProviderChain::first_try(
                aws_sdk_s3::config::Region::new(settings.s3_region.clone()),
            ))
            .credentials_provider(credentials)
            .load()
            .await;
        let s3_client = aws_sdk_s3::Client::new(&s3_config);

        let mut opts = ConnectOptions::new(&settings.db_url);
        opts.max_connections(settings.db_max_connections)
            .min_connections(settings.db_min_connections)
            .connect_timeout(Duration::from_secs(settings.db_connection_timeout))
            .acquire_timeout(Duration::from_secs(settings.db_acquire_timeout))
            .sqlx_logging(settings.db_logging)
            .sqlx_logging_level(settings.db_log_level);
        let pool = Database::connect(opts)
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(AppData {
            settings: settings.clone(),
            s3_client,
            pool,
        })
    }
}
