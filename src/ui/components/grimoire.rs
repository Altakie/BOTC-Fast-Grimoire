use leptos::prelude::*;
use reactive_stores::Store;
use tracing::info;

use crate::engine::{
    change_request::ChangeType,
    player::Alignment,
    state::{State, StateStoreFields},
};
use crate::ui::game::{TempState, TempStateStoreFields};
use crate::ui::utils::layout::calc_circle;

#[component]
pub(crate) fn GrimoireStage(children: Children) -> impl IntoView {
    view! {
        <div class="relative flex-[2] flex justify-center items-center bg-stage">
            {children()}
        </div>
    }
}

#[component]
pub(crate) fn Grimoire() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let players = game_state.players();
    let player_positions = calc_circle(players.read_untracked().len(), 75.0);

    let temp_state = expect_context::<Store<TempState>>();
    let currently_selected_player = temp_state.selected_player();
    let selected_players = temp_state.selected_players();

    view! {
        <div class="relative origin-bottom-right size-1/2 flex flex-wrap rounded-full justify-between items-between">
            <For
                each=move || players.get().into_iter().enumerate()
                key=|(i, _)| *i
                children=move |(i, _)| {
                    let pos = player_positions[i];
                    let player = Memo::new(move |_| players.get()[i].clone());
                    info!("New Signal Created");
                    let selected = move || temp_state.selected_players().get().contains(&i);

                    view! {
                        <div
                            class="translate-1/2 absolute size-fit"
                            style:right=move || { format!("calc(50% + {}%)", pos.0) }
                            style:top=move || format!("calc(35% + {}%)", pos.1)
                        >
                            <p class="absolute left-1/2 -translate-x-1/2 bottom-3/5 rounded px-1.5 py-0.5 text-xs font-medium whitespace-nowrap border-solid border text-center bg-nameplate text-black shadow-sm">
                                {player.get().name}
                            </p>
                            <button
                                class="size-[5rem] rounded-full text-center border bg-panel shadow-md flex flex-col items-center justify-center px-1 text-xs leading-tight"
                                disabled=move || {
                                    if let Some(cr) = temp_state.curr_change_request().get() && let Some(filter_func) = cr.get_filter_func() {
                                            return !filter_func.call(i, &player.read());
                                        }

                                    false
                                }
                                style:border-style=move || {
                                    if !player.get().ghost_vote {
                                        "dashed"
                                    } else {
                                        "solid"
                                    }
                                }
                                style:border-color=move || {
                                    if selected() {
                                        "var(--color-accent)"
                                    } else {
                                        "var(--color-seat)"
                                    }
                                }
                                style:border-width=move || {
                                    if selected() {
                                        "3px"
                                    } else {
                                        "1px"
                                    }
                                }
                                style:background=move || {
                                    if let Some(selected_player) = temp_state
                                        .currently_acting_player()
                                        .get() && selected_player == i {
                                            return "var(--color-active-turn)";
                                        }

                                    ""
                                }
                                style:color=move || {
                                    if player.get().dead {
                                        return "var(--color-faint)";
                                    }
                                    match player.get().alignment {
                                        Alignment::Good => "var(--color-good)",
                                        Alignment::Evil => "var(--color-evil)",
                                        Alignment::Any => "var(--color-any)",
                                    }
                                }
                                on:keypress=move |ev| {
                                    ev.prevent_default();
                                }
                                on:click=move |_| {
                                    if temp_state.curr_change_request().read().is_none() {
                                        currently_selected_player.set(Some(i));
                                        temp_state.pending_execution().set(None);
                                        return;
                                    }
                                    let cr = temp_state.curr_change_request().get().unwrap();
                                    let requested_num = match cr.get_change_type() {
                                        ChangeType::ChoosePlayers(num) => num,
                                        _ => {
                                            currently_selected_player.set(Some(i));
                                            return;
                                        }
                                    };
                                    if selected() {
                                        selected_players
                                            .update(|pv| {
                                                let remove_index = pv
                                                    .iter()
                                                    .position(|pi| *pi == i)
                                                    .unwrap();
                                                pv.remove(remove_index);
                                            });
                                        return;
                                    }
                                    if selected_players.read().len() >= requested_num {
                                        return;
                                    }
                                    selected_players.update(|pv| pv.push(i));
                                }
                            >
                                {move || { player.get().role.to_string() }}
                            </button>
                            <div class="flex flex-row flex-wrap justify-center items-start gap-1 absolute w-max max-w-[8rem] left-1/2 -translate-x-1/2 top-9/10">
                                {move || {
                                    let status_effects = game_state
                                        .with(|gs| gs.get_player(i).status_effects.clone());
                                    status_effects
                                        .iter()
                                        .map(|status_effect| {
                                            let str = status_effect.status_type.to_string();
                                            view! {
                                                <p class="text-[0.6rem] leading-tight whitespace-nowrap text-center rounded-full px-2 py-0.5 bg-status-badge text-black shadow-sm">
                                                    {str}
                                                </p>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </div>
                        </div>
                    }
                }
            />
        </div>
    }
}
