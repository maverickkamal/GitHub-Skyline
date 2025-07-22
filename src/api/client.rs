use reqwest::Client;
use std::env;
use crate::api::types::{GraphQLResponse, ContributionCalendar};
use crate::api::queries::build_query_body;

pub struct GitHubClient {
    client: Client,
    token: String,
}

impl GitHubClient {
    /// Creates a new GitHub API client with authentication
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let token = env::var("GITHUB_TOKEN")
            .map_err(|_| "GITHUB_TOKEN environment variable not set. Please check token_setup.md")?;
        
        let client = Client::new();
        
        Ok(GitHubClient { client, token })
    }
    
    /// Fetches contribution calendar data for a given username
    pub async fn fetch_contributions(&self, username: &str) -> Result<ContributionCalendar, Box<dyn std::error::Error>> {
        let query_body = build_query_body(username);
        
        let response = self.client
            .post("https://api.github.com/graphql")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "github-skyline/0.1.0")
            .json(&query_body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(format!("GitHub API request failed with status: {}", response.status()).into());
        }
        
        let graphql_response: GraphQLResponse = response.json().await?;
        
        // Check for GraphQL errors
        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.into_iter().map(|e| e.message).collect();
            return Err(format!("GraphQL errors: {}", error_messages.join(", ")).into());
        }
        
        // Extract the contribution calendar
        let calendar = graphql_response
            .data
            .ok_or("No data in response")?
            .user
            .ok_or("User not found")?
            .contributions_collection
            .contribution_calendar;
        
        Ok(calendar)
    }
} 