use rust_ddd as app;

#[actix_web::main]
async fn main() -> Result<(), app::Error> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    app::run().await
}
