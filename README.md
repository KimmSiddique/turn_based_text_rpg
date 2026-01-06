# Turn-Based Text RPG (Rust)

A terminal-based, turn-based battle game written in Rust.  
Two players build teams from elemental troops (Fire / Water / Rock) and battle until one side runs out of troops.

The project also tracks game statistics (battles played, rounds played, wins, draws) using JSON save/load via `serde`.

---

## Features

- **Two-player turn-based battle system**
- **Troop drafting**: each player picks 3 troops from a shared list
- **Battle Points (BP)** system: BP increases each round (faster after round 5)
- **Win conditions**
  - Player 1 wins if Player 2 has no troops remaining
  - Player 2 wins if Player 1 has no troops remaining
  - Draw if both teams are empty at the same time
- **Persistent stats**
  - Saved to `assets/stats.json`
  - Loaded automatically on startup (if present)
- **Instructions file**
  - Displayed from `assets/instructions.txt`

---

## Requirements

- Rust (stable) + Cargo  
  Install from: https://rustup.rs/

---

## Project Layout (key files)

- `src/` - Rust source code
- `assets/instructions.txt` - in-game instructions text
- `assets/stats.json` - saved statistics (auto-created/updated by the game)

---

## Build & Run

### Debug build (faster compile, slower runtime)
```bash
cargo run
