#[derive(Debug, Clone, Copy)]
pub enum Step {
    Setup,
    // Day
    DayDiscussion,
    DayExecution,
    // Night
    Night1,
    Night,
    // Input
    ChoosePlayers,
    ChooseRoles,
    Voting,
    // Display
    DisplayRoles,
    DisplayPlayers,
}
