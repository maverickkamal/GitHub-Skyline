use colored::*;
use rand::prelude::*;

pub enum MoonType {
    Crescent,
    Quarter,
    Gibbous,
    Full,
}

pub fn select_moon_type(total_contributions: u32) -> MoonType {
    match total_contributions {
        0..=499 => MoonType::Crescent,
        500..=1499 => MoonType::Quarter,
        1500..=2999 => MoonType::Gibbous,
        _ => MoonType::Full,
    }
}

pub fn print_night_sky(width: usize, moon_type: &MoonType) {
    let mut rng = rand::thread_rng();
        
    let star_chars = ['*', '·', '+', '✦', '⋆', '✧'];
    let twinkle_chars = ['.', '˚', '°', '*', '✦', ' '];
    
    let moon_lines = get_moon_art(moon_type);
    let moon_start_col = if width > 80 { width - 30 } else if width > 60 { width - 25 } else { width - 20 };
    let moon_start_row = 2;
    let moon_end_row = moon_start_row + moon_lines.len();
    
    for row in 0..15 {
        print!("{}", " ".repeat(5));
        
        for col in 0..width {
            let mut cell_filled = false;
            
            if row >= moon_start_row && row < moon_end_row && col >= moon_start_col {
                let moon_row_idx = row - moon_start_row;
                let moon_col_idx = col - moon_start_col;
                
                if moon_row_idx < moon_lines.len() {
                    let moon_line = &moon_lines[moon_row_idx];
                    if moon_col_idx < moon_line.len() {
                        let moon_char = moon_line.chars().nth(moon_col_idx).unwrap_or(' ');
                        if moon_char != ' ' {
                            print!("{}", format!("{}", moon_char).bright_yellow().bold());
                            cell_filled = true;
                        }
                    }
                }
            }
            
            if !cell_filled {
                let star_probability = match row {
                    0..=4 => 0.08,  
                    5..=9 => 0.05,  
                    10..=14 => 0.03, 
                    _ => 0.0,
                };
                
                if rng.gen::<f64>() < star_probability {
                    let use_twinkle = rng.gen::<f64>() < 0.3;
                    let star_char = if use_twinkle {
                        *twinkle_chars.choose(&mut rng).unwrap()
                    } else {
                        *star_chars.choose(&mut rng).unwrap()
                    };
                    
                    let colored_star = match rng.gen_range(0..6) {
                        0 => format!("{}", star_char).bright_white().to_string(),
                        1 => format!("{}", star_char).bright_cyan().to_string(),
                        2 => format!("{}", star_char).bright_blue().to_string(),
                        3 => format!("{}", star_char).cyan().to_string(),
                        4 => format!("{}", star_char).white().to_string(),
                        _ => format!("{}", star_char).bright_yellow().to_string(),
                    };
                    print!("{}", colored_star);
                } else {
                    print!(" ");
                }
            }
        }
        
        println!();
    }
}



fn get_moon_art(moon_type: &MoonType) -> Vec<&'static str> {
    match moon_type {
        MoonType::Crescent => vec![
            "               ____....",
            "           a#####~:::::::",
            "       a######P\";:::::::::::",
            "    a########:::::::::::::::::",
            "   ########P::::::::::::*::::::",
            "  ########P:::::::::::::::::.::",
            " ##### ##P::::::::::::::::::::",
            ".#### O ##:::::*::::::::::::::",
            "###### #### ::::::::::::::::::",
            "########@###,::::::::::::::::::",
            "#########~~~:::::::::::::::*:::",
            " ##### ##:::::::::::::::::::::",
            "  ####a__ay::::::::::::::::::",
            "   ########;::::::::::::::::",
            "    ########a:::::::::::::",
            "      ########.:::::::::*;:",
            "       `d######a.:::::::::",
            "          `~9#####.::::",
        ],
        MoonType::Quarter => vec![
            "          ______          ",
            "      .-'` .    `'-.    ",
            "    .'  '    .---.  '.  ",
            "   /  '    .'     `'. \\",
            "  ;  '    /          \\|",
            " :  '  _ ;            `",
            ";  :  /(\\",
            "|  .       '.",
            "|  ' /     --'",
            "|  .   '.__\\",
            ";  :       /",
            " ;  .     |            ,",
            "  ;  .    \\           /|",
            "   \\  .    '.       .'/" ,
            "    '.  '  . `'---'`.'",
            "      `'-..._____.-`",
        ],
        MoonType::Gibbous => vec![
            "o                     __...__",
            "              *   .--'    __.=-.",
            "     |          ./     .-'",
            "    -O-        /      /",
            "     |        /    '\"/",
            "             |     (@)",
            "            |        \\",
            "            |         \\",
            " *          |       ___\\",
            "             |  .   /  `",
            "              \\  `~~\\",
            "         o     \\     \\",
            "                `\\    `-.__",
            "    .             `--._    `--'",
            "                       `---~~`",
        ],
        MoonType::Full => vec![
            "         ___---___",
            "      .--         --.",
            "    ./   ()      .-. \\.",
            "   /   o    .   (   )  \\",
            "  / .            '-'    \\",
            " | ()    .  O         .  |",
            "|                         |",
            "|    o           ()       |",
            "|       .--.          O   |",
            " | .   |    |            |",
            "  \\    `.__.'    o   .  /",
            "   \\                   /",
            "    `\\  o    ()      /'",
            "      `--___   ___--'",
            "            ---",
        ],
    }
} 