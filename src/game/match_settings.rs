use serde::{Deserialize, Serialize};

/* Different match formats */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MatchFormat {
    // Single round: first WIN ends the match
    SingleRound,

    // Best of N rounds (N must be odd, e.g., 3, 5, 7)
    BestOf(u32),

    // First to K wins (first player to reach K wins)
    FirstTo(u32),
}

impl MatchFormat {
    // How many wins are required to win the match
    pub fn required_wins(&self) -> u32 {
        match *self {
            MatchFormat::SingleRound => 1,
            MatchFormat::BestOf(n) => (n / 2) + 1,
            MatchFormat::FirstTo(k) => k,
        }
    }
}
