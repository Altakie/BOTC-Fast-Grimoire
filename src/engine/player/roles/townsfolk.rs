use std::fmt::Display;
use std::sync::Arc;

use crate::engine::change_request::{ChangeResult, FilterFuncPtr, StateChangeFuncPtr};
use crate::engine::player::roles::RolePtr;
use crate::engine::state::log;
use crate::engine::state::status_effects::CleanupPhase;
use crate::engine::{
    change_request::{ChangeError, ChangeRequest, ChangeType, check_len},
    player::{Alignment, CharacterType, PlayerBehaviors, roles::Role},
    state::{
        PlayerIndex, State,
        status_effects::{StatusEffect, StatusType},
    },
};

fn washerwoman_librarian_investigator<
    RE: StatusType + Default + 'static,
    WE: StatusType + Default + 'static,
>(
    player_index: PlayerIndex,
    character_type: CharacterType,
) -> Option<ChangeRequest> {
    let right_description = format!("Select a {}", &character_type.to_string());
    let change_type = ChangeType::ChoosePlayers(1);

    let right_status =
        move || StatusEffect::new(std::sync::Arc::new(RE::default()), player_index, None);

    let wrong_status =
        move || StatusEffect::new(std::sync::Arc::new(WE::default()), player_index, None);

    let right_state_change = StateChangeFuncPtr::new(move |state, args| {
        let target_player_indices = args.extract_player_indicies()?;
        check_len(&target_player_indices, 1)?;

        let target_player = state.get_player_mut(target_player_indices[0]);
        target_player.add_status(right_status());

        return washerwoman_librarian_investigator_wrong(player_index, right_status, wrong_status);
    });

    return ChangeRequest::new(change_type, right_description, right_state_change).into();
}

fn washerwoman_librarian_investigator_wrong(
    player_index: PlayerIndex,
    right_status: impl Fn() -> StatusEffect + Send + Sync + 'static,
    wrong_status: impl Fn() -> StatusEffect + Send + Sync + 'static,
) -> ChangeResult {
    let wrong_description = "Select a different player";
    let wrong_change_type = ChangeType::ChoosePlayers(1);

    let wrong_state_change = StateChangeFuncPtr::new(move |state, args| {
        let target_player_indices = args.extract_player_indicies()?;
        check_len(&target_player_indices, 1)?;

        let target_player_index = target_player_indices[0];

        if target_player_index == player_index {
            return Err(ChangeError::InvalidSelectedPlayer {
                reason: "TODO".to_string(),
            });
        }

        let target_player = state.get_player(target_player_index);
        if target_player
            .get_statuses()
            .iter()
            .any(|se| *se == right_status())
        {
            return Err(ChangeError::InvalidSelectedPlayer {
                reason: "TODO".to_string(),
            });
        }

        // Assign the chosen player the wrong status effect
        let target_player = state.get_player_mut(target_player_indices[0]);
        target_player.add_status(wrong_status());

        Ok(None)
    });

    return ChangeRequest::new(
        wrong_change_type,
        wrong_description.into(),
        wrong_state_change,
    )
    .into();
}

#[derive(Default)]
pub(crate) struct Washerwoman();

#[derive(Default)]
struct WasherwomanTownsfolk();
impl StatusType for WasherwomanTownsfolk {}
impl Display for WasherwomanTownsfolk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Washerwoman Townsfolk")
    }
}

#[derive(Default)]
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
        washerwoman_librarian_investigator::<WasherwomanTownsfolk, WasherwomanWrong>(
            player_index,
            CharacterType::Townsfolk,
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

        ChangeRequest::new_display(change_type, message).into()
    }
}

impl Display for Washerwoman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Washerwoman")
    }
}

#[derive(Default)]
pub(crate) struct Librarian();

#[derive(Default)]
struct LibrarianOutsider();
impl StatusType for LibrarianOutsider {}
impl Display for LibrarianOutsider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Librarian Outsider")
    }
}

#[derive(Default)]
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
        state: &State,
    ) -> Option<ChangeRequest> {
        let outsider_count = state
            .get_players()
            .iter()
            .filter(|player| {
                matches!(
                    player.get_character_type(),
                    CharacterType::Outsider | CharacterType::Any
                )
            })
            .count();

        if outsider_count > 0 {
            return washerwoman_librarian_investigator::<LibrarianOutsider, LibrarianWrong>(
                player_index,
                CharacterType::Outsider,
            );
        }

        None
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

        let outsider_count = state
            .get_players()
            .iter()
            .filter(|player| {
                matches!(
                    player.get_character_type(),
                    CharacterType::Outsider | CharacterType::Any
                )
            })
            .count();

        let message = {
            if outsider_count == 0 {
                "Show the Librarian there are no outsiders in play".to_string()
            } else {
                format!("Show the {} the correct roles", player.role)
            }
        };
        let change_type = ChangeType::Display;

        ChangeRequest::new_display(change_type, message).into()
    }
}

impl Display for Librarian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Librarian")
    }
}

#[derive(Default)]
pub(crate) struct Investigator();

#[derive(Default)]
struct InvestigatorMinion();
impl StatusType for InvestigatorMinion {}
impl Display for InvestigatorMinion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Investigator Minion")
    }
}

#[derive(Default)]
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
        washerwoman_librarian_investigator::<InvestigatorMinion, InvestigatorWrong>(
            player_index,
            CharacterType::Minion,
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

        ChangeRequest::new_display(change_type, message).into()
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

        ChangeRequest::new_display(change_type, message).into()
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

        ChangeRequest::new_display(change_type, message).into()
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

        let state_change_func = StateChangeFuncPtr::new(move |state, args| {
            let target_player_indicies = args.extract_player_indicies()?;

            check_len(&target_player_indicies, 2)?;

            // Make sure there are no duplicate players
            if target_player_indicies[0] == target_player_indicies[1] {
                return Err(ChangeError::InvalidSelectedPlayer {
                    reason: "Please select unique players".into(),
                });
            }

            // Calculate whether any of the chosen players are either a red herring or a demon
            let demon_found = target_player_indicies.iter().any(|i| {
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

            ChangeRequest::new_display(change_type, message).into()
        });

        ChangeRequest::new(change_type, message.into(), state_change_func).into()
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
        // Get storyteller input on who red-herring is
        // Add a red-herring through status effects
        let state_change = StateChangeFuncPtr::new(move |state, args| {
            let target_player_indices = args.extract_player_indicies()?;

            check_len(&target_player_indices, 1)?;

            if target_player_indices[0] == player_index {
                return Err(ChangeError::InvalidSelectedPlayer {
                    reason: "Cannot select the fortune teller as their own red-herring".into(),
                });
            }

            let target_player_index = target_player_indices[0];
            let target_player = state.get_player_mut(target_player_index);
            let status = StatusEffect::new(Arc::new(FortunetellerRedHerring()), player_index, None);
            target_player.add_status(status);

            Ok(None)
        });

        ChangeRequest::new(change_type, description, state_change).into()
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

#[derive(Default)]
pub(crate) struct Undertaker();

impl Role for Undertaker {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn night_order(&self) -> Option<usize> {
        Some(70)
    }

    fn night_ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequest> {
        let dead = state.get_player(player_index).dead;

        if dead {
            return None;
        }

        let execution_event = state.log.search_previous_phase(|e| match *e {
            log::Event::Execution(_) => Some(e),
            _ => None,
        });

        let executed_player_index = match execution_event {
            Ok(log::Event::Execution(player_index)) => *player_index,
            Ok(_) | Err(_) => return None,
        };

        let executed_role = state.get_player(executed_player_index).role.clone();

        let change_type = ChangeType::Display;
        let description = format!(
            "Show the undertaker that the {} was executed yesterday",
            executed_role
        );

        ChangeRequest::new_display(change_type, description).into()
    }
}

impl Display for Undertaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Undertaker")
    }
}

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

    fn kill(
        &self,
        attacking_player_index: PlayerIndex,
        _target_player_index: PlayerIndex,
        state: &State,
    ) -> Option<bool> {
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

        let filter_func = FilterFuncPtr::new(move |pi, _| {
            if pi == player_index {
                return false;
            }

            return true;
        });

        let state_change_func = StateChangeFuncPtr::new(move |state, args| {
            // Check if there are any poisoned status effects inflicted by this player and clear
            // them
            let target_player_indices = args.extract_player_indicies()?;

            check_len(&target_player_indices, 1)?;

            // Make sure the monk can't protect themselves
            if target_player_indices[0] == player_index {
                return Err(ChangeError::InvalidSelectedPlayer {
                    reason: "Monk cannot protect themselves".into(),
                });
            }

            let target_player = state.get_player_mut(target_player_indices[0]);
            let status = StatusEffect::new(
                Arc::new(DemonProtected::default()),
                player_index,
                CleanupPhase::Dawn.into(),
            );
            target_player.add_status(status);

            Ok(None)
        });

        ChangeRequest::new_with_filter(change_type, message.into(), filter_func, state_change_func)
            .into()
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

        if self.ability_used || !player.dead {
            return None;
        }

        let message = "Prompt the Ravenkeeper to point to a player";
        let change_type = ChangeType::ChoosePlayers(1);

        let change_func = StateChangeFuncPtr::new(move |state, args| {
            let target_player_indices = args.extract_player_indicies()?;
            check_len(&target_player_indices, 1)?;

            state
                .get_player_mut(player_index)
                .role
                .reassign(RolePtr::from(Ravenkeeper { ability_used: true }));

            let target_player = state.get_player(target_player_indices[0]);

            // Create a new change request using the role of the target player
            let change_type = ChangeType::Display;
            let message = format!(
                "Show the Ravenkeeper that they selected the {}",
                target_player.role
            );

            ChangeRequest::new_display(change_type, message).into()
        });

        ChangeRequest::new(change_type, message.into(), change_func).into()
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

    // TODO: Want to make this method more idiomatic
    fn nominated(
        &self,
        nominating_player_index: PlayerIndex,
        target_player_index: PlayerIndex,
        state: &mut State,
    ) {
        if self.ability_used {
            return;
        }

        let player = state.get_player_mut(target_player_index);
        player
            .role
            .reassign(RolePtr::from(Virgin { ability_used: true }));
        // FIX: Doesn't account for drunkness or poisoned (bad account for drunkness)
        let nominator = state.get_player_mut(nominating_player_index);
        if nominator.role.get_true_character_type() == CharacterType::Townsfolk {
            state.execute_player(nominating_player_index);
        }
    }
}

impl Display for Virgin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Virgin")
    }
}
#[derive(Default)]
pub(crate) struct Slayer {
    ability_used: bool,
}

impl Role for Slayer {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn has_day_ability(&self) -> bool {
        if self.ability_used {
            return false;
        }

        true
    }

    fn day_ability(&self, player_index: PlayerIndex, _state: &State) -> Option<ChangeRequest> {
        // Choose a player
        // If it is a demon, kill the demon, otherwise do nothing
        // Either way, use your ability

        let change_type = ChangeType::ChoosePlayers(1);
        let description = "Prompt the slayer to point to a player";

        let change_func = StateChangeFuncPtr::new(move |state, args| {
            let target_player_indices = args.extract_player_indicies()?;
            check_len(&target_player_indices, 1)?;

            let slayer = state.get_player_mut(player_index);
            slayer
                .role
                .reassign(RolePtr::from(Self { ability_used: true }));

            let target_player = state.get_player_mut(target_player_indices[0]);

            if target_player.get_character_type() == CharacterType::Demon {
                return state.kill(player_index, target_player_indices[0]);
            }

            Ok(None)
        });

        ChangeRequest::new(change_type, description.into(), change_func).into()
    }
}

impl Display for Slayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Slayer")
    }
}

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
    fn kill(
        &self,
        attacking_player_index: PlayerIndex,
        _target_player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeResult> {
        let attacking_player = state.get_player(attacking_player_index);
        if attacking_player.role.get_true_character_type() == CharacterType::Demon {
            return Some(Ok(None));
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
pub(crate) struct Mayor();

impl Role for Mayor {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn kill(
        &self,
        attacking_player_index: PlayerIndex,
        player_index: PlayerIndex,
        _state: &State,
    ) -> Option<ChangeResult> {
        let change_type = ChangeType::ChoosePlayers(1);
        let description = "Choose a player to die (the mayor may bounce a kill)";

        let change_func = StateChangeFuncPtr::new(move |state, args| {
            let target_player_indices = args.extract_player_indicies()?;
            check_len(&target_player_indices, 1)?;

            let target_player_index = target_player_indices[0];

            // Stop infinite loop of mayor bouncing kills
            if target_player_index == player_index {
                state.get_player_mut(player_index).dead = true;
                return Ok(None);
            }

            return state.kill(attacking_player_index, target_player_index);
        });

        Some(ChangeRequest::new(change_type, description.into(), change_func).into())
    }
}

impl Display for Mayor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mayor")
    }
}

#[cfg(test)]
mod test {
    use crate::{engine::player::roles::Roles, scripts::trouble_brewing};

    use super::*;

    fn setup_test_state(roles: Vec<Roles>) -> State {
        let player_names = roles
            .iter()
            .map(|role| role.convert().to_string())
            .collect();
        State::new(roles, player_names, trouble_brewing()).unwrap()
    }

    #[test]
    fn test_undertaker_ability() {
        let roles = vec![
            Roles::Undertaker,
            Roles::Virgin,
            Roles::Soldier,
            Roles::Spy,
            Roles::Imp,
        ];
        let mut state = setup_test_state(roles);
        let undertaker_role = RolePtr::new::<Undertaker>();
        let undertaker_index = state
            .get_players()
            .iter()
            .position(|player| player.role.to_string() == "Undertaker")
            .expect("Undertaker not found");

        let cr = undertaker_role.night_ability(undertaker_index, &state);

        // FIX: This unit test sucks, probably need to refactor code, or use dependency injection
        // to have a dummy state
        assert!(cr.is_none());
        state.next_step();
        state.next_step();
        state.next_step();

        todo!()
    }
}
