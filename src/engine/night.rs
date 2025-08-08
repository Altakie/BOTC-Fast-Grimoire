// TODO: Realistically we are facing a couple major issues right now that are stopping us from
// implementing abilites
// 1. A change type that doesn't require storyteller intervention and is skipped by the interface
//    when it is detected (nothing pops up, the change is just applied)
//    - Really easy, just add another change type and have the next button detect it, apply the
//    change func, and skip to the next change effect without stopping
// 2. Some abilites need the log to be implemented so they can scan it
// 3. Some abilities need change effects to be able to be chained, but also somehow share
//    information
//  TODO: Think of clever solutions for all of these

use crate::{
    engine::{
        change_request::{ChangeArgs, ChangeRequest, ChangeType},
        player::{Alignment, roles::Roles},
        state::{PlayerIndex, State, status_effects::StatusType},
    },
    new_change_request, unwrap_args_err, unwrap_args_panic,
};

fn get_role_order_night1(role: Roles) -> usize {
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
        Roles::Poisoner => 26,
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
        Roles::Washerwoman => 45,
        Roles::Librarian => 46,
        Roles::Investigator => 47,
        Roles::Chef => 48,
        Roles::Empath => 49,
        Roles::Fortuneteller => 50,
        Roles::Butler => 51,
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
        Roles::Spy => 65,
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
        let prev_player_order = {
            match previous_player {
                Some(player_index) => self.get_player(player_index).role.night_one_order(),
                None => None,
            }
        };
        let mut next_player: Option<(PlayerIndex, usize)> = None;

        // TODO: Check for special roles

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

    // pub(crate) fn get_night_order(&self, player_indices: Vec<PlayerIndex>) -> Vec<PlayerIndex> {
    //     // Go through all roles and assign each of them a number
    //     // Maps night_order to player index
    //     let mut order_map: HashMap<usize, PlayerIndex> = HashMap::new();
    //     for index in player_indices {
    //         let role = self.players[index].role;
    //         let order: usize = match role {
    //             // TODO: make this work
    //
    //             // Role::DUSK => 0,
    //             // Role::Barista => 1,
    //             // Role::Bureaucrat => 2,
    //             // Role::Thief => 3,
    //             // Role::Harlot => 4,
    //             // Role::Bonecollector => 5,
    //             // Role::Philosopher => 6,
    //             // Role::Poppygrower => 7,
    //             // Role::Sailor => 8,
    //             // Role::Engineer => 9,
    //             // Role::Preacher => 10,
    //             // Role::Xaan => 11,
    //             Roles::Poisoner => 12,
    //             // Role::Courtier => 13,
    //             Roles::Innkeeper => 14,
    //             // Role::Wizard => 15,
    //             // Role::Gambler => 16,
    //             // Role::Acrobat => 17,
    //             // Role::Snakecharmer => 18,
    //             Roles::Monk => 19,
    //             // Role::Organgrinder => 20,
    //             // Role::Devilsadvocate => 21,
    //             // Role::Witch => 22,
    //             // Role::Cerenovus => 23,
    //             // Role::Pithag => 24,
    //             // Role::Fearmonger => 25,
    //             // Role::Harpy => 26,
    //             // Role::Mezepheles => 27,
    //             Roles::Scarletwoman => 28,
    //             // Role::Summoner => 29,
    //             // Role::Lunatic => 30,
    //             // Role::Exorcist => 31,
    //             // Role::Lycanthrope => 32,
    //             // Role::Legion => 33,
    //             Roles::Imp => 34,
    //             // Role::Zombuul => 35,
    //             // Role::Pukka => 36,
    //             // Role::Shabaloth => 37,
    //             // Role::Po => 38,
    //             // Role::Fanggu => 39,
    //             // Role::Nodashii => 40,
    //             // Role::Vortox => 41,
    //             // Role::Lordoftyphon => 42,
    //             // Role::Vigormortis => 43,
    //             // Role::Ojo => 44,
    //             // Role::Alhadikhia => 45,
    //             // Role::Lleech => 46,
    //             // Role::Lilmonsta => 47,
    //             // Role::Yaggababble => 48,
    //             // Role::Kazali => 49,
    //             // Role::Assassin => 50,
    //             // Role::Godfather => 51,
    //             // Role::Gossip => 52,
    //             // Role::Hatter => 53,
    //             // Role::Barber => 54,
    //             // Role::Sweetheart => 55,
    //             // Role::Sage => 56,
    //             // Role::Banshee => 57,
    //             // Role::Professor => 58,
    //             // Role::Choirboy => 59,
    //             // Role::Huntsman => 60,
    //             // Role::Damsel => 61,
    //             // Role::Amnesiac => 62,
    //             // Role::Farmer => 63,
    //             // Role::Tinker => 64,
    //             // Role::Moonchild => 65,
    //             // Role::Grandmother => 66,
    //             Roles::Ravenkeeper => 67,
    //             Roles::Empath => 68,
    //             Roles::Fortuneteller => 69,
    //             Roles::Undertaker => 70,
    //             // Role::Dreamer => 71,
    //             // Role::Flowergirl => 72,
    //             // Role::Towncrier => 73,
    //             // Role::Oracle => 74,
    //             // Role::Seamstress => 75,
    //             // Role::Juggler => 76,
    //             // Role::Balloonist => 77,
    //             // Role::Villageidiot => 78,
    //             // Role::King => 79,
    //             // Role::Bountyhunter => 80,
    //             // Role::Nightwatchman => 81,
    //             // Role::Cultleader => 82,
    //             Roles::Butler => 83,
    //             Roles::Spy => 84,
    //             // Role::Highpriestess => 85,
    //             // Role::General => 86,
    //             // Role::Chambermaid => 87,
    //             // Role::Mathematician => 88,
    //             // Role::DAWN => 89, //TODO: Figure this out
    //             // Role::Leviathan => 90,
    //             _ => 0,
    //         };
    //         if order != 0 {
    //             order_map.insert(order, index);
    //         }
    //     }
    //
    //     return self.get_order_from_map(order_map);
    // }
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
