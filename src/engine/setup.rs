use crate::{
    engine::{
        change_request::{ChangeArgs, ChangeRequest, ChangeType},
        player::{CharacterType, roles::Roles},
        state::{PlayerIndex, State, status_effects::StatusType},
    },
    new_change_request, unwrap_args_err, unwrap_args_panic,
};

// use leptos::prelude::*;
// use reactive_stores::Store;

impl State {
    pub(super) fn get_next_active_setup(
        &self,
        previous_player: Option<PlayerIndex>,
    ) -> Option<PlayerIndex> {
        let start_index = match previous_player {
            Some(i) => i + 1,
            None => 0,
        };

        let players = self.get_players();

        for (i, player) in players.iter().skip(start_index + 1).enumerate() {
            if let Some(_) = player.role.setup_order() {
                return Some(i);
            }
        }

        return None;
    }
}

// TODO: Move all this logic to separate files for each role. Perhaps make a mod that contains all
// the roles
impl Roles {
    pub(super) fn setup_action(&self, player_index: PlayerIndex) -> Option<Vec<ChangeRequest>> {
        match self {
            Roles::Soldier => {
                // Just add protected status effect and only remove upon death
                Some(add_status_to_self(
                    player_index,
                    *self,
                    StatusType::DemonProtected,
                ))
            }
            Roles::Mayor => {
                // No night one ability, but add effect to yourself
                Some(add_status_to_self(
                    player_index,
                    *self,
                    StatusType::MayorBounceKill,
                ))
            }
            _ => None,
        }
    }
}

fn fortune_teller(player_index: PlayerIndex) -> Vec<ChangeRequest> {}
