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
struct Imp();

impl Role for Imp {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Demon
    }

    fn night_order(&self) -> Option<usize> {
        Some(34)
    }

    fn night_ability(
        &self,
        _player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        let description = "Ask the Imp to point to the player they would like to kill";
        let change_type = ChangeType::ChoosePlayers(1);

        // TODO: Make sure that if the imp points to themselves, the game does not end if there is
        // a minion alive and instead the demon's role is transferred to the minion. If there is a
        // scarletwoman, they should be prioritized over other minions. If the demon does pick
        // themselves, there should be another change request triggered that allows the storyteller
        // to pick a player to become demon
        let check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> { todo!() };
        let change_func = move |state: &mut State, args: ChangeArgs| todo!();
        todo!()
    }
}

impl Display for Imp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Imp")
    }
}
