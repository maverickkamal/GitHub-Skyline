/// GraphQL query to fetch contribution calendar for a GitHub user
pub const CONTRIBUTION_QUERY: &str = r#"
query($userName: String!) {
  user(login: $userName) {
    contributionsCollection {
      contributionCalendar {
        totalContributions
        weeks {
          contributionDays {
            contributionCount
            date
          }
        }
      }
    }
  }
}
"#;

/// Builds the GraphQL request body for the contribution query
pub fn build_query_body(username: &str) -> serde_json::Value {
    serde_json::json!({
        "query": CONTRIBUTION_QUERY,
        "variables": {
            "userName": username
        }
    })
} 