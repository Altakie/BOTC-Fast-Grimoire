use leptos::IntoView;
use rand::{self, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, usize};

use crate::setup::Script;

pub trait StoryTellerInterface {
    // These methods should prompt a user for some input that corresponds to the return type, wait
    // for them to input it, and return the corresponding values
    fn choose_players(&self, num: usize, max_index: usize) -> Vec<PlayerIndex>;
    fn choose_roles(&self, num: usize, valid_roles: Vec<Role>) -> Vec<Role>;
    fn input_number(&self) -> usize;

    // These methods should just display some data for the storyteller/player and wait for them to
    // confirm they are done viewing it
    fn display_number(&self);
    fn display_players(&self);
    fn display_role(&self);
}

pub(crate) enum RoleSelectionType {
    InPlay,
    NotInPlay,
    Script,
}

// -- Game pub(crate) structures --

pub(crate) type PlayerIndex = usize;

pub(crate) struct Game<T: StoryTellerInterface> {
    players: Vec<Player>, // NOTE: Maybe should be a map instead
    // Want to have pointers to players
    win_cond_i: Option<PlayerIndex>,
    status_effects: Vec<StatusEffect>,
    day_phase: DayPhase,
    day_num: usize,
    log: Log,
    storyteller_interface: T,
    script: Script,
}

impl<T: StoryTellerInterface> Game<T> {
    pub(crate) fn new(
        mut roles: Vec<Role>,
        player_names: Vec<String>,
        script: Script,
        storyteller_interface: T,
    ) -> Result<Self, ()> {
        // Make this method conpub(crate) struct a new seating chart
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
        let status_effects: Vec<StatusEffect> = vec![];

        let log = Log::new();
        return Ok(Self {
            players,
            win_cond_i,
            status_effects, // active_roles,
            day_phase: DayPhase::Night,
            day_num: 0,
            log,
            storyteller_interface,
            script,
        });
    }

    pub(crate) fn choose_players(&self, num: usize) -> Vec<usize> {
        self.storyteller_interface
            .choose_players(num, self.players.len())
    }

    pub(crate) fn choose_roles(&self, num: usize, selector: RoleSelectionType) -> Vec<Role> {
        let roles_in_scope: Vec<Role> = match selector {
            RoleSelectionType::InPlay => self.players.iter().clone().map(|p| p.role).collect(),
            RoleSelectionType::NotInPlay => todo!(),
            RoleSelectionType::Script => self.script.roles.clone(),
        };

        self.storyteller_interface.choose_roles(num, roles_in_scope)
    }

    pub(crate) fn setup(&mut self) {
        for player_index in 0..self.players.len() {
            let role = &self.players[player_index].role;

            match role {
                Role::Washerwoman => loop {
                    let target_player_indices: Vec<PlayerIndex> = self.choose_players(2);
                    for target_player_index in target_player_indices {
                        let player = &self.players[target_player_index];
                        if player.role.get_type() == CharacterType::Townsfolk
                            || player.role == Role::Spy
                        {
                            break;
                        }
                    }
                    eprintln!("Storyteller should have selected a townsfolk");
                },
                Role::Librarian => {
                    // TODO: Prompt storyteller to select two players
                    // Check that at least one of those players is a outsider
                    loop {
                        let target_player_indices: Vec<PlayerIndex> = self.choose_players(2);
                        for target_player_index in target_player_indices {
                            let player = self.players[target_player_index].clone();
                            if player.role.get_type() == CharacterType::Outsider
                                || player.role == Role::Spy
                            {
                                break;
                            }
                        }
                        eprintln!("Storyteller should have selected a outsider");
                    }
                }
                Role::Investigator => {
                    // TODO: Prompt storyteller to select two players
                    // Check that at least one of those players is a minion
                    loop {
                        let target_player_indices: Vec<PlayerIndex> = self.choose_players(2);
                        for target_player_index in target_player_indices {
                            let player = self.players[target_player_index].clone();
                            if player.role.get_type() == CharacterType::Minion
                                || player.role == Role::Recluse
                            {
                                break;
                            }
                        }
                        eprintln!("Storyteller should have selected a minion");
                    }
                }
                Role::Drunk => {
                    // TODO: Choose a townsfolk role for the storyteller to replace the drunk with
                    // Swap the chosen role with drunk, but give them a status effect that they
                    // are actually the drunk
                    // Essentially, the drunk should never actually be in play, the actual role
                    // should be swapped out but a note is added that this player is indeed the
                    // drunk
                    let drunk_role = self.choose_roles(1, RoleSelectionType::Script)[0];
                    self.players[player_index].role = drunk_role;
                    self.add_status(StatusEffects::TheDrunk, player_index, player_index);
                }
                Role::Fortuneteller => {
                    // TODO: Add a red-herring through status effects
                    // Get storyteller input on who red-herring is
                    let target_player_index = self.choose_players(1)[0];
                    self.add_status(
                        StatusEffects::FortuneTellerRedHerring,
                        player_index,
                        target_player_index,
                    );
                }
                _ => (),
            }
            // TODO: Log events that happen in the setup
        }
    }

    // WARNING: Unused for now
    pub(crate) fn get_player_index(&self, player: &Player) -> PlayerIndex {
        self.players
            .iter()
            .position(|p| p == player)
            .expect("Player should be in player array")
    }

    pub(crate) fn add_status(
        &mut self,
        effect_type: StatusEffects,
        source_player_index: PlayerIndex,
        affected_player_index: PlayerIndex,
    ) {
        let new_status = StatusEffect::new(
            effect_type,
            source_player_index,
            self.players[source_player_index].role,
            affected_player_index,
        );
        self.status_effects.push(new_status);
    }

    pub(crate) fn remove_status(
        &mut self,
        effect_type: StatusEffects,
        source_player_index: PlayerIndex,
        affected_player_index: PlayerIndex,
    ) {
        let index = self
            .status_effects
            .iter()
            .position(|s| {
                s.status_type == effect_type
                    && s.source_player_index == source_player_index
                    && s.affected_player_index == affected_player_index
            })
            .expect("Tried to remove status effect not in game");
        self.status_effects.remove(index);
    }

    pub(crate) fn get_inflicted_statuses(
        &self,
        source_player_index: PlayerIndex,
    ) -> Vec<&StatusEffect> {
        self.status_effects
            .iter()
            .filter(|s| s.source_player_index == source_player_index)
            .collect()
    }

    pub(crate) fn get_afflicted_statuses(
        &self,
        affected_player_index: PlayerIndex,
    ) -> Vec<&StatusEffect> {
        self.status_effects
            .iter()
            .filter(|s| s.affected_player_index == affected_player_index)
            .collect()
    }

    // Should return true if the player was successfully killed, false if the player failed to die
    pub(crate) fn kill_player(
        &mut self,
        attacking_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
    ) -> bool {
        // Find the target player in the array and set their status to dead
        if self
            .get_afflicted_statuses(target_player_index)
            .iter()
            .any(|s| match s.status_type {
                StatusEffects::DeathProtected => true,
                StatusEffects::NightProtected if self.day_phase == DayPhase::Night => true,
                StatusEffects::DemonProtected
                    if self.players[attacking_player_index].role.get_type()
                        == CharacterType::Demon =>
                {
                    true
                }
                _ => false,
            })
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
        let player = self.players.get_mut(target_player_index).unwrap();
        player.dead = true;
        return true;
    }

    pub(crate) fn nominate_player(
        &mut self,
        source_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
    ) -> bool {
        // Should execute the target player if the vote succeeds
        // On nomination effects
        let source_player = &mut self.players[source_player_index];
        match source_player.role {
            _ => (),
        }

        // For now just check for virgin and whether enough votes to pass
        let target_player = &mut self.players[source_player_index];
        match target_player.role {
            Role::Virgin => {
                target_player.ability_active = false;
                return self.execute_player(source_player_index);
            }
            _ => (),
        }

        // TODO: Storyteller should input vote count
        let vote_count: usize = todo!();
        if vote_count >= self.living_player_count() / 2 {
            return self.execute_player(target_player_index);
        }

        return false;
    }

    pub(crate) fn living_player_count(&self) -> usize {
        self.players.iter().filter(|s| s.dead == false).count()
    }

    pub(crate) fn execute_player(&mut self, target_player_index: PlayerIndex) -> bool {
        // WARNING: There may be shared code between here and kill_player

        // Check if there is something that stops the player's death
        if self
            .get_afflicted_statuses(target_player_index)
            .iter()
            .any(|s| match s.status_type {
                StatusEffects::DeathProtected => true,
                _ => false,
            })
        {
            return true;
        }

        // Execute a player
        let target_player = &mut self.players[target_player_index];
        target_player.dead = true;

        // TODO: Handle player death based on their role and time of day

        // End the day
        return true;
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

    pub(crate) fn get_active_roles(&self) -> Vec<PlayerIndex> {
        let mut result: Vec<PlayerIndex> = vec![];
        for i in 0..self.players.len() {
            let player = &self.players[i];
            if !player.ability_active {
                continue;
            }
            result.push(i);
        }

        return result;
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

    pub(crate) fn get_order_from_map(
        &self,
        mut order_map: HashMap<usize, PlayerIndex>,
    ) -> Vec<PlayerIndex> {
        let mut final_order: Vec<PlayerIndex> = vec![];
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

    pub(crate) fn resolve_night_1(&mut self) {
        self.day_phase = DayPhase::Night;
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

    pub(crate) fn get_night_1_order(&self, player_indices: Vec<PlayerIndex>) -> Vec<PlayerIndex> {
        // Go through all roles and assign each of them a number
        // Maps night_order to player index
        let mut order_map: HashMap<usize, PlayerIndex> = HashMap::new();
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
            };
            if order != 0 {
                order_map.insert(order, index);
            }
        }

        return self.get_order_from_map(order_map);
    }

    pub(crate) fn resolve_night_1_ability(&mut self, player_index: PlayerIndex) {
        // Check if the role is active before resolving their ability, skip if the role is
        // inactive, but also warn
        // eprintln!("An inactive role tried to act during the night");
        // NOTE: I think that for info roles, the storyteller should still receive the correct
        // info, but there will be a warning that the player is poisoned on the screen somewhere,
        // letting the storyteller decide what number they should give
        // TODO: Implement abilities for every role
        let player = &mut self.players[player_index];
        let role = player.role;
        match role {
            Role::Investigator => {
                // WARNING: Can't actually resolve this, this should be decided during setup
                todo!()
            }
            Role::Empath => {
                let count = self.empath_ability(player_index);
                // For now, just print output
                println!("Empath count: {}", count);
            }
            Role::Gossip => todo!(),      // Should wait till v2
            Role::Innkeeper => todo!(),   // Should wait till v2
            Role::Washerwoman => todo!(), // Setup
            Role::Librarian => todo!(),   // Setup
            Role::Chef => {
                // Count pairs of evil players
                // For each evil, player, check if the right player is evil, if yes, increment the
                // pair count
                let mut pair_count = 0;

                for player_index in 0..self.players.len() {
                    let player = &self.players[player_index];
                    if player.alignment != Alignment::Evil {
                        continue;
                    }
                    let right_player = &self.players[self.right_player(player_index)];
                    if right_player.alignment == Alignment::Evil {
                        pair_count += 1;
                    }
                }
                println!("Chef Pair Count: {}", pair_count);
            }
            Role::Fortuneteller => todo!(), // Should be the same as ability from other nights, but
            // also need setup
            Role::Undertaker => {
                // TODO: Should scan the log for the entire day yesterday
                // If there was a execution event yesterday that resulted in death, grab the player
                // from that event
                // Grab that player's role and give it to the undertaker
                todo!();
            }
            Role::Virgin => {
                // Add a status effect that if someone nominates you, they die
                // Maybe instead add this to the nominate method
                todo!()
            }
            Role::Soldier => {
                // Just add protected status effect and only remove upon death
                self.add_status(StatusEffects::DemonProtected, player_index, player_index);
            }
            Role::Slayer => todo!(), // No night one ability
            Role::Mayor => {
                // No night one ability, but add effect to yourself
                self.add_status(StatusEffects::MayorBounceKill, player_index, player_index);
            }
            Role::Monk => todo!(), // No night one ability
            Role::Drunk => {
                // WARNING: This one is a little difficult
                // Maybe just add the role but make them drunk?
                // Maybe during setup swap the drunk with another role if they are selected but
                // give them a status effect as well?
                todo!()
            } // Should be handled during setup, also gets mimiced
            // role's ability
            Role::Saint => todo!(),  // No night one ability
            Role::Butler => todo!(), // Status effect?, also same as normal ability
            Role::Recluse => {
                // Status effect
                self.add_status(StatusEffects::AppearsEvil, player_index, player_index);
            }
            Role::Spy => {
                // Status effect and look at grimoire?
                self.add_status(StatusEffects::AppearsEvil, player_index, player_index);
                // Just tell the storyteller to let the spy look at the grimoire
                todo!()
            }
            Role::Baron => todo!(),        // Should affect setup
            Role::Scarletwoman => todo!(), // Basically shouldn't happen night one
            Role::Poisoner => todo!(),     // Add poison to someone until next night, same as
            // normal ability
            Role::Imp => todo!(), // Nothing to do night one
            Role::Ravenkeeper => todo!(), // No night ability unless they die, same as normal
                                   // ability
        }

        // TODO: Method should wait until storyteller explicitly advances to the next phase

        // TODO: The event should be logged at some point
    }

    pub(crate) fn resolve_day(&mut self) {
        // Only a few roles act during the day, and the storyteller only really needs to mark
        // whether someone claimed something
        // Some roles like savant come to the story teller during the day, the story teller should
        // have options for all such roles in the game. These options should be shown all at once,
        // (Like "these roles may come up to you today/ act during the day")
        // and the storyteller should be able to quickly log that these events happened
        //
        // FIX: For now, this method will just do nothing. The functionality for it can be
        // implemented later
        self.day_phase = DayPhase::Day;
        todo!();
    }

    pub(crate) fn resolve_night(&mut self) {
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
        self.day_phase = DayPhase::Night;
        // Order the roles in this game to get the order they should be woken up in (should be
        // unique to night 1)
        let ordered_player_indices = self.get_night_order(self.get_active_roles());
        // Wake each role up in order and show them the information they need to know, or the
        // choices that they get
        // For each choice:
        //      If that choice impacts the game state, change the game state accordingly
        //      If that choice tells the player info, give them that info
        //      Should be calling a generic method on the role class to get info on the role's
        //      ability
        // Once you have gone through all the roles, nothing to do: wake everyone up
        for i in ordered_player_indices.iter() {
            self.resolve_night_ability(*i);
        }
    }

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

    pub(crate) fn resolve_night_ability(&mut self, player_index: PlayerIndex) {
        // Check if the role is active before resolving their ability, skip if the role is
        // inactive, but also warn
        // eprintln!("An inactive role tried to act during the night");
        // NOTE: I think that for info roles, the storyteller should still receive the correct
        // info, but there will be a warning that the player is poisoned on the screen somewhere,
        // letting the storyteller decide what number they should give
        // TODO: Implement abilities for every role
        let player = &mut self.players[player_index];
        let role = player.role;
        match role {
            Role::Empath => {
                let count = self.empath_ability(player_index);
                // For now, just print output
                println!("Empath count: {}", count);
            }
            Role::Gossip => todo!(),    // wait for v2
            Role::Innkeeper => todo!(), // Wait for v2
            Role::Fortuneteller => todo!(),
            Role::Undertaker => todo!(),
            Role::Monk => {
                // Give the target the demon protected status effect
                // TODO: Prompt the storyteller to choose a player
                let target_index = todo!();
                self.add_status(StatusEffects::DemonProtected, player_index, target_index);
            }
            Role::Ravenkeeper => {
                // TODO:
                // Should only happen when the ravenkeeper is dead
                // Perhaps check every night if ravenkeeper is dead, or was killed that night?
                // After death, prompt storyteller to choose player
                let target_index: PlayerIndex = todo!();
                let role = self.players[target_index].role;
            }
            Role::Butler => {
                // TODO:
                // Prompt the storyteller to choose a player
                let target_index: PlayerIndex = self.choose_players(1)[0];
                self.add_status(StatusEffects::ButlerMaster, player_index, target_index);
            }
            Role::Spy => {
                // TODO: Literally just let them look at the grimoire
                // End the phase when they're done looking at the grimoire
            }
            Role::Scarletwoman => {
                // TODO: Check if the demon is dead at that point and there are more than 5 players
                // Scarlet woman becomes the demon, should actually become the demon before this,
                // but this is when they should be notified
            }
            Role::Poisoner => {
                // TODO: Poison someone
            }
            Role::Imp => {
                // TODO: Kill someone, if your target is yourself, kill yourself but transfer demon
                // to a minion
                // How to transfer demon to minion? Let storyteller decide. Prompt the storyteller
                // to choose a player. Validate that the player is a minion, if they aren't, prompt
                // them to choose again. If there are no minions in play, don't even give them the
                // option
            }
            _ => {
                eprintln!("A role that wasn't supposed to act acted");
                panic!()
            }
        }

        // TODO: Method should wait until storyteller explicitly advances to the next phase

        // TODO: The event should be logged at some point
    }

    // NOTE: Role Specific Abilities
    pub(crate) fn empath_ability(&mut self, player_index: PlayerIndex) -> usize {
        // Check how many players next to the empath are evil
        let mut count = 0;
        let left_player = &self.players[self.left_player(player_index)];
        let right_player = &self.players[self.right_player(player_index)];
        if left_player.alignment == Alignment::Evil {
            count += 1;
        }
        if right_player.alignment == Alignment::Evil {
            count += 1;
        }

        return count;
    }
}

// -- Logging --
// TODO: Implement all events
#[derive(Debug)]
pub(crate) enum EventType {
    // Game Time Events
    DayStart,
    DayEnd,
    NightStart,
    NightEnd,
    // Player Events
    Nomination,
    Execution,
    AttemptedKill,
    Protected,
    Death,
    // Ability Specific Events
}

pub(crate) struct Event {
    pub(crate) event_type: EventType,
    pub(crate) source_player: Option<PlayerIndex>,
    pub(crate) target_player: Option<PlayerIndex>,
}

impl Event {
    pub(crate) fn new(
        event_type: EventType,
        source_player: Option<PlayerIndex>,
        target_player: Option<PlayerIndex>,
    ) -> Self {
        Self {
            event_type,
            source_player,
            target_player,
        }
    }

    pub(crate) fn new_game_event(event_type: EventType) -> Self {
        Self {
            event_type,
            source_player: None,
            target_player: None,
        }
    }
    pub(crate) fn get_description(&self) -> String {
        todo!();
    }

    pub(crate) fn get_reason(&self) -> Option<String> {
        todo!();
    }
}

#[derive(PartialEq)]
pub(crate) enum DayPhase {
    Day,
    Night,
}

pub(crate) struct DayPhaseLog {
    day_phase: DayPhase,
    log: Vec<Event>,
}

pub(crate) struct Nychthemeron {
    day_num: usize,
    day: DayPhaseLog,
    night: DayPhaseLog,
}
pub(crate) struct Log {
    // TODO: Make this a tree eventually
    nychthemrons: Vec<Nychthemeron>,
}

impl Log {
    pub(crate) fn new() -> Self {
        Self {
            nychthemrons: vec![],
        }
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Alignment {
    Good,
    Evil,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CharacterType {
    Townsfolk,
    Outsider,
    Minion,
    Demon,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Role {
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

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Investigator => write!(f, "Investigator"),
            Role::Empath => write!(f, "Empath"),
            Role::Gossip => write!(f, "Gossip"),
            Role::Innkeeper => write!(f, "Innkeeper"),
            Role::Washerwoman => write!(f, "Washerwoman"),
            Role::Librarian => write!(f, "Librarian"),
            Role::Chef => write!(f, "Chef"),
            Role::Fortuneteller => write!(f, "Fortuneteller"),
            Role::Undertaker => write!(f, "Undertaker"),
            Role::Virgin => write!(f, "Virgin"),
            Role::Soldier => write!(f, "Soldier"),
            Role::Slayer => write!(f, "Slayer"),
            Role::Mayor => write!(f, "Mayor"),
            Role::Monk => write!(f, "Monk"),
            Role::Ravenkeeper => write!(f, "Ravenkeeper"),
            Role::Drunk => write!(f, "Drunk"),
            Role::Saint => write!(f, "Saint"),
            Role::Butler => write!(f, "Butler"),
            Role::Recluse => write!(f, "Recluse"),
            Role::Spy => write!(f, "Spy"),
            Role::Baron => write!(f, "Baron"),
            Role::Scarletwoman => write!(f, "Scarletwoman"),
            Role::Poisoner => write!(f, "Poisoner"),
            Role::Imp => write!(f, "Imp"),
        }
    }
}
impl Role {
    pub(crate) fn get_default_alignment(&self) -> Alignment {
        match self.get_type() {
            CharacterType::Minion | CharacterType::Demon => Alignment::Evil,
            _ => Alignment::Good,
        }
    }

    pub(crate) fn get_type(&self) -> CharacterType {
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

    pub(crate) fn is_win_condition(&self) -> bool {
        match self.get_type() {
            CharacterType::Demon => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct StatusEffect {
    status_type: StatusEffects,
    source_role: Role,
    source_player_index: PlayerIndex,
    affected_player_index: PlayerIndex,
}

impl StatusEffect {
    pub(crate) fn new(
        status_type: StatusEffects,
        source_player_index: PlayerIndex,
        source_role: Role,
        affected_player_index: PlayerIndex,
    ) -> Self {
        Self {
            status_type,
            source_player_index,
            source_role,
            affected_player_index,
        }
    }
}
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum StatusEffects {
    // General
    Drunk,
    Mad,
    Poisoned,
    DemonProtected,
    NightProtected,
    DeathProtected,
    // Role specific
    ButlerMaster,
    AppearsGood,
    AppearsEvil,
    MayorBounceKill,
    TheDrunk,
    FortuneTellerRedHerring,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) role: Role,
    // Order should be implemented through external array
    pub(crate) dead: bool,
    pub(crate) ability_active: bool, // WARNING: Not too happy about this implementation, might want to make
    // it cleaner
    pub(crate) ghost_vote: bool,
    // it cleaner
    pub(crate) alignment: Alignment,
}

impl Player {
    pub(crate) fn new(name: String, role: Role) -> Self {
        Self {
            name,
            role,
            ghost_vote: true,
            ability_active: true,
            dead: false,
            alignment: role.get_default_alignment(),
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

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: Testing Utils

    struct TestInput {}

    impl StoryTellerInterface for TestInput {
        fn choose_players(&self, num: usize, max_index: usize) -> Vec<PlayerIndex> {
            let mut rng = rand::rng();
            let mut values: Vec<usize> = (0..max_index).collect();
            values.shuffle(&mut rng);
            let mut result: Vec<PlayerIndex> = vec![];
            for i in 0..num {
                result.push(values.pop().unwrap());
            }

            return result;
        }

        fn choose_roles(&self, num: usize, mut valid_roles: Vec<Role>) -> Vec<Role> {
            let mut rng = rand::rng();
            valid_roles.shuffle(&mut rng);
            let mut result: Vec<Role> = vec![];
            for i in 0..num {
                result.push(valid_roles.pop().unwrap());
            }

            return result;
        }

        // These don't need to be implemented for testing
        fn input_number(&self) -> usize {
            todo!()
        }

        fn display_number(&self) {
            todo!()
        }

        fn display_players(&self) {
            todo!()
        }

        fn display_role(&self) {
            todo!()
        }
    }

    const TEST_INPUT: TestInput = TestInput {};

    const EMPTY_SCRIPT: Script = Script { roles: vec![] };

    // NOTE: Tests
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
    fn test_new_game() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let game = Game::new(roles.clone(), player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        assert_eq!(game.players.len(), 3);
        assert_eq!(game.players[0].name, "P1");
        assert_eq!(game.players[1].name, "P2");
        assert_eq!(game.players[2].name, "P3");

        assert_eq!(game.status_effects.len(), 0);

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

        // TODO: Maybe add a check here that all the assigment events were logged
    }

    #[test]
    fn game_setup() {
        // TODO: Do this after implementing setup method
        // Only way to really test this right now is through baron and drunk
        todo!()
    }

    #[test]
    fn kill_player() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        game.kill_player(0, 0);
        assert_eq!(game.players[0].dead, true);
        game.kill_player(1, 1);
        assert_eq!(game.players[1].dead, true);
        game.kill_player(2, 2);
        assert_eq!(game.players[2].dead, true);
    }

    #[test]
    fn kill_death_protected_player() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        game.add_status(StatusEffects::DeathProtected, 1, 1);

        game.kill_player(0, 0);
        assert_eq!(game.players[0].dead, true);
        game.kill_player(1, 1);
        assert_eq!(game.players[1].dead, false);
        game.kill_player(2, 2);
        assert_eq!(game.players[2].dead, true);

        game.remove_status(StatusEffects::DeathProtected, 1, 1);
        game.kill_player(1, 1);
        assert_eq!(game.players[1].dead, true);
    }

    #[test]
    fn kill_night_protected_player() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        game.day_phase = DayPhase::Night;
        game.add_status(StatusEffects::NightProtected, 1, 1);

        game.kill_player(0, 0);
        assert_eq!(game.players[0].dead, true);
        game.kill_player(1, 1);
        assert_eq!(game.players[1].dead, false);
        game.kill_player(2, 2);
        assert_eq!(game.players[2].dead, true);

        game.day_phase = DayPhase::Day;
        game.kill_player(1, 1);
        assert_eq!(game.players[1].dead, true);
    }

    #[test]
    fn kill_demon_protected_player() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        game.add_status(StatusEffects::DemonProtected, 1, 1);

        let demon_index = game.win_cond_i.unwrap();

        game.kill_player(demon_index, 0);
        assert_eq!(game.players[0].dead, true);
        game.kill_player(demon_index, 1);
        assert_eq!(game.players[1].dead, false);
        game.kill_player(demon_index, 2);
        assert_eq!(game.players[2].dead, true);

        game.kill_player(demon_index, 1);
        assert_eq!(game.players[1].dead, false);

        game.remove_status(StatusEffects::DemonProtected, 1, 1);
        game.kill_player(demon_index, 1);
        assert_eq!(game.players[1].dead, true);
    }

    #[test]
    fn test_left() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut game = Game::new(roles.clone(), player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        assert_eq!(game.players[game.left_player(1)], game.players[0]);

        // Kill set the left player to dead and see that the left player is updated accordingly
        game.kill_player(0, 0);
        assert_eq!(game.players[game.left_player(1)], game.players[2]);
    }

    #[test]
    fn test_right() {
        let roles = vec![Role::Investigator, Role::Innkeeper, Role::Imp];
        let player_names = vec![String::from("P1"), String::from("P2"), String::from("P3")];

        let mut game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        assert_eq!(game.players[game.right_player(1)], game.players[2]);

        // Kill the right player and make sure the right player is updated accordingly
        game.kill_player(0, 2);
        assert_eq!(game.players[game.right_player(1)], game.players[0]);
    }

    #[test]
    fn test_game_over() {
        todo!();
    }

    #[test]
    fn test_get_night_1_order() {
        let roles = vec![
            Role::Investigator,
            Role::Innkeeper,
            Role::Imp,
            Role::Chef,
            Role::Poisoner,
        ];
        let player_names = vec![
            String::from("P1"),
            String::from("P2"),
            String::from("P3"),
            String::from("P4"),
            String::from("P5"),
        ];

        let game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        let player_indices = vec![0, 1, 2, 3, 4];
        let order = game.get_night_1_order(player_indices);
        assert_eq!(game.players[order[0]].role, Role::Poisoner);
        assert_eq!(game.players[order[1]].role, Role::Investigator);
        assert_eq!(game.players[order[2]].role, Role::Chef);
        assert_eq!(order.len(), 3);
    }

    fn test_resolve_night_1() {
        todo!();
    }

    // TODO: Test that all night one abilities work as expected

    fn test_night_order() {
        let roles = vec![
            Role::Investigator,
            Role::Innkeeper,
            Role::Imp,
            Role::Chef,
            Role::Poisoner,
        ];
        let player_names = vec![
            String::from("P1"),
            String::from("P2"),
            String::from("P3"),
            String::from("P4"),
            String::from("P5"),
        ];

        let game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        let player_indices = vec![0, 1, 2, 3, 4];
        let order = game.get_night_order(player_indices);
        assert_eq!(game.players[order[0]].role, Role::Poisoner);
        assert_eq!(game.players[order[1]].role, Role::Innkeeper);
        assert_eq!(order.len(), 2);
    }

    // TODO: Test that all night abilities work as expected
    fn tsest_resolve_night() {
        todo!();
    }

    #[test]
    fn add_status_effect() {
        let roles = vec![
            Role::Investigator,
            Role::Innkeeper,
            Role::Imp,
            Role::Chef,
            Role::Poisoner,
        ];
        let player_names = vec![
            String::from("P1"),
            String::from("P2"),
            String::from("P3"),
            String::from("P4"),
            String::from("P5"),
        ];

        let mut game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        game.add_status(StatusEffects::Poisoned, 2, 0);

        assert_eq!(game.status_effects[0].status_type, StatusEffects::Poisoned);
        assert_eq!(game.status_effects[0].source_player_index, 2);
        assert_eq!(game.status_effects[0].affected_player_index, 0);
    }

    #[test]
    fn add_multiple_status_effects() {
        let roles = vec![
            Role::Investigator,
            Role::Innkeeper,
            Role::Imp,
            Role::Chef,
            Role::Poisoner,
        ];
        let player_names = vec![
            String::from("P1"),
            String::from("P2"),
            String::from("P3"),
            String::from("P4"),
            String::from("P5"),
        ];

        let mut game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        game.add_status(StatusEffects::Poisoned, 2, 0);
        game.add_status(StatusEffects::MayorBounceKill, 1, 3);
        game.add_status(StatusEffects::Drunk, 4, 2);

        assert_eq!(
            game.status_effects
                .iter()
                .filter(|s| {
                    s.status_type == StatusEffects::Poisoned
                        && s.source_player_index == 2
                        && s.source_role == game.players[2].role
                        && s.affected_player_index == 0
                })
                .count(),
            1
        );

        assert_eq!(
            game.status_effects
                .iter()
                .filter(|s| {
                    s.status_type == StatusEffects::MayorBounceKill
                        && s.source_player_index == 1
                        && s.source_role == game.players[1].role
                        && s.affected_player_index == 3
                })
                .count(),
            1
        );

        assert_eq!(
            game.status_effects
                .iter()
                .filter(|s| {
                    s.status_type == StatusEffects::Drunk
                        && s.source_player_index == 4
                        && s.source_role == game.players[4].role
                        && s.affected_player_index == 2
                })
                .count(),
            1
        );

        // Checks that same player can have multiple status effects applied to them
        // Checks that the same player can have multiple of the same status effect from differnet
        // sources applied to them
        //
        game.add_status(StatusEffects::Drunk, 3, 2);
        game.add_status(StatusEffects::Drunk, 1, 2);
        game.add_status(StatusEffects::Poisoned, 4, 2);
        game.add_status(StatusEffects::Drunk, 1, 0);

        assert_eq!(
            game.status_effects
                .iter()
                .filter(|s| { s.status_type == StatusEffects::Drunk })
                .count(),
            4
        );

        assert_eq!(
            game.status_effects
                .iter()
                .filter(|s| {
                    s.status_type == StatusEffects::Drunk && s.affected_player_index == 2
                })
                .count(),
            3
        );

        assert_eq!(
            game.status_effects
                .iter()
                .filter(|s| {
                    s.status_type == StatusEffects::Drunk
                        && s.source_player_index == 4
                        && s.source_role == game.players[4].role
                        && s.affected_player_index == 2
                })
                .count(),
            1
        );

        assert_eq!(
            game.status_effects
                .iter()
                .filter(|s| {
                    s.status_type == StatusEffects::Poisoned && s.affected_player_index == 2
                })
                .count(),
            1
        );

        assert_eq!(
            game.status_effects
                .iter()
                .filter(|s| {
                    s.source_player_index == 4
                        && s.source_role == game.players[4].role
                        && s.affected_player_index == 2
                })
                .count(),
            2
        );

        assert_eq!(
            game.status_effects
                .iter()
                .filter(|s| { s.affected_player_index == 2 })
                .count(),
            4
        );
    }

    #[test]
    fn find_status_effects_inflicted_by_player() {
        let roles = vec![
            Role::Investigator,
            Role::Innkeeper,
            Role::Imp,
            Role::Chef,
            Role::Poisoner,
        ];
        let player_names = vec![
            String::from("P1"),
            String::from("P2"),
            String::from("P3"),
            String::from("P4"),
            String::from("P5"),
        ];

        let mut game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        game.add_status(StatusEffects::Poisoned, 2, 0);
        game.add_status(StatusEffects::MayorBounceKill, 1, 3);
        game.add_status(StatusEffects::Drunk, 4, 2);

        game.add_status(StatusEffects::Drunk, 2, 2);
        game.add_status(StatusEffects::Drunk, 2, 1);
        game.add_status(StatusEffects::Drunk, 2, 0);

        let statuses = game.get_inflicted_statuses(2);
        assert_eq!(statuses.len(), 4);
        assert_eq!(
            statuses
                .iter()
                .filter(|s| s.status_type == StatusEffects::Drunk)
                .count(),
            3
        );
        assert_eq!(
            statuses
                .iter()
                .filter(|s| s.status_type == StatusEffects::Poisoned)
                .count(),
            1
        );
        assert!(statuses.iter().all(|s| s.source_player_index == 2));
        assert!(
            statuses
                .iter()
                .all(|s| s.source_role == game.players[2].role)
        );

        let no_statuses = game.get_inflicted_statuses(0);
        assert_eq!(no_statuses.len(), 0);
    }

    #[test]
    fn find_status_effects_inlicted_by_player() {
        let roles = vec![
            Role::Investigator,
            Role::Innkeeper,
            Role::Imp,
            Role::Chef,
            Role::Poisoner,
        ];
        let player_names = vec![
            String::from("P1"),
            String::from("P2"),
            String::from("P3"),
            String::from("P4"),
            String::from("P5"),
        ];

        let mut game = Game::new(roles, player_names, EMPTY_SCRIPT, TEST_INPUT).unwrap();

        game.add_status(StatusEffects::Poisoned, 2, 0);
        game.add_status(StatusEffects::MayorBounceKill, 1, 3);
        game.add_status(StatusEffects::Poisoned, 4, 2);

        game.add_status(StatusEffects::Drunk, 3, 2);
        game.add_status(StatusEffects::Drunk, 1, 2);
        game.add_status(StatusEffects::Drunk, 0, 2);

        let statuses = game.get_afflicted_statuses(2);
        assert_eq!(statuses.len(), 4);
        assert_eq!(
            statuses
                .iter()
                .filter(|s| s.status_type == StatusEffects::Drunk)
                .count(),
            3
        );
        assert_eq!(
            statuses
                .iter()
                .filter(|s| s.status_type == StatusEffects::Poisoned)
                .count(),
            1
        );
        assert!(statuses.iter().all(|s| s.affected_player_index == 2));

        let no_statuses = game.get_afflicted_statuses(4);
        assert_eq!(no_statuses.len(), 0);
    }

    #[test]
    fn remove_status_effect() {
        todo!();
    }

    #[test]
    fn remove_multiple_status_effects() {
        todo!();
    }
}
