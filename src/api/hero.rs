use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::DATABASE;

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
}

#[derive(Deserialize)]
pub struct HeroResponse {
    pub list: Vec<Hero>,
}

pub async fn get_random_hero() -> Hero {
    let db = DATABASE.lock().await;

    let redis_heroes = db.get("heroes");

    let hero_list = match redis_heroes {
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
    };

    hero_list
        .choose(&mut rand::thread_rng())
        .expect("No heroes found")
        .to_owned()
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
