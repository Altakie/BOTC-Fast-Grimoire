use super::{PlayerIndex, Role, State};

// TODO: Add status effect id
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StatusEffect {
    // pub(crate) status_type: StatusEffects,
    pub(crate) status_type: String,
    pub(crate) source_role: Role,
    pub(crate) source_player_index: PlayerIndex,
    pub(crate) affected_player_index: PlayerIndex,
}

impl StatusEffect {
    pub(crate) fn new(
        status_type: String,
        source_player_index: PlayerIndex,
        source_role: Role,
        affected_player_index: PlayerIndex,
    ) -> Self {
        Self {
            status_type,
            source_player_index,
            source_role,
            affected_player_index,
        }
    }
}

pub(crate) const DRUNK: &str = "Drunk";
pub(crate) const POISONED: &str = "Poisoned";
pub(crate) const NIGHT_PROTECTED: &str = "Night Protected";
pub(crate) const DEMON_PROTECTED: &str = "Demon Protected";
pub(crate) const DEATH_PROTECTED: &str = "Death Protected";
pub(crate) const NO_ABILITY: &str = "No Ability";
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum StatusEffects {
    // General
    Drunk,
    Mad,
    Poisoned,
    DemonProtected,
    NightProtected,
    DeathProtected,
    NoAbility,
    // Role specific
    ButlerMaster,
    AppearsGood,
    AppearsEvil,
    MayorBounceKill,
    TheDrunk,
    FortuneTellerRedHerring,
    WasherwomanTownsfolk,
    WasherwomanWrong,
    LibrarianOutsider,
    LibrarianWrong,
    InvestigatorMinion,
    InvestigatorWrong,
}
impl State {
    pub(crate) fn add_status(
        &mut self,
        effect_type: String,
        source_player_index: PlayerIndex,
        affected_player_index: PlayerIndex,
    ) {
        let new_status = StatusEffect::new(
            effect_type,
            source_player_index,
            self.players[source_player_index].role,
            affected_player_index,
        );
        self.status_effects.push(new_status);
    }

    pub(crate) fn remove_status(
        &mut self,
        effect_type: String,
        source_player_index: PlayerIndex,
        affected_player_index: PlayerIndex,
    ) {
        let index = self
            .status_effects
            .iter()
            .position(|s| {
                s.status_type.trim().to_lowercase() == effect_type.trim().to_lowercase()
                    && s.source_player_index == source_player_index
                    && s.affected_player_index == affected_player_index
            })
            .expect("Tried to remove status effect not in game");
        self.status_effects.remove(index);
    }

    pub(crate) fn get_inflicted_statuses(
        &self,
        source_player_index: PlayerIndex,
    ) -> Vec<StatusEffect> {
        self.status_effects
            .iter()
            .filter(|s| s.source_player_index == source_player_index)
            .cloned()
            .collect()
    }

    pub(crate) fn get_afflicted_statuses(
        &self,
        affected_player_index: PlayerIndex,
    ) -> Vec<StatusEffect> {
        self.status_effects
            .iter()
            .filter(|s| s.affected_player_index == affected_player_index)
            .cloned()
            .collect()
    }
}
