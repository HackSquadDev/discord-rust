use serde::{Deserialize, Serialize};

use crate::database::Database;

#[derive(Deserialize, Debug, Serialize)]
pub struct LeaderboardResponse {
    pub teams: Vec<Team>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TeamResponse {
    pub team: Team,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub score: u32,
    pub slug: String,
    pub prs: Option<String>,
    pub users: Option<Vec<User>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
    pub handle: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PR {
    pub status: Option<String>,
    pub title: String,
    pub url: String,
}

pub async fn get_leaderboard(database: &Database) -> Option<Vec<Team>> {
    Some(
        database
            .request::<LeaderboardResponse>(
                "https://www.hacksquad.dev/api/leaderboard",
                "leaderboard",
                database.config.cache_leaderboard_ttl,
            )
            .await
            .ok()?
            .teams,
    )
}

pub async fn get_team(database: &Database, team_id: &String) -> Option<Team> {
    Some(
        database
            .request::<TeamResponse>(
                &format!("https://www.hacksquad.dev/api/team?id={}", team_id),
                &format!("team:{}", team_id),
                database.config.cache_team_ttl,
            )
            .await
            .ok()?
            .team,
    )
}
