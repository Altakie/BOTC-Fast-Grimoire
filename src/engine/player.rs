use reactive_stores::Store;
use std::fmt::{Debug, Display};

use crate::engine::{
    change_request::{ChangeArgs, ChangeRequest, ChangeResult, StateChangeFuncPtr},
    player::roles::RolePtr,
    state::{
        PlayerIndex, State,
        status_effects::{CleanupPhase, StatusEffect},
    },
};

pub(crate) mod roles;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Alignment {
    Good,
    Evil,
    Any,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Alignment::Good => "Good",
            Alignment::Evil => "Evil",
            Alignment::Any => "Any",
        };

        f.write_str(str)
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CharacterType {
    Townsfolk,
    Outsider,
    Minion,
    Demon,
    Any,
}

impl Display for CharacterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            CharacterType::Townsfolk => "Townsfolk",
            CharacterType::Outsider => "Outsider",
            CharacterType::Minion => "Minion",
            CharacterType::Demon => "Demon",
            CharacterType::Any => "Any",
        };
        f.write_str(string)
    }
}

#[derive(Clone, Store)]
pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) role: RolePtr,
    pub(crate) dead: bool,
    pub(crate) ghost_vote: bool,
    pub(crate) alignment: Alignment,
    pub(crate) status_effects: Vec<StatusEffect>,
}

impl Player {
    pub(crate) fn new(name: String, role: RolePtr) -> Self {
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

    pub(crate) fn remove_players_statuses(&mut self, source_player_index: PlayerIndex) {
        self.status_effects
            .retain(|s| s.source_player_index != source_player_index);
    }

    pub(crate) fn cleanup_statuses(&mut self, cleanup_phase: CleanupPhase) {
        self.status_effects
            .retain(|s| s.cleanup_phase != Some(cleanup_phase));
    }

    // Player Behaviors

    /// Default behavior is that the player dies. If the player does not die, it should be because
    /// of their role or status effects.
    pub(crate) fn kill(
        &mut self,
        attacking_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
        state: &State,
    ) -> ChangeResult {
        // Status Effects
        // Basically go through each status, see if any prevent the player from dying
        // If any do, prevent the player from dying
        for status_effect in self.status_effects.iter() {
            if status_effect
                .status_type
                .behavior_type()
                .is_some_and(|behaviors| {
                    behaviors
                        .iter()
                        .any(|behavior| matches!(behavior, PlayerBehaviors::Kill))
                })
            {
                if let Some(false) = status_effect.status_type.kill(
                    attacking_player_index,
                    target_player_index,
                    state,
                ) {
                    return Ok(None);
                }
            }
        }

        // Roles
        if let Some(cr) = self
            .role
            .kill(attacking_player_index, target_player_index, state)
        {
            return cr;
        }

        // Default behavior
        self.dead = true;

        Ok(None)
    }

    /// Default behavior is that the player dies. If the player does not die, it should be because
    /// of their role or status effects
    pub(crate) fn execute(&mut self) {
        // Status Effects
        // Basically go through each status, see if any prevent the player from dying
        // If any do, prevent the player from dying
        let mut dead = true;
        for status_effect in self.status_effects.iter() {
            if status_effect
                .status_type
                .behavior_type()
                .is_some_and(|behaviors| {
                    behaviors
                        .iter()
                        .any(|behavior| matches!(behavior, PlayerBehaviors::Execute))
                })
            {
                if let Some(false) = status_effect.status_type.execute() {
                    dead = false;
                }
            }
        }

        if !dead {
            return;
        }

        if let Some(dead) = self.role.execute() {
            self.dead = dead;
            return;
        }
        // Default Behavior
        self.dead = true;
    }

    pub(crate) fn get_alignment(&self) -> Alignment {
        self.role.get_alignment()
    }

    pub(crate) fn get_character_type(&self) -> CharacterType {
        self.role.get_character_type()
    }

    // TODO: Figure out if you want to implement this
    pub(crate) fn nominate(
        &self,
        nominating_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
        state: &mut State,
    ) {
        self.role
            .nominated(nominating_player_index, target_player_index, state);
    }

    // TODO: Figure out if you want to implement this
    pub(crate) fn vote(&self) {}

    pub(crate) fn setup_order(&self) -> Option<usize> {
        self.role.setup_order()
    }

    pub(crate) fn setup_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequest> {
        self.role.setup_ability(player_index, state)
    }

    pub(crate) fn night_one_order(&self) -> Option<usize> {
        self.role.night_one_order()
    }

    pub(crate) fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequest> {
        let cr = self.role.night_one_ability(player_index, state)?;
        // Check for poison or drunk effects
        let status_effect = self.get_statuses().iter().find(|se| {
            se.status_type.behavior_type().is_some_and(|behaviors| {
                behaviors
                    .iter()
                    .any(|behavior| matches!(behavior, PlayerBehaviors::NightAbility))
            })
        });

        if let Some(status_effect) = status_effect {
            // for cr in res.iter_mut() {
            //     cr.state_change_func = None;
            //     cr.description = format!("(*{}*) ", status_effect) + cr.description.as_str();
            // }
            // FIX: This needs apply to all the crs in the chain, not just the first one (although
            // this technically disables the chain so we might need to play around that)
            let new_cr = drunkify(player_index, cr, status_effect.to_string());
            return new_cr.into();
        }

        return cr.into();
    }

    /// If the role has an ability that acts during the night (not including night one), this method should be overwritten and return
    /// Some(order) in which the ability acts
    pub fn night_order(&self) -> Option<usize> {
        self.role.night_order()
    }
    /// If the role has an ability that acts during the night (not including night one), this method should be overwritten and resolve the night ability
    pub fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let cr = self.role.night_ability(player_index, state)?;
        let status_effect = self.get_statuses().iter().find(|se| {
            se.status_type.behavior_type().is_some_and(|behaviors| {
                behaviors
                    .iter()
                    .any(|behavior| matches!(behavior, PlayerBehaviors::NightOneAbility))
            })
        });

        if let Some(status_effect) = status_effect {
            // for cr in res.iter_mut() {
            //     cr.state_change_func = None;
            //     cr.description = format!("(*{}*) ", status_effect) + cr.description.as_str();
            // }
            // FIX: This needs apply to all the crs in the chain, not just the first one (although
            // this technically disables the chain so we might need to play around that)
            let new_cr = drunkify(player_index, cr, status_effect.to_string());
            return new_cr.into();
        }

        return cr.into();
    }

    /// If the role has an ability that acts during the day (not including night one), this method should be overwritten and indicate which part(s) of the day this ability can be triggered during
    pub fn has_day_ability(&self) -> bool {
        self.role.has_day_ability()
    }
    /// If the role has an ability that acts during the day (not including night one), this method should be overwritten and resolve the day ability
    pub fn day_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let cr = self.role.day_ability(player_index, state)?;
        let status_effect = self.get_statuses().iter().find(|se| {
            se.status_type.behavior_type().is_some_and(|behaviors| {
                behaviors
                    .iter()
                    .any(|behavior| matches!(behavior, PlayerBehaviors::DayAbility))
            })
        });

        if let Some(status_effect) = status_effect {
            // for cr in res.iter_mut() {
            //     cr.state_change_func = None;
            //     cr.description = format!("(*{}*) ", status_effect) + cr.description.as_str();
            // }
            // FIX: This needs apply to all the crs in the chain, not just the first one (although
            // this technically disables the chain so we might need to play around that)
            let new_cr = drunkify(player_index, cr, status_effect.to_string());
            return new_cr.into();
        }

        return cr.into();
    }

    pub fn notify(&mut self, args: &ChangeArgs) {
        let role_change = self.role.notify(args);
        if let Some(role_change) = role_change {
            self.role.reassign(role_change);
        }
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
    DayAbility,
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

fn drunkify(
    player_index: PlayerIndex,
    mut change_request: ChangeRequest,
    status_string: String,
) -> ChangeRequest {
    if let Some(state_change_func) = change_request.state_change_func {
        let status_string_clone = status_string.clone();
        let wrapper_func = StateChangeFuncPtr::new(move |state, args| {
            let mut state_copy = state.clone();
            let next_cr = state_change_func(&mut state_copy, args)?;

            let player_role = state_copy.get_player(player_index).role.clone();
            state
                .get_player_mut(player_index)
                .role
                .reassign(player_role);

            if let Some(next_cr) = next_cr {
                let new_state_change =
                    drunkify(player_index, next_cr, status_string_clone.to_owned());
                return new_state_change.into();
            }

            Ok(None)
        });

        change_request.state_change_func = Some(wrapper_func);
    }

    change_request.description =
        format!("(*{}*) ", status_string) + change_request.description.as_str();

    return change_request;
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
