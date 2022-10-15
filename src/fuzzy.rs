use serde::Serialize;
use serde_json::{json, Value};
use simsearch::{SearchOptions, SimSearch};

use crate::{
    api::{
        hero::{get_all_heros, Hero},
        team::{get_leaderboard, Team},
    },
    database::Database,
};

#[derive(Serialize, Debug)]
struct Suggestion {
    name: String,
    value: String,
}

pub async fn search_teams(database: &Database, query: Option<Value>) -> Value {
    if let Some(query) = query {
        let leaderboard = match get_leaderboard(database).await {
            Some(leader) => leader,
            None => {
                return json!({
                    "error": "Failed to get leaderboard"
                })
            }
        };

        let mut engine =
            SimSearch::new_with(SearchOptions::new().case_sensitive(false).threshold(0.82));

        for team in &leaderboard {
            engine.insert(team.slug.clone(), &team.name);
        }

        let query: String = serde_json::from_value(query.clone()).unwrap();

        let mut res = engine.search(&query);

        if res.is_empty() {
            for team in &leaderboard {
                res.push(team.slug.clone())
            }

            if query.is_empty() {}

            res.retain(|x| x.starts_with(&query));
        }

        res.truncate(10);

        let iter = res.iter_mut();

        let mut suggestions: Vec<Team> = Vec::new();

        for (_index, slug) in iter.enumerate() {
            let team = leaderboard
                .iter()
                .find(|&p| p.slug == slug.clone())
                .cloned();

            if let Some(team) = team {
                suggestions.push(team);
            }
        }

        let suggestions: Vec<Suggestion> = suggestions
            .iter()
            .map(|x| Suggestion {
                value: x.slug.clone(),
                name: x.name.clone(),
            })
            .collect();

        json!(suggestions)
    } else {
        json!({})
    }
}

pub async fn search_hero(database: &Database, query: Option<Value>) -> Value {
    if let Some(query) = query {
        let hero_list = match get_all_heros(database).await {
            Some(hero_list) => hero_list,
            None => {
                return json!({
                    "error": "Failed to get hero list"
                })
            }
        };

        let mut engine =
            SimSearch::new_with(SearchOptions::new().case_sensitive(false).threshold(0.82));

        for hero in &hero_list {
            engine.insert(hero.github.clone(), &hero.github);
        }

        let query: String = serde_json::from_value(query.clone()).unwrap();

        let mut res = engine.search(&query);

        if res.is_empty() {
            for hero in &hero_list {
                res.push(hero.github.clone())
            }

            if query.is_empty() {}

            res.retain(|x| x.starts_with(&query));
        }

        res.truncate(10);

        let iter = res.iter_mut();

        let mut suggestions: Vec<Hero> = Vec::new();

        for (_index, slug) in iter.enumerate() {
            let hero = hero_list
                .iter()
                .find(|&p| p.github == slug.clone())
                .cloned();

            if let Some(hero) = hero {
                suggestions.push(hero);
            }
        }

        let suggestions: Vec<Suggestion> = suggestions
            .iter()
            .map(|x| Suggestion {
                value: x.github.clone(),
                name: x.github.clone(),
            })
            .collect();

        json!(suggestions)
    } else {
        json!({})
    }
}
