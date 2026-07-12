# Roles

This file lists the roles in "Trouble Brewing" and their implementation status in the current project.

## Townsfolk

### Washerwoman
- **Effect:** You start knowing that one of two players is a certain Townsfolk character.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `setup_ability` to allow the storyteller to choose a Townsfolk and then in `washerwoman_librarian_investigator_wrong` handles selecting a different player for the "wrong" information.

### Librarian
- **Effect:** You start knowing that one of two players is a certain Outsider character. (If there are no Outsiders in play, you get no information).
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `setup_ability` to count Outsiders and prompt the storyteller for the correct information if any exist.

### Investigator
- **Effect:** You start knowing that one of two players is a certain Minion character.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `setup_ability` to prompt the storyteller for a Minion and a different player for false info.

### Chef
- **Effect:** You start knowing how many pairs of evil players there are.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `night_one_ability` to calculate evil pairs in the current game state and display the count.

### Empath
- **Effect:** Each night, you learn how many of your two living neighbors are evil.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `night_ability` (and `night_one_ability`) to check the alignment of immediate neighbors and display the count.

### Fortuneteller
- **Effect:** Each night, you learn if one of two players is a Demon. (There is a "red herring" player who will always register as a Demon).
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `setup_ability` to set a red herring, and `night_ability` to check for Demons or the red herring.

### Undertaker
- **Effect:** Each night, you learn which character was executed that day.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `night_ability` to search the log for `Event::Execution` and return the role of the executed player.

### Monk
- **Effect:** Each night, choose a player (not yourself): they are protected from the Demon's attack tonight.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `night_ability` to apply the `StatusType::DemonProtected` effect to a chosen player.

### Ravenkeeper
- **Effect:** If the Demon kills you at night, you learn which player is the Demon.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `night_ability` to check for death events and if triggered, allows selecting a player to learn their role (note: this logic seems slightly different from the standard Ravenkeeper effect, but is what is currently implemented).

### Virgin
- **Effect:** The first time you are nominated, if the nominator is a Townsfolk, they are immediately executed.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `initialize` to register a `nomination_listener` that executes the nominator if they are a Townsfolk.

### Slayer
- **Effect:** Once per game, during the day, choose a player: if they are the Demon, they die.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `day_ability` with an `ability_used` flag to kill the target if they are a Demon.

### Soldier
- **Effect:** You are safe from the Demon's attack.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `initialize` to add a permanent `StatusType::DemonProtected` effect to the player.

### Mayor
- **Effect:** If no-one is executed during the day, you might die at night instead of the Demon's kill. If 3 or more players are alive, you win.
- **Implementation Status:** Implemented (Partial)
- **File Location:** `src/engine/player/roles/townsfolk.rs`
- **Implementation Details:** Implements `initialize` with an `attempted_kill_listener` that handles the "bouncing" of a kill.

## Outsiders

### Butler
- **Effect:** Each night, choose a player: you must vote if and only if they vote.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/outsiders.rs`
- **Implementation Details:** Implements `night_ability` to apply `StatusType::ButlerMaster` to a chosen player.

### Drunk
- **Effect:** You do not know you are the Drunk. You believe you are a Townsfolk character, but you are not. (Your ability does not work).
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/outsiders.rs`
- **Implementation Details:** Implements `setup_ability` to assign a role to the drunk, and wraps ability results in a "drunkified" version.

### Recluse
- **Effect:** You might register as evil or as a Minion or Demon, even if you are good.
- **Implementation Status:** Implemented (Minimal)
- **File Location:** `src/engine/player/roles/outsiders.rs`
- **Implementation Details:** Currently only overrides `get_alignment` to return `Alignment::Any`.

### Saint
- **Effect:** If you are executed, your team loses.
- **Implementation Status:** Todo
- **File Location:** `src/engine/player/roles/outsiders.rs`
- **Implementation Details:** Struct exists but functionality for the win condition is not implemented.

## Minions

### Poisoner
- **Effect:** Each night, choose a player: they are poisoned.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/minions.rs`
- **Implementation Details:** Implements `night_ability` to apply `StatusType::Poisoned` to a chosen player.

### Spy
- **Effect:** Each night, you see the Grimoire.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/minions.rs`
- **Implementation Details:** Implements `night_ability` to trigger a display request for the grimoire.

### Scarlet Woman
- **Effect:** If the Demon dies, you become the Demon.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/minions.rs`
- **Implementation Details:** Implements `initialize` to register a `death_listener` that swaps the role to the Demon if the Demon dies.

### Baron
- **Effect:** There are 2 extra Outsiders in play.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/minions.rs`
- **Implementation Details:** Implements `initialization_effect` to adjust character type counts.

## Demons

### Imp
- **Effect:** Each night, choose a player: they die. If you kill yourself, a Minion becomes the Imp.
- **Implementation Status:** Implemented
- **File Location:** `src/engine/player/roles/demons.rs`
- **Implementation Details:** Implements `night_ability` to kill a target and, if the target is self, triggers the logic for a Minion to become the new Imp.
