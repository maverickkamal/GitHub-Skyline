use colored::*;

#[derive(Debug, Clone)]
pub struct Achievement {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub tier: Tier,

}

#[derive(Debug, Clone, PartialEq)]
pub enum Tier {
    Bronze,
    Silver,
    Gold,
    Legendary,
}

impl Tier {
    pub fn color(&self) -> fn(&str) -> ColoredString {
        match self {
            Tier::Bronze => |s: &str| s.yellow(),
            Tier::Silver => |s: &str| s.bright_white(),
            Tier::Gold => |s: &str| s.bright_yellow(),
            Tier::Legendary => |s: &str| s.bright_magenta(),
        }
    }
    

}

pub fn calculate_achievements(contributions: &[u32]) -> Vec<Achievement> {
    let mut achievements = vec![];
    
    let total: u32 = contributions.iter().sum();
    let max_day = *contributions.iter().max().unwrap_or(&0);
    let active_days = contributions.iter().filter(|&&x| x > 0).count();
    let longest_streak = calculate_longest_streak(contributions);
    let avg_per_active_day = if active_days > 0 { 
        total as f32 / active_days as f32 
    } else { 
        0.0 
    };
    
   
    achievements.extend(check_total_achievements(total));
    
   
    achievements.extend(check_consistency_achievements(contributions, active_days));
    
   
    achievements.extend(check_streak_achievements(longest_streak));
    
   
    achievements.extend(check_intensity_achievements(max_day, avg_per_active_day));
    
   
    achievements.extend(check_pattern_achievements(contributions));
    
    achievements
}

fn check_total_achievements(total: u32) -> Vec<Achievement> {
    let mut achievements = vec![];
    
    if total >= 10000 {
        achievements.push(Achievement {
            name: "Code Titan".to_string(),
            description: "Made 10,000+ contributions in a year".to_string(),
            icon: "🏆".to_string(),
            tier: Tier::Legendary,

        });
    } else if total >= 5000 {
        achievements.push(Achievement {
            name: "Coding Machine".to_string(),
            description: "Made 5,000+ contributions in a year".to_string(),
            icon: "🥇".to_string(),
            tier: Tier::Gold,

        });
    } else if total >= 2500 {
        achievements.push(Achievement {
            name: "Prolific Coder".to_string(),
            description: "Made 2,500+ contributions in a year".to_string(),
            icon: "🥈".to_string(),
            tier: Tier::Silver,

        });
    } else if total >= 1000 {
        achievements.push(Achievement {
            name: "Active Developer".to_string(),
            description: "Made 1,000+ contributions in a year".to_string(),
            icon: "🥉".to_string(),
            tier: Tier::Bronze,

        });
    }
    
    achievements
}

fn check_consistency_achievements(contributions: &[u32], active_days: usize) -> Vec<Achievement> {
    let mut achievements = vec![];
    let total_days = contributions.len();
    let consistency_rate = active_days as f32 / total_days as f32;
    
    if consistency_rate >= 0.9 {
        achievements.push(Achievement {
            name: "Daily Grinder".to_string(),
            description: "Active on 90%+ of tracked days".to_string(),
            icon: "⚡".to_string(),
            tier: Tier::Legendary,

        });
    } else if consistency_rate >= 0.7 {
        achievements.push(Achievement {
            name: "Steady Coder".to_string(),
            description: "Active on 70%+ of tracked days".to_string(),
            icon: "📈".to_string(),
            tier: Tier::Gold,

        });
    } else if consistency_rate >= 0.5 {
        achievements.push(Achievement {
            name: "Regular Contributor".to_string(),
            description: "Active on 50%+ of tracked days".to_string(),
            icon: "📊".to_string(),
            tier: Tier::Silver,

        });
    } else if consistency_rate >= 0.25 {
        achievements.push(Achievement {
            name: "Weekend Warrior".to_string(),
            description: "Active on 25%+ of tracked days".to_string(),
            icon: "🏃".to_string(),
            tier: Tier::Bronze,

        });
    }
    
    achievements
}

fn check_streak_achievements(longest_streak: u32) -> Vec<Achievement> {
    let mut achievements = vec![];
    
    if longest_streak >= 365 {
        achievements.push(Achievement {
            name: "Year-Long Dedication".to_string(),
            description: "Maintained a 365+ day streak".to_string(),
            icon: "🔥".to_string(),
            tier: Tier::Legendary,

        });
    } else if longest_streak >= 100 {
        achievements.push(Achievement {
            name: "Centurion".to_string(),
            description: "Maintained a 100+ day streak".to_string(),
            icon: "💯".to_string(),
            tier: Tier::Gold,

        });
    } else if longest_streak >= 30 {
        achievements.push(Achievement {
            name: "Monthly Marathon".to_string(),
            description: "Maintained a 30+ day streak".to_string(),
            icon: "🏁".to_string(),
            tier: Tier::Silver,

        });
    } else if longest_streak >= 7 {
        achievements.push(Achievement {
            name: "Week Warrior".to_string(),
            description: "Maintained a 7+ day streak".to_string(),
            icon: "📅".to_string(),
            tier: Tier::Bronze,

        });
    }
    
    achievements
}

fn check_intensity_achievements(max_day: u32, avg_per_active_day: f32) -> Vec<Achievement> {
    let mut achievements = vec![];
    
    if max_day >= 50 {
        achievements.push(Achievement {
            name: "Power User".to_string(),
            description: "Made 50+ contributions in a single day".to_string(),
            icon: "⚡".to_string(),
            tier: Tier::Gold,

        });
    } else if max_day >= 25 {
        achievements.push(Achievement {
            name: "High Roller".to_string(),
            description: "Made 25+ contributions in a single day".to_string(),
            icon: "🎯".to_string(),
            tier: Tier::Silver,

        });
    } else if max_day >= 10 {
        achievements.push(Achievement {
            name: "Big Day".to_string(),
            description: "Made 10+ contributions in a single day".to_string(),
            icon: "📈".to_string(),
            tier: Tier::Bronze,

        });
    }
    
    if avg_per_active_day >= 20.0 {
        achievements.push(Achievement {
            name: "Quality over Quantity".to_string(),
            description: "Average 20+ contributions per active day".to_string(),
            icon: "💎".to_string(),
            tier: Tier::Gold,

        });
    } else if avg_per_active_day >= 10.0 {
        achievements.push(Achievement {
            name: "Efficient Coder".to_string(),
            description: "Average 10+ contributions per active day".to_string(),
            icon: "⚙️".to_string(),
            tier: Tier::Silver,

        });
    }
    
    achievements
}

fn check_pattern_achievements(contributions: &[u32]) -> Vec<Achievement> {
    let mut achievements = vec![];
    
   
    let perfect_streaks = count_perfect_months(contributions);
    if perfect_streaks >= 3 {
        achievements.push(Achievement {
            name: "Marathon Runner".to_string(),
            description: "Had 3+ perfect months (30 days straight)".to_string(),
            icon: "🏃‍♂️".to_string(),
            tier: Tier::Gold,

        });
    } else if perfect_streaks >= 1 {
        achievements.push(Achievement {
            name: "Perfect Month".to_string(),
            description: "Had at least one perfect month (30 days straight)".to_string(),
            icon: "📅".to_string(),
            tier: Tier::Silver,

        });
    }
    
   
    if has_comeback_story(contributions) {
        achievements.push(Achievement {
            name: "The Comeback".to_string(),
            description: "Returned to coding after a long break".to_string(),
            icon: "🔄".to_string(),
            tier: Tier::Bronze,

        });
    }
    
   
    if has_strong_finish(contributions) {
        achievements.push(Achievement {
            name: "Strong Finish".to_string(),
            description: "Ended the year with high activity".to_string(),
            icon: "🎯".to_string(),
            tier: Tier::Silver,

        });
    }
    
    achievements
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

fn count_perfect_months(contributions: &[u32]) -> u32 {
    let mut perfect_months = 0;
    let mut current_streak = 0;
    
    for &count in contributions {
        if count > 0 {
            current_streak += 1;
            if current_streak >= 30 {
                perfect_months += 1;
                current_streak = 0; 
            }
        } else {
            current_streak = 0;
        }
    }
    perfect_months
}

fn has_comeback_story(contributions: &[u32]) -> bool {
    let mut had_gap = false;
    let mut gap_size = 0;
    let mut recent_activity = 0;
    
    for &count in contributions {
        if count == 0 {
            gap_size += 1;
            if gap_size >= 30 {
                had_gap = true;
            }
        } else {
            if had_gap && gap_size >= 30 {
                recent_activity += 1;
            }
            gap_size = 0;
        }
    }
    
    had_gap && recent_activity >= 10
}

fn has_strong_finish(contributions: &[u32]) -> bool {
    if contributions.len() < 60 {
        return false;
    }
    
    let last_60_days = &contributions[contributions.len() - 60..];
    let total_last_60: u32 = last_60_days.iter().sum();
    let active_days_last_60 = last_60_days.iter().filter(|&&x| x > 0).count();
    
   
    total_last_60 >= 100 && active_days_last_60 >= 30
}

pub fn display_achievements(achievements: &[Achievement]) {
    if achievements.is_empty() {
        return;
    }
    
    println!("\n{}", "╔═══════════════════════════════════════════════════════════════╗".bright_cyan().bold());
    println!("{}", "║                    🏆 ACHIEVEMENTS UNLOCKED 🏆                ║".bright_cyan().bold());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_cyan().bold());
    
   
    let legendary: Vec<_> = achievements.iter().filter(|a| a.tier == Tier::Legendary).collect();
    let gold: Vec<_> = achievements.iter().filter(|a| a.tier == Tier::Gold).collect();
    let silver: Vec<_> = achievements.iter().filter(|a| a.tier == Tier::Silver).collect();
    let bronze: Vec<_> = achievements.iter().filter(|a| a.tier == Tier::Bronze).collect();
    
   
    let legendary_count = legendary.len();
    let gold_count = gold.len();
    let silver_count = silver.len();
    let bronze_count = bronze.len();
    
    for (tier_name, tier_achievements) in [
        ("LEGENDARY", legendary),
        ("GOLD", gold),
        ("SILVER", silver),
        ("BRONZE", bronze)
    ] {
        if !tier_achievements.is_empty() {
            println!("\n{}", format!("🏅 {} TIER", tier_name).bright_yellow().bold());
            for achievement in tier_achievements {
                let tier_color = achievement.tier.color();
                println!("{}", format!(
                    "   {} {} - {}",
                    achievement.icon,
                    tier_color(&achievement.name).bold(),
                    achievement.description.bright_white()
                ));
            }
        }
    }
    
    println!("\n{}", format!("🎖️  Total Achievements Earned: {}", achievements.len()).bright_green().bold());
    
    
    let tier_counts: Vec<_> = [
        ("Legendary", legendary_count),
        ("Gold", gold_count),
        ("Silver", silver_count),
        ("Bronze", bronze_count)
    ].into_iter().filter(|(_, count)| *count > 0).collect();
    
    if !tier_counts.is_empty() {
        print!("{}", "🏆 Breakdown: ".bright_cyan());
        for (i, (tier, count)) in tier_counts.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", format!("{} {}", count, tier).bright_white());
        }
        println!();
    }
}