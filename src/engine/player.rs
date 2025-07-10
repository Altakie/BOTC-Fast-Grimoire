use reactive_stores::Store;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Role {
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

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Investigator => write!(f, "Investigator"),
            Role::Empath => write!(f, "Empath"),
            Role::Gossip => write!(f, "Gossip"),
            Role::Innkeeper => write!(f, "Innkeeper"),
            Role::Washerwoman => write!(f, "Washerwoman"),
            Role::Librarian => write!(f, "Librarian"),
            Role::Chef => write!(f, "Chef"),
            Role::Fortuneteller => write!(f, "Fortuneteller"),
            Role::Undertaker => write!(f, "Undertaker"),
            Role::Virgin => write!(f, "Virgin"),
            Role::Soldier => write!(f, "Soldier"),
            Role::Slayer => write!(f, "Slayer"),
            Role::Mayor => write!(f, "Mayor"),
            Role::Monk => write!(f, "Monk"),
            Role::Ravenkeeper => write!(f, "Ravenkeeper"),
            Role::Drunk => write!(f, "Drunk"),
            Role::Saint => write!(f, "Saint"),
            Role::Butler => write!(f, "Butler"),
            Role::Recluse => write!(f, "Recluse"),
            Role::Spy => write!(f, "Spy"),
            Role::Baron => write!(f, "Baron"),
            Role::Scarletwoman => write!(f, "Scarletwoman"),
            Role::Poisoner => write!(f, "Poisoner"),
            Role::Imp => write!(f, "Imp"),
        }
    }
}
impl Role {
    pub(crate) fn get_default_alignment(&self) -> Alignment {
        match self.get_type() {
            CharacterType::Minion | CharacterType::Demon => Alignment::Evil,
            _ => Alignment::Good,
        }
    }

    pub(crate) fn get_type(&self) -> CharacterType {
        match *self {
            Role::Investigator
            | Role::Empath
            | Role::Gossip
            | Role::Innkeeper
            | Role::Washerwoman
            | Role::Librarian
            | Role::Chef
            | Role::Fortuneteller
            | Role::Undertaker
            | Role::Virgin
            | Role::Soldier
            | Role::Slayer
            | Role::Mayor
            | Role::Monk
            | Role::Ravenkeeper => CharacterType::Townsfolk,
            Role::Drunk | Role::Saint | Role::Butler | Role::Recluse => CharacterType::Outsider,
            Role::Spy | Role::Baron | Role::Scarletwoman | Role::Poisoner => CharacterType::Minion,
            Role::Imp => CharacterType::Demon,
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

#[derive(Debug, PartialEq, Eq, Clone, Store)]
pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) role: Role,
    // Order should be implemented through external array
    pub(crate) dead: bool,
    pub(crate) ability_active: bool, // WARNING: Not too happy about this implementation, might want to make
    // it cleaner
    pub(crate) ghost_vote: bool,
    // it cleaner
    pub(crate) alignment: Alignment,
}

impl Player {
    pub(crate) fn new(name: String, role: Role) -> Self {
        Self {
            name,
            role,
            ghost_vote: true,
            ability_active: true,
            dead: false,
            alignment: role.get_default_alignment(),
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
