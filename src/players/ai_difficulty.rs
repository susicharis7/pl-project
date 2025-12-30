use serde::{Deserialize, Serialize};

// AI difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AiDifficulty {
    Easy,
    Normal,
    Hard,
}
