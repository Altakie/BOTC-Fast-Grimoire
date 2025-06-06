use crate::engine::{
    player::{CharacterType, Role},
    state::{PlayerIndex, State, status_effects::StatusEffects},
};
use std::sync::Arc;
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

#[derive(Debug, Clone)]
pub(crate) enum ChangeType {
    ChoosePlayers(usize),
    ChooseRoles(usize),
    Voting,
    // Display
    Display,
}

#[derive(Debug, Clone)]
pub(crate) enum Args {
    PlayerIndices(Vec<PlayerIndex>),
    Roles(Vec<Role>),
}

macro_rules! unwrap_args_err {
    ($args:expr,$pat:pat => $guard:expr) => {
        match $args {
            $pat => $guard,
            _ => return Err(()),
        }
    };
}

macro_rules! unwrap_args_panic {
    ($args:expr,$pat:pat => $guard:expr) => {
        match $args {
            $pat => $guard,
            _ => panic!("Wrong Args"),
        }
    };
}

/// A struct to give the interface the necessary information to resolve
///
/// # Fields
///
/// * Change Type : A enum to tell the ui which input component to load and how much input to
/// ask for
/// * FnOnce(Args) -> bool : A check function that validates whether the input the component
/// returns will is a valid input for the state mutating function
/// * Fnonce(State, Args) -> A function that takes a state and args, and then mutates the state
/// accordingly. Think of this as saving the work that needs to be done for after the input
/// is received
#[derive(Clone)]
pub(crate) struct ChangeRequest {
    pub(crate) change_type: ChangeType,
    pub(crate) check_func: Arc<dyn Fn(&State, &Args) -> Result<bool, ()> + Send + Sync>,
    pub(crate) state_change_func: Arc<dyn Fn(&mut State, Args) + Send + Sync>,
}

macro_rules! new_change_request {
    ($cr:expr, $cf:expr, $scf:expr) => {
        ChangeRequest {
            change_type: $cr,
            check_func: Arc::new($cf),
            state_change_func: Arc::new($scf),
        }
    };
}

// TODO: What to do what to do
// Easy -> Want generic components for different types of inputs
// Those components should take in a signal, and also act kind of like a form
// When they return, they should what component it wants rendered and a function to be applied to
// the game state
impl State {
    pub(crate) fn get_active_players(&self) -> Vec<PlayerIndex> {
        let mut res: Vec<PlayerIndex> = vec![];
        for (i, player) in self.players.iter().enumerate() {
            match player.role {
                Role::Washerwoman
                | Role::Librarian
                | Role::Investigator
                | Role::Drunk
                | Role::Fortuneteller => {
                    res.push(i);
                }
                _ => (),
            }
        }

        return res;
    }
    /// Function to resolve a player's effect on the state in the setup phase
    ///
    /// # Args
    ///
    /// * player_index : Index of player to resolve for
    ///
    /// # Returns
    ///
    /// * Option<ChangeRequest> : A change request if the role does something, or none if it
    ///   doesn't
    pub(crate) fn resolve(&mut self, player_index: PlayerIndex) -> Option<ChangeRequest> {
        let role = &self.players[player_index].role;

        let res = match role {
            Role::Washerwoman => Some(washerwoman_librarian_investigator(
                player_index,
                *role,
                CharacterType::Townsfolk,
                StatusEffects::WasherwomanTownsfolk,
                StatusEffects::WasherwomanWrong,
            )),
            Role::Librarian => Some(washerwoman_librarian_investigator(
                player_index,
                *role,
                CharacterType::Outsider,
                StatusEffects::LibrarianOutsider,
                StatusEffects::LibrarianWrong,
            )),
            Role::Investigator => Some(washerwoman_librarian_investigator(
                player_index,
                *role,
                CharacterType::Minion,
                StatusEffects::InvestigatorMinion,
                StatusEffects::InvestigatorWrong,
            )),
            Role::Drunk => Some(drunk(player_index)),
            Role::Fortuneteller => Some(fortune_teller(player_index)),
            _ => None,
        };

        return res;
        // TODO: Log events that happen in the setup
    }
}

fn washerwoman_librarian_investigator(
    player_index: PlayerIndex,
    role: Role,
    target_char_type: CharacterType,
    right_effect: StatusEffects,
    wrong_effect: StatusEffects,
) -> ChangeRequest {
    assert!(matches!(
        role,
        Role::Washerwoman | Role::Librarian | Role::Investigator
    ));

    let change_type = ChangeType::ChoosePlayers(2);
    let check_func = move |state: &State, args: &Args| -> Result<bool, ()> {
        // let target_player_indices: &Vec<PlayerIndex> = match args {
        //     Args::PlayerIndices(pv) => pv,
        //     _ => return Err(()),
        // };
        let target_player_indices = unwrap_args_err!(args, Args::PlayerIndices(v) => v);

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

    let state_change = move |state: &mut State, args: Args| {
        let target_player_indices = unwrap_args_panic!(args, Args::PlayerIndices(v) => v);

        for target_player_index in target_player_indices {
            let player = &state.players[target_player_index];
            if player.role.get_type() == target_char_type || player.role == Role::Spy {
                state.add_status(right_effect, player_index, target_player_index);
            } else {
                state.add_status(wrong_effect, player_index, target_player_index);
            }
        }
    };

    new_change_request!(change_type, check_func, state_change)
}

fn drunk(player_index: PlayerIndex) -> ChangeRequest {
    let change_type = ChangeType::ChooseRoles(1);
    let check_func = move |_: &State, args: &Args| -> Result<bool, ()> {
        let roles = unwrap_args_err!(args, Args::Roles(r) => r);

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
    let state_change = move |state: &mut State, args: Args| {
        let roles = match args {
            Args::Roles(rv) => rv,
            _ => panic!("Wrong input type"),
        };
        state.players[player_index].role = roles[0];
        state.add_status(StatusEffects::TheDrunk, player_index, player_index);
    };

    new_change_request!(change_type, check_func, state_change)
}

fn fortune_teller(player_index: PlayerIndex) -> ChangeRequest {
    let change_type = ChangeType::ChoosePlayers(1);
    let check_func = move |_: &State, args: &Args| -> Result<bool, ()> {
        let target_players = unwrap_args_err!(args, Args::PlayerIndices(v) => v);

        if target_players.len() != 1 {
            return Err(());
        }

        return Ok(true);
    };
    // Get storyteller input on who red-herring is
    // Add a red-herring through status effects
    let state_change = move |state: &mut State, args: Args| {
        let target_players = unwrap_args_panic!(args, Args::PlayerIndices(v) => v);
        let affected_player_index = target_players[0];
        state.add_status(
            StatusEffects::FortuneTellerRedHerring,
            player_index,
            affected_player_index,
        );
    };

    new_change_request!(change_type, check_func, state_change)
}
