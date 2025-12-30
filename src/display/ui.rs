use crate::display::ascii::{
    ascii_gesture, main_menu_banner, section_divider, tie_banner, winner_banner,
    move_selection_banner,
};

use crate::display::colors::{accent, header, info, subtle, success, failure, ColorSupport};

use crate::game::game_state::GameState;
use crate::game::round::RoundResult;
use crate::game::rules::Ruleset;  // DODANO
use crate::scoreboard::scoreboard::Scoreboard;
use crate::scoreboard::stats::PlayerStats;
use crate::utils::input::{read_input, read_number};

/* ROUND SUMMARY */
pub fn show_round_summary(state: &GameState, result: &RoundResult, color_support: ColorSupport) {
    println!();
    println!("{}", section_divider());
    println!("{}", header(" ROUND SUMMARY ", color_support));
    println!("{}", section_divider());
    println!();

    if let (Some(p1), Some(p2)) = (&state.last_move_p1, &state.last_move_p2) {
        println!(
            "{}  {} {}",
            accent(&format!("{}:", state.player1.name), color_support),
            p1.as_str(),
            ascii_gesture(p1)
        );
        println!(
            "{}  {} {}",
            accent(&format!("{}:", state.player2.name), color_support),
            p2.as_str(),
            ascii_gesture(p2)
        );
        println!();
    }

    match result {
        RoundResult::Player1Win => println!(
            "{}",
            success(&format!(">> {} wins the round!", state.player1.name), color_support)
        ),
        RoundResult::Player2Win => println!(
            "{}",
            success(&format!(">> {} wins the round!", state.player2.name), color_support)
        ),
        RoundResult::Tie => println!("{}", info(">> The round is a tie.", color_support)),
    }

    println!();
    println!(
        "{}  {} {}  -  {} {}",
        accent("Score:", color_support),
        state.player1.name,
        state.score_player1,
        state.player2.name,
        state.score_player2
    );

    let required = state.match_format.required_wins();
    let remain_p1 = required.saturating_sub(state.score_player1);
    let remain_p2 = required.saturating_sub(state.score_player2);

    println!(
        "{} {} needs {}, {} needs {}",
        subtle("Wins to go:", color_support),
        state.player1.name,
        remain_p1,
        state.player2.name,
        remain_p2
    );
    println!();
}

/* MATCH VICTORY */
pub fn show_match_victory(
    state: &GameState,
    winner: Option<String>,
    color_support: ColorSupport,
) {
    println!();
    println!("{}", section_divider());

    match winner {
        Some(name) => {
            println!("{}", header(winner_banner(), color_support));
            println!("{}", success(&format!("{} wins the match!", name), color_support));
        }
        None => {
            println!("{}", header(tie_banner(), color_support));
            println!("{}", info("The match ended in a tie.", color_support));
        }
    }

    println!(
        "{} {} {}  -  {} {}",
        accent("Final Score:", color_support),
        state.player1.name,
        state.score_player1,
        state.player2.name,
        state.score_player2
    );

    /* Player Analytics */
    let total_rounds = state.history_p1.len() as u32;
    if total_rounds > 0 {
        let ties = total_rounds
            .saturating_sub(state.score_player1)
            .saturating_sub(state.score_player2);

        println!(
            "{} total rounds = {}, {} won {}, {} won {}, ties {}",
            subtle("Summary:", color_support),
            total_rounds,
            state.player1.name,
            state.score_player1,
            state.player2.name,
            state.score_player2,
            ties
        );

        if let Some(most) = most_frequent(&state.history_p1) {
            println!(
                "{} {} most played: {} {}",
                subtle("-", color_support),
                state.player1.name,
                most.as_str(),
                ascii_gesture(&most)
            );
        }

        if let Some(most) = most_frequent(&state.history_p2) {
            println!(
                "{} {} most played: {} {}",
                subtle("-", color_support),
                state.player2.name,
                most.as_str(),
                ascii_gesture(&most)
            );
        }
    }

    println!("{}", section_divider());
}

/* Most Frequent Gesture Helper */
fn most_frequent(history: &[crate::game::rules::Gesture]) -> Option<crate::game::rules::Gesture> {
    use crate::game::rules::Gesture;
    use std::collections::HashMap;

    let mut counts: HashMap<Gesture, u32> = HashMap::new();
    for g in history {
        *counts.entry(g.clone()).or_insert(0) += 1;
    }

    counts.into_iter().max_by_key(|(_, c)| *c).map(|(g, _)| g)
}

/* MAIN MENU */
pub fn show_main_menu() {
    println!("{}", main_menu_banner());
    println!("{}", section_divider());
    println!("{}", header(" MAIN MENU ", ColorSupport::Enabled));
    println!("{}", section_divider());
    println!();

    println!("{} {}", accent("1)", ColorSupport::Enabled), "Start New Game");
    println!("{} {}", accent("2)", ColorSupport::Enabled), "Continue Saved Game");
    println!("{} {}", accent("3)", ColorSupport::Enabled), "View Scoreboard");
    println!("{} {}", accent("4)", ColorSupport::Enabled), "Exit");
}

pub fn ask_main_menu_choice() -> u32 {
    println!();
    read_number("Enter choice: ")
}

/* SCOREBOARD */
pub fn show_scoreboard(scoreboard: &Scoreboard) {
    println!();
    println!("{}", section_divider());
    println!("{}", header(" SCOREBOARD ", ColorSupport::Enabled));
    println!("{}", section_divider());

    let sorted = scoreboard.sorted_by_wins();

    if sorted.is_empty() {
        println!("{}", subtle("No player statistics yet.", ColorSupport::Enabled));
        return;
    }

    println!("{:<20} {:>10} {:>10} {:>10}", "Player", "Matches", "Wins", "Win %");
    println!("{}", section_divider());

    for (name, stats) in sorted {
        print_player_line(name, stats);
    }
}

fn print_player_line(name: &str, stats: &PlayerStats) {
    let win_rate = if stats.matches_played > 0 {
        (stats.matches_won as f64 / stats.matches_played as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "{:<20} {:>10} {:>10} {:>9.2}%",
        name, stats.matches_played, stats.matches_won, win_rate
    );
}

pub fn show_scoreboard_sort_menu() -> u32 {
    println!();
    println!("{}", header(" SORT SCOREBOARD ", ColorSupport::Enabled));
    println!("{}", section_divider());
    println!("{} {}", accent("1)", ColorSupport::Enabled), "Matches won");
    println!("{} {}", accent("2)", ColorSupport::Enabled), "Win rate");
    read_number("Enter choice: ")
}

/* SAVE PROMPT */
pub fn show_save_prompt() -> bool {
    println!();
    let input = read_input("Save and return to main menu? (y/n): ");
    matches!(input.to_lowercase().as_str(), "y" | "yes")
}

/* END OF MATCH ACTIONS */
pub fn show_match_actions_menu() -> u32 {
    println!();
    println!("{}", header(" MATCH COMPLETE ", ColorSupport::Enabled));
    println!("{}", section_divider());

    println!("{} {}", accent("1)", ColorSupport::Enabled), "Rematch with same settings");
    println!("{} {}", accent("2)", ColorSupport::Enabled), "Change ruleset/format");
    println!("{} {}", accent("3)", ColorSupport::Enabled), "Return to main menu");

    read_number("Enter choice: ")
}


/* NEON ARCADE MOVE PICKER - DYNAMIC BASED ON RULESET */

pub fn show_move_prompt(player_name: &str, ruleset: &Ruleset, color: ColorSupport) -> String {
    println!("\n{}", move_selection_banner());
    println!("{} {}", accent("PLAYER:", color), player_name);
    
    /* Dynamic - based on ruleset */
    println!();
    println!("   ▓ 1. ROCK     ⛰");
    println!("   ▓ 2. PAPER    📄");
    println!("   ▓ 3. SCISSORS ✂️");
    
    if matches!(ruleset, Ruleset::Extended) {
        println!("   ▓ 4. LIZARD   🦎");
        println!("   ▓ 5. SPOCK    🖖");
    }
    println!();

    loop {
        let input = read_input(">> ").to_lowercase();

        match input.as_str() {
            "1" | "rock" | "r" => return "rock".into(),
            "2" | "paper" | "p" => return "paper".into(),
            "3" | "scissors" | "s" => return "scissors".into(),
            "4" | "lizard" | "l" if matches!(ruleset, Ruleset::Extended) => return "lizard".into(),
            "5" | "spock" | "k" if matches!(ruleset, Ruleset::Extended) => return "spock".into(),
            _ => {
                let hint = if matches!(ruleset, Ruleset::Extended) {
                    "Invalid input. Use 1-5 or r/p/s/l/k."
                } else {
                    "Invalid input. Use 1-3 or r/p/s."
                };
                println!("{}", failure(hint, color));
            }
        }
    }
}