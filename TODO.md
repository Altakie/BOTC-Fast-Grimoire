# Todos

## Major Features

- Manual mode
  - User should be able to access every property of the game state and change them
    - Print out the state in a pretty way
      - Structs should also be collapsed by default
        - For players it should show their name
        - For roles, it should show the text representation of the role
      - Vectors should be collapsed by default
      - For booleans, there should be a button to toggle them
      - Values requiring an input field
        - For strings, there should be an input field to change them
        - For numbers, there should also be an input field that only accepts the right numbers
          - Like you can't type in negative numbers if its unsigned, and cant type in any character that isn't alphanumeric
          - If somehow the user is able to an illegal character into the text field,
            it should display an error as a dropdown and not actually propagate their change into the state
      - For enum members, there should be a selection dropdown to choose a different enum member from
        - Selection should be color coded by alignment
          - Blue for good
          - Red for evil
- Better styling
  - I want the log to look better
- Undo functionality
  - Store a snapshot of the state beforehand
    - Requires saving closures as well
    - Does it make sense to replay the log from scratch and add the functionality to do that?
      - Yes
- Saving the game state
  - Alternative
    - Save all the actions that happened in the game in the log as well as their parameters
      - Save any inputs that the user was prompted for
      - Feed these inputs into the engine when it asks for user input instead of halting and asking the user
    - Add functionality to the engine to replay this log
  - Before the user is prompted for input and the engine halts, save the state to a file (probably a json file for now)
  - If the game is started up again and this file exists, prompt the user if they want to continue the game instead of the normal starting screen
    - Two choices
      - Resume game
        - Parse the state from the existing json file and reprompt the user with the last input
        - Make sure to save the existing change_request queue as well as event listeners so that they are callable functions
          - Need a way to save or reconstruct closures
      - Start new game
        - Should do the same as the start game button on the original screen
- More scripts (as in role scripts)
  - Should support all 3 base scripts by default
- Implementation of all roles
- Game overview after
  - Should be able to replay the log
- Real vote tallying during nominations
  - `ChangeType::Voting` and `Event::Voting` already exist in the engine (`change_request.rs`, `log.rs`) but are never constructed anywhere — nominations are logged, but there's no mechanism to record how many players voted or to require a majority before a player can be executed
  - Currently the storyteller can execute any player directly at will (via the UI's execute confirm gate), regardless of nomination/vote outcome

## Minor Features

- Display errors as a tooltip
- Explain setup-time role/bag conflicts (e.g. dealing the Baron when there aren't enough Outsiders left to satisfy the +2 Outsider/-2 Townsfolk swap) instead of silently blocking the role selection
- Implement Partial borrows
  - Much of the state is non-overlapping, but a single mutable borrow of the state will invalidate accesses to the rest of the state
  - This leads to a lot of behavior that is prohibited by the borrow checked but is completely valid
  - Instead of passing around a state object, perhaps pass around a state reference distributer?
    - This distributer will handle giving out references, and will literally return pointers to the state as references
