use super::{
    player::roles::Roles,
    state::{PlayerIndex, State},
};
use std::{fmt::Debug, sync::Arc};

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

/**
A struct to give the interface the necessary information to resolve

# Fields

* Change Type : A enum to tell the ui which input component to load and how much input to
  ask for
* FnOnce(Args) -> bool : A check function that validates whether the input the component
  returns will is a valid input for the state mutating function
* Fnonce(State, Args) -> A function that takes a state and args, and then mutates the state
  accordingly. Think of this as saving the work that needs to be done for after the input
  is received
*/
#[derive(Clone)]
pub(crate) struct ChangeRequest {
    pub(crate) change_type: ChangeType,
    pub(crate) check_func: Option<CheckFuncPtr>,
    pub(crate) state_change_func: Option<StateChangeFuncPtr>,
    pub(crate) description: String,
    pub(crate) clear: bool,
}

// impl ChangeRequest {
//     pub fn new()
// }

impl Debug for ChangeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChangeRequest")
            .field("change_type", &self.change_type)
            .field("description", &self.description)
            .field("clear", &self.clear)
            .finish()
    }
}

// TODO: This should return a Result<Option<bool>> unless we want to return some sort of error
// message as to why the result is invalid. Then we should make the result type a string or
// some sort of error type enum
// Types of errors:
// Invalid Player(s) selected (reason)
// Wrong Number of players selected(required, current)
pub type CheckFunc = dyn Fn(&State, &ChangeArgs) -> Result<bool, ChangeError> + Send + Sync;
#[derive(Clone)]
pub struct CheckFuncPtr(Arc<CheckFunc>);
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

// impl From<Arc<CheckFunc>> for CheckFuncPtr {
//     fn from(
//         value: Arc<dyn Fn(&State, &ChangeArgs) -> Result<bool, ChangeError> + Send + Sync>,
//     ) -> Self {
//         Self(value)
//     }
// }

pub type StateChangeFunc = dyn Fn(&mut State, ChangeArgs) -> Option<ChangeRequest> + Send + Sync;
#[derive(Clone)]
pub struct StateChangeFuncPtr(Arc<StateChangeFunc>);

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

impl ChangeRequest {
    pub(crate) fn new<Cf, Scf>(
        change_type: ChangeType,
        description: String,
        check_func: Cf,
        state_change_func: Scf,
    ) -> Self
    where
        Cf: Fn(&State, &ChangeArgs) -> Result<bool, ChangeError> + Send + Sync + 'static,
        Scf: Fn(&mut State, ChangeArgs) -> Option<ChangeRequest> + Send + Sync + 'static,
    {
        Self {
            change_type,
            check_func: Some(CheckFuncPtr::new(check_func)),
            state_change_func: Some(StateChangeFuncPtr::new(state_change_func)),
            description,
            clear: true,
        }
    }

    pub(crate) fn new_display(change_type: ChangeType, description: String) -> Self {
        Self {
            change_type,
            check_func: None,
            state_change_func: None,
            description,
            clear: true,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ChangeError {
    InvalidSelectedPlayer { reason: String },
    WrongNumberOfSelectedPlayers { wanted: usize, got: usize },
    WrongNumberOfSelectedRoles { wanted: usize, got: usize },
    WrongArgType,
}
