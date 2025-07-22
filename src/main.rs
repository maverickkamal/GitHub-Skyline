use clap::Parser;

mod api;
mod renderer;

use api::client::GitHubClient;
use renderer::skyline::render_skyline;

#[derive(Parser)]
#[command(name = "github_skyline")]
#[command(about = "Generate a city skyline from your GitHub contributions")]
#[command(version = "0.1.0")]
struct Args {
    username: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("🚀 Generating skyline for GitHub user: {}", args.username);
    
    let client = match GitHubClient::new() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("❌ Error creating GitHub client: {}", e);
            std::process::exit(1);
        }
    };
    
    match client.fetch_contributions(&args.username).await {
        Ok(calendar) => {
            let contributions = calendar.flatten_contributions();
            println!("✅ Fetched {} days of contribution data", contributions.len());
            println!("📊 Total contributions: {}", calendar.total_contributions);
            
            render_skyline(&contributions);
        }
        Err(e) => {
            eprintln!("❌ Error fetching contributions: {}", e);
            std::process::exit(1);
        }
    }
}

