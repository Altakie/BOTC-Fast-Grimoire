use std::fmt::Display;
use std::sync::Arc;

use macros::washerwoman_librarian_investigator;

use crate::{
    engine::{
        change_request::{ChangeArgs, ChangeRequest, ChangeType},
        player::{Alignment, CharacterType, roles::Role},
        state::{
            PlayerIndex, State,
            status_effects::{StatusEffect, StatusType},
        },
    },
    new_change_request, unwrap_args_err, unwrap_args_panic,
};

#[derive(Default)]
pub(crate) struct Washerwoman {}

pub(crate) struct WasherwomanTownsfolk {}
impl StatusType for WasherwomanTownsfolk {}
impl Display for WasherwomanTownsfolk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Washerwoman Townsfolk")
    }
}

pub(crate) struct WasherwomanWrong {}
impl StatusType for WasherwomanWrong {}
impl Display for WasherwomanWrong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Washerwoman Wrong")
    }
}

impl Role for Washerwoman {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn setup_order(&self) -> Option<usize> {
        Some(45)
    }

    fn setup_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        washerwoman_librarian_investigator!(
            player_index,
            WasherwomanTownsfolk,
            WasherwomanWrong,
            "Townsfolk"
        )
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(45)
    }

    fn night_one_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        let player = state.get_player(player_index);
        let message = format!("Show the {} the correct roles", player.role);
        let change_type = ChangeType::Display;

        Some(vec![new_change_request!(change_type, message)])
    }
}

impl Display for Washerwoman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Washerwoman")
    }
}

#[derive(Default)]
pub(crate) struct Librarian {}

pub(crate) struct LibrarianOutsider {}
impl StatusType for LibrarianOutsider {}
impl Display for LibrarianOutsider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Librarian Outsider")
    }
}

pub(crate) struct LibrarianWrong {}
impl StatusType for LibrarianWrong {}
impl Display for LibrarianWrong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Librarian Wrong")
    }
}

impl Role for Librarian {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Outsider
    }

    fn setup_order(&self) -> Option<usize> {
        Some(46)
    }

    fn setup_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        washerwoman_librarian_investigator!(
            player_index,
            LibrarianOutsider,
            LibrarianWrong,
            "Outsider"
        )
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(46)
    }

    fn night_one_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        let player = state.get_player(player_index);
        let message = format!("Show the {} the correct roles", player.role);
        let change_type = ChangeType::Display;

        Some(vec![new_change_request!(change_type, message)])
    }
}

impl Display for Librarian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Librarian")
    }
}

#[derive(Default)]
pub(crate) struct Investigator {}

pub(crate) struct InvestigatorMinion {}
impl StatusType for InvestigatorMinion {}
impl Display for InvestigatorMinion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Investigator Minion")
    }
}

pub(crate) struct InvestigatorWrong {}
impl StatusType for InvestigatorWrong {}
impl Display for InvestigatorWrong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Investigator Wrong")
    }
}

impl Role for Investigator {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn setup_order(&self) -> Option<usize> {
        Some(47)
    }

    fn setup_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        washerwoman_librarian_investigator!(
            player_index,
            InvestigatorMinion,
            InvestigatorWrong,
            "Minion"
        )
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(47)
    }

    fn night_one_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        let player = state.get_player(player_index);
        let message = format!("Show the {} the correct roles", player.role);
        let change_type = ChangeType::Display;

        Some(vec![new_change_request!(change_type, message)])
    }
}

impl Display for Investigator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Investigator")
    }
}

#[derive(Default)]
pub(crate) struct Chef {}

impl Role for Chef {
    fn get_default_alignment(&self) -> crate::engine::player::Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> crate::engine::player::CharacterType {
        CharacterType::Townsfolk
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(48)
    }

    fn night_one_ability(
        &self,
        _player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        // Count pairs of evil players
        // For each evil, player, check if the right player is evil, if yes, increment the
        // pair count
        let change_type = ChangeType::Display;
        let mut pair_count = 0;

        let players = state.get_players();

        for (player_index, player) in players.iter().enumerate() {
            if player.alignment != Alignment::Evil {
                continue;
            }
            let right_player = state.get_player(state.right_player(player_index));
            if right_player.alignment == Alignment::Evil {
                pair_count += 1;
            }
        }
        let message = format!(
            "Show the chef that there are {} pairs of evil players",
            pair_count
        );

        Some(vec![new_change_request!(change_type, message)])
    }
}

impl Display for Chef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Chef")
    }
}

struct Empath {}

impl Empath {
    fn ability(&self, player_index: PlayerIndex, state: &State) -> Option<Vec<ChangeRequest>> {
        // Check how many players next to the empath are evil
        let mut count = 0;
        {
            let left_player = state.get_player(state.left_player(player_index));
            if left_player.alignment == Alignment::Evil {
                count += 1;
            }
        }
        {
            let right_player = state.get_player(state.right_player(player_index));
            if right_player.alignment == Alignment::Evil {
                count += 1;
            }
        }
        let message = format!("Empath has {} evil neighbors", count);

        let change_type = ChangeType::Display;

        Some(vec![new_change_request!(change_type, message)])
    }
}

impl Role for Empath {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(49)
    }

    fn night_order(&self) -> Option<usize> {
        Some(68)
    }

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.ability(player_index, state)
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.ability(player_index, state)
    }
}

impl Display for Empath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Empath")
    }
}

// TODO:
// Fortuneteller
// Undertaker
// Monk
// Ravenkeeper
pub(crate) struct Virgin {
    ability_used: bool,
}

impl Default for Virgin {
    fn default() -> Self {
        Self {
            ability_used: false,
        }
    }
}

impl Role for Virgin {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn nominated(&self, nominating_player_index: PlayerIndex, state: &mut State) {
        let nominator = state.get_player_mut(nominating_player_index);
        nominator.execute();
    }
}

impl Display for Virgin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Virgin")
    }
}
// TODO:
// Slayer
// Soldier
// Mayor
