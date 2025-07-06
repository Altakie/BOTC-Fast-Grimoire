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
    pub(super) fn setup_action(&self, player_index: PlayerIndex) -> Option<Vec<ChangeRequest>> {
        match self {
            Role::Washerwoman => Some(washerwoman_librarian_investigator(
                player_index,
                *self,
                CharacterType::Townsfolk,
                "Washerwoman Townsfolk",
                "Washerwoman Wrong",
            )),
            Role::Librarian => Some(washerwoman_librarian_investigator(
                player_index,
                *self,
                CharacterType::Outsider,
                "Librarian Outsider",
                "Librarian Wrong",
            )),
            Role::Investigator => Some(washerwoman_librarian_investigator(
                player_index,
                *self,
                CharacterType::Minion,
                "Investigator Minion",
                "Investigator Wrong",
            )),
            Role::Fortuneteller => Some(fortune_teller(player_index)),
            Role::Drunk => Some(drunk(player_index)),
            _ => None,
        }
    }
}

fn washerwoman_librarian_investigator(
    player_index: PlayerIndex,
    role: Role,
    target_char_type: CharacterType,
    right_effect: &'static str,
    wrong_effect: &'static str,
) -> Vec<ChangeRequest> {
    // Only these 3 roles should be calling this method (for now)
    assert!(matches!(
        role,
        Role::Washerwoman | Role::Librarian | Role::Investigator
    ));

    let target_type = {
        match role {
            Role::Washerwoman => "Townsfolk",
            Role::Librarian => "Outsider",
            Role::Investigator => "Minion",
            _ => panic!("Should never happen"),
        }
    };
    let right_description = format!("Select a {target_type}");
    let wrong_description = "Select a different player".to_string();

    // TODO: Update this, a role should be able to trigger multiple chained change reqeusts
    // Trigger two chained change requests, one to choose the player the washerwoman is targeting,
    // and one for the wrong player
    let change_type = ChangeType::ChoosePlayers(1);
    let right_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> {
        // let target_player_indices: &Vec<PlayerIndex> = match args {
        //     Args::PlayerIndices(pv) => pv,
        //     _ => return Err(()),
        // };
        let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_player_indices.len() != 1 {
            return Err(());
        }

        // TODO: Redundant code that can be cleaned up here
        for target_player_index in target_player_indices {
            let player = &state.players[*target_player_index];
            if player.role.get_type() == target_char_type || player.role == Role::Spy {
                return Ok(true);
            }
        }

        return Ok(false);
    };

    let right_state_change = move |state: &mut State, args: ChangeArgs| {
        let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

        // TODO: Redundant code that can be cleaned up here
        for target_player_index in target_player_indices {
            let player = &state.players[target_player_index];
            if player.role.get_type() == target_char_type || player.role == Role::Spy {
                state.add_status(right_effect.to_string(), player_index, target_player_index);
            } else {
                state.add_status(wrong_effect.to_string(), player_index, target_player_index);
            }
        }
    };

    let wrong_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> {
        // let target_player_indices: &Vec<PlayerIndex> = match args {
        //     Args::PlayerIndices(pv) => pv,
        //     _ => return Err(()),
        // };
        let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_player_indices.len() != 1 {
            return Err(());
        }

        let target_player_index = target_player_indices[0];
        if state
            .get_afflicted_statuses(target_player_index)
            .iter()
            .any(|se| se.status_type == *right_effect.to_string())
        {
            return Ok(false);
        }
        return Ok(true);
    };

    let wrong_state_change = move |state: &mut State, args: ChangeArgs| {
        let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

        // Assign the chosen player the wrong status effect
        let target_player_index = target_player_indices[0];
        state.add_status(wrong_effect.to_string(), player_index, target_player_index);
    };

    vec![
        new_change_request!(
            change_type,
            right_check_func,
            right_state_change,
            right_description
        ),
        new_change_request!(
            change_type,
            wrong_check_func,
            wrong_state_change,
            wrong_description
        ),
    ]
}

fn drunk(player_index: PlayerIndex) -> Vec<ChangeRequest> {
    let description = "Select a not in play Townfolk role".to_string();
    let change_type = ChangeType::ChooseRoles(1);
    let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
        let roles = unwrap_args_err!(args, ChangeArgs::Roles(r) => r);

        if roles.len() != 1 {
            return Err(());
        }

        if roles[0].get_type() != CharacterType::Townsfolk {
            return Ok(false);
        }

        return Ok(true);
    };

    let state_change = move |state: &mut State, args: ChangeArgs| {
        let roles = match args {
            ChangeArgs::Roles(rv) => rv,
            _ => panic!("Wrong input type"),
        };
        state.add_status(format!("{} Ability", roles[0]), player_index, player_index);
    };

    vec![new_change_request!(
        change_type,
        check_func,
        state_change,
        description
    )]
}

fn fortune_teller(player_index: PlayerIndex) -> Vec<ChangeRequest> {
    let description = "Select a red-herring for the Fortune Teller".to_string();

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

    vec![new_change_request!(
        change_type,
        check_func,
        state_change,
        description
    )]
}
