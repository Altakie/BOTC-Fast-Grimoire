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
- Test suite
  - Need to figure out a better way to test roles
  - Write detailed test cases for each role
  - Test cases should also be partially derived from the role descriptions on the official dominion wiki
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

## Minor Features

- Display errors as a tooltip
