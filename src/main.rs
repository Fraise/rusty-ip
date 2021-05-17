mod handler;
mod config;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match config::parse("rusty-ip.conf") {
        Ok(c) => c,
        Err(e) => {
            println!("could not parse the configuration file: {}", e);
            return Ok(());
        }
    };

    HttpServer::new(|| {
        App::new()
            .route("/" ,web::get().to(handler::ip))
    })
    .bind(config.listen_address + ":" + config.listen_port.to_string().as_str())?
    .run()
    .await
}
