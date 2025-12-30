use crate::display::colors::ColorSupport;
use crate::display::ui::{
    show_match_actions_menu, show_match_victory, show_round_summary, show_save_prompt,
    show_move_prompt,
};
use crate::game::game_state::GameState;
use crate::game::round::{resolve_round, RoundResult};
use crate::io::save_load::save_game_state_and_scoreboard;
use crate::players::ai::generate_ai_move;
use crate::players::player::PlayerType;
use crate::scoreboard::scoreboard::Scoreboard;
use crate::scoreboard::stats::MatchOutcome;
use crate::utils::clear_screen::clear_screen;

use crate::game::rules::Gesture;

/* Runs a FULL MATCH Untill Someone Wins */
pub fn run_match(
    mut state: GameState,
    mut scoreboard: Scoreboard,
    color_support: ColorSupport,
) -> (Scoreboard, Option<GameState>) {
    let required_wins = state.match_format.required_wins();

    loop {
        clear_screen();
        println!();
        println!("============================================");
        println!(
            "     ⚔️  ROUND {} — {} vs {}  ⚔️",
            state.current_round, state.player1.name, state.player2.name
        );
        println!("============================================\n");

        println!(
            "Score →  {} {}  -  {} {}",
            state.player1.name,
            state.score_player1,
            state.score_player2,
            state.player2.name
        );
        println!("First to {} wins\n", required_wins);

        /* MOVE COLLECTION WITH ULTRA ARCADE UI */
        let (g1, g2) = get_round_moves(&state, color_support);

        let result = resolve_round(&mut state, g1, g2);
        show_round_summary(&state, &result, color_support);

        /* Victory condition */
        if state.score_player1 >= required_wins || state.score_player2 >= required_wins {
            let winner = if state.score_player1 > state.score_player2 {
                Some(state.player1.name.clone())
            } else if state.score_player2 > state.score_player1 {
                Some(state.player2.name.clone())
            } else {
                None
            };

            show_match_victory(&state, winner.clone(), color_support);

            /* Scoreboard Update */
            scoreboard.update_after_match(
                &state.player1.name,
                &state.player2.name,
                state.score_player1,
                state.score_player2,
                match winner {
                    Some(ref w) if w == &state.player1.name => MatchOutcome::Player1Win,
                    Some(ref w) if w == &state.player2.name => MatchOutcome::Player2Win,
                    _ => MatchOutcome::Tie,
                },
            );

            let _ = scoreboard.save();

            /* End Of Match MENU */
            match show_match_actions_menu() {
                1 => {
                    state.reset_for_rematch();
                    continue;
                }
                2 => return (scoreboard, None),
                3 => return (scoreboard, Some(state)),
                _ => return (scoreboard, None),
            }
        }

        /* Ask if player wants to save & exit */
        if show_save_prompt() {
            let _ = save_game_state_and_scoreboard(&state, &scoreboard);
            println!("Game saved. Returning to menu...");
            return (scoreboard, Some(state));
        }

        state.current_round += 1;
    }
}

/* Collects moves for the round */ 
fn get_round_moves(state: &GameState, color_support: ColorSupport) -> (Gesture, Gesture) {
    let ruleset = &state.ruleset;

    let history_p1 = &state.history_p1;
    let history_p2 = &state.history_p2;

    
    /* PLAYER 1 MOVE (HUMAN / AI) */
    let g1 = match &state.player1.player_type {
        PlayerType::Human => {
            let g = show_move_prompt(&state.player1.name, ruleset, color_support);  
            g.parse::<Gesture>().unwrap()
        }
        PlayerType::Computer(diff) => generate_ai_move(diff.clone(), ruleset, history_p2),
    };

    /* CLEAN SCREEN BETWEEN PLAYERS */
    if let PlayerType::Human = state.player2.player_type {
        clear_screen();
    }

   
    /* PLAYER 2 MOVE (HUMAN / AI) */
    let g2 = match &state.player2.player_type {
        PlayerType::Human => {
            let g = show_move_prompt(&state.player2.name, ruleset, color_support);  
            g.parse::<Gesture>().unwrap()
        }
        PlayerType::Computer(diff) => generate_ai_move(diff.clone(), ruleset, history_p1),
    };

    (g1, g2)
}