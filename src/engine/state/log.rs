use super::{status_effects::StatusEffect, PlayerIndex, State};
// -- Logging --
// TODO: Implement all events
#[derive(Clone, Debug)]
pub(crate) enum EventType {
    // Game Time Events
    PhaseStart,
    PhaseEnd,
    // Player Events
    Nomination,
    Voting,
    Execution,
    AttemptedKill,
    Death,
    // Ability Specific Events
    StatusApplied(StatusEffect)
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

    fn phase_start_event() -> Self {
        Self {
            event_type: EventType::PhaseStart,
            source_player: None,
            target_player: None,
        }
    }

    fn phase_end_event() -> Self {
        Self {
            event_type: EventType::PhaseEnd,
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
    Setup,
    DayDiscussion,
    DayExecution,
    Night,
}

#[derive(Clone)]
pub(crate) struct DayPhaseLog {
    day_phase: DayPhase,
    log: Vec<Event>,
    day_num: usize
}

// #[derive(Clone)]
// pub(crate) struct Nychthemeron {
//     day_num: usize,
//     day: DayPhaseLog,
//     night: DayPhaseLog,
// }

#[derive(Clone)]
pub(crate) struct Log {
    // TODO: Make this a tree eventually
    day_phases: Vec<DayPhaseLog>,
}

impl Log {
    pub(crate) fn new() -> Self {
        let setup_phase = DayPhaseLog {
            day_phase: DayPhase::Setup,
            log: vec![Event::phase_start_event()],
            day_num: 0,
        };
        Self {
            day_phases: vec![setup_phase],
        }
    }

    pub(crate) fn next_phase(&mut self) {
        // Check the latest day_phase
        match self.day_phases[self.day_phases.len() - 1].day_phase {
            DayPhase::Setup => {
            // Create night one in log
            let night_1 = DayPhaseLog {
                day_phase: DayPhase::Night,
                log: vec![Event::phase_start_event()],
                day_num: 1,
            };
                self.day_phases.push(night_1);
            }
            ,
            DayPhase::DayDiscussion => todo!(),
            DayPhase::DayExecution => todo!(),
            DayPhase::Night => todo!(),
        }
    }

    pub(crate) fn log_event(event : Event) {
        todo!();
    }
}

