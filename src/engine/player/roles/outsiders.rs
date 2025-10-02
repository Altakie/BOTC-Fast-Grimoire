use std::fmt::Display;
use std::sync::Arc;

use crate::engine::change_request::{ChangeArgs, ChangeError, ChangeRequest, StateChangeFuncPtr};
use crate::engine::player::roles::RolePtr;
use crate::engine::state::status_effects::CleanupPhase;
use crate::engine::{
    change_request::{ChangeRequestBuilder, ChangeType, check_len},
    player::{Alignment, CharacterType, roles::Role},
    state::{
        PlayerIndex, State,
        status_effects::{StatusEffect, StatusType},
    },
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
    fn ability(&self, player_index: PlayerIndex) -> Option<ChangeRequestBuilder> {
        // Clean up the old butler master status effect (if there is one), prompt for another
        // player, and give them the butler master status effect
        ChangeRequest::new(
            ChangeType::ChoosePlayers(1),
            "Prompt the butler to pick a player to be their master".to_string(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let target_players = args.extract_player_indicies()?;
            check_len(&target_players, 1)?;

            // Check that the butler is not picking themselves
            if target_players[0] == player_index {
                return Err(ChangeError::InvalidSelectedPlayer {
                    reason: "Butler Should not be able to pick themselves".into(),
                });
            }

            let target_player = state.get_player_mut(target_players[0]);
            let status = StatusEffect::new(
                Arc::new(BulterMaster {}),
                player_index,
                CleanupPhase::Dusk.into(),
            );
            target_player.add_status(status);
            Ok(None)
        }))
        .into()
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
    ) -> Option<ChangeRequestBuilder> {
        self.ability(player_index)
    }

    fn night_order(&self) -> Option<usize> {
        Some(83)
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

    // TODO: Should change based on what role is assigned
    fn setup_order(&self) -> Option<usize> {
        Some(1)
    }

    fn setup_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        // If the drunk has a role assigned, call its setup ability instead
        if let Some(role) = &self.role {
            let res = role.setup_ability(player_index, state);
            return match res {
                Some(cr) => Some(cr.clear_state_change_func()),
                None => None,
            };
        };

        ChangeRequest::new(
            ChangeType::ChooseRoles(1),
            "Select a not in play Townfolk role".into(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let roles = args.clone().extract_roles()?;

            check_len(&roles, 1)?;

            if roles[0].get_type() != CharacterType::Townsfolk {
                return Err(ChangeError::InvalidSelectedRole {
                    reason: "Drunk has to be a townsfolk role".into(),
                });
            }

            let state_snapshot = state.clone();

            let drunk = state.get_player_mut(player_index);
            drunk.notify(&args);
            Ok(drunk.setup_ability(player_index, &state_snapshot))
        }))
        .into()
    }

    fn notify(&self, args: &ChangeArgs) -> Option<RolePtr> {
        match &self.role {
            Some(role) => return role.notify(args),
            None => {
                if let ChangeArgs::Roles(roles) = args {
                    let role = roles[0];
                    return Some(RolePtr::from(Self {
                        role: Some(role.convert()),
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

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let role = self.role.clone()?;

        let res = role.night_one_ability(player_index, state);
        return match res {
            Some(mut cr) => {
                cr.state_change_func = None;
                cr.description = format!("(*Drunk*) {}", cr.description);
                Some(cr)
            }
            None => None,
        };
    }

    fn night_order(&self) -> Option<usize> {
        let role = self.role.clone()?;
        role.night_order()
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let role = self.role.clone()?;

        let res = role.night_ability(player_index, state);
        return match res {
            Some(cr) => Some(
                cr.clear_state_change_func()
                    .change_description(|desc| format!("(*Drunk*) {}", desc)),
            ),
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
