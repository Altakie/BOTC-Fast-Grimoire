use std::fmt::Display;
use std::sync::Arc;

use leptos::attr::target;
use macros::washerwoman_librarian_investigator;

use crate::{
    engine::{
        change_request::{
            ChangeArgs, ChangeError, ChangeRequest, ChangeType, CheckFuncPtr, StateChangeFuncPtr,
        },
        player::{Alignment, CharacterType, PlayerBehaviors, roles::Role},
        state::{
            PlayerIndex, State,
            status_effects::{StatusEffect, StatusType},
        },
    },
    new_change_request, unwrap_args_err, unwrap_args_panic,
};

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
        washerwoman_librarian_investigator!(
            player_index,
            WasherwomanTownsfolk,
            WasherwomanWrong,
            "Townsfolk"
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

        Some(new_change_request!(change_type, message))
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
        CharacterType::Outsider
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
            "Outsider"
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

        Some(new_change_request!(change_type, message))
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
        CharacterType::Minion
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
            "Minion"
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

        Some(new_change_request!(change_type, message))
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

        Some(new_change_request!(change_type, message))
    }
}

impl Display for Chef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Chef")
    }
}

struct Empath {}

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

        Some(new_change_request!(change_type, message))
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
        self.ability(player_index, state)
    }
}

impl Display for Empath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Empath")
    }
}

pub(crate) struct Fortuneteller();

struct FortunetellerRedHerring();
impl StatusType for FortunetellerRedHerring {}
impl Display for FortunetellerRedHerring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Fortuneteller Red Herring")
    }
}

impl Fortuneteller {
    fn ability(&self, _player_index: PlayerIndex, _state: &State) -> Option<ChangeRequest> {
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
            let status = StatusEffect::new(Arc::new(FortunetellerRedHerring()), player_index);
            target_player.add_status(status);

            None
        };

        Some(new_change_request!(
            change_type,
            description,
            check_func,
            state_change
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
        if attacking_player.get_character_type() == CharacterType::Demon {
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

    fn night_ability(&self, player_index: PlayerIndex, _state: &State) -> Option<ChangeRequest> {
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

                state.cleanup_statuses(player_index);

                let target_player_index =
                    unwrap_args_panic!(args, ChangeArgs::PlayerIndices(pv) => pv)[0];
                let target_player = state.get_player_mut(target_player_index);
                let status = StatusEffect::new(Arc::new(DemonProtected::default()), player_index);
                target_player.add_status(status);

                None
            };

        Some(new_change_request!(
            change_type,
            message,
            check_func,
            state_change_func
        ))
    }
}

impl Display for Monk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Monk")
    }
}

// TODO:
// Ravenkeeper (need to implement triggers or smth). Or more than likely, need to somehow hook up
// the night order ability to the state of the player (somehow), or store some internal state. Or
// change the main loop to skip None change effects

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

        let check_func = move |state: &State, args: &ChangeArgs| -> Result<bool, ChangeError> {
            // TODO: Check that only one player is selected
            todo!()
        };
        let change_func = move |state: &mut State, args: ChangeArgs| {
            // TODO: Shouldn't actually change anything, but should create another change request
            // that causes a display of the selected player's role
            todo!()
        };

        todo!()
    }
}

impl Display for Ravenkeeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ravenkeeper")
    }
}

pub(crate) struct Virgin {
    ability_used: bool,
}

impl Default for Virgin {
    fn default() -> Self {
        Self {
            ability_used: false,
        }
    }
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
struct Soldier();

impl Role for Soldier {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    // TODO: Overwrite kill method for Soldier so they can't be killed by a demon
    fn kill(&self, _attacking_player_index: PlayerIndex, _state: &State) -> Option<bool> {
        todo!()
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
    fn kill(&self, _attacking_player_index: PlayerIndex, state: &State) -> Option<bool> {
        todo!()
    }
}

impl Display for Mayor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Soldier")
    }
}
