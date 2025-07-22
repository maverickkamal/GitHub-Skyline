use colored::*;

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
    let moon = get_moon_art(moon_type);
    let moon_height = moon.len();
    let moon_width = moon.iter().map(|s| s.len()).max().unwrap_or(0);
    let moon_pos = if width > moon_width + 10 { width - moon_width - 5 } else { 0 };

    for i in 0..moon_height {
        let mut sky_line = String::new();
        let moon_slice = moon[i];
        
        for col in 0..width {
            if col >= moon_pos && col < moon_pos + moon_slice.len() {
                let moon_char_index = col - moon_pos;
                if let Some(moon_char) = moon_slice.chars().nth(moon_char_index) {
                    if moon_char != ' ' {
                        sky_line.push_str(&moon_char.to_string().bright_white().bold().to_string());
                        continue;
                    }
                }
            }

            let star_char = match (i, col) {
                (r, c) if r % 3 == 0 && c % 18 == 5 => Some("."),
                (r, c) if r % 4 == 1 && c % 15 == 2 => Some("*"),
                (r, c) if r % 2 == 0 && c % 20 == 12 => Some("+"),
                (r, c) if r % 5 == 2 && c % 12 == 9 => Some("'"),
                _ => None,
            };

            if let Some(star) = star_char {
                sky_line.push_str(&star.bright_yellow().to_string());
            } else {
                sky_line.push(' ');
            }
        }
        println!("{}", sky_line);
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