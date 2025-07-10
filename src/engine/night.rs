use crate::{
    engine::{
        change_request::{ChangeArgs, ChangeRequest, ChangeType},
        player::{Alignment, Role},
        state::{PlayerIndex, State, status_effects::StatusType},
    },
    new_change_request, unwrap_args_err, unwrap_args_panic,
};
use std::collections::HashMap;

fn get_role_order_night1(role: Role) -> usize {
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
        Role::Poisoner => 26,
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
    }
}
impl State {
    pub(crate) fn get_next_active_night1(
        &self,
        previous_player: Option<PlayerIndex>,
    ) -> Option<PlayerIndex> {
        let prev_player_order = self.get_night_1_order(previous_player);
        let mut next_player: Option<(PlayerIndex, usize)> = None;

        // TODO: Check for special roles

        for (player_index, _player) in self.players.iter().enumerate() {
            let order = self.get_night_1_order(Some(player_index));
            // Check that the player acts at night
            let order = match order {
                Some(order) => order,
                None => continue,
            };
            if prev_player_order.is_some() {
                let prev_player_order = prev_player_order.unwrap();
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

    pub(crate) fn get_night_1_order(
        &self,
        player_index: Option<PlayerIndex>,
    ) -> Option<PlayerIndex> {
        let player_index = player_index?;
        let role = self.get_acting_role(player_index);
        let order = match role {
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
            Role::Poisoner => 26,
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
            _ => return None,
        };

        return Some(order);
    }

    // pub(crate) fn get_night_1_order(&self, player_indices: Vec<PlayerIndex>) -> Vec<PlayerIndex> {
    //     // Go through all roles and assign each of them a number
    //     // Maps night_order to player index
    //     let mut order_map: HashMap<usize, PlayerIndex> = HashMap::new();
    //     for index in player_indices {
    //         let role = self.players[index].role;
    //         let order: usize = match role {
    //             // Role::DUSK => 0,
    //             // Role::Lordoftyphon => 1,
    //             // Role::Kazali => 2,
    //             // Role::Apprentice => 3,
    //             // Role::Barista => 4,
    //             // Role::Bureaucrat => 5,
    //             // Role::Thief => 6,
    //             // Role::Boffin => 7,
    //             // Role::Philosopher => 8,
    //             // Role::Alchemist => 9,
    //             // Role::Poppygrower => 10,
    //             // Role::Yaggababble => 11,
    //             // Role::Magician => 12,
    //             // Role::MINION => 13, TODO: Need to implement this shit
    //             // Role::Snitch => 14,
    //             // Role::Lunatic => 15,
    //             // Role::Summoner => 16,
    //             // Role::DEMON => 17, TODO: Need to implement this shit
    //             // Role::King => 18,
    //             // Role::Sailor => 19,
    //             // Role::Marionette => 20,
    //             // Role::Engineer => 21,
    //             // Role::Preacher => 22,
    //             // Role::Lilmonsta => 23,
    //             // Role::Lleech => 24,
    //             // Role::Xaan => 25,
    //             Role::Poisoner => 26,
    //             // Role::Widow => 27,
    //             // Role::Courtier => 28,
    //             // Role::Wizard => 29,
    //             // Role::Snakecharmer => 30,
    //             // Role::Godfather => 31,
    //             // Role::Organgrinder => 32,
    //             // Role::Devilsadvocate => 33,
    //             // Role::Eviltwin => 34,
    //             // Role::Witch => 35,
    //             // Role::Cerenovus => 36,
    //             // Role::Fearmonger => 37,
    //             // Role::Harpy => 38,
    //             // Role::Mezepheles => 39,
    //             // Role::Pukka => 40,
    //             // Role::Pixie => 41,
    //             // Role::Huntsman => 42,
    //             // Role::Damsel => 43,
    //             // Role::Amnesiac => 44,
    //             Role::Washerwoman => 45,
    //             Role::Librarian => 46,
    //             Role::Investigator => 47,
    //             Role::Chef => 48,
    //             Role::Empath => 49,
    //             Role::Fortuneteller => 50,
    //             Role::Butler => 51,
    //             // Role::Grandmother => 52,
    //             // Role::Clockmaker => 53,
    //             // Role::Dreamer => 54,
    //             // Role::Seamstress => 55,
    //             // Role::Steward => 56,
    //             // Role::Knight => 57,
    //             // Role::Noble => 58,
    //             // Role::Balloonist => 59,
    //             // Role::Shugenja => 60,
    //             // Role::Villageidiot => 61,
    //             // Role::Bountyhunter => 62,
    //             // Role::Nightwatchman => 63,
    //             // Role::Cultleader => 64,
    //             Role::Spy => 65,
    //             // Role::Ogre => 66,
    //             // Role::Highpriestess => 67,
    //             // Role::General => 68,
    //             // Role::Chambermaid => 69,
    //             // Role::Mathematician => 70,
    //             // Role::DAWN => 71, TODO: Figure out wtf this means
    //             // Role::Leviathan => 72,
    //             // Role::Vizier => 73
    //             _ => 0,
    //         };
    //         if order != 0 {
    //             order_map.insert(order, index);
    //         }
    //     }
    //
    //     return self.get_order_from_map(order_map);
    // }

    pub(crate) fn get_night_order(&self, player_indices: Vec<PlayerIndex>) -> Vec<PlayerIndex> {
        // Go through all roles and assign each of them a number
        // Maps night_order to player index
        let mut order_map: HashMap<usize, PlayerIndex> = HashMap::new();
        for index in player_indices {
            let role = self.players[index].role;
            let order: usize = match role {
                // TODO: make this work

                // Role::DUSK => 0,
                // Role::Barista => 1,
                // Role::Bureaucrat => 2,
                // Role::Thief => 3,
                // Role::Harlot => 4,
                // Role::Bonecollector => 5,
                // Role::Philosopher => 6,
                // Role::Poppygrower => 7,
                // Role::Sailor => 8,
                // Role::Engineer => 9,
                // Role::Preacher => 10,
                // Role::Xaan => 11,
                Role::Poisoner => 12,
                // Role::Courtier => 13,
                Role::Innkeeper => 14,
                // Role::Wizard => 15,
                // Role::Gambler => 16,
                // Role::Acrobat => 17,
                // Role::Snakecharmer => 18,
                Role::Monk => 19,
                // Role::Organgrinder => 20,
                // Role::Devilsadvocate => 21,
                // Role::Witch => 22,
                // Role::Cerenovus => 23,
                // Role::Pithag => 24,
                // Role::Fearmonger => 25,
                // Role::Harpy => 26,
                // Role::Mezepheles => 27,
                Role::Scarletwoman => 28,
                // Role::Summoner => 29,
                // Role::Lunatic => 30,
                // Role::Exorcist => 31,
                // Role::Lycanthrope => 32,
                // Role::Legion => 33,
                Role::Imp => 34,
                // Role::Zombuul => 35,
                // Role::Pukka => 36,
                // Role::Shabaloth => 37,
                // Role::Po => 38,
                // Role::Fanggu => 39,
                // Role::Nodashii => 40,
                // Role::Vortox => 41,
                // Role::Lordoftyphon => 42,
                // Role::Vigormortis => 43,
                // Role::Ojo => 44,
                // Role::Alhadikhia => 45,
                // Role::Lleech => 46,
                // Role::Lilmonsta => 47,
                // Role::Yaggababble => 48,
                // Role::Kazali => 49,
                // Role::Assassin => 50,
                // Role::Godfather => 51,
                // Role::Gossip => 52,
                // Role::Hatter => 53,
                // Role::Barber => 54,
                // Role::Sweetheart => 55,
                // Role::Sage => 56,
                // Role::Banshee => 57,
                // Role::Professor => 58,
                // Role::Choirboy => 59,
                // Role::Huntsman => 60,
                // Role::Damsel => 61,
                // Role::Amnesiac => 62,
                // Role::Farmer => 63,
                // Role::Tinker => 64,
                // Role::Moonchild => 65,
                // Role::Grandmother => 66,
                Role::Ravenkeeper => 67,
                Role::Empath => 68,
                Role::Fortuneteller => 69,
                Role::Undertaker => 70,
                // Role::Dreamer => 71,
                // Role::Flowergirl => 72,
                // Role::Towncrier => 73,
                // Role::Oracle => 74,
                // Role::Seamstress => 75,
                // Role::Juggler => 76,
                // Role::Balloonist => 77,
                // Role::Villageidiot => 78,
                // Role::King => 79,
                // Role::Bountyhunter => 80,
                // Role::Nightwatchman => 81,
                // Role::Cultleader => 82,
                Role::Butler => 83,
                Role::Spy => 84,
                // Role::Highpriestess => 85,
                // Role::General => 86,
                // Role::Chambermaid => 87,
                // Role::Mathematician => 88,
                // Role::DAWN => 89, //TODO: Figure this out
                // Role::Leviathan => 90,
                _ => 0,
            };
            if order != 0 {
                order_map.insert(order, index);
            }
        }

        return self.get_order_from_map(order_map);
    }
}

impl Role {
    pub(crate) fn resolve_night_1_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<Vec<ChangeRequest>> {
        // Check if the role is active before resolving their ability, skip if the role is
        // inactive, but also warn
        // eprintln!("An inactive role tried to act during the night");
        // NOTE: I think that for info roles, the storyteller should still receive the correct
        // info, but there will be a warning that the player is poisoned on the screen somewhere,
        // letting the storyteller decide what number they should give
        // TODO: Implement abilities for every role
        match self {
            Role::Investigator => Some(washerwoman_librarian_investigator_ability(*self)),
            Role::Empath => Some(empath_ability(state, player_index)),
            // Role::Gossip => todo!(),      // Should wait till v2
            // Role::Innkeeper => todo!(),   // Should wait till v2
            Role::Washerwoman => Some(washerwoman_librarian_investigator_ability(*self)),
            Role::Librarian => Some(washerwoman_librarian_investigator_ability(*self)),
            Role::Chef => Some(chef_ability(state)),
            Role::Fortuneteller => Some(fortuneteller_ability(state)),
            Role::Drunk => {
                let role = state.get_acting_role(player_index);
                role.resolve_night_1_ability(player_index, state)
            }
            Role::Butler => Some(butler_ability(player_index)),
            Role::Spy => {
                // Just tell the storyteller to let the spy look at the grimoire
                Some(spy_ability())
            }
            Role::Poisoner => Some(poisoner_ability(player_index)), // Add poison to someone until next night, same as
            // normal ability
            _ => None,
        }

        // TODO: The event should be logged at some point
    }
}

// pub(crate) fn resolve_night(&mut self) {
//     // TODO: Implement this method
//     // Order the roles in this game to get the order they should be woken up in (should be
//     // different from night 1)
//     // Wake each role up in order and show them the information they need to know, or the
//     // choices that they get
//     // For each choice:
//     //      If that choice impacts the game state, change the game state accordingly
//     //      If that choice tells the player info, give them that info
//     //      Should be calling a generic method on the role class to get info on the role's
//     //      ability
//     // Once you have gone through all the roles, nothing to do: wake everyone up
//     self.day_phase = DayPhase::Night;
//     // Order the roles in this game to get the order they should be woken up in (should be
//     // unique to night 1)
//     let ordered_player_indices = self.get_night_order(self.get_active_roles());
//     // Wake each role up in order and show them the information they need to know, or the
//     // choices that they get
//     // For each choice:
//     //      If that choice impacts the game state, change the game state accordingly
//     //      If that choice tells the player info, give them that info
//     //      Should be calling a generic method on the role class to get info on the role's
//     //      ability
//     // Once you have gone through all the roles, nothing to do: wake everyone up
//     for i in ordered_player_indices.iter() {
//         self.resolve_night_ability(*i);
//     }
// }
//
// pub(crate) fn resolve_night_ability(&mut self, player_index: PlayerIndex) {
//     // Check if the role is active before resolving their ability, skip if the role is
//     // inactive, but also warn
//     // eprintln!("An inactive role tried to act during the night");
//     // NOTE: I think that for info roles, the storyteller should still receive the correct
//     // info, but there will be a warning that the player is poisoned on the screen somewhere,
//     // letting the storyteller decide what number they should give
//     // TODO: Implement abilities for every role
//     let player = &mut self.players[player_index];
//     let role = player.role;
//     match role {
//         Role::Empath => {
//             let count = self.empath_ability(player_index);
//             // For now, just print output
//             println!("Empath count: {}", count);
//         }
//         Role::Gossip => todo!(),    // wait for v2
//         Role::Innkeeper => todo!(), // Wait for v2
//         Role::Fortuneteller => todo!(),
// Role::Undertaker => {
//     // TODO: Should scan the log for the entire day yesterday
//     // If there was a execution event yesterday that resulted in death, grab the player
//     // from that event
//     // Grab that player's role and give it to the undertaker
//     todo!();
// }
//         Role::Monk => {
//             // Give the target the demon protected status effect
//             // TODO: Prompt the storyteller to choose a player
//             let target_index = todo!();
//             self.add_status(StatusEffects::DemonProtected, player_index, target_index);
//         }
//         Role::Ravenkeeper => {
//             // TODO:
//             // Should only happen when the ravenkeeper is dead
//             // Perhaps check every night if ravenkeeper is dead, or was killed that night?
//             // After death, prompt storyteller to choose player
//             let target_index: PlayerIndex = todo!();
//             let role = self.players[target_index].role;
//         }
//         Role::Butler => {
//             // TODO:
//             // Prompt the storyteller to choose a player
//             let target_index: PlayerIndex = self.choose_players(1)[0];
//             self.add_status(StatusEffects::ButlerMaster, player_index, target_index);
//         }
//         Role::Spy => {
//             // TODO: Literally just let them look at the grimoire
//             // End the phase when they're done looking at the grimoire
//         }
//         Role::Scarletwoman => {
//             // TODO: Check if the demon is dead at that point and there are more than 5 players
//             // Scarlet woman becomes the demon, should actually become the demon before this,
//             // but this is when they should be notified
//         }
//         Role::Poisoner => {
//             // TODO: Poison someone
//         }
//         Role::Imp => {
//             // TODO: Kill someone, if your target is yourself, kill yourself but transfer demon
//             // to a minion
//             // How to transfer demon to minion? Let storyteller decide. Prompt the storyteller
//             // to choose a player. Validate that the player is a minion, if they aren't, prompt
//             // them to choose again. If there are no minions in play, don't even give them the
//             // option
//         }
//         _ => {
//             eprintln!("A role that wasn't supposed to act acted");
//             panic!()
//         }
//     }
//
//     // TODO: Method should wait until storyteller explicitly advances to the next phase
//
//     // TODO: The event should be logged at some point
// }

// NOTE: Role Specific Abilities
fn empath_ability(state: &State, player_index: PlayerIndex) -> Vec<ChangeRequest> {
    // Check how many players next to the empath are evil
    let mut count = 0;
    {
        let left_player = &state.players[state.left_player(player_index)];
        if left_player.alignment == Alignment::Evil {
            count += 1;
        }
    }
    {
        let right_player = &state.players[state.right_player(player_index)];
        if right_player.alignment == Alignment::Evil {
            count += 1;
        }
    }
    let message = format!("Empath has {} evil neighbors", count);

    let change_type = ChangeType::Display;

    vec![new_change_request!(change_type, message)]
}

fn washerwoman_librarian_investigator_ability(role: Role) -> Vec<ChangeRequest> {
    // TODO: Perhaps need find status method, or highlight the players with these statuses
    let message = format!("Show {} the correct roles", role);
    let change_type = ChangeType::Display;

    vec![new_change_request!(change_type, message)]
}

fn chef_ability(state: &State) -> Vec<ChangeRequest> {
    // Count pairs of evil players
    // For each evil, player, check if the right player is evil, if yes, increment the
    // pair count
    let change_type = ChangeType::Display;
    let mut pair_count = 0;

    for player_index in 0..state.players.len() {
        let player = &state.players[player_index];
        if player.alignment != Alignment::Evil {
            continue;
        }
        let right_player = &state.players[state.right_player(player_index)];
        if right_player.alignment == Alignment::Evil {
            pair_count += 1;
        }
    }
    let message = format!(
        "Show the chef that there are {} pairs of evil players",
        pair_count
    );

    vec![new_change_request!(change_type, message)]
}

fn fortuneteller_ability(state: &State) -> Vec<ChangeRequest> {
    // TODO: Prompt for a choice of two players
    // Should yield True or false based on whether at least one of those players is a demon or has the red
    // herring status effect
    // Chained change effects, but also need a way to communicate between them?
    // Maybe don't clear selected players between change effects unless specified?
    // Could add bool for this
    // Not exactly actually, message should switch when two players are selected?

    let message1 = "Prompt the FortuneTeller to point to two players";

    let change_type1 = ChangeType::ChoosePlayers(2);
    let change_type2 = ChangeType::Display;

    // let check_func = move |state, args| {};

    // let state_change_func = move |state, args| {};

    todo!()
}

fn butler_ability(player_index: PlayerIndex) -> Vec<ChangeRequest> {
    // Clean up the old butler master status effect (if there is one), prompt for another
    // player, and give them the butler master status effect

    let message = "Prompt the butler to pick a player to be their master".to_string();
    let change_type = ChangeType::ChoosePlayers(1);

    let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
        let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_players.len() != 1 {
            return Err(());
        }

        // Check that the butler is not picking themselves
        if target_players[0] == player_index {
            return Ok(false);
        }

        return Ok(true);
    };

    let state_change_func = move |state: &mut State, args: ChangeArgs| {
        // Check if there are any butler master status effects inflicted by this player and clear
        // them
        let prev_effects = state.get_inflicted_statuses(player_index);

        let prev_effect = prev_effects
            .iter()
            .find(|se| se.status_type == StatusType::ButlerMaster);

        if let Some(prev_effect) = prev_effect {
            state.remove_status(
                prev_effect.status_type,
                prev_effect.source_player_index,
                prev_effect.affected_player_index,
            );
        }

        let target_player_index = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv)[0];
        state.add_status(StatusType::ButlerMaster, player_index, target_player_index);
    };

    vec![new_change_request!(
        change_type,
        message,
        check_func,
        state_change_func
    )]
}

fn spy_ability() -> Vec<ChangeRequest> {
    let change_type = ChangeType::Display;
    let message = "Show the Spy the grimoire".to_string();

    vec![new_change_request!(change_type, message)]
}

// TODO: Merge abilities that add a new status every night (monk, poisoner, )
fn poisoner_ability(player_index: PlayerIndex) -> Vec<ChangeRequest> {
    // Clean up the old poisoned effect, prompt for another
    // player, and give them the poisoned effect

    let message = "Prompt the poisoner to pick a player to poison".to_string();
    let change_type = ChangeType::ChoosePlayers(1);

    let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
        let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_players.len() != 1 {
            return Err(());
        }

        return Ok(true);
    };

    let state_change_func = move |state: &mut State, args: ChangeArgs| {
        // Check if there are any poisoned status effects inflicted by this player and clear
        // them
        let prev_effects = state.get_inflicted_statuses(player_index);

        let prev_effect = prev_effects
            .iter()
            .find(|se| se.status_type == StatusType::Poisoned);

        if let Some(prev_effect) = prev_effect {
            state.remove_status(
                prev_effect.status_type,
                prev_effect.source_player_index,
                prev_effect.affected_player_index,
            );
        }

        let target_player_index = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv)[0];
        state.add_status(StatusType::Poisoned, player_index, target_player_index);
    };

    vec![new_change_request!(
        change_type,
        message,
        check_func,
        state_change_func
    )]
}

fn monk_ability(player_index: PlayerIndex) -> Vec<ChangeRequest> {
    let change_type = ChangeType::ChoosePlayers(1);
    let message = "Have the monk select a player to protect".to_string();

    let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
        let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_players.len() != 1 {
            return Err(());
        }

        // Make sure the monk can't protect themselves
        if target_players[0] == player_index {
            return Ok(false);
        }

        return Ok(true);
    };

    let state_change_func = move |state: &mut State, args: ChangeArgs| {
        // Check if there are any poisoned status effects inflicted by this player and clear
        // them
        let prev_effects = state.get_inflicted_statuses(player_index);

        let prev_effect = prev_effects
            .iter()
            .find(|se| se.status_type == StatusType::DemonProtected);

        if let Some(prev_effect) = prev_effect {
            state.remove_status(
                prev_effect.status_type,
                prev_effect.source_player_index,
                prev_effect.affected_player_index,
            );
        }

        let target_player_index = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv)[0];
        state.add_status(
            StatusType::DemonProtected,
            player_index,
            target_player_index,
        );
    };

    vec![new_change_request!(
        change_type,
        message,
        check_func,
        state_change_func
    )]
}

// TODO: Most demons kill, so maybe add a generic demon kill ability at some point
fn imp_ability(player_index: PlayerIndex) -> Vec<ChangeRequest> {
    let change_type = ChangeType::ChoosePlayers(1);
    let message = "".to_string();

    let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ()> {
        let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        if target_players.len() != 1 {
            return Err(());
        }

        return Ok(true);
    };

    let state_change_func = move |state: &mut State, args: ChangeArgs| {
        // Check if there are any poisoned status effects inflicted by this player and clear
        // them
        let prev_effects = state.get_inflicted_statuses(player_index);

        let prev_effect = prev_effects
            .iter()
            .find(|se| se.status_type == StatusType::DemonProtected);

        if let Some(prev_effect) = prev_effect {
            state.remove_status(
                prev_effect.status_type,
                prev_effect.source_player_index,
                prev_effect.affected_player_index,
            );
        }

        let target_player_index = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv)[0];
        state.add_status(
            StatusType::DemonProtected,
            player_index,
            target_player_index,
        );
    };

    vec![new_change_request!(
        change_type,
        message,
        check_func,
        state_change_func
    )]
}

#[cfg(test)]
mod tests {
    use crate::{
        Role,
        engine::{
            night::{chef_ability, empath_ability},
            state::{PlayerIndex, tests::setup_test_game},
        },
    };
    #[test]
    fn test_get_order() {
        let game = setup_test_game().0;

        let mut next_player_index = None;

        let mut assert_next_role = |role: Role| {
            next_player_index = game.get_next_active_night1(next_player_index);
            let role_pos = game.players.iter().position(|p| p.role == role).unwrap();
            assert_eq!(
                next_player_index.unwrap(),
                role_pos,
                "Next Player Role: {}\n {}'s Position is {}",
                game.players[next_player_index.unwrap()].role,
                role,
                role_pos
            );
        };

        assert_next_role(Role::Poisoner);
        assert_next_role(Role::Investigator);
        assert_next_role(Role::Chef);

        next_player_index = game.get_next_active_player(next_player_index);
        assert!(next_player_index.is_none());
    }

    #[test]
    fn test_empath_ability() {
        let test_cases = [
            (
                "Empath 0 evil neighbors",
                vec![Role::Investigator, Role::Empath, Role::Saint],
                vec![],
                0,
            ),
            (
                "Empath dead right neighbor",
                vec![
                    Role::Investigator,
                    Role::Empath,
                    Role::Saint,
                    Role::Poisoner,
                ],
                vec![2],
                1,
            ),
            (
                "Empath dead left neighbor",
                vec![
                    Role::Investigator,
                    Role::Empath,
                    Role::Saint,
                    Role::Chef,
                    Role::Scarletwoman,
                ],
                vec![0],
                1,
            ),
            (
                "Empath right evil neighbor",
                vec![Role::Investigator, Role::Empath, Role::Baron],
                vec![],
                1,
            ),
            (
                "Empath dead right neighbor initially evil",
                vec![
                    Role::Investigator,
                    Role::Empath,
                    Role::Baron,
                    Role::Saint,
                    Role::Washerwoman,
                ],
                vec![2],
                0,
            ),
            (
                "Empath dead right neighbor initially evil, new neighbor also evil",
                vec![
                    Role::Investigator,
                    Role::Empath,
                    Role::Baron,
                    Role::Saint,
                    Role::Washerwoman,
                ],
                vec![2],
                0,
            ),
            (
                "Empath left evil neighbor",
                vec![Role::Scarletwoman, Role::Empath, Role::Saint],
                vec![],
                1,
            ),
            (
                "Empath dead left evil neighbor initially evil",
                vec![
                    Role::Scarletwoman,
                    Role::Empath,
                    Role::Saint,
                    Role::Chef,
                    Role::Investigator,
                ],
                vec![0],
                0,
            ),
            (
                "Empath dead left evil neighbor initially evil, new neighbor also evil",
                vec![
                    Role::Scarletwoman,
                    Role::Empath,
                    Role::Saint,
                    Role::Chef,
                    Role::Poisoner,
                ],
                vec![0],
                1,
            ),
            (
                "Empath both evil neighbors",
                vec![Role::Poisoner, Role::Empath, Role::Imp],
                vec![],
                2,
            ),
            (
                "Empath initallly both evil neighbors, right dead",
                vec![
                    Role::Poisoner,
                    Role::Empath,
                    Role::Imp,
                    Role::Chef,
                    Role::Investigator,
                ],
                vec![0],
                1,
            ),
            (
                "Empath initallly both evil neighbors, left dead",
                vec![
                    Role::Poisoner,
                    Role::Empath,
                    Role::Imp,
                    Role::Chef,
                    Role::Investigator,
                ],
                vec![2],
                1,
            ),
            (
                "Empath initallly both evil neighbors, both dead",
                vec![
                    Role::Poisoner,
                    Role::Empath,
                    Role::Imp,
                    Role::Chef,
                    Role::Investigator,
                ],
                vec![0, 2],
                0,
            ),
            // TODO: I want the storyteller to be aware in some kind of way that an ability is
            // affecting the information
            // Perhaps duplicates of information passed out?
            // Perhaps just a reminder that something is amiss, to check the roles?
            // Need a cleaner solution for recluse and spy rather than just calling them evil
            // So actually, this is correct, it should give them the truth of the matter, BUT ALSO
            // NOTIFY THE STORYTELLER THAT THE RECLUSE OR SPY CAN AFFECT INFORMATION, GIVING THE
            // STORYTELLER THE CHOICE TO DO THAT IF THEY WISH
            // Maybe add a star to the message and highlight the relevant statuses on select?
            // Perhaps each change effect should also come with statues to highlight?
            (
                "Empath recluse evil neighbor",
                vec![Role::Investigator, Role::Empath, Role::Recluse],
                vec![],
                0,
            ),
            (
                "Empath spy evil neighbor",
                vec![Role::Spy, Role::Empath, Role::Investigator],
                vec![],
                1,
            ),
        ];

        for test_case in test_cases {
            // Create clean environment for each test
            let mut game = setup_test_game().0;
            let mut convert_player = |player_index: PlayerIndex| {
                game.players[player_index].role = test_case.1[player_index];
                game.players[player_index].alignment =
                    test_case.1[player_index].get_default_alignment();
            };

            for (i, _) in test_case.1.iter().enumerate() {
                convert_player(i);
            }

            for i in test_case.2 {
                game.players[i].dead = true;
            }

            let desired_num = test_case.3;

            let empath_message = empath_ability(&game, 1)[0].description.clone();
            let desired_message = format!("Empath has {} evil neighbors", desired_num);
            assert!(
                empath_message == desired_message,
                "{} failed. Expected {} evil neighbors, got {}",
                test_case.0,
                desired_num,
                empath_message
            )
        }
    }

    #[test]
    fn test_chef_ability() {
        let mut game = setup_test_game().0;

        let test_cases = [
            (
                "0 Chef Pairs",
                [
                    Role::Imp,
                    Role::Chef,
                    Role::Spy,
                    Role::Washerwoman,
                    Role::Empath,
                ],
                0,
            ),
            (
                "1 Chef Pair",
                [
                    Role::Chef,
                    Role::Imp,
                    Role::Spy,
                    Role::Washerwoman,
                    Role::Empath,
                ],
                1,
            ),
            (
                "1 Chef Pair with Wrap",
                [
                    Role::Imp,
                    Role::Chef,
                    Role::Washerwoman,
                    Role::Empath,
                    Role::Spy,
                ],
                1,
            ),
            (
                "3 Evil Players, two sitting together other separate",
                [
                    Role::Chef,
                    Role::Imp,
                    Role::Spy,
                    Role::Washerwoman,
                    Role::Poisoner,
                ],
                1,
            ),
            (
                "3 Evil in a row",
                [
                    Role::Imp,
                    Role::Chef,
                    Role::Washerwoman,
                    Role::Baron,
                    Role::Spy,
                ],
                2,
            ),
        ];

        for test_case in test_cases {
            let mut convert_player = |player_index: PlayerIndex| {
                game.players[player_index].role = test_case.1[player_index];
                game.players[player_index].alignment =
                    test_case.1[player_index].get_default_alignment();
            };

            for i in 0..5 {
                convert_player(i);
            }

            let chef_message = chef_ability(&game)[0].description.clone();
            let desired_message = format!(
                "Show the chef that there are {} pairs of evil players",
                test_case.2
            );
            assert!(
                chef_message == desired_message,
                "{} failed. Expected {} pairs of evil players, got {}",
                test_case.0,
                test_case.2,
                chef_message
            )
        }
    }

    // TODO: Test all night abilities (both check funcs and state application funcs)
    // Imp
    // Empath
    // Monk
    // Poisoner
    // Butler
    // Scarletwoman
    // FortuneTeller
    // Ravenkeeper
    // Undertaker
}
