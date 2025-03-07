#![allow(dead_code, unused_variables)]

use rand::{self, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::{io, isize, u8};

fn main() {
    // -- Setup --
    // First need to have the story teller upload a script with a list of roles
    //      Should make sure these roles are implemented before starting the game

    // TODO: Make this json parsing not shit
    let script = loop {
        let mut script_json = String::new();
        println!("Put in your script json");
        io::stdin()
            .read_line(&mut script_json)
            .expect("Failed to read line");
        let script_json = match serde_json::from_str::<ScriptJson>(&script_json) {
            Ok(res) => res,
            Err(_) => {
                eprintln!(
                    "[SCRIPT IMPORT FAILED] Your Json is in the wrong format: Please use the official format from the Blood on the Clocktower Custom Script Maker"
                );
                continue;
            }
        };
        break Script::new_from_json(script_json);
    };

    // TODO: Implement the rest of main to understand the API
    // Prompt the story teller to input the number of players
    let mut num_players = String::new();
    println!("Input the number of players:");
    io::stdin()
        .read_line(&mut num_players)
        .expect("Failed to read line");
    let num_players: usize = num_players.parse::<usize>().unwrap();

    // Assign default character numberes
    let player_counts = PlayerCounts::new(num_players);

    // Prompt the story teller to pick the appropriate number of roles from this list
    //      This includes the appropriate number of types of roles (travellers, outsiders, minions,
    //      demons)
    //      While picking roles, if the storyteller picks a role that modifies the setup in any way,
    //      those changes should be applied to the setup numbers, but should not be validated until
    //      the storyteller locks in a list of roles for the game
    // Prompt the storyteller to put in the names of all the players in the game in the order they
    // are sitting (might help to have an anchor point here somewhere)
    //      This should assemble a vector of names
    // Use the roles and player names to create a new seating chart

    // -- Night 1 --
    // Storyteller should give out all roles to players (game not needed here)
    // Game should tell storyteller to introduce demons and minions to each other (might want to
    // include this event in the night order)
    // Game should provide a night 1 specific order based on the roles that are in play (function
    // call)
    // Game should go through this night 1 specific order, providing the appropriate information,
    // or waiting for the appropraite input from the storyteller (through the player), waiting for the storyteller to mark each
    // step as resolved
}

// -- Script Structures --
#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    id: String,
    author: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ScriptEntry {
    Metadata(Metadata),
    Role(Role),
}

#[derive(Debug, Serialize, Deserialize)]
struct ScriptJson(Vec<ScriptEntry>);

struct Script {
    roles: Vec<Role>,
}

impl Script {
    fn new_from_json(json: ScriptJson) -> Self {
        let mut roles: Vec<Role> = vec![];
        for entry in json.0 {
            match entry {
                ScriptEntry::Metadata(metadata) => (),
                ScriptEntry::Role(role) => roles.push(role),
            }
        }

        Self { roles }
    }
}

// -- Setup Structures --
struct PlayerCounts {
    townsfolk: usize,
    outsiders: usize,
    minions: usize,
    demons: usize,
}

impl PlayerCounts {
    fn new(num_players: usize) -> Result<Self, ()> {
        match num_players {
            0..=4 => Err(()),
            5 => Ok(Self {
                townsfolk: 3,
                outsiders: 0,
                minions: 1,
                demons: 1,
            }),
            6 => Ok(Self {
                townsfolk: 3,
                outsiders: 1,
                minions: 1,
                demons: 1,
            }),
            7 => Ok(Self {
                townsfolk: 5,
                outsiders: 0,
                minions: 1,
                demons: 1,
            }),
            8 => Ok(Self {
                townsfolk: 5,
                outsiders: 1,
                minions: 1,
                demons: 1,
            }),
            9 => Ok(Self {
                townsfolk: 5,
                outsiders: 2,
                minions: 1,
                demons: 1,
            }),
            10 => Ok(Self {
                townsfolk: 7,
                outsiders: 0,
                minions: 2,
                demons: 1,
            }),
            11 => Ok(Self {
                townsfolk: 7,
                outsiders: 1,
                minions: 2,
                demons: 1,
            }),
            12 => Ok(Self {
                townsfolk: 7,
                outsiders: 2,
                minions: 2,
                demons: 1,
            }),
            13 => Ok(Self {
                townsfolk: 9,
                outsiders: 0,
                minions: 3,
                demons: 1,
            }),
            14 => Ok(Self {
                townsfolk: 9,
                outsiders: 1,
                minions: 3,
                demons: 1,
            }),
            15 => Ok(Self {
                townsfolk: 9,
                outsiders: 2,
                minions: 3,
                demons: 1,
            }),
            _ => Err(()),
        }
    }
}

// -- Game Structures --

struct SeatingChart {
    players: Vec<Player>,
    win_cond_i: Option<usize>,
}

impl SeatingChart {
    fn new(mut roles: Vec<Role>, player_names: Vec<String>) -> Result<Self, ()> {
        // Make this method construct a new seating chart
        let mut players: Vec<Player> = vec![];

        let mut rng = rand::rng();
        roles.shuffle(&mut rng);

        if roles.len() != player_names.len() {
            eprintln!("Number of players does not match number of roles");
            // Figure out to do errors here
            return Err(());
        }

        for i in 0..roles.len() {
            let player = Player::new(player_names[i].clone(), roles[i]);
            players.push(player);
        }

        assert!(
            players.iter().filter(|p| p.role.is_win_condition()).count() <= 1,
            "Shouldn't have more than one win condition when game starts"
        );

        let win_cond_i = players.iter().position(|p| p.role.is_win_condition());

        return Ok(Self {
            players,
            win_cond_i,
        });
    }

    fn get_player_index(&self, player: &Player) -> usize {
        todo!();
    }

    // Should return true if the player was successfully killed, false if the player failed to die
    fn kill_player(&mut self, player_index: usize) -> bool {
        // Find the target player in the array and set their status to dead
        let player = self.players.get_mut(player_index).unwrap();
        if player
            .statuses
            .iter()
            .find(|s| **s == StatusEffects::Protected)
            != None
        {
            return false;
        }

        player.dead = true;
        return true;
    }

    fn left_player(&self, player: &Player) -> &Player {
        todo!();
    }
    fn right_player(&self, player: &Player) -> &Player {
        todo!();
    }

    fn set_win_condition(&mut self, player: &Player) {
        self.win_cond_i = Some(self.get_player_index(player));
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Alignment {
    Good,
    Evil,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Role {
    Investigator,
    Empath,
    Gossip,
    Innkeeper,
    Washerwoman,
    Librarian,
    Chef,
    Fortuneteller,
    Undertaker,
    Virgin,
    Soldier,
    Slayer,
    Mayor,
    Monk,
    Ravenkeeper,
    Drunk,
    Saint,
    Butler,
    Recluse,
    Spy,
    Baron,
    Scarletwoman,
    Poisoner,
    Imp,
}

impl Role {
    // Also need to define an order on these roles as to their first night order and night order
    fn get_default_alignment(&self) -> Alignment {
        match *self {
            Role::Spy | Role::Baron | Role::Scarletwoman | Role::Poisoner | Role::Imp => {
                Alignment::Evil
            }
            _ => Alignment::Good,
        }
    }

    fn is_win_condition(&self) -> bool {
        match *self {
            Role::Imp => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum StatusEffects {
    Drunk,
    Mad,
    Poisoned,
    Protected,
}

#[derive(Debug)]
struct Player {
    name: String,
    role: Role,
    // Order should be implemented through external array
    dead: bool,
    ghost_vote: bool,
    alignment: Alignment,
    statuses: Vec<StatusEffects>,
}

impl Player {
    fn new(name: String, role: Role) -> Self {
        Self {
            name,
            role,
            ghost_vote: true,
            statuses: vec![],
            dead: false,
            alignment: role.get_default_alignment(),
        }
    }

    fn add_status(&mut self, status: StatusEffects) {
        self.statuses.push(status);
    }

    fn remove_status(&mut self, status: StatusEffects) {
        match self.statuses.iter().position(|s| *s == status) {
            Some(pos) => {
                self.statuses.remove(pos);
                return;
            }
            None => return,
        }
    }
}

impl ToString for Player {
    fn to_string(&self) -> String {
        let player_string = format!(
            "Player {}\n\tRole: {:?}\n
                \tDead?: Not Yet Implemented\n
                \t Statuses: Not yet implemented \n
                \tHas Ghost Vote?: {}\n",
            self.name, self.dead, self.ghost_vote
        );

        return player_string;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        if self.role != other.role {
            return false;
        }

        return true;
    }
}
impl Eq for Player {}

#[cfg(test)]
mod tests {
    // Setup Tests

    use super::*;
    #[test]
    fn test_player_constructor() {
        let good_player_name = String::from("Good");
        // Add in all good players here
        let good_player_roles = vec![
            Role::Investigator,
            Role::Empath,
            Role::Gossip,
            Role::Innkeeper,
        ];

        for role in good_player_roles {
            // Create a new player
            let player = Player::new(good_player_name.clone(), role);
            // Test that the player is alive, has a ghost vote, has the proper name, has no status
            // effects on them, has the right role, and is good
            assert_eq!(player.name, String::from("Good"));
            assert_eq!(player.role, role);
            assert_eq!(player.dead, false);
            assert_eq!(player.ghost_vote, true);
            assert_eq!(player.alignment, Alignment::Good);
        }

        let evil_player_name = String::from("Evil");
        let evil_player_roles = vec![Role::Imp];

        for role in evil_player_roles {
            // Create a new player
            let player = Player::new(evil_player_name.clone(), role);
            // Test that the player is alive, has a ghost vote, has the proper name, has no status
            // effects on them, has the right role, and is good
            assert_eq!(player.name, String::from("Evil"));
            assert_eq!(player.role, role);
            assert_eq!(player.dead, false);
            assert_eq!(player.ghost_vote, true);
            assert_eq!(player.alignment, Alignment::Evil);
        }
    }

    #[test]
    fn test_setup() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let seating_chart = SeatingChart::new(roles.clone(), player_names).unwrap();

        assert_eq!(seating_chart.players.len(), 3);
        assert_eq!(seating_chart.players[1].name, "P2");

        {
            let mut roles = roles.clone();
            for player in seating_chart.players {
                let role_i = match roles.iter().position(|&r| r == player.role) {
                    Some(x) => x,
                    None => {
                        eprintln!("Role not assigned to player");
                        panic!();
                    }
                };

                roles.remove(role_i);
            }

            assert_eq!(roles.len(), 0);
        }
    }

    #[test]
    fn kill_player() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut seating_chart = SeatingChart::new(roles, player_names).unwrap();

        seating_chart.kill_player(0);
        assert_eq!(seating_chart.players[0].dead, true);
        seating_chart.kill_player(1);
        assert_eq!(seating_chart.players[1].dead, true);
        seating_chart.kill_player(2);
        assert_eq!(seating_chart.players[2].dead, true);
    }

    #[test]
    fn kill_protected_player() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut seating_chart = SeatingChart::new(roles, player_names).unwrap();

        seating_chart.players[1].add_status(StatusEffects::Protected);

        seating_chart.kill_player(0);
        assert_eq!(seating_chart.players[0].dead, true);
        seating_chart.kill_player(1);
        assert_eq!(seating_chart.players[1].dead, false);
        seating_chart.kill_player(2);
        assert_eq!(seating_chart.players[2].dead, true);

        seating_chart.players[1].remove_status(StatusEffects::Protected);
        seating_chart.kill_player(1);
        assert_eq!(seating_chart.players[1].dead, true);
    }

    #[test]
    fn test_left() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let seating_chart = SeatingChart::new(roles.clone(), player_names).unwrap();

        assert_eq!(
            seating_chart.left_player(&seating_chart.players[2]).name,
            seating_chart.players[1].name
        );

        // Kill set the left player to dead and see that the left player is updated accordingly
        todo!();
    }

    #[test]
    fn test_right() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let seating_chart = SeatingChart::new(roles, player_names).unwrap();

        assert_eq!(
            seating_chart.left_player(&seating_chart.players[2]).name,
            seating_chart.players[1].name
        );

        // Kill the right player and make sure the right player is updated accordingly
        todo!();
    }
}

