use crate::ChangeRequest;
use crate::engine::change_request::{FilterFuncPtr, StateChangeFuncPtr, check_len};
use crate::engine::player::roles::Roles;
use std::fmt::Display;

use crate::engine::{
    change_request::{ChangeError, ChangeRequestBuilder, ChangeType},
    player::{Alignment, CharacterType, roles::Role},
    state::{PlayerIndex, State},
};

#[derive(Default, Debug, Clone)]
pub(crate) struct Imp {
    pub(crate) last_killed: Option<usize>,
    pub(crate) last_swapped: Option<usize>,
}

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

    fn is_win_condition(&self) -> bool {
        true
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let dead = state.get_player(player_index).dead;
        if dead {
            return None;
        }

        let day_num = state.day_num;

        if let Some(prev_day_num) = self.last_killed
            && prev_day_num == day_num
        {
            return None;
        }

        return ChangeRequest::new_builder(
            ChangeType::ChoosePlayers(1),
            "Ask the Imp to point to the player they would like to kill".into(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let target_players = args.extract_player_indicies()?;
            check_len(&target_players, 1)?;
            let target_player_index = target_players[0];
            state.kill(player_index, target_player_index);
            if let Roles::Imp(imp_data) = &mut state.get_player_mut(player_index).role {
                imp_data.last_killed = Some(day_num);
            }
            state.change_request_queue.push_back(
                // WARN: Unused description
                ChangeRequest::new_builder(ChangeType::NoStoryteller, String::new())
                    .state_change_func(StateChangeFuncPtr::new(move |state, _| {
                        if let Roles::Imp(Imp {
                            last_swapped: Some(day_num),
                            ..
                        }) = &state.get_player(player_index).role
                            && *day_num == state.day_num
                        {
                            return Ok(());
                        }

                        if target_player_index == player_index
                            && state.get_player(player_index).dead
                        {
                            state
                                .change_request_queue
                                .push_back(Imp::new_imp(player_index));
                        }

                        Ok(())
                    })),
            );

            Ok(())
        }))
        .into();
    }
}

impl Imp {
    fn new_imp(player_index: PlayerIndex) -> ChangeRequestBuilder {
        return ChangeRequest::new_builder(ChangeType::ChoosePlayers(1), "Choose a new Imp".into())
            .state_change_func(StateChangeFuncPtr::new(move |state, args| {
                let target_players = args.extract_player_indicies()?;
                check_len(&target_players, 1)?;

                let target_player_index = target_players[0];
                let target_player = state.get_player(target_player_index);
                if target_player.role.get_true_character_type() != CharacterType::Minion {
                    return Err(ChangeError::InvalidSelectedPlayer {
                        reason: "Cannot select a non-minion to become the new imp".into(),
                    });
                }
                let target_player_index = target_players[0];
                let day_num = state.day_num;
                let mut new_role = state.get_player(player_index).role.clone();
                if let Roles::Imp(imp_data) = &mut new_role {
                    imp_data.last_swapped = Some(day_num);
                }
                let target_player = state.get_player_mut(target_player_index);

                target_player.role = new_role;
                Ok(())
            }))
            .filter_func(FilterFuncPtr::new(move |_, player| {
                player.role.get_true_character_type() == CharacterType::Minion
            }));
    }
}

impl Display for Imp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Imp")
    }
}
