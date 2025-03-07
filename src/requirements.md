# BOTC Logic Requirements

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
