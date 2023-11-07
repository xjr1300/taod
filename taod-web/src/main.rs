use actix_web::{web, App, HttpServer};

use db::connection_pool;
use taod_web::handlers::{accident_list, health_check};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = connection_pool().await?;

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::scope("/api")
                .route("/health-check", web::get().to(health_check))
                .route("/accidents", web::get().to(accident_list)),
        )
    })
    .bind(("127.0.0.1", 8002))?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
