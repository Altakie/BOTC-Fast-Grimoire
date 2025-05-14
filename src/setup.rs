use std::{fmt::Display, isize};

use crate::game::{CharacterType, Role};

use serde_derive::{Deserialize, Serialize};

// -- Script Structures --
#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    id: String,
    author: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ScriptEntry {
    Metadata(Metadata),
    Role(Role),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ScriptJson(Vec<ScriptEntry>);

#[derive(Clone)]
pub(crate) struct Script {
    pub(crate) roles: Vec<Role>,
}

impl Script {
    pub(crate) fn new_from_json(json: ScriptJson) -> Self {
        let mut roles: Vec<Role> = vec![];
        for entry in json.0 {
            match entry {
                ScriptEntry::Metadata(metadata) => (),
                ScriptEntry::Role(role) => roles.push(role),
            }
        }

        Self { roles }
    }
}

// -- Setup Structures --
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CharacterTypeCounts {
    pub(crate) townsfolk: isize,
    pub(crate) outsiders: isize,
    pub(crate) minions: isize,
    pub(crate) demons: isize,
}

impl CharacterTypeCounts {
    pub(crate) fn new(num_players: usize) -> Result<Self, ()> {
        match num_players {
            0..=4 => Err(()),
            5 => Ok(Self {
                townsfolk: 3,
                outsiders: 0,
                minions: 1,
                demons: 1,
            }),
            6 => Ok(Self {
                townsfolk: 3,
                outsiders: 1,
                minions: 1,
                demons: 1,
            }),
            7 => Ok(Self {
                townsfolk: 5,
                outsiders: 0,
                minions: 1,
                demons: 1,
            }),
            8 => Ok(Self {
                townsfolk: 5,
                outsiders: 1,
                minions: 1,
                demons: 1,
            }),
            9 => Ok(Self {
                townsfolk: 5,
                outsiders: 2,
                minions: 1,
                demons: 1,
            }),
            10 => Ok(Self {
                townsfolk: 7,
                outsiders: 0,
                minions: 2,
                demons: 1,
            }),
            11 => Ok(Self {
                townsfolk: 7,
                outsiders: 1,
                minions: 2,
                demons: 1,
            }),
            12 => Ok(Self {
                townsfolk: 7,
                outsiders: 2,
                minions: 2,
                demons: 1,
            }),
            13 => Ok(Self {
                townsfolk: 9,
                outsiders: 0,
                minions: 3,
                demons: 1,
            }),
            14 => Ok(Self {
                townsfolk: 9,
                outsiders: 1,
                minions: 3,
                demons: 1,
            }),
            15 => Ok(Self {
                townsfolk: 9,
                outsiders: 2,
                minions: 3,
                demons: 1,
            }),
            _ => Err(()),
        }
    }

    pub(crate) fn new_empty() -> Self {
        Self {
            townsfolk: 0,
            outsiders: 0,
            minions: 0,
            demons: 0,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.townsfolk == 0 && self.outsiders == 0 && self.minions == 0 && self.demons == 0
    }

    pub(crate) fn diff(&self, other: &Self) -> Self {
        Self {
            townsfolk: self.townsfolk - other.townsfolk,
            outsiders: self.outsiders - other.outsiders,
            minions: self.minions - other.minions,
            demons: self.demons - other.demons,
        }
    }

    pub(crate) fn on_choose(&mut self, role: Role) {
        self.role_effects(role, 1);
    }

    pub(crate) fn on_remove(&mut self, role: Role) {
        self.role_effects(role, -1);
    }

    fn role_effects(&mut self, role: Role, multiplier: isize) {
        match role {
            Role::Baron => {
                let num = 2 * multiplier;
                self.outsiders += num;
                self.townsfolk -= num;
            }
            _ => (),
        }
    }

    pub(crate) fn set_count(&mut self, character_type: CharacterType, count: isize) {
        match character_type {
            CharacterType::Townsfolk => self.townsfolk = count,
            CharacterType::Outsider => self.outsiders = count,
            CharacterType::Minion => self.minions = count,
            CharacterType::Demon => self.demons = count,
        }
    }

    pub(crate) fn get_count(&self, character_type: CharacterType) -> isize {
        match character_type {
            CharacterType::Townsfolk => self.townsfolk,
            CharacterType::Outsider => self.outsiders,
            CharacterType::Minion => self.minions,
            CharacterType::Demon => self.demons,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counts_updated_on_choose() {
        // NOTE: Should have a test for all roles that modify character_types

        // Baron
        let character_counts = CharacterTypeCounts::new(5).unwrap();
        todo!()
    }
}
