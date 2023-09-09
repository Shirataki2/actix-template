use std::sync::Arc;

use actix_web::HttpRequest;

use crate::infrastructure::data::AppData;

pub mod user;

pub fn get_app_data(req: &HttpRequest) -> &Arc<AppData> {
    req.app_data::<Arc<AppData>>().unwrap()
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    user::configure(cfg);
}
