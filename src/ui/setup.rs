use leptos::prelude::*;
use crate::ui::InitializationStage;
use crate::initialization::{CharacterTypeCounts, Script, ScriptJson};
use crate::scripts::trouble_brewing;
use crate::engine::player::{CharacterType, roles::RoleNames};

#[component]
pub(crate) fn Starter(
    setup_stage: WriteSignal<InitializationStage>,
    next_setup_stage: InitializationStage,
) -> impl IntoView {
    view! {
        <div class="min-h-screen w-screen flex flex-col items-center justify-center gap-6 bg-chassis text-ink">
            <h1 class="font-mono uppercase tracking-widest text-2xl text-ink">
                "Blood on the Clocktower"
            </h1>
            <button
                class="px-6 py-3 rounded border border-solid border-input bg-panel hover:bg-panel-hover font-mono uppercase tracking-widest text-sm"
                on:click=move |_| { setup_stage.set(next_setup_stage) }
            >
                "Start Game"
            </button>
        </div>
    }
}

#[component]
pub(crate) fn PlayerInputer(
    players: RwSignal<Vec<String>>,
    setup_stage: WriteSignal<InitializationStage>,
    next_setup_stage: InitializationStage,
) -> impl IntoView {
    let name = RwSignal::new(String::new());

    view! {
        <div class="min-h-screen w-screen flex flex-col items-center justify-center gap-4 bg-chassis text-ink p-8">
            <h2 class="font-mono uppercase tracking-widest text-sm text-muted">"Players"</h2>
            <PlayerSetupList player_names=players />
            <p class="text-muted text-sm">"Input Player Name Below"</p>
            <input
                id="PlayerInput"
                type="text"
                class="bg-panel border border-solid border-input rounded px-2 py-1"
                bind:value=name
                on:keypress=move |ev| {
                    if ev.key() == "Enter" {
                        players.update(|pv| pv.push(name.get()));
                        name.set(String::from(""));
                    }
                }
            />
            <div class="flex items-center gap-2">
                <button
                    class="px-4 py-2 rounded border border-solid border-input bg-panel hover:bg-panel-hover"
                    on:click=move |_| {
                        players.update(|pv| pv.push(name.get()));
                        name.set(String::from(""));
                    }
                >
                    "Add Player"
                </button>
                <button
                    class="px-4 py-2 rounded border border-solid border-input bg-accent hover:bg-accent-hover disabled:opacity-40 disabled:bg-panel disabled:hover:bg-panel"
                    on:click=move |_| {
                        if players.get().len() >= 5 {
                            setup_stage.set(next_setup_stage);
                        }
                    }
                    disabled=move || { players.get().len() < 5 }
                >
                    "Finish"
                </button>
            </div>
        </div>
    }
}

#[component]
pub(crate) fn PlayerSetupList(player_names: RwSignal<Vec<String>>) -> impl IntoView {
    view! {
        <ol class="flex flex-col gap-1 min-w-64">
            <For
                each=move || player_names.get()
                key=|pn| pn.clone()
                children=move |player_name| {
                    let (player_name, _) = signal(player_name);
                    view! {
                        <li class="flex items-center justify-between gap-2 bg-panel border border-solid border-divider rounded px-3 py-1.5 text-sm">
                            <span>{player_name}</span>
                            <span class="flex items-center gap-1">
                                <button
                                    class="px-2 py-0.5 rounded border border-solid border-input hover:bg-panel-hover disabled:opacity-30"
                                    on:click=move |_| {
                                        let player_name = player_name.get();
                                        let i = player_names
                                            .read()
                                            .iter()
                                            .position(|p| *p == player_name)
                                            .unwrap();
                                        player_names
                                            .update(|pv| {
                                                if i == 0 {
                                                    return;
                                                }
                                                let temp = pv[i].clone();
                                                pv[i] = pv[i - 1].clone();
                                                pv[i - 1] = temp;
                                            })
                                    }
                                    disabled=move || {
                                        let i = player_names
                                            .read()
                                            .iter()
                                            .position(|p| *p == *player_name.read())
                                            .unwrap();
                                        i == 0
                                    }
                                >
                                    "Move Up"
                                </button>
                                <button
                                    class="px-2 py-0.5 rounded border border-solid border-input hover:bg-panel-hover disabled:opacity-30"
                                    on:click=move |_| {
                                        let i = player_names
                                            .read()
                                            .iter()
                                            .position(|p| *p == *player_name.read())
                                            .unwrap();
                                        player_names
                                            .update(|pv| {
                                                if (i + 1) >= pv.len() {
                                                    return;
                                                }
                                                let temp = pv[i].clone();
                                                pv[i] = pv[i + 1].clone();
                                                pv[i + 1] = temp;
                                            });
                                    }
                                    disabled=move || {
                                        let i = player_names
                                            .read()
                                            .iter()
                                            .position(|p| *p == *player_name.read())
                                            .unwrap();
                                        let len = player_names.read().len();
                                        (i + 1) >= len
                                    }
                                >
                                    "Move Down"
                                </button>
                                <button
                                    class="px-2 py-0.5 rounded border border-solid border-evil text-evil hover:bg-evil/10"
                                    on:click=move |_| {
                                        let i = player_names
                                            .read()
                                            .iter()
                                            .position(|p| *p == *player_name.read())
                                            .unwrap();
                                        player_names
                                            .update(|pv| {
                                                pv.remove(i);
                                            })
                                    }
                                >
                                    "X"
                                </button>
                            </span>
                        </li>
                    }
                }
            />
        </ol>
    }
}

#[component]
pub(crate) fn ScriptInputter(
    script: RwSignal<Script>,
    setup_stage: WriteSignal<InitializationStage>,
    next_setup_stage: InitializationStage,
) -> impl IntoView {
    let raw_json = RwSignal::new(String::new());
    view! {
        <div class="min-h-screen w-screen flex flex-col items-center justify-center gap-4 bg-chassis text-ink p-8">
            <h2 class="font-mono uppercase tracking-widest text-sm text-muted">"Choose a Script"</h2>
            <button
                class="block px-6 py-3 rounded border border-solid border-input bg-panel hover:bg-panel-hover font-mono uppercase tracking-widest text-sm"
                on:click=move |_| {
                    script.set(trouble_brewing());
                    setup_stage.set(next_setup_stage);
                }
            >
                "Trouble Brewing"
            </button>
            <p class="text-muted text-sm">"Input Custom Script Json Below"</p>
            <input
                type="text"
                class="bg-panel border border-solid border-input rounded px-2 py-1 w-96"
                bind:value=raw_json
            />
            <ErrorBoundary fallback=|_errors| ()>
                <button
                    class="px-4 py-2 rounded border border-solid border-input bg-accent hover:bg-accent-hover disabled:opacity-40 disabled:bg-panel disabled:hover:bg-panel"
                    on:click=move |_| {
                        let script_json = serde_json::from_str::<ScriptJson>(&raw_json.get());
                        raw_json.set(String::from(""));
                        let script_json = match script_json {
                            Ok(json) => json,
                            Err(_) => return,
                        };
                        script.set(Script::new_from_json(script_json));
                        setup_stage.set(next_setup_stage);
                    }
                    disabled=move || raw_json.get().is_empty()
                >
                    "Submit"
                </button>
            </ErrorBoundary>
        </div>
    }
}

#[component]
pub(crate) fn RoleChooser(
    setup_stage: WriteSignal<InitializationStage>,
    num_players: usize,
    script: ReadSignal<Script>,
    roles: RwSignal<Vec<RoleNames>>,
    next_setup_stage: InitializationStage,
) -> impl IntoView {
    let desired_character_type_counts =
        RwSignal::new(CharacterTypeCounts::new(num_players).unwrap());
    let curr_character_type_counts = RwSignal::new(CharacterTypeCounts::new_empty());

    let role_button = move |role: RoleNames| {
        let selected = RwSignal::new(false);
        view! {
            <button
                class="w-full text-left px-2 py-1 rounded border border-solid border-divider bg-panel text-ink text-sm hover:border-seat"
                style:border-color=move || if selected.get() { "var(--color-accent)" } else { "" }
                style:background=move || if selected.get() { "var(--color-selected-bg)" } else { "" }
                on:click=move |_| {
                    let role_type = role.get_type();
                    let desired_count = desired_character_type_counts.get().get_count(role_type);
                    let curr_count = curr_character_type_counts.get().get_count(role_type);
                    if !selected.get() {
                        let valid_choice = desired_count > curr_count;
                        if !valid_choice {
                            return;
                        }
                        roles.update(|v| v.push(role));
                        curr_character_type_counts
                            .update(|cct| cct.set_count(role_type, curr_count + 1));
                        desired_character_type_counts.update(|dct| dct.on_choose(role));
                    } else {
                        let element_index = roles.get().iter().position(|r| *r == role);
                        let role_i = match element_index {
                            Some(i) => i,
                            None => return,
                        };
                        roles
                            .update(|v| {
                                v.swap_remove(role_i);
                            });
                        curr_character_type_counts
                            .update(|cct| cct.set_count(role_type, curr_count - 1));
                        desired_character_type_counts.update(|dct| dct.on_remove(role));
                    }
                    selected.set(!selected.get());
                }
            >
                {move || role.to_string()}
            </button>
        }
    };

    view! {
        <div class="min-h-screen w-screen flex flex-col gap-4 bg-chassis text-ink p-6">
            <div class="flex flex-row items-center gap-6 border border-solid border-divider rounded p-3">
                <div class="flex flex-col gap-1 text-sm text-ink font-mono">
                    <p>
                        "Townsfolk: " {move || curr_character_type_counts.get().townsfolk} "/"
                        {move || desired_character_type_counts.get().townsfolk}
                    </p>
                    <p>
                        "Outsiders: " {move || curr_character_type_counts.get().outsiders} "/"
                        {move || desired_character_type_counts.get().outsiders}
                    </p>
                    <p>
                        "Minions: " {move || curr_character_type_counts.get().minions} "/"
                        {move || desired_character_type_counts.get().minions}
                    </p>
                    <p>
                        "Demons: " {move || curr_character_type_counts.get().demons} "/"
                        {move || desired_character_type_counts.get().demons}
                    </p>
                </div>
                <button
                    class="ml-auto px-4 py-2 rounded border border-solid border-input bg-accent hover:bg-accent-hover disabled:opacity-40 disabled:bg-panel disabled:hover:bg-panel font-mono uppercase tracking-widest text-sm"
                    on:click=move |_| {
                        if roles.get().len() == num_players {
                            setup_stage.set(next_setup_stage);
                        }
                    }
                    disabled=move || { roles.get().len() != num_players }
                >
                    "Finish"
                </button>
            </div>
            <div class="flex flex-row justify-start gap-6">
                <div class="flex flex-col gap-1 min-w-40">
                    <h3 class="font-mono uppercase tracking-widest text-xs text-muted mb-1">"Townsfolk"</h3>
                    <div class="flex flex-col gap-1">
                        {move || {
                            script
                                .get()
                                .roles
                                .into_iter()
                                .filter(|role| role.get_type() == CharacterType::Townsfolk)
                                .map(role_button)
                                .collect_view()
                        }}
                    </div>
                </div>
                <div class="flex flex-col gap-1 min-w-40">
                    <h3 class="font-mono uppercase tracking-widest text-xs text-muted mb-1">"Outsiders"</h3>
                    <div class="flex flex-col gap-1">
                        {move || {
                            script
                                .get()
                                .roles
                                .into_iter()
                                .filter(|role| role.get_type() == CharacterType::Outsider)
                                .map(role_button)
                                .collect_view()
                        }}
                    </div>
                </div>
                <div class="flex flex-col gap-1 min-w-40">
                    <h3 class="font-mono uppercase tracking-widest text-xs text-muted mb-1">"Minions"</h3>
                    <div class="flex flex-col gap-1">
                        {move || {
                            script
                                .get()
                                .roles
                                .into_iter()
                                .filter(|role| role.get_type() == CharacterType::Minion)
                                .map(role_button)
                                .collect_view()
                        }}
                    </div>
                </div>
                <div class="flex flex-col gap-1 min-w-40">
                    <h3 class="font-mono uppercase tracking-widest text-xs text-muted mb-1">"Demons"</h3>
                    <div class="flex flex-col gap-1">
                        {move || {
                            script
                                .get()
                                .roles
                                .into_iter()
                                .filter(|role| role.get_type() == CharacterType::Demon)
                                .map(role_button)
                                .collect_view()
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
