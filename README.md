# 🎮 NEON RPS - Rock Paper Scissors

A feature-rich, terminal-based Rock-Paper-Scissors game written in Rust. Supports both Classic (RPS) and Extended (Rock-Paper-Scissors-Lizard-Spock) rulesets with single-player AI and local multiplayer modes.

---

## 📋 Table of Contents

- [Features](#-features)
- [Screenshots](#-screenshots)
- [Installation](#-installation)
- [How to Play](#-how-to-play)
- [Game Rules](#-game-rules)
- [Project Structure](#-project-structure)
- [Technologies Used](#-technologies-used)
- [License](#-license)

---

## ✨ Features

### Game Modes
- **Single Player** - Play against AI with 3 difficulty levels
- **Multiplayer** - Local 2-player mode with hidden moves

### Rulesets
- **Classic RPS** - Rock, Paper, Scissors
- **Extended RPSLS** - Rock, Paper, Scissors, Lizard, Spock

### Match Formats
- **Single Round** - Quick one-round match
- **Best of N** - First to win majority (N = 3, 5, 7...)
- **First to K** - First player to reach K wins

### AI Difficulty Levels
| Level | Strategy |
|-------|----------|
| 🟢 Easy | Uniformly random choices |
| 🟡 Normal | Weighted randomness + counter heuristics |
| 🔴 Hard | Pattern tracking of recent moves |

### Additional Features
- 🎨 **Colorful Terminal UI** - Neon arcade-style graphics
- 💾 **Save/Load System** - Resume your match anytime
- 📊 **Persistent Scoreboard** - Track player statistics across sessions
- 🔄 **Rematch Option** - Quick replay with same settings

---

## 📸 Screenshots

```
████████████████████████████████████████
█    NEON RPS // PROGRAMMING LANGS     █
████████████████████████████████████████
────────────────────────────────────────
              MAIN MENU
────────────────────────────────────────

1) Start New Game
2) Continue Saved Game
3) View Scoreboard
4) Exit
```

```
════════════════════════════════════════
     ⚔️  ROUND 1 — Alice vs Bob  ⚔️
════════════════════════════════════════

Score →  Alice 0  -  0 Bob
First to 3 wins

████████████████████████████████████████
█            CHOOSE MOVE               █
████████████████████████████████████████

   ▓ 1. ROCK     ⛰
   ▓ 2. PAPER    📄
   ▓ 3. SCISSORS ✂️
```

---

## 🚀 Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (version 1.75 or higher)

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/neon-rps.git
cd neon-rps

# Build the project
cargo build --release

# Run the game
cargo run --release
```

### Quick Start

```bash
cargo run
```

---

## 🎯 How to Play

### Starting a New Game

1. Select **"Start New Game"** from the main menu
2. Enter your name
3. Choose game mode (Single Player / Multiplayer)
4. Select AI difficulty (Single Player only)
5. Choose ruleset (Classic / Extended)
6. Select match format
7. Play!

### Controls

| Input | Action |
|-------|--------|
| `1` or `r` | Rock |
| `2` or `p` | Paper |
| `3` or `s` | Scissors |
| `4` or `l` | Lizard (Extended only) |
| `5` or `k` | Spock (Extended only) |

### Saving Your Game

During a match, when prompted "Save and return to main menu?", enter `y` to save your progress.

### Loading a Saved Game

Select **"Continue Saved Game"** from the main menu to resume your last saved match.

---

## 📜 Game Rules

### Classic RPS
```
Rock     → crushes    → Scissors
Scissors → cuts       → Paper
Paper    → covers     → Rock
```

### Extended RPSLS (Rock-Paper-Scissors-Lizard-Spock)
```
Rock     → crushes    → Scissors, Lizard
Paper    → covers     → Rock, Spock
Scissors → cuts       → Paper, Lizard
Lizard   → poisons    → Spock, eats Paper
Spock    → smashes    → Scissors, vaporizes Rock
```

---

## 📁 Project Structure

```
neon-rps/
├── Cargo.toml              # Project dependencies
├── README.md               # This file
├── saves/                  # Save files (auto-generated)
│   ├── save_state.json     # Current game state
│   └── scoreboard.json     # Player statistics
└── src/
    ├── main.rs             # Entry point
    ├── display/            # UI and visuals
    │   ├── ascii.rs        # ASCII art banners
    │   ├── colors.rs       # Terminal colors
    │   └── ui.rs           # User interface
    ├── game/               # Core game logic
    │   ├── game_loop.rs    # Main game loop
    │   ├── game_state.rs   # Game state management
    │   ├── match_settings.rs # Match format definitions
    │   ├── round.rs        # Round resolution
    │   └── rules.rs        # RPS/RPSLS rules
    ├── io/                 # File operations
    │   ├── file_manager.rs # File read/write
    │   └── save_load.rs    # Save/Load system
    ├── menu/               # Menu screens
    │   ├── main_menu.rs    # Main menu logic
    │   └── settings_menu.rs # Game setup
    ├── players/            # Player management
    │   ├── ai.rs           # AI strategies
    │   ├── ai_difficulty.rs # Difficulty levels
    │   └── player.rs       # Player struct
    ├── scoreboard/         # Statistics
    │   ├── scoreboard.rs   # Scoreboard management
    │   └── stats.rs        # Player stats
    └── utils/              # Utilities
        ├── clear_screen.rs # Cross-platform screen clear
        └── input.rs        # Input handling
```

---

## 🛠️ Technologies Used

- **Language:** Rust 2024 Edition
- **Dependencies:**
  - `colored` - Terminal colors
  - `rand` - Random number generation
  - `serde` / `serde_json` - JSON serialization

---

## 📊 Scoreboard

The game tracks the following statistics for each player:

| Statistic | Description |
|-----------|-------------|
| Matches Played | Total number of matches |
| Matches Won | Number of matches won |
| Rounds Won | Total rounds won across all matches |
| Win Rate | Percentage of matches won |

Statistics persist across sessions and can be sorted by wins or win rate.

---

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👤 Author

**Haris Sušić**
- GitHub: [@susicharis7](https://github.com/susicharis7)

**Tarik Skender**
- GutHub: [@tarikskender04](https://github.com/tarikskender04)

---

## 🙏 Acknowledgments

- Inspired by the classic Rock-Paper-Scissors game
- Extended rules from "The Big Bang Theory" (Rock-Paper-Scissors-Lizard-Spock)

---

<p align="center">
  Made with ❤️ and Rust 🦀
</p>
