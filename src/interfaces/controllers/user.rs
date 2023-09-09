use actix_web::HttpResponse;

use crate::{error::NoneIs404, prelude::*};

#[actix_web::get("")]
async fn list_users(req: HttpRequest) -> ApiResponse {
    let db = &get_app_data(&req).pool;
    let users = User::find().all(db).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[actix_web::get("/{id}")]
async fn get_user(req: HttpRequest, id: web::Path<i32>) -> ApiResponse {
    let db = &get_app_data(&req).pool;
    let user = User::find_by_id(id.into_inner())
        .one(db)
        .await?
        .none_is_404()?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/users")
            .service(list_users)
            .service(get_user),
    );
}
