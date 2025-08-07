use super::{PlayerIndex, status_effects::StatusEffect};
use crate::Player;
// -- Logging --
// TODO: Implement all events
#[derive(Clone, Debug)]
pub enum EventType {
    // Game Time Events
    // PhaseStart(Step),
    // PhaseEnd(Step),
    // Player Events
    Nomination(Player),
    Voting,
    Execution(Player),
    AttemptedKill(Player),
    Death(Player),
    // Ability Specific Events
    StatusApplied(StatusEffect),
    InfoLearned(String),
}

#[derive(Clone)]
pub struct Event {
    pub event_type: EventType,
    pub source_player: Option<PlayerIndex>,
    pub target_player: Option<PlayerIndex>,
}

impl Event {
    pub fn new(
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

    pub fn new_game_event(event_type: EventType) -> Self {
        Self {
            event_type,
            source_player: None,
            target_player: None,
        }
    }

    // fn phase_start_event() -> Self {
    //     Self {
    //         event_type: EventType::PhaseStart,
    //         source_player: None,
    //         target_player: None,
    //     }
    // }
    //
    // fn phase_end_event() -> Self {
    //     Self {
    //         event_type: EventType::PhaseEnd,
    //         source_player: None,
    //         target_player: None,
    //     }
    // }
    pub fn get_description(&self) -> String {
        todo!();
    }

    pub fn get_reason(&self) -> Option<String> {
        todo!();
    }
}

#[derive(PartialEq, Clone)]
pub enum DayPhase {
    Setup,
    DayDiscussion,
    DayExecution,
    Night,
}

#[derive(Clone)]
pub struct DayPhaseLog {
    day_phase: DayPhase,
    log: Vec<Event>,
    day_num: usize,
}

// #[derive(Clone)]
// pub struct Nychthemeron {
//     day_num: usize,
//     day: DayPhaseLog,
//     night: DayPhaseLog,
// }

#[derive(Clone)]
pub struct Log {
    // TODO: Make this a tree eventually
    day_phases: Vec<DayPhaseLog>,
}

impl Log {
    pub fn new() -> Self {
        let setup_phase = DayPhaseLog {
            day_phase: DayPhase::Setup,
            log: vec![],
            day_num: 0,
        };
        Self {
            day_phases: vec![setup_phase],
        }
    }

    // TODO: Write search macro that takes in optional params (think like a python function)
    // Use opt args crate

    pub fn next_phase(&mut self) {
        // Check the latest day_phase
        match self.day_phases[self.day_phases.len() - 1].day_phase {
            DayPhase::Setup => {
                // Create night one in log
                let night_1 = DayPhaseLog {
                    day_phase: DayPhase::Night,
                    log: vec![],
                    day_num: 1,
                };
                self.day_phases.push(night_1);
            }
            DayPhase::DayDiscussion => todo!(),
            DayPhase::DayExecution => todo!(),
            DayPhase::Night => todo!(),
        }
    }

    fn latest_phase(&mut self) -> &mut DayPhaseLog {
        // WARN: This should never be empty anyway, but do fix this implementation
        self.day_phases.last_mut().unwrap()
    }

    pub fn log_event(&mut self, event: Event) {
        let latest_phase = self.latest_phase();
        latest_phase.log(event);
    }

    // fn search(&self, day_num: usize, event_type: EventType) -> Result<Event, SearchError> {
    //     self.day_phases.get(day_num)
    // }
}

enum SearchError {
    InvalidDayNum,
    EventNotFound,
}

impl DayPhaseLog {
    fn log(&mut self, event: Event) {
        self.log.push(event);
    }
}
