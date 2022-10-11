use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{CONFIG, DATABASE};

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
    pub totalPulls: u32,
    pub last_activity_occurred_at: String,
    pub pulls: Vec<Pulls>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pulls{
    pub url: String,
    pub title: String,
}

#[derive(Deserialize, Serialize)]
pub struct HeroResponse {
    pub list: Vec<Hero>,
}

pub async fn get_random_hero() -> Hero {
    let db = DATABASE.lock().await;

    let hero_list = db
        .request::<HeroResponse>(
            "https://contributors.novu.co/contributors",
            "heros",
            CONFIG.lock().await.cache_heros_ttl,
        )
        .await
        .list;

    hero_list
        .choose(&mut rand::thread_rng())
        .expect("No heroes found")
        .to_owned()
}

pub async fn get_hero(hero_github_id: &str) -> Hero {
    let db = DATABASE.lock().await;

    db.request::<Hero>(
        &format!(
            "https://contributors.novu.co/contributor/{}",
            hero_github_id
        ),
        &format!("hero:{}", hero_github_id),
        CONFIG.lock().await.cache_hero_ttl,
    )
    .await
}