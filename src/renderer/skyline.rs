use colored::*;
use crate::renderer::building::get_max_height;
use crate::renderer::sky_elements::{select_moon_type, print_night_sky};
use rand::seq::SliceRandom;

#[derive(Clone)]
struct Theme {
    building_colors: Vec<fn(&str) -> ColoredString>,
    window_colors: Vec<fn(&str) -> ColoredString>,
    antenna_color: fn(&str) -> ColoredString,
    roof_color: fn(&str) -> ColoredString,
    base_color: fn(&str) -> ColoredString,
}

fn get_theme(theme: &str) -> Theme {
    let theme_name = theme.to_lowercase();
    if theme_name == "random" {
        let themes = ["synthwave", "dracula", "solarized", "cyberpunk", "matrix", "sunset"];
        let mut rng = rand::thread_rng();
        let selected = themes.choose(&mut rng).unwrap();
        return get_theme(selected);
    }
    
    match theme_name.as_str() {
        "dracula" => Theme {
            building_colors: vec![
                |s| s.bright_black().bold(),
                |s| s.bright_blue().bold(),
                |s| s.bright_magenta().bold(),
                |s| s.bright_cyan().bold(),
                |s| s.bright_yellow().bold(),
                |s| s.bright_red().bold(),
            ],
            window_colors: vec![
                |s| s.yellow(),
                |s| s.bright_yellow(),
                |s| s.bright_white(),
                |s| s.bright_magenta(),
                |s| s.bright_cyan(),
                |s| s.bright_red(),
            ],
            antenna_color: |s| s.bright_red().bold(),
            roof_color: |s| s.bright_white().bold(),
            base_color: |s| s.bright_black().bold(),
        },
        "solarized" => Theme {
            building_colors: vec![
                |s| s.bright_yellow().bold(),
                |s| s.yellow().bold(),
                |s| s.bright_green().bold(),
                |s| s.green().bold(),
                |s| s.bright_blue().bold(),
                |s| s.blue().bold(),
            ],
            window_colors: vec![
                |s| s.bright_white(),
                |s| s.bright_yellow(),
                |s| s.bright_green(),
                |s| s.bright_blue(),
                |s| s.bright_cyan(),
                |s| s.bright_magenta(),
            ],
            antenna_color: |s| s.bright_yellow().bold(),
            roof_color: |s| s.bright_white().bold(),
            base_color: |s| s.bright_black().bold(),
        },
        "cyberpunk" => Theme {
            building_colors: vec![
                |s| s.bright_magenta().bold(),
                |s| s.magenta().bold(),
                |s| s.bright_cyan().bold(),
                |s| s.cyan().bold(),
                |s| s.bright_yellow().bold(),
                |s| s.yellow().bold(),
            ],
            window_colors: vec![
                |s| s.bright_white(),
                |s| s.bright_cyan(),
                |s| s.bright_magenta(),
                |s| s.bright_yellow(),
                |s| s.white(),
                |s| s.cyan(),
            ],
            antenna_color: |s| s.bright_magenta().bold(),
            roof_color: |s| s.bright_cyan().bold(),
            base_color: |s| s.magenta().bold(),
        },
        "matrix" => Theme {
            building_colors: vec![
                |s| s.green().bold(),
                |s| s.bright_green().bold(),
                |s| s.green(),
                |s| s.bright_green(),
                |s| s.bright_white().bold(),
                |s| s.white().bold(),
            ],
            window_colors: vec![
                |s| s.bright_green(),
                |s| s.green(),
                |s| s.bright_white(),
                |s| s.white(),
                |s| s.bright_green(),
                |s| s.green(),
            ],
            antenna_color: |s| s.bright_green().bold(),
            roof_color: |s| s.bright_white().bold(),
            base_color: |s| s.green().bold(),
        },
        "sunset" => Theme {
            building_colors: vec![
                |s| s.bright_red().bold(),
                |s| s.red().bold(),
                |s| s.bright_yellow().bold(),
                |s| s.yellow().bold(),
                |s| s.bright_magenta().bold(),
                |s| s.magenta().bold(),
            ],
            window_colors: vec![
                |s| s.bright_yellow(),
                |s| s.yellow(),
                |s| s.bright_white(),
                |s| s.bright_red(),
                |s| s.bright_magenta(),
                |s| s.red(),
            ],
            antenna_color: |s| s.bright_red().bold(),
            roof_color: |s| s.bright_yellow().bold(),
            base_color: |s| s.red().bold(),
        },
        _ => Theme { 
            building_colors: vec![
                |s| s.cyan().bold(),
                |s| s.bright_cyan().bold(),
                |s| s.blue().bold(),
                |s| s.bright_blue().bold(),
                |s| s.magenta().bold(),
                |s| s.bright_magenta().bold(),
            ],
            window_colors: vec![
                |s| s.cyan(),
                |s| s.bright_cyan(),
                |s| s.blue(),
                |s| s.bright_blue(),
                |s| s.magenta(),
                |s| s.bright_magenta(),
            ],
            antenna_color: |s| s.bright_magenta().bold(),
            roof_color: |s| s.bright_white().bold(),
            base_color: |s| s.bright_black().bold(),
        },
    }
}

pub fn render_skyline(contributions: &[u32], theme: &str) {
    if contributions.is_empty() {
        println!("{}", "❌ No contribution data to render!".bright_red().bold());
        return;
    }

    let max_contributions = get_max_height(contributions);
    let building_heights: Vec<u32> = contributions
        .iter()
        .map(|&count| dramatic_scale(count, max_contributions))
        .collect();
    
    let total_contributions: u32 = contributions.iter().sum();
    let moon_type = select_moon_type(total_contributions);
    let max_height = *building_heights.iter().max().unwrap_or(&1) + 6;

    print_header();
    println!("{}", format!("📈 Max daily contributions: {}", max_contributions).bright_yellow().bold());
    print_skyline_title();
    render_braille_skyline(&building_heights, contributions, max_height, &moon_type, theme);
    print_ground_section(25);
    print_statistics(contributions, max_contributions);
    
  
    let achievements = crate::achievements::calculate_achievements(contributions);
    crate::achievements::display_achievements(&achievements);
    
    print_legend();
    print_footer();
}

fn dramatic_scale(contribution_count: u32, max_contributions: u32) -> u32 {
    if contribution_count == 0 { return 0; }
    
    let norm = contribution_count as f32 / max_contributions as f32;
    let dramatic = norm.powf(1.2);
    let scaled = (dramatic * 28.0) + 2.0;
    scaled.round() as u32
}

fn render_braille_skyline(building_heights: &[u32], contributions: &[u32], max_height: u32, moon_type: &crate::renderer::sky_elements::MoonType, theme: &str) {
    let width = building_heights.len().min(25);
    print_night_sky(width * 4, moon_type);
    
    let total_contributions: u32 = contributions.iter().sum();
    let longest_streak = calculate_longest_streak(contributions);
    
    for row in (1..=max_height).rev() {
        let mut line = String::new();
        for i in 0..width {
            let day_contributions = contributions[contributions.len().saturating_sub(width) + i];
            
            
            if let Some(special_building) = get_special_building(i, day_contributions, total_contributions, longest_streak) {
                let special_height = special_building.len() as u32;
                if row <= special_height {
                    let building_line_index = (special_height - row) as usize;
                    line.push_str(&special_building[building_line_index]);
                } else {
                    line.push_str("   ");
                }
            } else {

                let height = building_heights[i];
                let building_part = get_building_part(height, row, day_contributions, theme);
                line.push_str(&building_part);
            }
            if i < width - 1 { line.push(' '); }
        }
        println!("{}", line);
    }
}

fn get_building_part(height: u32, current_row: u32, contributions: u32, theme: &str) -> String {
    let theme = get_theme(theme);
    let color_idx = match height {
        h if h > 25 => 5,
        h if h > 20 => 4,
        h if h > 15 => 3,
        h if h > 10 => 2,
        h if h > 5  => 1,
        _           => 0,
    };
    let building_color = theme.building_colors[color_idx.min(theme.building_colors.len()-1)];
    let window_color = theme.window_colors[color_idx.min(theme.window_colors.len()-1)];
    let antenna_color = theme.antenna_color;
    let roof_color = theme.roof_color;
    let base_color = theme.base_color;
    let antenna_height = match height {
        h if h > 25 => 5,
        h if h > 18 => 3, 
        h if h > 12 => 1,
        _ => 0,
    };
    if antenna_height > 0 && current_row > height && current_row <= height + antenna_height {
        return match current_row - height {
            1 => antenna_color(" ⢰ ").to_string(),
            2 => antenna_color(" ⢸ ").to_string(),
            _ => antenna_color(" ⢸ ").to_string(),
        };
    }
    if current_row > height { return "   ".to_string(); }
    if current_row == 1 && height > 0 {
        return base_color("⣸⣸⣸").to_string();
    }
    if current_row == height {
        return roof_color("¯¯¯").to_string();
    }
    let is_window = contributions > 0 && 
        ((current_row + contributions * 3) % 3 == 0 || 
         (current_row % 4 == 0 && contributions % 2 == 1));
    if is_window {
        return window_color("⣾⣾⣾").to_string();
    }
    building_color("⣿⣿⣿").to_string()
}

fn get_special_building(_display_index: usize, _contributions: u32, _total_contributions: u32, _longest_streak: u32) -> Option<Vec<String>> {
   
    None
}



fn print_header() {
    println!("\n{}", "╔═══════════════════════════════════════════════════════════════╗".bright_cyan().bold());
    println!("{}", "║                    🚀 GITHUB SKYLINE GENERATOR 🚀             ║".bright_cyan().bold());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_cyan().bold());
}

fn print_skyline_title() {
    println!("\n{}", "┌─────────────────────────────────────────────────────────────┐".bright_magenta().bold());
    println!("{}", "│                 BRAILLE-STYLE ASCII SKYLINE                 │".bright_magenta().bold());
    println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_magenta().bold());
}

fn print_ground_section(width: usize) {
    let line_width = (width * 3) + (width - 1);
    let mut ground_line = String::new();
    for i in 0..line_width {
        let char = match i % 3 {
            0 => "~".bright_blue().to_string(),
            1 => "~".blue().to_string(), 
            _ => "~".bright_cyan().to_string(),
        };
        ground_line.push_str(&char);
    }
    println!("{}", ground_line);
}

fn print_statistics(contributions: &[u32], max_contributions: u32) {
    let total: u32 = contributions.iter().sum();
    let avg = total as f32 / contributions.len() as f32;
    let active_days = contributions.iter().filter(|&&x| x > 0).count();
    let longest_streak = calculate_longest_streak(contributions);
    
    println!("\n{}", "╭─────────────────────────────────────────────────────────────╮".bright_blue().bold());
    println!("{}", "│               📊 CODING STATISTICS 📊                       │".bright_blue().bold());
    println!("{}", "├─────────────────────────────────────────────────────────────┤".bright_blue().bold());
    
    println!("{}", format!("│ 📅 Total days tracked: {}                              │", format!("{:>3}", contributions.len()).bright_white().bold()).bright_blue());
    println!("{}", format!("│ 🏙️  Days displayed:  {}                                 │", format!("{:>3}", contributions.len().min(25)).bright_white().bold()).bright_blue());
    println!("{}", format!("│ ⭐ Total contributions: {}                             │", format!("{:>4}", total).bright_yellow().bold()).bright_blue());
    println!("{}", format!("│ 📈 Average per day: {:.1}                                │", format!("{:>4.1}", avg).bright_cyan().bold()).bright_blue());
    println!("{}", format!("│ 🔥 Max daily contributions:  {}                        │", format!("{:>3}", max_contributions).bright_red().bold()).bright_blue());
    println!("{}", format!("│ 💪 Active coding days: {} ({:.1}%)                   │", 
             format!("{:>3}", active_days).bright_green().bold(),
             ((active_days as f32 / contributions.len() as f32) * 100.0)).bright_blue());
    println!("{}", format!("│ 🔥 Longest streak:  {} days                            │", format!("{:>3}", longest_streak).bright_magenta().bold()).bright_blue());
    
    println!("{}", "╰─────────────────────────────────────────────────────────────╯".bright_blue().bold());
}

fn calculate_longest_streak(contributions: &[u32]) -> u32 {
    let mut longest = 0;
    let mut current = 0;
    
    for &count in contributions {
        if count > 0 {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    longest
}

fn print_legend() {
    println!("\n{}", "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓".bright_green().bold());
    println!("{}", "┃              SYNTHWAVE ARCHITECTURE LEGEND                ┃".bright_green().bold());
    println!("{}", "┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫".bright_green().bold());
    
    println!("{}", format!("┃ {}  NEON SPIRE (26+) w/ Antenna ⢰                  ┃", "⣿⣿⣿".bright_magenta().bold()).bright_green());
    println!("{}", format!("┃ {}  MEGA TOWER (21-25) w/ Antenna ⢸                ┃", "⣿⣿⣿".magenta().bold()).bright_green());
    println!("{}", format!("┃ {}  Cyber Corp (16-20) w/ Roof ¯¯¯                ┃", "⣿⣿⣿".bright_blue().bold()).bright_green());
    println!("{}", format!("┃ {}  Data Hub (11-15)                              ┃", "⣿⣿⣿".blue().bold()).bright_green());
    println!("{}", format!("┃ {}  Apartments (6-10)                             ┃", "⣿⣿⣿".bright_cyan().bold()).bright_green());
    println!("{}", format!("┃ {}  Shops (1-5)                                   ┃", "⣿⣿⣿".cyan().bold()).bright_green());
    
    println!("{}", "┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫".bright_green().bold());
    println!("{}", "┃ Features: ⣸ Base, ⣾ Windows, ~~~ Water                  ┃".bright_green().bold());
    println!("{}", "┃ Sky: . * + Stars                                            ┃".bright_green().bold());
    println!("{}", "┃ Moons (by total contributions):                             ┃".bright_green().bold());
    println!("{}", "┃   0-499: Crescent  500-1499: Quarter                      ┃".bright_green().bold());
    println!("{}", "┃   1500-2999: Gibbous  3000+: Full Moon                    ┃".bright_green().bold());
    println!("{}", "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛".bright_green().bold());
}

fn print_footer() {
    println!("\n{}", "▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓".bright_magenta());
    println!("{}", "         Your Braille-Style ASCII Architectural Year!         ".bright_magenta().bold());
    println!("{}", "         Share your beautiful terminal cityscape!           ".bright_cyan().bold());
    println!("{}", "▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓".bright_magenta());
    
    println!("\n{}", "🎉 Your GitHub skyline is ready! Thanks for using GitHub Skyline!".bright_green().bold());
    println!("{}", "👤 Want to generate another skyline? Press Enter to continue or Ctrl+C to quit...".bright_yellow());
    use std::io::{self, Write};
    print!("{}", "   Press Enter: ".bright_cyan());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}