#![allow(dead_code, unused_variables, clippy::needless_return, unreachable_code)]

use game::{CharacterType, Role};
use setup::{PlayerCounts, Script, ScriptJson};
use std::{io, usize};

mod game;
mod setup;

// TODO: Communication between frontend and backend
// Also need a generic method to show something to the storyteller
// Could be json that I deserialize?
// Or could just be a string that I parse
// Either way this is something that should be sent from the frontend to the backend, the backend
// should be prompting the frontend for specific things at certain points in the game, as well as
// sending events to the frontend. Frontend and backend can be on same machine or different machine
// but there should only be one backend server at a time. All requests should go to the backend
// server and all logic should be handled there, but state should be synchronized across backend
// and frontend.
// I really want there to be a phone version of the application that is portable and makes logging
// stuff way faster as the storyteller. But this needs to sync with the desktop version. For now
// just make desktop version.
// TODO:
// Should have an option to spin up your machine as a new master (handles front end and backend and
// starts a new game), or connect to an old master. Your masters should be associated with your
// account. Not quite sure how to do this yet.
//

fn main() {
    // -- Setup --
    //  First need to have the story teller upload a script with a list of roles
    // Should make sure these roles are implemented before starting the game

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

    let mut game = game::Game::new(roles, player_names).unwrap();

    // Set up the game depending on certain roles
    game.setup();

    // -- Night 1 --
    // Storyteller should give out all roles to players (game not needed here)
    // Can make a checklist of all players so storyteller can keep track of who has gotten their
    // roles
    // Game should tell storyteller to introduce demons and minions to each other (might want to
    // include this event in the night order)
    // Game should provide a night 1 specific order based on the roles that are in play (function
    // call)
    // Game should go through this night 1 specific order, providing the appropriate information,
    // or waiting for the appropraite input from the storyteller (through the player), waiting for the storyteller to mark each
    // step as resolved

    game.resolve_night_1();
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

#[cfg(test)]
mod tests {
    // -- Other Tests --
    #[test]
    fn new_script_from_json() {
        // Pass in valid script
        todo!()
    }

    #[should_panic]
    #[test]
    fn invalid_script_from_json() {
        todo!()
    }
}
