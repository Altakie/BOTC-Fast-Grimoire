#![allow(unused_variables)]
use std::fmt::Display;

use crate::{
    engine::{
        change_request::{
            ChangeError, ChangeRequest, ChangeRequestBuilder, ChangeType, StateChangeFuncPtr,
            check_len,
        },
        player::{
            Alignment, CharacterType,
            roles::{Role, RoleNames, Roles, demons::Imp},
        },
        state::{
            EventListener, PlayerIndex, State, log,
            status_effects::{CleanupPhase, StatusEffect, StatusType},
        },
    },
    initialization::CharacterTypeCounts,
};

#[derive(Default, Debug, Clone)]
pub(crate) struct Spy();
impl Spy {
    fn ability(&self) -> Option<ChangeRequestBuilder> {
        ChangeRequest::new_builder(ChangeType::Display, "Show the Spy the grimoire".into()).into()
    }
}

impl Role for Spy {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_alignment(&self) -> Alignment {
        Alignment::Any
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn get_character_type(&self) -> CharacterType {
        CharacterType::Any
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(65)
    }

    fn night_one_ability(
        &self,
        _player_index: PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequestBuilder> {
        self.ability()
    }

    fn night_order(&self) -> Option<usize> {
        Some(84)
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
        self.ability()
    }
}

impl Display for Spy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Spy")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Baron();

impl Role for Baron {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn initialization_effect(&self) -> Option<crate::initialization::CharacterTypeCounts> {
        Some(CharacterTypeCounts {
            townsfolk: -2,
            outsiders: 2,
            minions: 0,
            demons: 0,
        })
    }
}

impl Display for Baron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Baron")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Poisoner();

impl Poisoner {
    fn ability(&self, player_index: PlayerIndex) -> Option<ChangeRequestBuilder> {
        // Clean up the old poisoned effect, prompt for another
        // player, and give them the poisoned effect

        ChangeRequest::new_builder(
            ChangeType::ChoosePlayers(1),
            "Prompt the poisoner to pick a player to poison".to_string(),
        )
        .state_change_func(StateChangeFuncPtr::new(move |state, args| {
            let target_players = args.extract_player_indicies()?;
            check_len(&target_players, 1)?;

            let status = StatusEffect::new(
                StatusType::Poisoned,
                player_index,
                CleanupPhase::Dusk.into(),
            );
            state.add_status(status, target_players[0]);

            Ok(())
        }))
        .into()
    }
}

impl Role for Poisoner {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn night_one_order(&self) -> Option<usize> {
        Some(26)
    }

    fn night_one_ability(
        &self,
        player_index: PlayerIndex,
        _state: &State,
    ) -> Option<ChangeRequestBuilder> {
        self.ability(player_index)
    }

    fn night_order(&self) -> Option<usize> {
        Some(12)
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

impl Display for Poisoner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Poisoner")
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct ScarletWoman();

impl Role for ScarletWoman {
    fn get_default_alignment(&self) -> Alignment {
        Alignment::Evil
    }

    fn get_true_character_type(&self) -> CharacterType {
        CharacterType::Minion
    }

    fn initialize(&self, player_index: PlayerIndex, state: &mut State) {
        let scarlet_listener = EventListener::new(
            player_index,
            |event_listener_state, state, death_event: log::Death| {
                let dead_player = state.get_player(death_event.player_index);
                if dead_player.role.get_true_character_type() != CharacterType::Demon
                    || state
                        .get_players()
                        .iter()
                        .filter(|player| !player.dead)
                        .count()
                        < 4
                {
                    return state;
                }

                let source_player_index = event_listener_state.source_player_index;
                // state.set_win_condition(state.get_player(source_player_index));

                state.change_request_queue.push_back(
                    ChangeRequest::new_builder(ChangeType::NoStoryteller, String::new())
                        .state_change_func(StateChangeFuncPtr::new(move |state, _| {
                            let swap_data = (state.step, state.day_num);
                            let dead_player = &mut state.get_player_mut(death_event.player_index);
                            if let Roles::Imp(imp_data) = &mut dead_player.role {
                                imp_data.last_swapped = Some(swap_data);
                            }

                            let dead_role = dead_player.role.clone();
                            state.cleanup_event_listeners(source_player_index);

                            let scarlet_player = state.get_player_mut(source_player_index);
                            scarlet_player.role = dead_role;

                            Ok(())
                        })),
                );
                let dead_player_string =
                    state.get_player(death_event.player_index).role.to_string();
                state
                    .change_request_queue
                    .push_back(ChangeRequest::new_builder(
                        ChangeType::Display,
                        format!("The Scarletwoman becomes the {}", dead_player_string),
                    ));

                state
            },
        );

        state.death_listeners.push(scarlet_listener);
    }
}

impl Display for ScarletWoman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Scarlet Woman")
    }
}

#[cfg(test)]
mod test {
    use crate::engine::change_request::ChangeArgs;
    use crate::engine::player::roles::RoleNames;
    use crate::engine::player::roles::test_utils::{find_role, setup_test_state};

    use super::*;

    // -------------------- Poisoner --------------------

    #[test]
    fn test_poisoner_applies_poisoned_status() {
        let roles = vec![
            RoleNames::Poisoner,
            RoleNames::Soldier,
            RoleNames::Monk,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let poisoner_index = find_role(&state, RoleNames::Poisoner);
        let target_index = find_role(&state, RoleNames::Soldier);

        let poisoner_role = Roles::new(&RoleNames::Poisoner);
        let cr = poisoner_role.night_ability(poisoner_index, &state).unwrap();

        let args = ChangeArgs::PlayerIndices(vec![target_index]);
        cr.state_change_func.unwrap().call(&mut state, args).unwrap();

        let statuses = state.get_player(target_index).get_statuses();
        let poison_status = statuses
            .iter()
            .find(|s| s.status_type == StatusType::Poisoned)
            .expect("Target should be poisoned");
        assert_eq!(poison_status.source_player_index, poisoner_index);
    }

    #[test]
    fn test_poisoner_suppresses_monk_protection() {
        // Deliberately NOT using Soldier as the protected target: Soldier's own ability
        // applies a permanent DemonProtected status in `initialize()` (townsfolk.rs), which
        // would make the "no protection applied" assertion below pass regardless of Monk.
        let roles = vec![
            RoleNames::Poisoner,
            RoleNames::Monk,
            RoleNames::Investigator,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let poisoner_index = find_role(&state, RoleNames::Poisoner);
        let monk_index = find_role(&state, RoleNames::Monk);
        let imp_index = find_role(&state, RoleNames::Imp);
        let target_index = find_role(&state, RoleNames::Investigator);

        // Poisoner poisons the Monk
        let poisoner_role = Roles::new(&RoleNames::Poisoner);
        let cr = poisoner_role.night_ability(poisoner_index, &state).unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![monk_index]))
            .unwrap();

        assert!(
            state
                .get_player(monk_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::Poisoned)
        );

        // Poisoned Monk attempts to protect the Investigator. Go through the Player wrapper
        // (not the raw Roles::night_ability), since poison suppression is applied centrally
        // there via `drunkify` -- calling the raw role method bypasses it entirely.
        let cr = state
            .get_player(monk_index)
            .night_ability(monk_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![target_index]))
            .unwrap();

        // Per wiki rule 2, a poisoned player's ability does not function, so the
        // protection should not actually be applied.
        assert!(
            !state
                .get_player(target_index)
                .get_statuses()
                .iter()
                .any(|s| s.status_type == StatusType::DemonProtected),
            "Poisoned Monk's protection should not apply"
        );

        state.kill(imp_index, target_index);
        assert!(
            state.get_player(target_index).dead,
            "the target should die because the poisoned Monk's protection had no real effect"
        );
    }

    #[test]
    fn test_poisoner_suppresses_slayer_kill_effect() {
        let roles = vec![
            RoleNames::Poisoner,
            RoleNames::Slayer,
            RoleNames::Soldier,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let poisoner_index = find_role(&state, RoleNames::Poisoner);
        let slayer_index = find_role(&state, RoleNames::Slayer);
        let imp_index = find_role(&state, RoleNames::Imp);

        // Poisoner poisons the Slayer
        let poisoner_role = Roles::new(&RoleNames::Poisoner);
        let cr = poisoner_role.night_ability(poisoner_index, &state).unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![slayer_index]))
            .unwrap();

        // Poisoned Slayer points at the Imp. Go through the Player wrapper so poison
        // suppression (`drunkify`) actually applies.
        let cr = state
            .get_player(slayer_index)
            .day_ability(slayer_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![imp_index]))
            .unwrap();

        // Per wiki rule 2, the poisoned Slayer's shot should have no real effect.
        assert!(
            !state.get_player(imp_index).dead,
            "Imp should survive a poisoned Slayer's shot"
        );
    }

    #[test]
    fn test_poisoner_poisoned_once_per_game_ability_still_consumed() {
        let roles = vec![
            RoleNames::Poisoner,
            RoleNames::Slayer,
            RoleNames::Soldier,
            RoleNames::Imp,
        ];
        let mut state = setup_test_state(roles);

        let poisoner_index = find_role(&state, RoleNames::Poisoner);
        let slayer_index = find_role(&state, RoleNames::Slayer);
        let imp_index = find_role(&state, RoleNames::Imp);

        // Poisoner poisons the Slayer
        let poisoner_role = Roles::new(&RoleNames::Poisoner);
        let cr = poisoner_role.night_ability(poisoner_index, &state).unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![slayer_index]))
            .unwrap();

        assert!(
            state.get_player(slayer_index).role.has_day_ability(),
            "Slayer's once-per-game ability should not be marked used yet"
        );

        // Poisoned Slayer uses their once-per-game ability anyway. Go through the Player
        // wrapper so poison suppression (`drunkify`) actually applies.
        let cr = state
            .get_player(slayer_index)
            .day_ability(slayer_index, &state)
            .unwrap();
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::PlayerIndices(vec![imp_index]))
            .unwrap();

        // Per wiki rule 3, a poisoned once-per-game ability use is still consumed,
        // even though it had no real effect.
        assert!(
            !state.get_player(slayer_index).role.has_day_ability(),
            "Slayer's once-per-game ability should be consumed even though it was poisoned"
        );
        assert!(
            !state.get_player(imp_index).dead,
            "the poisoned Slayer's shot should have had no real effect"
        );
    }

    // -------------------- Spy --------------------

    #[test]
    fn test_spy_sees_grimoire_night_one() {
        let roles = vec![RoleNames::Spy, RoleNames::Soldier, RoleNames::Imp];
        let state = setup_test_state(roles);

        let spy_index = find_role(&state, RoleNames::Spy);

        let spy_role = Roles::new(&RoleNames::Spy);
        let cr = spy_role
            .night_one_ability(spy_index, &state)
            .expect("Spy should see the Grimoire on night one");

        assert_eq!(cr.change_type, ChangeType::Display);
    }

    #[test]
    fn test_spy_sees_grimoire_each_night() {
        let roles = vec![RoleNames::Spy, RoleNames::Soldier, RoleNames::Imp];
        let state = setup_test_state(roles);

        let spy_index = find_role(&state, RoleNames::Spy);

        let spy_role = Roles::new(&RoleNames::Spy);
        let cr = spy_role
            .night_ability(spy_index, &state)
            .expect("Spy should see the Grimoire each night");

        assert_eq!(cr.change_type, ChangeType::Display);
    }

    #[test]
    fn test_spy_ability_continues_after_death() {
        let roles = vec![RoleNames::Spy, RoleNames::Soldier, RoleNames::Imp];
        let mut state = setup_test_state(roles);

        let spy_index = find_role(&state, RoleNames::Spy);
        state.get_player_mut(spy_index).dead = true;

        let spy_role = Roles::new(&RoleNames::Spy);
        // Per wiki rule 5, the Spy's ability continues to function even after death.
        assert!(
            spy_role.night_ability(spy_index, &state).is_some(),
            "Dead Spy should still see the Grimoire"
        );
    }

    #[test]
    fn test_spy_registers_as_storyteller_choice() {
        let roles = vec![RoleNames::Spy, RoleNames::Soldier, RoleNames::Imp];
        let state = setup_test_state(roles);

        let spy_index = find_role(&state, RoleNames::Spy);
        let spy_role = &state.get_player(spy_index).role;

        // True nature: an evil Minion.
        assert_eq!(spy_role.get_default_alignment(), Alignment::Evil);
        assert_eq!(spy_role.get_true_character_type(), CharacterType::Minion);

        // What detection abilities see is left open (Storyteller's choice), modeled
        // here as `Any` rather than a fixed good alignment / Townsfolk-or-Outsider type.
        assert_eq!(spy_role.get_alignment(), Alignment::Any);
        assert_eq!(spy_role.get_character_type(), CharacterType::Any);
    }

    // -------------------- Scarlet Woman --------------------

    #[test]
    fn test_scarlet_woman_promotes_to_demon_when_imp_dies_with_enough_players() {
        // 5 players alive when the Imp dies -> Scarlet Woman should become the Demon.
        let roles = vec![
            RoleNames::ScarletWoman,
            RoleNames::Imp,
            RoleNames::Soldier,
            RoleNames::Monk,
            RoleNames::Virgin,
        ];
        let mut state = setup_test_state(roles);

        let scarlet_index = find_role(&state, RoleNames::ScarletWoman);
        let imp_index = find_role(&state, RoleNames::Imp);

        state.execute_player(imp_index);

        assert_eq!(
            state.change_request_queue.len(),
            2,
            "Scarlet Woman's promotion and display change requests should be queued"
        );

        let swap_cr = state.change_request_queue.pop_front().unwrap();
        swap_cr
            .state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Blank)
            .unwrap();

        assert_eq!(
            state.get_player(scarlet_index).role.to_role_name(),
            RoleNames::Imp,
            "Scarlet Woman should become the Imp"
        );
        assert_eq!(
            state.get_player(scarlet_index).role.get_true_character_type(),
            CharacterType::Demon
        );
    }

    #[test]
    fn test_scarlet_woman_no_promotion_below_five_players() {
        // Only 4 players total -> fewer than 5 alive when the Imp dies, so no promotion.
        let roles = vec![
            RoleNames::ScarletWoman,
            RoleNames::Imp,
            RoleNames::Soldier,
            RoleNames::Monk,
        ];
        let mut state = setup_test_state(roles);

        let scarlet_index = find_role(&state, RoleNames::ScarletWoman);
        let imp_index = find_role(&state, RoleNames::Imp);

        state.execute_player(imp_index);

        assert!(
            state.change_request_queue.is_empty(),
            "Scarlet Woman should not be promoted with fewer than 5 players alive"
        );
        assert_eq!(
            state.get_player(scarlet_index).role.to_role_name(),
            RoleNames::ScarletWoman
        );
    }

    // -------------------- Baron --------------------

    #[test]
    fn test_baron_initialization_effect_adds_two_outsiders() {
        let baron_role = Roles::new(&RoleNames::Baron);
        let effect = baron_role
            .initialization_effect()
            .expect("Baron should have a setup character-count effect");

        assert_eq!(effect.townsfolk, -2);
        assert_eq!(effect.outsiders, 2);
        assert_eq!(effect.minions, 0);
        assert_eq!(effect.demons, 0);
    }

    #[test]
    fn test_baron_shifts_character_type_counts_on_choose() {
        // 7 players: 5 townsfolk, 0 outsiders, 1 minion, 1 demon by default.
        let mut counts = CharacterTypeCounts::new(7).unwrap();
        assert_eq!(counts.townsfolk, 5);
        assert_eq!(counts.outsiders, 0);

        counts.on_choose(RoleNames::Baron);

        assert_eq!(counts.townsfolk, 3);
        assert_eq!(counts.outsiders, 2);
    }
}
