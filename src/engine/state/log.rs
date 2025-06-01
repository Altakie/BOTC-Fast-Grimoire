use super::PlayerIndex;
// -- Logging --
// TODO: Implement all events
#[derive(Clone, Debug)]
pub(crate) enum EventType {
    // Game Time Events
    DayStart,
    DayEnd,
    NightStart,
    NightEnd,
    // Player Events
    Nomination,
    Execution,
    AttemptedKill,
    Protected,
    Death,
    // Ability Specific Events
}

#[derive(Clone)]
pub(crate) struct Event {
    pub(crate) event_type: EventType,
    pub(crate) source_player: Option<PlayerIndex>,
    pub(crate) target_player: Option<PlayerIndex>,
}

impl Event {
    pub(crate) fn new(
        event_type: EventType,
        source_player: Option<PlayerIndex>,
        target_player: Option<PlayerIndex>,
    ) -> Self {
        Self {
            event_type,
            source_player,
            target_player,
        }
    }

    pub(crate) fn new_game_event(event_type: EventType) -> Self {
        Self {
            event_type,
            source_player: None,
            target_player: None,
        }
    }
    pub(crate) fn get_description(&self) -> String {
        todo!();
    }

    pub(crate) fn get_reason(&self) -> Option<String> {
        todo!();
    }
}

#[derive(PartialEq, Clone)]
pub(crate) enum DayPhase {
    Day,
    Night,
}

#[derive(Clone)]
pub(crate) struct DayPhaseLog {
    day_phase: DayPhase,
    log: Vec<Event>,
}

#[derive(Clone)]
pub(crate) struct Nychthemeron {
    day_num: usize,
    day: DayPhaseLog,
    night: DayPhaseLog,
}
#[derive(Clone)]
pub(crate) struct Log {
    // TODO: Make this a tree eventually
    nychthemrons: Vec<Nychthemeron>,
}

impl Log {
    pub(crate) fn new() -> Self {
        Self {
            nychthemrons: vec![],
        }
    }
}
