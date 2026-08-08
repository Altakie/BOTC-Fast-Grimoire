pub mod utils;
pub mod game;
pub mod game_logic;
pub mod setup;
pub mod components;

#[derive(Clone, Copy)]
pub enum InitializationStage {
    Start,
    InputScript,
    InputPlayers,
    ChooseRoles,
    GameStart,
}
