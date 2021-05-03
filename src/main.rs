mod handler;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/" ,web::get().to(handler::ip))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}