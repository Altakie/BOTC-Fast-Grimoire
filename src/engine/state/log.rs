use super::{PlayerIndex, status_effects::StatusEffect};
// -- Logging --

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
    // TODO: Want to be able to notify roles of certain types of events happening
    // Maybe for now have an vec of check functions to see if they need to be notified or something
    // subscriber_map: HashMap<>
    day_num: usize,
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
            day_num: 0,
        }
    }

    // TODO: Probably update this method to be more generic or add more methods for different types
    // of searches
    /// Returns the latest event of this type in the log
    pub fn search<F>(&self, search_func: F) -> Result<&Event, SearchError>
    where
        F: Fn(&Event) -> Option<&Event>,
    {
        for day_phase in self.day_phases.iter().rev() {
            if let Some(event) = day_phase.search(&search_func) {
                return Ok(event);
            }
        }

        return Err(SearchError::EventNotFound);
    }

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
                self.day_num = 1;
                self.day_phases.push(night_1);
            }
            DayPhase::DayDiscussion => {
                self.day_phases.push(DayPhaseLog {
                    day_phase: DayPhase::DayExecution,
                    log: vec![],
                    day_num: self.day_num,
                });
            }
            DayPhase::DayExecution => {
                self.day_phases.push(DayPhaseLog {
                    day_phase: DayPhase::Night,
                    log: vec![],
                    day_num: self.day_num,
                });
            }
            DayPhase::Night => {
                // Only time we should increment day num
                self.day_num += 1;
                self.day_phases.push(DayPhaseLog {
                    day_phase: DayPhase::DayDiscussion,
                    log: vec![],
                    day_num: self.day_num,
                });
            }
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
    Nomination(PlayerIndex),
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

#[derive(PartialEq, Clone)]
pub enum DayPhase {
    Setup,
    DayDiscussion,
    DayExecution,
    Night,
}

pub enum SearchError {
    InvalidDayNum,
    EventNotFound,
}

mod tests {
    #![cfg(test)]

    use super::*;
}
