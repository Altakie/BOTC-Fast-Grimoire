use leptos::prelude::*;
use leptos::{leptos_dom::logging::{console_error, console_log}};
use reactive_stores::Store;

use crate::engine::{
    change_request::{ChangeArgs, ChangeRequest, ChangeRequestBuilder, ChangeType, StateChangeFuncPtr, check_len},
    player::{CharacterType, Player, roles::RoleNames},
    state::{PlayerIndex, State, StateStoreFields, Step, log},
};
use crate::initialization::Script;
use crate::ui::utils::layout::calc_circle;

#[derive(Clone, Debug, Store, Default)]
pub(crate) struct TempState {
    pub(crate) selected_player: Option<PlayerIndex>,
    pub(crate) curr_change_request: Option<ChangeRequest>,
    pub(crate) selected_players: Vec<PlayerIndex>,
    pub(crate) selected_roles: Vec<RoleNames>,
    pub(crate) currently_acting_player: Option<PlayerIndex>,
}

impl TempState {
    pub(crate) fn clear_selected(&mut self) {
        self.selected_players.clear();
        self.selected_roles.clear();
        self.curr_change_request = None;
        self.selected_player = None;
    }
    pub(crate) fn reset(&mut self) {
        self.selected_players.clear();
        self.selected_roles.clear();
        self.curr_change_request = None;
        self.selected_player = None;
        self.currently_acting_player = None;
    }
}

#[component]
pub(crate) fn GameInterface(
    roles: Vec<RoleNames>,
    player_names: Vec<String>,
    script: Script,
) -> impl IntoView {
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
    
    let apply_cr = move || {
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
                        console_log(format!("ChangeType {:?}", change_type).as_str());
                        console_log(format!("cr: {:#?}", cr).as_str());
                        console_error(format!("Error: {:?}", err).as_str());
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

            console_log(format!("Temp State: {:#?}", temp_state.get()).as_str());
            break;
        }

        (false, applied_cr)
    };

    let next_button = move || {
        let (ret, mut applied_cr) = apply_cr();
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
                            let (ret, app_cr) = apply_cr();
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

                console_log(format!("Next Player is {:?}", next_player).as_str());

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
                            let (ret, app_cr) = apply_cr();
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
            console_log(format!("Applied cr: {}", applied_cr).as_str());
            game_state.update(|gs| gs.next_step());
            if matches!(game_state.read().step, Step::Day) {
                return;
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
                                        crate::engine::player::Alignment::Good => "blue",
                                        crate::engine::player::Alignment::Evil => "red",
                                        crate::engine::player::Alignment::Any => "purple",
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
                            <div class="text-[0.5rem] flex flex-row flex-wrap justify-center items-start absolute w-fit border left-1/2 -translate-x-1/2 top-9/10 ">
                                {move || {
                                    let status_effects = game_state
                                        .with(|gs| gs.get_player(i).status_effects.clone());
                                    status_effects
                                        .iter()
                                        .map(|status_effect| {
                                            let str = status_effect.status_type.to_string();
                                            view! {
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

#[component]
fn Picker_Bar() -> impl IntoView {
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

fn build(change_option: Option<ChangeRequestBuilder>) -> Option<ChangeRequest> {
    match change_option {
        Some(cr) => Some(cr.build()),
        None => None,
    }
}
