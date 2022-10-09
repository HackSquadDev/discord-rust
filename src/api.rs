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
    let mut conn = DATABASE.lock().await.get_connection();
    let redis_leaderboard: Result<String, RedisError> = conn.get("leaderboard");

    match redis_leaderboard {
        Ok(data) => serde_json::from_str(&data).unwrap(),
        Err(_) => {
            //TODO: save to redis
            println!("APIIIII");
            let api_response: TeamsResponse =
                reqwest::get("https://www.hacksquad.dev/api/leaderboard")
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

            redis::pipe()
                .cmd("SET")
                .arg("leaderboard")
                .arg(json!(api_response.teams).to_string())
                .execute(&mut conn);

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
