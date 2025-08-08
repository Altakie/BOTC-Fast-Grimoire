use leptos::{attr::AttributeValue, prelude::StorageAccess};

use super::{
    player::roles::Roles,
    state::{PlayerIndex, State},
};
use std::{ops::Deref, sync::Arc};

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
/// Unwrap args and return an error if it's the wrong arg type. Takes in (args, pattern => guard)
macro_rules! unwrap_args_err {
    ($args:expr,$pat:pat => $guard:expr) => {
        match $args {
            $pat => $guard,
            _ => return Err(ChangeError::WrongArgType),
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
    // TODO: This should return a Result<Option<bool>> unless we want to return some sort of error
    // message as to why the result is invalid. Then we should make the result type a string or
    // some sort of error type enum
    // Types of errors:
    // Invalid Player(s) selected (reason)
    // Wrong Number of players selected(required, current)
    pub(crate) check_func: Option<CheckFuncPtr>,
    // TODO: Want state_change_func to return a Option<ChangeRequest>, that way change requests are
    // properly chainable, especially based on conditionals
    pub(crate) state_change_func: Option<StateChangeFuncPtr>,
    pub(crate) description: String,
    pub(crate) clear: bool,
}

// impl ChangeRequest {
//     pub fn new()
// }

#[derive(Clone)]
pub struct CheckFuncPtr(
    Arc<dyn Fn(&State, &ChangeArgs) -> Result<bool, ChangeError> + Send + Sync>,
);
impl CheckFuncPtr {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(&State, &ChangeArgs) -> Result<bool, ChangeError> + Send + Sync + 'static,
    {
        Self(Arc::new(func))
    }

    pub fn call(&self, state: &State, args: &ChangeArgs) -> Result<bool, ChangeError> {
        self.0(state, args)
    }
}

impl From<Arc<dyn Fn(&State, &ChangeArgs) -> Result<bool, ChangeError> + Send + Sync>>
    for CheckFuncPtr
{
    fn from(
        value: Arc<dyn Fn(&State, &ChangeArgs) -> Result<bool, ChangeError> + Send + Sync>,
    ) -> Self {
        Self(value)
    }
}

#[derive(Clone)]
pub struct StateChangeFuncPtr(
    Arc<dyn Fn(&mut State, ChangeArgs) -> Option<ChangeRequest> + Send + Sync>,
);

impl StateChangeFuncPtr {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(&mut State, ChangeArgs) -> Option<ChangeRequest> + Send + Sync + 'static,
    {
        Self(Arc::new(func))
    }

    pub fn call(&self, state: &mut State, args: ChangeArgs) -> Option<ChangeRequest> {
        self.0(state, args)
    }
}

impl From<Arc<dyn Fn(&mut State, ChangeArgs) -> Option<ChangeRequest> + Send + Sync>>
    for StateChangeFuncPtr
{
    fn from(
        value: Arc<dyn Fn(&mut State, ChangeArgs) -> Option<ChangeRequest> + Send + Sync>,
    ) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
pub enum ChangeError {
    InvalidSelectedPlayer { reason: String },
    WrongNumberOfSelectedPlayers { wanted: usize, got: usize },
    WrongNumberOfSelectedRoles { wanted: usize, got: usize },
    WrongArgType,
}

#[macro_export]
/// Args go as follows : change type, description, (Optional) check_func, state_change_func
macro_rules! new_change_request {
    ($ct:expr, $desc:expr, $cf:expr, $scf:expr) => {
        ChangeRequest {
            change_type: $ct,
            description: $desc.into(),
            check_func: CheckFuncPtr::new($cf).into(),
            state_change_func: StateChangeFuncPtr::new($scf).into(),
            clear: true,
        }
    };
    ($ct:expr, $desc:expr) => {
        ChangeRequest {
            change_type: $ct,
            check_func: None,
            state_change_func: None,
            description: $desc.into(),
            clear: true,
        }
    };
    (noclear: $ct:expr, $desc:expr, $cf:expr, $scf:expr) => {
        ChangeRequest {
            change_type: $ct,
            description: $desc.into(),
            check_func: CheckFuncPtr::new($cf).into(),
            state_change_func: StateChangeFuncPtr::new($scf).into(),
            clear: false,
        }
    };
}
