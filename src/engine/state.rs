#![allow(dead_code, clippy::needless_return)]
pub(crate) mod log;

use crate::console_error;
use leptos::leptos_dom::logging::console_log;
use log::Log;
use std::{collections::VecDeque, fmt::Debug, sync::Arc};
pub(crate) mod status_effects;

use rand::{self, seq::SliceRandom};
use reactive_stores::*;

use crate::{
    engine::{
        change_request::ChangeRequestBuilder,
        player::{
            Player,
            roles::{Role, RoleNames},
        },
        state::{
            log::Event,
            status_effects::{CleanupPhase, StatusType},
        },
    },
    initialization::Script,
};

pub(crate) type PlayerIndex = usize;
// #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
// pub(crate) struct PlayerIndex(usize);
// impl Deref for PlayerIndex {
//     type Target = usize;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl Add for PlayerIndex {
//     type Output = PlayerIndex;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Self(self.0 + rhs.0)
//     }
// }
//
// impl<T> Index<PlayerIndex> for T
// where
//     T: Index<usize>,
// {
//     type Output = T::Output;
//
//     fn index(&self, index: PlayerIndex) -> &Self::Output {
//         todo!()
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Step {
    #[default]
    Start,
    Setup,
    // Day
    // DayDiscussion,
    // DayExecution,
    Day,
    // Night
    NightOne,
    Night,
    // Input
    // ChoosePlayers,
    // ChooseRoles,
    // Voting,
    // Display
    // DisplayRoles,
    // DisplayPlayers,
}

#[derive(Clone)]
pub(crate) struct EventListener<EventType> {
    state: EventListenerState,
    listener: Arc<
        dyn for<'a> Fn(&mut EventListenerState, &'a mut State, EventType) -> &'a mut State
            + 'static
            + Send
            + Sync,
    >,
}

impl<EventType> Debug for EventListener<EventType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventListener")
            .field("state", &self.state)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct EventListenerState {
    pub(crate) source_player_index: PlayerIndex,
}

impl<EventType> EventListener<EventType> {
    pub(crate) fn new<F>(source_player_index: PlayerIndex, listener: F) -> Self
    where
        F: for<'a> Fn(&mut EventListenerState, &'a mut State, EventType) -> &'a mut State
            + 'static
            + Send
            + Sync,
    {
        Self {
            state: EventListenerState {
                source_player_index,
            },
            listener: Arc::new(listener),
        }
    }

    fn call<'a>(&mut self, state: &'a mut State, event: EventType) -> &'a mut State {
        (self.listener)(&mut self.state, state, event)
    }
}

#[derive(Store, Debug, Clone)]
pub(crate) struct State {
    players: Vec<Player>,
    win_cond_i: Option<PlayerIndex>,
    pub(crate) day_num: usize,
    pub(crate) log: Log,
    script: Script,
    pub(crate) step: Step,

    // pub(crate) curr_args: Option<ChangeArgs>,
    // pub(crate) curr_description: Option<String>,
    pub(crate) change_request_queue: VecDeque<ChangeRequestBuilder>,

    pub(crate) nomination_listeners: Vec<EventListener<log::Nomination>>,
    pub(crate) attempted_kill_listeners: Vec<EventListener<log::AttemptedKill>>,
    pub(crate) prevent_kill_default: bool,
    pub(crate) death_listeners: Vec<EventListener<log::Death>>,
}

impl State {
    pub(crate) fn new(
        mut roles: Vec<RoleNames>,
        player_names: Vec<String>,
        script: Script,
    ) -> Result<Self, ()> {
        let mut players: Vec<Player> = vec![];

        let mut rng = rand::rng();
        roles.shuffle(&mut rng);

        if roles.len() != player_names.len() {
            eprintln!("Number of players does not match number of roles");
            // TODO: Figure out to do errors here
            return Err(());
        }

        // TODO: Figure out how to store roles properly so we can create a game. Do we even need
        // the roles enum?
        // Idea: For now, just have a method to translate a member of the enum to a role trait
        for i in 0..roles.len() {
            let player = Player::new(player_names[i].clone(), roles[i].convert());
            players.push(player);
        }

        let win_cond_index = players
            .iter()
            .position(|player| player.role.is_win_condition())
            .unwrap();

        let _demon_listener = EventListener::new(
            win_cond_index,
            |listener, state, death_event: log::Death| {
                if death_event.player_index == listener.source_player_index {
                    let win_cond_index = state
                        .players
                        .iter()
                        .position(|player| player.role.is_win_condition() && !player.dead);
                    match win_cond_index {
                        Some(win_cond_index) => listener.source_player_index = win_cond_index,
                        None => {
                            // FIX: For now just setting all players to dead to indicate the game
                            // is over
                            state
                                .players
                                .iter_mut()
                                .for_each(|player| player.dead = true);
                        }
                    }
                }

                state
            },
        );

        // assert!(
        //     players.iter().filter(|p| p.role.is_win_condition()).count() <= 1,
        //     "Shouldn't have more than one win condition when game starts"
        // );

        let win_cond_i = players.iter().position(|p| p.role.is_win_condition());

        let log = Log::new();

        let mut state = Self {
            players,
            win_cond_i,
            day_num: 1,
            log,
            script,
            step: Step::default(),

            // curr_args: None,
            // curr_description: None,
            change_request_queue: VecDeque::new(),

            nomination_listeners: vec![],
            attempted_kill_listeners: vec![],
            prevent_kill_default: false,
            // TODO: Maybe add a listener for demon death?
            death_listeners: vec![],
        };

        for (player_index, player) in state.players.clone().iter().enumerate() {
            player.role.initialize(player_index, &mut state);
        }

        console_log(format!("Listeners: {:#?}", state.nomination_listeners).as_str());
        return Ok(state);
    }

    pub(crate) fn get_player_index(&self, player: &Player) -> PlayerIndex {
        self.players
            .iter()
            .position(|p| p == player)
            .expect("Player should be in player array")
    }

    pub(crate) fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub(crate) fn get_player(&self, player_index: PlayerIndex) -> &Player {
        &self.players[player_index]
    }

    pub(crate) fn get_player_mut(&mut self, player_index: PlayerIndex) -> &mut Player {
        &mut self.players[player_index]
    }

    pub(crate) fn living_player_count(&self) -> usize {
        self.players.iter().filter(|s| !s.dead).count()
    }

    pub(crate) fn left_player(&self, player_index: PlayerIndex) -> PlayerIndex {
        let mut index: PlayerIndex = (player_index + self.players.len() - 1) % self.players.len();
        // eprintln!("{}", index);
        while self.players[index].dead {
            // eprintln!("{}", index);
            index = (index + self.players.len() - 1) % self.players.len();
        }

        return index;
    }
    pub(crate) fn right_player(&self, player_index: PlayerIndex) -> PlayerIndex {
        let mut index: PlayerIndex = (player_index + self.players.len() + 1) % self.players.len();
        while self.players[index].dead {
            index = (index + self.players.len() + 1) % self.players.len();
        }

        return index;
    }

    pub(crate) fn set_win_condition(&mut self, player: &Player) {
        self.win_cond_i = Some(self.get_player_index(player));
    }

    pub(crate) fn game_over(&self) -> bool {
        let index = match self.win_cond_i {
            Some(i) => i,
            None =>
            // TODO: Need to implement this for athiest games, but this should be manually
            // resolved by story teller most likely
            {
                todo!()
            }
        };
        // Game ends if win condition player is dead
        self.players[index].dead
    }

    pub(crate) fn next_step(&mut self) {
        let next_step = match self.step {
            Step::Start => Step::Setup,
            Step::Setup => Step::NightOne,
            // Step::DayDiscussion => Step::DayExecution,
            // Step::DayExecution => {
            //     self.cleanup_statuses(CleanupPhase::Dusk);
            //     self.day_num += 1;
            //     Step::Night
            // }
            Step::Day => {
                self.cleanup_statuses(CleanupPhase::Dusk);
                self.day_num += 1;
                Step::Night
            }
            Step::NightOne | Step::Night => {
                self.cleanup_statuses(CleanupPhase::Dawn);
                Step::Day
            }
        };

        self.log.next_phase();

        self.step = next_step;
        // TODO: Log step change
    }

    pub(crate) fn get_next_active_player(
        &self,
        previous_player: Option<PlayerIndex>,
    ) -> Option<PlayerIndex> {
        match self.step {
            Step::Start => None,
            Step::Setup => self.get_next_active_setup(previous_player),
            Step::NightOne => self.get_next_active_night_one(previous_player),
            Step::Night => self.get_next_active_night(previous_player),
            _ => None,
        }
    }

    /// Function to resolve a player's effect on the state
    ///
    /// # Args
    ///
    /// * player_index : Index of player to resolve for
    ///
    /// # Returns
    ///
    /// * Option<ChangeRequest> : A change request if the role does something, or none if it
    ///   doesn't
    pub(crate) fn resolve(&mut self, player_index: PlayerIndex) {
        let player = self.get_player(player_index);

        let res = match self.step {
            Step::Setup => player.setup_ability(player_index, self),
            Step::NightOne => player.night_one_ability(player_index, self),
            Step::Night => player.night_ability(player_index, self),
            _ => None,
        };

        if let Some(cr) = res {
            self.change_request_queue.push_back(cr);
        }
        // TODO: Log events that happen in the setup
    }

    pub(crate) fn kill(
        &mut self,
        attacking_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
    ) {
        // Go through all kill listeners (can maybe set a change request up to go)
        self.prevent_kill_default = false;
        let mut state = self;
        state.log.log_event(Event::AttemptedKill {
            attacking_player_index,
            target_player_index,
        });
        let mut attempted_kill_listeners = std::mem::take(&mut state.attempted_kill_listeners);
        for listener in attempted_kill_listeners.iter_mut() {
            if state.players[listener.state.source_player_index]
                .status_effects
                .iter_mut()
                .any(|se| matches!(se.status_type, StatusType::Poisoned | StatusType::Drunk))
            {
                continue;
            }

            state = listener.call(
                state,
                log::AttemptedKill {
                    attacking_player_index,
                    target_player_index,
                },
            );
        }
        // Go through all status effects
        for status_effect in state.players[target_player_index].status_effects.iter() {
            if status_effect.status_type == status_effects::StatusType::DemonProtected {
                state.prevent_kill_default = true;
            }
        }

        state.attempted_kill_listeners = attempted_kill_listeners;
        console_error(
            format!(
                "Kill attempted and prevent_default {:?}",
                state.prevent_kill_default
            )
            .as_str(),
        );
        if state.prevent_kill_default {
            return;
        }

        // TODO: Return early if a listener needs us to (need to get this information from the
        // listener)

        // let cr = self.get_player_mut(target_player_index).kill(
        //     attacking_player_index,
        //     target_player_index,
        //     &state_snapshot,
        // );
        // FIX: Shouldn't always successfully kill
        state.get_player_mut(target_player_index).dead = true;

        let dead = state.get_player(target_player_index).dead;
        if dead {
            state.handle_death(target_player_index);
        }
    }

    pub(crate) fn handle_death(&mut self, player_index: PlayerIndex) {
        let mut state = self;
        state.log.log_event(Event::Death(player_index));
        let mut death_listeners = std::mem::take(&mut state.death_listeners);
        for listener in death_listeners.iter_mut() {
            if state.players[listener.state.source_player_index]
                .status_effects
                .iter_mut()
                .any(|se| matches!(se.status_type, StatusType::Poisoned | StatusType::Drunk))
            {
                continue;
            }
            state = listener.call(state, log::Death { player_index });
        }

        state.death_listeners = death_listeners;
        state.cleanup_event_listeners(player_index);
        state.cleanup_player_statuses(player_index);
    }

    pub(crate) fn describe_event(&self, event: Event) -> String {
        match event {
            Event::Nomination {
                nominator_player_index,
                target_player_index,
            } => {
                format!(
                    "{} nominated {} for execution",
                    self.get_player(nominator_player_index).name,
                    self.get_player(target_player_index).name
                )
            }
            Event::Voting {
                players_voted,
                target_player_index,
            } => {
                let player = self.get_player(target_player_index);
                let descriptor = match players_voted {
                    0 => "Nobody",
                    1 => "Person",
                    _ => "People",
                };
                format!(
                    "{} {} voted for {}({})",
                    players_voted, descriptor, player.name, player.role
                )
            }
            Event::Execution(player_index) => {
                let player = self.get_player(player_index);
                format!("{}({}) was executed", player.name, player.role)
            }
            Event::AttemptedKill {
                attacking_player_index,
                target_player_index,
            } => {
                let attacking_player = self.get_player(attacking_player_index);
                let target_player = self.get_player(target_player_index);
                format!(
                    "{}({}) attemped to kill {}({})",
                    attacking_player.name,
                    attacking_player.role,
                    target_player.name,
                    target_player.role
                )
            }
            Event::Death(player_index) => {
                let player = self.get_player(player_index);
                format!("{}({}) died", player.name, player.role)
            }
            Event::StatusApplied {
                source_player_index,
                target_player_index,
                status_effect,
            } => {
                let source_player = self.get_player(source_player_index);
                let target_player = self.get_player(target_player_index);
                format!(
                    "{}({}) gave {}({}) {} effect",
                    source_player.name,
                    source_player.role,
                    target_player.name,
                    target_player.role,
                    status_effect
                )
            }
            Event::InfoLearned(info) =>
            // TODO: Include player index
            {
                format!("{info} was learned")
            }
        }
    }

    pub(crate) fn nominate_player(
        &mut self,
        source_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
    ) {
        // target_player.nominate(source_player_index, target_player_index, self);
        let mut state = self;
        let mut nomination_listeners = std::mem::take(&mut state.nomination_listeners);
        for listener in nomination_listeners.iter_mut() {
            if state.players[listener.state.source_player_index]
                .status_effects
                .iter_mut()
                .any(|se| matches!(se.status_type, StatusType::Poisoned | StatusType::Drunk))
            {
                continue;
            }
            state = listener.call(
                state,
                log::Nomination {
                    nominator_player_index: source_player_index,
                    target_player_index,
                },
            );
        }
        state.nomination_listeners = nomination_listeners;

        state.log.log_event(Event::Nomination {
            nominator_player_index: source_player_index,
            target_player_index,
        });
    }

    pub(crate) fn execute_player(&mut self, target_player_index: PlayerIndex) {
        let target_player = self.get_player_mut(target_player_index);

        // FIX: Make this work properly again and prevent defaults
        // target_player.execute();
        target_player.dead = true;
        // TODO: Call execute listeners
        // Resolve their change requests (right away if possible)
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

    pub(crate) fn get_next_active_night_one(
        &self,
        previous_player: Option<PlayerIndex>,
    ) -> Option<PlayerIndex> {
        let prev_player_order = {
            match previous_player {
                Some(player_index) => self.get_player(player_index).role.night_one_order(),
                None => None,
            }
        };
        let mut next_player: Option<(PlayerIndex, usize)> = None;

        let players = self.get_players();
        for (player_index, player) in players.iter().enumerate() {
            let order = player.role.night_one_order();
            // Check that the player acts at night
            let order = match order {
                Some(order) => order,
                None => continue,
            };
            if let Some(prev_player_order) = prev_player_order {
                if order < prev_player_order {
                    continue;
                } else if order == prev_player_order + 1 {
                    return Some(player_index);
                }
                // If there's a duplicate, return the next player with a higher player index than the
                // current player that has the same role
                else if order == prev_player_order {
                    let previous_player_index = match previous_player {
                        Some(i) => i,
                        None => panic!("Next player has order of 0, should be impossible"), // Should never happen
                    };
                    if player_index <= previous_player_index {
                        continue;
                    }
                    return Some(player_index);
                }
            }
            let next_player_info = match next_player {
                Some(info) => info,
                None => {
                    next_player = Some((player_index, order));
                    continue;
                }
            };
            if order > next_player_info.1 {
                continue;
            };
            // Getting to this point means order is more than the previous_player but less than the
            // current next_player
            next_player = Some((player_index, order));
        }

        match next_player {
            Some(player) => return Some(player.0),
            None => return None,
        }
    }

    pub(crate) fn get_next_active_night(
        &self,
        previous_player: Option<PlayerIndex>,
    ) -> Option<PlayerIndex> {
        let prev_player_order = {
            match previous_player {
                Some(player_index) => self.get_player(player_index).role.night_order(),
                None => None,
            }
        };
        let mut next_player: Option<(PlayerIndex, usize)> = None;

        let players = self.get_players();
        for (player_index, player) in players.iter().enumerate() {
            let order = player.role.night_order();
            // Check that the player acts at night
            let order = match order {
                Some(order) => order,
                None => continue,
            };
            if let Some(prev_player_order) = prev_player_order {
                if order < prev_player_order {
                    continue;
                }
                // If there's a duplicate, return the next player with a higher player index than the
                // current player that has the same role
                else if order == prev_player_order {
                    let previous_player_index = match previous_player {
                        Some(i) => i,
                        None => panic!("Next player has order of 0, should be impossible"), // Should never happen
                    };
                    if player_index <= previous_player_index {
                        continue;
                    }
                    return Some(player_index);
                } else if order == prev_player_order + 1 {
                    return Some(player_index);
                }
            }

            // (player index, order)
            let next_player_info = match next_player {
                Some(info) => info,
                None => {
                    next_player = Some((player_index, order));
                    continue;
                }
            };
            if order >= next_player_info.1 {
                continue;
            };
            // Getting to this point means order is more than the previous_player but less than the
            // current next_player
            next_player = Some((player_index, order));
        }

        match next_player {
            Some(player) => return Some(player.0),
            None => return None,
        }
    }

    fn get_role_order_night1(role: RoleNames) -> usize {
        match role {
            // Role::DUSK => 0,
            // Role::Lordoftyphon => 1,
            // Role::Kazali => 2,
            // Role::Apprentice => 3,
            // Role::Barista => 4,
            // Role::Bureaucrat => 5,
            // Role::Thief => 6,
            // Role::Boffin => 7,
            // Role::Philosopher => 8,
            // Role::Alchemist => 9,
            // Role::Poppygrower => 10,
            // Role::Yaggababble => 11
            // Role::Magician => 12,
            // Role::MINION => 13, // TODO: Need to implement this shit
            // Role::Snitch => 14,
            // Role::Lunatic => 15,
            // Role::Summoner => 16,
            // Role::DEMON => 17, // TODO: Need to implement this shit
            // Role::King => 18,
            // Role::Sailor => 19,
            // Role::Marionette => 20,
            // Role::Engineer => 21,
            // Role::Preacher => 22,
            // Role::Lilmonsta => 23,
            // Role::Lleech => 24,
            // Role::Xaan => 25,
            RoleNames::Poisoner => 26,
            // Role::Widow => 27,
            // Role::Courtier => 28,
            // Role::Wizard => 29,
            // Role::Snakecharmer => 30,
            // Role::Godfather => 31,
            // Role::Organgrinder => 32,
            // Role::Devilsadvocate => 33,
            // Role::Eviltwin => 34,
            // Role::Witch => 35,
            // Role::Cerenovus => 36,
            // Role::Fearmonger => 37,
            // Role::Harpy => 38,
            // Role::Mezepheles => 39,
            // Role::Pukka => 40,
            // Role::Pixie => 41,
            // Role::Huntsman => 42,
            // Role::Damsel => 43,
            // Role::Amnesiac => 44,
            RoleNames::Washerwoman => 45,
            RoleNames::Librarian => 46,
            RoleNames::Investigator => 47,
            RoleNames::Chef => 48,
            RoleNames::Empath => 49,
            RoleNames::Fortuneteller => 50,
            RoleNames::Butler => 51,
            // Role::Grandmother => 52,
            // Role::Clockmaker => 53,
            // Role::Dreamer => 54,
            // Role::Seamstress => 55,
            // Role::Steward => 56,
            // Role::Knight => 57,
            // Role::Noble => 58,
            // Role::Balloonist => 59,
            // Role::Shugenja => 60,
            // Role::Villageidiot => 61,
            // Role::Bountyhunter => 62,
            // Role::Nightwatchman => 63,
            // Role::Cultleader => 64,
            RoleNames::Spy => 65,
            // Role::Ogre => 66,
            // Role::Highpriestess => 67,
            // Role::General => 68,
            // Role::Chambermaid => 69,
            // Role::Mathematician => 70,
            // Role::DAWN => 71, TODO: Figure out wtf this means
            // Role::Leviathan => 72,
            // Role::Vizier => 73
            _ => 0,
        }
    }
}

/// Status Effects can either be visual (just for the storyteller) and do nothing or they can
/// overwrite player behaviors
impl State {
    pub(crate) fn cleanup_player_statuses(&mut self, source_player_index: PlayerIndex) {
        for player in self.players.iter_mut() {
            player.remove_players_statuses(source_player_index);
        }
    }

    pub(crate) fn cleanup_statuses(&mut self, cleanup_phase: CleanupPhase) {
        for player in self.players.iter_mut() {
            player.cleanup_statuses(cleanup_phase);
        }
    }

    pub(crate) fn cleanup_event_listeners(&mut self, player_index: PlayerIndex) {
        console_log(format!("Cleanup for the {}", self.get_player_mut(player_index).role).as_str());
        console_log(format!("Event Listeners are: {:#?}", self.death_listeners).as_str());
        self.nomination_listeners
            .retain(|listener| listener.state.source_player_index != player_index);
        self.attempted_kill_listeners
            .retain(|listener| listener.state.source_player_index != player_index);
        self.death_listeners
            .retain(|listener| listener.state.source_player_index != player_index);
    }
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//
//     // NOTE: Testing Utils
//
//     pub(crate) fn setup_test_game() -> (State, Vec<Roles>) {
//         let roles = vec![
//             Roles::Investigator,
//             Roles::Innkeeper,
//             Roles::Imp,
//             Roles::Chef,
//             Roles::Poisoner,
//         ];
//         let player_names = vec![
//             String::from("P1"),
//             String::from("P2"),
//             String::from("P3"),
//             String::from("P4"),
//             String::from("P5"),
//         ];
//
//         return (
//             State::new(roles.clone(), player_names, EMPTY_SCRIPT).unwrap(),
//             roles,
//         );
//     }
//     pub(crate) const EMPTY_SCRIPT: Script = Script { roles: vec![] };
//     //
//     // // NOTE: Tests
//     // #[test]
//     // fn test_player_constructor() {
//     //     let good_player_name = String::from("Good");
//     //     // Add in all good players here
//     //     let good_player_roles = vec![
//     //         Role::Investigator,
//     //         Role::Empath,
//     //         Role::Gossip,
//     //         Role::Innkeeper,
//     //     ];
//     //
//     //     for role in good_player_roles {
//     //         // Create a new player
//     //         let player = Player::new(good_player_name.clone(), role);
//     //         // Test that the player is alive, has a ghost vote, has the proper name, has no status
//     //         // effects on them, has the right role, and is good
//     //         assert_eq!(player.name, String::from("Good"));
//     //         assert_eq!(player.role, role);
//     //         assert!(!player.dead);
//     //         assert!(player.ghost_vote);
//     //         assert_eq!(player.alignment, Alignment::Good);
//     //     }
//     //
//     //     let evil_player_name = String::from("Evil");
//     //     let evil_player_roles = vec![Role::Imp];
//     //
//     //     for role in evil_player_roles {
//     //         // Create a new player
//     //         let player = Player::new(evil_player_name.clone(), role);
//     //         // Test that the player is alive, has a ghost vote, has the proper name, has no status
//     //         // effects on them, has the right role, and is good
//     //         assert_eq!(player.name, String::from("Evil"));
//     //         assert_eq!(player.role, role);
//     //         assert!(!player.dead);
//     //         assert!(player.ghost_vote);
//     //         assert_eq!(player.alignment, Alignment::Evil);
//     //     }
//     // }
//     //
//     #[test]
//     fn test_new_game() {
//         let (game, roles) = setup_test_game();
//
//         assert_eq!(game.players.len(), 5);
//         assert_eq!(game.players[0].name, "P1");
//         assert_eq!(game.players[1].name, "P2");
//         assert_eq!(game.players[2].name, "P3");
//         assert_eq!(game.players[3].name, "P4");
//         assert_eq!(game.players[4].name, "P5");
//
//         assert_eq!(game.status_effects.len(), 0);
//
//         {
//             let mut roles = roles.clone();
//             for player in game.players {
//                 let role_i = match roles.iter().position(|&r| r == player.role) {
//                     Some(x) => x,
//                     None => {
//                         eprintln!("Role not assigned to player");
//                         panic!();
//                     }
//                 };
//
//                 roles.remove(role_i);
//             }
//
//             assert_eq!(roles.len(), 0);
//         }
//
//         // TODO: Maybe add a check here that all the assigment events were logged
//     }
//
//     // #[test]
//     // fn game_setup() {
//     //     // TODO: Do this after implementing setup method
//     //     // Only way to really test this right now is through baron and drunk
//     //     todo!()
//     // }
//     //
//     #[test]
//     fn kill_player() {
//         let mut game = setup_test_game().0;
//
//         game.kill_player(0, 0);
//         assert!(game.players[0].dead);
//         game.kill_player(1, 1);
//         assert!(game.players[1].dead);
//         game.kill_player(2, 2);
//         assert!(game.players[2].dead);
//     }
//
//     #[test]
//     fn kill_death_protected_player() {
//         let mut game = setup_test_game().0;
//
//         game.add_status(StatusType::DeathProtected, 1, 1);
//
//         game.kill_player(0, 0);
//         assert!(game.players[0].dead);
//         game.kill_player(1, 1);
//         assert!(!game.players[1].dead);
//         game.kill_player(2, 2);
//         assert!(game.players[2].dead);
//
//         game.remove_status(StatusType::DeathProtected, 1, 1);
//         game.kill_player(1, 1);
//         assert!(game.players[1].dead);
//     }
//
//     #[test]
//     fn kill_night_protected_player() {
//         let mut game = setup_test_game().0;
//
//         game.day_phase = DayPhase::Night;
//         game.add_status(StatusType::NightProtected, 1, 1);
//
//         game.kill_player(0, 0);
//         assert!(game.players[0].dead);
//         game.kill_player(1, 1);
//         assert!(!game.players[1].dead);
//         game.kill_player(2, 2);
//         assert!(game.players[2].dead);
//
//         game.day_phase = DayPhase::DayDiscussion;
//         game.kill_player(1, 1);
//         assert!(game.players[1].dead);
//     }
//
//     #[test]
//     fn kill_demon_protected_player() {
//         let mut game = setup_test_game().0;
//
//         game.add_status(StatusType::DemonProtected, 1, 1);
//
//         let demon_index = game.win_cond_i.unwrap();
//
//         game.kill_player(demon_index, 0);
//         assert!(game.players[0].dead);
//         game.kill_player(demon_index, 1);
//         assert!(!game.players[1].dead);
//         game.kill_player(demon_index, 2);
//         assert!(game.players[2].dead);
//
//         game.kill_player(demon_index, 1);
//         assert!(!game.players[1].dead);
//
//         game.remove_status(StatusType::DemonProtected, 1, 1);
//         game.kill_player(demon_index, 1);
//         assert!(game.players[1].dead);
//     }
//     //
//     // #[test]
//     // fn test_left() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     assert_eq!(game.players[game.left_player(1)], game.players[0]);
//     //
//     //     // Kill set the left player to dead and see that the left player is updated accordingly
//     //     game.kill_player(0, 0);
//     //     assert_eq!(game.players[game.left_player(1)], game.players[2]);
//     // }
//     //
//     // #[test]
//     // fn test_right() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     assert_eq!(game.players[game.right_player(1)], game.players[2]);
//     //
//     //     // Kill the right player and make sure the right player is updated accordingly
//     //     game.kill_player(0, 2);
//     //     assert_eq!(game.players[game.right_player(1)], game.players[0]);
//     // }
//     //
//     // #[test]
//     // fn test_game_over() {
//     //     todo!();
//     // }
//     //
//     // #[test]
//     // fn test_get_night_1_order() {
//     //     let game = setup_test_game().0;
//     //
//     //     let player_indices = vec![0, 1, 2, 3, 4];
//     //     let order = game.get_night_1_order(player_indices);
//     //     assert_eq!(game.players[order[0]].role, Role::Poisoner);
//     //     assert_eq!(game.players[order[1]].role, Role::Investigator);
//     //     assert_eq!(game.players[order[2]].role, Role::Chef);
//     //     assert_eq!(order.len(), 3);
//     // }
//     //
//     // fn test_resolve_night_1() {
//     //     todo!();
//     // }
//     //
//     // // TODO: Test that all night one abilities work as expected
//     //
//     // fn test_night_order() {
//     //     let game = setup_test_game().0;
//     //
//     //     let player_indices = vec![0, 1, 2, 3, 4];
//     //     let order = game.get_night_order(player_indices);
//     //     assert_eq!(game.players[order[0]].role, Role::Poisoner);
//     //     assert_eq!(game.players[order[1]].role, Role::Innkeeper);
//     //     assert_eq!(order.len(), 2);
//     // }
//     //
//     // // TODO: Test that all night abilities work as expected
//     // fn tsest_resolve_night() {
//     //     todo!();
//     // }
//     //
//     // #[test]
//     // fn add_status_effect() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     game.add_status(StatusEffects::Poisoned, 2, 0);
//     //
//     //     assert_eq!(game.status_effects[0].status_type, StatusEffects::Poisoned);
//     //     assert_eq!(game.status_effects[0].source_player_index, 2);
//     //     assert_eq!(game.status_effects[0].affected_player_index, 0);
//     // }
//     //
//     // #[test]
//     // fn add_multiple_status_effects() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     game.add_status(StatusEffects::Poisoned, 2, 0);
//     //     game.add_status(StatusEffects::MayorBounceKill, 1, 3);
//     //     game.add_status(StatusEffects::Drunk, 4, 2);
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Poisoned
//     //                     && s.source_player_index == 2
//     //                     && s.source_role == game.players[2].role
//     //                     && s.affected_player_index == 0
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::MayorBounceKill
//     //                     && s.source_player_index == 1
//     //                     && s.source_role == game.players[1].role
//     //                     && s.affected_player_index == 3
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Drunk
//     //                     && s.source_player_index == 4
//     //                     && s.source_role == game.players[4].role
//     //                     && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     // Checks that same player can have multiple status effects applied to them
//     //     // Checks that the same player can have multiple of the same status effect from differnet
//     //     // sources applied to them
//     //     //
//     //     game.add_status(StatusEffects::Drunk, 3, 2);
//     //     game.add_status(StatusEffects::Drunk, 1, 2);
//     //     game.add_status(StatusEffects::Poisoned, 4, 2);
//     //     game.add_status(StatusEffects::Drunk, 1, 0);
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| { s.status_type == StatusEffects::Drunk })
//     //             .count(),
//     //         4
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Drunk && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         3
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Drunk
//     //                     && s.source_player_index == 4
//     //                     && s.source_role == game.players[4].role
//     //                     && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.status_type == StatusEffects::Poisoned && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         1
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| {
//     //                 s.source_player_index == 4
//     //                     && s.source_role == game.players[4].role
//     //                     && s.affected_player_index == 2
//     //             })
//     //             .count(),
//     //         2
//     //     );
//     //
//     //     assert_eq!(
//     //         game.status_effects
//     //             .iter()
//     //             .filter(|s| { s.affected_player_index == 2 })
//     //             .count(),
//     //         4
//     //     );
//     // }
//     //
//     // #[test]
//     // fn find_status_effects_inflicted_by_player() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     game.add_status(StatusEffects::Poisoned, 2, 0);
//     //     game.add_status(StatusEffects::MayorBounceKill, 1, 3);
//     //     game.add_status(StatusEffects::Drunk, 4, 2);
//     //
//     //     game.add_status(StatusEffects::Drunk, 2, 2);
//     //     game.add_status(StatusEffects::Drunk, 2, 1);
//     //     game.add_status(StatusEffects::Drunk, 2, 0);
//     //
//     //     let statuses = game.get_inflicted_statuses(2);
//     //     assert_eq!(statuses.len(), 4);
//     //     assert_eq!(
//     //         statuses
//     //             .iter()
//     //             .filter(|s| s.status_type == StatusEffects::Drunk)
//     //             .count(),
//     //         3
//     //     );
//     //     assert_eq!(
//     //         statuses
//     //             .iter()
//     //             .filter(|s| s.status_type == StatusEffects::Poisoned)
//     //             .count(),
//     //         1
//     //     );
//     //     assert!(statuses.iter().all(|s| s.source_player_index == 2));
//     //     assert!(
//     //         statuses
//     //             .iter()
//     //             .all(|s| s.source_role == game.players[2].role)
//     //     );
//     //
//     //     let no_statuses = game.get_inflicted_statuses(0);
//     //     assert_eq!(no_statuses.len(), 0);
//     // }
//     //
//     // #[test]
//     // fn find_status_effects_inlicted_by_player() {
//     //     let mut game = setup_test_game().0;
//     //
//     //     game.add_status(StatusEffects::Poisoned, 2, 0);
//     //     game.add_status(StatusEffects::MayorBounceKill, 1, 3);
//     //     game.add_status(StatusEffects::Poisoned, 4, 2);
//     //
//     //     game.add_status(StatusEffects::Drunk, 3, 2);
//     //     game.add_status(StatusEffects::Drunk, 1, 2);
//     //     game.add_status(StatusEffects::Drunk, 0, 2);
//     //
//     //     let statuses = game.get_afflicted_statuses(2);
//     //     assert_eq!(statuses.len(), 4);
//     //     assert_eq!(
//     //         statuses
//     //             .iter()
//     //             .filter(|s| s.status_type == StatusEffects::Drunk)
//     //             .count(),
//     //         3
//     //     );
//     //     assert_eq!(
//     //         statuses
//     //             .iter()
//     //             .filter(|s| s.status_type == StatusEffects::Poisoned)
//     //             .count(),
//     //         1
//     //     );
//     //     assert!(statuses.iter().all(|s| s.affected_player_index == 2));
//     //
//     //     let no_statuses = game.get_afflicted_statuses(4);
//     //     assert_eq!(no_statuses.len(), 0);
//     // }
//     //
//     // #[test]
//     // fn remove_status_effect() {
//     //     todo!();
//     // }
//     //
//     // #[test]
//     // fn remove_multiple_status_effects() {
//     //     todo!();
//     // }
// }
//

// NOTE: Role Specific Abilities

// TODO: Make these tests work with the new roles
// #[cfg(test)]
// mod tests {
//     use crate::{
//         Roles,
//         engine::{
//             night::{chef_ability, empath_ability},
//             state::{PlayerIndex, tests::setup_test_game},
//         },
//     };
//     #[test]
//     fn test_get_order() {
//         let game = setup_test_game().0;
//
//         let mut next_player_index = None;
//
//         let mut assert_next_role = |role: Roles| {
//             next_player_index = game.get_next_active_night1(next_player_index);
//             let role_pos = game.players.iter().position(|p| p.role == role).unwrap();
//             assert_eq!(
//                 next_player_index.unwrap(),
//                 role_pos,
//                 "Next Player Role: {}\n {}'s Position is {}",
//                 game.players[next_player_index.unwrap()].role,
//                 role,
//                 role_pos
//             );
//         };
//
//         assert_next_role(Roles::Poisoner);
//         assert_next_role(Roles::Investigator);
//         assert_next_role(Roles::Chef);
//
//         next_player_index = game.get_next_active_player(next_player_index);
//         assert!(next_player_index.is_none());
//     }
//
//     #[test]
//     fn test_empath_ability() {
//         let test_cases = [
//             (
//                 "Empath 0 evil neighbors",
//                 vec![Roles::Investigator, Roles::Empath, Roles::Saint],
//                 vec![],
//                 0,
//             ),
//             (
//                 "Empath dead right neighbor",
//                 vec![
//                     Roles::Investigator,
//                     Roles::Empath,
//                     Roles::Saint,
//                     Roles::Poisoner,
//                 ],
//                 vec![2],
//                 1,
//             ),
//             (
//                 "Empath dead left neighbor",
//                 vec![
//                     Roles::Investigator,
//                     Roles::Empath,
//                     Roles::Saint,
//                     Roles::Chef,
//                     Roles::Scarletwoman,
//                 ],
//                 vec![0],
//                 1,
//             ),
//             (
//                 "Empath right evil neighbor",
//                 vec![Roles::Investigator, Roles::Empath, Roles::Baron],
//                 vec![],
//                 1,
//             ),
//             (
//                 "Empath dead right neighbor initially evil",
//                 vec![
//                     Roles::Investigator,
//                     Roles::Empath,
//                     Roles::Baron,
//                     Roles::Saint,
//                     Roles::Washerwoman,
//                 ],
//                 vec![2],
//                 0,
//             ),
//             (
//                 "Empath dead right neighbor initially evil, new neighbor also evil",
//                 vec![
//                     Roles::Investigator,
//                     Roles::Empath,
//                     Roles::Baron,
//                     Roles::Saint,
//                     Roles::Washerwoman,
//                 ],
//                 vec![2],
//                 0,
//             ),
//             (
//                 "Empath left evil neighbor",
//                 vec![Roles::Scarletwoman, Roles::Empath, Roles::Saint],
//                 vec![],
//                 1,
//             ),
//             (
//                 "Empath dead left evil neighbor initially evil",
//                 vec![
//                     Roles::Scarletwoman,
//                     Roles::Empath,
//                     Roles::Saint,
//                     Roles::Chef,
//                     Roles::Investigator,
//                 ],
//                 vec![0],
//                 0,
//             ),
//             (
//                 "Empath dead left evil neighbor initially evil, new neighbor also evil",
//                 vec![
//                     Roles::Scarletwoman,
//                     Roles::Empath,
//                     Roles::Saint,
//                     Roles::Chef,
//                     Roles::Poisoner,
//                 ],
//                 vec![0],
//                 1,
//             ),
//             (
//                 "Empath both evil neighbors",
//                 vec![Roles::Poisoner, Roles::Empath, Roles::Imp],
//                 vec![],
//                 2,
//             ),
//             (
//                 "Empath initallly both evil neighbors, right dead",
//                 vec![
//                     Roles::Poisoner,
//                     Roles::Empath,
//                     Roles::Imp,
//                     Roles::Chef,
//                     Roles::Investigator,
//                 ],
//                 vec![0],
//                 1,
//             ),
//             (
//                 "Empath initallly both evil neighbors, left dead",
//                 vec![
//                     Roles::Poisoner,
//                     Roles::Empath,
//                     Roles::Imp,
//                     Roles::Chef,
//                     Roles::Investigator,
//                 ],
//                 vec![2],
//                 1,
//             ),
//             (
//                 "Empath initallly both evil neighbors, both dead",
//                 vec![
//                     Roles::Poisoner,
//                     Roles::Empath,
//                     Roles::Imp,
//                     Roles::Chef,
//                     Roles::Investigator,
//                 ],
//                 vec![0, 2],
//                 0,
//             ),
//             (
//                 "Empath recluse evil neighbor",
//                 vec![Roles::Investigator, Roles::Empath, Roles::Recluse],
//                 vec![],
//                 0,
//             ),
//             (
//                 "Empath spy evil neighbor",
//                 vec![Roles::Spy, Roles::Empath, Roles::Investigator],
//                 vec![],
//                 1,
//             ),
//         ];
//
//         for test_case in test_cases {
//             // Create clean environment for each test
//             let mut game = setup_test_game().0;
//             let mut convert_player = |player_index: PlayerIndex| {
//                 game.players[player_index].role = test_case.1[player_index];
//                 game.players[player_index].alignment =
//                     test_case.1[player_index].get_default_alignment();
//             };
//
//             for (i, _) in test_case.1.iter().enumerate() {
//                 convert_player(i);
//             }
//
//             for i in test_case.2 {
//                 game.players[i].dead = true;
//             }
//
//             let desired_num = test_case.3;
//
//             let empath_message = empath_ability(&game, 1)[0].description.clone();
//             let desired_message = format!("Empath has {} evil neighbors", desired_num);
//             assert!(
//                 empath_message == desired_message,
//                 "{} failed. Expected {} evil neighbors, got {}",
//                 test_case.0,
//                 desired_num,
//                 empath_message
//             )
//         }
//     }
//
//     #[test]
//     fn test_chef_ability() {
//         let mut game = setup_test_game().0;
//
//         let test_cases = [
//             (
//                 "0 Chef Pairs",
//                 [
//                     Roles::Imp,
//                     Roles::Chef,
//                     Roles::Spy,
//                     Roles::Washerwoman,
//                     Roles::Empath,
//                 ],
//                 0,
//             ),
//             (
//                 "1 Chef Pair",
//                 [
//                     Roles::Chef,
//                     Roles::Imp,
//                     Roles::Spy,
//                     Roles::Washerwoman,
//                     Roles::Empath,
//                 ],
//                 1,
//             ),
//             (
//                 "1 Chef Pair with Wrap",
//                 [
//                     Roles::Imp,
//                     Roles::Chef,
//                     Roles::Washerwoman,
//                     Roles::Empath,
//                     Roles::Spy,
//                 ],
//                 1,
//             ),
//             (
//                 "3 Evil Players, two sitting together other separate",
//                 [
//                     Roles::Chef,
//                     Roles::Imp,
//                     Roles::Spy,
//                     Roles::Washerwoman,
//                     Roles::Poisoner,
//                 ],
//                 1,
//             ),
//             (
//                 "3 Evil in a row",
//                 [
//                     Roles::Imp,
//                     Roles::Chef,
//                     Roles::Washerwoman,
//                     Roles::Baron,
//                     Roles::Spy,
//                 ],
//                 2,
//             ),
//         ];
//
//         for test_case in test_cases {
//             let mut convert_player = |player_index: PlayerIndex| {
//                 game.players[player_index].role = test_case.1[player_index];
//                 game.players[player_index].alignment =
//                     test_case.1[player_index].get_default_alignment();
//             };
//
//             for i in 0..5 {
//                 convert_player(i);
//             }
//
//             let chef_message = chef_ability(&game)[0].description.clone();
//             let desired_message = format!(
//                 "Show the chef that there are {} pairs of evil players",
//                 test_case.2
//             );
//             assert!(
//                 chef_message == desired_message,
//                 "{} failed. Expected {} pairs of evil players, got {}",
//                 test_case.0,
//                 test_case.2,
//                 chef_message
//             )
//         }
//     }
//
//     // TODO: Test all night abilities (both check funcs and state application funcs)
//     // Imp
//     // Empath
//     // Monk
//     // Poisoner
//     // Butler
//     // Scarletwoman
//     // FortuneTeller
//     // Ravenkeeper
//     // Undertaker
// }
