use std::sync::Arc;
use std::{fmt::Display, ops::Deref};

use leptos::attr::Align;
use macros::{roleptr, roleptr_from, washerwoman_librarian_investigator};

use crate::engine::player::roles::RolePtr;
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
struct Butler();

struct BulterMaster();

impl StatusType for BulterMaster {}

impl Display for BulterMaster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Butler Master")
    }
}

impl Butler {
    fn ability(&self, player_index: PlayerIndex) -> Option<Vec<ChangeRequest>> {
        // Clean up the old butler master status effect (if there is one), prompt for another
        // player, and give them the butler master status effect

        let message = "Prompt the butler to pick a player to be their master".to_string();
        let change_type = ChangeType::ChoosePlayers(1);

        let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
            let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

            if target_players.len() != 1 {
                return Err(());
            }

            // Check that the butler is not picking themselves
            if target_players[0] == player_index {
                return Ok(false);
            }

            return Ok(true);
        };

        let state_change_func = move |state: &mut State, args: ChangeArgs| {
            // Check if there are any butler master status effects inflicted by this player and clear
            // them
            state.cleanup_statuses(player_index);

            let target_player_index =
                unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv)[0];
            let target_player = state.get_player_mut(target_player_index);
            let status = StatusEffect::new(Arc::new(BulterMaster {}), player_index);
            target_player.add_status(status);
        };

        Some(vec![new_change_request!(
            change_type,
            message,
            check_func,
            state_change_func
        )])
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
    ) -> Option<Vec<ChangeRequest>> {
        self.ability(player_index)
    }

    fn night_order(&self) -> Option<usize> {
        Some(83)
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        _state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        self.ability(player_index)
    }
}

impl Display for Butler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Butler")
    }
}

#[derive(Default)]
struct Drunk {
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

    fn setup_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        // TODO: For now just instantly trigger the chnage effect for the townsfolk role that is
        // picked after it is picked
        let description = "Select a not in play Townfolk role";
        let change_type = ChangeType::ChooseRoles(1);
        let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
            let roles = unwrap_args_err!(args, ChangeArgs::Roles(r) => r);

            if roles.len() != 1 {
                return Err(());
            }

            // FIX: Fix to use new role traits instead of enum
            if roles[0].get_type() != CharacterType::Townsfolk {
                return Ok(false);
            }

            return Ok(true);
        };

        let state_change = move |state: &mut State, args: ChangeArgs| {
            let roles = match &args {
                ChangeArgs::Roles(rv) => rv,
                _ => panic!("Wrong input type"),
            };

            let drunk = state.get_player_mut(player_index);
            // TODO: Add a method called notify onto the role trait. This method will do nothing by
            // default, but can be overwritten. This method should take in some args and then
            // figure out what to do with them from there. This is HUGE because it allows the
            // interface to pass the args back to the role if needed
            drunk.notify(&args);
        };

        Some(vec![new_change_request!(
            change_type,
            description,
            check_func,
            state_change
        )])
    }

    fn notify(&self, args: &ChangeArgs) -> Option<RolePtr> {
        match &self.role {
            Some(role) => return role.notify(&args),
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

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        let role = self.role.clone()?;
        role.night_one_ability(player_index, state)
    }

    fn night_order(&self) -> Option<usize> {
        let role = self.role.clone()?;
        role.night_order()
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        let role = self.role.clone()?;
        role.night_ability(player_index, state)
    }
}

impl Display for Drunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = self.role.clone();
        match role {
            Some(role) => write!(f, "The Drunk {}", role.to_string()),
            None => f.write_str("The Drunk"),
        }
    }
}

#[derive(Default)]
struct Recluse();

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
struct Saint();
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
