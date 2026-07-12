# Blood on the Clocktower (BOTC) Fast Grimoire - Specification

"BOTC Fast Grimoire" is a Storyteller Assistance Tool for Blood on the Clocktower. It automates the grimoire logic (such as night order wakeups, character interactions, status effects, and rules verification) while allowing the Storyteller to make final, manual interventions or override automated outcomes. Currently, it implements the "Trouble Brewing" edition.

---

## 1. Tech Stack & Technologies

- **Language:** Rust (2024 Edition)
- **Target:** WebAssembly (wasm32-unknown-unknown)
- **Build/Dev Server:** Trunk (Trunk.toml)
- **Frontend Framework:** Leptos (v0.8.2) in client-side rendering (csr) mode, utilizing reactive_stores for global state synchronization.
- **Styling:** Tailwind CSS (v4) built via Trunk hooks calling @tailwindcss/cli.
- **Dependencies:** serde & serde_json for custom script inputs, rand for seating & role distributions, enum_dispatch for high-performance dynamic dispatch.

---

## 2. Directory Layout & Architecture

```text
├── Cargo.toml                  # Main dependencies
├── Trunk.toml                  # Trunk dev server & pre_build Tailwind config
├── index.html                  # Leptos web entry point
├── input.css                   # Custom Tailwind inputs
├── tailwind.config.js          # Tailwind styling definitions (if any)
├── requirements.md             # Developer roadmap & design requirements
├── src/
│   ├── main.rs                 # Web application entry, routing, stage management, and Leptos views
│   ├── initialization.rs       # Custom script deserializer & CharacterType count tables
│   ├── scripts.rs              # Hardcoded script templates (e.g., Trouble Brewing)
│   ├── template.rs             # Legacy/placeholder architecture examples
│   └── engine/                 # Core BOTC State and Rules Engine
│       ├── change_request.rs   # Storyteller interaction request and callback system
│       ├── engine.rs           # Core module export definitions
│       ├── player.rs           # Player structure, status managers, and "drunkify" utility
│       ├── state.rs            # Game state manager, game steps, event dispatchers
│       ├── state/
│       │   ├── log.rs          # Event tracking, history log, and rollback search engine
│       │   └── status_effects.rs # Player status effects definitions (Poisoned, Drunk, Protection)
│       └── player/
│           ├── roles.rs        # Roles dispatch enum and the `Role` trait
│           └── roles/          # Individual character behaviors grouped by category
│               ├── townsfolk.rs
│               ├── outsiders.rs
│               ├── minions.rs
│               └── demons.rs
└── macros/                     # Placeholder for procedurally generated helpers (empty)
```

---

## 3. Core Engine Concepts & Flow

### A. Phase Machine (Step enum)
The game flows sequentially through:
Start -> Setup -> NightOne -> Day -> Night -> Day -> Night ...

When moving between steps via State::next_step(), the log advances, and status cleanup is performed:
- **Dusk Cleanup:** Cleans up CleanupPhase::Dusk statuses (e.g., Poisoner's poison, Butler's master choice) when transitioning Day -> Night.
- **Dawn Cleanup:** Cleans up CleanupPhase::Dawn statuses (e.g., Monk's protection) when transitioning Night -> Day.

### B. Game Loop Resolution
When a night/setup step is active, the game steps through characters in a set order:
1. State::get_next_active_player(previous_player) scans all players to find who wakes next according to their role's setup/night order.
2. State::resolve(player_index) is invoked on the acting player.
3. The player's active role ability triggers and returns an optional ChangeRequestBuilder.
4. The engine pushes this request to the change_request_queue of the global state, waiting for Storyteller decision.

---

## 4. State Management & Leptos Sync

- **Global Context Stores:** 
  - Store<State> is the main source of truth for players, active game step, and the storyteller's choice queues.
  - Store<TempState> stores transient user interface signals (e.g., currently selected players/roles in the grimoire, active acting player, and currently displayed ChangeRequest).
- **Storyteller Interaction via ChangeRequest:**
  Since WebAssembly is non-interactive in background tasks, asynchronous storyteller choices (such as waking a Poisoner to select a target) are modeled via a deferred callback queue:
  - ChangeRequest holds:
    - change_type: The layout/interaction expected (ChoosePlayers(usize), ChooseRoles(usize), Display, NoStoryteller, etc.).
    - description: Text displayed to the Storyteller.
    - filter_func: A pointer to a function filtering which players are selectable in the UI.
    - state_change_func: A state transaction callback invoked once the Storyteller confirms their inputs.
  - When the Storyteller clicks "Next":
    1. UI collects selected player indices/roles from TempState.
    2. Runs state_change_func with the inputs, applying the results to Store<State>.
    3. Pops the next request from change_request_queue or advances to the next player/phase.

---

## 5. Extensible Character System

### A. The Role Trait and Enum Dispatch
To avoid massive runtime costs, the game implements static dispatch via enum_dispatch on the Roles enum. All characters must implement the Role trait:

```rust
pub(crate) trait Role: Display + Send + Sync {
    fn get_default_alignment(&self) -> Alignment;
    fn get_true_character_type(&self) -> CharacterType;
    
    // Disguises / Masking
    fn get_alignment(&self) -> Alignment { self.get_default_alignment() }
    fn get_character_type(&self) -> CharacterType { self.get_true_character_type() }
    
    // Win conditions (e.g., Demons)
    fn is_win_condition(&self) -> bool { false }

    fn initialize(&self, player_index: PlayerIndex, state: &mut State) {}
    fn initialization_effect(&self) -> Option<CharacterTypeCounts> { None }

    // Phase Order and Hook Methods
    fn setup_order(&self) -> Option<usize> { None }
    fn setup_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequestBuilder> { None }
    
    fn night_one_order(&self) -> Option<usize> { None }
    fn night_one_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequestBuilder> { None }
    
    fn night_order(&self) -> Option<usize> { None }
    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequestBuilder> { None }
    
    fn has_day_ability(&self) -> bool { false }
    fn day_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequestBuilder> { None }
}
```

### B. Status Effects and "Drunkification"
- **Statuses (StatusType):** Represent applied effects like Poisoned, Drunk, or DemonProtected.
- **Drunkification Wrapper (drunkify):** 
  If a player is poisoned or drunk when resolving their setup/night/day ability, their returned ChangeRequestBuilder is automatically wrapped using the drunkify function in player.rs.
  - It intercepts and voids state_change_func callbacks so they do not write real statuses (or writes dummy data if required).
  - It recursively wraps subsequent change requests pushed to the queue during resolution.
  - It prepends (*Drunk*) or (*Poisoned*) to the request descriptions in the UI, informing the Storyteller that the feedback being displayed represents misinformation. Note: The drunkify implementation is not final and may be revised.

---

## 6. Dynamic Event Listeners

To maintain separation of concerns, passive character abilities (e.g., the Virgin executing a townsfolk nominee, the Mayor bouncing a kill, or the Scarlet Woman taking over when the Demon dies) do not bloat the main loop. They register dynamic callbacks on State:

- **Active Listeners:**
  - nomination_listeners: Intercepts log::Nomination events (e.g., Virgin).
  - attempted_kill_listeners: Intercepts log::AttemptedKill events (e.g., Mayor).
  - death_listeners: Intercepts log::Death events (e.g., Scarlet Woman).
- **Callbacks:** Registered in a role's initialize function. When an event occurs, the engine iterates through valid, non-drunk, non-poisoned listeners and evaluates them sequentially, potentially mutating state or enqueueing custom ChangeRequest operations.
- **Cleanup:** Registered listeners are automatically removed (cleanup_event_listeners) when the host player dies.

---

## 7. Event Logging, Replays & Rollback

The Log system models history chronologically as a vector of DayPhaseLog entries. Each phase holds a chronological sequence of Event objects:
- **Event Types:** Nomination, Voting, Execution, AttemptedKill, Death, StatusApplied, and InfoLearned.
- **Search System:** Provides APIs like search_previous_phase and search_current_phase allowing characters like the Undertaker or Ravenkeeper to inspect history safely.
- **Rollback (Undo) Strategy:** Reversing actions will be handled by replaying events from the initial randomized state or performing reverse transformations of logged events.

---

## 8. Conventions & Implementation Rules

1. **Adding New Roles:**
   - Define role names in RoleNames and register them in the Roles enum.
   - Implement the Role trait on the new character struct inside src/engine/player/roles/.
   - Update RoleNames::convert and Roles::new to link them up.
2. **Never Directly Bypass Leptos Signals:**
   - All mutations during game resolution should flow through try_update blocks or the transaction callbacks inside ChangeRequest.
3. **No Direct State Manipulation for Status-Affecting Actions:**
   - Always verify if a target player has relevant status effects (e.g., DemonProtected) before applying actions like kills.
