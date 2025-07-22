use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphQLResponse {
    pub data: Option<Data>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GraphQLError {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub user: Option<User>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "contributionsCollection")]
    pub contributions_collection: ContributionsCollection,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContributionsCollection {
    #[serde(rename = "contributionCalendar")]
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContributionCalendar {
    #[serde(rename = "totalContributions")]
    pub total_contributions: u32,
    pub weeks: Vec<Week>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Week {
    #[serde(rename = "contributionDays")]
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContributionDay {
    #[serde(rename = "contributionCount")]
    pub contribution_count: u32,
    pub date: String,
}

impl ContributionCalendar {
    /// Flattens the weeks and days into a single vector of contribution counts
    pub fn flatten_contributions(&self) -> Vec<u32> {
        self.weeks
            .iter()
            .flat_map(|week| &week.contribution_days)
            .map(|day| day.contribution_count)
            .collect()
    }
} 