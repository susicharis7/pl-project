use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

use super::ai_difficulty::AiDifficulty;
use crate::game::rules::{beats, Gesture, Ruleset};

/* AI Move based on HARD & History of players moves */
pub fn generate_ai_move(
    difficulty: AiDifficulty,
    ruleset: &Ruleset,
    player_history: &[Gesture],
) -> Gesture {
    match difficulty {
        AiDifficulty::Easy => generate_easy_move(ruleset),
        AiDifficulty::Normal => generate_normal_move(ruleset, player_history),
        AiDifficulty::Hard => generate_hard_move(ruleset, player_history),
    }
}

// EASY
fn generate_easy_move(ruleset: &Ruleset) -> Gesture {
    let allowed = Gesture::gestures_for_ruleset(ruleset);
    let mut rng = thread_rng();
    allowed.choose(&mut rng).unwrap().clone()
}

// NORMAL: Weighted Random + Counter 
fn generate_normal_move(ruleset: &Ruleset, history: &[Gesture]) -> Gesture {
    if history.is_empty() {
        return generate_easy_move(ruleset);
    }

    // Frequency Counter 
    let mut counts: HashMap<Gesture, u32> = HashMap::new();
    for g in history {
        *counts.entry(g.clone()).or_insert(0) += 1;
    }

    let predicted = counts
        .into_iter()
        .max_by_key(|(_, c)| *c)
        .map(|(g, _)| g)
        .unwrap_or_else(|| history[history.len() - 1].clone());

    // Find all moves that are going to win -> "predicted"
    let allowed = Gesture::gestures_for_ruleset(ruleset);
    let counters: Vec<Gesture> = allowed
        .into_iter()
        .filter(|g| beats(g, &predicted))
        .collect();

    if counters.is_empty() {
        generate_easy_move(ruleset)
    } else {
        let mut rng = thread_rng();
        counters.choose(&mut rng).unwrap().clone()
    }
}


/* HARD - Pattern Tracking (recent moves)
 * Watches last 5 moves 
 * New recent moves have higher weight
 * Calculates Weighted most frequent gesture
 * Chooses the move that beats predicted gesture
 * With a small probability (20%) - inserts a random move to avoid being trivially predictable
*/
fn generate_hard_move(ruleset: &Ruleset, history: &[Gesture]) -> Gesture {
    if history.is_empty() {
        return generate_easy_move(ruleset);
    }

    let mut rng = thread_rng();

    // 20% šansa for AI to play completely random
    if rng.r#gen::<f64>() < 0.2 {
        return generate_easy_move(ruleset);
    }

    let len = history.len();
    let n = len.min(5);

    let mut weighted_counts: HashMap<Gesture, u32> = HashMap::new();

    // Last move - Highest Weight
    for offset in 0..n {
        let idx = len - 1 - offset;
        let g = history[idx].clone();
        let weight = (n - offset) as u32;
        *weighted_counts.entry(g).or_insert(0) += weight;
    }

    let predicted = weighted_counts
        .into_iter()
        .max_by_key(|(_, w)| *w)
        .map(|(g, _)| g)
        .unwrap_or_else(|| history[history.len() - 1].clone());

    let allowed = Gesture::gestures_for_ruleset(ruleset);
    let counters: Vec<Gesture> = allowed
        .into_iter()
        .filter(|g| beats(g, &predicted))
        .collect();

    if counters.is_empty() {
        generate_easy_move(ruleset)
    } else {
        counters.choose(&mut rng).unwrap().clone()
    }
}
