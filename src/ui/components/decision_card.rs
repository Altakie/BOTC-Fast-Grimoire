use leptos::prelude::*;
use reactive_stores::Store;

use crate::ui::components::primitives::{ButtonSize, PrimaryButton, SecondaryButton};

use crate::engine::{
    change_request::{
        ChangeRequest, ChangeRequestBuilder, ChangeType, StateChangeFuncPtr, check_len,
    },
    state::{PlayerIndex, State, StateStoreFields, Step},
};
use crate::initialization::Script;
use crate::ui::game::{TempState, TempStateStoreFields, UiMode};
use crate::ui::game_logic::{next_button, set_player_dead, set_player_ghost_vote};

#[component]
pub(crate) fn DecisionCard(children: Children) -> impl IntoView {
    view! {
        <div class="flex-1 flex flex-col border-l border-solid border-divider p-4 overflow-y-auto min-w-[20rem]">
            {children()}
        </div>
    }
}

#[component]
fn DecisionCardHeader(#[prop(into)] title: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <div>
            <h2 class="font-mono uppercase tracking-widest text-xs text-muted">{title}</h2>
            {children()}
        </div>
    }
}

#[component]
pub(crate) fn AutoResolveDecisionCard() -> impl IntoView {
    let state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();

    let title = move || format!("{:?}", state.step().get());
    let prompt = move || {
        temp_state
            .curr_change_request()
            .get()
            .map(|cr| cr.get_description())
            .unwrap_or_else(|| "Waiting...".to_string())
    };

    let display = move || {
        if temp_state
            .curr_change_request()
            .get()
            .is_some_and(|cr| matches!(cr.get_change_type(), ChangeType::ChooseRoles(_)))
        {
            return RoleSelector().into_any();
        }

        if matches!(state.step().get(), Step::Day)
            && temp_state.curr_change_request().get().is_none()
        {
            return DayAbilitySelector().into_any();
        }

        ().into_any()
    };

    view! {
        <div class="flex flex-col gap-4 h-full">
            <DecisionCardHeader title=Signal::derive(title)>
                <p class="text-sm">{prompt}</p>
            </DecisionCardHeader>
            <div>{display}</div>
            <NightPlayerDetail />
            <div class="mt-auto">
                <PrimaryButton full_width=true on_click=Callback::new(move |_| next_button(state, temp_state))>
                    "Next"
                </PrimaryButton>
            </div>
        </div>
    }
}

#[component]
pub(crate) fn NightPlayerDetail() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();

    let current_player_info = move || {
        let player_index = temp_state.currently_acting_player().get();
        let player_index = match player_index {
            Some(pi) => pi,
            None => {
                return ().into_any();
            }
        };
        let player = game_state.players().read()[player_index].clone();

        return view! {
            <div class="border border-solid border-divider rounded w-full p-3 flex flex-col gap-1 text-sm">
                <h3 class="font-mono uppercase tracking-widest text-xs text-muted">"Current Player"</h3>
                <p class="font-medium">{player.name}</p>
                <p class="text-muted">"Role: "<span class="text-ink">{player.role.to_string()}</span></p>
                <p class="text-muted flex items-center gap-2">
                    "Status: "<span class="text-ink">{if player.dead { "Dead" } else { "Alive" }}</span>
                    <SecondaryButton size=ButtonSize::Sm on_click=Callback::new(move |_| {
                        set_player_dead(game_state, player_index, !player.dead);
                    })>
                        "Toggle"
                    </SecondaryButton>
                </p>
                <p class="text-muted">"Ghost Vote: "<span class="text-ink">{if player.dead { "Yes" } else { "No" }}</span></p>
                <p class="text-muted">"Alignment: "<span class="text-ink">{player.alignment.to_string()}</span></p>
            </div>
        }
        .into_any();
    };
    let selected_player_info = move || {
        let player_index = temp_state.selected_player().get();
        let player_index = match player_index {
            Some(pi) => pi,
            None => {
                return ().into_any();
            }
        };
        let player = game_state.players().read()[player_index].clone();

        return view! {
            <div class="border border-solid border-divider rounded w-full p-3 flex flex-col gap-1 text-sm">
                <h3 class="font-mono uppercase tracking-widest text-xs text-muted">"Selected Player"</h3>
                <p class="font-medium">{player.name}</p>
                <Show
                    when=move || temp_state.pending_execution().get() == Some(player_index)
                    fallback=move || {
                        view! {
                            <p class="text-muted flex items-center gap-2">
                                "Status: "<span class="text-ink">{if player.dead { "Dead" } else { "Alive" }}</span>
                                <SecondaryButton
                                    size=ButtonSize::Sm
                                    danger=true
                                    disabled=Signal::derive(move || !matches!(game_state.step().get(), Step::Day))
                                    on_click=Callback::new(move |_| {
                                        temp_state.pending_execution().set(Some(player_index));
                                    })
                                >
                                    "Execute"
                                </SecondaryButton>
                            </p>
                        }
                    }
                >
                    <ExecutionConfirm player_index=player_index />
                </Show>
                <p class="text-muted flex items-center gap-2">
                    "Ghost Vote: "<span class="text-ink">{if player.ghost_vote { "Yes" } else { "No" }}</span>
                    <SecondaryButton size=ButtonSize::Sm on_click=Callback::new(move |_| {
                        set_player_ghost_vote(game_state, player_index, !player.ghost_vote);
                    })>
                        "Toggle"
                    </SecondaryButton>
                </p>
                <p class="text-muted">"Alignment: "<span class="text-ink">{player.alignment.to_string()}</span></p>
            </div>
        }
        .into_any();
    };

    view! {
        <div class="flex flex-col gap-2">
            {selected_player_info}
            {current_player_info}
        </div>
    }
}

#[component]
fn ExecutionConfirm(player_index: PlayerIndex) -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();
    view! {
        <div class="p-2 border border-solid border-evil rounded flex flex-col gap-2 text-xs">
            <p class="text-evil">"Execute this player? This cannot be undone."</p>
            <div class="flex gap-2">
                <PrimaryButton danger=true on_click=Callback::new(move |_| {
                    game_state.update(|gs| gs.execute_player(player_index));
                    temp_state.pending_execution().set(None);
                })>
                    "Confirm Execute"
                </PrimaryButton>
                <SecondaryButton on_click=Callback::new(move |_| {
                    temp_state.pending_execution().set(None);
                })>
                    "Cancel"
                </SecondaryButton>
            </div>
        </div>
    }
}

#[component]
pub(crate) fn EndOfGameSummary() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();

    view! {
        <div class="flex flex-col gap-4 h-full">
            <DecisionCardHeader title=Signal::derive(|| "Game Over".to_string())>
                <p class={move || format!("text-lg font-medium text-{}", game_state.winner().get().map(|winner| match winner {
                    crate::engine::player::Alignment::Good => "good",
                    crate::engine::player::Alignment::Evil => "evil",
                    crate::engine::player::Alignment::Any => "any",
                }).unwrap_or("white")) }>{move || format!("{} Wins", game_state.winner().get().map(|alignment| alignment.to_string()).unwrap_or("No One".to_string()))}</p>
            </DecisionCardHeader>
            <div class="border border-solid border-divider rounded p-3 flex flex-col gap-2">
                <h3 class="font-mono uppercase tracking-widest text-xs text-muted">"Final Roster"</h3>
                <For
                    each=move || game_state.players().get().into_iter().enumerate()
                    key=|(i, _)| *i
                    children=move |(_, player)| {
                        let dead = player.dead;
                        view! {
                            <div class="flex items-center justify-between text-sm gap-4">
                                <span class="font-medium">{player.name.clone()}</span>
                                <span class="text-muted">{player.role.to_string()}</span>
                                <span class=if dead { "text-faint" } else { "text-ink" }>
                                    {if dead { "Dead" } else { "Alive" }}
                                </span>
                            </div>
                        }
                    }
                />
            </div>
            <div class="mt-auto flex flex-col gap-2">
                <p class="text-xs text-muted">
                    "The log and grimoire are still available on the left. If the game ended in error, use Resume to switch to Manual mode and correct the state (e.g. toggle the wrong player back to alive), then switch back to Auto."
                </p>
                <SecondaryButton full_width=true on_click=Callback::new(move |_| {
                    temp_state.ui_mode().set(UiMode::Manual);
                })>
                    "Resume Game (Manual Mode)"
                </SecondaryButton>
            </div>
        </div>
    }
}

#[component]
fn RoleSelector() -> impl IntoView {
    let script = expect_context::<RwSignal<Script>>();
    let temp_state = expect_context::<Store<TempState>>();
    view! {
        <div class="flex flex-col gap-1">
            {move || {
                script
                    .get()
                    .roles
                    .into_iter()
                    .map(move |role| {
                        let selected = RwSignal::new(false);
                        view! {
                            <button
                                class="text-left px-2 py-1 rounded border border-solid border-divider bg-panel text-ink text-sm hover:border-seat"
                                style:border-color=move || if selected.get() { "var(--color-accent)" } else { "" }
                                style:background=move || if selected.get() { "var(--color-selected-bg)" } else { "" }
                                on:click=move |_| {
                                    let cr = {
                                        if temp_state.curr_change_request().read().is_none() {
                                            return;
                                        }
                                        temp_state.curr_change_request().get().unwrap()
                                    };
                                    let requested_num = match cr.get_change_type() {
                                        ChangeType::ChooseRoles(num) => num,
                                        _ => {
                                            return;
                                        }
                                    };
                                    let selected_roles = temp_state.selected_roles();
                                    if selected.get() {
                                        selected_roles
                                            .update(|pv| {
                                                let remove_index = pv
                                                    .iter()
                                                    .position(|r| *r == role)
                                                    .unwrap();
                                                pv.remove(remove_index);
                                            });
                                        selected.set(false);
                                        return;
                                    }
                                    if selected_roles.read().len() >= requested_num {
                                        return;
                                    }
                                    selected_roles.update(|pv| pv.push(role));
                                    selected.set(true);
                                }
                            >
                                {role.to_string()}
                            </button>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

#[component]
fn DayAbilitySelector() -> impl IntoView {
    let state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();

    let nominate_button = move |_| {
        temp_state.update(|ts| ts.reset());
        let description = "Select the nominating player";
        let change_type = ChangeType::ChoosePlayers(1);

        let state_change_func = StateChangeFuncPtr::new(move |state, args| {
            let nominating_players = args.extract_player_indicies()?;
            check_len(&nominating_players, 1)?;

            let nominating_player = nominating_players[0];

            let description = "Select the nominated player";

            let state_change_func = StateChangeFuncPtr::new(move |state, args| {
                let target_players = args.extract_player_indicies()?;
                check_len(&target_players, 1)?;

                let nominated_player = target_players[0];
                state.nominate_player(nominating_player, nominated_player);
                Ok(())
            });

            state.change_request_queue.push_back(
                ChangeRequest::new_builder(change_type, description.into())
                    .state_change_func(state_change_func),
            );

            Ok(())
        });

        let nominate_request = ChangeRequest::new_builder(change_type, description.into())
            .state_change_func(state_change_func)
            .build();
        temp_state.curr_change_request().set(Some(nominate_request));
    };

    view! {
        <div class="flex flex-col gap-2">
            <SecondaryButton on_click=Callback::new(nominate_button)>
                "Nominate"
            </SecondaryButton>
            <div class="flex flex-col gap-1">
                {move || {
                    let active_players = state.read().get_day_active();
                    active_players
                        .into_iter()
                        .map(|player_index| {
                            let role = state.read().get_player(player_index).role.clone();

                            view! {
                                <button
                                    class="text-left px-2 py-1 rounded border border-solid border-divider bg-panel text-ink text-sm hover:border-seat"
                                    on:click=move |_| {
                                        temp_state.update(|ts| ts.reset());
                                        let player_ability = state
                                            .try_update(|gs| gs.resolve_day_ability(player_index))
                                            .unwrap();
                                        temp_state.curr_change_request().set(build(player_ability));
                                        temp_state.currently_acting_player().set(Some(player_index));
                                    }
                                >
                                    {move || { format!("{} Ability", role) }}
                                </button>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}

fn build(change_option: Option<ChangeRequestBuilder>) -> Option<ChangeRequest> {
    match change_option {
        Some(cr) => Some(cr.build()),
        None => None,
    }
}
