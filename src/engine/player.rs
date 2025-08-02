use reactive_stores::Store;
use std::{fmt::Debug, sync::Arc};

use crate::engine::{
    change_request::ChangeRequest,
    player::roles::Role,
    state::{PlayerIndex, State, Step, status_effects::StatusEffect},
};

pub(crate) mod roles;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Alignment {
    Good,
    Evil,
    Any,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CharacterType {
    Townsfolk,
    Outsider,
    Minion,
    Demon,
    Any,
}

#[derive(Clone, Store)]
pub(crate) struct Player {
    pub(crate) name: String,
    // TODO: Might need to be Arc instead of rc
    pub(crate) role: Arc<dyn Role>,
    // Order should be implemented through external array
    pub(crate) dead: bool,
    pub(crate) ghost_vote: bool,
    // it cleaner
    pub(crate) alignment: Alignment,
    pub(crate) status_effects: Vec<StatusEffect>,
}

impl Player {
    pub(crate) fn new(name: String, role: Arc<dyn Role>) -> Self {
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

    pub(crate) fn get_statuses(&self) -> &Vec<StatusEffect> {
        &self.status_effects
    }

    pub(crate) fn add_status(&mut self, status: StatusEffect) {
        self.status_effects.push(status);
    }

    pub(crate) fn remove_status(&mut self, status_name: &str) -> Option<StatusEffect> {
        let pos = self
            .status_effects
            .iter()
            .position(|se| se.status_type.name() == status_name);

        if let Some(pos) = pos {
            return Some(self.status_effects.remove(pos));
        }
        return None;
    }

    // Player Behaviors

    /// Default behavior is that the player dies. If the player does not die, it should be because
    /// of their role or status effects
    pub(crate) fn kill(&mut self, attacking_player_index: PlayerIndex, state: &State) {
        // Status Effects
        // Basically go through each status, see if any prevent the player from dying
        // If any do, prevent the player from dying
        let mut dead = true;
        for status_effect in self.status_effects.iter() {
            if matches!(
                status_effect.status_type.behavior_type(),
                Some(PlayerBehaviors::Kill)
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
        if let Some(dead) = self.role.kill(attacking_player_index, state) {
            self.dead = dead;
            return;
        }

        // Default behavior
        self.dead = true;
    }

    /// Default behavior is that the player dies. If the player does not die, it should be because
    /// of their role or status effects
    pub(crate) fn execute(&mut self, state: &State) {
        // Status Effects
        // Basically go through each status, see if any prevent the player from dying
        // If any do, prevent the player from dying
        let mut dead = true;
        for status_effect in self.status_effects.iter() {
            if matches!(
                status_effect.status_type.behavior_type(),
                Some(PlayerBehaviors::Execute)
            ) {
                if let Some(false) = status_effect.status_type.execute(state) {
                    dead = false;
                }
            }
        }

        if !dead {
            return;
        }

        if let Some(dead) = self.role.execute(state) {
            self.dead = dead;
            return;
        }
        // Default Behavior
        self.dead = true;
    }

    pub(crate) fn get_alignment(&self) -> Alignment {
        // Roles
        if let Some(alignment) = self.role.get_alignment() {
            return alignment;
        }

        return self.alignment;
    }

    pub(crate) fn get_character_type(&self) -> CharacterType {
        self.role.get_character_type()
    }

    // TODO: Figure out if you want to implement this
    pub(crate) fn nominate(&self) {}

    // TODO: Figure out if you want to implement this
    pub(crate) fn vote(&self) {}

    pub(crate) fn setup_order(&self) -> Option<usize> {
        self.role.setup_order()
    }

    pub(crate) fn setup_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.role.setup_ability(player_index, state)
    }

    pub(crate) fn night_one_order(&self) -> Option<usize> {
        self.role.night_one_order()
    }

    pub(crate) fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.role.night_one_ability(player_index, state)
    }

    /// If the role has an ability that acts during the night (not including night one), this method should be overwritten and return
    /// Some(order) in which the ability acts
    fn night_order(&self) -> Option<usize> {
        self.role.night_order()
    }
    /// If the role has an ability that acts during the night (not including night one), this method should be overwritten and resolve the night ability
    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.role.night_ability(player_index, state)
    }

    /// If the role has an ability that acts during the day (not including night one), this method should be overwritten and indicate which part(s) of the day this ability can be triggered during
    fn has_day_ability(&self) -> Option<Step> {
        self.role.has_day_ability()
    }
    /// If the role has an ability that acts during the day (not including night one), this method should be overwritten and resolve the day ability
    fn day_ability(&self, player_index: PlayerIndex, state: &State) -> Option<Vec<ChangeRequest>> {
        self.role.day_ability(player_index, state)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum PlayerBehaviors {
    Kill,
    Execute,
    Nominate,
    Vote,
    ShowAlignment,
    SetupAbility,
    NightOneAbility,
    NightAbility,
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
