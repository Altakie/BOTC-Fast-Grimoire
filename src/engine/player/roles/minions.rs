use std::{fmt::Display, sync::Arc};

use crate::{
    engine::{
        change_request::{ChangeArgs, ChangeRequest, ChangeType},
        player::{Alignment, CharacterType, PlayerBehaviors, roles::Role},
        state::{
            PlayerIndex, State,
            status_effects::{Poisoned, StatusEffect, StatusType},
        },
    },
    initialization::CharacterTypeCounts,
    new_change_request, unwrap_args_err, unwrap_args_panic,
};

#[derive(Default)]
struct Spy {}
impl Spy {
    fn ability(&self) -> Option<Vec<ChangeRequest>> {
        let change_type = ChangeType::Display;
        let message = "Show the Spy the grimoire".to_string();

        Some(vec![new_change_request!(change_type, message)])
    }
}

impl Role for Spy {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_alignment(&self) -> Alignment {
        Alignment::Any
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn get_character_type(&self) -> CharacterType {
        CharacterType::Any
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(65)
    }

    fn night_one_ability(
        &self,
        _player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.ability()
    }

    fn night_order(&self) -> Option<usize> {
        Some(84)
    }

    fn night_ability(
        &self,
        _player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.ability()
    }
}

impl Display for Spy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Spy")
    }
}

#[derive(Default)]
struct Baron {}

impl Role for Baron {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn initialization_effect(&self) -> Option<crate::initialization::CharacterTypeCounts> {
        Some(CharacterTypeCounts {
            townsfolk: 0,
            outsiders: 2,
            minions: 0,
            demons: 0,
        })
    }
}

impl Display for Baron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Baron")
    }
}

#[derive(Default)]
struct Poisoner {}

impl Poisoner {
    fn ability(&self, player_index: PlayerIndex) -> Option<Vec<ChangeRequest>> {
        // Clean up the old poisoned effect, prompt for another
        // player, and give them the poisoned effect

        let message = "Prompt the poisoner to pick a player to poison";
        let change_type = ChangeType::ChoosePlayers(1);

        let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
            let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

            if target_players.len() != 1 {
                return Err(());
            }

            return Ok(true);
        };

        let state_change_func = move |state: &mut State, args: ChangeArgs| {
            // Check if there are any poisoned status effects inflicted by this player and clear
            // them
            state.cleanup_statuses(player_index);

            let target_player_index =
                unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv)[0];
            let target_player = state.get_player_mut(target_player_index);
            let status = StatusEffect::new(Arc::new(Poisoned {}), player_index);
            target_player.add_status(status);
        };

        Some(vec![new_change_request!(
            change_type,
            message.to_string(),
            check_func,
            state_change_func
        )])
    }
}

impl Role for Poisoner {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(26)
    }

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.ability(player_index)
    }

    fn night_order(&self) -> Option<usize> {
        Some(12)
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.ability(player_index)
    }
}

impl Display for Poisoner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Poisoner")
    }
}

#[derive(Default)]
struct ScarletWoman {}

impl Role for ScarletWoman {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn night_order(&self) -> Option<usize> {
        Some(28)
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        // TODO: This might be a little tricky because the scarlet woman should immediately become
        // demon when the demon dies. Potentially could have role abilities trigger on events that
        // are added to the log as well. This could be useful for scarlet woman. Then have a method
        // called on event that needs to be overwritten. This "subscribes" that role to that event
        // type in the log. Essentially, whenever that event fires, it will call a function to
        // notify all subscribers. Subscribers should be stored in a hash map and initialized at
        // the start of the game. Something like that
        // Needs to be done for imp as well
        // Check player count and if demon is dead. The change type is dynamic here
        // WARN: Update this method when travelers are added
        let living_player_count = state
            .get_players()
            .iter()
            .filter(|player| !player.dead)
            .count();

        let demon_alive = state.get_players();

        if living_player_count < 5 {
            let change_type = ChangeType::NoStoryteller;
        }

        todo!()
    }
}

impl Display for ScarletWoman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Scarlet Woman")
    }
}
