#![allow(dead_code, clippy::needless_return)]
mod day;
mod night;
mod player;
mod role;
mod setup;
mod state;
use state::State;

use crate::Script;
use reactive_stores::Store;

#[derive(Debug, Clone, Copy, Default)]
pub enum Step {
    #[default]
    Setup,
    // Day
    DayDiscussion,
    DayExecution,
    // Night
    Night1,
    Night,
    // Input
    ChoosePlayers,
    ChooseRoles,
    Voting,
    // Display
    DisplayRoles,
    DisplayPlayers,
}

#[derive(Store, Clone)]
struct Engine {
    state: State,
    step: Step,
}

impl Engine {
    fn new(roles: Vec<Role>, player_names: Vec<String>, script: Script) -> Self {
        let state = State::new(roles, player_names, script);
        Self {
            state,
            step: Step::default(),
        }
    }
}
