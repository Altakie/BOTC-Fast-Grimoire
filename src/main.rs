use leptos::mount::mount_to_body;
use leptos::prelude::*;

mod setup;
use setup::{Script, ScriptJson};

mod game;
use game::{Game, Player, Role};
// use leptos_router::components::*;
// use leptos_router::path;

fn main() {
    // Stack Traces
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[derive(Clone)]
enum SetUpStage {
    Start,
    InputPlayers,
    InputScript,
    ChooseRoles,
    // GameStart,
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
        //     <main>
        //         <Routes fallback=|| "Not Found">
        //             <Route path=path!("/") view=Starter/>
        //             </Route>
        //         </Routes>
        //     </main>
        // </Router>

        <div>{move || {
            match setup_stage.get() {
            SetUpStage::InputPlayers => {
                view! {
                    <PlayerInputer players=player_names setup_stage=setup_stage.write_only()/>
                }.into_any()
            },
            SetUpStage::InputScript => {
                    view! {
                    }.into_any()
            },
            SetUpStage::ChooseRoles => {
                    let num_players = player_names.get().len();
                    view! {
                        <RoleChooser num_players=num_players roles=roles/>
                    }.into_any()
            },
            _ => {
                view! {
                    <Starter setup_stage=setup_stage.write_only()/>
                }.into_any()
            }
        }
        }}
        </div>
    }
}

#[component]
fn Starter(setup_stage: WriteSignal<SetUpStage>) -> impl IntoView {
    view! {
        <button
        on:click = move|_| {setup_stage.set(SetUpStage::InputPlayers)}
        > "Press me!" </button>
    }
}

#[component]
fn PlayerInputer(
    players: RwSignal<Vec<String>>,
    setup_stage: WriteSignal<SetUpStage>,
) -> impl IntoView {
    let name = RwSignal::new(String::new());

    view! {
        <PlayerSetupList player_names=players/>
        // <form>
        <p>"Input Player Name Below"</p>
        <input id="PlayerInput" type="text" bind:value=name></input>
        <div>
            <button on:click = move|_| {players.update(|pv| pv.push(name.get()));
            name.set(String::from(""));
        }>
                "Add Player"
            </button>
            // TODO: Implement this button
            <button on:click = move |_| {setup_stage.set(SetUpStage::ChooseRoles);}>"Finish"</button>
        </div>
        // </form>
    }
}

#[component]
fn PlayerSetupList(player_names: RwSignal<Vec<String>>) -> impl IntoView {
    view! {
        <ol>
            {move || {
            player_names.get().into_iter().map(|player_name| {
                // WARN: This seems wrong
                let (player, _) = signal(player_name);
                view! {
                <li>
                    {player} "  "
                    <button on:click=move|_| {
                        player_names.update(|pv| {
                            let player_index = pv.iter().position(|p| *p == player.get()).unwrap();
                            if player_index <= 0 {
                            return
                            }
                            let temp = pv[player_index].clone();
                            pv[player_index] = pv[player_index - 1].clone();
                            pv[player_index - 1] = temp;
                        })
                    }>"Move Up"</button>
                    <button on:click=move|_| {
                        player_names.update(|pv| {
                            let player_index = pv.iter().position(|p| *p == player.get()).unwrap();
                            if (player_index + 1) >= pv.len() {
                            return
                            }
                            let temp = pv[player_index].clone();
                            pv[player_index] = pv[player_index + 1].clone();
                            pv[player_index + 1] = temp;
                    });
                    }>"Move Down"</button>
                    <button on:click=move|_| player_names.update(|pv| {
                            pv.remove(pv.iter().position(|p| *p == player.get()).unwrap());})>"X"</button>
                </li>
            }}).collect_view()
            }}
        </ol>
    }
}

#[component]
fn ScriptInputter(script: RwSignal<Script>, setup_stage: WriteSignal<SetUpStage>) -> impl IntoView {
    let raw_json = RwSignal::new(String::new());
    view! {
        // TODO: Might want to add an error boundary here
        <input type="text" bind:value=raw_json></input>
        <button on:click=move |_| {
            let script_json = serde_json::from_str::<ScriptJson>(&raw_json.get()).unwrap();
            script.set(Script::new_from_json(script_json));
            raw_json.set(String::from(""));
            setup_stage.set(SetUpStage::ChooseRoles);
        }>"Submit"</button>
    }
}

#[component]
fn RoleChooser(num_players: usize, roles: RwSignal<Vec<Role>>) -> impl IntoView {
    view! {}
}

#[component]
fn RoleChoosingDisplay(roles: RwSignal<Vec<Role>>) -> impl IntoView {
    view! {}
}

#[component]
fn Turn() -> impl IntoView {
    // let game = use_context::<ReadSignal<Game>>().expect("Why no game bruh");
    // let game_setter = use_context::<WriteSignal<Game>>().expect("Why no game bruh");
    // view! {
    //     <Game_Data />
    //     <input on:change:target=move|this| {game_setter.update(|game| game.prev_input = this.target().value());}></input>
    //     <button on:click = move |_|Player::new(name.get()) {game_setter.update(|game| game.update_game())} >"Next Turn"</button>
    // }
}

#[component]
fn Game_Data() -> impl IntoView {
    // let game = use_context::<ReadSignal<Game>>().expect("Why no game bruh");
    // view! {
    //     <div>
    //         <p>"Turn Count: " {move || game.read().turn_count} </p>
    //         <p>"Prev Input: " {move || game.read().prev_input.clone()} </p>
    //     </div>
    // }
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
