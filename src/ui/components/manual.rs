use leptos::prelude::*;
use reactive_stores::Store;

use crate::engine::{
    player::{Alignment, Player, roles::RoleNames},
    state::{
        PlayerIndex, State, StateStoreFields, Step,
        status_effects::{CleanupPhase, StatusEffect, StatusType},
    },
};
use crate::initialization::Script;
use crate::ui::game_logic::{set_player_dead, set_player_ghost_vote};

fn selectable_roles() -> Vec<RoleNames> {
    vec![
        RoleNames::Investigator,
        RoleNames::Empath,
        RoleNames::Washerwoman,
        RoleNames::Librarian,
        RoleNames::Chef,
        RoleNames::Fortuneteller,
        RoleNames::Undertaker,
        RoleNames::Virgin,
        RoleNames::Soldier,
        RoleNames::Slayer,
        RoleNames::Mayor,
        RoleNames::Monk,
        RoleNames::Ravenkeeper,
        RoleNames::Drunk,
        RoleNames::Saint,
        RoleNames::Butler,
        RoleNames::Recluse,
        RoleNames::Spy,
        RoleNames::Baron,
        RoleNames::ScarletWoman,
        RoleNames::Poisoner,
        RoleNames::Imp,
    ]
}

fn selectable_status_types() -> Vec<StatusType> {
    vec![
        StatusType::Poisoned,
        StatusType::Drunk,
        StatusType::WasherwomanTownsfolk,
        StatusType::WasherwomanWrong,
        StatusType::LibrarianOutsider,
        StatusType::LibrarianWrong,
        StatusType::InvestigatorMinion,
        StatusType::InvestigatorWrong,
        StatusType::ButlerMaster,
        StatusType::FortuneTellerRedHerring,
        StatusType::DemonProtected,
    ]
}

// -- Field-editor primitives (reused everywhere that value type appears) --

#[component]
pub(crate) fn BoolToggle(
    #[prop(into)] value: Signal<bool>,
    on_change: Callback<bool>,
) -> impl IntoView {
    view! {
        <button
            class="px-2 py-1 rounded border border-solid border-input"
            on:click=move |_| {
                on_change.run(!value.get());
            }
        >
            {move || if value.get() { "Yes" } else { "No" }}
        </button>
    }
}

#[component]
pub(crate) fn NumberField<T>(
    #[prop(into)] value: Signal<T>,
    on_change: Callback<T>,
    #[prop(default = "w-24".to_string(), into)] class: String,
) -> impl IntoView
where
    T: ToString + std::str::FromStr + Clone + Send + Sync + 'static,
{
    let error: RwSignal<Option<String>> = RwSignal::new(None);
    view! {
        <div class="flex flex-col gap-1">
            <input
                type="text"
                inputmode="numeric"
                class=move || format!("bg-panel border border-solid border-input rounded px-2 py-1 {}", class)
                prop:value=move || value.get().to_string()
                on:change=move |ev| {
                    let raw = event_target_value(&ev);
                    match raw.parse::<T>() {
                        Ok(v) => {
                            error.set(None);
                            on_change.run(v);
                        }
                        Err(_) => {
                            error.set(Some("Must be a whole number, 0 or greater".to_string()));
                        }
                    }
                }
            />
            <Show when=move || error.get().is_some()>
                <p class="text-evil text-xs">{move || error.get().unwrap_or_default()}</p>
            </Show>
        </div>
    }
}

#[component]
pub(crate) fn StringField(
    #[prop(into)] value: Signal<String>,
    on_change: Callback<String>,
    #[prop(default = "flex-1".to_string(), into)] class: String,
) -> impl IntoView {
    view! {
        <input
            type="text"
            class=move || format!("bg-panel border border-solid border-input rounded px-2 py-1 {}", class)
            prop:value=value
            on:change=move |ev| {
                on_change.run(event_target_value(&ev));
            }
        />
    }
}

/// Generic enum-valued dropdown. `label` is a required mapping function rather than relying on
/// `Display`/`Debug`, since sibling types in this codebase disagree on which one matches the
/// string form used for matching/round-tripping (e.g. `RoleNames`'s `Display` diverges from its
/// derived `Debug` for `ScarletWoman`/`"Scarletwoman"`).
#[component]
pub(crate) fn EnumDropdown<T>(
    #[prop(into)] value: Signal<T>,
    options: Vec<T>,
    label: Callback<T, String>,
    on_change: Callback<T>,
    #[prop(optional, into)] color: Option<Callback<T, String>>,
) -> impl IntoView
where
    T: PartialEq + Clone + Send + Sync + 'static,
{
    let options_for_lookup = options.clone();
    view! {
        <select
            class="bg-panel border border-solid border-input rounded px-2 py-1"
            style:color=move || {
                color.as_ref().map(|f| f.run(value.get())).unwrap_or_default()
            }
            on:change=move |ev| {
                let raw = event_target_value(&ev);
                if let Some(found) = options_for_lookup.iter().find(|o| label.run((*o).clone()) == raw) {
                    on_change.run(found.clone());
                }
            }
        >
            {options.iter().map(|o| {
                let text = label.run(o.clone());
                let text_for_selected = text.clone();
                view! {
                    <option
                        value=text.clone()
                        prop:selected=move || label.run(value.get()) == text_for_selected
                    >
                        {text.clone()}
                    </option>
                }
            }).collect_view()}
        </select>
    }
}

#[component]
pub(crate) fn RoleDropdown(
    #[prop(into)] value: Signal<RoleNames>,
    on_change: Callback<RoleNames>,
) -> impl IntoView {
    view! {
        <EnumDropdown
            value=value
            options=selectable_roles()
            label=Callback::new(|r: RoleNames| r.to_string())
            on_change=on_change
        />
    }
}

#[component]
pub(crate) fn AlignmentDropdown(
    #[prop(into)] value: Signal<Alignment>,
    on_change: Callback<Alignment>,
) -> impl IntoView {
    view! {
        <EnumDropdown
            value=value
            options=vec![Alignment::Good, Alignment::Evil, Alignment::Any]
            label=Callback::new(|a: Alignment| a.to_string())
            on_change=on_change
            color=Callback::new(|a: Alignment| match a {
                Alignment::Good => "var(--color-good)".to_string(),
                Alignment::Evil => "var(--color-evil)".to_string(),
                Alignment::Any => "var(--color-any)".to_string(),
            })
        />
    }
}

// One shared reusable wrapper — genuinely used many times below with different content.
#[component]
pub(crate) fn Collapsible(
    #[prop(into)] title: Signal<String>,
    #[prop(default = false)] default_open: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let open = RwSignal::new(default_open);
    view! {
        <div class="border border-solid border-divider rounded">
            <button
                class="w-full flex items-center justify-between px-3 py-2 text-left font-mono text-xs uppercase tracking-widest text-ink"
                on:click=move |_| open.update(|o| *o = !*o)
            >
                <span>{title}</span>
                <span>{move || if open.get() { "\u{25BE}" } else { "\u{25B8}" }}</span>
            </button>
            <Show when=move || open.get()>
                <div class="p-3 border-t border-solid border-divider flex flex-col gap-2">
                    {children()}
                </div>
            </Show>
        </div>
    }
}

#[component]
pub(crate) fn ManualModeEditor() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let script = expect_context::<RwSignal<Script>>();

    let player_count = move || game_state.players().get().len();
    let role_count = move || script.get().roles.len();

    let step_options = vec![
        Step::Start,
        Step::Setup,
        Step::Day,
        Step::NightOne,
        Step::Night,
    ];

    view! {
        <div class="flex-1 overflow-y-auto p-4 flex flex-col gap-4 text-sm bg-chassis text-ink">
            <div class="border border-solid border-divider rounded p-3 flex flex-col gap-3">
                <h2 class="font-mono uppercase tracking-widest text-xs text-muted">"Game"</h2>

                <div class="flex items-center gap-2">
                    <span class="w-24 text-muted">"Day Num"</span>
                    <NumberField
                        value=Signal::derive(move || game_state.day_num().get())
                        on_change=Callback::new(move |v| game_state.day_num().set(v))
                    />
                </div>

                <div class="flex items-center gap-2">
                    <span class="w-24 text-muted">"Step"</span>
                    <EnumDropdown
                        value=Signal::derive(move || game_state.step().get())
                        options=step_options.clone()
                        label=Callback::new(|s: Step| format!("{:?}", s))
                        on_change=Callback::new(move |s| game_state.step().set(s))
                    />
                </div>

                <p class="text-faint text-xs">
                    {move || format!("{} pending change requests (read-only)", game_state.change_request_queue().get().len())}
                </p>
                <p class="text-faint text-xs">
                    {move || format!("{} nomination listeners (read-only)", game_state.nomination_listeners().get().len())}
                </p>
                <p class="text-faint text-xs">
                    {move || format!("{} attempted kill listeners (read-only)", game_state.attempted_kill_listeners().get().len())}
                </p>
                <p class="text-faint text-xs">
                    {move || format!("{} death listeners (read-only)", game_state.death_listeners().get().len())}
                </p>
            </div>

            <Collapsible title=Signal::derive(move || format!("Players ({})", player_count())) default_open=true>
                <For
                    each=move || game_state.players().get().into_iter().enumerate()
                    key=|(i, _)| *i
                    children=move |(i, _)| view! { <PlayerEditor player_index=i /> }
                />
            </Collapsible>

            <Collapsible title=Signal::derive(move || format!("Script Roles ({})", role_count())) default_open=false>
                <ScriptRolesEditor />
            </Collapsible>
        </div>
    }
}

#[component]
fn ScriptRolesEditor() -> impl IntoView {
    let script = expect_context::<RwSignal<Script>>();
    let new_role: RwSignal<RoleNames> = RwSignal::new(RoleNames::Investigator);

    view! {
        <div class="flex flex-col gap-2">
            <For
                each=move || script.get().roles.into_iter().enumerate()
                key=|(i, r)| (*i, *r)
                children=move |(i, role)| {
                    view! {
                        <div class="flex items-center justify-between gap-2 text-xs">
                            <span>{role.to_string()}</span>
                            <button
                                class="px-2 py-0.5 rounded border border-solid border-input"
                                on:click=move |_| {
                                    script.update(|s| {
                                        s.roles.remove(i);
                                    });
                                }
                            >
                                "Remove"
                            </button>
                        </div>
                    }
                }
            />

            <div class="flex items-center gap-2">
                <RoleDropdown
                    value=Signal::derive(move || new_role.get())
                    on_change=Callback::new(move |r| new_role.set(r))
                />
                <button
                    class="px-2 py-1 rounded bg-accent text-white text-xs"
                    on:click=move |_| {
                        script.update(|s| s.roles.push(new_role.get()));
                    }
                >
                    "+ Add"
                </button>
            </div>
        </div>
    }
}

#[component]
fn PlayerEditor(player_index: PlayerIndex) -> impl IntoView {
    let game_state = expect_context::<Store<State>>();

    view! {
        <Collapsible title=Signal::derive(move || game_state.players().read()[player_index].name.clone()) default_open=false>
            <div class="flex items-center gap-2">
                <span class="w-24 text-muted">"Name"</span>
                <StringField
                    value=Signal::derive(move || game_state.players().read()[player_index].name.clone())
                    on_change=Callback::new(move |raw: String| {
                        game_state.players().update(|players: &mut Vec<Player>| {
                            players[player_index].name = raw;
                        });
                    })
                />
            </div>

            <div class="flex items-center gap-2">
                <span class="w-24 text-muted">"Role"</span>
                <RoleDropdown
                    value=Signal::derive(move || {
                        let label = game_state.players().read()[player_index].role.to_role_name();
                        selectable_roles().into_iter().find(|r| *r == label).unwrap_or(RoleNames::Investigator)
                    })
                    on_change=Callback::new(move |role_name: RoleNames| {
                        let default_alignment = role_name.get_default_alignment();
                        game_state.players().update(|players: &mut Vec<Player>| {
                            players[player_index].role = role_name.convert();
                            players[player_index].alignment = default_alignment;
                        });
                    })
                />
            </div>

            <div class="flex items-center gap-2">
                <span class="w-24 text-muted">"Alignment"</span>
                <AlignmentDropdown
                    value=Signal::derive(move || game_state.players().read()[player_index].alignment)
                    on_change=Callback::new(move |alignment: Alignment| {
                        game_state.players().update(|players: &mut Vec<Player>| {
                            players[player_index].alignment = alignment;
                        });
                    })
                />
            </div>

            <div class="flex items-center gap-2">
                <span class="w-24 text-muted">"Dead"</span>
                <BoolToggle
                    value=Signal::derive(move || game_state.players().read()[player_index].dead)
                    on_change=Callback::new(move |dead: bool| {
                        set_player_dead(game_state, player_index, dead);
                    })
                />
            </div>

            <div class="flex items-center gap-2">
                <span class="w-24 text-muted">"Ghost Vote"</span>
                <BoolToggle
                    value=Signal::derive(move || game_state.players().read()[player_index].ghost_vote)
                    on_change=Callback::new(move |ghost_vote: bool| {
                        set_player_ghost_vote(game_state, player_index, ghost_vote);
                    })
                />
            </div>

            <Collapsible title=Signal::derive(|| "Status Effects".to_string()) default_open=false>
                <StatusEffectsEditor player_index=player_index />
            </Collapsible>
        </Collapsible>
    }
}

#[component]
fn StatusEffectsEditor(player_index: PlayerIndex) -> impl IntoView {
    let game_state = expect_context::<Store<State>>();

    let new_status_type: RwSignal<StatusType> = RwSignal::new(StatusType::Poisoned);
    let new_source: RwSignal<PlayerIndex> = RwSignal::new(player_index);
    let new_cleanup: RwSignal<Option<CleanupPhase>> = RwSignal::new(None);

    let cleanup_options: Vec<Option<CleanupPhase>> =
        vec![None, Some(CleanupPhase::Dusk), Some(CleanupPhase::Dawn)];
    let cleanup_label = |cp: Option<CleanupPhase>| match cp {
        None => "None".to_string(),
        Some(CleanupPhase::Dusk) => "Dusk".to_string(),
        Some(CleanupPhase::Dawn) => "Dawn".to_string(),
    };

    view! {
        <div class="flex flex-col gap-2">
            <For
                each=move || game_state.players().read()[player_index].status_effects.clone().into_iter().enumerate()
                key=|(i, se)| (*i, se.status_type)
                children=move |(_, se): (usize, StatusEffect)| {
                    let source_name = move || {
                        game_state.players().read()[se.source_player_index].name.clone()
                    };
                    let status_label = se.status_type.to_string();
                    let status_name_for_removal = se.status_type.name();
                    view! {
                        <div class="flex items-center justify-between gap-2 text-xs">
                            <span>{status_label}" from "{source_name}</span>
                            <button
                                class="px-2 py-0.5 rounded border border-solid border-input"
                                on:click=move |_| {
                                    let name = status_name_for_removal.clone();
                                    game_state.players().update(|players: &mut Vec<Player>| {
                                        players[player_index].remove_status(&name);
                                    });
                                }
                            >
                                "Remove"
                            </button>
                        </div>
                    }
                }
            />

            <div class="flex items-center gap-2 flex-wrap">
                <EnumDropdown
                    value=Signal::derive(move || new_status_type.get())
                    options=selectable_status_types()
                    label=Callback::new(|st: StatusType| st.to_string())
                    on_change=Callback::new(move |st| new_status_type.set(st))
                />

                <select
                    class="bg-panel border border-solid border-input rounded px-2 py-1 text-xs"
                    prop:value=move || game_state.players().read()[new_source.get()].name.clone()
                    on:change=move |ev| {
                        let raw = event_target_value(&ev);
                        let players = game_state.players().get();
                        if let Some(idx) = players.iter().position(|p| p.name == raw) {
                            new_source.set(idx);
                        }
                    }
                >
                    {move || game_state.players().get().into_iter().map(|p| {
                        view! { <option value=p.name.clone()>{p.name.clone()}</option> }
                    }).collect_view()}
                </select>

                <EnumDropdown
                    value=Signal::derive(move || new_cleanup.get())
                    options=cleanup_options.clone()
                    label=Callback::new(cleanup_label)
                    on_change=Callback::new(move |cp| new_cleanup.set(cp))
                />

                <button
                    class="px-2 py-1 rounded bg-accent text-white text-xs"
                    on:click=move |_| {
                        let effect = StatusEffect::new(new_status_type.get(), new_source.get(), new_cleanup.get());
                        game_state.players().update(|players: &mut Vec<Player>| {
                            players[player_index].add_status(effect);
                        });
                    }
                >
                    "+ Add"
                </button>
            </div>
        </div>
    }
}
