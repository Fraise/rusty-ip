mod handler;
mod config;
use config::Config;

use actix_web::{App, HttpServer, web};
use clap::{Clap, ValueHint};

#[derive(Clap, Debug)]
#[clap(name = "rusty_ip")]
struct CliArgs {
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    config: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Config;
    let args = CliArgs::parse();

    match args.config {
        Some(conf) => {
            config = match config::parse(&conf) {
                Ok(c) => c,
                Err(e) => {
                    println!("could not parse the configuration file: {}", e);
                    return Ok(());
                }
            };
        },
        None => {
            config = config::default();
            // println!("no configuration file provided, using default:\n{}", config);
        }
    }


    HttpServer::new(|| {
        App::new()
            .route("/" ,web::get().to(handler::ip))
    })
    .bind(config.listen_address + ":" + config.listen_port.to_string().as_str())?
    .run()
    .await
}
