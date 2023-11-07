use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new(move || App::new().service(health_check))
        .bind(("127.0.0.1", 8002))?
        .run()
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
