use leptos::prelude::*;
use reactive_stores::Store;

use crate::engine::state::{State, StateStoreFields, Step};
use crate::ui::game::{TempState, TempStateStoreFields, UiMode};

#[component]
pub(crate) fn ConsoleShell(children: Children) -> impl IntoView {
    view! {
        <div class="h-screen w-screen flex flex-col bg-chassis text-ink">
            <TopBar />
            <div class="flex-1 flex flex-row min-h-0">{children()}</div>
        </div>
    }
}

#[component]
pub(crate) fn TopBar() -> impl IntoView {
    let game_state = expect_context::<Store<State>>();
    let temp_state = expect_context::<Store<TempState>>();

    let phase_label = move || {
        let step = game_state.step().get();
        match step {
            Step::Start => "Start".to_string(),
            Step::Setup => "Setup".to_string(),
            Step::Day => format!("Day {}", game_state.day_num().get()),
            Step::NightOne | Step::Night => format!("Night {}", game_state.day_num().get()),
        }
    };

    view! {
        <div class="flex items-center justify-between px-4 py-2 border-b border-solid border-divider">
            <span class="font-mono uppercase tracking-widest text-sm">{phase_label}</span>
            <div class="flex items-center gap-2 font-mono text-xs uppercase tracking-widest">
                <span style:color=move || if matches!(temp_state.ui_mode().get(), UiMode::Auto) { "var(--color-ink)" } else { "var(--color-muted)" }>
                    "Auto"
                </span>
                <button
                    class="w-11 h-6 rounded-full relative border border-solid border-seat bg-panel"
                    on:click=move |_| {
                        temp_state.ui_mode().update(|m| {
                            *m = match *m {
                                UiMode::Auto => UiMode::Manual,
                                UiMode::Manual => UiMode::Auto,
                            };
                        });
                    }
                >
                    <span
                        class="absolute top-0.5 size-5 rounded-full bg-white"
                        style:left=move || {
                            if matches!(temp_state.ui_mode().get(), UiMode::Manual) {
                                "calc(100% - 1.375rem)"
                            } else {
                                "0.125rem"
                            }
                        }
                    ></span>
                </button>
                <span style:color=move || if matches!(temp_state.ui_mode().get(), UiMode::Manual) { "var(--color-ink)" } else { "var(--color-muted)" }>
                    "Manual"
                </span>
            </div>
        </div>
    }
}
