use clap::Parser;
mod api;
mod renderer;
mod cli;
mod output;
mod achievements;
use api::client::GitHubClient;
use renderer::skyline::render_skyline;
use cli::interactive::{show_splash_screen, interactive_mode};
use output::{render_skyline_to_string, save_skyline_to_file};


#[derive(Parser)]
#[command(name = "github_skyline")]
#[command(about = "Generate a city skyline from your GitHub contributions")]
#[command(version = "0.1.0")]
struct Args {
   
    #[arg(help = "GitHub username")]
    username: Option<String>,
    
   
    #[arg(long, help = "Color theme for the skyline (synthwave, dracula, solarized, cyberpunk, matrix, sunset, random)")]
    theme: Option<String>,
    
   
    #[arg(long, help = "Skip interactive mode")]
    no_interactive: bool,
    
   
    #[arg(short, long, help = "Output file to save the skyline (e.g., skyline.txt)")]
    output: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
   
    if args.username.is_none() && !args.no_interactive {
        show_splash_screen();
       
        loop {
            interactive_mode().await;
        }
    }
    
   
    let username = match args.username {
        Some(u) => u,
        None => {
            eprintln!("❌ Username required in CLI mode. Use --help for more info.");
            std::process::exit(1);
        }
    };
    
    let theme = args.theme.unwrap_or_else(|| "synthwave".to_string());
    
    println!("🚀 Generating skyline for GitHub user: {}", username);
    
    let client = match GitHubClient::new() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("❌ Error creating GitHub client: {}", e);
            eprintln!("💡 Tip: Run without arguments for interactive setup!");
            std::process::exit(1);
        }
    };
    
    match client.fetch_contributions(&username).await {
        Ok(calendar) => {
            let contributions = calendar.flatten_contributions();
            println!("✅ Fetched {} days of contribution data", contributions.len());
            println!("📊 Total contributions: {}", calendar.total_contributions);
            
            if let Some(output_file) = args.output {
               
                let (skyline_content, total_contribs) = render_skyline_to_string(&contributions, &theme, &username);
                match save_skyline_to_file(&skyline_content, &output_file, &username, &theme, total_contribs) {
                    Ok(_) => println!("🎉 Skyline generation complete!"),
                    Err(e) => eprintln!("❌ Error saving to file: {}", e),
                }
            } else {
                
                render_skyline(&contributions, &theme);
            }
        }
        Err(e) => {
            eprintln!("❌ Error fetching contributions: {}", e);
            std::process::exit(1);
        }
    }
}

