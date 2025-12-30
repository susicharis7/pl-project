use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::stats::{MatchOutcome, PlayerStats};
use crate::io::save_load::load_scoreboard_only;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scoreboard {
    pub players: HashMap<String, PlayerStats>,
}

impl Scoreboard {
    pub fn new() -> Self {
        Scoreboard {
            players: HashMap::new(),
        }
    }

    pub fn load() -> Self {
        load_scoreboard_only()
    }

    pub fn save(&self) -> Result<(), String> {
        use crate::io::file_manager::write_to_file;
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Serialize scoreboard failed: {e}"))?;
        write_to_file("saves/scoreboard.json", &json)
            .map_err(|e| format!("Write scoreboard file failed: {e}"))?;
        Ok(())
    }

    fn ensure_player(&mut self, name: &str) {
        self.players
            .entry(name.to_string())
            .or_insert_with(PlayerStats::new);
    }

    pub fn update_after_match(
    &mut self,
    player1_name: &str,
    player2_name: &str,
    rounds_won_p1: u32,
    rounds_won_p2: u32,
    outcome: MatchOutcome,
) {
    self.ensure_player(player1_name);
    self.ensure_player(player2_name);

    // Temporary take the stats out to avoid double mutable borrow
    let mut stats1 = self.players.get(player1_name).unwrap().clone();
    let mut stats2 = self.players.get(player2_name).unwrap().clone();

    match outcome {
        MatchOutcome::Player1Win => {
            stats1.register_match(rounds_won_p1, true);
            stats2.register_match(rounds_won_p2, false);
        }
        MatchOutcome::Player2Win => {
            stats1.register_match(rounds_won_p1, false);
            stats2.register_match(rounds_won_p2, true);
        }
        MatchOutcome::Tie => {
            stats1.register_match(rounds_won_p1, false);
            stats2.register_match(rounds_won_p2, false);
        }
    }

    // Now write back values
    self.players.insert(player1_name.to_string(), stats1);
    self.players.insert(player2_name.to_string(), stats2);
}


    // Returns players sorted by matches won (descending)
    pub fn sorted_by_wins(&self) -> Vec<(&str, &PlayerStats)> {
        let mut v: Vec<(&str, &PlayerStats)> =
            self.players.iter().map(|(k, v)| (k.as_str(), v)).collect();
        v.sort_by(|(_, a), (_, b)| b.matches_won.cmp(&a.matches_won));
        v
    }

    // Returns players sorted by win rate (descending)
    pub fn sorted_by_win_rate(&self) -> Vec<(&str, &PlayerStats)> {
        let mut v: Vec<(&str, &PlayerStats)> =
            self.players.iter().map(|(k, v)| (k.as_str(), v)).collect();

        v.sort_by(|(_, a), (_, b)| {
            let rate_a = if a.matches_played > 0 {
                a.matches_won as f64 / a.matches_played as f64
            } else {
                0.0
            };
            let rate_b = if b.matches_played > 0 {
                b.matches_won as f64 / b.matches_played as f64
            } else {
                0.0
            };
            rate_b
                .partial_cmp(&rate_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        v
    }
}
