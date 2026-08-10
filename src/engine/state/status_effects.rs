use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::sync::Arc;

use tracing::info;

use crate::engine::player::Player;
use crate::engine::state::log;
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

/// Status Effects can either be visual (just for the storyteller) and do nothing or they can
/// overwrite player behaviors
impl State {
    pub(crate) fn add_status(&mut self, status: StatusEffect, target_player_index: PlayerIndex) {
        self.handle_status_added(&status, target_player_index);
        self.get_player_mut(target_player_index).add_status(status);
    }

    fn handle_status_added(&mut self, status: &StatusEffect, target_player_index: PlayerIndex) {
        let mut state = self;
        state.log.log_event(
            log::Event::StatusApplied {
                source_player_index: status.source_player_index,
                target_player_index,
                status_effect: status.status_type,
            },
            Some(status.source_player_index),
        );
        let mut status_listeners = std::mem::take(&mut state.add_status_listeners);
        for listener in status_listeners.iter_mut() {
            if state.players[listener.state.source_player_index]
                .status_effects
                .iter_mut()
                .any(|se| matches!(se.status_type, StatusType::Poisoned | StatusType::Drunk))
            {
                continue;
            }
            state = listener.call(
                state,
                log::StatusApplied {
                    source_player_index: status.source_player_index,
                    target_player_index,
                    status_effect: status.status_type,
                },
            );
        }

        state.add_status_listeners = status_listeners;
    }

    pub(crate) fn cleanup_player_statuses(&mut self, source_player_index: PlayerIndex) {
        for player in self.players.iter_mut() {
            player.remove_players_statuses(source_player_index);
        }
    }

    pub(crate) fn cleanup_statuses(&mut self, cleanup_phase: CleanupPhase) {
        for player in self.players.iter_mut() {
            player.cleanup_statuses(cleanup_phase);
        }
    }

    pub(crate) fn cleanup_event_listeners(&mut self, player_index: PlayerIndex) {
        info!(role = ?self.get_player(player_index).role, "Cleanup for the player");
        info!(?self.death_listeners, "Event Listeners");
        self.nomination_listeners
            .retain(|listener| listener.state.source_player_index != player_index);
        self.attempted_kill_listeners
            .retain(|listener| listener.state.source_player_index != player_index);
        self.death_listeners
            .retain(|listener| listener.state.source_player_index != player_index);
    }
}
