use crate::engine::{
    change_request::{ChangeArgs, ChangeRequest, ChangeType},
    player::{CharacterType, Role},
    state::{PlayerIndex, State},
};
use crate::new_change_request;
use crate::unwrap_args_err;
use crate::unwrap_args_panic;

// use leptos::prelude::*;
// use reactive_stores::Store;

// TODO: Iterate through all players until none left
// Iteration done manually using button
// Each iter step:
// Check how current player's role affects setup - trigger appropriate components with proper
// parameters, if doesn't affect, skip
// Parameters to components -> trigger specic parameters, and a function that can be applied to
// the game state with the information gotten from the user
// Function passed should apply changes to game state, and then return to setup
// Don't need different components based on game state, just need to pass different functions

// TODO: What to do what to do
// Easy -> Want generic components for different types of inputs
// Those components should take in a signal, and also act kind of like a form
// When they return, they should what component it wants rendered and a function to be applied to
// the game state
impl State {
    // pub(crate) fn get_active_players(&self) -> Vec<PlayerIndex> {
    //     let mut res: Vec<PlayerIndex> = vec![];
    //     for (i, player) in self.players.iter().enumerate() {
    //         match player.role {
    //             Role::Washerwoman
    //             | Role::Librarian
    //             | Role::Investigator
    //             | Role::Drunk
    //             | Role::Fortuneteller => {
    //                 res.push(i);
    //             }
    //             _ => (),
    //         }
    //     }
    //
    //     return res;
    // }

    pub(super) fn get_next_active_setup(
        &self,
        previous_player: Option<PlayerIndex>,
    ) -> Option<PlayerIndex> {
        let start_index = match previous_player {
            Some(i) => i + 1,
            None => 0,
        };

        for i in start_index..self.players.len() {
            let role = self.players[i].role;
            match role {
                Role::Washerwoman
                | Role::Librarian
                | Role::Investigator
                | Role::Drunk
                | Role::Fortuneteller => {
                    return Some(i);
                }
                _ => (),
            }
        }

        return None;
    }
}

impl Role {
    pub(super) fn setup_action(&self, player_index: PlayerIndex) -> Option<ChangeRequest> {
        match self {
            Role::Washerwoman => Some(washerwoman_librarian_investigator(
                player_index,
                *self,
                CharacterType::Townsfolk,
                "Washerwoman Townsfolk".to_owned(),
                "Washerwoman Wrong".to_owned(),
            )),
            Role::Librarian => Some(washerwoman_librarian_investigator(
                player_index,
                *self,
                CharacterType::Outsider,
                "Librarian Outsider".to_owned(),
                "Librarian Wrong".to_owned(),
            )),
            Role::Investigator => Some(washerwoman_librarian_investigator(
                player_index,
                *self,
                CharacterType::Minion,
                "Investigator Minion".to_owned(),
                "Investigator Wrong".to_owned(),
            )),
            Role::Drunk => Some(drunk(player_index)),
            Role::Fortuneteller => Some(fortune_teller(player_index)),
            _ => None,
        }
    }
}

fn washerwoman_librarian_investigator(
    player_index: PlayerIndex,
    role: Role,
    target_char_type: CharacterType,
    right_effect: String,
    wrong_effect: String,
) -> ChangeRequest {
    assert!(matches!(
        role,
        Role::Washerwoman | Role::Librarian | Role::Investigator
    ));

    // TODO: Update this, a role should be able to trigger multiple chained change reqeusts
    let change_type = ChangeType::ChoosePlayers(2);
    let check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> {
        // let target_player_indices: &Vec<PlayerIndex> = match args {
        //     Args::PlayerIndices(pv) => pv,
        //     _ => return Err(()),
        // };
        let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_player_indices.len() != 2 {
            return Err(());
        }

        for target_player_index in target_player_indices {
            let player = &state.players[*target_player_index];
            if player.role.get_type() == target_char_type || player.role == Role::Spy {
                return Ok(true);
            }
        }

        return Ok(false);
    };

    let state_change = move |state: &mut State, args: ChangeArgs| {
        let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

        for target_player_index in target_player_indices {
            let player = &state.players[target_player_index];
            if player.role.get_type() == target_char_type || player.role == Role::Spy {
                state.add_status(right_effect.clone(), player_index, target_player_index);
            } else {
                state.add_status(wrong_effect.clone(), player_index, target_player_index);
            }
        }
    };

    new_change_request!(change_type, check_func, state_change)
}

fn drunk(player_index: PlayerIndex) -> ChangeRequest {
    let change_type = ChangeType::ChooseRoles(1);
    let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
        let roles = unwrap_args_err!(args, ChangeArgs::Roles(r) => r);

        if roles.len() != 1 {
            return Err(());
        }

        return Ok(true);
    };

    // Choose a townsfolk role for the storyteller to replace the drunk with
    // Swap the chosen role with drunk, but give them a status effect that they
    // are actually the drunk
    // Essentially, the drunk should never actually be in play, the actual role
    // should be swapped out but a note is added that this player is indeed the
    // drunk
    let state_change = move |state: &mut State, args: ChangeArgs| {
        let roles = match args {
            ChangeArgs::Roles(rv) => rv,
            _ => panic!("Wrong input type"),
        };
        state.add_status(format!("{} Ability", roles[0]), player_index, player_index);
    };

    new_change_request!(change_type, check_func, state_change)
}

fn fortune_teller(player_index: PlayerIndex) -> ChangeRequest {
    let change_type = ChangeType::ChoosePlayers(1);
    let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
        let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_players.len() != 1 {
            return Err(());
        }

        return Ok(true);
    };
    // Get storyteller input on who red-herring is
    // Add a red-herring through status effects
    let state_change = move |state: &mut State, args: ChangeArgs| {
        let target_players = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);
        let affected_player_index = target_players[0];
        state.add_status(
            "Fortune Teller Red Herring".to_owned(),
            player_index,
            affected_player_index,
        );
    };

    new_change_request!(change_type, check_func, state_change)
}
