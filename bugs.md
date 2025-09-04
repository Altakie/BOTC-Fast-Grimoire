# List of Bugs

## Gameplay Bugs

### Major Bugs

- Not all players have their abilities disabled when they die
  - Have the order func take in player data or a function pointer that yields player data, so that it can clear if the ability should have an order during that night
- Scarletwoman, mayor are not implemented
- Drunk and poison status effects do not work properly
  - They should not really mutate state but cause one time abilities to be used up
  - This is hard to do because change effects are lazily evaluated,
  so thus would need to create the equivalent of an iter that you attach to the change request to make it so they don't mutate state in a meaningful way
  - Could add a separate method, but that seems like extra complexity
- Baron Does not subtract from townsfolk
- Drunk does not show drunk for info roles
- Drunkness isn't shown night 1
- Sometimes button has to be pressed twice to skip through setup
- Ravenkeeper still triggers if they die during the day

### Minor Bugs

## UI Bugs

### Major Bugs

### Minor Bugs
