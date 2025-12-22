use crate::engine::player::roles::Role;
use crate::engine::state::{PlayerIndex, State};

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

        for (i, player) in players.iter().enumerate().skip(start_index) {
            if player.role.setup_order().is_some() {
                return Some(i);
            }
        }

        return None;
    }
}
