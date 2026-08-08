use leptos::prelude::*;
use reactive_stores::Store;

use crate::engine::state::log::Event;
use crate::engine::state::{PlayerIndex, State, StateStoreFields, Step};
use crate::ui::game::{TempState, TempStateStoreFields};

#[component]
pub(crate) fn LeftRail(children: Children) -> impl IntoView {
    view! {
        <div class="flex-1 flex flex-col border-r border-solid border-divider overflow-y-auto min-w-[16rem]">
            {children()}
        </div>
    }
}

#[component]
pub(crate) fn LogPanel() -> impl IntoView {
    let state = expect_context::<Store<State>>();
    let log_modal_open: RwSignal<bool> = RwSignal::new(false);

    view! {
        <div class="border-t border-solid border-divider p-2 text-xs">
            <div class="flex items-center justify-between mb-1">
                <h2 class="font-mono uppercase tracking-widest text-[0.65rem] text-muted">"Log"</h2>
                <button
                    class="text-[0.65rem] text-muted hover:text-ink underline"
                    on:click=move |_| log_modal_open.set(true)
                >
                    "Expand"
                </button>
            </div>
            <LogModal open=log_modal_open />
            <div>
            <For
                each= move|| state.log().get().day_phases.into_iter().enumerate()
                key= |(i, _day_phase)| *i
                children=move|(i,_)| {
                    let day_phase = Memo::new(move |_| state.log().get().day_phases[i].clone());
                view! {
                <div class="border">
                    <h3>{format!("{:?} {}", day_phase.get().day_phase, day_phase.get().day_num) }</h3>
                    <For
                        each=move|| day_phase.get().log.into_iter().enumerate()
                        key=|(index, _)| *index
                        children=move |(_, event)| {
                            view! {
                                <p>{state.read().describe_event(event.event)}</p>
                            }
                        }
                    />
                </div>
                }
                }
            />
        </div>
        </div>
    }
}

#[component]
pub(crate) fn LogModal(open: RwSignal<bool>) -> impl IntoView {
    let state = expect_context::<Store<State>>();
    let deaths_only: RwSignal<bool> = RwSignal::new(false);

    view! {
        <Show when=move || open.get()>
            <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50 p-8">
                <div class="bg-chassis border border-solid border-divider rounded max-w-2xl w-full max-h-[80vh] flex flex-col">
                    <div class="flex items-center justify-between p-3 border-b border-solid border-divider">
                        <h2 class="font-mono uppercase tracking-widest text-sm text-ink">"Full Log"</h2>
                        <div class="flex items-center gap-3">
                            <label class="flex items-center gap-1 text-xs text-muted">
                                <input
                                    type="checkbox"
                                    prop:checked=move || deaths_only.get()
                                    on:change=move |ev| deaths_only.set(event_target_checked(&ev))
                                />
                                "DEATHS only"
                            </label>
                            <button
                                class="px-2 py-1 rounded border border-solid border-input hover:bg-panel-hover text-xs"
                                on:click=move |_| open.set(false)
                            >
                                "Close"
                            </button>
                        </div>
                    </div>
                    <div class="overflow-y-auto p-3 flex flex-col gap-3 text-xs">
                        <For
                            each=move || state.log().get().day_phases.into_iter().enumerate()
                            key=|(i, _)| *i
                            children=move |(i, _)| {
                                let day_phase = Memo::new(move |_| state.log().get().day_phases[i].clone());
                                view! {
                                    <div>
                                        <h3 class="font-mono uppercase tracking-widest text-[0.65rem] text-muted mb-1">
                                            {move || format!("{:?} {}", day_phase.get().day_phase, day_phase.get().day_num)}
                                        </h3>
                                        <div class="flex flex-col gap-0.5">
                                            <For
                                                each=move || {
                                                    day_phase
                                                        .get()
                                                        .log
                                                        .into_iter()
                                                        .enumerate()
                                                        .filter(|(_, le)| {
                                                            !deaths_only.get()
                                                                || matches!(le.event, Event::Death(_) | Event::Execution(_))
                                                        })
                                                        .collect::<Vec<_>>()
                                                }
                                                key=|(index, _)| *index
                                                children=move |(_, event)| {
                                                    view! { <p class="text-ink">{state.read().describe_event(event.event)}</p> }
                                                }
                                            />
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub(crate) fn StepList() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();
    let viewing_step: RwSignal<Option<PlayerIndex>> = RwSignal::new(None);

    let rows = move || {
        if !matches!(game_state.step().get(), Step::Setup | Step::NightOne | Step::Night) {
            return Vec::new();
        }
        let mut order: Vec<PlayerIndex> = Vec::new();
        let mut prev = None;
        loop {
            let next = game_state.read().get_next_active_player(prev);
            match next {
                Some(pi) => {
                    order.push(pi);
                    prev = Some(pi);
                }
                None => break,
            }
        }
        let acting = temp_state.currently_acting_player().get();
        let acting_pos = acting.and_then(|a| order.iter().position(|pi| *pi == a));
        order
            .into_iter()
            .enumerate()
            .map(|(idx, pi)| {
                let status = match acting_pos {
                    Some(ap) if idx < ap => "done",
                    Some(ap) if idx == ap => "current",
                    _ => "upcoming",
                };
                (pi, status)
            })
            .collect::<Vec<_>>()
    };

    let viewed_events = move || {
        let Some(target) = viewing_step.get() else {
            return Vec::new();
        };
        let day_phases = game_state.log().get().day_phases;
        let latest = match day_phases.last() {
            Some(l) => l.clone(),
            None => return Vec::new(),
        };
        latest
            .log
            .iter()
            .filter(|le| le.actor == Some(target))
            .map(|le| game_state.read().describe_event(le.event.clone()))
            .collect::<Vec<_>>()
    };

    view! {
        <div class="p-2 text-xs border-b border-solid border-divider">
            <h2 class="font-mono uppercase tracking-widest text-[0.65rem] text-muted mb-1">"Order"</h2>
            <div class="flex flex-col gap-1">
                <For
                    each=rows
                    key=|(pi, status)| (*pi, status.to_string())
                    children=move |(pi, status)| {
                        let role = game_state.read().get_player(pi).role.to_string();
                        let name = game_state.read().get_player(pi).name.clone();
                        let clickable = status == "done";
                        view! {
                            <div
                                class="flex justify-between px-2 py-1 rounded"
                                style:cursor=move || if clickable { "pointer" } else { "default" }
                                style:opacity=move || if status == "done" { "0.5" } else { "1" }
                                style:background=move || if status == "current" { "rgba(255,255,255,0.08)" } else { "transparent" }
                                on:click=move |_| {
                                    if !clickable {
                                        return;
                                    }
                                    if viewing_step.get() == Some(pi) {
                                        viewing_step.set(None);
                                    } else {
                                        viewing_step.set(Some(pi));
                                    }
                                }
                            >
                                <span>{role}</span>
                                <span class="text-faint">{name}</span>
                            </div>
                        }
                    }
                />
            </div>
            <Show when=move || viewing_step.get().is_some()>
                <div class="mt-2 p-2 border border-solid border-divider rounded flex flex-col gap-1">
                    <For
                        each=viewed_events
                        key=|s| s.clone()
                        children=move |text| view! { <p>{text}</p> }
                    />
                </div>
            </Show>
        </div>
    }
}
