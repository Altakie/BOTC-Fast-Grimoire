#![allow(unused_variables)]
use std::{fmt::Display, sync::Arc};

use crate::{
    engine::{
        change_request::{ChangeRequest, ChangeType, StateChangeFuncPtr, check_len},
        player::{
            Alignment, CharacterType,
            roles::{Role, RolePtr, demons::Imp},
        },
        state::{
            PlayerIndex, State,
            status_effects::{CleanupPhase, Poisoned, StatusEffect},
        },
    },
    initialization::CharacterTypeCounts,
};

#[derive(Default)]
pub(crate) struct Spy();
impl Spy {
    fn ability(&self) -> Option<ChangeRequest> {
        let change_type = ChangeType::Display;
        let message = "Show the Spy the grimoire";

        ChangeRequest::new_display(change_type, message.into()).into()
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
    ) -> Option<ChangeRequest> {
        self.ability()
    }

    fn night_order(&self) -> Option<usize> {
        Some(84)
    }

    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let dead = state.get_player(player_index).dead;
        if dead {
            return None;
        }
        self.ability()
    }
}

impl Display for Spy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Spy")
    }
}

#[derive(Default)]
pub(crate) struct Baron();

impl Role for Baron {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn initialization_effect(&self) -> Option<crate::initialization::CharacterTypeCounts> {
        Some(CharacterTypeCounts {
            townsfolk: -2,
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
pub(crate) struct Poisoner();

impl Poisoner {
    fn ability(&self, player_index: PlayerIndex) -> Option<ChangeRequest> {
        // Clean up the old poisoned effect, prompt for another
        // player, and give them the poisoned effect

        let message = "Prompt the poisoner to pick a player to poison";
        let change_type = ChangeType::ChoosePlayers(1);

        let state_change_func = StateChangeFuncPtr::new(move |state, args| {
            let target_players = args.extract_player_indicies()?;
            check_len(&target_players, 1)?;

            let target_player = state.get_player_mut(target_players[0]);
            let status = StatusEffect::new(
                Arc::new(Poisoned {}),
                player_index,
                CleanupPhase::Dusk.into(),
            );
            target_player.add_status(status);

            Ok(None)
        });

        ChangeRequest::new(change_type, message.to_string(), state_change_func).into()
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
    ) -> Option<ChangeRequest> {
        self.ability(player_index)
    }

    fn night_order(&self) -> Option<usize> {
        Some(12)
    }

    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let dead = state.get_player(player_index).dead;
        if dead {
            return None;
        }
        self.ability(player_index)
    }
}

impl Display for Poisoner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Poisoner")
    }
}

#[derive(Default)]
pub(crate) struct ScarletWoman();

impl Role for ScarletWoman {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    // fn night_order(&self) -> Option<usize> {
    //     Some(28)
    // }

    // fn night_ability(&self, _player_index: PlayerIndex, state: &State) -> ChangeResult {
    //     // TODO: This might be a little tricky because the scarlet woman should immediately become
    //     // demon when the demon dies. Potentially could have role abilities trigger on events that
    //     // are added to the log as well. This could be useful for scarlet woman. Then have a method
    //     // called on event that needs to be overwritten. This "subscribes" that role to that event
    //     // type in the log. Essentially, whenever that event fires, it will call a function to
    //     // notify all subscribers. Subscribers should be stored in a hash map and initialized at
    //     // the start of the game. Something like that
    //     // Needs to be done for imp as well
    //     // Check player count and if demon is dead. The change type is dynamic here
    //     // WARN: Update this method when travelers are added
    //     let living_player_count = state
    //         .get_players()
    //         .iter()
    //         .filter(|player| !player.dead)
    //         .count();
    //
    //     let demon_alive = state.get_players();
    //
    //     if living_player_count < 5 {
    //         let change_type = ChangeType::NoStoryteller;
    //     }
    //
    //     todo!()
    // }

    // FIX: Very temporary so that scarlet woman can work somehow
    fn has_day_ability(&self) -> bool {
        true
    }

    fn day_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let demon_alive = state.get_players().iter().any(|player| {
            player.role.get_true_character_type() == CharacterType::Demon && !player.dead
        });
        let living_player_count = state
            .get_players()
            .iter()
            .filter(|player| !player.dead)
            .count();

        if living_player_count < 4 || demon_alive {
            return None;
        }

        let change_type = ChangeType::NoStoryteller;
        let description = "The Scarletwoman becomes the imp";
        let change_func = StateChangeFuncPtr::new(move |state, args| {
            let scarlet_woman = state.get_player_mut(player_index);
            scarlet_woman.role.reassign(RolePtr::new::<Imp>());
            Ok(None)
        });

        ChangeRequest::new(change_type, description.into(), change_func).into()
    }
}

impl Display for ScarletWoman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Scarlet Woman")
    }
}
