# List of Bugs

## Gameplay Bugs

### Major Bugs

- Scarletwoman is not implemented
- Mayor
  - If bounces a kill onto a protected player, it will kill them (not intentional), attempt to bounce on dead player if possible
- Scarletwoman
  - Add event listeners?
  - Store functions that change the state in reaction to a certain type of event (in the log?, maybe the state)
  - Run these functions when a specific event is logged (have a log event function that is overwritten)
- Drunk and poison status effects do not work properly
  - They should not really mutate state but cause one time abilities to be used up
  - This is hard to do because change effects are lazily evaluated,
  so thus would need to create the equivalent of an iter that you attach to the change request to make it so they don't mutate state in a meaningful way
  - Could add a separate method, but that seems like extra complexity
- Virgin and slayer will keep their abilities if they are poisoned when they use them
- Sometimes button has to be pressed twice to skip through setup
- Ravenkeeper still triggers if they die during the day

### Minor Bugs

## UI Bugs

### Major Bugs

### Minor Bugs
