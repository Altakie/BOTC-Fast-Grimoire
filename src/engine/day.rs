use crate::engine::{
    change_request::{ChangeRequest, ChangeResult},
    state::{PlayerIndex, State, log::Event},
};

impl State {
    pub(crate) fn nominate_player(
        &mut self,
        source_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
    ) {
        // Execute the nominated player's effect on the state
        let target_player = self.get_player(target_player_index).clone();

        target_player.nominate(source_player_index, target_player_index, self);
        self.log.log_event(Event::Nomination {
            nominator_player_index: source_player_index,
            target_player_index,
        });
    }

    pub(crate) fn execute_player(&mut self, target_player_index: PlayerIndex) {
        let target_player = self.get_player_mut(target_player_index);

        target_player.execute();
        self.log.log_event(Event::Execution(target_player_index));

        // After a player is executed, immediately go to night
        self.next_step();
    }

    pub(crate) fn get_day_active(&self) -> Vec<PlayerIndex> {
        self.get_players()
            .iter()
            .enumerate()
            .filter_map(|(index, player)| {
                if player.has_day_ability() {
                    return Some(index);
                }

                None
            })
            .collect()
    }

    pub(crate) fn day_ability(&self, player_index: PlayerIndex) -> Option<ChangeRequest> {
        self.get_player(player_index)
            .day_ability(player_index, self)
    }
}
// pub(crate) fn nominate_player(
//     &mut self,
//     source_player_index: PlayerIndex,
//     target_player_index: PlayerIndex,
// ) {
//
// }
//
// pub(crate) fn execute_player(&mut self, target_player_index: PlayerIndex) -> bool {
//     // WARNING: There may be shared code between here and kill_player
//
//     // Check if there is something that stops the player's death
//     if self
//         .get_afflicted_statuses(target_player_index)
//         .iter()
//         .any(|s| matches!(s.status_type, StatusEffects::DeathProtected))
//     {
//         return true;
//     }
//
//     // Execute a player
//     let target_player = &mut self.players[target_player_index];
//     target_player.dead = true;
//
//     // TODO: Handle player death based on their role and time of day
//
//     // End the day
//     return true;
// }
