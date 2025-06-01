
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
