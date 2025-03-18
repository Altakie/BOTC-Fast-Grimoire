#![allow(dead_code, unused_variables)]

use rand::{
    self,
    seq::{IndexedRandom, SliceRandom},
};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, io, isize, rc::Rc, u8, usize};

// TODO: ACTUALLY IMPORTANT STUFF
// Split this file into 3 parts: 1 for the setup, 1 for the engine, and 1 for the all the commonly
// used stuff (enums for roles, alignment, etc)
// Maybe even split up the engine into two parts, one part for the setup of the engine and the
// other parts for game logic
// Definitely need more modularity, maybe sketch out all the modules that need to be in your system
// and how they will interact: what data they should be passing to each other and what not
// Refactor the engine (currently there seems to be a lot of issues in how the gameplay loop is
// implemented)

fn main() {
    // -- Setup --
    // First need to have the story teller upload a script with a list of roles
    //      Should make sure these roles are implemented before starting the game

    let script = get_script();

    let player_names = get_player_names();
    // let num_players = get_player_count();

    // Assign default character numbers

    // Prompt the story teller to pick the appropriate number of roles from this list
    //      This includes the appropriate number of types of roles (travellers, outsiders, minions,
    //      demons)
    //      While picking roles, if the storyteller picks a role that modifies the setup in any way,
    //      those changes should be applied to the setup numbers, but should not be validated until
    //      the storyteller locks in a list of roles for the game

    let roles = choose_roles(player_names.len(), script.clone());
    // Prompt the storyteller to put in the names of all the players in the game in the order they
    // are sitting (might help to have an anchor point here somewhere)
    //      This should assemble a vector of names

    // Use the roles and player names to create a new seating chart

    let game = Game::new(roles, player_names).unwrap();

    // -- Night 1 --
    // Storyteller should give out all roles to players (game not needed here)
    // Game should tell storyteller to introduce demons and minions to each other (might want to
    // include this event in the night order)
    // Game should provide a night 1 specific order based on the roles that are in play (function
    // call)
    // Game should go through this night 1 specific order, providing the appropriate information,
    // or waiting for the appropraite input from the storyteller (through the player), waiting for the storyteller to mark each
    // step as resolved

    game.resolve_night_1();
    // TODO: Keep a list of alive roles that are active
    // Whenever someone dies, remove that role from the list
    // Order that list by night order (should be different for night one and other nights)
    loop {
        game.resolve_day();
        if game.game_over() {
            break;
        }
        game.resolve_night();
    }

    println!("Game Over!");
}

fn get_script() -> Script {
    loop {
        let mut script_json = String::new();
        println!("Put in your script json");
        match io::stdin().read_line(&mut script_json) {
            Ok(_) => (),
            Err(_) => eprintln!("Something went wrong, please try again)"),
        };
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
    }
}

fn get_player_names() -> Vec<String> {
    let names: Vec<String> = vec![];
    println!("Input the names of the players in the order in which they are seated");
    let count: usize = 0;
    loop {
        println!("Would you like to add another player? (y/n)");
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Something went wrong, please try again");
                continue;
            }
        }
        match buf.trim().to_lowercase().as_str() {
            "y" => (),
            "n" => continue,
            _ => continue,
        }

        println!("Input the name of player {}:", count);
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Something went wrong, please try again");
                continue;
            }
        }
    }
}

fn get_player_count() -> usize {
    loop {
        let mut buf = String::new();
        println!("Input the number of players:");
        match io::stdin().read_line(&mut buf) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Something went wrong, please try again");
                continue;
            }
        }
        match buf.trim().parse::<usize>() {
            Ok(v) => break v,
            Err(_) => {
                eprintln!("Please Input a number");
                continue;
            }
        }
    }
}

fn choose_roles(num_players: usize, mut script: Script) -> Vec<Role> {
    // TODO: Implement a method to check if a role alters the setup
    let mut roles: Vec<Role> = vec![];
    let player_counts = PlayerCounts::new(num_players).unwrap();
    let mut temp_counts = PlayerCounts::new_empty();

    loop {
        println!("Currently Selected Roles");
        for role in roles.iter() {
            println!("\t{:?}", role);
        }

        if roles.len() == num_players {
            let valid_count = validate_player_counts(&player_counts, &temp_counts);
            if valid_count {
                break;
            }
            panic!("Invalid character types");
        }

        let diff = player_counts.diff(&temp_counts);

        println!(
            "Roles left to add\n\tTownsfolk: {}\n\tOutsiders {}\n\t Minions: {}\n\t Demons: {}",
            diff.townsfolk, diff.outsiders, diff.minions, diff.demons
        );

        println!(
            "Select another role to put into play (Select the number of the role to add them):"
        );
        let mut count = 1;
        for role in script.roles.iter() {
            println!("{} {:?}", count, role);
            count += 1;
        }

        let mut buf = String::new();
        println!("Input the number of the role you would like to add:");
        match io::stdin().read_line(&mut buf) {
            Ok(_) => (),
            Err(_) => {
                eprintln!("Something went wrong, please try again");
                continue;
            }
        }
        let role_index = match buf.trim().parse::<usize>() {
            Ok(v) => {
                if v <= script.roles.len() {
                    v - 1
                } else {
                    eprintln!("Please input a number between 0 and {}", script.roles.len());
                    continue;
                }
            }
            Err(_) => {
                eprintln!("Please Input a number");
                continue;
            }
        };

        let role = script.roles.remove(role_index);
        match role.get_type() {
            CharacterType::Townsfolk => temp_counts.townsfolk += 1,
            CharacterType::Outsider => temp_counts.outsiders += 1,
            CharacterType::Minion => temp_counts.minions += 1,
            CharacterType::Demon => temp_counts.demons += 1,
        }
        roles.push(role);
    }

    return roles;
}

fn validate_player_counts(ideal: &PlayerCounts, actual: &PlayerCounts) -> bool {
    ideal.diff(actual).is_empty()
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

#[derive(Clone)]
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
#[derive(Debug, PartialEq, Eq)]
struct PlayerCounts {
    townsfolk: isize,
    outsiders: isize,
    minions: isize,
    demons: isize,
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

    fn new_empty() -> Self {
        Self {
            townsfolk: 0,
            outsiders: 0,
            minions: 0,
            demons: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.townsfolk == 0 && self.outsiders == 0 && self.minions == 0 && self.demons == 0
    }

    fn diff(&self, other: &Self) -> Self {
        Self {
            townsfolk: self.townsfolk - other.townsfolk,
            outsiders: self.outsiders - other.outsiders,
            minions: self.minions - other.minions,
            demons: self.demons - other.demons,
        }
    }
}

// -- Game Structures --

struct Game {
    players: Vec<Player>, // NOTE: Maybe should be a map instead
    // Want to have pointers to players
    win_cond_i: Option<usize>,
    // WARNING: Deprecated for now - active_roles: HashMap<Role, usize>, // Should hold a role and the corresponding player's index
    // TODO: Implement a log here of all the events that have happened during the game
}

impl Game {
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

        // WARNING: Deprecated for now, might need to remove later
        // let mut active_roles: HashMap<Role, usize> = HashMap::new();
        //
        // let index = 0;
        // for player in players.iter() {
        //     active_roles.insert(player.role.clone(), index);
        // }

        return Ok(Self {
            players,
            win_cond_i,
            // active_roles,
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

        // Resolve the player's death
        // TODO: Need to figure out a clean way to handle this
        // Usually when a player dies, their ability is deactivated. However there are a few
        // exceptions: Recluse, Spy, Ravenkeeper, Zombuul
        // Ideas: Maybe make a match where the default case is deactivate the ability upon death
        // but for other cases you actually want to activate the ability
        // Feels like I need to refactor something here
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

    fn get_active_roles(&self) -> Vec<usize> {
        let mut result: Vec<usize> = vec![];
        for i in 0..self.players.len() {
            let player = &self.players[i];
            if !player.ability_active {
                continue;
            }
            result.push(i);
        }

        return result;
    }

    fn game_over(&self) -> bool {
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

    fn resolve_night_1(&self) {
        // Order the roles in this game to get the order they should be woken up in (should be
        // unique to night 1)
        let ordered_player_indices = self.get_night_1_order(self.get_active_roles());
        // Wake each role up in order and show them the information they need to know, or the
        // choices that they get
        // For each choice:
        //      If that choice impacts the game state, change the game state accordingly
        //      If that choice tells the player info, give them that info
        //      Should be calling a generic method on the role class to get info on the role's
        //      ability
        // Once you have gone through all the roles, nothing to do: wake everyone up
        for i in ordered_player_indices.iter() {
            self.resolve_night_1_ability(*i);
        }
    }

    fn get_night_1_order(&self, player_indices: Vec<usize>) -> Vec<usize> {
        // Go through all roles and assign each of them a number
        // Maps night_order to player index
        let mut order_map: HashMap<usize, usize> = HashMap::new();
        for index in player_indices {
            let role = self.players[index].role;
            let order: usize = match role {
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
                // Role::Yaggababble => 11,
                // Role::Magician => 12,
                // Role::MINION => 13, TODO: Need to implement this shit
                // Role::Snitch => 14,
                // Role::Lunatic => 15,
                // Role::Summoner => 16,
                // Role::DEMON => 17, TODO: Need to implement this shit
                // Role::King => 18,
                // Role::Sailor => 19,
                // Role::Marionette => 20,
                // Role::Engineer => 21,
                // Role::Preacher => 22,
                // Role::Lilmonsta => 23,
                // Role::Lleech => 24,
                // Role::Xaan => 25,
                // Role::Poisoner => 26,
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
                Role::Washerwoman => 45,
                Role::Librarian => 46,
                Role::Investigator => 47,
                Role::Chef => 48,
                Role::Empath => 49,
                Role::Fortuneteller => 50,
                Role::Butler => 51,
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
                Role::Spy => 65,
                // Role::Ogre => 66,
                // Role::Highpriestess => 67,
                // Role::General => 68,
                // Role::Chambermaid => 69,
                // Role::Mathematician => 70,
                // Role::DAWN => 71, TODO: Figure out wtf this means
                // Role::Leviathan => 72,
                // Role::Vizier => 73
                _ => 0,
            };
            if order != 0 {
                order_map.insert(order, index);
            }
        }

        let mut final_order: Vec<usize> = vec![];
        // Pull out the minimum number role and put it into vector until all roles are ordered
        while order_map.keys().len() != 0 {
            let min_key = *order_map
                .keys()
                .min()
                .expect("There should be an minimum in the map");
            let next_role = order_map.remove(&min_key).unwrap();
            final_order.push(next_role);
        }

        // Return the new vector
        return final_order;
    }

    fn resolve_night_1_ability(&self, player_index: usize) {
        // Check if the role is active before resolving their ability, skip if the role is
        // inactive, but also warn
        // eprintln!("An inactive role tried to act during the night");
        // TODO: Implement abilities for every role
        let player = &self.players[player_index];
        let role = player.role;
        match role {
            Role::Investigator => {
                // WARNING: Can't actually resolve this, this should be decided during setup
                todo!()
            }
            Role::Empath => todo!(),
            Role::Gossip => todo!(),
            Role::Innkeeper => todo!(),
            Role::Washerwoman => todo!(),
            Role::Librarian => todo!(),
            Role::Chef => todo!(),
            Role::Fortuneteller => todo!(),
            Role::Undertaker => todo!(),
            Role::Virgin => {
                // Add a status effect that if someone nominates you, they die
                // Maybe instead add this to the nominate method
                todo!()
            }
            Role::Soldier => todo!(),
            Role::Slayer => todo!(),
            Role::Mayor => todo!(),
            Role::Monk => todo!(),
            Role::Drunk => todo!(),
            Role::Saint => todo!(),
            Role::Butler => todo!(),
            Role::Recluse => todo!(),
            Role::Spy => todo!(),
            Role::Baron => todo!(),
            Role::Scarletwoman => todo!(),
            Role::Poisoner => todo!(),
            Role::Imp => todo!(),
            Role::Ravenkeeper => todo!(),
        }

        // TODO: The event should be logged at some point
    }

    fn resolve_day(&self) {
        // Only a few roles act during the day, and the storyteller only really needs to mark
        // whether someone claimed something
        // Some roles like savant come to the story teller during the day, the story teller should
        // have options for all such roles in the game. These options should be shown all at once,
        // (Like "these roles may come up to you today/ act during the day")
        // and the storyteller should be able to quickly log that these events happened
        //
        // FIX: For now, this method will just do nothing. The functionality for it can be
        // implemented later
        todo!();
    }

    fn resolve_night(&self) {
        // TODO: Implement this method
        // Order the roles in this game to get the order they should be woken up in (should be
        // different from night 1)
        // Wake each role up in order and show them the information they need to know, or the
        // choices that they get
        // For each choice:
        //      If that choice impacts the game state, change the game state accordingly
        //      If that choice tells the player info, give them that info
        //      Should be calling a generic method on the role class to get info on the role's
        //      ability
        // Once you have gone through all the roles, nothing to do: wake everyone up
        todo!();
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Alignment {
    Good,
    Evil,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CharacterType {
    Townsfolk,
    Outsider,
    Minion,
    Demon,
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

    fn get_type(&self) -> CharacterType {
        match *self {
            Role::Investigator
            | Role::Empath
            | Role::Gossip
            | Role::Innkeeper
            | Role::Washerwoman
            | Role::Librarian
            | Role::Chef
            | Role::Fortuneteller
            | Role::Undertaker
            | Role::Virgin
            | Role::Soldier
            | Role::Slayer
            | Role::Mayor
            | Role::Monk
            | Role::Ravenkeeper => CharacterType::Townsfolk,
            Role::Drunk | Role::Saint | Role::Butler | Role::Recluse => CharacterType::Outsider,
            Role::Spy | Role::Baron | Role::Scarletwoman | Role::Poisoner => CharacterType::Minion,
            Role::Imp => CharacterType::Demon,
        }
    }

    fn is_win_condition(&self) -> bool {
        match *self {
            Role::Imp => true,
            _ => false,
        }
    }
}

struct StatusEffect {
    status_type: StatusEffects,
    inflicting_role: Role,
    inflicting_player_index: usize,
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
    ability_active: bool, // WARNING: Not too happy about this implementation, might want to make
    // it cleaner
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
            ability_active: true,
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

        let game = Game::new(roles.clone(), player_names).unwrap();

        assert_eq!(game.players.len(), 3);
        assert_eq!(game.players[1].name, "P2");

        {
            let mut roles = roles.clone();
            for player in game.players {
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

        let mut game = Game::new(roles, player_names).unwrap();

        game.kill_player(0);
        assert_eq!(game.players[0].dead, true);
        game.kill_player(1);
        assert_eq!(game.players[1].dead, true);
        game.kill_player(2);
        assert_eq!(game.players[2].dead, true);
    }

    #[test]
    fn kill_protected_player() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut game = Game::new(roles, player_names).unwrap();

        game.players[1].add_status(StatusEffects::Protected);

        game.kill_player(0);
        assert_eq!(game.players[0].dead, true);
        game.kill_player(1);
        assert_eq!(game.players[1].dead, false);
        game.kill_player(2);
        assert_eq!(game.players[2].dead, true);

        game.players[1].remove_status(StatusEffects::Protected);
        game.kill_player(1);
        assert_eq!(game.players[1].dead, true);
    }

    #[test]
    fn test_left() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let game = Game::new(roles.clone(), player_names).unwrap();

        assert_eq!(
            game.left_player(&game.players[2]).name,
            game.players[1].name
        );

        // Kill set the left player to dead and see that the left player is updated accordingly
        todo!();
    }

    #[test]
    fn test_right() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let game = Game::new(roles, player_names).unwrap();

        assert_eq!(
            game.left_player(&game.players[2]).name,
            game.players[1].name
        );

        // Kill the right player and make sure the right player is updated accordingly
        todo!();
    }
}
