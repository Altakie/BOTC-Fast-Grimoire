use std::fmt::Display;

use leptos::leptos_dom::logging::console_log;

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
        log::{AttemptedKill, Event, Nomination},
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

        let target_player = state.get_player_mut(target_player_indices[0]);
        target_player.add_status(right_status());

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
        let target_player = state.get_player_mut(target_player_indices[0]);
        target_player.add_status(wrong_status());

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
            let target_player = state.get_player_mut(target_player_index);
            let status = StatusEffect::new(StatusType::FortuneTellerRedHerring, player_index, None);
            target_player.add_status(status);

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

            let target_player = state.get_player_mut(target_player_indices[0]);
            let status = StatusEffect::new(
                StatusType::DemonProtected,
                player_index,
                CleanupPhase::Dawn.into(),
            );
            target_player.add_status(status);

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
                console_log("I was called");
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
    use crate::{engine::player::roles::RoleNames, scripts::trouble_brewing};

    use super::*;

    fn setup_test_state(roles: Vec<RoleNames>) -> State {
        let player_names = roles
            .iter()
            .map(|role| role.convert().to_string())
            .collect();
        State::new(roles, player_names, trouble_brewing()).unwrap()
    }

    #[test]
    fn test_undertaker_ability() {
        let roles = vec![
            RoleNames::Undertaker,
            RoleNames::Virgin,
            RoleNames::Soldier,
            RoleNames::Spy,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);
        let undertaker_role = Roles::new(&RoleNames::Undertaker);
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
