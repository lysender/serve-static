use std::path::PathBuf;
use std::{process, net::SocketAddr};
use axum::http::{Request, Method};
use axum::middleware;
use axum::Router;
use axum::routing::get_service;
use clap::Parser;
use log::info;

use config::{Args, Config};
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;

mod config;
mod error;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let mut routes_all = Router::new()
        .merge(routes_static(&config.dir))
        .layer(middleware::map_request(main_request_mapper));


    if config.cors {
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(Any);
        routes_all = routes_all.layer(cors).to_owned();
    }

    // Setup the server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_static(dir: &PathBuf) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(dir)))
}

async fn main_request_mapper<B>(req: Request<B>) -> Request<B> {
    info!("{} {}", req.method(), req.uri().path());
    req
}

