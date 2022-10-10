use serde::{Deserialize, Serialize};
use serde_json::json;

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

#[derive(Serialize, Deserialize)]
pub struct Hero {
    pub name: String,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub github: String,
    pub devto: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub discord: Option<String>,
    pub activities_count: u32,
    pub activities_score: u32,
}

#[derive(Deserialize)]
pub struct HeroResponse {
    pub list: Vec<Hero>,
}

pub async fn get_heroes() -> Vec<Hero> {
    let db = DATABASE.lock().await;

    let redis_heroes = db.get("heroes");

    match redis_heroes {
        Ok(heroes) => {
            let heroes: Vec<Hero> = serde_json::from_str(&heroes).unwrap();
            heroes
        }
        Err(_) => {
            let heroes: HeroResponse = reqwest::get("https://contributors.novu.co/contributors")
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            db.save("heroes", &json!(heroes.list).to_string());

            heroes.list
        }
    }
}

pub async fn get_hero(hero_github_id: &str) -> Hero {
    let db = DATABASE.lock().await;

    let redis_hero = db.get(&format!("hero-{}", hero_github_id));

    match redis_hero {
        Ok(data) => serde_json::from_str(&data).unwrap(),
        Err(_) => {
            let hero: Hero = reqwest::get(&format!(
                "https://contributors.novu.co/contributor/{}",
                hero_github_id
            ))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

            db.save(
                &format!("hero-{}", hero_github_id),
                &json!(hero).to_string(),
            );

            hero
        }
    }
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

pub async fn get_team(team_id: &String) -> Team {
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
