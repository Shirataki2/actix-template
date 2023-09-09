use std::sync::Arc;

use actix_web::{App as ActixApp, HttpServer};

use crate::{
    error::Error,
    infrastructure::{config::Settings, data::AppData},
};

pub struct App {
    pub settings: Settings,
    pub data: AppData,
}

#[actix_web::get("/")]
async fn index() -> String {
    String::from("Ready")
}

impl App {
    pub fn new(settings: Settings, data: AppData) -> Self {
        App { settings, data }
    }

    pub async fn run(self) -> Result<(), Error> {
        let data = Arc::new(self.data);
        let server = HttpServer::new(move || {
            ActixApp::new()
                .wrap(actix_web::middleware::Logger::default())
                .app_data(data.clone())
                .service(index)
                .configure(crate::interfaces::controllers::configure)
        })
        .bind(format!("{}:{}", self.settings.host, self.settings.port))?;
        server.run().await?;
        Ok(())
    }
}
