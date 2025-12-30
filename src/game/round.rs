use super::game_state::GameState;
use super::rules::{beats, Gesture};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RoundResult {
    Player1Win,
    Player2Win,
    Tie,
}

// Process a Single Round >> Updates GameState and Returns The Result
pub fn resolve_round(
    state: &mut GameState,
    gesture_p1: Gesture,
    gesture_p2: Gesture,
) -> RoundResult {
    state.last_move_p1 = Some(gesture_p1.clone());
    state.last_move_p2 = Some(gesture_p2.clone());

    /* Write to History for Analytics/AI */
    state.history_p1.push(gesture_p1.clone());
    state.history_p2.push(gesture_p2.clone());

    let result = if gesture_p1 == gesture_p2 {
        RoundResult::Tie
    } else if beats(&gesture_p1, &gesture_p2) {
        RoundResult::Player1Win
    } else if beats(&gesture_p2, &gesture_p1) {
        RoundResult::Player2Win
    } else {
        RoundResult::Tie
    };

    match result {
        RoundResult::Player1Win => state.score_player1 += 1,
        RoundResult::Player2Win => state.score_player2 += 1,
        RoundResult::Tie => {}
    }


    result
}