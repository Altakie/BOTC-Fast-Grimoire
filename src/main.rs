use leptos::mount::mount_to_body;
use leptos::prelude::*;

mod setup;
use setup::{CharacterTypeCounts, Script, ScriptJson};

mod game;
use game::{Game, Player, Role, StoryTellerInterface};
// use leptos_router::components::*;
// use leptos_router::path;

fn main() {
    // Stack Traces
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[derive(Clone, Copy)]
enum SetUpStage {
    Start,
    InputScript,
    InputPlayers,
    ChooseRoles,
    GameStart,
}

#[component]
fn App() -> impl IntoView {
    let setup_stage = RwSignal::new(SetUpStage::Start);
    let player_names = RwSignal::new(Vec::<String>::new());
    let roles = RwSignal::new(Vec::<Role>::new());
    let script = RwSignal::new(Script { roles: vec![] });

    // let game = RwSignal::new(Game::new());
    // provide_context(game);

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
                match setup_stage.get() {
                    SetUpStage::Start => {
                        view! {
                            <Starter
                                setup_stage=setup_stage.write_only()
                                next_setup_stage=SetUpStage::InputScript
                            />
                        }
                            .into_any()
                    }
                    SetUpStage::InputScript => {
                        view! {
                            <ScriptInputter
                                script=script
                                setup_stage=setup_stage.write_only()
                                next_setup_stage=SetUpStage::InputPlayers
                            />
                        }
                            .into_any()
                    }
                    SetUpStage::InputPlayers => {
                        view! {
                            <PlayerInputer
                                players=player_names
                                setup_stage=setup_stage.write_only()
                                next_setup_stage=SetUpStage::ChooseRoles
                            />
                        }
                            .into_any()
                    }
                    SetUpStage::ChooseRoles => {
                        let num_players = player_names.get().len();
                        view! {
                            <RoleChooser
                                num_players=num_players
                                script=script.read_only()
                                roles=roles
                                setup_stage=setup_stage.write_only()
                                next_setup_stage=SetUpStage::GameStart
                            />
                        }
                            .into_any()
                    }
                    SetUpStage::GameStart => {
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
fn Starter(setup_stage: WriteSignal<SetUpStage>, next_setup_stage: SetUpStage) -> impl IntoView {
    view! { <button on:click=move |_| { setup_stage.set(next_setup_stage) }>"Start Game"</button> }
}

#[component]
fn PlayerInputer(
    players: RwSignal<Vec<String>>,
    setup_stage: WriteSignal<SetUpStage>,
    next_setup_stage: SetUpStage,
) -> impl IntoView {
    let name = RwSignal::new(String::new());

    view! {
        <PlayerSetupList player_names=players />
        <p>"Input Player Name Below"</p>
        <input id="PlayerInput" type="text" bind:value=name />
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
            // <For each=move || player_names.get()
            // key=|player_name| player_name.as_str()
            // children = move |player_name| {
            // let (player, _) = signal(player_name);
            // view! {
            // <li>
            // {player} "  "
            // <button on:click=move|_| {
            // player_names.update(|pv| {
            // let player_index = pv.iter().position(|p| *p == player.get()).unwrap();
            // if player_index <= 0 {
            // return
            // }
            // let temp = pv[player_index].clone();
            // pv[player_index] = pv[player_index - 1].clone();
            // pv[player_index - 1] = temp;
            // })
            // }>"Move Up"</button>
            // <button on:click=move|_| {
            // player_names.update(|pv| {
            // let player_index = pv.iter().position(|p| *p == player.get()).unwrap();
            // if (player_index + 1) >= pv.len() {
            // return
            // }
            // let temp = pv[player_index].clone();
            // pv[player_index] = pv[player_index + 1].clone();
            // pv[player_index + 1] = temp;
            // });
            // }>"Move Down"</button>
            // <button on:click=move|_| player_names.update(|pv| {
            // pv.remove(pv.iter().position(|p| *p == player.get()).unwrap());})>"X"</button>
            // </li>
            // }}
            // />

            {move || {
                player_names
                    .get()
                    .into_iter()
                    .map(|player_name| {
                        let (player, _) = signal(player_name);
                        view! {
                            <li>
                                {player} "  "
                                <button
                                    on:click=move |_| {
                                        player_names
                                            .update(|pv| {
                                                let player_index = pv
                                                    .iter()
                                                    .position(|p| *p == player.get())
                                                    .unwrap();
                                                if player_index == 0 {
                                                    return;
                                                }
                                                let temp = pv[player_index].clone();
                                                pv[player_index] = pv[player_index - 1].clone();
                                                pv[player_index - 1] = temp;
                                            })
                                    }
                                    disabled=move || {
                                        let pv = player_names.get();
                                        let player_index = pv
                                            .iter()
                                            .position(|p| *p == player.get())
                                            .unwrap();
                                        player_index == 0
                                    }
                                >
                                    "Move Up"
                                </button>
                                <button
                                    on:click=move |_| {
                                        player_names
                                            .update(|pv| {
                                                let player_index = pv
                                                    .iter()
                                                    .position(|p| *p == player.get())
                                                    .unwrap();
                                                if (player_index + 1) >= pv.len() {
                                                    return;
                                                }
                                                let temp = pv[player_index].clone();
                                                pv[player_index] = pv[player_index + 1].clone();
                                                pv[player_index + 1] = temp;
                                            });
                                    }
                                    disabled=move || {
                                        let pv = player_names.get();
                                        let player_index = pv
                                            .iter()
                                            .position(|p| *p == player.get())
                                            .unwrap();
                                        (player_index + 1) >= pv.len()
                                    }
                                >
                                    "Move Down"
                                </button>
                                <button on:click=move |_| {
                                    player_names
                                        .update(|pv| {
                                            pv.remove(
                                                pv.iter().position(|p| *p == player.get()).unwrap(),
                                            );
                                        })
                                }>"X"</button>
                            </li>
                        }
                    })
                    .collect_view()
            }}
        </ol>
    }
}

#[component]
fn ScriptInputter(
    script: RwSignal<Script>,
    setup_stage: WriteSignal<SetUpStage>,
    next_setup_stage: SetUpStage,
) -> impl IntoView {
    let raw_json = RwSignal::new(String::new());
    view! {
        // TODO: Might want to add an error boundary here
        <p>"Input Script Json Below"</p>
        <input type="text" bind:value=raw_json />
        <ErrorBoundary fallback=|_errors| view! {}>
            <button
                on:click=move |_| {
                    let script_json = serde_json::from_str::<ScriptJson>(&raw_json.get()).unwrap();
                    script.set(Script::new_from_json(script_json));
                    raw_json.set(String::from(""));
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
    setup_stage: WriteSignal<SetUpStage>,
    num_players: usize,
    script: ReadSignal<Script>,
    roles: RwSignal<Vec<Role>>,
    next_setup_stage: SetUpStage,
) -> impl IntoView {
    // Iterate through roles in script
    // For each role, make it clickable?
    // When it is clicked, check that it is valid to be added to the len roles and add it if it is
    // If it isn't do nothing
    // Add a done button

    let desired_character_type_counts =
        RwSignal::new(CharacterTypeCounts::new(num_players).unwrap());
    let curr_character_type_counts = RwSignal::new(CharacterTypeCounts::new_empty());

    view! {
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
        <div>
            <For
                each=move || script.get().roles
                // WARN: Key might not always be unique in other scripts with repeated roles
                key=|role| role.clone()
                children=move |role| {
                    let selected = RwSignal::new(false);
                    view! {
                        <button
                            on:click=move |_| {
                                let role_type = role.get_type();
                                let desired_count = desired_character_type_counts
                                    .get()
                                    .get_count(role_type);
                                let curr_count = curr_character_type_counts
                                    .get()
                                    .get_count(role_type);
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
                }
            />
        </div>
        <button
            on:click=move |_| {
                if roles.get().len() == num_players {
                    setup_stage.set(next_setup_stage);
                }
            }
            disabled=move || { roles.get().len() != num_players }
        >
            "Finish"
        </button>
    }
}

#[component]
fn GameInterface(roles: Vec<Role>, player_names: Vec<String>, script: Script) -> impl IntoView {
    // TODO: Create a new game using the data we have just collected from the user
    let game =
        RwSignal::new(Game::new(roles, player_names, script, WebStoryTellerInterface {}).unwrap());

    let player_view = move |player: Player| {
        view! {
            <div style:border="solid">
                <p>{player.name.to_string()}</p>
                <p>{player.role.to_string()}</p>
            </div>
        }
    };
    view! {
        <p>"Not yet implemented"</p>
        <div>
            <For
                each=move || game.get().get_players()
                key=|p| p.name.clone()
                children=player_view
            />
        </div>
    }
}

#[derive(Clone)]
struct WebStoryTellerInterface {}

impl StoryTellerInterface for WebStoryTellerInterface {
    // Change a value that renders a certain view, grab the values from that view
    fn choose_players(&self, num: usize, max_index: usize) -> Vec<game::PlayerIndex> {
        todo!()
    }

    fn choose_roles(&self, num: usize, valid_roles: Vec<Role>) -> Vec<Role> {
        todo!()
    }

    fn input_number(&self) -> usize {
        todo!()
    }

    fn display_number(&self) {
        todo!()
    }

    fn display_players(&self) {
        todo!()
    }

    fn display_role(&self) {
        todo!()
    }
}

// struct GenSignal<T> {
//     pub(crate) defined_at: &'static Location<'static>,
//     read_signal: ReadSignal<T>,
//     write_signal: WriteSignal<T>,
// }
//
// impl<T: Sync + Send + 'static> GenSignal<T> {
//     fn new(value: T) -> Self {
//         let (read_signal, write_signal) = signal(value);
//         Self {
//             read_signal,
//             write_signal,
//         }
//     }
//
// }
