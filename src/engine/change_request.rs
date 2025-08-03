use super::{
    player::roles::Roles,
    state::{PlayerIndex, State},
};
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ChangeType {
    ChoosePlayers(usize),
    ChooseRoles(usize),
    Voting,
    NoStoryteller,
    // Display
    Display,
}

#[derive(Debug, Clone)]
pub(crate) enum ChangeArgs {
    PlayerIndices(Vec<PlayerIndex>),
    Roles(Vec<Roles>),
}

#[macro_export]
macro_rules! unwrap_args_err {
    ($args:expr,$pat:pat => $guard:expr) => {
        match $args {
            $pat => $guard,
            _ => return Err(()),
        }
    };
}

#[macro_export]
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
    pub(crate) check_func:
        Option<Arc<dyn Fn(&State, &ChangeArgs) -> Result<bool, ()> + Send + Sync>>,
    pub(crate) state_change_func: Option<Arc<dyn Fn(&mut State, ChangeArgs) + Send + Sync>>,
    pub(crate) description: String,
    pub(crate) clear: bool,
}

#[macro_export]
macro_rules! new_change_request {
    ($ct:expr, $desc:expr, $cf:expr, $scf:expr) => {
        ChangeRequest {
            change_type: $ct,
            description: $desc,
            check_func: Some(std::sync::Arc::new($cf)),
            state_change_func: Some(std::sync::Arc::new($scf)),
            clear: true,
        }
    };
    ($ct:expr, $desc:expr) => {
        ChangeRequest {
            change_type: $ct,
            check_func: None,
            state_change_func: None,
            description: $desc,
            clear: true,
        }
    };
    (noclear: $ct:expr, $desc:expr, $cf:expr, $scf:expr) => {
        ChangeRequest {
            change_type: $ct,
            description: $desc,
            check_func: Some(std::sync::Arc::new($cf)),
            state_change_func: Some(std::sync::Arc::new($scf)),
            clear: false,
        }
    };
}
