use serde::{Deserialize, Serialize};

use super::ai_difficulty::AiDifficulty;

// Type of Player :: Human or Computer with Difficulty
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerType {
    Human,
    Computer(AiDifficulty),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub player_type: PlayerType,
}

impl Player {
    pub fn new_human(name: &str) -> Self {
        Self {
            name: name.to_string(),
            player_type: PlayerType::Human,
        }
    }

    pub fn new_ai(name: &str, difficulty: AiDifficulty) -> Self {
        Self {
            name: name.to_string(),
            player_type: PlayerType::Computer(difficulty),
        }
    }
}

