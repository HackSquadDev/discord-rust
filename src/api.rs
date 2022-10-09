use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TeamsResponse {
    pub teams: Vec<Team>,
}

#[derive(Deserialize, Debug)]
pub struct TeamResponse {
    pub team: Team,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Team {
    pub name: String,
    pub score: u32,
    pub slug: String,
    pub prs: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PR {
    pub status: Option<String>,
}

pub async fn get_teams() -> Vec<Team> {
    //TODO: add caching here
    let api_response: TeamsResponse = reqwest::get("https://www.hacksquad.dev/api/leaderboard")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    api_response.teams
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
