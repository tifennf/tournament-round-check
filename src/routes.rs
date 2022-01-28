use std::time::Duration;

use axum::extract::Extension;
use reqwest::Client;
use tokio::time::sleep;
use tracing::log::debug;

use crate::{utils::unregister_player, Player, State, Tournament};

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
