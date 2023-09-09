pub use crate::domain::models::user::Model as UserModel;

pub use crate::domain::models::{prelude::*, *};
pub use crate::error::Error;
pub use crate::infrastructure::config::Settings;
pub use crate::infrastructure::data::AppData;
pub use crate::interfaces::controllers::get_app_data;

pub use actix_web::{web, HttpRequest, HttpResponse};
pub use sea_orm::prelude::*;

pub type ApiResponse = Result<HttpResponse, Error>;
