use enum_dispatch::enum_dispatch;
use leptos::prelude::StorageAccess;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::sync::Arc;

use crate::engine::change_request::{ChangeArgs, ChangeResult};
use crate::engine::player::roles::demons::Imp;
use crate::engine::player::roles::minions::{Baron, Poisoner, ScarletWoman, Spy};
use crate::engine::player::roles::outsiders::{Butler, Drunk, Recluse, Saint};
use crate::{
    engine::{
        change_request::ChangeRequestBuilder,
        player::{roles::townsfolk::*, *},
        state::{PlayerIndex, State},
    },
    initialization::CharacterTypeCounts,
};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum RoleNames {
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
    ScarletWoman,
    Poisoner,
    Imp,
}

#[enum_dispatch(Role)]
#[derive(Clone)]
pub(crate) enum Roles {
    // Normal Roles
    Investigator,
    Empath,
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
    ScarletWoman,
    Poisoner,
    Imp,
}

impl Roles {
    fn new(role_name: &RoleNames) -> Self {
        match role_name {
            RoleNames::Investigator => Self::Investigator(Investigator::default()),
            RoleNames::Empath => Self::Empath(Empath::default()),
            // RoleNames::Gossip => todo!(),
            // RoleNames::Innkeeper => todo!(),
            RoleNames::Washerwoman => Self::Washerwoman(Washerwoman::default()),
            RoleNames::Librarian => Self::Librarian(Librarian::default()),
            RoleNames::Chef => Self::Chef(Chef::default()),
            RoleNames::Fortuneteller => Self::Fortuneteller(Fortuneteller::default()),
            RoleNames::Undertaker => Self::Undertaker(Undertaker::default()),
            RoleNames::Virgin => Self::Virgin(Virgin::default()),
            RoleNames::Soldier => Self::Soldier(Soldier::default()),
            RoleNames::Slayer => Self::Slayer(Slayer::default()),
            RoleNames::Mayor => Self::Mayor(Mayor::default()),
            RoleNames::Monk => Self::Monk(Monk::default()),
            RoleNames::Ravenkeeper => Self::Ravenkeeper(Ravenkeeper::default()),
            RoleNames::Drunk => Self::Drunk(Drunk::default()),
            RoleNames::Saint => Self::Saint(Saint::default()),
            RoleNames::Butler => Self::Butler(Butler::default()),
            RoleNames::Recluse => Self::Recluse(Recluse::default()),
            RoleNames::Spy => Self::Spy(Spy::default()),
            RoleNames::Baron => Self::Baron(Baron::default()),
            RoleNames::ScarletWoman => Self::ScarletWoman(ScarletWoman::default()),
            RoleNames::Poisoner => Self::Poisoner(Poisoner::default()),
            RoleNames::Imp => Self::Imp(Imp::default()),
            _ => todo!(),
        }
    }

    fn to_role_name(&self) -> RoleNames {
        match self {
            Roles::Investigator(_) => RoleNames::Investigator,
            Roles::Empath(_) => RoleNames::Empath,
            Roles::Washerwoman(_) => RoleNames::Washerwoman,
            Roles::Librarian(_) => RoleNames::Librarian,
            Roles::Chef(_) => RoleNames::Chef,
            Roles::Fortuneteller(_) => RoleNames::Fortuneteller,
            Roles::Undertaker(_) => RoleNames::Undertaker,
            Roles::Virgin(_) => RoleNames::Virgin,
            Roles::Soldier(_) => RoleNames::Soldier,
            Roles::Slayer(_) => RoleNames::Slayer,
            Roles::Mayor(_) => RoleNames::Mayor,
            Roles::Monk(_) => RoleNames::Monk,
            Roles::Ravenkeeper(_) => RoleNames::Ravenkeeper,
            Roles::Drunk(_) => RoleNames::Drunk,
            Roles::Saint(_) => RoleNames::Saint,
            Roles::Butler(_) => RoleNames::Butler,
            Roles::Recluse(_) => RoleNames::Recluse,
            Roles::Spy(_) => RoleNames::Spy,
            Roles::Baron(_) => RoleNames::Baron,
            Roles::ScarletWoman(_) => RoleNames::ScarletWoman,
            Roles::Poisoner(_) => RoleNames::Poisoner,
            Roles::Imp(_) => RoleNames::Imp,
        }
    }
}

impl Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.to_role_name(), f)
    }
}

impl Display for RoleNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleNames::Investigator => write!(f, "Investigator"),
            RoleNames::Empath => write!(f, "Empath"),
            RoleNames::Gossip => write!(f, "Gossip"),
            RoleNames::Innkeeper => write!(f, "Innkeeper"),
            RoleNames::Washerwoman => write!(f, "Washerwoman"),

            RoleNames::Librarian => write!(f, "Librarian"),
            RoleNames::Chef => write!(f, "Chef"),
            RoleNames::Fortuneteller => write!(f, "Fortuneteller"),
            RoleNames::Undertaker => write!(f, "Undertaker"),
            RoleNames::Virgin => write!(f, "Virgin"),
            RoleNames::Soldier => write!(f, "Soldier"),
            RoleNames::Slayer => write!(f, "Slayer"),
            RoleNames::Mayor => write!(f, "Mayor"),
            RoleNames::Monk => write!(f, "Monk"),
            RoleNames::Ravenkeeper => write!(f, "Ravenkeeper"),
            RoleNames::Drunk => write!(f, "Drunk"),
            RoleNames::Saint => write!(f, "Saint"),
            RoleNames::Butler => write!(f, "Butler"),
            RoleNames::Recluse => write!(f, "Recluse"),
            RoleNames::Spy => write!(f, "Spy"),
            RoleNames::Baron => write!(f, "Baron"),
            RoleNames::ScarletWoman => write!(f, "Scarletwoman"),
            RoleNames::Poisoner => write!(f, "Poisoner"),
            RoleNames::Imp => write!(f, "Imp"),
        }
    }
}

#[enum_dispatch]
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

    fn initialize(&self, _player_index: PlayerIndex, _state: &mut State) {}

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
    ) -> Option<ChangeRequestBuilder> {
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
    ) -> Option<ChangeRequestBuilder> {
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
    ) -> Option<ChangeRequestBuilder> {
        None
    }

    /// If the role has an ability that acts during the day (not including night one), this method should be overwritten and indicate which part(s) of the day this ability can be triggered during
    fn has_day_ability(&self) -> bool {
        false
    }
    /// If the role has an ability that acts during the day (not including night one), this method should be overwritten and resolve the day ability
    fn day_ability(
        &self,
        _player_index: PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequestBuilder> {
        None
    }
}

impl RoleNames {
    pub(crate) fn convert(&self) -> Roles {
        // TODO: Make classes to roles and resolve them here
        Roles::new(self)
    }

    pub(crate) fn get_default_alignment(&self) -> Alignment {
        match self.get_type() {
            CharacterType::Minion | CharacterType::Demon => Alignment::Evil,
            _ => Alignment::Good,
        }
    }

    pub(crate) fn get_type(&self) -> CharacterType {
        match *self {
            RoleNames::Investigator
            | RoleNames::Empath
            | RoleNames::Gossip
            | RoleNames::Innkeeper
            | RoleNames::Washerwoman
            | RoleNames::Librarian
            | RoleNames::Chef
            | RoleNames::Fortuneteller
            | RoleNames::Undertaker
            | RoleNames::Virgin
            | RoleNames::Soldier
            | RoleNames::Slayer
            | RoleNames::Mayor
            | RoleNames::Monk
            | RoleNames::Ravenkeeper => CharacterType::Townsfolk,
            RoleNames::Drunk | RoleNames::Saint | RoleNames::Butler | RoleNames::Recluse => {
                CharacterType::Outsider
            }
            RoleNames::Spy | RoleNames::Baron | RoleNames::ScarletWoman | RoleNames::Poisoner => {
                CharacterType::Minion
            }
            RoleNames::Imp => CharacterType::Demon,
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
