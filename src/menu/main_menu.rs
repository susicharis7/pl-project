use crate::display::colors::ColorSupport;
use crate::display::ui::{
    ask_main_menu_choice, show_main_menu, show_scoreboard, show_scoreboard_sort_menu,
};
use crate::game::game_loop::run_match;
use crate::game::game_state::GameState;
use crate::io::save_load::load_game_state_and_scoreboard;
use crate::menu::settings_menu::setup_new_game;
use crate::scoreboard::scoreboard::Scoreboard;

pub fn run_main_menu() {
    let mut scoreboard = Scoreboard::load();
    let color_support = ColorSupport::Enabled;

    let mut maybe_saved_state: Option<GameState> = None;

    loop {
        show_main_menu();
        let choice = ask_main_menu_choice();

        match choice {
            1 => {
                // New Game
                let state = setup_new_game(color_support);
                let (updated_scoreboard, state_opt) =
                    run_match(state, scoreboard.clone(), color_support);
                scoreboard = updated_scoreboard;
                maybe_saved_state = state_opt;
            }
            2 => {
                // Continue saved
                match load_game_state_and_scoreboard() {
                    Ok((state, sb)) => {
                        let (updated_scoreboard, state_opt) =
                            run_match(state, sb.clone(), color_support);
                        scoreboard = updated_scoreboard;
                        maybe_saved_state = state_opt;
                    }
                    Err(e) => {
                        println!("Could not load saved game: {e}");
                    }
                }
            }
            3 => {
                // View scoreboard
                let sorted_choice = show_scoreboard_sort_menu();
                match sorted_choice {
                    1 => {
                        show_scoreboard(&scoreboard);
                    }
                    2 => {
                        println!("\n=== Scoreboard sorted by win rate ===");
                        let sorted = scoreboard.sorted_by_win_rate();
                        if sorted.is_empty() {
                            println!("No player statistics yet.");
                        } else {
                            println!("{:<20} {:>8} {:>8} {:>8}", "Player", "Matches", "Wins", "Win %");
                            for (name, stats) in sorted {
                                let win_rate = if stats.matches_played > 0 {
                                    (stats.matches_won as f64 / stats.matches_played as f64) * 100.0
                                } else {
                                    0.0
                                };
                                println!(
                                    "{:<20} {:>8} {:>8} {:>7.2}%",
                                    name, stats.matches_played, stats.matches_won, win_rate
                                );
                            }
                        }
                    }
                    _ => {
                        println!("Invalid choice.");
                    }
                }
            }
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Try again."),
        }
    }
}