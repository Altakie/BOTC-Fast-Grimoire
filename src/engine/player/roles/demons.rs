use crate::ChangeRequest;
use crate::engine::change_request::{FilterFuncPtr, StateChangeFuncPtr, check_len};
use crate::engine::player::roles::Roles;
use crate::engine::state::Step;
use crate::engine::state::log::DayPhaseLog;
use std::fmt::Display;

use crate::engine::{
    change_request::{ChangeError, ChangeRequestBuilder, ChangeType},
    player::{Alignment, CharacterType, roles::Role},
    state::{PlayerIndex, State},
};

#[derive(Default, Debug, Clone)]
pub(crate) struct Imp {
    pub(crate) last_killed: Option<(Step, usize)>,
    pub(crate) last_swapped: Option<(Step, usize)>,
}

impl Role for Imp {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Demon
    }

    fn night_order(&self) -> Option<usize> {
        Some(34)
    }

    fn is_win_condition(&self) -> bool {
        true
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

        if let Some((step, prev_day_num)) = self.last_killed
            && prev_day_num == state.day_num
            && step == state.step
        {
            return None;
        }

        return ChangeRequest::new_builder(
            ChangeType::ChoosePlayers(1),
            "Ask the Imp to point to the player they would like to kill".into(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let target_players = args.extract_player_indicies()?;
            check_len(&target_players, 1)?;
            let target_player_index = target_players[0];
            state.kill(player_index, target_player_index);

            let kill_data = (state.step, state.day_num);
            if let Roles::Imp(imp_data) = &mut state.get_player_mut(player_index).role {
                imp_data.last_killed = Some(kill_data);
            }
            state.change_request_queue.push_back(
                ChangeRequest::new_builder(ChangeType::NoStoryteller, String::new())
                    .state_change_func(StateChangeFuncPtr::new(move |state, _| {
                        if let Roles::Imp(Imp {
                            last_swapped: Some((step, day_num)),
                            ..
                        }) = &state.get_player(player_index).role
                            && *day_num == state.day_num
                            && *step == state.step
                        {
                            return Ok(());
                        }

                        if target_player_index == player_index
                            && state.get_player(player_index).dead
                        {
                            state
                                .change_request_queue
                                .push_back(Imp::new_imp(player_index));
                        }

                        Ok(())
                    })),
            );

            Ok(())
        }))
        .into();
    }
}

impl Imp {
    fn new_imp(player_index: PlayerIndex) -> ChangeRequestBuilder {
        return ChangeRequest::new_builder(ChangeType::ChoosePlayers(1), "Choose a new Imp".into())
            .state_change_func(StateChangeFuncPtr::new(move |state, args| {
                let target_players = args.extract_player_indicies()?;
                check_len(&target_players, 1)?;

                let target_player_index = target_players[0];
                let target_player = state.get_player(target_player_index);
                if target_player.role.get_true_character_type() != CharacterType::Minion {
                    return Err(ChangeError::InvalidSelectedPlayer {
                        reason: "Cannot select a non-minion to become the new imp".into(),
                    });
                }
                let target_player_index = target_players[0];
                let day_num = state.day_num;
                let mut new_role = state.get_player(player_index).role.clone();
                if let Roles::Imp(imp_data) = &mut new_role {
                    imp_data.last_swapped = Some((state.step, state.day_num));
                }
                let target_player = state.get_player_mut(target_player_index);

                target_player.role = new_role;
                Ok(())
            }))
            .filter_func(FilterFuncPtr::new(move |_, player| {
                player.role.get_true_character_type() == CharacterType::Minion
            }));
    }
}

impl Display for Imp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Imp")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::change_request::ChangeArgs;
    use crate::engine::player::roles::RoleNames;
    use crate::engine::player::roles::test_utils::{find_role, setup_test_state};

    #[test]
    fn test_imp_night_kill() {
        let roles = vec![
            RoleNames::Imp,
            RoleNames::Poisoner,
            RoleNames::Washerwoman,
        ];
        let mut state = setup_test_state(roles);

        let imp_index = find_role(&state, RoleNames::Imp);
        let target_index = find_role(&state, RoleNames::Washerwoman);

        // Imp acts "each night*" -- i.e. not night one.
        state.step = Step::Night;

        let cr = state
            .get_player(imp_index)
            .night_ability(imp_index, &state)
            .expect("Imp should have a night ability");

        let args = ChangeArgs::PlayerIndices(vec![target_index]);
        cr.state_change_func.unwrap().call(&mut state, args).unwrap();

        assert!(
            state.get_player(target_index).dead,
            "Imp's chosen target should die"
        );
    }

    #[test]
    fn test_imp_has_no_night_one_ability() {
        // Wiki: "Each night*, choose a player: they die." The `*` means the Imp does not
        // act on night one.
        let roles = vec![
            RoleNames::Imp,
            RoleNames::Poisoner,
            RoleNames::Washerwoman,
        ];
        let state = setup_test_state(roles);

        let imp_index = find_role(&state, RoleNames::Imp);

        assert!(
            state
                .get_player(imp_index)
                .night_one_ability(imp_index, &state)
                .is_none(),
            "Imp should not have a night one ability"
        );
    }

    #[test]
    fn test_imp_death_ends_game_for_good() {
        // Wiki rule 3: "If the Imp dies (by any means), good wins."
        let roles = vec![
            RoleNames::Imp,
            RoleNames::Poisoner,
            RoleNames::Washerwoman,
        ];
        let mut state = setup_test_state(roles);

        let imp_index = find_role(&state, RoleNames::Imp);

        assert!(
            !state.game_over(),
            "Game should not be over while the Imp is alive"
        );

        // Imp dies by a means other than its own kill ability (e.g. execution).
        state.execute_player(imp_index);

        assert!(state.get_player(imp_index).dead);
        assert!(
            state.game_over(),
            "Good should win once the (non-star-passed) Imp is dead"
        );
    }

    #[test]
    fn test_imp_star_pass_creates_new_imp() {
        // Wiki rule 4: "If you kill yourself this way, a Minion becomes the Imp."
        let roles = vec![
            RoleNames::Imp,
            RoleNames::Poisoner,
            RoleNames::Washerwoman,
        ];
        let mut state = setup_test_state(roles);

        let imp_index = find_role(&state, RoleNames::Imp);
        let minion_index = find_role(&state, RoleNames::Poisoner);

        state.step = Step::Night;

        let cr = state
            .get_player(imp_index)
            .night_ability(imp_index, &state)
            .expect("Imp should have a night ability");

        // Imp targets itself.
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![imp_index]))
            .unwrap();

        assert!(
            state.get_player(imp_index).dead,
            "Imp should die from targeting itself"
        );

        // The kill's state_change_func queues a NoStoryteller follow-up that detects the
        // star-pass and, if it occurred, queues a "choose a new Imp" request.
        let follow_up = state
            .change_request_queue
            .pop_front()
            .expect("expected a queued follow-up change request after Imp self-kill");
        follow_up
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Blank)
            .unwrap();

        let new_imp_request = state
            .change_request_queue
            .pop_front()
            .expect("expected a queued 'choose new Imp' request after star-pass");

        new_imp_request
            .state_change_func
            .unwrap()
            .call(
                &mut state,
                ChangeArgs::PlayerIndices(vec![minion_index]),
            )
            .unwrap();

        assert_eq!(
            state.get_player(minion_index).role.to_role_name(),
            RoleNames::Imp,
            "The alive Minion should become the new Imp after a star-pass"
        );
        assert_eq!(
            state.get_player(minion_index).role.get_true_character_type(),
            CharacterType::Demon
        );
    }

    #[test]
    fn test_imp_star_pass_game_should_not_be_over() {
        // Wiki rule 4: star-passing keeps the game going via a new Imp instead of ending
        // it. This checks the win-condition bookkeeping actually reflects that instead of
        // just the role transfer.
        let roles = vec![
            RoleNames::Imp,
            RoleNames::Poisoner,
            RoleNames::Washerwoman,
        ];
        let mut state = setup_test_state(roles);

        let imp_index = find_role(&state, RoleNames::Imp);
        let minion_index = find_role(&state, RoleNames::Poisoner);

        state.step = Step::Night;

        let cr = state
            .get_player(imp_index)
            .night_ability(imp_index, &state)
            .expect("Imp should have a night ability");
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![imp_index]))
            .unwrap();

        let follow_up = state.change_request_queue.pop_front().unwrap();
        follow_up
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Blank)
            .unwrap();

        let new_imp_request = state.change_request_queue.pop_front().unwrap();
        new_imp_request
            .state_change_func
            .unwrap()
            .call(
                &mut state,
                ChangeArgs::PlayerIndices(vec![minion_index]),
            )
            .unwrap();

        assert!(
            !state.game_over(),
            "Game should not be over after a star-pass, since a new Imp is alive"
        );
    }

    #[test]
    fn test_imp_cannot_act_twice_in_same_night() {
        // There's no explicit wiki rule against this (the Storyteller simply wouldn't wake
        // the Imp twice), but the engine tracks `last_killed` to enforce a once-per-night
        // constraint. This test documents/pins that behavior as currently implemented.
        let roles = vec![
            RoleNames::Imp,
            RoleNames::Poisoner,
            RoleNames::Washerwoman,
            RoleNames::Chef,
        ];
        let mut state = setup_test_state(roles);

        let imp_index = find_role(&state, RoleNames::Imp);
        let target_index = find_role(&state, RoleNames::Washerwoman);

        state.step = Step::Night;

        let cr = state
            .get_player(imp_index)
            .night_ability(imp_index, &state)
            .expect("Imp should have a night ability the first time");
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![target_index]))
            .unwrap();

        assert!(state.get_player(target_index).dead);

        // Same night/step -- the Imp should not be able to act again.
        let second_ability = state
            .get_player(imp_index)
            .night_ability(imp_index, &state);
        assert!(
            second_ability.is_none(),
            "Imp should not be able to act twice in the same night/step"
        );
    }
}
