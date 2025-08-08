use std::fmt::Display;

use macros::roleptr;

use crate::{
    engine::{
        change_request::{
            ChangeArgs, ChangeError, ChangeRequest, ChangeType, CheckFuncPtr, StateChangeFuncPtr,
        },
        player::{
            Alignment, CharacterType,
            roles::{Role, RolePtr},
        },
        state::{PlayerIndex, State},
    },
    new_change_request, unwrap_args_err, unwrap_args_panic,
};

#[derive(Default)]
pub(crate) struct Imp();

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

    fn night_ability(&self, player_index: PlayerIndex, _state: &State) -> Option<ChangeRequest> {
        let description = "Ask the Imp to point to the player they would like to kill";
        let change_type = ChangeType::ChoosePlayers(1);

        let check_func = move |_state: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
            let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(pv) => pv);
            let len = target_players.len();
            if len != 1 {
                return Err(ChangeError::WrongNumberOfSelectedPlayers {
                    wanted: 1,
                    got: len,
                });
            }

            return Ok(true);
        };
        let change_func = move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
            let target_players = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv);
            let target_player_index = target_players[0];
            let state_snapshot = state.clone();
            let target_player = state.get_player_mut(target_player_index);

            target_player.kill(player_index, &state_snapshot);

            if target_player_index == player_index {
                let description = "Choose a new Imp";
                let change_type = ChangeType::ChoosePlayers(1);

                let check_func =
                    move |state: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
                        let target_players =
                            unwrap_args_err!(args, ChangeArgs::PlayerIndices(pv) => pv);
                        let len = target_players.len();
                        if len != 1 {
                            return Err(ChangeError::WrongNumberOfSelectedPlayers {
                                wanted: 1,
                                got: len,
                            });
                        }

                        let target_player_index = target_players[0];
                        let target_player = state.get_player(target_player_index);
                        if target_player.role.get_true_character_type() != CharacterType::Minion {
                            return Ok(false);
                        }

                        return Ok(true);
                    };

                let change_func =
                    move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
                        let target_players =
                            unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv);
                        let target_player_index = target_players[0];
                        let target_player = state.get_player_mut(target_player_index);

                        let new_role = roleptr!(Imp);

                        target_player.role.reassign(new_role);
                        None
                    };

                return Some(new_change_request!(
                    change_type,
                    description,
                    check_func,
                    change_func
                ));
            }

            return None;
        };

        return Some(new_change_request!(
            change_type,
            description,
            check_func,
            change_func
        ));
    }
}

impl Display for Imp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Imp")
    }
}
