use std::fmt::Display;

use tracing::error;

use crate::engine::{
    change_request::{
        ChangeError, ChangeRequest, ChangeRequestBuilder, ChangeResult, ChangeType, FilterFuncPtr,
        StateChangeFuncPtr, check_len,
    },
    player::{
        Alignment, CharacterType, PlayerBehaviors,
        roles::{Role, Roles},
    },
    state::{
        EventListener, PlayerIndex, State,
        log::{self, AttemptedKill, Event, Nomination},
        status_effects::{CleanupPhase, StatusEffect, StatusType},
    },
};

fn washerwoman_librarian_investigator(
    player_index: PlayerIndex,
    character_type: CharacterType,
    right_status: StatusType,
    wrong_status: StatusType,
) -> Option<ChangeRequestBuilder> {
    let right_status = move || StatusEffect::new(right_status, player_index, None);

    let wrong_status = move || StatusEffect::new(wrong_status, player_index, None);

    return ChangeRequest::new_builder(
        ChangeType::ChoosePlayers(1),
        format!("Select a {}", &character_type.to_string()),
    )
    .state_change_func(StateChangeFuncPtr::new(move |state, args| {
        let target_player_indices = args.extract_player_indicies()?;
        check_len(&target_player_indices, 1)?;

        state.add_status(right_status(), target_player_indices[0]);

        state
            .change_request_queue
            .push_back(washerwoman_librarian_investigator_wrong(
                player_index,
                right_status,
                wrong_status,
            ));

        Ok(())
    }))
    .into();
}

fn washerwoman_librarian_investigator_wrong(
    player_index: PlayerIndex,
    right_status: impl Fn() -> StatusEffect + Send + Sync + 'static,
    wrong_status: impl Fn() -> StatusEffect + Send + Sync + 'static,
) -> ChangeRequestBuilder {
    return ChangeRequest::new_builder(
        ChangeType::ChoosePlayers(1),
        "Select a different player".into(),
    )
    .state_change_func(StateChangeFuncPtr::new(move |state, args| {
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
        state.add_status(wrong_status(), target_player_indices[0]);

        Ok(())
    }));
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Washerwoman();

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
    ) -> Option<ChangeRequestBuilder> {
        washerwoman_librarian_investigator(
            player_index,
            CharacterType::Townsfolk,
            StatusType::WasherwomanTownsfolk,
            StatusType::WasherwomanWrong,
        )
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(45)
    }

    fn night_one_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let player = state.get_player(player_index);
        ChangeRequest::new_builder(
            ChangeType::Display,
            format!("Show the {} the correct roles", player.role),
        )
        .into()
    }
}

impl Display for Washerwoman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Washerwoman")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Librarian();

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
    ) -> Option<ChangeRequestBuilder> {
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
            return washerwoman_librarian_investigator(
                player_index,
                CharacterType::Outsider,
                StatusType::LibrarianOutsider,
                StatusType::LibrarianWrong,
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
    ) -> Option<ChangeRequestBuilder> {
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

        ChangeRequest::new_builder(ChangeType::Display, {
            if outsider_count == 0 {
                "Show the Librarian there are no outsiders in play".to_string()
            } else {
                format!("Show the {} the correct roles", player.role)
            }
        })
        .into()
    }
}

impl Display for Librarian {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Librarian")
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Investigator();

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
    ) -> Option<ChangeRequestBuilder> {
        washerwoman_librarian_investigator(
            player_index,
            CharacterType::Minion,
            StatusType::InvestigatorMinion,
            StatusType::InvestigatorWrong,
        )
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(47)
    }

    fn night_one_ability(
        &self,
        player_index: crate::engine::state::PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let player = state.get_player(player_index);

        ChangeRequest::new_builder(
            ChangeType::Display,
            format!("Show the {} the correct roles", player.role),
        )
        .into()
    }
}

impl Display for Investigator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Investigator")
    }
}

#[derive(Default, Debug, Clone)]
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
    ) -> Option<ChangeRequestBuilder> {
        // Count pairs of evil players
        // For each evil, player, check if the right player is evil, if yes, increment the
        // pair count
        let players = state.get_players();

        let pair_count = players
            .iter()
            .enumerate()
            .filter(|(pi, player)| {
                let right_player = state.get_player(state.right_player(*pi));
                player.alignment == Alignment::Evil && right_player.alignment == Alignment::Evil
            })
            .count();

        ChangeRequest::new_builder(
            ChangeType::Display,
            format!(
                "Show the chef that there are {} pairs of evil players",
                pair_count
            ),
        )
        .into()
    }
}

impl Display for Chef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Chef")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Empath();

impl Empath {
    fn ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequestBuilder> {
        // Check how many players next to the empath are evil
        let mut count = 0;
        let left_player = state.get_player(state.left_player(player_index));
        if left_player.alignment == Alignment::Evil {
            count += 1;
        }
        let right_player = state.get_player(state.right_player(player_index));
        if right_player.alignment == Alignment::Evil {
            count += 1;
        }

        ChangeRequest::new_builder(
            ChangeType::Display,
            format!("Empath has {} evil neighbors", count),
        )
        .into()
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

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        self.ability(player_index, state)
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
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

#[derive(Default, Debug, Clone)]
pub(crate) struct Fortuneteller();

impl Fortuneteller {
    fn ability(&self, player_index: PlayerIndex, state: &State) -> Option<ChangeRequestBuilder> {
        let dead = state.get_player(player_index).dead;
        if dead {
            return None;
        }

        ChangeRequest::new_builder(
            ChangeType::ChoosePlayers(2),
            "Prompt the FortuneTeller to point to two players".into(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
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
                        && matches!(se.status_type, StatusType::FortuneTellerRedHerring)
                })
            });
            state
                .change_request_queue
                .push_back(ChangeRequest::new_builder(
                    ChangeType::Display,
                    format!(
                        "Show the Fortuneteller a {}",
                        match demon_found {
                            true => "Thumbs Up",
                            false => "Thumbs Down",
                        }
                    ),
                ));
            Ok(())
        }))
        .into()
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

    fn setup_ability(
        &self,
        player_index: PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequestBuilder> {
        // Get storyteller input on who red-herring is
        // Add a red-herring through status effects
        ChangeRequest::new_builder(
            ChangeType::ChoosePlayers(1),
            "Select a red-herring for the Fortune Teller".to_string(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let target_player_indices = args.extract_player_indicies()?;

            check_len(&target_player_indices, 1)?;

            if target_player_indices[0] == player_index {
                return Err(ChangeError::InvalidSelectedPlayer {
                    reason: "Cannot select the fortune teller as their own red-herring".into(),
                });
            }

            let target_player_index = target_player_indices[0];
            let status = StatusEffect::new(StatusType::FortuneTellerRedHerring, player_index, None);
            state.add_status(status, target_player_index);

            Ok(())
        }))
        .into()
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(50)
    }

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        self.ability(player_index, state)
    }

    fn night_order(&self) -> Option<usize> {
        Some(69)
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        self.ability(player_index, state)
    }
}

impl Display for Fortuneteller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Fortuneteller")
    }
}

#[derive(Default, Debug, Clone)]
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

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let dead = state.get_player(player_index).dead;

        if dead {
            return None;
        }

        let execution_event = state.log.search_previous_phase(|e| match *e {
            Event::Execution(_) => Some(e),
            _ => None,
        });

        let executed_player_index = match execution_event {
            Ok(Event::Execution(player_index)) => *player_index,
            Ok(_) | Err(_) => return None,
        };

        let executed_role = state.get_player(executed_player_index).role.clone();

        ChangeRequest::new_builder(
            ChangeType::Display,
            format!(
                "Show the undertaker that the {} was executed yesterday",
                executed_role
            ),
        )
        .into()
    }
}

impl Display for Undertaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Undertaker")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Monk();

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

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let dead = state.get_player(player_index).dead;
        if dead {
            return None;
        }
        ChangeRequest::new_builder(
            ChangeType::ChoosePlayers(1),
            "Have the monk select a player to protect".into(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
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

            let status = StatusEffect::new(
                StatusType::DemonProtected,
                player_index,
                CleanupPhase::Dawn.into(),
            );
            state.add_status(status, target_player_indices[0]);

            Ok(())
        }))
        .filter_func(FilterFuncPtr::new(move |pi, _| pi != player_index))
        .into()
    }
}

impl Display for Monk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Monk")
    }
}

#[derive(Default, Debug, Clone)]
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

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let death_event = state.log.search_current_phase(|event| match event {
            Event::Death(pi) => {
                if *pi == player_index {
                    Some(event)
                } else {
                    None
                }
            }
            _ => None,
        });

        if death_event.is_err() || self.ability_used {
            return None;
        }

        ChangeRequest::new_builder(
            ChangeType::ChoosePlayers(1),
            "Prompt the Ravenkeeper to point to a player".into(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let target_player_indices = args.extract_player_indicies()?;
            check_len(&target_player_indices, 1)?;

            state.get_player_mut(player_index).role =
                Roles::Ravenkeeper(Ravenkeeper { ability_used: true });

            let target_player = state.get_player(target_player_indices[0]);

            // Create a new change request using the role of the target player
            state
                .change_request_queue
                .push_back(ChangeRequest::new_builder(
                    ChangeType::Display,
                    format!(
                        "Show the Ravenkeeper that they selected the {}",
                        target_player.role
                    ),
                ));

            Ok(())
        }))
        .into()
    }
}

impl Display for Ravenkeeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ravenkeeper")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Virgin {}

impl Role for Virgin {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn initialize(&self, player_index: PlayerIndex, state: &mut State) {
        let virgin_listener = EventListener::new(
            player_index,
            |event_listener_state, state, nomination_event: Nomination| {
                if nomination_event.target_player_index != event_listener_state.source_player_index
                {
                    return state;
                }

                let source_player_index = event_listener_state.source_player_index;
                state.change_request_queue.push_back(
                    ChangeRequest::new_builder(ChangeType::NoStoryteller, String::new())
                        .state_change_func(StateChangeFuncPtr::new(move |state, _| {
                            // FIX: Doesn't account for drunkness or poisoned (bad account for drunkness)
                            let nominator =
                                state.get_player_mut(nomination_event.nominator_player_index);
                            if nominator.role.get_true_character_type() == CharacterType::Townsfolk
                            {
                                state.execute_player(nomination_event.nominator_player_index);
                            }
                            state.cleanup_event_listeners(source_player_index);
                            Ok(())
                        })),
                );

                state
            },
        );

        state.nomination_listeners.push(virgin_listener);
    }
}

impl Display for Virgin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Virgin")
    }
}
#[derive(Default, Debug, Clone)]
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

    fn day_ability(
        &self,
        player_index: PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequestBuilder> {
        // Choose a player
        // If it is a demon, kill the demon, otherwise do nothing
        // Either way, use your ability

        ChangeRequest::new_builder(
            ChangeType::ChoosePlayers(1),
            "Prompt the slayer to point to a player".into(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let target_player_indices = args.extract_player_indicies()?;
            check_len(&target_player_indices, 1)?;

            let slayer = state.get_player_mut(player_index);
            slayer.role = Roles::Slayer(Self { ability_used: true });

            let target_player = state.get_player_mut(target_player_indices[0]);

            if target_player.get_character_type() == CharacterType::Demon {
                state.kill(player_index, target_player_indices[0]);
            }

            Ok(())
        }))
        .into()
    }
}

impl Display for Slayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Slayer")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Soldier();

impl Role for Soldier {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    fn initialize(&self, player_index: PlayerIndex, state: &mut State) {
        // WARN: No indicator that the soldier is demon protected
        // state.add_status(
        //     StatusEffect::new(StatusType::DemonProtected, player_index, None),
        //     player_index,
        // );
        state.attempted_kill_listeners.push(EventListener::new(
            player_index,
            |ev_state, state, event: log::AttemptedKill| {
                if event.target_player_index == ev_state.source_player_index {
                    state.prevent_kill_default = true
                }
                state
            },
        ));
    }

    // Overwrite kill method for Soldier so they can't be killed by a demon
    // fn kill(
    //     &self,
    //     attacking_player_index: PlayerIndex,
    //     _target_player_index: PlayerIndex,
    //     state: &State,
    // ) -> Option<ChangeResult> {
    //     let attacking_player = state.get_player(attacking_player_index);
    //     if attacking_player.role.get_true_character_type() == CharacterType::Demon {
    //         return Some(Ok(None));
    //     }
    //
    //     None
    // }
}

impl Display for Soldier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Soldier")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Mayor();

impl Role for Mayor {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Townsfolk
    }

    // TODO: Test this
    fn initialize(&self, player_index: PlayerIndex, state: &mut State) {
        let mayor_listener = EventListener::new(
            player_index,
            move |event_listener_state, state, attempted_kill_event: AttemptedKill| {
                error!("I was called");
                if attempted_kill_event.target_player_index
                    != event_listener_state.source_player_index
                {
                    return state;
                }

                state.prevent_kill_default = true;

                state.change_request_queue.push_back(
                    ChangeRequest::new_builder(
                        ChangeType::ChoosePlayers(1),
                        "Choose a player to die (the mayor may bounce a kill)".into(),
                    )
                    .state_change_func(StateChangeFuncPtr::new(
                        move |state, args| {
                            let target_player_indices = args.extract_player_indicies()?;
                            check_len(&target_player_indices, 1)?;

                            let target_player_index = target_player_indices[0];

                            // Stop infinite loop of mayor bouncing kills
                            if target_player_index == player_index {
                                state.get_player_mut(player_index).dead = true;
                                state.handle_death(player_index);
                                return Ok(());
                            }

                            state.kill(
                                attempted_kill_event.attacking_player_index,
                                target_player_index,
                            );

                            Ok(())
                        },
                    )),
                );
                state
            },
        );

        state.attempted_kill_listeners.push(mayor_listener);
    }
}

impl Display for Mayor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mayor")
    }
}

#[cfg(test)]
mod test {
    use crate::engine::change_request::ChangeArgs;
    use crate::engine::player::roles::RoleNames;
    use crate::engine::player::roles::test_utils::{find_role, setup_test_state};

    use super::*;

    #[test]
    fn test_monk_protection() {
        // Deliberately NOT using Soldier as the protected target: Soldier's own ability
        // applies a permanent DemonProtected status in `initialize()` (see townsfolk.rs),
        // which would make this assertion pass even if Monk's ability were a no-op.
        let roles = vec![RoleNames::Monk, RoleNames::Investigator, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let monk_index = find_role(&state, RoleNames::Monk);
        let imp_index = find_role(&state, RoleNames::Imp);
        let target_index = find_role(&state, RoleNames::Investigator);

        // Monk protects the Investigator
        let monk_role = Roles::new(&RoleNames::Monk);
        let cr = monk_role.night_ability(monk_index, &state).unwrap();

        // Execute the change request
        let args = crate::engine::change_request::ChangeArgs::PlayerIndices(vec![target_index]);
        cr.state_change_func
            .unwrap()
            .call(&mut state, args)
            .unwrap();

        assert!(
            state
                .get_player(target_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::DemonProtected)
        );

        // Imp kills the Investigator
        state.kill(imp_index, target_index);

        assert!(
            !state.get_player(target_index).dead,
            "the protected player should survive"
        );
    }

    #[test]
    fn test_monk_poisoned_protection_blocked() {
        // Deliberately NOT using Soldier as the protected target: Soldier's own ability
        // applies a permanent DemonProtected status in `initialize()`, which would make the
        // "no protection applied" assertion below meaningless (it'd already be present
        // regardless of Monk).
        let roles = vec![RoleNames::Monk, RoleNames::Investigator, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let monk_index = find_role(&state, RoleNames::Monk);
        let imp_index = find_role(&state, RoleNames::Imp);
        let target_index = find_role(&state, RoleNames::Investigator);

        // Poison the Monk
        state.add_status(
            StatusEffect::new(StatusType::Poisoned, imp_index, None),
            monk_index,
        );

        // Monk attempts to protect the Investigator. Go through the Player wrapper (not the
        // raw Roles::night_ability), since poison suppression is applied centrally there via
        // `drunkify` -- calling the raw role method bypasses it entirely.
        let cr = state
            .get_player(monk_index)
            .night_ability(monk_index, &state)
            .unwrap();

        // Execute the change request - should fail or not apply protection because they are poisoned
        let args = crate::engine::change_request::ChangeArgs::PlayerIndices(vec![target_index]);
        cr.state_change_func
            .unwrap()
            .call(&mut state, args)
            .unwrap();

        // Protection status should NOT be applied because Monk is poisoned
        assert!(
            !state
                .get_player(target_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::DemonProtected)
        );

        // Imp kills the Investigator
        state.kill(imp_index, target_index);

        assert!(
            state.get_player(target_index).dead,
            "the target should die because protection was blocked by poison"
        );
    }

    // -- Washerwoman --

    #[test]
    fn test_washerwoman_marks_townsfolk_and_wrong_status() {
        let roles = vec![
            RoleNames::Washerwoman,
            RoleNames::Chef,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let ww_index = find_role(&state, RoleNames::Washerwoman);
        let chef_index = find_role(&state, RoleNames::Chef);
        let saint_index = find_role(&state, RoleNames::Saint);

        let cr = state
            .get_player(ww_index)
            .setup_ability(ww_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![chef_index]))
            .unwrap();

        assert!(
            state
                .get_player(chef_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::WasherwomanTownsfolk)
        );
        assert_eq!(
            state.change_request_queue.len(),
            1,
            "the 'select a different player' follow-up should be queued"
        );

        let wrong_cr = state.change_request_queue.pop_front().unwrap();
        wrong_cr
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![saint_index]))
            .unwrap();

        assert!(
            state
                .get_player(saint_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::WasherwomanWrong)
        );
    }

    #[test]
    fn test_washerwoman_wrong_pick_rejects_same_player_as_townsfolk_pick() {
        let roles = vec![
            RoleNames::Washerwoman,
            RoleNames::Chef,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let ww_index = find_role(&state, RoleNames::Washerwoman);
        let chef_index = find_role(&state, RoleNames::Chef);

        let cr = state
            .get_player(ww_index)
            .setup_ability(ww_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![chef_index]))
            .unwrap();

        let wrong_cr = state.change_request_queue.pop_front().unwrap();
        let result = wrong_cr
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![chef_index]));

        assert!(
            result.is_err(),
            "should not be able to pick the same player already marked as the Townsfolk"
        );
    }

    // -- Librarian --

    #[test]
    fn test_librarian_marks_outsider_and_wrong_status() {
        let roles = vec![
            RoleNames::Librarian,
            RoleNames::Saint,
            RoleNames::Chef,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let lib_index = find_role(&state, RoleNames::Librarian);
        let saint_index = find_role(&state, RoleNames::Saint);
        let chef_index = find_role(&state, RoleNames::Chef);

        let cr = state
            .get_player(lib_index)
            .setup_ability(lib_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![saint_index]))
            .unwrap();

        assert!(
            state
                .get_player(saint_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::LibrarianOutsider)
        );

        let wrong_cr = state.change_request_queue.pop_front().unwrap();
        wrong_cr
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![chef_index]))
            .unwrap();

        assert!(
            state
                .get_player(chef_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::LibrarianWrong)
        );
    }

    #[test]
    fn test_librarian_no_outsiders_in_play_has_no_setup_ability_and_says_so() {
        let roles = vec![
            RoleNames::Librarian,
            RoleNames::Chef,
            RoleNames::Investigator,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let lib_index = find_role(&state, RoleNames::Librarian);

        assert!(
            state
                .get_player(lib_index)
                .setup_ability(lib_index, &state)
                .is_none(),
            "no outsiders in play means there is no one to mark"
        );

        let cr = state
            .get_player(lib_index)
            .night_one_ability(lib_index, &state)
            .unwrap();
        assert!(
            cr.description.to_lowercase().contains("no outsiders"),
            "got: {}",
            cr.description
        );
    }

    #[test]
    fn test_librarian_counts_drunk_as_outsider() {
        let roles = vec![
            RoleNames::Librarian,
            RoleNames::Drunk,
            RoleNames::Chef,
            RoleNames::Imp,
        ];
        let state = setup_test_state(roles);

        let lib_index = find_role(&state, RoleNames::Librarian);

        assert!(
            state
                .get_player(lib_index)
                .setup_ability(lib_index, &state)
                .is_some(),
            "the Drunk should count as an Outsider for the Librarian"
        );
    }

    // -- Investigator --

    #[test]
    fn test_investigator_marks_minion_and_wrong_status() {
        let roles = vec![
            RoleNames::Investigator,
            RoleNames::Poisoner,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let inv_index = find_role(&state, RoleNames::Investigator);
        let poisoner_index = find_role(&state, RoleNames::Poisoner);
        let saint_index = find_role(&state, RoleNames::Saint);

        let cr = state
            .get_player(inv_index)
            .setup_ability(inv_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![poisoner_index]))
            .unwrap();

        assert!(
            state
                .get_player(poisoner_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::InvestigatorMinion)
        );

        let wrong_cr = state.change_request_queue.pop_front().unwrap();
        wrong_cr
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![saint_index]))
            .unwrap();

        assert!(
            state
                .get_player(saint_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::InvestigatorWrong)
        );
    }

    #[test]
    fn test_investigator_wrong_pick_rejects_same_player_as_minion_pick() {
        let roles = vec![
            RoleNames::Investigator,
            RoleNames::Poisoner,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let inv_index = find_role(&state, RoleNames::Investigator);
        let poisoner_index = find_role(&state, RoleNames::Poisoner);

        let cr = state
            .get_player(inv_index)
            .setup_ability(inv_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![poisoner_index]))
            .unwrap();

        let wrong_cr = state.change_request_queue.pop_front().unwrap();
        let result = wrong_cr
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![poisoner_index]));

        assert!(result.is_err());
    }

    // -- Chef --

    #[test]
    fn test_chef_counts_single_adjacent_evil_pair() {
        let roles = vec![
            RoleNames::Chef,
            RoleNames::Investigator,
            RoleNames::Empath,
            RoleNames::Undertaker,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let chef_index = find_role(&state, RoleNames::Chef);

        let n = state.get_players().len();
        for i in 0..n {
            state.get_player_mut(i).alignment = Alignment::Good;
        }
        state.get_player_mut(0).alignment = Alignment::Evil;
        state.get_player_mut(1).alignment = Alignment::Evil;

        let cr = state
            .get_player(chef_index)
            .night_one_ability(chef_index, &state)
            .unwrap();
        assert!(
            cr.description.contains("there are 1 pairs"),
            "got: {}",
            cr.description
        );
    }

    #[test]
    fn test_chef_counts_wraparound_adjacent_pair() {
        let roles = vec![
            RoleNames::Chef,
            RoleNames::Investigator,
            RoleNames::Empath,
            RoleNames::Undertaker,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let chef_index = find_role(&state, RoleNames::Chef);

        let n = state.get_players().len();
        for i in 0..n {
            state.get_player_mut(i).alignment = Alignment::Good;
        }
        state.get_player_mut(n - 1).alignment = Alignment::Evil;
        state.get_player_mut(0).alignment = Alignment::Evil;

        let cr = state
            .get_player(chef_index)
            .night_one_ability(chef_index, &state)
            .unwrap();
        assert!(
            cr.description.contains("there are 1 pairs"),
            "got: {}",
            cr.description
        );
    }

    #[test]
    fn test_chef_counts_player_in_two_pairs() {
        let roles = vec![
            RoleNames::Chef,
            RoleNames::Investigator,
            RoleNames::Empath,
            RoleNames::Undertaker,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let chef_index = find_role(&state, RoleNames::Chef);

        let n = state.get_players().len();
        for i in 0..n {
            state.get_player_mut(i).alignment = Alignment::Good;
        }
        state.get_player_mut(0).alignment = Alignment::Evil;
        state.get_player_mut(1).alignment = Alignment::Evil;
        state.get_player_mut(2).alignment = Alignment::Evil;

        let cr = state
            .get_player(chef_index)
            .night_one_ability(chef_index, &state)
            .unwrap();
        assert!(
            cr.description.contains("there are 2 pairs"),
            "got: {}",
            cr.description
        );
    }

    // -- Empath --

    #[test]
    fn test_empath_zero_evil_neighbors() {
        let roles = vec![
            RoleNames::Empath,
            RoleNames::Investigator,
            RoleNames::Chef,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let empath_index = find_role(&state, RoleNames::Empath);
        let left = state.left_player(empath_index);
        let right = state.right_player(empath_index);
        state.get_player_mut(left).alignment = Alignment::Good;
        state.get_player_mut(right).alignment = Alignment::Good;

        let cr = state
            .get_player(empath_index)
            .night_one_ability(empath_index, &state)
            .unwrap();
        assert!(
            cr.description.contains("0 evil neighbors"),
            "got: {}",
            cr.description
        );
    }

    #[test]
    fn test_empath_two_evil_neighbors() {
        let roles = vec![
            RoleNames::Empath,
            RoleNames::Investigator,
            RoleNames::Chef,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let empath_index = find_role(&state, RoleNames::Empath);
        let left = state.left_player(empath_index);
        let right = state.right_player(empath_index);
        state.get_player_mut(left).alignment = Alignment::Evil;
        state.get_player_mut(right).alignment = Alignment::Evil;

        let cr = state
            .get_player(empath_index)
            .night_one_ability(empath_index, &state)
            .unwrap();
        assert!(
            cr.description.contains("2 evil neighbors"),
            "got: {}",
            cr.description
        );
    }

    #[test]
    fn test_empath_skips_dead_neighbor_to_count_next_alive_one() {
        let roles = vec![
            RoleNames::Empath,
            RoleNames::Investigator,
            RoleNames::Chef,
            RoleNames::Undertaker,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let empath_index = find_role(&state, RoleNames::Empath);

        let immediate_left = state.left_player(empath_index);
        let next_left = state.left_player(immediate_left);
        let right = state.right_player(empath_index);

        state.get_player_mut(right).alignment = Alignment::Good;
        state.get_player_mut(next_left).alignment = Alignment::Evil;
        state.get_player_mut(immediate_left).dead = true;

        let cr = state
            .get_player(empath_index)
            .night_ability(empath_index, &state)
            .unwrap();
        assert!(
            cr.description.contains("1 evil neighbors"),
            "the Empath should skip the dead neighbor and count the next alive one. got: {}",
            cr.description
        );
    }

    #[test]
    fn test_empath_dead_has_no_night_ability() {
        let roles = vec![
            RoleNames::Empath,
            RoleNames::Investigator,
            RoleNames::Chef,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let empath_index = find_role(&state, RoleNames::Empath);
        state.get_player_mut(empath_index).dead = true;

        assert!(
            state
                .get_player(empath_index)
                .night_ability(empath_index, &state)
                .is_none()
        );
    }

    // -- Fortuneteller --

    #[test]
    fn test_fortuneteller_thumbs_up_when_selecting_the_demon() {
        let roles = vec![
            RoleNames::Fortuneteller,
            RoleNames::Investigator,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let ft_index = find_role(&state, RoleNames::Fortuneteller);
        let imp_index = find_role(&state, RoleNames::Imp);
        let other_index = find_role(&state, RoleNames::Investigator);

        let cr = state
            .get_player(ft_index)
            .night_one_ability(ft_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(
                &mut state,
                ChangeArgs::PlayerIndices(vec![imp_index, other_index]),
            )
            .unwrap();

        let display_cr = state.change_request_queue.pop_front().unwrap();
        assert!(
            display_cr.description.contains("Thumbs Up"),
            "got: {}",
            display_cr.description
        );
    }

    #[test]
    fn test_fortuneteller_thumbs_down_when_no_demon_or_red_herring() {
        let roles = vec![
            RoleNames::Fortuneteller,
            RoleNames::Investigator,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let ft_index = find_role(&state, RoleNames::Fortuneteller);
        let inv_index = find_role(&state, RoleNames::Investigator);
        let saint_index = find_role(&state, RoleNames::Saint);

        let cr = state
            .get_player(ft_index)
            .night_one_ability(ft_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(
                &mut state,
                ChangeArgs::PlayerIndices(vec![inv_index, saint_index]),
            )
            .unwrap();

        let display_cr = state.change_request_queue.pop_front().unwrap();
        assert!(
            display_cr.description.contains("Thumbs Down"),
            "got: {}",
            display_cr.description
        );
    }

    #[test]
    fn test_fortuneteller_red_herring_registers_as_demon() {
        let roles = vec![
            RoleNames::Fortuneteller,
            RoleNames::Investigator,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let ft_index = find_role(&state, RoleNames::Fortuneteller);
        let inv_index = find_role(&state, RoleNames::Investigator);
        let saint_index = find_role(&state, RoleNames::Saint);

        let setup_cr = state
            .get_player(ft_index)
            .setup_ability(ft_index, &state)
            .unwrap();
        setup_cr
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![inv_index]))
            .unwrap();

        let cr = state
            .get_player(ft_index)
            .night_one_ability(ft_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(
                &mut state,
                ChangeArgs::PlayerIndices(vec![inv_index, saint_index]),
            )
            .unwrap();

        let display_cr = state.change_request_queue.pop_front().unwrap();
        assert!(
            display_cr.description.contains("Thumbs Up"),
            "the Red Herring should register as a Demon. got: {}",
            display_cr.description
        );
    }

    #[test]
    fn test_fortuneteller_dead_demon_still_registers() {
        let roles = vec![
            RoleNames::Fortuneteller,
            RoleNames::Investigator,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let ft_index = find_role(&state, RoleNames::Fortuneteller);
        let imp_index = find_role(&state, RoleNames::Imp);
        let inv_index = find_role(&state, RoleNames::Investigator);
        state.get_player_mut(imp_index).dead = true;

        let cr = state
            .get_player(ft_index)
            .night_ability(ft_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(
                &mut state,
                ChangeArgs::PlayerIndices(vec![imp_index, inv_index]),
            )
            .unwrap();

        let display_cr = state.change_request_queue.pop_front().unwrap();
        assert!(
            display_cr.description.contains("Thumbs Up"),
            "a dead Demon should still register. got: {}",
            display_cr.description
        );
    }

    #[test]
    fn test_fortuneteller_rejects_duplicate_player_selection() {
        let roles = vec![
            RoleNames::Fortuneteller,
            RoleNames::Investigator,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let ft_index = find_role(&state, RoleNames::Fortuneteller);
        let inv_index = find_role(&state, RoleNames::Investigator);

        let cr = state
            .get_player(ft_index)
            .night_one_ability(ft_index, &state)
            .unwrap();
        let result = cr.state_change_func.unwrap().call(
            &mut state,
            ChangeArgs::PlayerIndices(vec![inv_index, inv_index]),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fortuneteller_can_be_own_red_herring() {
        let roles = vec![
            RoleNames::Fortuneteller,
            RoleNames::Investigator,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let ft_index = find_role(&state, RoleNames::Fortuneteller);

        let setup_cr = state
            .get_player(ft_index)
            .setup_ability(ft_index, &state)
            .unwrap();
        let result = setup_cr
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![ft_index]));

        assert!(
            result.is_ok(),
            "the wiki explicitly allows the Fortune Teller to be their own Red Herring"
        );
    }

    // -- Undertaker --

    #[test]
    fn test_undertaker_learns_executed_role() {
        let roles = vec![RoleNames::Undertaker, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let undertaker_index = find_role(&state, RoleNames::Undertaker);
        let saint_index = find_role(&state, RoleNames::Saint);

        state.log.next_phase();
        state.log.log_event(Event::Execution(saint_index), None);
        state.log.next_phase();

        let cr = state
            .get_player(undertaker_index)
            .night_ability(undertaker_index, &state)
            .unwrap();
        assert!(cr.description.contains("Saint"), "got: {}", cr.description);
    }

    #[test]
    fn test_undertaker_learns_nothing_when_no_execution() {
        let roles = vec![RoleNames::Undertaker, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let undertaker_index = find_role(&state, RoleNames::Undertaker);

        state.log.next_phase();
        state.log.next_phase();

        assert!(
            state
                .get_player(undertaker_index)
                .night_ability(undertaker_index, &state)
                .is_none()
        );
    }

    #[test]
    fn test_undertaker_ignores_non_execution_death() {
        let roles = vec![RoleNames::Undertaker, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let undertaker_index = find_role(&state, RoleNames::Undertaker);
        let saint_index = find_role(&state, RoleNames::Saint);

        state.log.next_phase();
        state.log.log_event(Event::Death(saint_index), None);
        state.log.next_phase();

        assert!(
            state
                .get_player(undertaker_index)
                .night_ability(undertaker_index, &state)
                .is_none(),
            "the Undertaker should only learn about executions, not other deaths"
        );
    }

    #[test]
    fn test_undertaker_dead_has_no_ability() {
        let roles = vec![RoleNames::Undertaker, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let undertaker_index = find_role(&state, RoleNames::Undertaker);
        let saint_index = find_role(&state, RoleNames::Saint);
        state.get_player_mut(undertaker_index).dead = true;

        state.log.next_phase();
        state.log.log_event(Event::Execution(saint_index), None);
        state.log.next_phase();

        assert!(
            state
                .get_player(undertaker_index)
                .night_ability(undertaker_index, &state)
                .is_none()
        );
    }

    #[test]
    fn test_undertaker_shows_drunks_true_character_when_executed() {
        let roles = vec![RoleNames::Undertaker, RoleNames::Drunk, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let undertaker_index = find_role(&state, RoleNames::Undertaker);
        let drunk_index = find_role(&state, RoleNames::Drunk);

        // Assign the Drunk a believed role, as the real game would during setup
        let drunk_setup_cr = state
            .get_player(drunk_index)
            .setup_ability(drunk_index, &state)
            .unwrap();
        drunk_setup_cr
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Roles(vec![RoleNames::Chef]))
            .unwrap();

        state.log.next_phase();
        state.log.log_event(Event::Execution(drunk_index), None);
        state.log.next_phase();

        let cr = state
            .get_player(undertaker_index)
            .night_ability(undertaker_index, &state)
            .unwrap();
        assert!(
            cr.description.contains("Drunk"),
            "the Undertaker should be shown the Drunk's true character, not the believed Townsfolk role. got: {}",
            cr.description
        );
    }

    // -- Ravenkeeper --

    #[test]
    fn test_ravenkeeper_triggers_after_night_death_and_learns_role() {
        let roles = vec![RoleNames::Ravenkeeper, RoleNames::Chef, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let rk_index = find_role(&state, RoleNames::Ravenkeeper);
        let chef_index = find_role(&state, RoleNames::Chef);

        state.get_player_mut(rk_index).dead = true;
        state.log.log_event(Event::Death(rk_index), None);

        let cr = state
            .get_player(rk_index)
            .night_ability(rk_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![chef_index]))
            .unwrap();

        let display_cr = state.change_request_queue.pop_front().unwrap();
        assert!(
            display_cr.description.contains("Chef"),
            "got: {}",
            display_cr.description
        );
    }

    #[test]
    fn test_ravenkeeper_no_trigger_without_death_this_phase() {
        let roles = vec![RoleNames::Ravenkeeper, RoleNames::Chef, RoleNames::Imp];
        let state = setup_test_state(roles);
        let rk_index = find_role(&state, RoleNames::Ravenkeeper);

        assert!(
            state
                .get_player(rk_index)
                .night_ability(rk_index, &state)
                .is_none()
        );
    }

    #[test]
    fn test_ravenkeeper_death_in_earlier_phase_does_not_trigger() {
        let roles = vec![RoleNames::Ravenkeeper, RoleNames::Chef, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let rk_index = find_role(&state, RoleNames::Ravenkeeper);

        state.log.next_phase();
        state.log.log_event(Event::Death(rk_index), None);
        state.log.next_phase();

        assert!(
            state
                .get_player(rk_index)
                .night_ability(rk_index, &state)
                .is_none(),
            "a death logged in an earlier phase should not trigger the Ravenkeeper now"
        );
    }

    #[test]
    fn test_ravenkeeper_ability_only_usable_once() {
        let roles = vec![
            RoleNames::Ravenkeeper,
            RoleNames::Chef,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let rk_index = find_role(&state, RoleNames::Ravenkeeper);
        let chef_index = find_role(&state, RoleNames::Chef);

        state.get_player_mut(rk_index).dead = true;
        state.log.log_event(Event::Death(rk_index), None);

        let cr = state
            .get_player(rk_index)
            .night_ability(rk_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![chef_index]))
            .unwrap();
        state.change_request_queue.clear();

        // Simulate a second death event in the current phase and try to trigger again
        state.log.log_event(Event::Death(rk_index), None);
        let second_attempt = state.get_player(rk_index).night_ability(rk_index, &state);
        assert!(
            second_attempt.is_none(),
            "the Ravenkeeper's ability should only be usable once"
        );
    }

    // -- Virgin --

    #[test]
    fn test_virgin_executes_first_townsfolk_nominator() {
        let roles = vec![
            RoleNames::Virgin,
            RoleNames::Investigator,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let virgin_index = find_role(&state, RoleNames::Virgin);
        let nominator_index = find_role(&state, RoleNames::Investigator);

        state.nominate_player(nominator_index, virgin_index);

        let cr = state
            .change_request_queue
            .pop_front()
            .expect("Virgin should queue an execution check");
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Blank)
            .unwrap();

        assert!(
            state.get_player(nominator_index).dead,
            "a Townsfolk nominator should be executed immediately"
        );
    }

    #[test]
    fn test_virgin_non_townsfolk_nominator_is_not_executed() {
        let roles = vec![
            RoleNames::Virgin,
            RoleNames::Poisoner,
            RoleNames::Saint,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let virgin_index = find_role(&state, RoleNames::Virgin);
        let nominator_index = find_role(&state, RoleNames::Poisoner);

        state.nominate_player(nominator_index, virgin_index);

        let cr = state
            .change_request_queue
            .pop_front()
            .expect("Virgin should still queue the check");
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Blank)
            .unwrap();

        assert!(
            !state.get_player(nominator_index).dead,
            "only a Townsfolk nominator should be executed"
        );
    }

    #[test]
    fn test_virgin_only_first_ever_nomination_can_trigger() {
        let roles = vec![
            RoleNames::Virgin,
            RoleNames::Poisoner,
            RoleNames::Investigator,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let virgin_index = find_role(&state, RoleNames::Virgin);
        let minion_index = find_role(&state, RoleNames::Poisoner);
        let townsfolk_index = find_role(&state, RoleNames::Investigator);

        // First nomination, by a non-Townsfolk - consumes the ability without executing anyone
        state.nominate_player(minion_index, virgin_index);
        let cr = state
            .change_request_queue
            .pop_front()
            .expect("first nomination should queue the check");
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Blank)
            .unwrap();
        assert!(!state.get_player(minion_index).dead);

        // Second nomination, by a Townsfolk - should have no effect since the ability is used up
        state.nominate_player(townsfolk_index, virgin_index);
        assert!(
            state.change_request_queue.is_empty(),
            "the Virgin's ability should already be spent after the first nomination"
        );
        assert!(!state.get_player(townsfolk_index).dead);
    }

    #[test]
    fn test_virgin_poisoned_first_nomination_should_still_consume_ability() {
        let roles = vec![
            RoleNames::Virgin,
            RoleNames::Poisoner,
            RoleNames::Investigator,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let virgin_index = find_role(&state, RoleNames::Virgin);
        let poisoner_index = find_role(&state, RoleNames::Poisoner);
        let townsfolk_index = find_role(&state, RoleNames::Investigator);

        // Poison the Virgin, then nominate them - the wiki says the ability is consumed even
        // when it is blocked (e.g. by poisoning)
        state.add_status(
            StatusEffect::new(StatusType::Poisoned, poisoner_index, None),
            virgin_index,
        );
        state.nominate_player(poisoner_index, virgin_index);
        assert!(
            state.change_request_queue.is_empty(),
            "while poisoned no check should be queued"
        );

        // Un-poison and nominate again with a Townsfolk - per the wiki this second nomination
        // should do nothing, since the ability was already spent by the first (blocked) one
        state.get_player_mut(virgin_index).status_effects.clear();
        state.nominate_player(townsfolk_index, virgin_index);
        if let Some(cr) = state.change_request_queue.pop_front() {
            cr.state_change_func
                .unwrap()
                .call(&mut state, ChangeArgs::Blank)
                .unwrap();
        }

        assert!(
            !state.get_player(townsfolk_index).dead,
            "the Virgin's ability should have been consumed by the first (poisoned) nomination, not the second"
        );
    }

    // -- Soldier --

    #[test]
    fn test_soldier_survives_demon_kill() {
        let roles = vec![RoleNames::Soldier, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let soldier_index = find_role(&state, RoleNames::Soldier);
        let imp_index = find_role(&state, RoleNames::Imp);

        state.kill(imp_index, soldier_index);

        assert!(!state.get_player(soldier_index).dead);
    }

    #[test]
    fn test_soldier_can_still_be_executed() {
        let roles = vec![RoleNames::Soldier, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let soldier_index = find_role(&state, RoleNames::Soldier);

        state.execute_player(soldier_index);

        assert!(state.get_player(soldier_index).dead);
    }

    #[test]
    fn test_soldier_protection_persists_across_cleanup_phases() {
        let roles = vec![RoleNames::Soldier, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let soldier_index = find_role(&state, RoleNames::Soldier);
        let imp_index = find_role(&state, RoleNames::Imp);

        state.cleanup_statuses(CleanupPhase::Dawn);
        state.cleanup_statuses(CleanupPhase::Dusk);
        state.kill(imp_index, soldier_index);

        assert!(
            !state.get_player(soldier_index).dead,
            "the Soldier's protection should be permanent, not phase-limited"
        );
    }

    #[test]
    fn test_poisoned_soldier_dies_to_demon_kill() {
        // Poison negates the Soldier's ability, so a poisoned Soldier should NOT be immune
        // to a demon kill. Soldier's protection is implemented as an attempted_kill_listener
        // sourced from the Soldier, which `State::kill` already skips when its source is
        // poisoned -- so this is checked live at kill time, not just at grant time.
        let roles = vec![RoleNames::Poisoner, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let poisoner_index = find_role(&state, RoleNames::Poisoner);
        let soldier_index = find_role(&state, RoleNames::Soldier);
        let imp_index = find_role(&state, RoleNames::Imp);

        let poisoner_role = Roles::new(&RoleNames::Poisoner);
        let cr = poisoner_role
            .night_ability(poisoner_index, &state)
            .expect("Poisoner should have a night ability");
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![soldier_index]))
            .unwrap();

        state.kill(imp_index, soldier_index);

        assert!(
            state.get_player(soldier_index).dead,
            "a poisoned Soldier should not be immune to a demon kill"
        );
    }

    #[test]
    fn test_soldier_immune_again_after_poison_wears_off() {
        let roles = vec![RoleNames::Poisoner, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let poisoner_index = find_role(&state, RoleNames::Poisoner);
        let soldier_index = find_role(&state, RoleNames::Soldier);
        let imp_index = find_role(&state, RoleNames::Imp);

        let poisoner_role = Roles::new(&RoleNames::Poisoner);
        let cr = poisoner_role
            .night_ability(poisoner_index, &state)
            .expect("Poisoner should have a night ability");
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![soldier_index]))
            .unwrap();

        state.cleanup_statuses(CleanupPhase::Dusk);

        state.kill(imp_index, soldier_index);

        assert!(
            !state.get_player(soldier_index).dead,
            "the Soldier should be immune again once the Poisoner's poison has worn off"
        );
    }

    // -- Slayer --

    #[test]
    fn test_slayer_kills_the_demon() {
        let roles = vec![RoleNames::Slayer, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let slayer_index = find_role(&state, RoleNames::Slayer);
        let imp_index = find_role(&state, RoleNames::Imp);

        let cr = state
            .get_player(slayer_index)
            .day_ability(slayer_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![imp_index]))
            .unwrap();

        assert!(
            state.get_player(imp_index).dead,
            "the Demon should die when the Slayer correctly identifies them"
        );
        assert!(
            !state.get_player(slayer_index).has_day_ability(),
            "the once-per-game ability should now be spent"
        );
    }

    #[test]
    fn test_slayer_wrong_target_no_kill_but_ability_still_spent() {
        let roles = vec![RoleNames::Slayer, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let slayer_index = find_role(&state, RoleNames::Slayer);
        let saint_index = find_role(&state, RoleNames::Saint);

        let cr = state
            .get_player(slayer_index)
            .day_ability(slayer_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![saint_index]))
            .unwrap();

        assert!(!state.get_player(saint_index).dead);
        assert!(
            !state.get_player(slayer_index).has_day_ability(),
            "the ability is spent even when the guess is wrong"
        );
    }

    #[test]
    fn test_slayer_poisoned_consumes_ability_without_killing_demon() {
        let roles = vec![RoleNames::Slayer, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let slayer_index = find_role(&state, RoleNames::Slayer);
        let imp_index = find_role(&state, RoleNames::Imp);

        state.add_status(
            StatusEffect::new(StatusType::Poisoned, imp_index, None),
            slayer_index,
        );

        let cr = state
            .get_player(slayer_index)
            .day_ability(slayer_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![imp_index]))
            .unwrap();

        assert!(
            !state.get_player(imp_index).dead,
            "a poisoned Slayer's guess should have no real effect"
        );
        assert!(
            !state.get_player(slayer_index).has_day_ability(),
            "the once-per-game ability is still consumed even though poisoned"
        );
    }

    // -- Mayor --

    #[test]
    fn test_mayor_redirects_night_kill_to_another_player() {
        let roles = vec![RoleNames::Mayor, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let mayor_index = find_role(&state, RoleNames::Mayor);
        let imp_index = find_role(&state, RoleNames::Imp);
        let saint_index = find_role(&state, RoleNames::Saint);

        state.kill(imp_index, mayor_index);
        assert!(
            !state.get_player(mayor_index).dead,
            "the kill should be prevented by default and redirected"
        );

        let cr = state
            .change_request_queue
            .pop_front()
            .expect("Mayor should queue a redirect choice");
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![saint_index]))
            .unwrap();

        assert!(
            state.get_player(saint_index).dead,
            "the redirected target should die instead"
        );
        assert!(!state.get_player(mayor_index).dead);
    }

    #[test]
    fn test_mayor_redirect_onto_protected_player_results_in_no_death() {
        let roles = vec![RoleNames::Mayor, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let mayor_index = find_role(&state, RoleNames::Mayor);
        let imp_index = find_role(&state, RoleNames::Imp);
        let soldier_index = find_role(&state, RoleNames::Soldier);

        state.kill(imp_index, mayor_index);
        let cr = state.change_request_queue.pop_front().unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![soldier_index]))
            .unwrap();

        assert!(
            !state.get_player(soldier_index).dead,
            "redirecting onto an already-protected player should result in no death at all"
        );
        assert!(!state.get_player(mayor_index).dead);
    }

    #[test]
    fn test_poisoned_mayor_kill_not_redirected() {
        let roles = vec![RoleNames::Mayor, RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let mayor_index = find_role(&state, RoleNames::Mayor);
        let imp_index = find_role(&state, RoleNames::Imp);

        state.add_status(
            StatusEffect::new(StatusType::Poisoned, imp_index, None),
            mayor_index,
        );

        state.kill(imp_index, mayor_index);

        assert!(
            state.get_player(mayor_index).dead,
            "a poisoned Mayor should not redirect the kill, and should die normally"
        );
    }
}
