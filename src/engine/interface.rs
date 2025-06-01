use super::{Game, PlayerIndex, Role};

#[derive(Clone, Copy)]
pub(crate) enum RoleSelectionType {
    InPlay,
    NotInPlay,
    Script,
}

#[derive(Debug, Clone, Copy)]
pub enum Step {
    Setup,
    // Day
    DayDiscussion,
    DayExecution,
    // Night
    Night1,
    Night,
    // Input
    ChoosePlayers(usize),
    ChooseRoles(usize, Vec<Role>),
    Voting,
    // Display
    DisplayRoles,
    DisplayPlayers,
}
// Yes we want states
// Yes we want to transition between states
// Okay so what do we want actually
// Basically, setup state -> Night1 state, very easy, can be done through button
// Night1 state needs Input
// Can do this certain way
// CONTINUATION
// EACH NIGHT HAS A CONTINUATION
//

impl Game {
    pub(crate) fn choose_players(&mut self, num: usize) -> Vec<usize> {}

    pub(crate) fn choose_roles(&mut self, num: usize, selector: RoleSelectionType) -> Vec<Role> {}
}

#[cfg(test)]
mod tests {
    // use super::super::tests::setup_test_game;
    // use super::*;
    // use crate::game::GameStoreFields;
    // use leptos::prelude::*;
    // use reactive_stores::Store;
    // use std::{
    //     thread::{self, sleep},
    //     time::Duration,
    // };
}
