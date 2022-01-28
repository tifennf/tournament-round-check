use std::{collections::HashMap, sync::Arc, time::Duration};

use axum::{
    extract::{Extension, Path},
    Json,
};

use reqwest::{Client, StatusCode};
use serde_json::Value;
use tokio::{sync::Mutex, time::sleep};
use tracing::log::debug;

use crate::{utils::unregister_player, ApiRes, Player, State, Tournament};

pub async fn info(Extension(state): Extension<SharedState>) -> Result<Json<State>, StatusCode> {
    let state = state.lock().await;

    Ok(Json(state.clone()))
}

type SharedState = Arc<Mutex<State>>;

pub async fn start(
    Extension(state): Extension<SharedState>,
    Path(time): Path<u64>,
) -> Result<(StatusCode, String), StatusCode> {
    let c_state = state.clone();

    let mut state = state.lock().await;
    state.on_check = true;

    let client = Client::new();
    let res = client
        .get("http://localhost:3024/info")
        .send()
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    let res = res.json::<ApiRes>().await.map_err(|err| {
        println!("{}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let player_list = res.data.player_list.list;

    let duration = Duration::from_secs(time);

    state.player_list = player_list;

    tokio::spawn(async move {
        sleep(duration).await;

        let mut state = c_state.lock().await;

        for player in state.player_list.iter() {
            let res = unregister_player(&client, player.discord_id.clone()).await;

            println!("{:#?}", res);

            if let Err(err) = res {
                println!("Could not unregister player: {}", err);
            }
        }

        state.on_check = false;
    });

    let res = (
        StatusCode::OK,
        format!("Check-in started, duration: {} seconds", duration.as_secs()),
    );

    Ok(res)
}

pub async fn check(
    Extension(state): Extension<SharedState>,
    Path(discord_id): Path<String>,
) -> (StatusCode, String) {
    let mut state = state.lock().await;

    let base_len = state.player_list.len();

    let player_list: Vec<Player> = state
        .player_list
        .clone()
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
