use leptos::prelude::*;
use reactive_stores::Store;
use tracing::info;

use crate::engine::{
    change_request::ChangeRequest,
    player::roles::RoleNames,
    state::{PlayerIndex, State},
};
use crate::initialization::Script;
use crate::ui::components::{
    AutoResolveDecisionCard, ConsoleShell, DecisionCard, EndOfGameSummary, Grimoire, GrimoireStage,
    LeftRail, LogPanel, ManualModeEditor, StepList,
};
use crate::ui::game_logic::next_button;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum UiMode {
    #[default]
    Auto,
    Manual,
}

#[derive(Clone, Debug, Store, Default)]
pub(crate) struct TempState {
    pub(crate) selected_player: Option<PlayerIndex>,
    pub(crate) curr_change_request: Option<ChangeRequest>,
    pub(crate) selected_players: Vec<PlayerIndex>,
    pub(crate) selected_roles: Vec<RoleNames>,
    pub(crate) currently_acting_player: Option<PlayerIndex>,
    pub(crate) ui_mode: UiMode,
    pub(crate) pending_execution: Option<PlayerIndex>,
}

impl TempState {
    pub(crate) fn clear_selected(&mut self) {
        self.selected_players.clear();
        self.selected_roles.clear();
        self.curr_change_request = None;
        self.selected_player = None;
        self.pending_execution = None;
    }
    pub(crate) fn reset(&mut self) {
        self.selected_players.clear();
        self.selected_roles.clear();
        self.curr_change_request = None;
        self.selected_player = None;
        self.currently_acting_player = None;
        self.pending_execution = None;
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
            <ConsoleShell>
                {move || match temp_state.ui_mode().get() {
                    UiMode::Auto => {
                        view! {
                            <LeftRail>
                                <StepList />
                                <LogPanel />
                            </LeftRail>
                            <GrimoireStage>
                                <Game />
                            </GrimoireStage>
                            <DecisionCard>
                                {move || {
                                    // if state.read().game_over() {
                                    //     view! { <EndOfGameSummary /> }.into_any()
                                    // } else {
                                        view! { <AutoResolveDecisionCard /> }.into_any()
                                    // }
                                }}
                            </DecisionCard>
                        }
                            .into_any()
                    }
                    UiMode::Manual => view! { <ManualModeEditor /> }.into_any(),
                }}
            </ConsoleShell>
        </ErrorBoundary>
    }
    .into_any()
}

#[component]
fn Game() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();

    let game_element: NodeRef<leptos::html::Div> = NodeRef::new();

    view! {
        <div
            class="relative w-full h-full flex justify-center items-center focus:outline-none"
            on:keydown=move |ev| {
                if ev.key() == "Enter" {
                    info!("Next Button Pressed");
                    next_button(game_state, temp_state)
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
            <Grimoire />
        </div>
    }
}
