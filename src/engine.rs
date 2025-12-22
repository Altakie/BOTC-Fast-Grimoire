#![allow(dead_code, clippy::needless_return)]
pub(crate) mod change_request;
pub(crate) mod day;
pub(crate) mod night;
pub(crate) mod player;
pub(crate) mod setup;
pub(crate) mod state;

// use leptos::prelude::RwSignal;
// use state::State;
//
// use crate::Script;
// use crate::engine::{player::Role, state::PlayerIndex};
// use reactive_stores::Store;

// #[derive(Store, Clone)]
// pub(crate) struct Engine {
//     state: State,
//     step: Step,
// }
//
// impl Engine {
//     pub(crate) fn new(roles: Vec<Role>, player_names: Vec<String>, script: Script) -> Self {
//         let state = State::new(roles, player_names, script).unwrap();
//         Self {
//             state,
//             step: Step::default(),
//         }
//     }
// }
