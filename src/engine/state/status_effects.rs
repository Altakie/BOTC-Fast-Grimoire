use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::sync::Arc;

use leptos::leptos_dom::logging::{console_error, console_log};

use crate::engine::player::Player;
use crate::engine::{
    player::PlayerBehaviors,
    state::{PlayerIndex, State},
};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub(crate) enum CleanupPhase {
    Dusk,
    Dawn,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatusType {
    // General Effects
    Poisoned,
    Drunk,

    // Role Specific Effects
    WasherwomanTownsfolk,
    WasherwomanWrong,
    LibrarianOutsider,
    LibrarianWrong,
    InvestigatorMinion,
    InvestigatorWrong,
    ButlerMaster,
    FortuneTellerRedHerring,
    DemonProtected,
}

impl StatusType {
    pub fn name(&self) -> String {
        self.to_string()
    }
}

impl Display for StatusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusType::Drunk => f.write_str("Drunk"),
            // StatusType::Mad => f.write_str("Mad"),
            StatusType::Poisoned => f.write_str("Poisoned"),
            StatusType::DemonProtected => f.write_str("Demon Protected"),
            // StatusType::NightProtected => f.write_str("Night Protected"),
            // StatusType::DeathProtected => f.write_str("Death Protected"),
            // StatusType::NoAbility => f.write_str("No Ability"),
            StatusType::ButlerMaster => f.write_str("Butler Master"),
            // StatusType::AppearsGood => f.write_str("Appears Good"),
            // StatusType::AppearsEvil => f.write_str("Appears Evil"),
            // StatusType::MayorBounceKill => f.write_str("Mayor Bounce Kill"),
            // StatusType::OtherRoleAbility(role) => write!(f, "{}'s Ability", role.to_string()),
            StatusType::FortuneTellerRedHerring => f.write_str("Fortune Teller Red Herring"),
            StatusType::WasherwomanTownsfolk => f.write_str("Washerwoman Townsfolk"),
            StatusType::WasherwomanWrong => f.write_str("Washerwoman Wrong"),
            StatusType::LibrarianOutsider => f.write_str("Librarian Outsider"),
            StatusType::LibrarianWrong => f.write_str("Librarian Wrong"),
            StatusType::InvestigatorMinion => f.write_str("Investigator Minion"),
            StatusType::InvestigatorWrong => f.write_str("Investigator Wrong"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct StatusEffect {
    // pub(crate) status_type: StatusEffects,
    pub(crate) status_type: StatusType,
    pub(crate) source_player_index: PlayerIndex,
    pub(crate) cleanup_phase: Option<CleanupPhase>,
    pub(crate) behavior_types: Option<Vec<PlayerBehaviors>>,
}

impl StatusEffect {
    pub(crate) fn new(
        status_type: StatusType,
        source_player_index: PlayerIndex,
        cleanup_phase: Option<CleanupPhase>,
    ) -> Self {
        let behavior_types = match status_type {
            StatusType::Poisoned | StatusType::Drunk => Some(vec![
                PlayerBehaviors::DayAbility,
                PlayerBehaviors::NightOneAbility,
                PlayerBehaviors::NightAbility,
            ]),
            StatusType::DemonProtected => Some(vec![PlayerBehaviors::Kill]),
            _ => None,
        };

        Self {
            status_type,
            source_player_index,
            cleanup_phase,
            behavior_types,
        }
    }
}

impl Debug for StatusEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusEffect")
            .field("status_type", &self.status_type.name())
            .field("source_player_index", &self.source_player_index)
            .finish()
    }
}

impl Display for StatusEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status_type)
    }
}

impl PartialEq for StatusEffect {
    fn eq(&self, other: &Self) -> bool {
        self.status_type.name() == other.status_type.name()
            && self.source_player_index == other.source_player_index
    }
}

// impl State {
//     pub(crate) fn add_status(
//         &mut self,
//         status_type: StatusType,
//         source_player_index: PlayerIndex,
//         affected_player_index: PlayerIndex,
//     ) {
//         let new_status = StatusEffect::new(
//             status_type,
//             source_player_index,
//             self.players[source_player_index].role,
//             affected_player_index,
//         );
//         self.status_effects.push(new_status);
//     }
//
//     pub(crate) fn remove_status(
//         &mut self,
//         status_type: StatusType,
//         source_player_index: PlayerIndex,
//         affected_player_index: PlayerIndex,
//     ) {
//         let index = self
//             .status_effects
//             .iter()
//             .position(|s| {
//                 s.status_type == status_type
//                     && s.source_player_index == source_player_index
//                     && s.affected_player_index == affected_player_index
//             })
//             .expect("Tried to remove status effect not in game");
//         self.status_effects.remove(index);
//     }
//
//     pub(crate) fn get_inflicted_statuses(
//         &self,
//         source_player_index: PlayerIndex,
//     ) -> Vec<StatusEffect> {
//         self.status_effects
//             .iter()
//             .filter(|s| s.source_player_index == source_player_index)
//             .cloned()
//             .collect()
//     }
//
//     pub(crate) fn get_afflicted_statuses(
//         &self,
//         affected_player_index: PlayerIndex,
//     ) -> Vec<StatusEffect> {
//         self.status_effects
//             .iter()
//             .filter(|s| s.affected_player_index == affected_player_index)
//             .cloned()
//             .collect()
//     }
// }
