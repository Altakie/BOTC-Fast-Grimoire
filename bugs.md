# List of Bugs

## Gameplay Bugs

### Major Bugs

- Not all players have their abilities disabled when they die
  - Have the order func take in player data or a function pointer that yields player data, so that it can clear if the ability should have an order during that night
- Scarletwoman, mayor, undertaker, and slayer are not implemented
- Drunk and poison status effects do not work properly
  - They should not really mutate state but cause one time abilities to be used up
  - This is hard to do because change effects are lazily evaluated,
  so thus would need to create the equivalent of an iter that you attach to the change request to make it so they don't mutate state in a meaningful way
  - Could add a separate method, but that seems like extra complexity

### Minor Bugs

## UI Bugs

### Major Bugs

### Minor Bugs
