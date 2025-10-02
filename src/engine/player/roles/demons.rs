use crate::ChangeRequest;
use crate::engine::{
    change_request::{ChangeResult, FilterFuncPtr, StateChangeFuncPtr, check_len},
    player::roles::RolePtr,
};
use std::fmt::Display;

use crate::engine::{
    change_request::{ChangeError, ChangeRequestBuilder, ChangeType},
    player::{Alignment, CharacterType, roles::Role},
    state::{PlayerIndex, State},
};

#[derive(Default)]
pub(crate) struct Imp {
    last_killed: Option<usize>,
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

        if let Some(prev_day_num) = self.last_killed {
            if prev_day_num == day_num {
                return None;
            }
        }

        return ChangeRequest::new(
            ChangeType::ChoosePlayers(1),
            "Ask the Imp to point to the player they would like to kill".into(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let target_players = args.extract_player_indicies()?;
            check_len(&target_players, 1)?;
            let target_player_index = target_players[0];
            let kill_cr = state.kill(player_index, target_player_index)?;
            if kill_cr.is_some() {
                return Ok(kill_cr);
            }

            if target_player_index == player_index && state.get_player(player_index).dead {
                return Imp::new_imp();
            }

            return Ok(None);
        }))
        .into();
    }
}

impl Imp {
    fn new_imp() -> ChangeResult {
        return ChangeRequest::new(ChangeType::ChoosePlayers(1), "Choose a new Imp".into())
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
                let target_player = state.get_player_mut(target_player_index);

                let new_role = RolePtr::from(Imp {
                    last_killed: Some(day_num),
                });

                target_player.role.reassign(new_role);
                Ok(None)
            }))
            .filter_func(FilterFuncPtr::new(move |_, player| {
                player.role.get_true_character_type() == CharacterType::Minion
            }))
            .into();
    }
}

impl Display for Imp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Imp")
    }
}
