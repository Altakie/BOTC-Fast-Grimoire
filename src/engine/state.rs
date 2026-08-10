#![allow(dead_code, clippy::needless_return)]
pub(crate) mod log;

use log::Log;
use std::{collections::VecDeque, fmt::Debug, sync::Arc};
use tracing::{error, info, warn};
pub(crate) mod status_effects;

use rand::{self, seq::SliceRandom};
use reactive_stores::*;

use crate::{
    engine::{
        change_request::ChangeRequestBuilder,
        player::{
            Alignment, CharacterType, Player,
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
    pub(crate) day_num: usize,
    pub(crate) log: Log,
    script: Script,
    pub(crate) step: Step,
    pub(crate) current_step_actor: Option<PlayerIndex>,

    // pub(crate) curr_args: Option<ChangeArgs>,
    // pub(crate) curr_description: Option<String>,
    pub(crate) change_request_queue: VecDeque<ChangeRequestBuilder>,

    pub(crate) nomination_listeners: Vec<EventListener<log::Nomination>>,
    pub(crate) execution_listeners: Vec<EventListener<log::Execution>>,
    pub(crate) attempted_kill_listeners: Vec<EventListener<log::AttemptedKill>>,
    pub(crate) prevent_kill_default: bool,
    pub(crate) death_listeners: Vec<EventListener<log::Death>>,
    pub(crate) add_status_listeners: Vec<EventListener<log::StatusApplied>>,

    pub(crate) winner: Option<Alignment>,
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
            error!(
                num_roles = roles.len(),
                num_players = player_names.len(),
                "Number of players does not match number of roles"
            );
            // TODO: Figure out to do errors here
            return Err(());
        }

        for i in 0..roles.len() {
            let player = Player::new(player_names[i].clone(), roles[i].convert());
            players.push(player);
        }

        let demon_index = players
            .iter()
            .position(|player| player.role.get_true_character_type() == CharacterType::Demon)
            .unwrap();

        let log = Log::new();

        let mut state = Self {
            players,
            day_num: 1,
            log,
            script,
            step: Step::default(),
            current_step_actor: None,

            // curr_args: None,
            // curr_description: None,
            change_request_queue: VecDeque::new(),

            nomination_listeners: vec![],
            execution_listeners: vec![],
            attempted_kill_listeners: vec![],
            prevent_kill_default: false,
            // TODO: Maybe add a listener for demon death?
            death_listeners: vec![],
            add_status_listeners: vec![],

            winner: None,
        };

        for (player_index, player) in state.players.clone().iter().enumerate() {
            player.role.initialize(player_index, &mut state);
        }

        info!(?state.nomination_listeners, "Listeners");
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

    /// Returns None if the game is still going, or which team won the game (Good or Evil)
    pub(crate) fn game_over(&self) -> Option<Alignment> {
        return self.winner;
    }

    fn update_winner(&mut self) {
        if self.winner.is_some() {
            return;
        }
        // Check if there are any living demons
        let demon_index = self.get_players().iter().position(|player| {
            player.role.get_true_character_type() == CharacterType::Demon && !player.dead
        });

        if demon_index.is_none() {
            self.winner = Some(Alignment::Good);
            return;
        }
        let living_players: Vec<&Player> = self.players.iter().filter(|p| !p.dead).collect();

        if living_players.len() <= 2 {
            self.winner = Some(Alignment::Evil);
            return;
        }

        if living_players
            .iter()
            .filter(|p| p.alignment != Alignment::Evil)
            .count()
            == 0
        {
            self.winner = Some(Alignment::Evil);
            return;
        }
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
                // self.update_winner();
                self.day_num += 1;
                Step::Night
            }
            Step::NightOne | Step::Night => {
                self.cleanup_statuses(CleanupPhase::Dawn);
                self.update_winner();
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
        self.current_step_actor = Some(player_index);
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

    fn log_event(&mut self, event: Event) {
        let actor = self.current_step_actor;
        self.log.log_event(event, actor);
    }

    pub(crate) fn kill(
        &mut self,
        attacking_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
    ) {
        // Go through all kill listeners (can maybe set a change request up to go)
        self.prevent_kill_default = false;
        let mut state = self;
        state.log_event(Event::AttemptedKill {
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
        state.get_player_mut(target_player_index).dead = true;

        let dead = state.get_player(target_player_index).dead;
        if dead {
            state.handle_death(target_player_index);
        }
    }

    pub(crate) fn handle_death(&mut self, player_index: PlayerIndex) {
        let mut state = self;
        state.log_event(Event::Death(player_index));
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
        self.current_step_actor = Some(source_player_index);
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

        state.log_event(Event::Nomination {
            nominator_player_index: source_player_index,
            target_player_index,
        });
    }

    pub(crate) fn execute_player(&mut self, target_player_index: PlayerIndex) {
        self.current_step_actor = Some(target_player_index);

        let mut state = self;

        let mut execution_listeners = std::mem::take(&mut state.execution_listeners);
        for listener in execution_listeners.iter_mut() {
            if state.players[listener.state.source_player_index]
                .status_effects
                .iter_mut()
                .any(|se| matches!(se.status_type, StatusType::Poisoned | StatusType::Drunk))
            {
                continue;
            }
            state = listener.call(state, log::Execution(target_player_index));
        }
        state.execution_listeners = execution_listeners;

        let target_player = state.get_player_mut(target_player_index);
        target_player.dead = true;
        state.handle_death(target_player_index);
        state.log_event(Event::Execution(target_player_index));

        // After a player is executed, immediately go to night
        state.next_step();
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

    pub(crate) fn resolve_day_ability(
        &mut self,
        player_index: PlayerIndex,
    ) -> Option<ChangeRequestBuilder> {
        self.current_step_actor = Some(player_index);
        self.day_ability(player_index)
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
