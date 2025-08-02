use std::fmt::Display;
use std::sync::Arc;

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
        let right_description = format!("Select a Townsfolk");
        let wrong_description = "Select a different player".to_string();

        let right_status =
            move || StatusEffect::new(Arc::new(WasherwomanTownsfolk {}), player_index);
        let wrong_status = move || StatusEffect::new(Arc::new(WasherwomanWrong {}), player_index);

        let change_type = ChangeType::ChoosePlayers(1);
        let right_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> {
            let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

            if target_player_indices.len() != 1 {
                return Err(());
            }

            for target_player_index in target_player_indices {
                if *target_player_index == player_index {
                    return Ok(false);
                }

                let player = state.get_player(*target_player_index);
                if matches!(
                    player.get_character_type(),
                    CharacterType::Townsfolk | CharacterType::Any
                ) {
                    return Ok(true);
                }
            }

            return Ok(false);
        };

        let right_state_change = move |state: &mut State, args: ChangeArgs| {
            let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

            let target_player = state.get_player_mut(target_player_indices[0]);
            target_player.add_status(right_status());
        };

        let wrong_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> {
            let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

            if target_player_indices.len() != 1 {
                return Err(());
            }

            let target_player_index = target_player_indices[0];

            if target_player_index == player_index {
                return Ok(false);
            }

            let target_player = state.get_player(target_player_index);
            if target_player
                .get_statuses()
                .iter()
                .any(|se| *se == right_status())
            {
                return Ok(false);
            }
            return Ok(true);
        };

        let wrong_state_change = move |state: &mut State, args: ChangeArgs| {
            let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

            // Assign the chosen player the wrong status effect
            let target_player = state.get_player_mut(target_player_indices[0]);
            target_player.add_status(wrong_status());
        };

        Some(vec![
            new_change_request!(
                change_type,
                right_description,
                right_check_func,
                right_state_change
            ),
            new_change_request!(
                change_type,
                wrong_description,
                wrong_check_func,
                wrong_state_change
            ),
        ])
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

fn washerwoman_librarian_investigator<R, W>(
    player_index: PlayerIndex,
    target: &str,
) -> Option<Vec<ChangeRequest>>
where
    R: StatusType,
    W: StatusType,
{
    let right_description = format!("Select a {target}");
    let wrong_description = "Select a different player".to_string();

    let right_status = move || StatusEffect::new(Arc::new(R {}), player_index);
    let wrong_status = move || StatusEffect::new(Arc::new(W {}), player_index);

    let change_type = ChangeType::ChoosePlayers(1);
    let right_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> {
        let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_player_indices.len() != 1 {
            return Err(());
        }

        for target_player_index in target_player_indices {
            if *target_player_index == player_index {
                return Ok(false);
            }

            let player = state.get_player(*target_player_index);
            if matches!(
                player.get_character_type(),
                CharacterType::Townsfolk | CharacterType::Any
            ) {
                return Ok(true);
            }
        }

        return Ok(false);
    };

    let right_state_change = move |state: &mut State, args: ChangeArgs| {
        let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

        let target_player = state.get_player_mut(target_player_indices[0]);
        target_player.add_status(right_status());
    };

    let wrong_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ()> {
        let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_player_indices.len() != 1 {
            return Err(());
        }

        let target_player_index = target_player_indices[0];

        if target_player_index == player_index {
            return Ok(false);
        }

        let target_player = state.get_player(target_player_index);
        if target_player
            .get_statuses()
            .iter()
            .any(|se| *se == right_status())
        {
            return Ok(false);
        }
        return Ok(true);
    };

    let wrong_state_change = move |state: &mut State, args: ChangeArgs| {
        let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

        // Assign the chosen player the wrong status effect
        let target_player = state.get_player_mut(target_player_indices[0]);
        target_player.add_status(wrong_status());
    };

    Some(vec![
        new_change_request!(
            change_type,
            right_description,
            right_check_func,
            right_state_change
        ),
        new_change_request!(
            change_type,
            wrong_description,
            wrong_check_func,
            wrong_state_change
        ),
    ])
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
