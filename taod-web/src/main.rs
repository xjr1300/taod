use std::net::TcpListener;

use actix_web::{middleware::ErrorHandlers, web, App, HttpServer};

use db::connection_pool;
use taod_web::{
    handlers::{accident_list, accident_list_geojson, default_error_handler, health_check},
    settings::get_settings,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let settings = get_settings().map_err(|e| anyhow::anyhow!(e))?;
    let pool = connection_pool().await?;

    let address = format!("{}:{}", settings.web_app.host, settings.web_app.port);
    let listener = TcpListener::bind(address)?;

    HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandlers::new().default_handler_client(default_error_handler))
            .app_data(web::Data::new(settings.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .route("/health-check", web::get().to(health_check))
                    .route("/accidents/{z}/{x}/{y}", web::get().to(accident_list))
                    .route(
                        "/accidents-geojson/{z}/{x}/{y}",
                        web::get().to(accident_list_geojson),
                    ),
            )
    })
    .listen(listener)?
    .run()
    .await
    .map_err(|e| anyhow::anyhow!(e))
}
