use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::DATABASE;

#[derive(Deserialize, Debug)]
pub struct TeamsResponse {
    pub teams: Vec<Team>,
}

#[derive(Deserialize, Debug)]
pub struct TeamResponse {
    pub team: TeamWithUser,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub score: u32,
    pub slug: String,
    pub prs: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamWithUser {
    pub name: String,
    pub score: u32,
    pub slug: String,
    pub prs: Option<String>,
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PR {
    pub status: Option<String>,
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub handle: String,
}

pub async fn get_teams() -> Vec<Team> {
    let db = DATABASE.lock().await;

    let redis_leaderboard = db.get("leaderboard");

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

            db.save("leaderboard", &json!(api_response.teams).to_string());

            api_response.teams
        }
    }
}

pub async fn get_team(team_id: &String) -> TeamWithUser {
    let db = DATABASE.lock().await;

    let redis_team = db.get(&format!("team:{}", team_id));

    match redis_team {
        Ok(data) => serde_json::from_str(&data).unwrap(),
        Err(_) => {
            let api_response: TeamResponse = reqwest::get(&format!(
                "https://www.hacksquad.dev/api/team?id={}",
                team_id
            ))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

            db.save(
                &format!("team:{}", team_id),
                &json!(api_response.team).to_string(),
            );

            api_response.team
        }
    }
}
