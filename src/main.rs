use std::path::PathBuf;
use std::{process, net::SocketAddr};
use axum::http::Method;
use axum::Router;
use axum::routing::get_service;
use clap::Parser;
use tracing::info;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;
use tower::ServiceBuilder;
use tracing::Level;
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};

use config::{Args, Config};

mod config;
mod error;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "serve_static=info,tower_http=info",
        )
    }

    // tracing_subscriber::fmt::init();
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = Args::parse();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let mut routes_all = Router::new()
        .merge(routes_static(&config.dir))
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
            )
        );


    if config.cors {
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(Any);
        routes_all = routes_all.layer(cors).to_owned();
    }

    // Setup the server
    let ip = match config.public {
        true => [0, 0, 0, 0],
        false => [127, 0, 0, 1],
    };
    let addr = SocketAddr::from((ip, config.port));
    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_static(dir: &PathBuf) -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(dir)))
}

