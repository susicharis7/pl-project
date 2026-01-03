use crate::game::game_state::GameState;
use crate::io::file_manager::{read_from_file, write_to_file};
use crate::scoreboard::scoreboard::Scoreboard;

const SAVE_STATE_PATH: &str = "saves/save_state.json";
const SCOREBOARD_PATH: &str = "saves/scoreboard.json";

pub fn save_game_state_and_scoreboard(
    state: &GameState,
    scoreboard: &Scoreboard,
) -> Result<(), String> {
    let state_json =
        serde_json::to_string_pretty(state).map_err(|e| format!("Serialize state failed: {e}"))?;
    let sb_json = serde_json::to_string_pretty(scoreboard)
        .map_err(|e| format!("Serialize scoreboard failed: {e}"))?;

    write_to_file(SAVE_STATE_PATH, &state_json)
        .map_err(|e| format!("Write state file failed: {e}"))?;
    write_to_file(SCOREBOARD_PATH, &sb_json)
        .map_err(|e| format!("Write scoreboard file failed: {e}"))?;

    Ok(())
}

pub fn load_game_state_and_scoreboard() -> Result<(GameState, Scoreboard), String> {
    let state_str =
        read_from_file(SAVE_STATE_PATH).map_err(|e| format!("Read state failed: {e}"))?;
    let sb_str =
        read_from_file(SCOREBOARD_PATH).map_err(|e| format!("Read scoreboard failed: {e}"))?;

    let state: GameState =
        serde_json::from_str(&state_str).map_err(|e| format!("Parse state failed: {e}"))?;
    let sb: Scoreboard =
        serde_json::from_str(&sb_str).map_err(|e| format!("Parse scoreboard failed: {e}"))?;

    Ok((state, sb))
}

pub fn load_scoreboard_only() -> Scoreboard {
    match read_from_file(SCOREBOARD_PATH) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_else(|_| Scoreboard::new()),
        Err(_) => Scoreboard::new(),
    }
}


pub fn delete_save_state() {
    let _ = std::fs::remove_file("saves/save_state.json");
}
