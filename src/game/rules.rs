use serde::{Deserialize, Serialize};

// All Possible Gestures in The Game (supports Classic and Extended)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Gesture {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

use std::str::FromStr;

impl FromStr for Gesture {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rock" | "r" => Ok(Gesture::Rock),
            "paper" | "p" => Ok(Gesture::Paper),
            "scissors" | "s" => Ok(Gesture::Scissors),
            "lizard" | "l" => Ok(Gesture::Lizard),
            "spock" | "k" => Ok(Gesture::Spock),
            _ => Err(()),
        }
    }
}


// Ruleset: Classic (RPS) or Extended (RPSLS)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Ruleset {
    Classic,
    Extended,
}

impl Gesture {
    // All Valid Gestures for a Given Ruleset.
    pub fn gestures_for_ruleset(ruleset: &Ruleset) -> Vec<Gesture> {
        match ruleset {
            Ruleset::Classic => vec![Gesture::Rock, Gesture::Paper, Gesture::Scissors],
            Ruleset::Extended => vec![
                Gesture::Rock,
                Gesture::Paper,
                Gesture::Scissors,
                Gesture::Lizard,
                Gesture::Spock,
            ],
        }
    }

    // Friendly String for Display.
    pub fn as_str(&self) -> &'static str {
        match self {
            Gesture::Rock => "Rock",
            Gesture::Paper => "Paper",
            Gesture::Scissors => "Scissors",
            Gesture::Lizard => "Lizard",
            Gesture::Spock => "Spock",
        }
    }
}

// Returns true if `a` beats `b` in RPSLS rules
pub fn beats(a: &Gesture, b: &Gesture) -> bool {
    use Gesture::*;

    matches!(
        (a, b),
        (Rock, Scissors)
            | (Rock, Lizard)
            | (Paper, Rock)
            | (Paper, Spock)
            | (Scissors, Paper)
            | (Scissors, Lizard)
            | (Lizard, Spock)
            | (Lizard, Paper)
            | (Spock, Scissors)
            | (Spock, Rock)
    )
}
