use crate::display::ascii::section_divider;
use crate::display::colors::{accent, header, info, subtle, ColorSupport};
use crate::game::game_state::GameState;
use crate::game::match_settings::MatchFormat;
use crate::game::rules::Ruleset;
use crate::players::ai_difficulty::AiDifficulty;
use crate::players::player::{Player, PlayerType};
use crate::utils::input::{read_nonempty, read_number};

pub fn setup_new_game(color_support: ColorSupport) -> GameState {
    clear_screen_soft();

    /* Main Header for SETUP */
    println!("{}", section_divider());
    println!(
        "{}",
        header(" ⚡ CREATE NEW MATCH ⚡ ", color_support)
    );
    println!("{}", section_divider());
    println!();

    // Player 1 Name
    println!(
        "{}",
        info("🎮 Player 1 — enter your name:", color_support)
    );
    let p1_name = read_nonempty(">> ");

    println!();
    println!("{}", section_divider());
    println!(
        "{}",
        header(" 🎮 GAME MODE ", color_support)
    );
    println!("{}", section_divider());
    println!(
        "{} {}",
        accent("1)", color_support),
        "Single Player  (vs AI 🤖)"
    );
    println!(
        "{} {}",
        accent("2)", color_support),
        "Multiplayer   (Local PvP 🧑‍🤝‍🧑)"
    );

    let mode = loop {
        let choice = read_number("\nSelect game mode (1-2): ");
        match choice {
            1 | 2 => break choice,
            _ => println!("{}", subtle("Please enter 1 or 2.", color_support)),
        }
    };

    let mut player1 = Player::new_human(&p1_name);
    let mut player2: Player;

    let ai_difficulty: Option<AiDifficulty>;

    match mode {
        1 => {
            // Singleplayer
            println!();
            println!("{}", section_divider());
            println!(
                "{}",
                header(" 🤖 AI DIFFICULTY ", color_support)
            );
            println!("{}", section_divider());
            println!(
                "{} {}",
                accent("1)", color_support),
                "Easy   – random choices"
            );
            println!(
                "{} {}",
                accent("2)", color_support),
                "Normal – weighted + counters"
            );
            println!(
                "{} {}",
                accent("3)", color_support),
                "Hard   – pattern tracking"
            );

            let diff = loop {
                let choice = read_number("\nSelect difficulty (1-3): ");
                match choice {
                    1 => break AiDifficulty::Easy,
                    2 => break AiDifficulty::Normal,
                    3 => break AiDifficulty::Hard,
                    _ => println!("{}", subtle("Please enter 1, 2 or 3.", color_support)),
                }
            };

            ai_difficulty = Some(diff.clone());
            player2 = Player::new_ai("Computer", diff);
        }
        2 => {
            // Multiplayer
            println!();
            println!("{}", section_divider());
            println!(
                "{}",
                info("🎮 Player 2 — enter your name:", color_support)
            );
            let p2_name = read_nonempty(">> ");
            player2 = Player::new_human(&p2_name);
            ai_difficulty = None;
        }
        _ => unreachable!(),
    }

    // Ruleset
    println!();
    println!("{}", section_divider());
    println!(
        "{}",
        header(" 📜 RULESET ", color_support)
    );
    println!("{}", section_divider());
    println!(
        "{} {}",
        accent("1)", color_support),
        "Classic  (Rock, Paper, Scissors)"
    );
    println!(
        "{} {}",
        accent("2)", color_support),
        "Extended (Rock, Paper, Scissors, Lizard, Spock)"
    );

    let ruleset = loop {
        let choice = read_number("\nSelect ruleset (1-2): ");
        match choice {
            1 => break Ruleset::Classic,
            2 => break Ruleset::Extended,
            _ => println!("{}", subtle("Please enter 1 or 2.", color_support)),
        }
    };

    // Match Format
    println!();
    println!("{}", section_divider());
    println!(
        "{}",
        header(" 🎯 MATCH FORMAT ", color_support)
    );
    println!("{}", section_divider());
    println!(
        "{} {}",
        accent("1)", color_support),
        "Single round"
    );
    println!(
        "{} {}",
        accent("2)", color_support),
        "Best of N (N odd, e.g. 3, 5, 7)"
    );
    println!(
        "{} {}",
        accent("3)", color_support),
        "First to K wins"
    );

    let match_format = loop {
        let choice = read_number("\nSelect format (1-3): ");
        match choice {
            1 => break MatchFormat::SingleRound,
            2 => {
                let n = loop {
                    let val = read_number("Enter odd N (3,5,7,...): ");
                    if val >= 1 && val % 2 == 1 {
                        break val;
                    } else {
                        println!("{}", subtle("N must be an odd positive number.", color_support));
                    }
                };
                break MatchFormat::BestOf(n);
            }
            3 => {
                let k = loop {
                    let val = read_number("Enter K (1+): ");
                    if val >= 1 {
                        break val;
                    } else {
                        println!("{}", subtle("K must be at least 1.", color_support));
                    }
                };
                break MatchFormat::FirstTo(k);
            }
            _ => println!("{}", subtle("Please enter 1, 2 or 3.", color_support)),
        }
    };

    println!();
    println!("{}", section_divider());
    println!(
        "{}",
        header(" ✅ MATCH READY ", color_support)
    );
    println!("{}", section_divider());
    println!(
        "{} {} vs {}",
        accent("Players:", color_support),
        player1.name,
        player2.name
    );
    println!(
        "{} {:?}",
        accent("Ruleset:", color_support),
        ruleset
    );
    println!(
        "{} {:?}",
        accent("Format:", color_support),
        match_format
    );
    if let Some(diff) = ai_difficulty {
        println!(
            "{} {:?}",
            accent("AI Difficulty:", color_support),
            diff
        );
    }
    println!();
    println!("{}", subtle("Press ENTER to start the match...", color_support));
    let _ = read_input_silent();

    GameState::new(player1, player2, ruleset, match_format)
}


fn clear_screen_soft() {
    /* We don't use full clear to keep neon banner */
    println!("\n\n");
}

/* Waiting for ENTER without any message */
fn read_input_silent() -> String {
    use std::io::{self, Read};

    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    s
}
