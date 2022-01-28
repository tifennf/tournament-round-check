use reqwest::{Client, Error, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct DummyPlayer {
    league_name: String,
    discord_username: String,
    tag: u16,
    pub discord_id: String,
    riot_account_id: String,
    puuid: String,
}

impl DummyPlayer {
    fn new(discord_id: String) -> Self {
        let dum = DummyPlayer {
            league_name: "xxx".to_string(),
            discord_username: "xxx".to_string(),
            tag: 0,
            discord_id,
            riot_account_id: "xxx".to_string(),
            puuid: "xxx".to_string(),
        };

        dum
    }
}

pub async fn unregister_player(client: &Client, discord_id: String) -> Result<Response, Error> {
    let player = DummyPlayer::new(discord_id);

    client
        .delete("http://localhost:3024/inscriptions")
        .json(&player)
        .send()
        .await
}
