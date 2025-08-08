use colored::*;
use std::io::{self, Write};
use std::env;
use crate::api::client::GitHubClient;
use crate::renderer::skyline::render_skyline_with_options;

pub fn show_splash_screen() {
    println!("{}", "╔══════════════════════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║                                                                              ║".bright_cyan());
    println!("{}", "║  ░██████╗░██╗████████╗██╗░░██╗██╗░░░██╗██████╗░  ░██████╗██╗░░██╗██╗░░░██╗  ║".bright_magenta().bold());
    println!("{}", "║  ██╔════╝░██║╚══██╔══╝██║░░██║██║░░░██║██╔══██╗  ██╔════╝██║░██╔╝╚██╗░██╔╝  ║".bright_magenta().bold());
    println!("{}", "║  ██║░░██╗░██║░░░██║░░░███████║██║░░░██║██████╦╝  ╚█████╗░█████═╝░░╚████╔╝░  ║".bright_magenta().bold());
    println!("{}", "║  ██║░░╚██╗██║░░░██║░░░██╔══██║██║░░░██║██╔══██╗  ░╚═══██╗██╔═██╗░░░╚██╔╝░░  ║".bright_magenta().bold());
    println!("{}", "║  ╚██████╔╝██║░░░██║░░░██║░░██║╚██████╔╝██████╦╝  ██████╔╝██║░╚██╗░░░██║░░░  ║".bright_magenta().bold());
    println!("{}", "║  ░╚═════╝░╚═╝░░░╚═╝░░░╚═╝░░╚═╝░╚═════╝░╚═════╝░  ╚═════╝░╚═╝░░╚═╝░░░╚═╝░░░  ║".bright_magenta().bold());
    println!("{}", "║                                                                              ║".bright_cyan());
    println!("{}", "║                     ██╗     ██╗███╗   ██╗███████╗                          ║".bright_blue().bold());
    println!("{}", "║                     ██║     ██║████╗  ██║██╔════╝                          ║".bright_blue().bold());
    println!("{}", "║                     ██║     ██║██╔██╗ ██║█████╗                            ║".bright_blue().bold());
    println!("{}", "║                     ██║     ██║██║╚██╗██║██╔══╝                            ║".bright_blue().bold());
    println!("{}", "║                     ███████╗██║██║ ╚████║███████╗                          ║".bright_blue().bold());
    println!("{}", "║                     ╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝                          ║".bright_blue().bold());
    println!("{}", "║                                                                              ║".bright_cyan());
    println!("{}", "║          🏙️  Transform your GitHub contributions into city skylines! 🏙️     ║".bright_white().bold());
    println!("{}", "║                                                                              ║".bright_cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
    println!("{}", "                        🌟 Welcome to GitHub Skyline! 🌟".bright_yellow().bold());
    println!();
}

pub async fn interactive_mode() {
    println!("{}", "🚀 Let's set up your GitHub skyline generation!".bright_green().bold());
    println!();
    
   
    let token_available = check_and_setup_token().await;
    if !token_available {
        println!("{}", "❌ Unable to proceed without a valid GitHub token.".bright_red());
        return;
    }
    
    
    let username = get_username_input();
    if username.is_empty() {
        println!("{}", "❌ Username is required to generate skyline.".bright_red());
        return;
    }
    
    
    let theme = get_theme_input();
    let style = get_style_input();
    let scale = get_scale_input();
    let ascii_only = get_ascii_only_input();
    let sky_mode = get_sky_mode_input();
    
    
    let output_file = get_output_preference();
    
    println!();
    println!("{}", "🎨 Generating your GitHub skyline...".bright_cyan().bold());
    println!();
    
    
    let client = match GitHubClient::new() {
        Ok(client) => client,
        Err(e) => {
            println!("{} Error creating GitHub client: {}", "❌".bright_red(), e);
            return;
        }
    };
    
    match client.fetch_contributions(&username).await {
        Ok(calendar) => {
            let contributions = calendar.flatten_contributions();
            println!("{} Fetched {} days of contribution data", "✅".bright_green(), contributions.len());
            println!("{} Total contributions: {}", "📊".bright_blue(), calendar.total_contributions);
            println!();
            
            if let Some(filename) = output_file {
                
                use crate::output::{render_skyline_to_string, save_skyline_to_file};
                let (skyline_content, total_contribs) = render_skyline_to_string(
                    &contributions,
                    &theme,
                    &username,
                    &style,
                    &scale,
                    ascii_only,
                    &sky_mode,
                    None,
                );
                match save_skyline_to_file(&skyline_content, &filename, &username, &theme, total_contribs) {
                    Ok(_) => {},
                    Err(e) => println!("{} Error saving to file: {}", "❌".bright_red(), e),
                }
            } else {
                render_skyline_with_options(
                    &contributions,
                    &theme,
                    &style,
                    &scale,
                    ascii_only,
                    &sky_mode,
                    None,
                );
            }
        }
        Err(e) => {
            println!("{} Error fetching contributions: {}", "❌".bright_red(), e);
        }
    }
}

fn get_style_input() -> String {
    println!("{}", "🎭 Style Selection".bright_cyan().bold());
    println!("{}", "   Choose your rendering style:".bright_white());
    println!("{}", "   [1] Braille (default)".bright_magenta());
    println!("{}", "   [2] Blocks".bright_blue());
    println!("{}", "   [3] ASCII".bright_green());
    println!("{}", "   [4] Hash (#)".bright_cyan());
    print!("{}", "   > ".bright_yellow());
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_ok() {
        match choice.trim() { "2" => "blocks".into(), "3" => "ascii".into(), "4" => "hash".into(), _ => "braille".into() }
    } else { "braille".into() }
}

fn get_scale_input() -> String {
    println!("{}", "📐 Scaling".bright_cyan().bold());
    println!("{}", "   Choose scaling algorithm:".bright_white());
    println!("{}", "   [1] Dramatic (default)".bright_magenta());
    println!("{}", "   [2] Linear".bright_blue());
    println!("{}", "   [3] Sqrt".bright_green());
    println!("{}", "   [4] Log".bright_cyan());
    print!("{}", "   > ".bright_yellow());
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_ok() {
        match choice.trim() { "2" => "linear".into(), "3" => "sqrt".into(), "4" => "log".into(), _ => "dramatic".into() }
    } else { "dramatic".into() }
}

fn get_ascii_only_input() -> bool {
    println!("{}", "🔤 ASCII-only?".bright_cyan().bold());
    println!("{}", "   Use only ASCII characters (y/N)?".bright_white());
    print!("{}", "   > ".bright_yellow());
    io::stdout().flush().unwrap();
    let mut ans = String::new();
    if io::stdin().read_line(&mut ans).is_ok() { matches!(ans.trim().to_lowercase().as_str(), "y" | "yes") } else { false }
}

fn get_sky_mode_input() -> String {
    println!("{}", "🌌 Sky".bright_cyan().bold());
    println!("{}", "   Sky detail:".bright_white());
    println!("{}", "   [1] Detailed (default)".bright_magenta());
    println!("{}", "   [2] None".bright_blue());
    print!("{}", "   > ".bright_yellow());
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_ok() { if choice.trim() == "2" { "none".into() } else { "detailed".into() } } else { "detailed".into() }
}

fn get_username_input() -> String {
    println!("{}", "👤 GitHub Username Setup".bright_cyan().bold());
    println!("{}", "   Enter the GitHub username you'd like to generate a skyline for:".bright_white());
    print!("{}", "   > ".bright_yellow());
    io::stdout().flush().unwrap();
    
    let mut username = String::new();
    match io::stdin().read_line(&mut username) {
        Ok(_) => username.trim().to_string(),
        Err(_) => {
            println!("{}", "❌ Failed to read username input.".bright_red());
            String::new()
        }
    }
}

fn get_theme_input() -> String {
    println!();
    println!("{}", "🎨 Theme Selection".bright_cyan().bold());
    println!("{}", "   Choose your preferred color theme:".bright_white());
    println!("{}", "   [1] 🌆 Synthwave (default) - Neon cyan and magenta vibes".bright_cyan());
    println!("{}", "   [2] 🧛 Dracula - Dark theme with vibrant colors".bright_magenta());
    println!("{}", "   [3] ☀️  Solarized - Warm, balanced earth tones".bright_yellow());
    println!("{}", "   [4] 🌃 Cyberpunk - Electric magenta and cyan future".bright_magenta());
    println!("{}", "   [5] 🟢 Matrix - Digital green rain aesthetic".bright_green());
    println!("{}", "   [6] 🌅 Sunset - Warm reds, oranges, and yellows".bright_red());
    println!("{}", "   [7] 🎲 Random - Surprise me!".bright_white());
    print!("{}", "   > ".bright_yellow());
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    match io::stdin().read_line(&mut choice) {
        Ok(_) => {
            match choice.trim() {
                "1" | "" => "synthwave".to_string(),
                "2" => "dracula".to_string(),
                "3" => "solarized".to_string(),
                "4" => "cyberpunk".to_string(),
                "5" => "matrix".to_string(),
                "6" => "sunset".to_string(),
                "7" => "random".to_string(),
                _ => {
                    println!("{}", "   🎨 Using default theme (synthwave)".bright_cyan());
                    "synthwave".to_string()
                }
            }
        }
        Err(_) => {
            println!("{}", "   🎨 Using default theme (synthwave)".bright_cyan());
            "synthwave".to_string()
        }
    }
}

fn get_output_preference() -> Option<String> {
    println!();
    println!("{}", "💾 Output Options".bright_cyan().bold());
    println!("{}", "   Choose how to output your skyline:".bright_white());
    println!("{}", "   [1] 🖥️  Display in terminal (default)".bright_green());
    println!("{}", "   [2] 💾 Save to file".bright_blue());
    print!("{}", "   > ".bright_yellow());
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    match io::stdin().read_line(&mut choice) {
        Ok(_) => {
            match choice.trim() {
                "2" => {
                    println!("{}", "   📁 Enter filename (e.g., skyline.txt):".bright_white());
                    print!("{}", "   > ".bright_yellow());
                    io::stdout().flush().unwrap();
                    
                    let mut filename = String::new();
                    match io::stdin().read_line(&mut filename) {
                        Ok(_) => {
                            let filename = filename.trim();
                            if filename.is_empty() {
                                println!("{}", "   💾 Using default filename: skyline.txt".bright_blue());
                                Some("skyline.txt".to_string())
                            } else {
                                Some(filename.to_string())
                            }
                        }
                        Err(_) => {
                            println!("{}", "   💾 Using default filename: skyline.txt".bright_blue());
                            Some("skyline.txt".to_string())
                        }
                    }
                }
                _ => {
                    println!("{}", "   🖥️  Displaying in terminal".bright_green());
                    None
                }
            }
        }
        Err(_) => {
            println!("{}", "   🖥️  Displaying in terminal (default)".bright_green());
            None
        }
    }
}

async fn check_and_setup_token() -> bool {
    println!("{}", "🔑 GitHub Token Setup".bright_cyan().bold());
    
    
    if env::var("GITHUB_TOKEN").is_ok() {
        println!("{}", "   ✅ GitHub token found! Testing connection...".bright_green());
        
        
        match GitHubClient::new() {
            Ok(_) => {
                println!("{}", "   ✅ Token is valid and ready to use!".bright_green());
                return true;
            }
            Err(_) => {
                println!("{}", "   ⚠️  Token found but appears invalid. Let's set up a new one.".bright_yellow());
            }
        }
    } else {
        println!("{}", "   ⚠️  No GitHub token found. Let's set one up!".bright_yellow());
    }
    
    println!();
    println!("{}", "   📝 How to get your GitHub token:".bright_white());
    println!("{}", "   1. Go to: https://github.com/settings/tokens".bright_blue());
    println!("{}", "   2. Click 'Generate new token' > 'Generate new token (classic)'".bright_blue());
    println!("{}", "   3. Give it a name like 'GitHub Skyline'".bright_blue());
    println!("{}", "   4. You can leave all scopes unchecked (public data only)".bright_blue());
    println!("{}", "   5. Click 'Generate token'".bright_blue());
    println!("{}", "   6. Copy the token and paste it below".bright_blue());
    println!();
    println!("{}", "   🔒 Enter your GitHub token:".bright_white());
    println!("{}", "   💡 Tip: Right-click to paste in most Windows terminals".bright_cyan());
    println!("{}", "   💡 Or try Ctrl+Shift+V in Windows Terminal".bright_cyan());
    print!("{}", "   > ".bright_yellow());
    io::stdout().flush().unwrap();
    
    let mut token = String::new();
    match io::stdin().read_line(&mut token) {
        Ok(_) => {
            let token = token.trim();
            if token.is_empty() {
                println!("{}", "   ❌ No token provided.".bright_red());
                return false;
            }
            
            
            if token.contains("^V") || token.contains("^C") {
                println!("{}", "   ⚠️  It looks like the paste didn't work properly.".bright_yellow());
                println!("{}", "   💡 Try right-clicking in the terminal to paste, or use Ctrl+Shift+V".bright_cyan());
                println!("{}", "   💡 Make sure you copied the token from GitHub first".bright_cyan());
                return false;
            }
            
            
            unsafe {
                std::env::set_var("GITHUB_TOKEN", token);
            }
            
            
            println!("{}", "   🔍 Testing token...".bright_blue());
            match GitHubClient::new() {
                Ok(_) => {
                    println!("{}", "   ✅ Token is valid! Ready to generate skylines!".bright_green());
                    println!();
                    println!("{}", "   💡 Pro tip: To avoid entering the token each time, set it permanently:".bright_blue());
                    #[cfg(target_os = "windows")]
                    println!("{}", "      For this session: $env:GITHUB_TOKEN=\"your_token_here\"".bright_white());
                    #[cfg(not(target_os = "windows"))]
                    println!("{}", "      For this session: export GITHUB_TOKEN=\"your_token_here\"".bright_white());
                    println!();
                    true
                }
                Err(e) => {
                    println!("{} Token test failed: {}", "❌".bright_red(), e);
                    println!("{}", "   💡 Common issues:".bright_cyan());
                    println!("{}", "      • Make sure you copied the entire token".bright_cyan());
                    println!("{}", "      • Check for extra spaces or characters".bright_cyan());
                    println!("{}", "      • Generate a new token if this one is old".bright_cyan());
                    println!("{}", "      • Try manually typing the token if pasting fails".bright_cyan());
                    println!("{}", "   Please try again.".bright_yellow());
                    false
                }
            }
        }
        Err(_) => {
            println!("{}", "   ❌ Failed to read token input.".bright_red());
            println!("{}", "   💡 Try typing or pasting the token again".bright_cyan());
            false
        }
    }
} 