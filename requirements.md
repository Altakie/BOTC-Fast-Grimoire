
# BOTC Logic Requirements

## Major TODOS

- Add tests for all roles
- Add travellers
- Add Day phase and make it operational
- Finish log
- Players should no longer deliver change requests when dead, fix this to make it more idiomatic
- Bind enter to the next button (currently broken)

## Game Modes

- Mostly automated
  - Logic is mostly handled by the engine
  - Some things will be done automatically
- Manual
  - Game doesn't automatically execute logic
  - Game will still display night order
  - Events will still be logged
- Should be able to switch between modes at will
  - If something goes wrong, want to be able to undo and go back to manual mode
  - Sometimes there are bits of logic that are unable to be captured, and manual mode is good for that

## Undoing Actions

- There should be a way to rollback events using the log
  - Might want to research how this is done
  - Each action should have an opposite action

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
  - Roles that publicly annouce something that may have an effect later
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
