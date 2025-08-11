#![allow(dead_code, clippy::needless_return)]
pub(crate) mod log;
use log::{DayPhase, Log};
pub(crate) mod status_effects;
use status_effects::StatusEffect;

use rand::{self, seq::SliceRandom};
use reactive_stores::*;
use std::collections::HashMap;

use crate::{
    engine::{
        change_request::ChangeRequest,
        player::{Player, roles::Roles},
        state::status_effects::CleanupPhase,
    },
    initialization::Script,
};

pub(crate) type PlayerIndex = usize;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Step {
    #[default]
    Start,
    Setup,
    // Day
    DayDiscussion,
    DayExecution,
    // Night
    NightOne,
    Night,
    // Input
    // ChoosePlayers,
    // ChooseRoles,
    // Voting,
    // Display
    // DisplayRoles,
    // DisplayPlayers,
}

#[derive(Clone, Store)]
pub(crate) struct State {
    players: Vec<Player>,
    win_cond_i: Option<PlayerIndex>,
    status_effects: Vec<StatusEffect>,
    day_phase: DayPhase,
    pub(crate) day_num: usize,
    pub(crate) log: Log,
    script: Script,
    pub(crate) step: Step,
}

impl State {
    pub(crate) fn new(
        mut roles: Vec<Roles>,
        player_names: Vec<String>,
        script: Script,
    ) -> Result<Self, ()> {
        let mut players: Vec<Player> = vec![];

        let mut rng = rand::rng();
        roles.shuffle(&mut rng);

        if roles.len() != player_names.len() {
            eprintln!("Number of players does not match number of roles");
            // TODO: Figure out to do errors here
            return Err(());
        }

        // TODO: Figure out how to store roles properly so we can create a game. Do we even need
        // the roles enum?
        // Idea: For now, just have a method to translate a member of the enum to a role trait
        for i in 0..roles.len() {
            let player = Player::new(player_names[i].clone(), roles[i].convert());
            players.push(player);
        }

        assert!(
            players.iter().filter(|p| p.role.is_win_condition()).count() <= 1,
            "Shouldn't have more than one win condition when game starts"
        );

        let win_cond_i = players.iter().position(|p| p.role.is_win_condition());
        let status_effects: Vec<StatusEffect> = vec![];

        let log = Log::new();

        return Ok(Self {
            players,
            win_cond_i,
            status_effects, // active_roles,
            day_phase: DayPhase::Night,
            day_num: 1,
            log,
            script,
            step: Step::default(),
        });
    }

    pub(crate) fn get_player_index(&self, player: &Player) -> PlayerIndex {
        self.players
            .iter()
            .position(|p| p == player)
            .expect("Player should be in player array")
    }

    pub(crate) fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub(crate) fn get_player(&self, player_index: PlayerIndex) -> &Player {
        &self.players[player_index]
    }

    pub(crate) fn get_player_mut(&mut self, player_index: PlayerIndex) -> &mut Player {
        &mut self.players[player_index]
    }

    pub(crate) fn living_player_count(&self) -> usize {
        self.players.iter().filter(|s| !s.dead).count()
    }

    pub(crate) fn left_player(&self, player_index: PlayerIndex) -> PlayerIndex {
        let mut index: PlayerIndex = (player_index + self.players.len() - 1) % self.players.len();
        // eprintln!("{}", index);
        while self.players[index].dead {
            // eprintln!("{}", index);
            index = (index + self.players.len() - 1) % self.players.len();
        }

        return index;
    }
    pub(crate) fn right_player(&self, player_index: PlayerIndex) -> PlayerIndex {
        let mut index: PlayerIndex = (player_index + self.players.len() + 1) % self.players.len();
        while self.players[index].dead {
            index = (index + self.players.len() + 1) % self.players.len();
        }

        return index;
    }

    pub(crate) fn set_win_condition(&mut self, player: &Player) {
        self.win_cond_i = Some(self.get_player_index(player));
    }

    pub(crate) fn game_over(&self) -> bool {
        let index = match self.win_cond_i {
            Some(i) => i,
            None =>
            // TODO: Need to implement this for athiest games, but this should be manually
            // resolved by story teller most likely
            {
                todo!()
            }
        };
        // Game ends if win condition player is dead
        self.players[index].dead
    }

    pub(crate) fn next_step(&mut self) {
        let next_step = match self.step {
            Step::Start => Step::Setup,
            Step::Setup => Step::NightOne,
            Step::DayDiscussion => Step::DayExecution,
            Step::DayExecution => {
                self.cleanup_statuses(CleanupPhase::Dusk);
                self.day_num += 1;
                Step::Night
            }
            Step::NightOne | Step::Night => {
                self.cleanup_statuses(CleanupPhase::Dawn);
                Step::DayDiscussion
            }
        };

        self.step = next_step;
        // TODO: Log step change
    }

    pub(crate) fn get_next_active_player(
        &self,
        previous_player: Option<PlayerIndex>,
    ) -> Option<PlayerIndex> {
        match self.step {
            Step::Start => None,
            Step::Setup => self.get_next_active_setup(previous_player),
            Step::NightOne => self.get_next_active_night_one(previous_player),
            Step::Night => self.get_next_active_night(previous_player),
            _ => None,
        }
    }

    /// Function to resolve a player's effect on the state
    ///
    /// # Args
    ///
    /// * player_index : Index of player to resolve for
    ///
    /// # Returns
    ///
    /// * Option<ChangeRequest> : A change request if the role does something, or none if it
    ///   doesn't
    pub(crate) fn resolve(&self, player_index: PlayerIndex) -> Option<ChangeRequest> {
        let player = self.get_player(player_index);

        let res = match self.step {
            Step::Setup => player.setup_ability(player_index, self),
            Step::NightOne => player.night_one_ability(player_index, self),
            Step::Night => player.night_ability(player_index, self),
            _ => None,
        };

        return res;
        // TODO: Log events that happen in the setup
    }

    pub(crate) fn kill(
        &mut self,
        attacking_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
    ) {
        let state_snapshot = self.clone();
        self.get_player_mut(target_player_index)
            .kill(attacking_player_index, &state_snapshot);
        let dead = self.get_player(target_player_index).dead;
        if dead {
            self.cleanup_player_statuses(target_player_index);
        }
    }
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//
//     // NOTE: Testing Utils
//
//     pub(crate) fn setup_test_game() -> (State, Vec<Roles>) {
//         let roles = vec![
//             Roles::Investigator,
//             Roles::Innkeeper,
//             Roles::Imp,
//             Roles::Chef,
//             Roles::Poisoner,
//         ];
//         let player_names = vec![
//             String::from("P1"),
//             String::from("P2"),
//             String::from("P3"),
//             String::from("P4"),
//             String::from("P5"),
//         ];
//
//         return (
//             State::new(roles.clone(), player_names, EMPTY_SCRIPT).unwrap(),
//             roles,
//         );
//     }
//     pub(crate) const EMPTY_SCRIPT: Script = Script { roles: vec![] };
//     //
//     // // NOTE: Tests
//     // #[test]
//     // fn test_player_constructor() {
//     //     let good_player_name = String::from("Good");
//     //     // Add in all good players here
//     //     let good_player_roles = vec![
//     //         Role::Investigator,
//     //         Role::Empath,
//     //         Role::Gossip,
//     //         Role::Innkeeper,
//     //     ];
//     //
//     //     for role in good_player_roles {
//     //         // Create a new player
//     //         let player = Player::new(good_player_name.clone(), role);
//     //         // Test that the player is alive, has a ghost vote, has the proper name, has no status
//     //         // effects on them, has the right role, and is good
//     //         assert_eq!(player.name, String::from("Good"));
//     //         assert_eq!(player.role, role);
//     //         assert!(!player.dead);
//     //         assert!(player.ghost_vote);
//     //         assert_eq!(player.alignment, Alignment::Good);
//     //     }
//     //
//     //     let evil_player_name = String::from("Evil");
//     //     let evil_player_roles = vec![Role::Imp];
//     //
//     //     for role in evil_player_roles {
//     //         // Create a new player
//     //         let player = Player::new(evil_player_name.clone(), role);
//     //         // Test that the player is alive, has a ghost vote, has the proper name, has no status
//     //         // effects on them, has the right role, and is good
//     //         assert_eq!(player.name, String::from("Evil"));
//     //         assert_eq!(player.role, role);
//     //         assert!(!player.dead);
//     //         assert!(player.ghost_vote);
//     //         assert_eq!(player.alignment, Alignment::Evil);
//     //     }
//     // }
//     //
//     #[test]
//     fn test_new_game() {
//         let (game, roles) = setup_test_game();
//
//         assert_eq!(game.players.len(), 5);
//         assert_eq!(game.players[0].name, "P1");
//         assert_eq!(game.players[1].name, "P2");
//         assert_eq!(game.players[2].name, "P3");
//         assert_eq!(game.players[3].name, "P4");
//         assert_eq!(game.players[4].name, "P5");
//
//         assert_eq!(game.status_effects.len(), 0);
//
//         {
//             let mut roles = roles.clone();
//             for player in game.players {
//                 let role_i = match roles.iter().position(|&r| r == player.role) {
//                     Some(x) => x,
//                     None => {
//                         eprintln!("Role not assigned to player");
//                         panic!();
//                     }
//                 };
//
//                 roles.remove(role_i);
//             }
//
//             assert_eq!(roles.len(), 0);
//         }
//
//         // TODO: Maybe add a check here that all the assigment events were logged
//     }
//
//     // #[test]
//     // fn game_setup() {
//     //     // TODO: Do this after implementing setup method
//     //     // Only way to really test this right now is through baron and drunk
//     //     todo!()
//     // }
//     //
//     #[test]
//     fn kill_player() {
//         let mut game = setup_test_game().0;
//
//         game.kill_player(0, 0);
//         assert!(game.players[0].dead);
//         game.kill_player(1, 1);
//         assert!(game.players[1].dead);
//         game.kill_player(2, 2);
//         assert!(game.players[2].dead);
//     }
//
//     #[test]
//     fn kill_death_protected_player() {
//         let mut game = setup_test_game().0;
//
//         game.add_status(StatusType::DeathProtected, 1, 1);
//
//         game.kill_player(0, 0);
//         assert!(game.players[0].dead);
//         game.kill_player(1, 1);
//         assert!(!game.players[1].dead);
//         game.kill_player(2, 2);
//         assert!(game.players[2].dead);
//
//         game.remove_status(StatusType::DeathProtected, 1, 1);
//         game.kill_player(1, 1);
//         assert!(game.players[1].dead);
//     }
//
//     #[test]
//     fn kill_night_protected_player() {
//         let mut game = setup_test_game().0;
//
//         game.day_phase = DayPhase::Night;
//         game.add_status(StatusType::NightProtected, 1, 1);
//
//         game.kill_player(0, 0);
//         assert!(game.players[0].dead);
//         game.kill_player(1, 1);
//         assert!(!game.players[1].dead);
//         game.kill_player(2, 2);
//         assert!(game.players[2].dead);
//
//         game.day_phase = DayPhase::DayDiscussion;
//         game.kill_player(1, 1);
//         assert!(game.players[1].dead);
//     }
//
//     #[test]
//     fn kill_demon_protected_player() {
//         let mut game = setup_test_game().0;
//
//         game.add_status(StatusType::DemonProtected, 1, 1);
//
//         let demon_index = game.win_cond_i.unwrap();
//
//         game.kill_player(demon_index, 0);
//         assert!(game.players[0].dead);
//         game.kill_player(demon_index, 1);
//         assert!(!game.players[1].dead);
//         game.kill_player(demon_index, 2);
//         assert!(game.players[2].dead);
//
//         game.kill_player(demon_index, 1);
//         assert!(!game.players[1].dead);
//
//         game.remove_status(StatusType::DemonProtected, 1, 1);
//         game.kill_player(demon_index, 1);
//         assert!(game.players[1].dead);
//     }
//     //
//     // #[test]
//     // fn test_left() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     assert_eq!(game.players[game.left_player(1)], game.players[0]);
//     //
//     //     // Kill set the left player to dead and see that the left player is updated accordingly
//     //     game.kill_player(0, 0);
//     //     assert_eq!(game.players[game.left_player(1)], game.players[2]);
//     // }
//     //
//     // #[test]
//     // fn test_right() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     assert_eq!(game.players[game.right_player(1)], game.players[2]);
//     //
//     //     // Kill the right player and make sure the right player is updated accordingly
//     //     game.kill_player(0, 2);
//     //     assert_eq!(game.players[game.right_player(1)], game.players[0]);
//     // }
//     //
//     // #[test]
//     // fn test_game_over() {
//     //     todo!();
//     // }
//     //
//     // #[test]
//     // fn test_get_night_1_order() {
//     //     let game = setup_test_game().0;
//     //
//     //     let player_indices = vec![0, 1, 2, 3, 4];
//     //     let order = game.get_night_1_order(player_indices);
//     //     assert_eq!(game.players[order[0]].role, Role::Poisoner);
//     //     assert_eq!(game.players[order[1]].role, Role::Investigator);
//     //     assert_eq!(game.players[order[2]].role, Role::Chef);
//     //     assert_eq!(order.len(), 3);
//     // }
//     //
//     // fn test_resolve_night_1() {
//     //     todo!();
//     // }
//     //
//     // // TODO: Test that all night one abilities work as expected
//     //
//     // fn test_night_order() {
//     //     let game = setup_test_game().0;
//     //
//     //     let player_indices = vec![0, 1, 2, 3, 4];
//     //     let order = game.get_night_order(player_indices);
//     //     assert_eq!(game.players[order[0]].role, Role::Poisoner);
//     //     assert_eq!(game.players[order[1]].role, Role::Innkeeper);
//     //     assert_eq!(order.len(), 2);
//     // }
//     //
//     // // TODO: Test that all night abilities work as expected
//     // fn tsest_resolve_night() {
//     //     todo!();
//     // }
//     //
//     // #[test]
//     // fn add_status_effect() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     game.add_status(StatusEffects::Poisoned, 2, 0);
//     //
//     //     assert_eq!(game.status_effects[0].status_type, StatusEffects::Poisoned);
//     //     assert_eq!(game.status_effects[0].source_player_index, 2);
//     //     assert_eq!(game.status_effects[0].affected_player_index, 0);
//     // }
//     //
//     // #[test]
//     // fn add_multiple_status_effects() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     game.add_status(StatusEffects::Poisoned, 2, 0);
//     //     game.add_status(StatusEffects::MayorBounceKill, 1, 3);
//     //     game.add_status(StatusEffects::Drunk, 4, 2);
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Poisoned
//     //                     && s.source_player_index == 2
//     //                     && s.source_role == game.players[2].role
//     //                     && s.affected_player_index == 0
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::MayorBounceKill
//     //                     && s.source_player_index == 1
//     //                     && s.source_role == game.players[1].role
//     //                     && s.affected_player_index == 3
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Drunk
//     //                     && s.source_player_index == 4
//     //                     && s.source_role == game.players[4].role
//     //                     && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     // Checks that same player can have multiple status effects applied to them
//     //     // Checks that the same player can have multiple of the same status effect from differnet
//     //     // sources applied to them
//     //     //
//     //     game.add_status(StatusEffects::Drunk, 3, 2);
//     //     game.add_status(StatusEffects::Drunk, 1, 2);
//     //     game.add_status(StatusEffects::Poisoned, 4, 2);
//     //     game.add_status(StatusEffects::Drunk, 1, 0);
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| { s.status_type == StatusEffects::Drunk })
//     //             .count(),
//     //         4
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Drunk && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         3
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Drunk
//     //                     && s.source_player_index == 4
//     //                     && s.source_role == game.players[4].role
//     //                     && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Poisoned && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.source_player_index == 4
//     //                     && s.source_role == game.players[4].role
//     //                     && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         2
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| { s.affected_player_index == 2 })
//     //             .count(),
//     //         4
//     //     );
//     // }
//     //
//     // #[test]
//     // fn find_status_effects_inflicted_by_player() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     game.add_status(StatusEffects::Poisoned, 2, 0);
//     //     game.add_status(StatusEffects::MayorBounceKill, 1, 3);
//     //     game.add_status(StatusEffects::Drunk, 4, 2);
//     //
//     //     game.add_status(StatusEffects::Drunk, 2, 2);
//     //     game.add_status(StatusEffects::Drunk, 2, 1);
//     //     game.add_status(StatusEffects::Drunk, 2, 0);
//     //
//     //     let statuses = game.get_inflicted_statuses(2);
//     //     assert_eq!(statuses.len(), 4);
//     //     assert_eq!(
//     //         statuses
//     //             .iter()
//     //             .filter(|s| s.status_type == StatusEffects::Drunk)
//     //             .count(),
//     //         3
//     //     );
//     //     assert_eq!(
//     //         statuses
//     //             .iter()
//     //             .filter(|s| s.status_type == StatusEffects::Poisoned)
//     //             .count(),
//     //         1
//     //     );
//     //     assert!(statuses.iter().all(|s| s.source_player_index == 2));
//     //     assert!(
//     //         statuses
//     //             .iter()
//     //             .all(|s| s.source_role == game.players[2].role)
//     //     );
//     //
//     //     let no_statuses = game.get_inflicted_statuses(0);
//     //     assert_eq!(no_statuses.len(), 0);
//     // }
//     //
//     // #[test]
//     // fn find_status_effects_inlicted_by_player() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     game.add_status(StatusEffects::Poisoned, 2, 0);
//     //     game.add_status(StatusEffects::MayorBounceKill, 1, 3);
//     //     game.add_status(StatusEffects::Poisoned, 4, 2);
//     //
//     //     game.add_status(StatusEffects::Drunk, 3, 2);
//     //     game.add_status(StatusEffects::Drunk, 1, 2);
//     //     game.add_status(StatusEffects::Drunk, 0, 2);
//     //
//     //     let statuses = game.get_afflicted_statuses(2);
//     //     assert_eq!(statuses.len(), 4);
//     //     assert_eq!(
//     //         statuses
//     //             .iter()
//     //             .filter(|s| s.status_type == StatusEffects::Drunk)
//     //             .count(),
//     //         3
//     //     );
//     //     assert_eq!(
//     //         statuses
//     //             .iter()
//     //             .filter(|s| s.status_type == StatusEffects::Poisoned)
//     //             .count(),
//     //         1
//     //     );
//     //     assert!(statuses.iter().all(|s| s.affected_player_index == 2));
//     //
//     //     let no_statuses = game.get_afflicted_statuses(4);
//     //     assert_eq!(no_statuses.len(), 0);
//     // }
//     //
//     // #[test]
//     // fn remove_status_effect() {
//     //     todo!();
//     // }
//     //
//     // #[test]
//     // fn remove_multiple_status_effects() {
//     //     todo!();
//     // }
// }
