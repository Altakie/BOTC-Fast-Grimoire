use crate::engine::player::Player;

use super::{
    player::roles::RoleNames,
    state::{PlayerIndex, State},
};
use std::{fmt::Debug, ops::Deref, sync::Arc};

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
    Roles(Vec<RoleNames>),
    Blank,
}

impl ChangeArgs {
    pub fn extract_player_indicies(&self) -> Result<Vec<PlayerIndex>, ChangeError> {
        match self {
            ChangeArgs::PlayerIndices(items) => Ok(items.to_owned()),
            _ => Err(ChangeError::WrongArgType),
        }
    }

    pub fn extract_roles(self) -> Result<Vec<RoleNames>, ChangeError> {
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

    Ok(())
}

#[derive(Clone)]
pub(crate) struct ChangeRequest {
    change_type: ChangeType,
    filter_func: Option<FilterFuncPtr>,
    state_change_func: Option<StateChangeFuncPtr>,
    // on_success: Option<SuccessFunc>
    description: String,
}

impl ChangeRequest {
    pub(crate) fn new_builder(
        change_type: ChangeType,
        description: String,
    ) -> ChangeRequestBuilder {
        ChangeRequestBuilder {
            change_type,
            filter_func: None,
            state_change_func: None,
            description,
        }
    }

    pub(crate) fn get_change_type(&self) -> ChangeType {
        self.change_type
    }

    pub(crate) fn get_description(&self) -> String {
        self.description.clone()
    }

    pub(crate) fn get_filter_func(&self) -> Option<&FilterFuncPtr> {
        match &self.filter_func {
            Some(filter_func) => Some(filter_func),
            None => None,
        }
    }

    pub(crate) fn get_state_change_func(&self) -> Option<&StateChangeFuncPtr> {
        match &self.state_change_func {
            Some(state_change_func) => Some(state_change_func),
            None => None,
        }
    }
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

#[derive(Clone, Debug)]
pub(crate) struct ChangeRequestBuilder {
    pub(crate) change_type: ChangeType,
    pub(crate) filter_func: Option<FilterFuncPtr>,
    pub(crate) state_change_func: Option<StateChangeFuncPtr>,
    pub(crate) description: String,
}

impl ChangeRequestBuilder {
    pub(crate) fn build(self) -> ChangeRequest {
        ChangeRequest {
            change_type: self.change_type,
            filter_func: self.filter_func,
            state_change_func: self.state_change_func,
            description: self.description,
        }
    }

    // This is because the description should be able to be modified after creation
    pub(crate) fn description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub(crate) fn change_description<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> String,
    {
        self.description = f(self.description);
        self
    }

    pub(crate) fn filter_func(mut self, filter_func: FilterFuncPtr) -> Self {
        self.filter_func = Some(filter_func);
        self
    }

    pub(crate) fn state_change_func(mut self, state_change_func: StateChangeFuncPtr) -> Self {
        self.state_change_func = Some(state_change_func);
        self
    }

    pub(crate) fn clear_state_change_func(mut self) -> Self {
        self.state_change_func = None;
        self
    }
}

pub(crate) type FilterFunc = dyn Fn(PlayerIndex, &Player) -> bool + Send + Sync;
#[derive(Clone)]
pub(crate) struct FilterFuncPtr(Arc<FilterFunc>);
impl FilterFuncPtr {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(PlayerIndex, &Player) -> bool + Send + Sync + 'static,
    {
        Self(Arc::new(func))
    }

    pub fn call(&self, player_index: PlayerIndex, player: &Player) -> bool {
        self.0(player_index, player)
    }
}

impl Debug for FilterFuncPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FilterFuncPtr").finish()
    }
}

// impl From<Arc<CheckFunc>> for CheckFuncPtr {
//     fn from
//         value: Arc<dyn Fn(&State, &ChangeArgs) -> Result<bool, ChangeError> + Send + Sync>,
//     ) -> Self {
//         Self(value)
//     }
// }

pub(crate) type StateChangeFunc = dyn Fn(&mut State, ChangeArgs) -> ChangeResult + Send + Sync;
#[derive(Clone)]
pub(crate) struct StateChangeFuncPtr(Arc<StateChangeFunc>);

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

    pub fn reassign(&mut self, other: StateChangeFuncPtr) {
        self.0 = other.0
    }
}

impl Deref for StateChangeFuncPtr {
    type Target = Arc<StateChangeFunc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for StateChangeFuncPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StateChangeFuncPtr").finish()
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

pub type ChangeResult = Result<(), ChangeError>;

// impl From<ChangeRequestBuilder> for ChangeResult {
//     fn from(value: ChangeRequestBuilder) -> Self {
//         Ok(Some(value))
//     }
// }
