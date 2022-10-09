use redis::{Commands, RedisError};
use redis_derive::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::DATABASE;

#[derive(Deserialize, Debug)]
pub struct TeamsResponse {
    pub teams: Vec<Team>,
}

#[derive(Deserialize, Debug)]
pub struct TeamResponse {
    pub team: Team,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub score: u32,
    pub slug: String,
    pub prs: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PR {
    pub status: Option<String>,
}

pub async fn get_teams() -> Vec<Team> {
    let db = DATABASE.lock().await;

    let redis_leaderboard: Result<String, RedisError> = db.get("leaderboard");

    match redis_leaderboard {
        Ok(data) => serde_json::from_str(&data).unwrap(),
        Err(_) => {
            let api_response: TeamsResponse =
                reqwest::get("https://www.hacksquad.dev/api/leaderboard")
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            db.save("leaderboard", &json!(api_response.teams).to_string(), 60);

            api_response.teams
        }
    }
}

pub async fn get_team(team_id: &String) -> Team {
    //TODO: add caching here
    let api_response: TeamResponse = reqwest::get(&format!(
        "https://www.hacksquad.dev/api/team?id={}",
        team_id
    ))
    .await
    .unwrap()
    .json()
    .await
    .unwrap();

    api_response.team
}
