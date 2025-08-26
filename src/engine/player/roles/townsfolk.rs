use std::fmt::Display;
use std::sync::Arc;

use macros::washerwoman_librarian_investigator;

use crate::engine::state::status_effects::CleanupPhase;
use crate::{
    engine::{
        change_request::{ChangeArgs, ChangeError, ChangeRequest, ChangeType},
        player::{Alignment, CharacterType, PlayerBehaviors, roles::Role},
        state::{
            PlayerIndex, State,
            status_effects::{StatusEffect, StatusType},
        },
    },
    unwrap_args_err, unwrap_args_panic,
};

fn washerwoman_librarian_investigator<
    RE: StatusType + Default + 'static,
    WE: StatusType + Default + 'static,
>(
    player_index: PlayerIndex,
    character_type: CharacterType,
) -> Option<ChangeRequest> {
    let right_description = format!("Select a {}", &character_type.to_string());

    let right_status =
        move || StatusEffect::new(std::sync::Arc::new(RE::default()), player_index, None);

    let change_type = ChangeType::ChoosePlayers(1);

    let right_check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
        let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

        let len = target_player_indices.len();
        if len != 1 {
            return Err(ChangeError::WrongNumberOfSelectedPlayers {
                wanted: 1,
                got: len,
            });
        }

        for target_player_index in target_player_indices {
            if *target_player_index == player_index {
                return Ok(false);
            }

            let ct = state.get_player(*target_player_index).get_character_type();
            if ct == character_type || ct == CharacterType::Any {
                return Ok(true);
            }
        }

        return Ok(false);
    };

    let right_state_change = move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
        let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

        let target_player = state.get_player_mut(target_player_indices[0]);
        target_player.add_status(right_status());

        let wrong_status =
            move || StatusEffect::new(std::sync::Arc::new(WE::default()), player_index, None);
        let wrong_description = "Select a different player";

        let wrong_change_type = ChangeType::ChoosePlayers(1);

        let wrong_check_func = move |state: &State,
                                     args: &ChangeArgs|
              -> Result<bool, ChangeError> {
            let target_player_indices = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

            let len = target_player_indices.len();
            if len != 1 {
                return Err(ChangeError::WrongNumberOfSelectedPlayers {
                    wanted: 1,
                    got: len,
                });
            }

            let target_player_index = target_player_indices[0];

            if target_player_index == player_index {
                return Ok(false);
            }

            let target_player = state.get_player(target_player_index);
            if target_player
                .get_statuses()
                .iter()
                .any(|se| *se == right_status())
            {
                return Ok(false);
            }
            return Ok(true);
        };

        let wrong_state_change = move |state: &mut State,
                                       args: ChangeArgs|
              -> Option<ChangeRequest> {
            let target_player_indices = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);

            // Assign the chosen player the wrong status effect
            let target_player = state.get_player_mut(target_player_indices[0]);
            target_player.add_status(wrong_status());

            None
        };

        return Some(ChangeRequest::new(
            wrong_change_type,
            wrong_description.into(),
            wrong_check_func,
            wrong_state_change,
        ));
    };

    return Some(ChangeRequest::new(
        change_type,
        right_description.into(),
        right_check_func,
        right_state_change,
    ));
}

#[derive(Default)]
pub(crate) struct Washerwoman();

struct WasherwomanTownsfolk();
impl StatusType for WasherwomanTownsfolk {}
impl Display for WasherwomanTownsfolk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Washerwoman Townsfolk")
    }
}

struct WasherwomanWrong();
impl StatusType for WasherwomanWrong {}
impl Display for WasherwomanWrong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Washerwoman Wrong")
    }
}

impl Role for Washerwoman {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn setup_order(&self) -> Option<usize> {
        Some(45)
    }

    fn setup_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequest> {
        // TODO: Change back into a function
        washerwoman_librarian_investigator!(
            player_index,
            WasherwomanTownsfolk,
            WasherwomanWrong,
            CharacterType::Townsfolk
        )
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(45)
    }

    fn night_one_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequest> {
        let player = state.get_player(player_index);
        let message = format!("Show the {} the correct roles", player.role);
        let change_type = ChangeType::Display;

        Some(ChangeRequest::new_display(change_type, message))
    }
}

impl Display for Washerwoman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Washerwoman")
    }
}

#[derive(Default)]
pub(crate) struct Librarian();

struct LibrarianOutsider();
impl StatusType for LibrarianOutsider {}
impl Display for LibrarianOutsider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Librarian Outsider")
    }
}

struct LibrarianWrong();
impl StatusType for LibrarianWrong {}
impl Display for LibrarianWrong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Librarian Wrong")
    }
}

impl Role for Librarian {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn setup_order(&self) -> Option<usize> {
        Some(46)
    }

    fn setup_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequest> {
        washerwoman_librarian_investigator!(
            player_index,
            LibrarianOutsider,
            LibrarianWrong,
            CharacterType::Outsider
        )
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(46)
    }

    fn night_one_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequest> {
        let player = state.get_player(player_index);
        let message = format!("Show the {} the correct roles", player.role);
        let change_type = ChangeType::Display;

        Some(ChangeRequest::new_display(change_type, message))
    }
}

impl Display for Librarian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Librarian")
    }
}

#[derive(Default)]
pub(crate) struct Investigator();

struct InvestigatorMinion();
impl StatusType for InvestigatorMinion {}
impl Display for InvestigatorMinion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Investigator Minion")
    }
}

struct InvestigatorWrong();
impl StatusType for InvestigatorWrong {}
impl Display for InvestigatorWrong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Investigator Wrong")
    }
}

impl Role for Investigator {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn setup_order(&self) -> Option<usize> {
        Some(47)
    }

    fn setup_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequest> {
        washerwoman_librarian_investigator!(
            player_index,
            InvestigatorMinion,
            InvestigatorWrong,
            CharacterType::Minion
        )
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(47)
    }

    fn night_one_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequest> {
        let player = state.get_player(player_index);
        let message = format!("Show the {} the correct roles", player.role);
        let change_type = ChangeType::Display;

        Some(ChangeRequest::new_display(change_type, message))
    }
}

impl Display for Investigator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Investigator")
    }
}

#[derive(Default)]
pub(crate) struct Chef();

impl Role for Chef {
    fn get_default_alignment(&self) -> crate::engine::player::Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> crate::engine::player::CharacterType {
        CharacterType::Townsfolk
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(48)
    }

    fn night_one_ability(
        &self,
        _player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequest> {
        // Count pairs of evil players
        // For each evil, player, check if the right player is evil, if yes, increment the
        // pair count
        let change_type = ChangeType::Display;
        let mut pair_count = 0;

        let players = state.get_players();

        for (player_index, player) in players.iter().enumerate() {
            if player.alignment != Alignment::Evil {
                continue;
            }
            let right_player = state.get_player(state.right_player(player_index));
            if right_player.alignment == Alignment::Evil {
                pair_count += 1;
            }
        }
        let message = format!(
            "Show the chef that there are {} pairs of evil players",
            pair_count
        );

        Some(ChangeRequest::new_display(change_type, message))
    }
}

impl Display for Chef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Chef")
    }
}

#[derive(Default)]
pub(crate) struct Empath();

impl Empath {
    fn ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        // Check how many players next to the empath are evil
        let mut count = 0;
        {
            let left_player = state.get_player(state.left_player(player_index));
            if left_player.alignment == Alignment::Evil {
                count += 1;
            }
        }
        {
            let right_player = state.get_player(state.right_player(player_index));
            if right_player.alignment == Alignment::Evil {
                count += 1;
            }
        }
        let message = format!("Empath has {} evil neighbors", count);

        let change_type = ChangeType::Display;

        Some(ChangeRequest::new_display(change_type, message))
    }
}

impl Role for Empath {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(49)
    }

    fn night_order(&self) -> Option<usize> {
        Some(68)
    }

    fn night_one_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        self.ability(player_index, state)
    }

    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let dead = state.get_player(player_index).dead;
        if dead {
            return None;
        }
        self.ability(player_index, state)
    }
}

impl Display for Empath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Empath")
    }
}

#[derive(Default)]
pub(crate) struct Fortuneteller();

struct FortunetellerRedHerring();
impl StatusType for FortunetellerRedHerring {}
impl Display for FortunetellerRedHerring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Fortuneteller Red Herring")
    }
}

impl Fortuneteller {
    fn ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let dead = state.get_player(player_index).dead;
        if dead {
            return None;
        }

        let message = "Prompt the FortuneTeller to point to two players";

        let change_type = ChangeType::ChoosePlayers(2);

        let check_func = move |_state: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
            let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(pv) => pv);
            let len = target_players.len();
            if len != 2 {
                return Err(ChangeError::WrongNumberOfSelectedPlayers {
                    wanted: 2,
                    got: len,
                });
            }

            // Make sure there are no duplicate players
            if target_players[0] == target_players[1] {
                return Err(ChangeError::InvalidSelectedPlayer {
                    reason: "Please select unique players".into(),
                });
            }

            return Ok(true);
        };

        let state_change_func =
            move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
                let target_players = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv);

                // Calculate whether any of the chosen players are either a red herring or a demon
                let demon_found = target_players.iter().any(|i| {
                    let player = state.get_player(*i);
                    matches!(
                        player.get_character_type(),
                        CharacterType::Demon | CharacterType::Any
                    ) || player.get_statuses().iter().any(|se| {
                        se.source_player_index == player_index
                            && se.to_string() == FortunetellerRedHerring().to_string()
                    })
                });
                let message = format!(
                    "Show the Fortuneteller a {}",
                    match demon_found {
                        true => "Thumbs Up",
                        false => "Thumbs Down",
                    }
                );

                let change_type = ChangeType::Display;

                Some(ChangeRequest::new_display(change_type, message))
            };

        Some(ChangeRequest::new(
            change_type,
            message.into(),
            check_func,
            state_change_func,
        ))
    }
}

impl Role for Fortuneteller {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn setup_order(&self) -> Option<usize> {
        Some(50)
    }

    fn setup_ability(&self, player_index: PlayerIndex, _state: &State) -> Option<ChangeRequest> {
        let description = "Select a red-herring for the Fortune Teller".to_string();

        let change_type = ChangeType::ChoosePlayers(1);
        let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
            let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

            let len = target_players.len();
            if target_players.len() != 1 {
                return Err(ChangeError::WrongNumberOfSelectedPlayers {
                    wanted: 1,
                    got: len,
                });
            }

            if target_players[0] == player_index {
                return Ok(false);
            }

            return Ok(true);
        };
        // Get storyteller input on who red-herring is
        // Add a red-herring through status effects
        let state_change = move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
            let target_players = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(v) => v);
            let target_player_index = target_players[0];
            let target_player = state.get_player_mut(target_player_index);
            let status = StatusEffect::new(Arc::new(FortunetellerRedHerring()), player_index, None);
            target_player.add_status(status);

            None
        };

        Some(ChangeRequest::new(
            change_type,
            description,
            check_func,
            state_change,
        ))
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(50)
    }

    fn night_one_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        self.ability(player_index, state)
    }

    fn night_order(&self) -> Option<usize> {
        Some(69)
    }

    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        self.ability(player_index, state)
    }
}

impl Display for Fortuneteller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Fortuneteller")
    }
}

// TODO:
// Undertaker

#[derive(Default)]
pub(crate) struct Monk();

struct DemonProtected {
    behaviors: Vec<PlayerBehaviors>,
}

impl Default for DemonProtected {
    fn default() -> Self {
        Self {
            behaviors: vec![PlayerBehaviors::Kill],
        }
    }
}

impl StatusType for DemonProtected {
    fn behavior_type(&self) -> Option<&[crate::engine::player::PlayerBehaviors]> {
        Some(&self.behaviors)
    }

    fn kill(&self, attacking_player_index: PlayerIndex, state: &State) -> Option<bool> {
        let attacking_player = state.get_player(attacking_player_index);
        if attacking_player.role.get_true_character_type() == CharacterType::Demon {
            return Some(false);
        }

        return None;
    }
}

impl Display for DemonProtected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Demon Protected")
    }
}

impl Role for Monk {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn night_order(&self) -> Option<usize> {
        Some(19)
    }

    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let dead = state.get_player(player_index).dead;
        if dead {
            return None;
        }
        let change_type = ChangeType::ChoosePlayers(1);
        let message = "Have the monk select a player to protect";

        let check_func = move |_: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
            let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(v) => v);

            let len = target_players.len();
            if len != 1 {
                return Err(ChangeError::WrongNumberOfSelectedPlayers {
                    wanted: 1,
                    got: len,
                });
            }

            // Make sure the monk can't protect themselves
            if target_players[0] == player_index {
                return Ok(false);
            }

            return Ok(true);
        };

        let state_change_func =
            move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
                // Check if there are any poisoned status effects inflicted by this player and clear
                // them

                let target_player_index =
                    unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv)[0];
                let target_player = state.get_player_mut(target_player_index);
                let status = StatusEffect::new(
                    Arc::new(DemonProtected::default()),
                    player_index,
                    CleanupPhase::Dawn.into(),
                );
                target_player.add_status(status);

                None
            };

        Some(ChangeRequest::new(
            change_type,
            message.into(),
            check_func,
            state_change_func,
        ))
    }
}

impl Display for Monk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Monk")
    }
}

#[derive(Default)]
pub(crate) struct Ravenkeeper {
    ability_used: bool,
}

impl Role for Ravenkeeper {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn night_order(&self) -> Option<usize> {
        // if player == dead and ability not used, then order
        // Otherwise no order
        // Or might be easier to do in ability
        Some(67)
    }

    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let player = state.get_player(player_index);

        if !player.dead || self.ability_used {
            return None;
        }

        let message = "Prompt the Ravenkeeper to point to a player";
        let change_type = ChangeType::ChoosePlayers(1);

        let check_func = move |_state: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
            let target_players = unwrap_args_err!(args, ChangeArgs::PlayerIndices(pv) => pv);
            let len = target_players.len();
            if len != 1 {
                return Err(ChangeError::WrongNumberOfSelectedPlayers {
                    wanted: 1,
                    got: len,
                });
            }

            return Ok(true);
        };
        let change_func = move |state: &mut State, args: ChangeArgs| -> Option<ChangeRequest> {
            let target_players = unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv);
            let target_player = state.get_player(target_players[0]);

            // Create a new change request using the role of the target player
            let change_type = ChangeType::Display;
            let message = format!(
                "Show the Ravenkeeper that they selected the {}",
                target_player.role
            );

            Some(ChangeRequest::new_display(change_type, message))
        };

        Some(ChangeRequest::new(
            change_type,
            message.into(),
            check_func,
            change_func,
        ))
    }
}

impl Display for Ravenkeeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ravenkeeper")
    }
}

#[derive(Default)]
pub(crate) struct Virgin {
    ability_used: bool,
}

impl Role for Virgin {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn nominated(&self, nominating_player_index: PlayerIndex, state: &mut State) {
        let nominator = state.get_player_mut(nominating_player_index);
        nominator.execute();
    }
}

impl Display for Virgin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Virgin")
    }
}
// TODO:
// Slayer

#[derive(Default)]
pub(crate) struct Soldier();

impl Role for Soldier {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    // Overwrite kill method for Soldier so they can't be killed by a demon
    fn kill(&self, attacking_player_index: PlayerIndex, state: &State) -> Option<bool> {
        let attacking_player = state.get_player(attacking_player_index);
        if attacking_player.role.get_true_character_type() == CharacterType::Demon {
            return Some(false);
        }

        None
    }
}

impl Display for Soldier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Soldier")
    }
}

#[derive(Default)]
struct Mayor();

impl Role for Mayor {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    // TODO: Overwrite kill for mayor. Perhaps kill should also trigger a change request or
    // something like that.
    fn kill(&self, _attacking_player_index: PlayerIndex, _state: &State) -> Option<bool> {
        todo!()
    }
}

impl Display for Mayor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Soldier")
    }
}
