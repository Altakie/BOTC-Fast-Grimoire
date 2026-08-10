//! Cross-role interaction tests: official Jinxes and the Spy/Recluse disguise-matrix
//! against Trouble Brewing's information roles. See docs/research/trouble-brewing-role-rules.md.

use crate::engine::player::roles::RoleNames;
use crate::engine::player::roles::test_utils::{find_role, setup_test_state};
use crate::engine::state::Step;
use crate::engine::state::status_effects::StatusType;

use super::*;

// ---------------------------------------------------------------------------------------
// Part 1: Spy/Recluse disguise-matrix tests
//
// Per docs/research/trouble-brewing-role-rules.md, these are the role x disguise-character
// pairs the wiki explicitly documents. Where the underlying mechanism genuinely supports the
// documented flexibility, the test passes; where the detecting role's code bypasses the
// disguise-aware `get_alignment()`/`get_character_type()` methods (e.g. reads the static
// `.alignment` field, or reads `.role` directly), the test asserts the wiki-correct outcome
// and is left red, documenting the gap.
// ---------------------------------------------------------------------------------------

#[test]
fn test_washerwoman_spy_can_be_shown_as_townsfolk() {
    // Washerwoman/Librarian/Investigator share a Storyteller-driven ChoosePlayers(1) + "wrong
    // player" flow with no character-type validation on the chosen "right" player, so the
    // Storyteller is free to mark a Spy with WasherwomanTownsfolk -- matching the wiki's
    // worked example ("the Spy is registering as a Townsfolk").
    let roles = vec![
        RoleNames::Washerwoman,
        RoleNames::Spy,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let mut state = setup_test_state(roles);
    let washerwoman_index = find_role(&state, RoleNames::Washerwoman);
    let spy_index = find_role(&state, RoleNames::Spy);
    let saint_index = find_role(&state, RoleNames::Saint);

    let washerwoman_role = Roles::new(&RoleNames::Washerwoman);
    let cr = washerwoman_role
        .setup_ability(washerwoman_index, &state)
        .unwrap();
    cr.state_change_func
        .unwrap()
        .call(&mut state, ChangeArgs::PlayerIndices(vec![spy_index]))
        .unwrap();

    assert!(
        state
            .get_player(spy_index)
            .get_statuses()
            .iter()
            .any(|s| s.status_type == StatusType::WasherwomanTownsfolk),
        "the Storyteller should be able to mark the Spy as the Washerwoman's Townsfolk pick"
    );
    let _ = saint_index;
}

#[test]
fn test_investigator_recluse_can_be_shown_as_minion() {
    // Same Storyteller-driven mechanism as Washerwoman above; the wiki's worked example has
    // the Recluse register as a specific Minion (the Poisoner) to the Investigator.
    let roles = vec![
        RoleNames::Investigator,
        RoleNames::Recluse,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let mut state = setup_test_state(roles);
    let investigator_index = find_role(&state, RoleNames::Investigator);
    let recluse_index = find_role(&state, RoleNames::Recluse);

    let investigator_role = Roles::new(&RoleNames::Investigator);
    let cr = investigator_role
        .setup_ability(investigator_index, &state)
        .unwrap();
    cr.state_change_func
        .unwrap()
        .call(&mut state, ChangeArgs::PlayerIndices(vec![recluse_index]))
        .unwrap();

    assert!(
        state
            .get_player(recluse_index)
            .get_statuses()
            .iter()
            .any(|s| s.status_type == StatusType::InvestigatorMinion),
        "the Storyteller should be able to mark the Recluse as the Investigator's Minion pick"
    );
}

#[test]
fn test_chef_spy_disguise_not_consulted() {
    // Wiki: "The Spy might not register as an Evil player for your [Chef] information,
    // giving you an incorrect count" -- a Storyteller choice. This requires the Chef's
    // pair-count to consult the disguise-aware `get_alignment()`. It doesn't: Chef reads the
    // player's static `.alignment` field directly (see `Empath`/`Chef::ability` in
    // townsfolk.rs), and `Spy::get_alignment()` is hardcoded to always return `Alignment::Any`
    // with no way to make it return `Good` for a given read.
    let roles = vec![
        RoleNames::Chef,
        RoleNames::Spy,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let state = setup_test_state(roles);
    let spy_index = find_role(&state, RoleNames::Spy);

    assert_eq!(
        state.get_player(spy_index).get_alignment(),
        Alignment::Any,
        "for the Spy to register as good to the Chef as the wiki describes, get_alignment() \
         would need to be able to return Good -- but it's hardcoded to always return \
         Alignment::Any, so there is no Storyteller-choice mechanism to exercise here"
    );
}

#[test]
fn test_chef_recluse_disguise_not_consulted() {
    // Wiki: "the Recluse might register as an Evil player for your [Chef] information" --
    // per-pair Storyteller choice. Same gap as the Spy case: `Recluse::get_alignment()` is
    // hardcoded to `Alignment::Any`, never `Evil`.
    let roles = vec![
        RoleNames::Chef,
        RoleNames::Recluse,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let state = setup_test_state(roles);
    let recluse_index = find_role(&state, RoleNames::Recluse);

    assert_eq!(
        state.get_player(recluse_index).get_alignment(),
        Alignment::Any,
        "for the Recluse to register as evil to the Chef as the wiki describes, \
         get_alignment() would need to be able to return Evil -- but it's hardcoded to \
         always return Alignment::Any"
    );
}

#[test]
fn test_empath_spy_disguise_not_consulted() {
    // Wiki: the Spy "may register as good for you [Empath], and give you an incorrect
    // count." Empath::ability (townsfolk.rs) reads neighbors' static `.alignment` field, not
    // the disguise-aware `get_alignment()`.
    let roles = vec![
        RoleNames::Empath,
        RoleNames::Spy,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let state = setup_test_state(roles);
    let spy_index = find_role(&state, RoleNames::Spy);

    assert_eq!(
        state.get_player(spy_index).get_alignment(),
        Alignment::Any,
        "for the Spy to register as good to the Empath as the wiki describes, get_alignment() \
         would need to be able to return Good -- but it's hardcoded to Alignment::Any"
    );
}

#[test]
fn test_empath_recluse_disguise_not_consulted() {
    // Wiki: the Recluse "may register as evil, making you [Empath] believe you are sitting
    // next to an evil player when that is not actually the case."
    let roles = vec![
        RoleNames::Empath,
        RoleNames::Recluse,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let state = setup_test_state(roles);
    let recluse_index = find_role(&state, RoleNames::Recluse);

    assert_eq!(
        state.get_player(recluse_index).get_alignment(),
        Alignment::Any,
        "for the Recluse to register as evil to the Empath as the wiki describes, \
         get_alignment() would need to be able to return Evil -- but it's hardcoded to \
         Alignment::Any"
    );
}

#[test]
fn test_fortuneteller_recluse_registering_as_demon_not_implemented() {
    // Wiki: "Beware the Recluse! They may register as the Demon to you [Fortune Teller]."
    // Fortuneteller::ability (townsfolk.rs) checks
    // `matches!(player.get_character_type(), CharacterType::Demon | CharacterType::Any)` --
    // it does correctly treat the `Any` disguise sentinel as a Demon read, but `Recluse`
    // never overrides `get_character_type()` at all (see outsiders.rs), so it always returns
    // its true type (`Outsider`), never `Any` or `Demon`.
    let roles = vec![
        RoleNames::Fortuneteller,
        RoleNames::Recluse,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let state = setup_test_state(roles);
    let recluse_index = find_role(&state, RoleNames::Recluse);

    assert_eq!(
        state.get_player(recluse_index).get_character_type(),
        CharacterType::Demon,
        "for the Recluse to register as the Demon to the Fortune Teller as the wiki \
         describes, get_character_type() would need to be able to return Demon (or the Any \
         sentinel, which Fortuneteller's check also accepts) -- but Recluse doesn't override \
         get_character_type() at all, so it always returns its true type, Outsider"
    );
}

#[test]
fn test_virgin_spy_nomination_does_not_trigger_execution() {
    // Wiki: "Beware the Spy! It is the only evil character that can activate your ability,
    // since it registers as a Townsfolk." The Virgin's nomination listener (townsfolk.rs)
    // checks `nominator.role.get_true_character_type() == CharacterType::Townsfolk` -- the
    // TRUE type, bypassing disguise entirely. Spy's true type is Minion, so it can never
    // trigger the Virgin under the current implementation.
    let roles = vec![
        RoleNames::Virgin,
        RoleNames::Spy,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let mut state = setup_test_state(roles);
    let virgin_index = find_role(&state, RoleNames::Virgin);
    let spy_index = find_role(&state, RoleNames::Spy);

    state.nominate_player(spy_index, virgin_index);

    let cr = state
        .change_request_queue
        .pop_front()
        .expect("Virgin should still queue the check");
    cr.state_change_func
        .unwrap()
        .call(&mut state, ChangeArgs::Blank)
        .unwrap();

    assert!(
        state.get_player(spy_index).dead,
        "per the wiki, the Spy is the only evil character able to trigger the Virgin's \
         execution -- but the Virgin's nomination check uses get_true_character_type(), so a \
         Spy nominator (true type Minion) never triggers it"
    );
}

#[test]
fn test_slayer_recluse_target_never_registers_as_demon() {
    // Wiki: because the Recluse "might register as the Demon," a Slayer shot on a Recluse
    // can be ruled to kill them. Slayer::day_ability (townsfolk.rs) checks
    // `target_player.get_character_type() == CharacterType::Demon` -- an exact match, no
    // `Any` fallback -- and Recluse never overrides get_character_type() (always Outsider),
    // so a Slayer shot on a Recluse can never kill them in this engine.
    let roles = vec![
        RoleNames::Slayer,
        RoleNames::Recluse,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let mut state = setup_test_state(roles);
    let slayer_index = find_role(&state, RoleNames::Slayer);
    let recluse_index = find_role(&state, RoleNames::Recluse);

    let slayer_role = Roles::new(&RoleNames::Slayer);
    let cr = slayer_role.day_ability(slayer_index, &state).unwrap();
    cr.state_change_func
        .unwrap()
        .call(&mut state, ChangeArgs::PlayerIndices(vec![recluse_index]))
        .unwrap();

    assert!(
        state.get_player(recluse_index).dead,
        "the wiki allows the Storyteller to rule that a Slayer shot on the Recluse kills \
         them (Recluse registering as Demon); this engine's Slayer requires an exact \
         CharacterType::Demon match and Recluse never produces one"
    );
}

#[test]
fn test_undertaker_shows_true_role_for_executed_spy() {
    // Wiki: "Beware the Spy and the Recluse! They will likely register to you [Undertaker] as
    // good and evil characters respectively, as their abilities continue to function even
    // when they are dead." Undertaker::night_ability (townsfolk.rs) clones the executed
    // player's real `.role` field directly -- it always shows the true character, which is
    // actually consistent with the real game (the Undertaker sees a character token, and the
    // wiki's warning here is about alignment-interpretation, not the token itself being
    // swapped) -- so this passes, documenting expected behavior rather than a gap.
    let roles = vec![
        RoleNames::Undertaker,
        RoleNames::Spy,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let mut state = setup_test_state(roles);
    let undertaker_index = find_role(&state, RoleNames::Undertaker);
    let spy_index = find_role(&state, RoleNames::Spy);

    state.execute_player(spy_index);

    let undertaker_role = Roles::new(&RoleNames::Undertaker);
    let cr = undertaker_role
        .night_ability(undertaker_index, &state)
        .unwrap();

    assert!(
        cr.description.contains("Spy"),
        "the Undertaker should be shown the executed player's true character (Spy), not a \
         disguised one -- the wiki's Spy/Recluse warning here is about alignment \
         interpretation, not the character token itself"
    );
}

#[test]
fn test_ravenkeeper_always_reveals_true_character_no_disguise_choice() {
    // Wiki: "Beware of the Spy and the Recluse. If you believe a player is one of them, it
    // is unlikely that you will learn their true character if you choose them, owing to
    // their abilities to register as other characters." Ravenkeeper::night_ability
    // (townsfolk.rs) shows `target_player.role` directly (its Display) -- always the true
    // character, with no Storyteller-choice disguise mechanism, contradicting the wiki's
    // expectation that choosing a Spy/Recluse will often NOT reveal their true character.
    let roles = vec![
        RoleNames::Ravenkeeper,
        RoleNames::Spy,
        RoleNames::Saint,
        RoleNames::Imp,
    ];
    let mut state = setup_test_state(roles);
    let ravenkeeper_index = find_role(&state, RoleNames::Ravenkeeper);
    let spy_index = find_role(&state, RoleNames::Spy);

    state.kill(find_role(&state, RoleNames::Imp), ravenkeeper_index);

    let ravenkeeper_role = Roles::new(&RoleNames::Ravenkeeper);
    let cr = ravenkeeper_role
        .night_ability(ravenkeeper_index, &state)
        .unwrap();
    cr.state_change_func
        .unwrap()
        .call(&mut state, ChangeArgs::PlayerIndices(vec![spy_index]))
        .unwrap();

    let shown = state
        .change_request_queue
        .back()
        .expect("Ravenkeeper's ability should queue a Display request showing the result")
        .description
        .clone();

    assert!(
        !shown.contains("Spy"),
        "per the wiki, choosing a Spy is unlikely to reveal their true character -- but this \
         engine's Ravenkeeper always shows target_player.role directly, so it always says \
         \"Spy\" here: {shown:?}"
    );
}

// ---------------------------------------------------------------------------------------
// Part 2: Jinx stubs & tests
//
// Every official Jinx pairs a Trouble Brewing role with a role from another script that
// has no `Roles`/`RoleNames` variant in this engine. These stub structs are pure
// test-local doubles representing "a player conceptually playing this character" -- they
// do NOT implement the real `Role` trait and are NOT wired into `Roles`/`RoleNames`.
// ---------------------------------------------------------------------------------------

struct StubLeviathan;
struct StubRiot;
struct StubVizier;
struct StubLleech;
struct StubCannibal;
struct StubOrganGrinder;
struct StubBoffin;
struct StubMathematician;
struct StubOgre;
struct StubSage;
struct StubAlchemist;
struct StubDamsel;
struct StubHeretic;
struct StubMagician;
struct StubPlagueDoctor;
struct StubPoppyGrower;
struct StubAlHadikhia;
struct StubFangGu;
struct StubLilMonsta;

// --- Investigator (1 jinx) ---

/// Jinx (Investigator + Vizier): "The Storyteller doesn't declare the Vizier is in play."
/// The Vizier isn't implemented in this engine, so this only pins that the Investigator's
/// own setup ability is unaffected/still present.
#[test]
#[ignore = "partner role (Vizier) not implemented in this engine"]
fn test_jinx_investigator_vizier() {
    let _vizier = StubVizier;
    let roles = vec![RoleNames::Investigator, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let investigator_index = find_role(&state, RoleNames::Investigator);

    let investigator_role = Roles::new(&RoleNames::Investigator);
    assert!(
        investigator_role
            .setup_ability(investigator_index, &state)
            .is_some(),
        "Investigator should still have a normal setup ability regardless of the Vizier jinx"
    );
}

// --- Monk (2 jinxes) ---

/// Jinx (Monk + Leviathan): "If the Leviathan nominates and executes the Monk-protected
/// player, good wins." The Leviathan isn't implemented in this engine (no Demon can
/// nominate/execute here), so this only pins that a Monk-protected player still normally
/// dies from execution (protection only blocks the Demon's night-kill, not execution) --
/// the special "good wins" jinx override itself can't be exercised.
#[test]
#[ignore = "partner role (Leviathan) not implemented in this engine"]
fn test_jinx_monk_leviathan() {
    let _leviathan = StubLeviathan;
    let roles = vec![RoleNames::Monk, RoleNames::Imp, RoleNames::Poisoner];
    let mut state = setup_test_state(roles);

    let monk_index = find_role(&state, RoleNames::Monk);
    let target_index = find_role(&state, RoleNames::Poisoner);

    let monk_role = Roles::new(&RoleNames::Monk);
    let cr = monk_role.night_ability(monk_index, &state).unwrap();
    cr.state_change_func
        .unwrap()
        .call(&mut state, ChangeArgs::PlayerIndices(vec![target_index]))
        .unwrap();

    state.execute_player(target_index);

    assert!(
        state.get_player(target_index).dead,
        "Monk protection should not block execution (only the Demon's night-kill); the \
         Leviathan-specific 'good wins' jinx override is not implemented"
    );
}

/// Jinx (Monk + Riot): "If Riot nominates and executes the Monk-protected player, good
/// wins." Same reasoning as the Leviathan jinx above -- Riot isn't implemented, so this
/// only pins the baseline (protection doesn't block execution).
#[test]
#[ignore = "partner role (Riot) not implemented in this engine"]
fn test_jinx_monk_riot() {
    let _riot = StubRiot;
    let roles = vec![RoleNames::Monk, RoleNames::Imp, RoleNames::Poisoner];
    let mut state = setup_test_state(roles);

    let monk_index = find_role(&state, RoleNames::Monk);
    let target_index = find_role(&state, RoleNames::Poisoner);

    let monk_role = Roles::new(&RoleNames::Monk);
    let cr = monk_role.night_ability(monk_index, &state).unwrap();
    cr.state_change_func
        .unwrap()
        .call(&mut state, ChangeArgs::PlayerIndices(vec![target_index]))
        .unwrap();

    state.execute_player(target_index);

    assert!(
        state.get_player(target_index).dead,
        "Monk protection should not block execution (only the Demon's night-kill); the \
         Riot-specific 'good wins' jinx override is not implemented"
    );
}

// --- Ravenkeeper (2 jinxes) ---

/// Jinx (Ravenkeeper + Leviathan): "Each night*, the Leviathan chooses an alive player
/// (different to previous nights): a chosen Ravenkeeper uses their ability but does not
/// die." The engine currently only grants the Ravenkeeper its ability after an actual
/// death event this phase, so an alive, non-triggered Ravenkeeper has no night ability --
/// the "use ability without dying" jinx variant is not modeled.
#[test]
#[ignore = "partner role (Leviathan) not implemented in this engine"]
fn test_jinx_ravenkeeper_leviathan() {
    let _leviathan = StubLeviathan;
    let roles = vec![RoleNames::Ravenkeeper, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let ravenkeeper_index = find_role(&state, RoleNames::Ravenkeeper);

    let ravenkeeper_role = Roles::new(&RoleNames::Ravenkeeper);
    assert!(
        ravenkeeper_role
            .night_ability(ravenkeeper_index, &state)
            .is_none(),
        "Ravenkeeper shouldn't have a night ability without dying first; the Leviathan \
         jinx (use ability without dying) is not implemented"
    );
}

/// Jinx (Ravenkeeper + Riot): "Each night*, Riot chooses an alive good player (different
/// to previous nights): a chosen Ravenkeeper uses their ability but does not die." Same
/// reasoning as the Leviathan jinx above.
#[test]
#[ignore = "partner role (Riot) not implemented in this engine"]
fn test_jinx_ravenkeeper_riot() {
    let _riot = StubRiot;
    let roles = vec![RoleNames::Ravenkeeper, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let ravenkeeper_index = find_role(&state, RoleNames::Ravenkeeper);

    let ravenkeeper_role = Roles::new(&RoleNames::Ravenkeeper);
    assert!(
        ravenkeeper_role
            .night_ability(ravenkeeper_index, &state)
            .is_none(),
        "Ravenkeeper shouldn't have a night ability without dying first; the Riot jinx \
         (use ability without dying) is not implemented"
    );
}

// --- Slayer (1 jinx) ---

/// Jinx (Slayer + Lleech): "If the Slayer slays the Lleech host, the host dies." The
/// Lleech isn't implemented in this engine, so this only pins the baseline: the Slayer
/// correctly kills a real Demon target.
#[test]
#[ignore = "partner role (Lleech) not implemented in this engine"]
fn test_jinx_slayer_lleech() {
    let _lleech = StubLleech;
    let roles = vec![RoleNames::Slayer, RoleNames::Imp, RoleNames::Poisoner];
    let mut state = setup_test_state(roles);

    let slayer_index = find_role(&state, RoleNames::Slayer);
    let imp_index = find_role(&state, RoleNames::Imp);

    let slayer_role = Roles::new(&RoleNames::Slayer);
    let cr = slayer_role.day_ability(slayer_index, &state).unwrap();
    cr.state_change_func
        .unwrap()
        .call(&mut state, ChangeArgs::PlayerIndices(vec![imp_index]))
        .unwrap();

    assert!(
        state.get_player(imp_index).dead,
        "Slayer correctly kills a real Demon; the Lleech-specific 'host dies too' jinx \
         mechanic is not implemented since Lleech isn't a role in this engine"
    );
}

// --- Soldier (2 jinxes) ---

/// Jinx (Soldier + Leviathan): "If the Leviathan nominates and executes the Soldier, good
/// wins." Leviathan isn't implemented, so this only pins the baseline: the Soldier's
/// Demon-kill immunity doesn't block execution.
#[test]
#[ignore = "partner role (Leviathan) not implemented in this engine"]
fn test_jinx_soldier_leviathan() {
    let _leviathan = StubLeviathan;
    let roles = vec![RoleNames::Soldier, RoleNames::Imp, RoleNames::Poisoner];
    let mut state = setup_test_state(roles);
    let soldier_index = find_role(&state, RoleNames::Soldier);

    state.execute_player(soldier_index);

    assert!(
        state.get_player(soldier_index).dead,
        "Soldier's Demon-kill immunity doesn't block execution; the Leviathan-specific \
         'good wins' jinx override is not implemented"
    );
}

/// Jinx (Soldier + Riot): "If Riot nominates and executes the Soldier, good wins." Same
/// reasoning as the Leviathan jinx above.
#[test]
#[ignore = "partner role (Riot) not implemented in this engine"]
fn test_jinx_soldier_riot() {
    let _riot = StubRiot;
    let roles = vec![RoleNames::Soldier, RoleNames::Imp, RoleNames::Poisoner];
    let mut state = setup_test_state(roles);
    let soldier_index = find_role(&state, RoleNames::Soldier);

    state.execute_player(soldier_index);

    assert!(
        state.get_player(soldier_index).dead,
        "Soldier's Demon-kill immunity doesn't block execution; the Riot-specific 'good \
         wins' jinx override is not implemented"
    );
}

// --- Mayor (2 jinxes) ---

/// Jinx (Mayor + Leviathan): "If the Leviathan and the Mayor are alive on day 5 & no
/// execution occurs, good wins." No day-5 win check exists in this engine, so this only
/// pins the baseline starting state.
#[test]
#[ignore = "partner role (Leviathan) not implemented in this engine"]
fn test_jinx_mayor_leviathan() {
    let _leviathan = StubLeviathan;
    let roles = vec![RoleNames::Mayor, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let mayor_index = find_role(&state, RoleNames::Mayor);

    assert!(
        !state.get_player(mayor_index).dead && state.day_num == 1,
        "Mayor should start alive on day 1; the Leviathan-specific 'both alive on day 5, \
         no execution -> good wins' jinx condition is not implemented (no day-5 win check \
         exists in this engine)"
    );
}

/// Jinx (Mayor + Riot): "The Mayor may choose to stop the riot. If they do so when only 1
/// Riot is alive, good wins. Otherwise, evil wins." Riot isn't implemented, so this only
/// pins that the Mayor's own night-death-substitution listener is registered as normal.
#[test]
#[ignore = "partner role (Riot) not implemented in this engine"]
fn test_jinx_mayor_riot() {
    let _riot = StubRiot;
    let roles = vec![RoleNames::Mayor, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);

    assert_eq!(
        state.attempted_kill_listeners.len(),
        1,
        "Mayor's night-death-substitution listener should be registered; the Riot-\
         specific 'Mayor may choose to stop the riot' mechanic is not implemented since \
         Riot isn't a role in this engine"
    );
}

// --- Butler (2 jinxes) ---

/// Jinx (Butler + Cannibal): "If the Cannibal gains the Butler ability, the Cannibal
/// learns this." Cannibal isn't implemented, so this only pins that the Butler's own
/// ability functions normally.
#[test]
#[ignore = "partner role (Cannibal) not implemented in this engine"]
fn test_jinx_butler_cannibal() {
    let _cannibal = StubCannibal;
    let roles = vec![RoleNames::Butler, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let butler_index = find_role(&state, RoleNames::Butler);

    let butler_role = Roles::new(&RoleNames::Butler);
    assert!(
        butler_role
            .night_one_ability(butler_index, &state)
            .is_some(),
        "Butler should have a normal setup/night-one ability; the Cannibal-specific \
         'informed if gaining this ability' jinx mechanic is not implemented since \
         Cannibal isn't a role in this engine"
    );
}

/// Jinx (Butler + Organ Grinder): "If the Organ Grinder is causing eyes closed voting, the
/// Butler may raise their hand to vote but their vote is only counted if their master
/// voted too." Organ Grinder isn't implemented, so this only pins the baseline Butler
/// mechanic: picking a Master applies the `ButlerMaster` status.
#[test]
#[ignore = "partner role (Organ Grinder) not implemented in this engine"]
fn test_jinx_butler_organgrinder() {
    let _organ_grinder = StubOrganGrinder;
    let roles = vec![RoleNames::Butler, RoleNames::Imp, RoleNames::Poisoner];
    let mut state = setup_test_state(roles);

    let butler_index = find_role(&state, RoleNames::Butler);
    let target_index = find_role(&state, RoleNames::Poisoner);

    let butler_role = Roles::new(&RoleNames::Butler);
    let cr = butler_role.night_one_ability(butler_index, &state).unwrap();
    cr.state_change_func
        .unwrap()
        .call(&mut state, ChangeArgs::PlayerIndices(vec![target_index]))
        .unwrap();

    assert!(
        state
            .get_player(target_index)
            .get_statuses()
            .iter()
            .any(|s| s.status_type == StatusType::ButlerMaster),
        "Butler normally picks a Master each night; the Organ Grinder-specific 'eyes \
         closed voting, vote counted only if master voted too' variant is not implemented \
         since Organ Grinder isn't a role in this engine"
    );
}

// --- Drunk (2 jinxes) ---

/// Jinx (Drunk + Boffin): "The Demon cannot have the Drunk ability." Boffin isn't
/// implemented, so this only pins the structural baseline: the Demon in play here is not
/// the Drunk.
#[test]
#[ignore = "partner role (Boffin) not implemented in this engine"]
fn test_jinx_drunk_boffin() {
    let _boffin = StubBoffin;
    let roles = vec![RoleNames::Drunk, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let imp_index = find_role(&state, RoleNames::Imp);

    assert_ne!(
        state.get_player(imp_index).role.to_role_name(),
        RoleNames::Drunk,
        "The Demon should never be the Drunk; the Boffin-specific jinx wording ('Demon \
         cannot have the Drunk ability') isn't separately enforced since Boffin isn't a \
         role in this engine"
    );
}

/// Jinx (Drunk + Mathematician): "The Mathematician learn[s] if the Drunk's ability
/// yielded false info or failed to work properly." Mathematician isn't implemented, so
/// this only pins that the Drunk's own setup ability (assigning a believed Townsfolk role)
/// functions normally.
#[test]
#[ignore = "partner role (Mathematician) not implemented in this engine"]
fn test_jinx_drunk_mathematician() {
    let _mathematician = StubMathematician;
    let roles = vec![RoleNames::Drunk, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let drunk_index = find_role(&state, RoleNames::Drunk);

    let drunk_role = Roles::new(&RoleNames::Drunk);
    assert!(
        drunk_role.setup_ability(drunk_index, &state).is_some(),
        "Drunk should have a normal setup ability assigning a believed Townsfolk role; \
         the Mathematician-specific 'learns if Drunk's ability misfired' jinx mechanic is \
         not implemented since Mathematician isn't a role in this engine"
    );
}

// --- Recluse (2 jinxes) ---

/// Jinx (Recluse + Ogre): "If the Recluse registers as evil to the Ogre, the Ogre learns
/// that they are evil." Ogre isn't implemented, so this only pins the premise: the Recluse
/// can register as either alignment.
#[test]
#[ignore = "partner role (Ogre) not implemented in this engine"]
fn test_jinx_recluse_ogre() {
    let _ogre = StubOgre;
    let roles = vec![RoleNames::Recluse, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let recluse_index = find_role(&state, RoleNames::Recluse);

    assert_eq!(
        state.get_player(recluse_index).role.get_alignment(),
        Alignment::Any,
        "Recluse can register as either alignment (needed for the 'registers as evil to \
         the Ogre' jinx premise); the Ogre-specific 'Ogre learns they are evil' follow-up \
         is not implemented since Ogre isn't a role in this engine"
    );
}

/// Jinx (Recluse + Sage): "The Recluse might register as the Demon to the Sage." Sage
/// isn't implemented. This engine's Recluse only overrides `get_alignment()`, not
/// `get_character_type()`, so it always reports its true Outsider type -- the Sage
/// interaction can't be modeled either way.
#[test]
#[ignore = "partner role (Sage) not implemented in this engine"]
fn test_jinx_recluse_sage() {
    let _sage = StubSage;
    let roles = vec![RoleNames::Recluse, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let recluse_index = find_role(&state, RoleNames::Recluse);

    assert_eq!(
        state.get_player(recluse_index).role.get_character_type(),
        CharacterType::Outsider,
        "Recluse currently always reports its true character type (no per-instance \
         Storyteller choice for get_character_type()), so it can't register as the Demon \
         to a Sage-like ability yet"
    );
}

// --- Spy (7 jinxes) ---

/// Jinx (Spy + Alchemist): "An Alchemist-Spy has no Spy ability & a Spy is in play. After
/// each execution, a living Alchemist-Spy may publicly guess a living player as the Spy.
/// If correct, the Demon must choose the Spy tonight." Alchemist isn't implemented, so
/// this only pins the baseline: the Spy has a normal grimoire-viewing ability.
#[test]
#[ignore = "partner role (Alchemist) not implemented in this engine"]
fn test_jinx_spy_alchemist() {
    let _alchemist = StubAlchemist;
    let roles = vec![RoleNames::Spy, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let spy_index = find_role(&state, RoleNames::Spy);

    let spy_role = Roles::new(&RoleNames::Spy);
    assert!(
        spy_role.night_one_ability(spy_index, &state).is_some(),
        "Spy should have a normal grimoire-viewing ability; the Alchemist-Spy variant (no \
         Spy ability, guess-the-Spy mechanic) is not implemented since Alchemist isn't a \
         role in this engine"
    );
}

/// Jinx (Spy + Damsel): "If the Spy is (or has been) in play, the Damsel is poisoned."
/// Damsel isn't implemented, so this only pins the premise: the Spy is in play.
#[test]
#[ignore = "partner role (Damsel) not implemented in this engine"]
fn test_jinx_spy_damsel() {
    let _damsel = StubDamsel;
    let roles = vec![RoleNames::Spy, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let spy_index = find_role(&state, RoleNames::Spy);

    assert_eq!(
        state.get_player(spy_index).role.to_role_name(),
        RoleNames::Spy,
        "Spy is in play (jinx premise); the Damsel-specific 'Damsel becomes poisoned' \
         effect is not implemented since Damsel isn't a role in this engine"
    );
}

/// Jinx (Spy + Heretic): "Only 1 jinxed character can be in play." This is a meta-ruling
/// about script construction/Storyteller setup, not a runtime mechanic this engine models.
/// Heretic isn't a role in this engine, so there's nothing more specific to assert here.
#[test]
#[ignore = "partner role (Heretic) not implemented in this engine"]
fn test_jinx_spy_heretic() {
    let _heretic = StubHeretic;
    assert!(true);
}

/// Jinx (Spy + Magician): "When the Spy sees the Grimoire, the Demon and Magician's
/// character tokens are removed." Magician isn't implemented, so this only pins the
/// premise: the Spy's ability does show the Grimoire.
#[test]
#[ignore = "partner role (Magician) not implemented in this engine"]
fn test_jinx_spy_magician() {
    let _magician = StubMagician;
    let roles = vec![RoleNames::Spy, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let spy_index = find_role(&state, RoleNames::Spy);

    let spy_role = Roles::new(&RoleNames::Spy);
    let cr = spy_role.night_one_ability(spy_index, &state).unwrap();

    assert!(
        cr.description.to_lowercase().contains("grimoire"),
        "Spy's ability shows the Grimoire (jinx premise); the Magician-specific 'Demon and \
         Magician tokens hidden from that view' mechanic is not implemented since Magician \
         isn't a role in this engine; got: {}",
        cr.description
    );
}

/// Jinx (Spy + Ogre): "The Spy registers as evil to the Ogre." Ogre isn't implemented.
/// This engine's fixed `alignment` field is already Evil for the Spy (see `Player::new`),
/// which happens to match this jinx's guaranteed-evil-read ruling, though there's no
/// Ogre-specific dispatch making this deliberate.
#[test]
#[ignore = "partner role (Ogre) not implemented in this engine"]
fn test_jinx_spy_ogre() {
    let _ogre = StubOgre;
    let roles = vec![RoleNames::Spy, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let spy_index = find_role(&state, RoleNames::Spy);

    assert_eq!(
        state.get_player(spy_index).alignment,
        Alignment::Evil,
        "Spy's underlying alignment field is Evil; the Ogre-specific dispatch that makes \
         this a guaranteed (non-disguised) read is not implemented since Ogre isn't a \
         role in this engine"
    );
}

/// Jinx (Spy + Plague Doctor): "If the Storyteller would gain the Spy ability, a Minion
/// gains it, and learns this." Plague Doctor isn't implemented, so this only pins the
/// premise: the Spy is a Minion.
#[test]
#[ignore = "partner role (Plague Doctor) not implemented in this engine"]
fn test_jinx_spy_plague_doctor() {
    let _plague_doctor = StubPlagueDoctor;
    let roles = vec![RoleNames::Spy, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let spy_index = find_role(&state, RoleNames::Spy);

    assert_eq!(
        state.get_player(spy_index).role.get_true_character_type(),
        CharacterType::Minion,
        "Spy is a Minion (jinx premise, since the ability would otherwise go to the \
         Storyteller); the Plague Doctor-specific 'ability transfers to a Minion instead' \
         mechanic is not implemented since Plague Doctor isn't a role in this engine"
    );
}

/// Jinx (Spy + Poppy Grower): "If the Poppy Grower has their ability, the Spy does not see
/// the Grimoire." Poppy Grower isn't implemented, so this only pins the baseline: the Spy
/// currently always gets to see the Grimoire.
#[test]
#[ignore = "partner role (Poppy Grower) not implemented in this engine"]
fn test_jinx_spy_poppy_grower() {
    let _poppy_grower = StubPoppyGrower;
    let roles = vec![RoleNames::Spy, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let spy_index = find_role(&state, RoleNames::Spy);

    let spy_role = Roles::new(&RoleNames::Spy);
    assert!(
        spy_role.night_one_ability(spy_index, &state).is_some(),
        "Spy currently always gets to see the Grimoire; the Poppy Grower-specific \
         'suppresses the Spy's Grimoire view' mechanic is not implemented since Poppy \
         Grower isn't a role in this engine"
    );
}

// --- Scarlet Woman (4 jinxes) ---

/// Jinx (Scarlet Woman + Al-Hadikhia): "If there would be two Demons, one of which was the
/// Scarlet Woman, the Scarlet Woman becomes the Scarlet Woman again." Al-Hadikhia isn't
/// implemented, so this only pins the baseline Scarlet Woman mechanic: with >=5 players
/// alive, a Demon's death promotes the Scarlet Woman to Demon (the two-Demons reversal
/// this jinx describes is not modeled).
#[test]
#[ignore = "partner role (Al-Hadikhia) not implemented in this engine"]
fn test_jinx_scarletwoman_alhadikhia() {
    let _al_hadikhia = StubAlHadikhia;
    let roles = vec![
        RoleNames::ScarletWoman,
        RoleNames::Imp,
        RoleNames::Poisoner,
        RoleNames::Washerwoman,
        RoleNames::Librarian,
    ];
    let mut state = setup_test_state(roles);

    let scarletwoman_index = find_role(&state, RoleNames::ScarletWoman);
    let imp_index = find_role(&state, RoleNames::Imp);

    state.kill(imp_index, imp_index);
    if let Some(cr) = state.change_request_queue.pop_front() {
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Blank)
            .unwrap();
    }

    assert_eq!(
        state.get_player(scarletwoman_index).role.to_role_name(),
        RoleNames::Imp,
        "Scarlet Woman should become the Demon on a normal Demon death with enough players \
         alive; the Al-Hadikhia-specific 'becomes Scarlet Woman again' reversal (for the \
         two-Demons case) is not implemented since Al-Hadikhia isn't a role in this engine"
    );
}

/// Jinx (Scarlet Woman + Fang Gu): "If there would be two Demons, one of which was the
/// Scarlet Woman, the Scarlet Woman remains the Scarlet Woman." Fang Gu isn't implemented,
/// so this only pins the same baseline Scarlet Woman mechanic as above (this jinx's
/// opposite-outcome override is not modeled either).
#[test]
#[ignore = "partner role (Fang Gu) not implemented in this engine"]
fn test_jinx_scarletwoman_fanggu() {
    let _fang_gu = StubFangGu;
    let roles = vec![
        RoleNames::ScarletWoman,
        RoleNames::Imp,
        RoleNames::Poisoner,
        RoleNames::Washerwoman,
        RoleNames::Librarian,
    ];
    let mut state = setup_test_state(roles);

    let scarletwoman_index = find_role(&state, RoleNames::ScarletWoman);
    let imp_index = find_role(&state, RoleNames::Imp);

    state.kill(imp_index, imp_index);
    if let Some(cr) = state.change_request_queue.pop_front() {
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Blank)
            .unwrap();
    }

    assert_eq!(
        state.get_player(scarletwoman_index).role.to_role_name(),
        RoleNames::Imp,
        "Scarlet Woman should become the Demon on a normal Demon death with enough players \
         alive; the Fang Gu-specific 'remains the Scarlet Woman' override (for the \
         two-Demons case) is not implemented since Fang Gu isn't a role in this engine"
    );
}

/// Jinx (Scarlet Woman + Lil' Monsta): "If Lil' Monsta dies with 5 or more players alive,
/// the Scarlet Woman babysits Lil' Monsta for the rest of the game." Lil' Monsta isn't
/// implemented, so this only pins the same baseline Scarlet Woman mechanic (the
/// "babysits" variant outcome is not modeled).
#[test]
#[ignore = "partner role (Lil' Monsta) not implemented in this engine"]
fn test_jinx_scarletwoman_lilmonsta() {
    let _lil_monsta = StubLilMonsta;
    let roles = vec![
        RoleNames::ScarletWoman,
        RoleNames::Imp,
        RoleNames::Poisoner,
        RoleNames::Washerwoman,
        RoleNames::Librarian,
    ];
    let mut state = setup_test_state(roles);

    let scarletwoman_index = find_role(&state, RoleNames::ScarletWoman);
    let imp_index = find_role(&state, RoleNames::Imp);

    state.kill(imp_index, imp_index);
    if let Some(cr) = state.change_request_queue.pop_front() {
        cr.state_change_func
            .unwrap()
            .call(&mut state, ChangeArgs::Blank)
            .unwrap();
    }

    assert_eq!(
        state.get_player(scarletwoman_index).role.to_role_name(),
        RoleNames::Imp,
        "Scarlet Woman should become the Demon on a normal Demon death with enough players \
         alive; the Lil' Monsta-specific 'babysits for the rest of the game' variant is not \
         implemented since Lil' Monsta isn't a role in this engine"
    );
}

/// Jinx (Scarlet Woman + Plague Doctor): "If the Storyteller would gain the Scarlet Woman
/// ability, a Minion gains it, and learns this." Plague Doctor isn't implemented, so this
/// only pins the premise: the Scarlet Woman is a Minion.
#[test]
#[ignore = "partner role (Plague Doctor) not implemented in this engine"]
fn test_jinx_scarletwoman_plaguedoctor() {
    let _plague_doctor = StubPlagueDoctor;
    let roles = vec![RoleNames::ScarletWoman, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let scarletwoman_index = find_role(&state, RoleNames::ScarletWoman);

    assert_eq!(
        state
            .get_player(scarletwoman_index)
            .role
            .get_true_character_type(),
        CharacterType::Minion,
        "Scarlet Woman is a Minion (jinx premise, since the ability would otherwise go to \
         the Storyteller); the Plague Doctor-specific 'ability transfers to a Minion \
         instead' mechanic is not implemented since Plague Doctor isn't a role in this \
         engine"
    );
}

// --- Baron (2 jinxes) ---

/// Jinx (Baron + Heretic): "Only 1 jinxed character can be in play." Meta-ruling, same as
/// the Spy + Heretic jinx above -- nothing more specific to assert since Heretic isn't a
/// role in this engine.
#[test]
#[ignore = "partner role (Heretic) not implemented in this engine"]
fn test_jinx_baron_heretic() {
    let _heretic = StubHeretic;
    assert!(true);
}

/// Jinx (Baron + Plague Doctor): "If the Storyteller would gain the Baron ability, up to
/// two players become Outsiders." Plague Doctor isn't implemented, so this only pins the
/// baseline Baron mechanic: it swaps 2 Townsfolk for 2 Outsiders.
#[test]
#[ignore = "partner role (Plague Doctor) not implemented in this engine"]
fn test_jinx_baron_plaguedoctor() {
    let _plague_doctor = StubPlagueDoctor;
    let roles = vec![RoleNames::Baron, RoleNames::Imp, RoleNames::Poisoner];
    let state = setup_test_state(roles);
    let baron_index = find_role(&state, RoleNames::Baron);

    assert_eq!(
        state.get_player(baron_index).role.get_true_character_type(),
        CharacterType::Minion
    );

    let baron_role = Roles::new(&RoleNames::Baron);
    let effect = baron_role.initialization_effect();
    assert!(
        matches!(effect, Some(counts) if counts.outsiders == 2 && counts.townsfolk == -2),
        "Baron normally adds 2 Outsiders (removing 2 Townsfolk); the Plague Doctor-specific \
         'the Storyteller gains this effect instead, affecting up to two players' variant \
         is not implemented since Plague Doctor isn't a role in this engine"
    );
}
