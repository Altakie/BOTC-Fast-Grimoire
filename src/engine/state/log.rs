use std::fmt::Display;

use crate::engine::state::Step;

use super::{PlayerIndex, status_effects::StatusEffect};
// -- Logging --

#[derive(Clone, Debug)]
pub struct DayPhaseLog {
    pub(crate) day_phase: Step,
    pub(crate) log: Vec<Event>,
    pub(crate) day_num: usize,
}

// #[derive(Clone)]
// pub struct Nychthemeron {
//     day_num: usize,
//     day: DayPhaseLog,
//     night: DayPhaseLog,
// }

#[derive(Clone, Debug)]
pub struct Log {
    // TODO: Make this a tree eventually
    pub(crate) day_phases: Vec<DayPhaseLog>,
    // TODO: Want to be able to notify roles of certain types of events happening
    // Maybe for now have an vec of check functions to see if they need to be notified or something
    // subscriber_map: HashMap<>
    pub(crate) day_num: usize,
}

impl Log {
    pub fn new() -> Self {
        let setup_phase = DayPhaseLog {
            day_phase: Step::Setup,
            log: vec![],
            day_num: 0,
        };
        Self {
            day_phases: vec![setup_phase],
            day_num: 0,
        }
    }

    // TODO: Probably update this method to be more generic or add more methods for different types
    // of searches
    /// Returns the latest event of this type in the log
    pub fn search_previous_phase<F>(&self, search_func: F) -> Result<&Event, SearchError>
    where
        F: Fn(&Event) -> Option<&Event>,
    {
        let day_phase = match self.get_previous_phase() {
            Some(day_phase) => day_phase,
            None => return Err(SearchError::InvalidDayNum),
        };
        if let Some(event) = day_phase.search(&search_func) {
            return Ok(event);
        }

        return Err(SearchError::EventNotFound);
    }

    pub fn search_current_phase<F>(&self, search_func: F) -> Result<&Event, SearchError>
    where
        F: Fn(&Event) -> Option<&Event>,
    {
        let day_phase = self.get_latest_phase();
        if let Some(event) = day_phase.search(&search_func) {
            return Ok(event);
        }

        return Err(SearchError::EventNotFound);
    }

    pub fn next_phase(&mut self) {
        // Check the latest day_phase
        match self.get_mut_latest_phase().day_phase {
            Step::Setup => {
                // Create night one in log
                let night_1 = DayPhaseLog {
                    day_phase: Step::NightOne,
                    log: vec![],
                    day_num: 1,
                };
                self.day_num = 1;
                self.day_phases.push(night_1);
            }
            Step::Day => {
                self.day_phases.push(DayPhaseLog {
                    day_phase: Step::Night,
                    log: vec![],
                    day_num: self.day_num,
                });
            }
            Step::NightOne => {
                // Only time we should increment day num
                self.day_num += 1;
                self.day_phases.push(DayPhaseLog {
                    day_phase: Step::Day,
                    log: vec![],
                    day_num: self.day_num,
                });
            }
            Step::Night => {
                // Only time we should increment day num
                self.day_num += 1;
                self.day_phases.push(DayPhaseLog {
                    day_phase: Step::Day,
                    log: vec![],
                    day_num: self.day_num,
                });
            }
            Step::Start => panic!("Log should never have Start Phase"),
        }
    }

    fn get_previous_phase(&self) -> Option<&DayPhaseLog> {
        let len = self.day_phases.len();
        self.day_phases.get(len - 2)
    }

    fn get_mut_previous_phase(&mut self) -> Option<&mut DayPhaseLog> {
        let len = self.day_phases.len();
        self.day_phases.get_mut(len - 2)
    }

    fn get_latest_phase(&self) -> &DayPhaseLog {
        // WARN: This should never be empty anyway, but do fix this implementation to not panic if
        // it is
        assert!(!self.day_phases.is_empty());
        self.day_phases.last().unwrap()
    }

    fn get_mut_latest_phase(&mut self) -> &mut DayPhaseLog {
        // WARN: This should never be empty anyway, but do fix this implementation to not panic if
        // it is
        assert!(!self.day_phases.is_empty());
        self.day_phases.last_mut().unwrap()
    }

    pub fn log_event(&mut self, event: Event) {
        let latest_phase = self.get_mut_latest_phase();
        latest_phase.log(event);
    }

    // fn search(&self, day_num: usize, event_type: EventType) -> Result<Event, SearchError> {
    //     self.day_phases.get(day_num)
    // }
}

impl DayPhaseLog {
    fn log(&mut self, event: Event) {
        self.log.push(event);
    }

    fn search<F>(&self, func: F) -> Option<&Event>
    where
        F: FnMut(&Event) -> Option<&Event>,
    {
        self.log.iter().rev().find_map(func)
    }
}

// macro_rules! event_type {
//     ($event:ty) => {{ EventType(PhantomData::from($event::default())) }};
// }

//
// TODO: Implement all events
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    // Game Time Events
    // PhaseStart(Step),
    // PhaseEnd(Step),
    // Player Events
    Nomination {
        nominator_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
    },
    Voting {
        players_voted: Option<usize>,
    },
    Execution(PlayerIndex),
    AttemptedKill {
        attacking_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
    },
    Death(PlayerIndex),
    // Ability Specific Events
    StatusApplied(StatusEffect),
    InfoLearned(String),
}

impl Event {
    pub fn get_description(&self) -> String {
        todo!();
    }

    pub fn get_reason(&self) -> Option<String> {
        todo!();
    }
}

#[derive(Debug)]
pub enum SearchError {
    InvalidDayNum,
    EventNotFound,
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let day_phases_str = self
            .day_phases
            .iter()
            .map(|day_phase| format!("{}", day_phase))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "Day {}\n\n{}", self.day_num, day_phases_str)
    }
}

impl Display for DayPhaseLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let log_str = self
            .log
            .iter()
            .map(|event| format!("\t{}", event))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{:?} {}\n{}", self.day_phase, self.day_num, log_str)
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_search_previous_phase() {
        let mut log = Log::new();
        log.next_phase();
        log.next_phase();
        let execution_event = Event::Execution(2);
        log.log_event(execution_event.clone());
        log.next_phase();

        let event = log.search_previous_phase(|ev| match *ev {
            Event::Execution(_) => Some(ev),
            _ => None,
        });

        assert!(
            execution_event == *event.expect("No execution event was found in the previous phase")
        );
    }
}
