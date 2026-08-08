use leptos::prelude::*;
use reactive_stores::Store;
use tracing::{error, info};

use crate::engine::{
    change_request::{ChangeArgs, ChangeType},
    player::Player,
    state::{PlayerIndex, State, StateStoreFields, Step},
};
use crate::ui::game::{TempState, TempStateStoreFields};

/// Sets a player's `dead` flag directly on the store. Shared by the Night Console
/// (toggle button) and the Manual mode editor (bool dropdown) so both stay in sync
/// with a single place that reaches into `Vec<Player>`.
pub(crate) fn set_player_dead(game_state: Store<State>, player_index: PlayerIndex, dead: bool) {
    game_state.players().update(|players: &mut Vec<Player>| {
        players[player_index].dead = dead;
    });
}

/// Sets a player's `ghost_vote` flag directly on the store. See `set_player_dead`.
pub(crate) fn set_player_ghost_vote(
    game_state: Store<State>,
    player_index: PlayerIndex,
    ghost_vote: bool,
) {
    game_state.players().update(|players: &mut Vec<Player>| {
        players[player_index].ghost_vote = ghost_vote;
    });
}

pub(crate) fn apply_cr(game_state: Store<State>, temp_state: Store<TempState>) -> (bool, bool) {
    let mut applied_cr = false;
    loop {
        let cr = temp_state.curr_change_request().get();
        if let Some(cr) = cr {
            let change_type = cr.get_change_type();
            let args = match change_type {
                ChangeType::ChoosePlayers(_) => Some(ChangeArgs::PlayerIndices(
                    temp_state.selected_players().get(),
                )),
                ChangeType::ChooseRoles(_) => {
                    Some(ChangeArgs::Roles(temp_state.selected_roles().get()))
                }
                ChangeType::NoStoryteller => Some(ChangeArgs::Blank),
                _ => None,
            };

            if let Some(args) = args
                && let Some(state_func) = cr.get_state_change_func()
            {
                let err = game_state
                    .try_update(|gs| state_func.call(gs, args))
                    .unwrap();
                if let Err(err) = err {
                    info!(?change_type, "ChangeType");
                    info!(?cr, "cr");
                    error!(?err, "Error");
                    return (true, applied_cr);
                }
                applied_cr = true;
            }
            temp_state.update(|ts| ts.clear_selected());

            let cr = game_state
                .try_update(|gs| gs.change_request_queue.pop_front())
                .unwrap();
            if let Some(cr) = cr {
                let cr = cr.build();
                let change_type = cr.get_change_type();
                temp_state.curr_change_request().set(Some(cr));
                if matches!(change_type, ChangeType::NoStoryteller) {
                    continue;
                }
                return (true, applied_cr);
            }
            break;
        }

        info!(?temp_state, "Temp State");
        break;
    }

    (false, applied_cr)
}

pub(crate) fn next_button(game_state: Store<State>, temp_state: Store<TempState>) {
    let (ret, mut applied_cr) = apply_cr(game_state, temp_state);
    if ret {
        return;
    }

    let mut currently_acting_player = temp_state.currently_acting_player().get();
    loop {
        if currently_acting_player.is_none() {
            let next_player = game_state.read().get_next_active_player(None);
            currently_acting_player = next_player;
            if let Some(next_player) = next_player {
                game_state.update(|gs| gs.resolve(next_player));
                if let Some(cr) = game_state
                    .change_request_queue()
                    .try_update(|queue| queue.pop_front())
                    .unwrap()
                {
                    let cr = cr.build();
                    let change_type = cr.get_change_type();
                    temp_state.curr_change_request().set(Some(cr));
                    temp_state.currently_acting_player().set(Some(next_player));

                    if change_type == ChangeType::NoStoryteller {
                        let (ret, app_cr) = apply_cr(game_state, temp_state);
                        applied_cr = app_cr;
                        if ret {
                            return;
                        }
                        continue;
                    }
                    return;
                }
            }
        }

        while game_state.read().change_request_queue.is_empty()
            && let Some(acting_player) = currently_acting_player
        {
            let next_player = game_state
                .read()
                .get_next_active_player(Some(acting_player));

            info!(?next_player, "Next Player is");

            if let Some(next_player) = next_player {
                game_state.update(|gs| gs.resolve(next_player));
                if let Some(cr) = game_state
                    .try_update(|gs| gs.change_request_queue.pop_front())
                    .unwrap()
                {
                    let cr = cr.build();
                    let change_type = cr.get_change_type();
                    temp_state.curr_change_request().set(Some(cr));
                    temp_state.currently_acting_player().set(Some(next_player));

                    if change_type == ChangeType::NoStoryteller {
                        let (ret, app_cr) = apply_cr(game_state, temp_state);
                        applied_cr = app_cr;
                        if ret {
                            return;
                        }
                        continue;
                    }
                    return;
                }
            }
            currently_acting_player = next_player;
        }

        temp_state.update(|ts| ts.reset());
        if game_state.read().step == Step::Day && applied_cr {
            return;
        }
        info!(?applied_cr, "Applied cr");
        game_state.update(|gs| gs.next_step());
        if matches!(game_state.read().step, Step::Day) {
            return;
        }
    }
}
