use config::{Config, ConfigError, File};
use log::LevelFilter;

#[derive(Debug, Clone)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub s3_bucket: String,
    pub s3_region: String,
    pub aws_access_key: String,
    pub aws_secret_key: String,
    pub db_url: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
    pub db_connection_timeout: u64,
    pub db_acquire_timeout: u64,
    pub db_logging: bool,
    pub db_log_level: LevelFilter,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .build()?;
        let db_log_level =
            Self::to_log_level(&s.get_string("db.log_level").unwrap_or("info".to_string()));
        Ok(Settings {
            host: s.get_string("host").unwrap_or("localhost".to_string()),
            port: s.get_int("port").unwrap_or(8080) as u16,
            s3_bucket: s.get_string("s3.bucket")?,
            s3_region: s.get_string("s3.region")?,
            aws_access_key: s.get_string("aws.access_key")?,
            aws_secret_key: s.get_string("aws.secret_key")?,
            db_url: s.get_string("db.url")?,
            db_max_connections: s.get_int("db.max_connections").unwrap_or(5) as u32,
            db_min_connections: s.get_int("db.min_connections").unwrap_or(1) as u32,
            db_connection_timeout: s.get_int("db.connection_timeout").unwrap_or(5000) as u64,
            db_acquire_timeout: s.get_int("db.acquire_timeout").unwrap_or(5000) as u64,
            db_logging: s.get_bool("db.logging").unwrap_or(false),
            db_log_level,
        })
    }

    fn to_log_level(level: &str) -> LevelFilter {
        match level.to_lowercase().as_str() {
            "off" => LevelFilter::Off,
            "error" => LevelFilter::Error,
            "warn" => LevelFilter::Warn,
            "info" => LevelFilter::Info,
            "debug" => LevelFilter::Debug,
            "trace" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Settings;

    #[test]
    fn test_load_default_config() {
        println!("{:?}", Settings::new().unwrap());
    }
}
