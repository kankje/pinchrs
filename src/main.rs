mod encode;
mod fetcher;
mod operation;
mod params;
mod routes;
mod signature;
mod util;

use crate::fetcher::Fetcher;
use crate::fetcher::web::WebFetcher;
use crate::routes::health::health;
use crate::routes::process::process;
use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use reqwest::Url;
use std::env;
use std::sync::Arc;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Clone)]
struct AppState {
    key: Option<Arc<str>>,
    web_fetcher: Arc<WebFetcher>,
}

impl AppState {
    pub fn resolve_fetcher(&self, url: &str) -> Option<Arc<impl Fetcher>> {
        let url = Url::parse(url).ok()?;
        match url.scheme() {
            "http" | "https" => Some(self.web_fetcher.clone()),
            _ => None,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| {
                    EnvFilter::try_new(format!("{}=info,tower_http=warn", env!("CARGO_CRATE_NAME")))
                })
                .unwrap(),
        )
        .init();

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());

    let key = env::var("KEY")
        .ok()
        .filter(|key| !key.is_empty())
        .map(|key| Arc::from(key.into_boxed_str()));
    if key.is_none() {
        warn!(
            "Signature checking is disabled and this server is vulnerable to denial-of-service \
             attacks. Set the KEY environment variable to enable signature checking."
        );
    }

    let app = Router::new()
        .route("/healthz", get(health))
        .route("/{signature}/{*rest}", get(process))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {
            key,
            web_fetcher: Arc::new(WebFetcher::new()),
        });

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();

    info!(
        "{} {} listening on {host}:{port}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
