#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;

pub mod application;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod interfaces;
pub mod prelude;

pub use application::App;
pub use error::Error;

pub async fn run() -> Result<(), Error> {
    let settings =
        infrastructure::config::Settings::new().map_err(|e| Error::Configuration(e.to_string()))?;
    let data = infrastructure::data::AppData::create(&settings).await?;
    let app = application::App::new(settings, data);
    app.run().await
}
