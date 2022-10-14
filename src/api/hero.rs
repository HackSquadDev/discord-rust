use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::database::Database;

#[derive(Serialize, Deserialize, Clone)]
pub struct Hero {
    pub name: Option<String>,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub github: String,
    pub devto: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub discord: Option<String>,
    pub activities_count: Option<u32>,
    pub activities_score: Option<u32>,
    #[serde(rename = "totalPulls")]
    pub total_pulls: Option<u32>,
    pub last_activity_occurred_at: Option<String>,
    pub pulls: Vec<Pulls>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pulls {
    pub url: String,
    pub title: String,
}

#[derive(Deserialize, Serialize)]
pub struct HeroResponse {
    pub list: Vec<Hero>,
}

pub async fn get_random_hero(database: &Database) -> Option<Hero> {
    let hero_list = database
        .request::<HeroResponse>(
            "https://contributors.novu.co/contributors",
            "heros",
            database.config.cache_heros_ttl,
        )
        .await
        .ok()?
        .list;

    Some(
        hero_list
            .choose(&mut rand::thread_rng())
            .expect("No heroes found")
            .to_owned(),
    )
}

pub async fn get_hero(database: &Database, hero_github_id: &str) -> Option<Hero> {
    database
        .request::<Hero>(
            &format!(
                "https://contributors.novu.co/contributor/{}",
                hero_github_id
            ),
            &format!("hero:{}", hero_github_id),
            database.config.cache_hero_ttl,
        )
        .await
        .ok()
}
