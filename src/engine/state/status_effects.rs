use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::sync::Arc;

use crate::engine::{
    player::PlayerBehaviors,
    state::{PlayerIndex, State},
};

#[derive(Clone)]
pub(crate) struct StatusEffect {
    // pub(crate) status_type: StatusEffects,
    pub(crate) status_type: Arc<dyn StatusType>,
    pub(crate) source_player_index: PlayerIndex,
}

impl StatusEffect {
    pub(crate) fn new(status_type: Arc<dyn StatusType>, source_player_index: PlayerIndex) -> Self {
        Self {
            status_type,
            source_player_index,
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
pub(crate) trait StatusType: Send + Sync + Display {
    fn name(&self) -> String {
        self.to_string()
    }

    // TODO: How to actually handle overwriting default player behaviors
    // Could check every effect manually to see if it affects any aspect of a player
    // Could have a function that defines which aspects of a player's behavior the status effect
    // modifies
    // The player then uses that function to figure out what function it should run from the trait?

    /// This function indicates what type of player behavior this status effect affects. By
    /// default, status effects will not affect player behavior
    fn behavior_type(&self) -> Option<&[PlayerBehaviors]> {
        None
    }

    /// Should be overwritten if this status effect changes how a player dies
    fn kill(&self, _attacking_player_index: PlayerIndex, _state: &State) -> Option<bool> {
        None
    }

    /// Should be overwritten if this status effect changes how a player is execute
    fn execute(&self) -> Option<bool> {
        None
    }
}

impl State {
    pub(crate) fn cleanup_statuses(&mut self, source_player_index: PlayerIndex) {
        for player in self.players.iter_mut() {
            player.remove_players_statuses(source_player_index);
        }
    }
}

pub struct Poisoned {}

impl StatusType for Poisoned {
    fn behavior_type(&self) -> Option<&[crate::engine::player::PlayerBehaviors]> {
        Some(
            [
                PlayerBehaviors::NightOneAbility,
                PlayerBehaviors::NightAbility,
            ]
            .as_ref(),
        )
    }
}

impl Display for Poisoned {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Posioned")
    }
}

pub struct Drunk {}

impl StatusType for Drunk {
    fn behavior_type(&self) -> Option<&[crate::engine::player::PlayerBehaviors]> {
        Some(
            [
                PlayerBehaviors::NightOneAbility,
                PlayerBehaviors::NightAbility,
            ]
            .as_ref(),
        )
    }
}

impl Display for Drunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Drunk")
    }
}
// #[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub(crate) enum StatusType {
//     // General
//     Drunk,
//     Mad,
//     Poisoned,
//     DemonProtected,
//     NightProtected,
//     DeathProtected,
//     NoAbility,
//     // Role specific
//     ButlerMaster,
//     AppearsGood,
//     AppearsEvil,
//     MayorBounceKill,
//     OtherRoleAbility(Roles),
//     FortuneTellerRedHerring,
//     WasherwomanTownsfolk,
//     WasherwomanWrong,
//     LibrarianOutsider,
//     LibrarianWrong,
//     InvestigatorMinion,
//     InvestigatorWrong,
// }
//
// impl Display for StatusType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             StatusType::Drunk => f.write_str("Drunk"),
//             StatusType::Mad => f.write_str("Mad"),
//             StatusType::Poisoned => f.write_str("Poisoned"),
//             StatusType::DemonProtected => f.write_str("Demon Protected"),
//             StatusType::NightProtected => f.write_str("Night Protected"),
//             StatusType::DeathProtected => f.write_str("Death Protected"),
//             StatusType::NoAbility => f.write_str("No Ability"),
//             StatusType::ButlerMaster => f.write_str("Butler Master"),
//             StatusType::AppearsGood => f.write_str("Appears Good"),
//             StatusType::AppearsEvil => f.write_str("Appears Evil"),
//             StatusType::MayorBounceKill => f.write_str("Mayor Bounce Kill"),
//             StatusType::OtherRoleAbility(role) => write!(f, "{}'s Ability", role.to_string()),
//             StatusType::FortuneTellerRedHerring => f.write_str("Fortune Teller Red Herring"),
//             StatusType::WasherwomanTownsfolk => f.write_str("Washerwoman Townsfolk"),
//             StatusType::WasherwomanWrong => f.write_str("Washerwoman Wrong"),
//             StatusType::LibrarianOutsider => f.write_str("Librarian Outsider"),
//             StatusType::LibrarianWrong => f.write_str("Librarian Wrong"),
//             StatusType::InvestigatorMinion => f.write_str("Investigator Minion"),
//             StatusType::InvestigatorWrong => f.write_str("Investigator Wrong"),
//         }
//     }
// }

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
