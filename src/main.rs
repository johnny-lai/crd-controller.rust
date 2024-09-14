use actix_web::{
    get, middleware, web::Data, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
pub use controller::State;
use prometheus::{Encoder, TextEncoder};

#[get("/health")]
async fn health(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

#[get("/metrics")]
async fn metrics(c: Data<State>, _req: HttpRequest) -> impl Responder {
    let metrics = c.metrics();
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&metrics, &mut buffer).unwrap();
    HttpResponse::Ok().body(buffer)
}

#[derive(Parser)] // requires `derive` feature
#[command(name = "crd-controller")]
#[command(bin_name = "crd-controller")]
enum Cli {
    Server(ServerArgs),
    Version,
}

#[derive(clap::Args)]
struct ServerArgs {
    /// address and port to listen on
    #[arg(short, long, default_value_t = String::from("0.0.0.0:8080"))]
    listen: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args {
        Cli::Server(server_args) => {
            // Handle the 'Server' subcommand here
            let state = State::default();

            // Start web server
            let server = HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(state.clone()))
                    .wrap(middleware::Logger::default().exclude("/health"))
                    .service(health)
                    .service(metrics)
            })
            .bind(server_args.listen)?
            .shutdown_timeout(5);

            // Both runtimes implements graceful shutdown, so poll until both are done
            tokio::join!(server.run()).0?;
        }
        Cli::Version => {
            // Handle the 'Version' subcommand here
            println!("Version subcommand");
        }
    }

    Ok(())
}
