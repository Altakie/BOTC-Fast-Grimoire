use std::fmt::Display;

use crate::engine::change_request::{ChangeArgs, ChangeError, ChangeRequest, StateChangeFuncPtr};
use crate::engine::player::roles::Roles;
use crate::engine::state::status_effects::CleanupPhase;
use crate::engine::state::{EventListener, log};
use crate::engine::{
    change_request::{ChangeRequestBuilder, ChangeType, check_len},
    player::{Alignment, CharacterType, roles::Role},
    state::{
        PlayerIndex, State,
        status_effects::{StatusEffect, StatusType},
    },
};

#[derive(Default, Debug, Clone)]
pub(crate) struct Butler();

impl Butler {
    fn ability(&self, player_index: PlayerIndex) -> Option<ChangeRequestBuilder> {
        // Clean up the old butler master status effect (if there is one), prompt for another
        // player, and give them the butler master status effect
        ChangeRequest::new_builder(
            ChangeType::ChoosePlayers(1),
            "Prompt the butler to pick a player to be their master".to_string(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let target_players = args.extract_player_indicies()?;
            check_len(&target_players, 1)?;

            // Check that the butler is not picking themselves
            if target_players[0] == player_index {
                return Err(ChangeError::InvalidSelectedPlayer {
                    reason: "Butler Should not be able to pick themselves".into(),
                });
            }

            let status = StatusEffect::new(
                StatusType::ButlerMaster,
                player_index,
                CleanupPhase::Dusk.into(),
            );
            state.add_status(status, target_players[0]);
            Ok(())
        }))
        .into()
    }
}

impl Role for Butler {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Outsider
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(51)
    }

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequestBuilder> {
        self.ability(player_index)
    }

    fn night_order(&self) -> Option<usize> {
        Some(83)
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
        self.ability(player_index)
    }
}

impl Display for Butler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Butler")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Drunk {
    // WARN: Why can we use boxes here. Isn't this not thread safe?
    role: Option<Box<Roles>>,
}

impl Role for Drunk {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Outsider
    }

    // TODO: Should change based on what role is assigned
    fn setup_order(&self) -> Option<usize> {
        Some(1)
    }

    fn setup_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        // If the drunk has a role assigned, call its setup ability instead
        if let Some(role) = &self.role {
            let res = role.setup_ability(player_index, state);
            return res.map(|cr| cr.clear_state_change_func());
        };

        // Otherwise assign a role to the drunk
        ChangeRequest::new_builder(
            ChangeType::ChooseRoles(1),
            "Select a not in play Townfolk role".into(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let roles = args.extract_roles()?;

            check_len(&roles, 1)?;

            if roles[0].get_type() != CharacterType::Townsfolk {
                return Err(ChangeError::InvalidSelectedRole {
                    reason: "Drunk has to be a townsfolk role".into(),
                });
            }

            let drunk = state.get_player_mut(player_index);
            drunk.role = Roles::Drunk(Drunk {
                role: Some(Box::new(roles[0].convert())),
            });

            let drunk = state.get_player(player_index);
            if let Some(ability) = drunk.setup_ability(player_index, state) {
                state.change_request_queue.push_back(ability);
            }

            Ok(())
        }))
        .into()
    }

    fn night_one_order(&self) -> Option<usize> {
        let role = self.role.clone()?;
        role.night_one_order()
    }

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let role = self.role.clone()?;

        let res = role.night_one_ability(player_index, state);
        return match res {
            Some(mut cr) => {
                cr.state_change_func = None;
                cr.description = format!("(*Drunk*) {}", cr.description);
                Some(cr)
            }
            None => None,
        };
    }

    fn night_order(&self) -> Option<usize> {
        let role = self.role.clone()?;
        role.night_order()
    }

    fn night_ability(
        &self,
        player_index: PlayerIndex,
        state: &State,
    ) -> Option<ChangeRequestBuilder> {
        let role = self.role.clone()?;

        let res = role.night_ability(player_index, state);
        return match res {
            Some(cr) => Some(
                cr.clear_state_change_func()
                    .change_description(|desc| format!("(*Drunk*) {}", desc)),
            ),
            None => None,
        };
    }
}

impl Display for Drunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role = &self.role;
        match role {
            Some(role) => write!(f, "The Drunk {}", role),
            None => f.write_str("The Drunk"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Recluse();

impl Role for Recluse {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Outsider
    }

    fn get_alignment(&self) -> Alignment {
        Alignment::Any
    }
}

impl Display for Recluse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Recluse")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Saint();
// TODO:
// Saint is technically a win condition, figure out how winning the game actually comes about

impl Role for Saint {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Good
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Outsider
    }

    fn initialize(&self, player_index: PlayerIndex, state: &mut State) {
        state.execution_listeners.push(EventListener::new(
            player_index,
            |ev_state, state, event: log::Execution| {
                if event.0 == ev_state.source_player_index {
                    state.winner = Some(
                        match state.get_player(ev_state.source_player_index).alignment {
                            Alignment::Good => Alignment::Evil,
                            Alignment::Evil => Alignment::Good,
                            Alignment::Any => {
                                unreachable!("Saint's true alignment should never be Any");
                            }
                        },
                    )
                }
                state
            },
        ));
    }
}

impl Display for Saint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Saint")
    }
}

#[cfg(test)]
mod test {
    use crate::engine::player::roles::RoleNames;
    use crate::engine::player::roles::test_utils::{find_role, setup_test_state};

    use super::*;

    // ---------------------------------------------------------------
    // Butler
    // ---------------------------------------------------------------

    #[test]
    fn test_butler_chooses_master() {
        let roles = vec![RoleNames::Butler, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let butler_index = find_role(&state, RoleNames::Butler);
        let target_index = find_role(&state, RoleNames::Soldier);

        let butler_role = Roles::new(&RoleNames::Butler);
        let cr = butler_role.night_one_ability(butler_index, &state).unwrap();

        let args = ChangeArgs::PlayerIndices(vec![target_index]);
        cr.state_change_func
            .unwrap()
            .call(&mut state, args)
            .unwrap();

        assert!(
            state
                .get_player(target_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::ButlerMaster
                    && s.source_player_index == butler_index),
            "Butler's chosen master should be marked with a ButlerMaster status sourced from the Butler"
        );
    }

    #[test]
    fn test_butler_cannot_choose_self_as_master() {
        let roles = vec![RoleNames::Butler, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let butler_index = find_role(&state, RoleNames::Butler);

        let butler_role = Roles::new(&RoleNames::Butler);
        let cr = butler_role.night_one_ability(butler_index, &state).unwrap();

        let args = ChangeArgs::PlayerIndices(vec![butler_index]);
        let result = cr.state_change_func.unwrap().call(&mut state, args);

        assert!(
            matches!(result, Err(ChangeError::InvalidSelectedPlayer { .. })),
            "The Butler's ability text explicitly says 'not yourself'"
        );
    }

    #[test]
    fn test_butler_can_choose_a_dead_player_as_master() {
        // Rule 6: "A dead player (with a vote token) can still be chosen as Master, and the
        // same voting rules apply."
        let roles = vec![RoleNames::Butler, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let butler_index = find_role(&state, RoleNames::Butler);
        let target_index = find_role(&state, RoleNames::Soldier);

        // Directly mark the target dead rather than going through State::kill, since the
        // Soldier is immune to the Demon's kill and that immunity isn't what this test is about.
        state.get_player_mut(target_index).dead = true;
        assert!(state.get_player(target_index).dead);

        let butler_role = Roles::new(&RoleNames::Butler);
        let cr = butler_role.night_one_ability(butler_index, &state).unwrap();

        let args = ChangeArgs::PlayerIndices(vec![target_index]);
        cr.state_change_func
            .unwrap()
            .call(&mut state, args)
            .unwrap();

        assert!(
            state
                .get_player(target_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::ButlerMaster),
            "a dead player should still be a legal Master choice"
        );
    }

    #[test]
    fn test_butler_master_status_is_cleared_at_dusk() {
        // Rule 1: each night the Butler chooses a (possibly new) Master, implying the previous
        // night's Master marking doesn't persist forever. The ability applies the ButlerMaster
        // status with a Dusk cleanup phase, so it should be cleared by the dusk status cleanup.
        let roles = vec![RoleNames::Butler, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let butler_index = find_role(&state, RoleNames::Butler);
        let target_index = find_role(&state, RoleNames::Soldier);

        let butler_role = Roles::new(&RoleNames::Butler);
        let cr = butler_role.night_one_ability(butler_index, &state).unwrap();
        let args = ChangeArgs::PlayerIndices(vec![target_index]);
        cr.state_change_func
            .unwrap()
            .call(&mut state, args)
            .unwrap();

        assert!(
            state
                .get_player(target_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::ButlerMaster)
        );

        state.cleanup_statuses(CleanupPhase::Dusk);

        assert!(
            !state
                .get_player(target_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::ButlerMaster),
            "the Master marking should be cleared at dusk so the Butler can pick a new Master next night"
        );
    }

    #[test]
    fn test_dead_butler_has_no_night_ability() {
        let roles = vec![RoleNames::Butler, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let butler_index = find_role(&state, RoleNames::Butler);
        state.get_player_mut(butler_index).dead = true;

        let butler_role = Roles::new(&RoleNames::Butler);
        assert!(butler_role.night_ability(butler_index, &state).is_none());
    }

    // ---------------------------------------------------------------
    // Drunk
    // ---------------------------------------------------------------

    #[test]
    fn test_drunk_with_no_believed_role_has_no_night_abilities() {
        // A freshly-created Drunk hasn't been assigned a believed Townsfolk role yet, so it
        // should not offer any night one / night ability.
        let roles = vec![RoleNames::Drunk, RoleNames::Imp];
        let state = setup_test_state(roles);

        let drunk_index = find_role(&state, RoleNames::Drunk);
        let drunk_role = Roles::new(&RoleNames::Drunk);

        assert!(drunk_role.night_one_ability(drunk_index, &state).is_none());
        assert!(drunk_role.night_ability(drunk_index, &state).is_none());
    }

    #[test]
    fn test_drunk_setup_rejects_non_townsfolk_role() {
        let roles = vec![RoleNames::Drunk, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let drunk_role = Roles::new(&RoleNames::Drunk);
        let drunk_index = find_role(&state, RoleNames::Drunk);
        let cr = drunk_role.setup_ability(drunk_index, &state).unwrap();

        let args = ChangeArgs::Roles(vec![RoleNames::Imp]);
        let result = cr.state_change_func.unwrap().call(&mut state, args);

        assert!(
            matches!(result, Err(ChangeError::InvalidSelectedRole { .. })),
            "the Drunk must be assigned a Townsfolk character, per rule 1"
        );
    }

    #[test]
    fn test_drunk_setup_assigns_believed_townsfolk_role_and_strips_its_real_ability() {
        // Rule 1/3: during setup the Drunk is assigned a believed Townsfolk role, but "the Drunk
        // has no actual ability -- their believed Townsfolk ability never functions".
        let roles = vec![RoleNames::Drunk, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let drunk_index = find_role(&state, RoleNames::Drunk);
        let drunk_role = Roles::new(&RoleNames::Drunk);
        let cr = drunk_role.setup_ability(drunk_index, &state).unwrap();

        // Washerwoman's setup ability has a real (functional) state_change_func that applies
        // WasherwomanTownsfolk/WasherwomanWrong statuses.
        let args = ChangeArgs::Roles(vec![RoleNames::Washerwoman]);
        cr.state_change_func
            .unwrap()
            .call(&mut state, args)
            .unwrap();

        let assigned_role = match &state.get_player(drunk_index).role {
            Roles::Drunk(drunk) => drunk.role.clone(),
            other => panic!("expected player to still be RoleNames::Drunk, got {other}"),
        };
        assert!(
            matches!(assigned_role.as_deref(), Some(Roles::Washerwoman(_))),
            "the Drunk should now believe itself to be the selected Townsfolk role"
        );

        // The believed role's real setup ability gets queued up (so the Storyteller still "goes
        // through the motions"), but it must not be able to produce a real effect.
        let queued = state
            .change_request_queue
            .pop_front()
            .expect("the believed role's setup ability should still be queued for show");
        assert!(
            queued.state_change_func.is_none(),
            "the Drunk's believed ability must never actually function"
        );
    }

    #[test]
    fn test_drunk_night_ability_never_produces_a_real_effect() {
        // Directly assign a believed role that has a real, functioning night_ability (Monk) and
        // confirm the Drunk-wrapped version strips out the functional part.
        let roles = vec![RoleNames::Drunk, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let drunk_index = find_role(&state, RoleNames::Drunk);

        state.get_player_mut(drunk_index).role = Roles::Drunk(Drunk {
            role: Some(Box::new(RoleNames::Monk.convert())),
        });

        let drunk_role = state.get_player(drunk_index).role.clone();
        let cr = drunk_role
            .night_ability(drunk_index, &state)
            .expect("Drunk should relay a change request for its believed role's night ability");

        assert!(
            cr.state_change_func.is_none(),
            "the Drunk's substituted ability should never produce a real effect, even though the \
             believed role (Monk) normally has a functional night ability"
        );
    }

    // ---------------------------------------------------------------
    // Recluse
    // ---------------------------------------------------------------

    #[test]
    fn test_recluse_true_nature_is_good_outsider() {
        let recluse_role = Roles::new(&RoleNames::Recluse);
        assert_eq!(recluse_role.get_default_alignment(), Alignment::Good);
        assert_eq!(
            recluse_role.get_true_character_type(),
            CharacterType::Outsider
        );
    }

    #[test]
    fn test_recluse_may_register_as_evil_alignment() {
        // Rule 2: "Whenever the Recluse's alignment is detected, the Storyteller chooses whether
        // the Recluse registers as good or evil." The engine should be able to represent the
        // Recluse registering as Evil to a detection ability.
        let recluse_role = Roles::new(&RoleNames::Recluse);
        assert_eq!(
            recluse_role.get_alignment(),
            Alignment::Evil,
            "the Recluse's disguised alignment should be able to resolve to Evil"
        );
    }

    #[test]
    fn test_recluse_may_register_as_minion_or_demon_character_type() {
        // Rule 3: "Whenever the Recluse is targeted by an ability that affects specific Minions
        // or Demons, the Storyteller chooses whether the Recluse registers as that specific
        // Minion or Demon."
        let recluse_role = Roles::new(&RoleNames::Recluse);
        assert_eq!(
            recluse_role.get_character_type(),
            CharacterType::Demon,
            "the Recluse's disguised character type should be able to resolve to Minion/Demon"
        );
    }

    #[test]
    fn test_recluse_disguise_grants_no_real_ability() {
        // Rule 5: "A Recluse that registers as a particular Minion or Demon does not have this
        // character's ability."
        let roles = vec![RoleNames::Recluse, RoleNames::Imp];
        let state = setup_test_state(roles);
        let recluse_index = find_role(&state, RoleNames::Recluse);

        let recluse_role = Roles::new(&RoleNames::Recluse);
        assert!(
            recluse_role
                .night_one_ability(recluse_index, &state)
                .is_none()
        );
        assert!(recluse_role.night_ability(recluse_index, &state).is_none());
    }

    #[test]
    fn test_recluse_disguise_still_functions_after_death() {
        // Rule 6: "This ability continues to function even after the Recluse has died."
        let roles = vec![RoleNames::Recluse, RoleNames::Imp];
        let mut state = setup_test_state(roles);
        let recluse_index = find_role(&state, RoleNames::Recluse);

        state.get_player_mut(recluse_index).dead = true;

        let recluse_role = &state.get_player(recluse_index).role;
        // Whatever the disguised alignment/character-type resolve to, they should be unaffected
        // by the Recluse being dead.
        assert_eq!(
            recluse_role.get_alignment(),
            Roles::new(&RoleNames::Recluse).get_alignment()
        );
        assert_eq!(
            recluse_role.get_character_type(),
            Roles::new(&RoleNames::Recluse).get_character_type()
        );
    }

    // ---------------------------------------------------------------
    // Saint
    // ---------------------------------------------------------------

    #[test]
    fn test_saint_true_nature_is_good_outsider() {
        let saint_role = Roles::new(&RoleNames::Saint);
        assert_eq!(saint_role.get_default_alignment(), Alignment::Good);
        assert_eq!(
            saint_role.get_true_character_type(),
            CharacterType::Outsider
        );
    }

    #[test]
    fn test_saint_executed_ends_the_game_with_evil_winning() {
        // Rule 1: "If the Saint dies by execution, the game ends immediately with evil winning
        // (good loses)."
        let roles = vec![RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let saint_index = find_role(&state, RoleNames::Saint);
        state.execute_player(saint_index);

        assert!(
            state.get_player(saint_index).dead,
            "the Saint should be dead after execution"
        );
        assert!(
            state
                .game_over()
                .is_some_and(|winner| winner == Alignment::Evil),
            "executing the Saint should immediately end the game (evil wins)"
        );
    }

    #[test]
    fn test_saint_death_by_demon_kill_does_not_end_the_game() {
        // Rule 2: "If the Saint dies by any means other than execution (e.g. the Demon's
        // night-kill), play continues as normal."
        let roles = vec![RoleNames::Saint, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let saint_index = find_role(&state, RoleNames::Saint);
        let imp_index = find_role(&state, RoleNames::Imp);

        state.kill(imp_index, saint_index);

        assert!(state.get_player(saint_index).dead);
        assert!(
            state.game_over().is_none(),
            "the Saint dying to a non-execution kill should not end the game"
        );
    }
}
