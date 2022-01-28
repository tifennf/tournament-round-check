use std::time::Duration;

use axum::{
    extract::{Extension, Path},
    Json,
};
use reqwest::{Client, StatusCode};
use tokio::time::sleep;
use tracing::log::debug;

use crate::{utils::unregister_player, Player, State, Tournament};

pub async fn info(Extension(state): Extension<State>) -> Json<State> {
    Json(state)
}

pub async fn start(Extension(mut state): Extension<State>) -> String {
    state.on_check = true;

    let duration = Duration::from_secs(600);
    let client = Client::new();

    let res = client
        .get("http://localhost:3024/info")
        .send()
        .await
        .unwrap();
    let player_list = res.json::<Tournament>().await.unwrap().player_list;

    state.player_list = player_list;

    tokio::spawn(async move {
        sleep(duration).await;

        for player in state.player_list {
            let res = unregister_player(&client, player.discord_id).await;

            if let Err(err) = res {
                debug!("Could not unregister player: {}", err);
            }
        }

        state.on_check = false;
    });
    format!("Check-in started, duration: {} seconds", duration.as_secs())
}

pub async fn check(
    Extension(mut state): Extension<State>,
    Path(discord_id): Path<String>,
) -> (StatusCode, String) {
    let base_len = state.player_list.len();

    let player_list: Vec<Player> = state
        .player_list
        .into_iter()
        .filter(|p| p.discord_id != discord_id)
        .collect();

    if player_list.len() < base_len {
        state.player_list = player_list;

        (StatusCode::OK, "Player validated his check-in".to_string())
    } else {
        (StatusCode::BAD_REQUEST, "Player not found".to_string())
    }
}
