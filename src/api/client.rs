use reqwest::Client;
use std::env;
use crate::api::types::{GraphQLResponse, ContributionCalendar};
use crate::api::queries::build_query_body;

pub struct GitHubClient {
    client: Client,
    token: String,
}

impl GitHubClient {
   
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let token = env::var("GITHUB_TOKEN")
            .map_err(|_| "GITHUB_TOKEN environment variable not set. Please check token_setup.md")?;
        
        let trimmed_token = token.trim().to_string();
        if trimmed_token.is_empty() {
            return Err("GITHUB_TOKEN is empty".into());
        }
        

        if !trimmed_token.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("Invalid GitHub token format. Token should only contain letters, numbers, and underscores.".into());
        }
        
        let client = Client::new();
        
        Ok(GitHubClient { client, token: trimmed_token })
    }
    

    pub async fn fetch_contributions(&self, username: &str) -> Result<ContributionCalendar, Box<dyn std::error::Error>> {
        let query_body = build_query_body(username);
        
        let response = self.client
            .post("https://api.github.com/graphql")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "github-skyline/0.1.0")
            .json(&query_body)
            .send()
            .await
            .map_err(|e| format!("Network request failed: {}. Please check your internet connection and GitHub token.", e))?;
        
        if !response.status().is_success() {
            return Err(format!("GitHub API request failed with status: {}", response.status()).into());
        }
        
        let graphql_response: GraphQLResponse = response.json().await?;
        

        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.into_iter().map(|e| e.message).collect();
            return Err(format!("GraphQL errors: {}", error_messages.join(", ")).into());
        }
        

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