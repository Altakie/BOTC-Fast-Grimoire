#![allow(clippy::needless_return)]

use leptos::mount::mount_to_body;
use leptos::{
    leptos_dom::logging::{console_error, console_log},
    prelude::*,
};
use reactive_stores::Store;

mod initialization;
use initialization::{CharacterTypeCounts, Script, ScriptJson};

mod engine;
use engine::{
    change_request::{ChangeArgs, ChangeRequest, ChangeType},
    player::{CharacterType, Player, roles::Roles},
    state::{PlayerIndex, State, StateStoreFields, Step},
};

mod scripts;
use scripts::*;

use crate::engine::change_request::{ChangeRequestBuilder, StateChangeFuncPtr, check_len};
use crate::engine::state::log;

const DEBUG: bool = true;
// use leptos_router::components::*;
// use leptos_router::path;

fn main() {
    // Stack Traces
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[derive(Clone, Copy)]
enum InitializationStage {
    Start,
    InputScript,
    InputPlayers,
    ChooseRoles,
    GameStart,
}

#[component]
fn App() -> impl IntoView {
    let initialization_stage = RwSignal::new(InitializationStage::Start);
    let player_names = RwSignal::new(Vec::<String>::new());
    let roles = RwSignal::new(Vec::<Roles>::new());
    let script = RwSignal::new(Script { roles: vec![] });
    provide_context(script);

    // NOTE: Debug only
    if DEBUG {
        roles.set(vec![
            Roles::Ravenkeeper,
            Roles::Virgin,
            Roles::Monk,
            Roles::Slayer,
            // Role::Monk,
            Roles::Scarletwoman,
            Roles::Poisoner,
            Roles::Imp,
            Roles::Imp,
        ]);

        player_names.set(vec![
            "Artem".to_string(),
            "Naim".to_string(),
            "Alec".to_string(),
            "Nathaniel".to_string(),
            "Messiah".to_string(),
            "Isaac".to_string(),
            "Ben".to_string(),
            "Zhi".to_string(),
        ]);
        let script_json = serde_json::from_str("
     [{\"id\":\"_meta\",\"author\":\"\",\"name\":\"Trouble Brewing\"},\"washerwoman\",\"librarian\",\"investigator\",\"chef\",\"empath\",\"fortuneteller\",\"undertaker\",\"virgin\",\"soldier\",\"slayer\",\"mayor\",\"monk\",\"ravenkeeper\",\"drunk\",\"saint\",\"butler\",\"recluse\",\"spy\",\"baron\",\"scarletwoman\",\"poisoner\",\"imp\"]
     ");
        script.set(Script::new_from_json(script_json.unwrap()));

        initialization_stage.set(InitializationStage::GameStart);
    }

    view! {
        // Could not get router to work
        // <Router>
        // <main>
        // <Routes fallback=|| "Not Found">
        // <Route path=path!("/") view=Starter/>
        // </Route>
        // </Routes>
        // </main>
        // </Router>

        <div>
            {move || {
                match initialization_stage.get() {
                    InitializationStage::Start => {
                        view! {
                            <Starter
                                setup_stage=initialization_stage.write_only()
                                next_setup_stage=InitializationStage::InputScript
                            />
                        }
                            .into_any()
                    }
                    InitializationStage::InputScript => {
                        view! {
                            <ScriptInputter
                                script=script
                                setup_stage=initialization_stage.write_only()
                                next_setup_stage=InitializationStage::InputPlayers
                            />
                        }
                            .into_any()
                    }
                    InitializationStage::InputPlayers => {
                        view! {
                            <PlayerInputer
                                players=player_names
                                setup_stage=initialization_stage.write_only()
                                next_setup_stage=InitializationStage::ChooseRoles
                            />
                        }
                            .into_any()
                    }
                    InitializationStage::ChooseRoles => {
                        let num_players = player_names.get().len();
                        view! {
                            <RoleChooser
                                num_players=num_players
                                script=script.read_only()
                                roles=roles
                                setup_stage=initialization_stage.write_only()
                                next_setup_stage=InitializationStage::GameStart
                            />
                        }
                            .into_any()
                    }
                    InitializationStage::GameStart => {
                        view! {
                            <GameInterface
                                roles=roles.get()
                                player_names=player_names.get()
                                script=script.get()
                            />
                        }
                            .into_any()
                    }
                }
            }}
        </div>
    }
}

#[component]
fn Starter(
    setup_stage: WriteSignal<InitializationStage>,
    next_setup_stage: InitializationStage,
) -> impl IntoView {
    view! { <button on:click=move |_| { setup_stage.set(next_setup_stage) }>"Start Game"</button> }
}

#[component]
fn PlayerInputer(
    players: RwSignal<Vec<String>>,
    setup_stage: WriteSignal<InitializationStage>,
    next_setup_stage: InitializationStage,
) -> impl IntoView {
    let name = RwSignal::new(String::new());

    view! {
        <PlayerSetupList player_names=players />
        <p>"Input Player Name Below"</p>
        <input
            id="PlayerInput"
            type="text"
            bind:value=name
            on:keypress=move |ev| {
                if ev.key() == "Enter" {
                    players.update(|pv| pv.push(name.get()));
                    name.set(String::from(""));
                }
            }
        />
        <div>
            <button on:click=move |_| {
                players.update(|pv| pv.push(name.get()));
                name.set(String::from(""));
            }>"Add Player"</button>
            // TODO: Implement this button
            // TODO: Make sure you can't click this button unless you have at least 5 players
            // Because this is not handled, this button will throw an error
            <button
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
    }
}

#[component]
fn PlayerSetupList(player_names: RwSignal<Vec<String>>) -> impl IntoView {
    view! {
        <ol>
            // WARN: Should probably use this for loop for this and make it work, but for now other
            // option is fine
            <For
                each=move || player_names.get()
                key=|pn| pn.clone()
                children=move |player_name| {
                    let (player_name, _) = signal(player_name);
                    view! {
                        <li>
                            {player_name} "  "
                            <button
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
                            <button on:click=move |_| {
                                let i = player_names
                                    .read()
                                    .iter()
                                    .position(|p| *p == *player_name.read())
                                    .unwrap();
                                player_names
                                    .update(|pv| {
                                        pv.remove(i);
                                    })
                            }>"X"</button>
                        </li>
                    }
                }
            />
        </ol>
    }
}

#[component]
fn ScriptInputter(
    script: RwSignal<Script>,
    setup_stage: WriteSignal<InitializationStage>,
    next_setup_stage: InitializationStage,
) -> impl IntoView {
    let raw_json = RwSignal::new(String::new());
    view! {
        // TODO: Might want to add an error boundary here
        <button
            class="block"
            on:click=move |_| {
                script.set(trouble_brewing());
                setup_stage.set(next_setup_stage);
            }
        >
            "Trouble Brewing"
        </button>
        <p>"Input Custom Script Json Below"</p>
        <input type="text" bind:value=raw_json />
        <ErrorBoundary fallback=|_errors| ()>
            <button
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
    }
}

#[component]
fn RoleChooser(
    setup_stage: WriteSignal<InitializationStage>,
    num_players: usize,
    script: ReadSignal<Script>,
    roles: RwSignal<Vec<Roles>>,
    next_setup_stage: InitializationStage,
) -> impl IntoView {
    // Iterate through roles in script
    // For each role, make it clickable?
    // When it is clicked, check that it is valid to be added to the len roles and add it if it is
    // If it isn't do nothing
    // Add a done button

    let desired_character_type_counts =
        RwSignal::new(CharacterTypeCounts::new(num_players).unwrap());
    let curr_character_type_counts = RwSignal::new(CharacterTypeCounts::new_empty());

    let role_button = move |role: Roles| {
        let selected = RwSignal::new(false);
        view! {
            <button
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
                style:color=move || if selected.get() { "red" } else { "black" }
            >
                {move || role.to_string()}
            </button>
            <br />
        }
    };

    view! {
        <div class="flex flex-row gap-[2%]">
            <div>
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
                class="flex-1"
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
        <div class="flex flex-row justify-start gap-[2%]">
            <div>
                <h3>"Townsfolk"</h3>
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
            <div>
                <h3>"Outsiders"</h3>
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
            <div>
                <h3>"Minions"</h3>
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
            <div>
                <h3>"Demons"</h3>
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
    }
}

#[derive(Clone, Debug, Store, Default)]
struct TempState {
    selected_player: Option<PlayerIndex>,
    curr_change_request: Option<ChangeRequest>,
    selected_players: Vec<PlayerIndex>,
    selected_roles: Vec<Roles>,
    currently_acting_player: Option<PlayerIndex>,
}

impl TempState {
    fn clear_selected(&mut self) {
        self.selected_players.clear();
        self.selected_roles.clear();
        self.curr_change_request = None
    }
    fn reset(&mut self) {
        self.selected_player = None;
        self.selected_players.clear();
        self.curr_change_request = None;
        self.selected_roles.clear();
        self.currently_acting_player = None;
    }
}

#[component]
fn GameInterface(roles: Vec<Roles>, player_names: Vec<String>, script: Script) -> impl IntoView {
    // Create a new game using the data we have just collected from the user
    let state = Store::new(State::new(roles, player_names, script).unwrap());
    provide_context(state);
    let temp_state = Store::new(TempState::default());
    provide_context(temp_state);

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <p>"Errors:"</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <div class="h-screen border border-dashed flex justify-between">
                <Info />
                <Game />
                <Picker_Bar />
            </div>
        </ErrorBoundary>

        <LogDisplay />
    }
    .into_any()
}

#[component]
fn Info() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();

    let stage_info = move || {
        let step = game_state.step().get();
        match step {
            Step::Start => "Start".to_string(),
            Step::Setup => "Setup".to_string(),
            // Step::DayDiscussion => {
            //     format!("Day {} Discussion", game_state.day_num().get()).to_string()
            // }
            // Step::DayExecution => {
            //     format!("Day {} Execution", game_state.day_num().get()).to_string()
            // }
            Step::Day => format!("Day {}", game_state.day_num().get()).to_string(),
            Step::NightOne | Step::Night => {
                format!("Night {}", game_state.day_num().get()).to_string()
            }
        }
    };

    let change_info = move || {
        let cr = temp_state.curr_change_request().get();
        match cr {
            Some(cr) => cr.get_description(),
            // match cr.change_type {
            //     ChangeType::ChoosePlayers(num) => format!("Choose {} Players", num),
            //     ChangeType::ChooseRoles(num) => format!("Choose {} Roles", num),
            //     ChangeType::Display(string) => string,
            //     _ => "Not Yet Implemented".to_string(),
            // }
            None => "None".to_string(),
        }
    };

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
            <div class="border border-solid w-full p-[1rem]">
                <h3>"Current Player"</h3>
                <p class="mx-auto">{player.name}</p>
                <p>"Role: "{player.role.to_string()}</p>
                <p>
                    "Status: "{if player.dead { "Dead" } else { "Alive" }}
                    <button on:click=move |_| {
                        game_state
                            .players()
                            .update(|players: &mut Vec<Player>| {
                                let dead = &mut players[player_index].dead;
                                *dead = !*dead;
                            });
                    }>"Toggle"</button>
                </p>
                <p>"Ghost Vote: "{if player.dead { "Yes" } else { "No" }}</p>
                <p>"Alignment: " {player.alignment.to_string()}</p>
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
            <div class="border border-solid w-full p-[1rem]">
                <h3>"Selected Player"</h3>
                <p class="mx-auto">{player.name}</p>
                <p>"Role: "{player.role.to_string()}</p>
                <p>
                    "Status: "{if player.dead { "Dead" } else { "Alive" }}
                    <button
                        on:click=move |_| {
                            game_state.update(|gs| gs.execute_player(player_index));
                        }
                        disabled=move || { !matches!(game_state.step().get(), Step::Day) }
                    >
                        "Execute"
                    </button>
                </p>
                <p>
                    "Ghost Vote: "{if player.ghost_vote { "Yes" } else { "No" }}
                    <button on:click=move |_| {
                        game_state
                            .players()
                            .update(|players: &mut Vec<Player>| {
                                let ghost_vote = &mut players[player_index].ghost_vote;
                                *ghost_vote = !*ghost_vote;
                            });
                    }>"Toggle"</button>
                </p>
                <p>"Alignment: " {player.alignment.to_string()}</p>
            </div>
        }
        .into_any();
    };
    view! {
        <div class="flex flex-col items-start flex-1">
            <div class="border border-solid w-full p-[1rem]">
                <h3>"Game Info"</h3>
                <p>{stage_info}</p>
                <p>"Change Type: "{change_info}</p>
            // <For
            // each=move || players.get()
            // key=|p| p.name.clone()
            // children=move |player| {
            // view! { <p>"Player "{player.name}</p> }
            // }
            // />
            </div>
            {selected_player_info}
            {current_player_info}
        </div>
    }
}

#[component]
fn Game() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();
    let next_button = move || {
        // If there is a change request in the queue, process it
        if let Some(cr) = temp_state.curr_change_request().get() {
            // console_log(&format!("Curr cr is {:?}", cr));
            // Do check func and return early if it doesn't pass
            let args = match cr.get_change_type() {
                ChangeType::ChoosePlayers(_) => Some(ChangeArgs::PlayerIndices(
                    temp_state.selected_players().get(),
                )),
                ChangeType::ChooseRoles(_) => {
                    Some(ChangeArgs::Roles(temp_state.selected_roles().get()))
                }
                ChangeType::NoStoryteller => Some(ChangeArgs::Blank),
                _ => None,
            };

            // Only apply funcs if change_type requires action
            if let Some(args) = args
                && let Some(state_func) = cr.get_state_change_func()
            {
                let next_cr = game_state
                    .try_update(|gs| state_func.call(gs, args))
                    .unwrap();

                let next_cr = match next_cr {
                    Ok(cr) => cr,
                    // TODO: Actually inform the player what went wrong using the result
                    Err(e) => {
                        console_log(format!("Error: {:?}", e).as_str());
                        console_log(format!("TempState: {:#?}", temp_state.get()).as_str());
                        return;
                    }
                };

                // console_log(&format!("{:?}", next_cr));
                // Set the next cr

                if next_cr.is_some() {
                    temp_state.update(|ts| ts.clear_selected());
                    temp_state.curr_change_request().set(build(next_cr));
                    // console_log(&format!(
                    //     "New Cr set as {:?}, curr cr is now {:?}",
                    //     next_cr,
                    //     temp_state.curr_change_request().get()
                    // ));
                    return;
                }
            }
        }

        console_log("Moving to next player");

        // Only get the next player's change requests if the current change request queue is empty

        // Get next active player based off of current player
        let mut loop_break = false;
        loop {
            let currently_acting_player = temp_state.read().currently_acting_player;
            let has_curr_cr = temp_state.get().curr_change_request.is_some();
            temp_state.update(|ts| ts.reset());
            // Check for next active player
            let next_player = game_state
                .read()
                .get_next_active_player(currently_acting_player);
            console_log(format!("Next Player is {:?}", next_player).as_str());
            match next_player {
                Some(p) => {
                    let next_cr = game_state.read().resolve(p);
                    temp_state.currently_acting_player().set(Some(p));
                    match next_cr {
                        Some(next_cr) => {
                            temp_state.curr_change_request().set(Some(next_cr.build()));
                            break;
                        }
                        None => {
                            // If there is a player with no change request, just skip them
                            console_error("Next Change Request is none");
                            continue;
                        }
                    }
                }
                // Switch to next step when get next active player yields None
                None => {
                    if loop_break {
                        break;
                    }
                    console_log(&format!("has curr cr{}", has_curr_cr));
                    if game_state.step().get() == Step::Day && has_curr_cr {
                        // Don't want to accidentally move to next step during day
                        break;
                    }
                    game_state.update(|gs| gs.next_step());
                    loop_break = true;
                }
            }
        }
    };

    let game_element: NodeRef<leptos::html::Div> = NodeRef::new();

    view! {
        <div
            class="relative w-3/5 flex justify-center items-center focus:outline-none"
            on:keydown=move |ev| {
                if ev.key() == "Enter" {
                    console_log("Next Button Pressed");
                    next_button()
                }
            }
            on:mouseenter=move |_| {
                if let Some(element) = game_element.get() {
                    _ = element.focus();
                }
            }
            on:mouseleave=move |_| {
                if let Some(element) = game_element.get() {
                    _ = element.blur();
                }
            }
            tabindex=-1
            node_ref=game_element
        >
            <Player_Display />
            <button class="absolute right-[0px] top-[0px]" on:click=move |_| next_button()>
                "Next"
            </button>
        </div>
    }
}

#[component]
fn Player_Display() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let players = game_state.players();
    let player_positions = calc_circle(players.read_untracked().len(), 75.0);

    let temp_state = expect_context::<Store<TempState>>();
    let currently_selected_player = temp_state.selected_player();
    let selected_players = temp_state.selected_players();
    // Want to place children in a circle within the container, centered around it's origin
    // Radius of circle should dynamically grow based on number of items

    view! {
        <div class="relative origin-bottom-right size-1/2 flex flex-wrap rounded-full justify-between items-between">
            <For
                each=move || players.get().into_iter().enumerate()
                key=|(i, _)| *i
                children=move |(i, _)| {
                    let pos = player_positions[i];
                    let player = Memo::new(move |_| players.get()[i].clone());
                    console_log("New Signal Created");
                    let selected = move || temp_state.selected_players().get().contains(&i);

                    view! {
                        <div
                            class="translate-1/2 absolute size-fit"
                            style:right=move || { format!("calc(50% + {}%)", pos.0) }
                            style:top=move || format!("calc(35% + {}%)", pos.1)
                        >
                            <p class="absolute left-1/2 -translate-x-1/2 bottom-3/5 border-solid border text-center bg-[#ffffff]">
                                {player.get().name}
                            </p>
                            <button
                                class="size-[5rem] rounded-full text-center border border-[#000000]"
                                disabled=move || {
                                    if let Some(cr) = temp_state.curr_change_request().get() && let Some(filter_func) = cr.get_filter_func() {
                                            return !filter_func.call(i, &player.read());
                                        }

                                    false
                                }
                                style:border-style=move || {
                                    if selected() {
                                        "solid"
                                    } else if !player.get().ghost_vote {
                                        "dashed"
                                    } else {
                                        "none"
                                    }
                                }
                                style:background=move || {
                                    if let Some(selected_player) = temp_state
                                        .currently_acting_player()
                                        .get() && selected_player == i {
                                            return "aquamarine";
                                        }

                                    ""
                                }
                                style:color=move || {
                                    if player.get().dead {
                                        return "gray";
                                    }
                                    match player.get().alignment {
                                        engine::player::Alignment::Good => "blue",
                                        engine::player::Alignment::Evil => "red",
                                        engine::player::Alignment::Any => "purple",
                                    }
                                }
                                on:keypress=move |ev| {
                                    ev.prevent_default();
                                }
                                on:click=move |_| {
                                    if temp_state.curr_change_request().read().is_none() {
                                        currently_selected_player.set(Some(i));
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
                            // Status effects
                            <div class="text-[0.5rem] flex flex-row flex-wrap justify-center items-start absolute w-fit border left-1/2 -translate-x-1/2 top-9/10 ">
                                {move || {
                                    let status_effects = game_state
                                        .with(|gs| gs.get_player(i).status_effects.clone());
                                    status_effects
                                        .iter()
                                        .map(|status_effect| {
                                            let str = status_effect.status_type.to_string();
                                            view! {
                                                // TODO: Standardize effect box sizes
                                                <p class="size-fit text-center border border-solid m-[0%] rounded-full p-[5px] bg-[#ffff00]">
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

fn calc_circle(num_players: usize, radius: f64) -> Vec<(f64, f64)> {
    // Calculate circumference
    // Calculate position using radius and angle
    // First find differnence in angle
    let angle_diff = -(360.0 / num_players as f64);

    let mut res: Vec<(f64, f64)> = vec![];

    for i in 0..num_players {
        let angle = ((i as f64) * angle_diff - 90.0).to_radians();
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        res.push((x, y));
    }

    return res;
}

#[component]
fn Picker_Bar() -> impl IntoView {
    // TODO: Implement search bar with fuzzy finding
    // Need input box bound to string signal
    // Need a for function or something of the sort that shows a scrollable list of items
    // Finish button that calls a generic function passed in through something
    // When an item is selected, add it to a result list
    // This should reset and disable after finish button is clicked
    let state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();

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
        <div class="flex-1 border-solid border p-[1rem]">
            <div>{display}</div>
        </div>
    }
}

#[component]
fn RoleSelector() -> impl IntoView {
    let script = expect_context::<RwSignal<Script>>();
    let temp_state = expect_context::<Store<TempState>>();
    view! {
        <div class="flex flex-col">
            {move || {
                script
                    .get()
                    .roles
                    .into_iter()
                    .map(move |role| {
                        let selected = RwSignal::new(false);
                        view! {
                            <button
                                style:color=move || { if selected.get() { "red" } else { "" } }
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

        let state_change_func = StateChangeFuncPtr::new(move |_, args| {
            let nominating_players = args.extract_player_indicies()?;
            check_len(&nominating_players, 1)?;

            let nominating_player = nominating_players[0];

            let description = "Select the nominated player";

            let state_change_func = StateChangeFuncPtr::new(move |state, args| {
                let target_players = args.extract_player_indicies()?;
                check_len(&target_players, 1)?;

                let nominated_player = target_players[0];
                state.nominate_player(nominating_player, nominated_player);
                Ok(None)
            });

            ChangeRequest::new_builder(change_type, description.into())
                .state_change_func(state_change_func)
                .into()
        });

        let nominate_request = ChangeRequest::new_builder(change_type, description.into())
            .state_change_func(state_change_func)
            .build();
        temp_state.curr_change_request().set(Some(nominate_request));
    };

    view! {
        <div class="flex flex-col">
            <button on:click=nominate_button>"Nominate"</button>
            {move || {
                let active_players = state.read().get_day_active();
                active_players
                    .into_iter()
                    .map(|player_index| {
                        let role = state.read().get_player(player_index).role.clone();

                        view! {
                            <button on:click=move |_| {
                                temp_state.update(|ts| ts.reset());
                                let player_ability = state.read().day_ability(player_index);
                                temp_state.curr_change_request().set(build(player_ability));
                                temp_state.currently_acting_player().set(Some(player_index));
                            }>{move || { format!("{} Ability", role) }}</button>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

#[component]
fn LogDisplay() -> impl IntoView {
    let state = expect_context::<Store<State>>();

    view! {
        <div class="border">
            <h2>"Log"</h2>
            <div>
        // {move || { format!("{:#?}", state.log().get()) }}
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
                                <p>{state.read().describe_event(event)}</p>
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

// #[component]
// fn DayPhaseDisplay(day_phase: Memo<log::DayPhaseLog>) -> impl IntoView {
//     view! {};
// }

fn build(change_option: Option<ChangeRequestBuilder>) -> Option<ChangeRequest> {
    match change_option {
        Some(cr) => Some(cr.build()),
        None => None,
    }
}
