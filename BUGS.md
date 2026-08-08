# List of Bugs

## Gameplay Bugs

### Major Bugs

- Virgin and slayer will keep their abilities if they are poisoned when they use them
  - This is because poison only affects setup and night(one) abilities
- Sometimes button has to be pressed twice to skip through setup
- Soldier doesn't die when poisoned
- Scarletwoman doesn't kill first night when she switches sometimes
- `win_cond_i` (the tracked win-condition/demon player index used by `State::game_over()`) is never updated when demonhood transfers to a new player (e.g. Scarletwoman becoming the demon)
  - `State::new()` builds a `_demon_listener` that correctly re-points tracking to the new alive demon (or ends the game if none exists), but it's never registered into `state.death_listeners` (it's an unused local, prefixed with `_`) — see the `EventListener::new(win_cond_index, ...)` block in `state.rs`
  - As a result, `game_over()` keeps checking the *original* demon's `dead` flag forever, so the moment the original demon dies — even via a successful Scarletwoman transfer — the game is reported over and Good is shown as the winner (see the end-of-game screen), even though Scarletwoman correctly inherited the demon role and the game should continue
  - Related to the "Scarletwoman doesn't kill first night when she switches sometimes" entry above, but distinct: this is about win-condition/game-over tracking, not the new demon's night-ability timing

### Minor Bugs

## UI Bugs

### Major Bugs

### Minor Bugs
