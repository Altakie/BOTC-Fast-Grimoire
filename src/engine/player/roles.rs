use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::sync::Arc;

use crate::{
    engine::{
        change_request::ChangeRequest,
        player::{roles::townsfolk::*, *},
        state::{PlayerIndex, State, Step},
    },
    initialization::CharacterTypeCounts,
};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Roles {
    // Special Roles that are in every game
    // DEMON,
    // MINION,
    // Normal Roles
    Investigator,
    Empath,
    Gossip,
    Innkeeper,
    Washerwoman,
    Librarian,
    Chef,
    Fortuneteller,
    Undertaker,
    Virgin,
    Soldier,
    Slayer,
    Mayor,
    Monk,
    Ravenkeeper,
    Drunk,
    Saint,
    Butler,
    Recluse,
    Spy,
    Baron,
    Scarletwoman,
    Poisoner,
    Imp,
}

impl Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Roles::Investigator => write!(f, "Investigator"),
            Roles::Empath => write!(f, "Empath"),
            Roles::Gossip => write!(f, "Gossip"),
            Roles::Innkeeper => write!(f, "Innkeeper"),
            Roles::Washerwoman => write!(f, "Washerwoman"),
            Roles::Librarian => write!(f, "Librarian"),
            Roles::Chef => write!(f, "Chef"),
            Roles::Fortuneteller => write!(f, "Fortuneteller"),
            Roles::Undertaker => write!(f, "Undertaker"),
            Roles::Virgin => write!(f, "Virgin"),
            Roles::Soldier => write!(f, "Soldier"),
            Roles::Slayer => write!(f, "Slayer"),
            Roles::Mayor => write!(f, "Mayor"),
            Roles::Monk => write!(f, "Monk"),
            Roles::Ravenkeeper => write!(f, "Ravenkeeper"),
            Roles::Drunk => write!(f, "Drunk"),
            Roles::Saint => write!(f, "Saint"),
            Roles::Butler => write!(f, "Butler"),
            Roles::Recluse => write!(f, "Recluse"),
            Roles::Spy => write!(f, "Spy"),
            Roles::Baron => write!(f, "Baron"),
            Roles::Scarletwoman => write!(f, "Scarletwoman"),
            Roles::Poisoner => write!(f, "Poisoner"),
            Roles::Imp => write!(f, "Imp"),
        }
    }
}

impl Roles {
    pub(crate) fn convert(&self) -> Arc<dyn Role> {
        // TODO: Make classes to roles and resolve them here
        match self {
            Roles::Investigator => todo!(),
            Roles::Empath => todo!(),
            Roles::Gossip => todo!(),
            Roles::Innkeeper => todo!(),
            Roles::Washerwoman => todo!(),
            Roles::Librarian => todo!(),
            Roles::Chef => Arc::new(Chef::default()),
            Roles::Fortuneteller => todo!(),
            Roles::Undertaker => todo!(),
            Roles::Virgin => todo!(),
            Roles::Soldier => todo!(),
            Roles::Slayer => todo!(),
            Roles::Mayor => todo!(),
            Roles::Monk => todo!(),
            Roles::Ravenkeeper => todo!(),
            Roles::Drunk => todo!(),
            Roles::Saint => todo!(),
            Roles::Butler => todo!(),
            Roles::Recluse => todo!(),
            Roles::Spy => todo!(),
            Roles::Baron => todo!(),
            Roles::Scarletwoman => todo!(),
            Roles::Poisoner => todo!(),
            Roles::Imp => todo!(),
        }
    }
}

pub(crate) trait Role: Display + Send + Sync {
    fn name(&self) -> String {
        self.to_string()
    }

    fn get_default_alignment(&self) -> Alignment;

    /// If the role disguises their alignment, this method should be overwritten
    fn get_alignment(&self) -> Alignment {
        self.get_default_alignment()
    }

    /// This gets the true character type of the player. This is what should be used by the state
    /// for setup, logging, etc...
    fn get_true_character_type(&self) -> CharacterType;

    /// Should be overwritten when a role wants to mask their default character type as
    /// another character. This is the method that should be used by role abilities
    fn get_character_type(&self) -> CharacterType {
        self.get_true_character_type()
    }

    /// By default, most roles are not win conditions. This should only be overwritten if they are
    fn is_win_condition(&self) -> bool {
        false
    }

    /// A kill condition for this role
    /// # Return
    ///     * Returns a Option<bool> based on whether or not the role overwrites the default kill behavior of
    ///     the player. By default, it does not do anything and returns None. A true indicates the
    ///     player should die.
    fn kill(&self, _attacking_player_index: PlayerIndex, _state: &State) -> Option<bool> {
        return None;
    }

    /// A execute condition for this role
    /// # Return
    ///     * Returns a Option<bool> based on whether or not the role overwrites the default kill behavior of
    ///     the player. By default, it does not do anything and returns None. A true indicates the
    ///     player should die.
    fn execute(&self, _state: &State) -> Option<bool> {
        return None;
    }

    /// If the role being in the game affects character type counts, overwrite this method. The
    /// CharacterTypeCounts returned from this function will be added to the ones currently in the
    /// game
    fn initialization_effect(&self) -> Option<CharacterTypeCounts> {
        None
    }

    // TODO: These has blah blah blah ability may not be necessary
    // Instead of this implement a function to get the order that night

    /// If the role has an ability that acts during the setup phase, this method should be overwritten and return
    /// Some(order) in which the ability acts. This is NOT the same as affecting the character
    /// counts in the game. That is the initialization phase.
    fn setup_order(&self) -> Option<usize> {
        None
    }
    /// If the role has an ability that acts during the setup phase, this method should be overwritten and
    /// resolve the setup ability. This is NOT the same as affecting the character
    /// counts in the game. That is the initialization phase.
    fn setup_ability(
        &self,
        _player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        None
    }

    /// If the role has an ability that acts during night one, this method should be overwritten and return
    /// Some(order) in which the ability acts
    fn night_one_order(&self) -> Option<usize> {
        None
    }
    /// If the role has an ability that acts during night one, this method should be overwritten and resolve the night 1 ability
    fn night_one_ability(
        &self,
        _player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        None
    }

    /// If the role has an ability that acts during the night (not including night one), this method should be overwritten and return
    /// Some(order) in which the ability acts
    fn night_order(&self) -> Option<usize> {
        None
    }
    /// If the role has an ability that acts during the night (not including night one), this method should be overwritten and resolve the night ability
    fn night_ability(
        &self,
        _player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        None
    }

    /// If the role has an ability that acts during the day (not including night one), this method should be overwritten and indicate which part(s) of the day this ability can be triggered during
    fn has_day_ability(&self) -> Option<Step> {
        None
    }
    /// If the role has an ability that acts during the day (not including night one), this method should be overwritten and resolve the day ability
    fn day_ability(
        &self,
        _player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        None
    }
}

impl Roles {
    pub(crate) fn get_default_alignment(&self) -> Alignment {
        match self.get_type() {
            CharacterType::Minion | CharacterType::Demon => Alignment::Evil,
            _ => Alignment::Good,
        }
    }

    pub(crate) fn get_type(&self) -> CharacterType {
        match *self {
            Roles::Investigator
            | Roles::Empath
            | Roles::Gossip
            | Roles::Innkeeper
            | Roles::Washerwoman
            | Roles::Librarian
            | Roles::Chef
            | Roles::Fortuneteller
            | Roles::Undertaker
            | Roles::Virgin
            | Roles::Soldier
            | Roles::Slayer
            | Roles::Mayor
            | Roles::Monk
            | Roles::Ravenkeeper => CharacterType::Townsfolk,
            Roles::Drunk | Roles::Saint | Roles::Butler | Roles::Recluse => CharacterType::Outsider,
            Roles::Spy | Roles::Baron | Roles::Scarletwoman | Roles::Poisoner => {
                CharacterType::Minion
            }
            Roles::Imp => CharacterType::Demon,
        }
    }

    pub(crate) fn is_win_condition(&self) -> bool {
        matches!(self.get_type(), CharacterType::Demon)
    }
}

// Role Modules
// TODO: Make these dynamically loaded based off what files are available
// Could be useful for custom roles

pub(crate) mod demons;
pub(crate) mod minions;
pub(crate) mod outsiders;
pub(crate) mod townsfolk;
// pub(crate) mod empath;
// pub(crate) mod fortuneteller;
// pub(crate) mod undertaker;
// pub(crate) mod monk;
// pub(crate) mod ravenkeeper;
// pub(crate) mod virgin;
// pub(crate) mod slayer;
// pub(crate) mod soldier;
// pub(crate) mod mayor;
// pub(crate) mod butler;
// pub(crate) mod drunk;
// pub(crate) mod recluse;
// pub(crate) mod saint;
// pub(crate) mod poisoner;
// pub(crate) mod spy;
// pub(crate) mod scarletwoman;
// pub(crate) mod baron;
// pub(crate) mod imp;
//
