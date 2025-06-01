use crate::engine::state::State;

impl State {
    fn setup(&mut self) {
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
}
