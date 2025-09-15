use crate::engine::player::Player;

use super::{
    player::roles::Roles,
    state::{PlayerIndex, State},
};
use std::{borrow::Cow, fmt::Debug, ops::Deref, sync::Arc};

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
    Blank,
}

impl ChangeArgs {
    pub fn extract_player_indicies(&self) -> Result<Vec<PlayerIndex>, ChangeError> {
        match self {
            ChangeArgs::PlayerIndices(items) => Ok(items.to_owned()),
            _ => Err(ChangeError::WrongArgType),
        }
    }

    pub fn extract_roles(self) -> Result<Vec<Roles>, ChangeError> {
        match self {
            ChangeArgs::Roles(items) => Ok(items.to_owned()),
            _ => Err(ChangeError::WrongArgType),
        }
    }
}

pub fn check_len<T>(vec: &[T], desired_len: usize) -> Result<(), ChangeError> {
    let len = vec.len();
    if len != desired_len {
        return Err(ChangeError::WrongNumberOfSelectedPlayers {
            wanted: desired_len,
            got: len,
        });
    }

    return Ok(());
}

#[derive(Clone)]
pub(crate) struct ChangeRequest {
    pub(crate) change_type: ChangeType,
    pub(crate) filter_func: Option<FilterFuncPtr>,
    pub(crate) state_change_func: Option<StateChangeFuncPtr>,
    pub(crate) description: String,
}

// impl ChangeRequest {
//     pub fn new()
// }

impl Debug for ChangeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChangeRequest")
            .field("change_type", &self.change_type)
            .field("description", &self.description)
            .finish()
    }
}

pub type FilterFunc = dyn Fn(&Player) -> bool + Send + Sync;
#[derive(Clone)]
pub struct FilterFuncPtr(Arc<FilterFunc>);
impl FilterFuncPtr {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(&Player) -> bool + Send + Sync + 'static,
    {
        Self(Arc::new(func))
    }

    pub fn call(&self, player: &Player) -> bool {
        self.0(player)
    }
}

// impl From<Arc<CheckFunc>> for CheckFuncPtr {
//     fn from
//         value: Arc<dyn Fn(&State, &ChangeArgs) -> Result<bool, ChangeError> + Send + Sync>,
//     ) -> Self {
//         Self(value)
//     }
// }

pub type StateChangeFunc = dyn Fn(&mut State, ChangeArgs) -> ChangeResult + Send + Sync;
#[derive(Clone)]
pub struct StateChangeFuncPtr(Arc<StateChangeFunc>);

impl StateChangeFuncPtr {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(&mut State, ChangeArgs) -> ChangeResult + Send + Sync + 'static,
    {
        Self(Arc::new(func))
    }

    pub fn call(&self, state: &mut State, args: ChangeArgs) -> ChangeResult {
        self.0(state, args)
    }
}

impl ChangeRequest {
    pub(crate) fn new(
        change_type: ChangeType,
        description: String,
        state_change_func: StateChangeFuncPtr,
    ) -> Self
where {
        Self {
            change_type,
            filter_func: None,
            state_change_func: Some(state_change_func),
            description,
        }
    }

    pub(crate) fn new_with_filter(
        change_type: ChangeType,
        description: String,
        filter_func: FilterFuncPtr,
        state_change_func: StateChangeFuncPtr,
    ) -> Self
where {
        Self {
            change_type,
            filter_func: Some(filter_func),
            state_change_func: Some(state_change_func),
            description,
        }
    }

    pub(crate) fn new_display(change_type: ChangeType, description: String) -> Self {
        Self {
            change_type,
            filter_func: None,
            state_change_func: None,
            description,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ChangeError {
    InvalidSelectedPlayer { reason: String },
    InvalidSelectedRole { reason: String },
    WrongNumberOfSelectedPlayers { wanted: usize, got: usize },
    WrongNumberOfSelectedRoles { wanted: usize, got: usize },
    WrongArgType,
    BlankArgs,
}

pub type ChangeResult = Result<Option<ChangeRequest>, ChangeError>;

impl From<ChangeRequest> for ChangeResult {
    fn from(value: ChangeRequest) -> Self {
        Ok(Some(value))
    }
}
