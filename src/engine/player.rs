use reactive_stores::Store;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::engine::state::Step;
use crate::engine::state::status_effects::StatusEffect;
use crate::engine::{
    change_request::ChangeRequest,
    state::{PlayerIndex, State},
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

pub(crate) trait Role {
    fn name(&self) -> String;

    fn get_default_alignment(&self) -> Alignment;

    fn get_type(&self) -> CharacterType;

    /// By default, most roles are not win conditions. This should only be overwritten if they are
    fn is_win_condition(&self, state: &State) -> bool {
        false
    }

    /// A kill condition for this role
    /// # Return
    ///     * Returns a Option<bool> based on whether or not the role overwrites the default kill behavior of
    ///     the player. By default, it does not do anything and returns None. A true indicates the
    ///     player should die.
    fn kill(&self) -> Option<bool> {
        return None;
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
    fn night_1_order(&self) -> Option<usize> {
        None
    }
    /// If the role has an ability that acts during night one, this method should be overwritten and resolve the night 1 ability
    fn night_1_ability(
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

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Alignment {
    Good,
    Evil,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CharacterType {
    Townsfolk,
    Outsider,
    Minion,
    Demon,
}

#[derive(Clone, Store)]
pub(crate) struct Player {
    pub(crate) name: String,
    // TODO: Might need to be Arc instead of rc
    pub(crate) role: Rc<dyn Role>,
    // Order should be implemented through external array
    pub(crate) dead: bool,
    pub(crate) ghost_vote: bool,
    // it cleaner
    pub(crate) alignment: Alignment,
    pub(crate) status_effects: Vec<StatusEffect>,
}

impl Player {
    pub(crate) fn new(name: String, role: Rc<dyn Role>) -> Self {
        let alignment = role.get_default_alignment();
        Self {
            name,
            role: role.clone(),
            ghost_vote: true,
            dead: false,
            alignment,
            status_effects: vec![],
        }
    }

    // Player Behaviors

    /// Default behavior is that the player dies. If the player does not die, it should be because
    /// of their role or status effects
    pub(crate) fn kill(&mut self, attacking_player_index: PlayerIndex, state: &State) {
        // Status Effects
        // Basically go through each status, see if any prevent the player from dying
        // If any do, prevent the player from dying
        let mut dead = true;
        for status_effect in self.status_effects {
            if matches!(
                status_effect.status_type.behavior_type(),
                PlayerBehaviors::Kill
            ) {
                if let Some(false) = status_effect
                    .status_type
                    .kill(attacking_player_index, state)
                {
                    dead = false;
                }
            }
        }

        if !dead {
            return;
        }

        // Roles
        if let Some(dead) = self.role.kill() {
            self.dead = dead;
            return;
        }

        // Default behavior
        self.dead = true;
    }

    /// Default behavior is that the player dies. If the player does not die, it should be because
    /// of their role or status effects
    pub(crate) fn execute(&mut self) {
        // Default Behavior
        self.dead = true;
    }

    pub(crate) fn nominate(&self) {}

    pub(crate) fn vote(&self) {}

    pub(crate) fn use_ability(&self, player_index: PlayerIndex, state: &State) {}
}

#[derive(Clone, Copy)]
pub(crate) enum PlayerBehaviors {
    Kill,
    Execute,
    Nominate,
    UseAbility,
    Vote,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name
            && self.role.name() == other.role.name()
            && self.dead == other.dead
            && self.ghost_vote == other.ghost_vote
            && self.alignment == other.alignment;
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("name", &self.name)
            .field("role", &self.role.name())
            .field("dead", &self.dead)
            .field("ghost_vote", &self.ghost_vote)
            .field("alignment", &self.alignment)
            .finish()
    }
}

// impl Display for Player {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Player {}\n\tRole: {:?}\n
//                 \tDead?: Not Yet Implemented\n
//                 \t Statuses: Not yet implemented \n
//                 \tHas Ghost Vote?: {}\n",
//             self.name, self.dead, self.ghost_vote
//         )
//     }
// }
