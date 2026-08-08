use crate::engine::player::roles::RoleNames;
use crate::engine::state::State;
use crate::scripts::trouble_brewing;

/// Builds a `State` with one player per given role, named after their role, on the
/// Trouble Brewing script. `State::new` shuffles seating order, so tests must look
/// player indices up by role (e.g. via `find_role`) rather than assuming input order.
pub(crate) fn setup_test_state(roles: Vec<RoleNames>) -> State {
    let player_names = roles
        .iter()
        .map(|role| role.convert().to_string())
        .collect();
    State::new(roles, player_names, trouble_brewing()).unwrap()
}

/// Finds the player index holding `role_name`.
/// Panics if no such player exists — tests should only call this for roles they set up.
///
/// Compares via `Roles::to_role_name()` rather than `Display`, since some roles'
/// `Display` impls show in-game-visible text rather than their role name — e.g. the
/// Drunk displays as "The Drunk {believed character}", not "Drunk".
pub(crate) fn find_role(state: &State, role_name: RoleNames) -> usize {
    state
        .get_players()
        .iter()
        .position(|p| p.role.to_role_name() == role_name)
        .unwrap_or_else(|| panic!("no player with role {role_name} in test state"))
}
