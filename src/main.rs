#![allow(clippy::needless_return)]

use leptos::mount::mount_to_body;
use leptos::{
    leptos_dom::logging::{console_error, console_log},
    prelude::*,
};
use reactive_stores::Store;

mod engine;
mod initialization;
mod scripts;
mod ui;

use crate::engine::change_request::{ChangeRequestBuilder, StateChangeFuncPtr, check_len};
use crate::engine::state::{self, log};
use crate::ui::{InitializationStage, game::GameInterface, setup::*};
use engine::{
    change_request::{ChangeArgs, ChangeRequest, ChangeType},
    player::{CharacterType, Player, roles::RoleNames},
    state::{PlayerIndex, State, StateStoreFields, Step},
};
use initialization::{CharacterTypeCounts, Script, ScriptJson};
use scripts::*;

fn main() {
    // Stack Traces
    console_error_panic_hook::set_once();
    
    // Tracing initialization
    use tracing_wasm::WASMLayerConfigBuilder;
    tracing_wasm::set_as_global_default_with_config(
        WASMLayerConfigBuilder::default()
            .build(),
    );

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let initialization_stage = RwSignal::new(InitializationStage::Start);
    let player_names = RwSignal::new(Vec::<String>::new());
    let roles = RwSignal::new(Vec::<RoleNames>::new());
    let script = RwSignal::new(Script { roles: vec![] });
    provide_context(script);

    // NOTE: Debug only
    //
    let debug: bool = std::env::var("DEBUG")
        .is_ok_and(|val| matches!(&*val.trim().to_lowercase(), "1" | "true" | "t"));
    if debug {
        roles.set(vec![
            RoleNames::Soldier,
            RoleNames::Virgin,
            RoleNames::Monk,
            RoleNames::Slayer,
            RoleNames::ScarletWoman,
            RoleNames::Poisoner,
            RoleNames::Mayor,
            RoleNames::Imp,
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
