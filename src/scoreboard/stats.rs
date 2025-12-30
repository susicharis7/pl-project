use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub matches_played: u32,
    pub matches_won: u32,
    pub rounds_won: u32,
}

impl PlayerStats {
    pub fn new() -> Self {
        Self {
            matches_played: 0,
            matches_won: 0,
            rounds_won: 0,
        }
    }

    pub fn register_match(&mut self, rounds_won_in_match: u32, win: bool) {
        self.matches_played += 1;
        self.rounds_won += rounds_won_in_match;
        if win {
            self.matches_won += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MatchOutcome {
    Player1Win,
    Player2Win,
    Tie,
}
