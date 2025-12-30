use serde::{Deserialize, Serialize};

use super::match_settings::MatchFormat;
use super::rules::{Gesture, Ruleset};
use crate::players::player::Player;

/* Represents the entire current match state (used for Save/Load). */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player1: Player,
    pub player2: Player,

    pub ruleset: Ruleset,
    pub match_format: MatchFormat,

    pub score_player1: u32,
    pub score_player2: u32,

    pub current_round: u32,

    pub last_move_p1: Option<Gesture>,
    pub last_move_p2: Option<Gesture>,

    /* History of all gestures that Player 1 has played (for analytics / AI). */
    pub history_p1: Vec<Gesture>,

    /* History of all gestures that Player 2 has played (for analytics / AI). */
    pub history_p2: Vec<Gesture>,
}

impl GameState {
    pub fn new(
        player1: Player,
        player2: Player,
        ruleset: Ruleset,
        match_format: MatchFormat,
    ) -> Self {
        GameState {
            player1,
            player2,
            ruleset,
            match_format,
            score_player1: 0,
            score_player2: 0,
            current_round: 1,
            last_move_p1: None,
            last_move_p2: None,
            history_p1: Vec::new(),
            history_p2: Vec::new(),
        }
    }

    /* Reset score, round and histories for rematch. */
    pub fn reset_for_rematch(&mut self) {
        self.score_player1 = 0;
        self.score_player2 = 0;
        self.current_round = 1;
        self.last_move_p1 = None;
        self.last_move_p2 = None;
        self.history_p1.clear();
        self.history_p2.clear();
    }
}
