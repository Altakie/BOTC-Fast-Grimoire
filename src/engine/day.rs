use crate::engine::{
    change_request::{ChangeRequest, ChangeRequestBuilder},
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
        self.handle_death(target_player_index);
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

    pub(crate) fn day_ability(&self, player_index: PlayerIndex) -> Option<ChangeRequestBuilder> {
        self.get_player(player_index)
            .day_ability(player_index, self)
    }
}
