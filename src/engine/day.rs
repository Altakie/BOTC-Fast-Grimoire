pub(crate) fn resolve_day(&mut self) {
    // Only a few roles act during the day, and the storyteller only really needs to mark
    // whether someone claimed something
    // Some roles like savant come to the story teller during the day, the story teller should
    // have options for all such roles in the game. These options should be shown all at once,
    // (Like "these roles may come up to you today/ act during the day")
    // and the storyteller should be able to quickly log that these events happened
    //
    // FIX: For now, this method will just do nothing. The functionality for it can be
    // implemented later
    self.day_phase = DayPhase::Day;
    todo!();
}
pub(crate) fn nominate_player(
    &mut self,
    source_player_index: PlayerIndex,
    target_player_index: PlayerIndex,
) -> bool {
    // Should execute the target player if the vote succeeds
    // On nomination effects
    let source_player = &mut self.players[source_player_index];
    // match source_player.role {
    //     _ => (),
    // }

    // For now just check for virgin and whether enough votes to pass
    let target_player = &mut self.players[source_player_index];
    match target_player.role {
        Role::Virgin => {
            target_player.ability_active = false;
            return self.execute_player(source_player_index);
        }
        _ => (),
    }

    // TODO: Storyteller should input vote count
    let vote_count: usize = todo!();
    if vote_count >= self.living_player_count() / 2 {
        return self.execute_player(target_player_index);
    }

    return false;
}

pub(crate) fn execute_player(&mut self, target_player_index: PlayerIndex) -> bool {
    // WARNING: There may be shared code between here and kill_player

    // Check if there is something that stops the player's death
    if self
        .get_afflicted_statuses(target_player_index)
        .iter()
        .any(|s| matches!(s.status_type, StatusEffects::DeathProtected))
    {
        return true;
    }

    // Execute a player
    let target_player = &mut self.players[target_player_index];
    target_player.dead = true;

    // TODO: Handle player death based on their role and time of day

    // End the day
    return true;
}
