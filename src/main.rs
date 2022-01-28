mod routes;
pub mod utils;

use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, AddExtensionLayer, Router};
use routes::{check, info, start};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tournament-round-check=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3025));

    let state = Arc::new(Mutex::new(State {
        on_check: false,
        player_list: Vec::new(),
    }));

    let middlewares_package = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    let app = Router::new()
        .route("/check/:id", get(check))
        .route("/start/:time", get(start))
        .route("/info", get(info))
        .layer(middlewares_package);

    tracing::debug!("Listening on address: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Clone, Serialize)]
pub struct State {
    pub on_check: bool,
    pub player_list: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub league_name: String,
    pub discord_name: DiscordName,
    pub discord_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordName {
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRes {
    pub status: u16,
    pub data: Tournament,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub player_list: PlayerList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerList {
    pub list: Vec<Player>,
}
