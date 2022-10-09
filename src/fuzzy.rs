use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use simsearch::SimSearch;

#[derive(Deserialize, Debug)]
struct Response {
    teams: Vec<Team>,
}

#[derive(Deserialize, Debug, Clone)]
struct Team {
    name: String,
    slug: String,
}

#[derive(Serialize, Debug)]
struct Suggestion {
    name: String,
    value: String,
}

pub async fn search_teams(query: Option<Value>) -> Value {
    if let Some(query) = query {
        let api_response: Response = reqwest::get("https://www.hacksquad.dev/api/leaderboard")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let mut engine = SimSearch::new();

        for team in &api_response.teams {
            engine.insert(team.slug.clone(), &team.name);
        }

        let mut res = engine.search(&query.to_string());
        res.truncate(10);

        let iter = res.iter_mut();

        let mut suggestions: Vec<Team> = Vec::new();

        for (_index, slug) in iter.enumerate() {
            let team = api_response
                .teams
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
