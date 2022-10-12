use serde::Serialize;
use serde_json::{json, Value};
use simsearch::{SearchOptions, SimSearch};

use crate::api::team::{get_leaderboard, Team};

#[derive(Serialize, Debug)]
struct Suggestion {
    name: String,
    value: String,
}

pub async fn search_teams(query: Option<Value>) -> Value {
    if let Some(query) = query {
        let leaderboard = get_leaderboard().await;

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
