use reactive_stores::Store;
use serde_derive::{Deserialize, Serialize};
use std::{fmt::Display, rc::Rc};

use crate::engine::{
    change_request::ChangeRequest,
    state::{PlayerIndex, State},
};

// TODO: Roles should no longer be enums
// A role should be a trait, each "role" should implement the role trait
// Roles could also be enums i guess?, but that's cringe, let's try this on a different branch
//
// #[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// #[serde(rename_all = "lowercase")]
// pub(crate) enum Role {
//     // Special Roles that are in every game
//     // DEMON,
//     // MINION,
//     // Normal Roles
//     Investigator,
//     Empath,
//     Gossip,
//     Innkeeper,
//     Washerwoman,
//     Librarian,
//     Chef,
//     Fortuneteller,
//     Undertaker,
//     Virgin,
//     Soldier,
//     Slayer,
//     Mayor,
//     Monk,
//     Ravenkeeper,
//     Drunk,
//     Saint,
//     Butler,
//     Recluse,
//     Spy,
//     Baron,
//     Scarletwoman,
//     Poisoner,
//     Imp,
// }
//
// impl Display for Role {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Role::Investigator => write!(f, "Investigator"),
//             Role::Empath => write!(f, "Empath"),
//             Role::Gossip => write!(f, "Gossip"),
//             Role::Innkeeper => write!(f, "Innkeeper"),
//             Role::Washerwoman => write!(f, "Washerwoman"),
//             Role::Librarian => write!(f, "Librarian"),
//             Role::Chef => write!(f, "Chef"),
//             Role::Fortuneteller => write!(f, "Fortuneteller"),
//             Role::Undertaker => write!(f, "Undertaker"),
//             Role::Virgin => write!(f, "Virgin"),
//             Role::Soldier => write!(f, "Soldier"),
//             Role::Slayer => write!(f, "Slayer"),
//             Role::Mayor => write!(f, "Mayor"),
//             Role::Monk => write!(f, "Monk"),
//             Role::Ravenkeeper => write!(f, "Ravenkeeper"),
//             Role::Drunk => write!(f, "Drunk"),
//             Role::Saint => write!(f, "Saint"),
//             Role::Butler => write!(f, "Butler"),
//             Role::Recluse => write!(f, "Recluse"),
//             Role::Spy => write!(f, "Spy"),
//             Role::Baron => write!(f, "Baron"),
//             Role::Scarletwoman => write!(f, "Scarletwoman"),
//             Role::Poisoner => write!(f, "Poisoner"),
//             Role::Imp => write!(f, "Imp"),
//         }
//     }
// }

pub(crate) trait Role: Display {
    fn name(&self) -> String;

    fn get_default_alignment(&self) -> Alignment;

    fn get_type(&self) -> CharacterType;

    fn is_win_condition(&self) -> bool;

    /// A kill condition for this role
    /// # Return
    /// * Returns a Option<bool> based on whether or not the role overwrites the default kill behavior of
    /// the player. By default, it does not do anything and returns None. A true indicates the
    /// player should die.
    fn kill(&self) -> Option<bool> {
        return None;
    }

    // TODO: These has blah blah blah ability may not be necessary
    // Instead of this implement a function to get the order that night
    fn setup_order(&self) -> Option<usize> {
        None
    }
    fn setup_ability(&self, player_index: PlayerIndex, state: &State) -> Vec<ChangeRequest>;

    fn night_1_order(&self) -> Option<usize> {
        None
    }
    fn night_1_ability(&self, player_index: PlayerIndex, state: &State) -> Vec<ChangeRequest>;

    fn night_order(&self) -> Option<usize> {
        None
    }
    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Vec<ChangeRequest>;

    fn has_day_ability(&self) -> bool {
        false
    }
    fn day_ability(&self, player_index: PlayerIndex, state: &State) -> Vec<ChangeRequest>;
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

// TODO: How am I actually going to do this
// Players should have default behavior
// Wait what is our problem right now
// The code isn't scalable because it isn't modular
// The state should handle interactions between players and the interface but ultimately players
// should manage their own state, and the state should only tell them how to modify their state
// Thus players should have default behavior that lets them modify this state, but this behavior
// should change based on their role and what status effects they have inflicted
#[derive(Store, Clone)]
pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) role: Rc<dyn Role>,
    // Order should be implemented through external array
    pub(crate) dead: bool,
    pub(crate) ability_active: bool, // WARNING: Not too happy about this implementation, might want to make
    // it cleaner
    pub(crate) ghost_vote: bool,
    // it cleaner
    pub(crate) alignment: Alignment,
    // TODO: pub(crate) effects: Vec<Box<dyn StatusEffect>>,
}

impl Player {
    pub(crate) fn new(name: String, role: Rc<dyn Role>) -> Self {
        let alignment = role.get_default_alignment();
        Self {
            name,
            role,
            ghost_vote: true,
            ability_active: true,
            dead: false,
            alignment,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player {}\n\tRole: {:?}\n
                \tDead?: Not Yet Implemented\n
                \t Statuses: Not yet implemented \n
                \tHas Ghost Vote?: {}\n",
            self.name, self.dead, self.ghost_vote
        )
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.role.name() == other.role.name()
            && self.dead == other.dead
            && self.ability_active == other.ability_active
            && self.ghost_vote == other.ghost_vote
            && self.alignment == other.alignment
    }
}
