# List of Bugs

## Gameplay Bugs

### Major Bugs

- Not all players have their abilities disabled when they die
  - Have the order func take in player data or a function pointer that yields player data, so that it can clear if the ability should have an order during that night
- Scarletwoman, mayor, undertaker, and slayer are not implemented
- Imp will trigger itself again when it switches to another minion
  - Imp can store an ability used bool that resets every night?. Or can use log
- Drunk does not actually disable the ability of the role it imitates
- Drunk and poison status effects do not work properly
  - They should not really mutate state but cause one time abilities to be used up
  - This is hard to do because change effects are lazily evaluated, so thus would need to create the equivalent of an iter that you attach to the change request to make it so they don't mutate state in a meaningful way
  - Could add a separate method, but that seems like extra complexity
- Status effects are not cleaned up on kill
  - Add clean up to kill method of player

### Minor Bugs

## UI Bugs

### Major Bugs

- Day count never changes

### Minor Bugs
