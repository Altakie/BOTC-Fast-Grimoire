use crate::{
    engine::{
        change_request::{ChangeArgs, ChangeRequest, ChangeType},
        player::{CharacterType, roles::Roles},
        state::{PlayerIndex, State, status_effects::StatusType},
    },
    new_change_request, unwrap_args_err, unwrap_args_panic,
};

// use leptos::prelude::*;
// use reactive_stores::Store;

impl State {
    pub(super) fn get_next_active_setup(
        &self,
        previous_player: Option<PlayerIndex>,
    ) -> Option<PlayerIndex> {
        let start_index = match previous_player {
            Some(i) => i + 1,
            None => 0,
        };

        let players = self.get_players();

        for (i, player) in players.iter().skip(start_index + 1).enumerate() {
            if let Some(_) = player.role.setup_order() {
                return Some(i);
            }
        }

        return None;
    }
}

// TODO: Move all this logic to separate files for each role. Perhaps make a mod that contains all
// the roles
impl Roles {
    pub(super) fn setup_action(&self, player_index: PlayerIndex) -> Option<Vec<ChangeRequest>> {
        match self {
            Roles::Washerwoman => Some(washerwoman_librarian_investigator(
                player_index,
                *self,
                CharacterType::Townsfolk,
                StatusType::WasherwomanTownsfolk,
                StatusType::WasherwomanWrong,
            )),
            Roles::Librarian => Some(washerwoman_librarian_investigator(
                player_index,
                *self,
                CharacterType::Outsider,
                StatusType::LibrarianOutsider,
                StatusType::LibrarianWrong,
            )),
            Roles::Investigator => Some(washerwoman_librarian_investigator(
                player_index,
                *self,
                CharacterType::Minion,
                StatusType::InvestigatorMinion,
                StatusType::InvestigatorWrong,
            )),
            Roles::Fortuneteller => Some(fortune_teller(player_index)),
            Roles::Drunk => Some(drunk(player_index)),
            Roles::Soldier => {
                // Just add protected status effect and only remove upon death
                Some(add_status_to_self(
                    player_index,
                    *self,
                    StatusType::DemonProtected,
                ))
            }
            Roles::Mayor => {
                // No night one ability, but add effect to yourself
                Some(add_status_to_self(
                    player_index,
                    *self,
                    StatusType::MayorBounceKill,
                ))
            }
            Roles::Recluse => Some(add_status_to_self(
                player_index,
                *self,
                StatusType::AppearsEvil,
            )),
            Roles::Spy => Some(add_status_to_self(
                player_index,
                *self,
                StatusType::AppearsGood,
            )),
            // Role::Virgin => {
            // TODO: Should have change effects that can get resolved without storyteller
            // involvment. For now maybe just have as display
            //     // FIX: Also should happen with setup
            //     // Add a status effect that if someone nominates you, they die
            //     // Maybe instead add this to the nominate method, for now it is easier to add to
            //      // the nominate method
            //     todo!()
            // }
            // Role::Saint =>
            // FIX: Should also have a status added in setup that causes game end? Maybe no status
            // needed
            // todo!(),  // No night one ability
            _ => None,
        }
    }
}

fn washerwoman_librarian_investigator(
    player_index: PlayerIndex,
    role: Roles,
    target_char_type: CharacterType,
    right_effect: StatusType,
    wrong_effect: StatusType,
) -> Vec<ChangeRequest> {
    // Only these 3 roles should be calling this method (for now)
    assert!(matches!(
        role,
        Roles::Washerwoman | Roles::Librarian | Roles::Investigator
    ));

    let target_type = {
        match role {
            Roles::Washerwoman => "Townsfolk",
            Roles::Librarian => "Outsider",
            Roles::Investigator => "Minion",
            _ => panic!("Should never happen"),
        }
    };
    let right_description = format!("Select a {target_type}");
    let wrong_description = "Select a different player".to_string();

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
            if *target_player_index == player_index {
                return Ok(false);
            }

            let player = &state.players[*target_player_index];
            if player.role.get_type() == target_char_type
                || matches!(player.role, Roles::Spy | Roles::Recluse)
            {
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
            if player.role.get_type() == target_char_type
                || matches!(player.role, Roles::Spy | Roles::Recluse)
            {
                state.add_status(right_effect, player_index, target_player_index);
            } else {
                state.add_status(wrong_effect, player_index, target_player_index);
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

        if target_player_index == player_index {
            return Ok(false);
        }

        if state
            .get_afflicted_statuses(target_player_index)
            .iter()
            .any(|se| se.status_type == right_effect)
        {
            return Ok(false);
        }
        return Ok(true);
    };

    let wrong_state_change = move |state: &mut State, args: ChangeArgs| {
        let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

        // Assign the chosen player the wrong status effect
        let target_player_index = target_player_indices[0];
        state.add_status(wrong_effect, player_index, target_player_index);
    };

    vec![
        new_change_request!(
            change_type,
            right_description,
            right_check_func,
            right_state_change
        ),
        new_change_request!(
            change_type,
            wrong_description,
            wrong_check_func,
            wrong_state_change
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
        state.add_status(
            StatusType::OtherRoleAbility(roles[0]),
            player_index,
            player_index,
        );
        state.add_status(StatusType::Drunk, player_index, player_index);
    };

    vec![new_change_request!(
        change_type,
        description,
        check_func,
        state_change
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

        if target_players[0] == player_index {
            return Ok(false);
        }

        return Ok(true);
    };
    // Get storyteller input on who red-herring is
    // Add a red-herring through status effects
    let state_change = move |state: &mut State, args: ChangeArgs| {
        let target_players = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);
        let affected_player_index = target_players[0];
        state.add_status(
            StatusType::FortuneTellerRedHerring,
            player_index,
            affected_player_index,
        );
    };

    vec![new_change_request!(
        change_type,
        description,
        check_func,
        state_change
    )]
}

fn add_status_to_self(
    player_index: PlayerIndex,
    role: Roles,
    status_type: StatusType,
) -> Vec<ChangeRequest> {
    // TODO: Need new change type that requires no storyteller involvement
    let change_type = ChangeType::Display;
    let message = format!(
        "{} will add status \"{}\" to themselves. Nothing to do, just take note of this",
        role, status_type
    );

    let check_func = move |_: &State, _: &ChangeArgs| -> Result<bool, ()> {
        return Ok(true);
    };

    let state_change = move |state: &mut State, _| {
        state.add_status(status_type, player_index, player_index);
    };

    vec![new_change_request!(
        change_type,
        message,
        check_func,
        state_change
    )]
}
