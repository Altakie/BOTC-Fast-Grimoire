
# BOTC Logic Requirements

## Major TODOS

- Event listeners
  - Want an idiomatic way to add and remove event listeners
- Move day state to player type (day 1, setup)
- Add in ways to add demon bluffs and notify the demon and minions to get to know each other
- Also remind storyteller to give out roles
- How to trigger scarletwoman effect?
  - Event listeners (demon death)
  - End of game functions
- Maybe have code cleanup?? Or at least code review where you check if stuff makes still makes sense/ can be better?
  - Do this after scarletwoman and mayor are implemented
  - Maybe I am being too strict in how I allow roles to modify the game
  - Maybe all interactions with a role (besides getters) should return some kind of change request that bubbles up and is resolved?
  - IE: Executions, Nominations, kills abilities
  - Maybe i should be passing along state a lot more than I am
  - Roles obviously should be able to be swapped out, so they must be stored within an arc, but I also want them to have interior mutability, because recreating them each time is kind of annoying (or maybe this is not an issue).
  - Rethink change requests slightly (not completely) to see if they should work a little differently. See if there are any ways to make writing roles less grueling and less boilerplate.
  - Rework status effects slightly. Maybe the categories of what they should affect should be different
- End of game
  - Gather win conditions at the start of the game, and also when they should be checked. Respond to log events that correspond to the win condition's trigger?
  - Win condition should have a trigger, like a death, or a day switch event?
- Display for the log
- Event logging for all events (and in roles)
  - Most likely just add loggers to the main event methods (kill, execute, nominate, switch day) etc..
- Players should no longer deliver change requests when dead, fix this to make it more idiomatic
- Get a way to replace sticky notes
  - You are blah blah blah
- Add travellers
- Replaying the log
- Add tests for all roles (and transfer old ones)
  - Perhaps do some refactoring to do dependency injection?
  - Make state
- Disable players/options you can't pick in the ui
  - Partially implemented, add filters for each role
- PlayerIndex type?
- Enum dispatch (crate)

## Game Modes

- Mostly automated
  - Logic is mostly handled by the engine
  - Some things will be done automatically
- Manual
  - Game doesn't automatically execute logic
  - Game will still display night order
  - Events will still be logged
  - Add associated effects to players and the ability to add them?
- Should be able to switch between modes at will
  - If something goes wrong, want to be able to undo and go back to manual mode
  - Sometimes there are bits of logic that are unable to be captured, and manual mode is good for that

## Undoing Actions

- There should be a way to rollback events using the log
  - Might want to research how this is done
  - Each action should have an opposite action
  - Could also store state at each step (but this is not memory efficient)
  - Also could store series of transformations to restore state from initial state

# Old Requirements

## General

- Day / Night Phases
- Day / Night Counter
- Log of events that happen during the game
- GUI
  - A visual way to see all the information that is stored by the game engine
- Misc story teller notes

## Setup

- Character counts
  - Engine should automatically see which characters are in play and update default character counts accordingly
- Role selector
  - Ability to select random roles
- Seat distributor
  - A way to randomly distribute the roles in a random order
  - These should also be visually distributed in a circular order
  - Should be able to group these visual tokes as well (to represent couches and the like)

## Roles

- Roles should have the following information associated with them
  - Player name
  - Order (who is sitting to the left and to the right)
  - Ability
    - Ability trigger (public or private, acts during day or night)
    - Ability effects (ex: poisoning, protection)
  - Statuses inflicted on the player
    - Drunk
    - Poisoned
    - Mad
    - Protected
  - Whether the player is dead or not
  - Whether the player has used their ghost vote or not

## Day

- All info is public
- There are roles that act only during the day
  - Roles that ask the story teller something privately
  - Roles that publicly announce something that may have an effect later
- Storyteller should manually:
  - Mark that someone claimed a role
  - I think this would be more role specific

## Night

- Roles need to be woken up in a set order
  - When night phase is switched on, the program should automatically run through this order
  - Each role should be treated as a phase, where its effects are resolved
    - Roles can pick people and apply effects
    - Roles can get information
    - Roles can have a one time use ability that they can use
    - Needs to have manual storyteller intervention to:
      - Mark who is affected by the ability
      - Mark whether the player wants to use their ability
      - Mark when the phase has ended
    - Game should automatically:
      - Resolve the effects of a role after its phase is ended
      - Apply those effects to the game state
      - Log that an event happened and what happened in this event

## Events

### Types of Events
