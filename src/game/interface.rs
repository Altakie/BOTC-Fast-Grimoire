use std::sync::{Arc, Condvar, Mutex};

use super::{Game, PlayerIndex, Role};

#[derive(Clone, Copy)]
pub(crate) enum RoleSelectionType {
    InPlay,
    NotInPlay,
    Script,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Phase {
    Default,
    Done,
    ChoosingRoles,
    ChoosingPlayers,
    InputtingNumber,
    DisplayingNumber,
    DisplayingPlayers,
    DisplayingRoles,
}

#[derive(Debug, Clone)]
pub(crate) struct Info {
    lock: Arc<Mutex<bool>>,
    phase: Phase,
    args: Option<Args>,
    reply: Option<Replies>,
}

impl Info {
    pub(crate) fn new() -> Self {
        Self {
            lock: Arc::new(Mutex::new(false)),
            phase: Phase::Default,
            args: None,
            reply: None,
        }
    }

    // Should only be used if you have a lock
    pub(crate) fn reset(&mut self) {
        self.phase = Phase::Default;
        self.args = None;
        self.reply = None;
    }
}

#[derive(Debug, Clone)]
enum Args {
    ChoosePlayers(usize),
    ChooseRoles(usize, Vec<Role>),
    DisplayNum(usize),
    DisplayPlayers(Vec<String>),
    DisplayRoles(Vec<Role>),
}

#[derive(Debug, Clone, PartialEq)]
enum Replies {
    None,
    Players(Vec<PlayerIndex>),
    Roles(Vec<Role>),
    Number(usize),
}

impl Game {
    fn await_reply(&mut self, args: Args) -> Replies {
        if self.interface_info.reply != None {
            self.interface_info.reply = None;
        }
        self.interface_info.args = Some(args);

        let cv = Condvar::new();
        let reply = loop {
            cv.wait(self.interface_info.lock.lock().unwrap());
            self.interface_info.lock.lock();
            let res = match self.interface_info.reply.clone() {
                Some(reply) => reply,
                None => continue,
            };
            self.interface_info.reset();
            break res;
        };

        return reply;
    }
    pub(crate) fn choose_players(&mut self, num: usize) -> Vec<usize> {
        let reply = self.await_reply(Args::ChoosePlayers(num));

        match reply {
            Replies::Players(p) => return p,
            _ => panic!("Wrong reply type"),
        }
    }

    pub(crate) fn choose_roles(&mut self, num: usize, selector: RoleSelectionType) -> Vec<Role> {
        let roles_in_scope: Vec<Role> = match selector {
            RoleSelectionType::InPlay => self.players.iter().clone().map(|p| p.role).collect(),
            RoleSelectionType::NotInPlay => todo!(),
            RoleSelectionType::Script => self.script.roles.clone(),
        };

        let reply = self.await_reply(Args::ChooseRoles(num, roles_in_scope));

        match reply {
            Replies::Roles(r) => return r,
            _ => panic!("Wrong reply type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{GameStoreFields, tests::EMPTY_SCRIPT};
    use crate::setup_test_game;
    use leptos::prelude::*;
    use reactive_stores::Store;
    use std::{
        thread::{self, sleep},
        time::Duration,
    };

    #[test]
    fn reply_test() {
        let game = Store::new(setup_test_game!().0);

        let ideal_reply = Replies::Players(vec![0, 1]);

        let thread_test = move || {
            game.update(|g| {
                let reply = g.await_reply(Args::ChoosePlayers(2));
                assert_eq!(reply, Replies::Players(vec![0, 1]));
            });
        };

        thread::spawn(thread_test);

        sleep(Duration::new(1, 0));
        game.interface_info().update(|info| {
            info.lock.lock().unwrap();
            info.reply = Some(ideal_reply);
        });
    }
}
