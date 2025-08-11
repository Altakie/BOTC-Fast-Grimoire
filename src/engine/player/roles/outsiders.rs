use std::fmt::Display;
use std::sync::Arc;

use macros::roleptr_from;

use crate::engine::change_request::ChangeError;
use crate::engine::player::roles::RolePtr;
use crate::engine::state::status_effects::CleanupPhase;
use crate::{
    engine::{
        change_request::{ChangeArgs, ChangeRequest, ChangeType, CheckFuncPtr, StateChangeFuncPtr},
        player::{Alignment, CharacterType, roles::Role},
        state::{
            PlayerIndex, State,
            status_effects::{StatusEffect, StatusType},
        },
    },
    new_change_request, unwrap_args_err, unwrap_args_panic,
};

#[derive(Default)]
pub(crate) struct Butler();

struct BulterMaster();

impl StatusType for BulterMaster {}

impl Display for BulterMaster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Butler Master")
    }
}

impl Butler {
    fn ability(&self, player_index: PlayerIndex) -> Option<ChangeRequest> {
        // Clean up the old butler master status effect (if there is one), prompt for another
        // player, and give them the butler master status effect
        let message = "Prompt the butler to pick a player to be their master".to_string();
        let change_type = ChangeType::ChoosePlayers(1);

        let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
            let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

            let len = target_players.len();
            if len != 1 {
                return Err(ChangeError::WrongNumberOfSelectedPlayers {
                    wanted: 1,
                    got: len,
                });
            }

            // Check that the butler is not picking themselves
            if target_players[0] == player_index {
                return Ok(false);
            }

            return Ok(true);
        };

        let state_change_func =
            move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
                let target_player_index =
                    unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv)[0];
                let target_player = state.get_player_mut(target_player_index);
                let status = StatusEffect::new(
                    Arc::new(BulterMaster {}),
                    player_index,
                    CleanupPhase::Dusk.into(),
                );
                target_player.add_status(status);
                None
            };

        Some(new_change_request!(
            change_type,
            message,
            check_func,
            state_change_func
        ))
    }
}

impl Role for Butler {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Outsider
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(51)
    }

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequest> {
        self.ability(player_index)
    }

    fn night_order(&self) -> Option<usize> {
        Some(83)
    }

    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let dead = state.get_player(player_index).dead;
        if dead {
            return None;
        }
        self.ability(player_index)
    }
}

impl Display for Butler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Butler")
    }
}

#[derive(Default)]
pub(crate) struct Drunk {
    role: Option<RolePtr>,
}

impl Drunk {}

impl Role for Drunk {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Outsider
    }

    fn setup_order(&self) -> Option<usize> {
        Some(1)
    }

    fn setup_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        // If the drunk has a role assigned, call its setup ability instead
        if let Some(role) = &self.role {
            let res = role.setup_ability(player_index, state);
            return match res {
                Some(mut cr) => {
                    cr.state_change_func = None;
                    Some(cr)
                }
                None => None,
            };
        };

        let description = "Select a not in play Townfolk role";
        let change_type = ChangeType::ChooseRoles(1);
        let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
            let roles = unwrap_args_err!(args, ChangeArgs::Roles(r) => r);

            let len = roles.len();
            if len != 1 {
                return Err(ChangeError::WrongNumberOfSelectedRoles {
                    wanted: 1,
                    got: len,
                });
            }

            // FIX: Fix to use new role traits instead of enum
            if roles[0].get_type() != CharacterType::Townsfolk {
                return Ok(false);
            }

            return Ok(true);
        };

        let state_change = move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
            let _roles = match &args {
                ChangeArgs::Roles(rv) => rv,
                _ => panic!("Wrong input type"),
            };

            let state_snapshot = state.clone();

            let drunk = state.get_player_mut(player_index);
            drunk.notify(&args);
            drunk.setup_ability(player_index, &state_snapshot)
        };

        Some(new_change_request!(
            change_type,
            description,
            check_func,
            state_change
        ))
    }

    fn notify(&self, args: &ChangeArgs) -> Option<RolePtr> {
        match &self.role {
            Some(role) => return role.notify(args),
            None => {
                if let ChangeArgs::Roles(roles) = args {
                    let role = roles[0];
                    return Some(roleptr_from!(Self {
                        role: Some(role.convert())
                    }));
                } else {
                    return None;
                }
            }
        }
    }

    fn night_one_order(&self) -> Option<usize> {
        let role = self.role.clone()?;
        role.night_one_order()
    }

    fn night_one_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let role = self.role.clone()?;
        let res = role.night_one_ability(player_index, state);
        return match res {
            Some(mut cr) => {
                cr.state_change_func = None;
                Some(cr)
            }
            None => None,
        };
    }

    fn night_order(&self) -> Option<usize> {
        let role = self.role.clone()?;
        role.night_order()
    }

    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let role = self.role.clone()?;
        let res = role.night_ability(player_index, state);
        return match res {
            Some(mut cr) => {
                cr.state_change_func = None;
                Some(cr)
            }
            None => None,
        };
    }
}

impl Display for Drunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = self.role.clone();
        match role {
            Some(role) => write!(f, "The Drunk {}", role),
            None => f.write_str("The Drunk"),
        }
    }
}

#[derive(Default)]
pub(crate) struct Recluse();

impl Role for Recluse {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Outsider
    }

    fn get_alignment(&self) -> Alignment {
        Alignment::Any
    }
}

impl Display for Recluse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Recluse")
    }
}

#[derive(Default)]
pub(crate) struct Saint();
// TODO:
// Saint is technically a win condition, figure out how winning the game actually comes about

impl Role for Saint {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Outsider
    }
}

impl Display for Saint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Saint")
    }
}
