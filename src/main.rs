#![allow(clippy::needless_return)]
use leptos::mount::mount_to_body;
use leptos::{
    leptos_dom::logging::{console_error, console_log},
    prelude::*,
};
use reactive_stores::Store;
use std::collections::VecDeque;

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
            Roles::Washerwoman,
            Roles::Investigator,
            Roles::Librarian,
            Roles::Fortuneteller,
            // Role::Monk,
            Roles::Drunk,
            Roles::Empath,
            Roles::Spy,
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

#[derive(Clone, Store, Default)]
struct TempState {
    selected_player: Option<PlayerIndex>,
    change_requests: VecDeque<ChangeRequest>,
    selected_players: Vec<PlayerIndex>,
    selected_roles: Vec<Roles>,
    currently_acting_player: Option<PlayerIndex>,
}

impl TempState {
    fn clear_selected(&mut self) {
        self.selected_players.clear();
        self.selected_roles.clear();
    }
    fn reset(&mut self) {
        self.selected_player = None;
        self.selected_players.clear();
        self.change_requests.clear();
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
            Step::DayDiscussion | Step::DayExecution => {
                format!("Day {}", game_state.day_num().get()).to_string()
            }
            Step::Night1 | Step::Night => {
                format!("Night {}", game_state.day_num().get()).to_string()
            }
        }
    };

    let change_info = move || {
        let crs = temp_state.change_requests().get();
        match crs.front() {
            Some(cr) => cr.description.clone(),
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
                <p>"Ghost Vote: "{if player.ghost_vote { "Yes" } else { "No" }}</p>
                <p>
                    "Alignment: "
                    {player.alignment.to_string()}
                </p>
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
                <p>"Ghost Vote: "{if player.ghost_vote { "Yes" } else { "No" }}</p>
                <p>
                    "Alignment: "
                    {player.alignment.to_string()}
                </p>
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
    let next_button = move |_| {
        // TODO: This function is too complicated and I don't like it
        // Want to make the logic a little simpler so it is easier to debug
        // Break it up into multiple functions
        // Maybe a clean up func at the end
        // Maybe check check func and a apply state func function? Could be good to split them
        // because there is not always a change func and state change func
        let mut loop_break = false;
        loop {
            // If there is a change request in the queue, process it
            if !temp_state.change_requests().read().is_empty() {
                let cr = temp_state.change_requests().read().front().unwrap().clone();
                // Do check func and return early if it doesn't pass
                let args = match &cr.change_type {
                    ChangeType::ChoosePlayers(_) => Some(ChangeArgs::PlayerIndices(
                        temp_state.selected_players().get(),
                    )),
                    ChangeType::ChooseRoles(_) => {
                        Some(ChangeArgs::Roles(temp_state.selected_roles().get()))
                    }
                    _ => None,
                };

            // Only apply funcs if change_type requires action
            if args.is_some() {
                let args = args.unwrap();
                let check_func = cr.check_func;
                let res = game_state.with(|gs| {
                    let cf = check_func.unwrap();
                    cf.call(gs, &args)
                });
                match res {
                    Ok(boolean) => {
                        if boolean {
                        } else {
                            return;
                        }
                    }
                    // If it passes, do the apply state func and move on
                    let state_func = cr.state_change_func.unwrap().clone();
                    game_state.update(|gs| state_func(gs, args));
                } else if let ChangeRequest {
                    change_type: ChangeType::NoStoryteller,
                    state_change_func: Some(change_func),
                    ..
                } = cr
                {
                    // TODO: Apply the state change func and skip to the next thing without another
                    // press of the button
                    // Current problem: this will be added to the queue, but will not automatically
                    // call the function again
                    // Can be fixed by making this a loop and having it not return

                    todo!()
                }
                // If it passes, do the apply state func and move on
                let state_func = cr.state_change_func.unwrap();
                game_state.update(|gs| state_func.call(gs, args));
            }

            temp_state.update(|ts| ts.clear_selected());
            // Only get the next player's change requests if the current change request queue is empty
            if !temp_state.change_requests().read().is_empty() {
                return;
            }
            console_log("Last cr");

            // Get next active player based off of current player
            // TODO: Need phases for dawn dusk (maybe), but also waking up minions and demons to reveal
            // themselves to each other. These are problematic because they are not tied to players,
            // and thus not actively with the night order. They need to be inserted in there, maybe
            // just as a player, or maybe resolve the system to not work off of a next active player,
            // but maybe a next active event, that has an associated player? Might just have to redo
            // the whole ordering system at some point
            let currently_acting_player = temp_state.read().currently_acting_player;
            temp_state.update(|ts| ts.reset());
            // Check for next active player
            let next_player = game_state
                .read()
                .get_next_active_player(currently_acting_player);
            console_log(format!("Next Player is {:?}", next_player).as_str());
            match next_player {
                Some(p) => {
                    let next_crs = game_state.try_update(|gs| gs.resolve(p));
                    temp_state.currently_acting_player().set(Some(p));
                    // FIX: I think this can fail if the next cr is None, this fix is weird, figure
                    // out when the none case can happen. It's when a player doesn't have an
                    // associated change request, which means something is broken. Meaning it's
                    // okay to panic
                    match next_crs.unwrap() {
                        Some(next_crs) => {
                            temp_state
                                .change_requests()
                                .update(|crs| crs.extend(next_crs));
                            // Should not break
                        }
                        None => {
                            console_error("Next Change Request is none");
                            break;
                        }
                    }
                }
                // Switch to next step when get next active player yields None
                None => {
                    if loop_break {
                        break;
                    }
                    game_state.update(|gs| gs.next_step());
                    loop_break = true;
                }
            }
        }
    };
    view! {
        <div class="relative w-3/5 flex justify-center items-center">
            <Player_Display />
            <button class="absolute right-[0px] top-[0px]" on:click=next_button>
                "Next"
            </button>
        </div>
    }
}

#[component]
fn Player_Display() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let players = game_state.players();
    let player_positions = calc_circle(players.read().len(), 75.0);

    let temp_state = expect_context::<Store<TempState>>();
    let currently_selected_player = temp_state.selected_player();
    let selected_players = temp_state.selected_players();
    // Want to place children in a circle within the container, centered around it's origin
    // Radius of circle should dynamically grow based on number of items

    view! {
        <div class="relative origin-bottom-right size-1/2 flex flex-wrap rounded-full justify-between items-between">
            // Static list because players don't change
            {move || {
                players
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, player)| {
                        let pos = player_positions[i];
                        let selected = RwSignal::new(false);
                        view! {
                            <div
                                class="translate-1/2 absolute size-fit"
                                style:right=move || { format!("calc(50% + {}%)", pos.0) }
                                style:top=move || format!("calc(35% + {}%)", pos.1)
                            >
                                <p class="absolute left-1/2 -translate-x-1/2 bottom-3/5 border-solid border text-center bg-[#ffffff]">
                                    {player.name}
                                </p>
                                <button
                                    class="size-[5rem] rounded-full text-center border border-[#000000]"
                                    style:border-style=move || {
                                        if selected.get() { "solid" } else { "none" }
                                    }
                                    style:color=move || {
                                        if player.dead {
                                            return "gray";
                                        }
                                        match player.alignment {
                                            engine::player::Alignment::Good => "blue",
                                            engine::player::Alignment::Evil => "red",
                                            engine::player::Alignment::Any => "purple"
                                        }
                                    }
                                    on:click=move |_| {
                                        if temp_state.change_requests().read().is_empty() {
                                            currently_selected_player.set(Some(i));
                                            return;
                                        }
                                        let cr = temp_state
                                            .change_requests()
                                            .read()
                                            .front()
                                            .unwrap()
                                            .clone();
                                        let requested_num = match cr.change_type {
                                            ChangeType::ChoosePlayers(num) => num,
                                            _ => {
                                                currently_selected_player.set(Some(i));
                                                return;
                                            }
                                        };
                                        if selected.get() {
                                            selected_players
                                                .update(|pv| {
                                                    let remove_index = pv
                                                        .iter()
                                                        .position(|pi| *pi == i)
                                                        .unwrap();
                                                    pv.remove(remove_index);
                                                });
                                            selected.set(false);
                                            return;
                                        }
                                        if selected_players.read().len() >= requested_num {
                                            return;
                                        }
                                        selected_players.update(|pv| pv.push(i));
                                        selected.set(true);
                                    }
                                >
                                    {move || { player.role.to_string() }}
                                </button>
                                // Status effects
                                <div class="text-[0.5rem] flex flex-row flex-wrap justify-center items-start absolute w-fit border left-1/2 -translate-x-1/2 top-9/10 ">
                                    {move || {
                                        let status_effects = game_state.with(|gs| gs.get_player(i).status_effects.clone());
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
                    })
                    .collect_view()
            }}
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
    let _state = expect_context::<Store<State>>();
    let script = expect_context::<RwSignal<Script>>();
    let temp_state = expect_context::<Store<TempState>>();
    let _input = RwSignal::new(String::new());

    view! {
        <div class="flex-1 border-solid border p-[1rem]">
            <div>
                <input bind:input />
                <button>"Finish"</button>
            </div>
            <div class="flex flex-col">
                <For
                    each=move || script.get().roles
                    key=|r| *r
                    children=move |role| {
                        let selected = RwSignal::new(false);
                        view! {
                            <button
                                style:color=move || { if selected.get() { "red" } else { "" } }
                                on:click=move |_| {
                                    let cr = {
                                        if temp_state.change_requests().read().is_empty() {
                                            return;
                                        }
                                        temp_state.change_requests().get().front().unwrap().clone()
                                    };
                                    let requested_num = match cr.change_type {
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
                    }
                />
            </div>
        </div>
    }
}
