use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum ButtonSize {
    #[default]
    Md,
    Sm,
}

#[component]
pub(crate) fn PrimaryButton(
    #[prop(default = false)] danger: bool,
    #[prop(default = false)] full_width: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    on_click: Callback<()>,
    children: Children,
) -> impl IntoView {
    let mut class = String::from(
        "text-center px-4 py-2 rounded text-white font-medium disabled:opacity-40 disabled:cursor-not-allowed",
    );
    if full_width {
        class.push_str(" w-full");
    }
    if danger {
        class.push_str(" bg-evil-solid hover:bg-evil-solid-hover");
    } else {
        class.push_str(" bg-accent hover:bg-accent-hover");
    }

    view! {
        <button class=class disabled=disabled on:click=move |_| on_click.run(())>
            {children()}
        </button>
    }
}

#[component]
pub(crate) fn SecondaryButton(
    #[prop(default = false)] danger: bool,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(default = false)] full_width: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    on_click: Callback<()>,
    children: Children,
) -> impl IntoView {
    let mut class = String::from("text-center rounded border border-solid disabled:opacity-40 disabled:cursor-not-allowed");
    if full_width {
        class.push_str(" w-full");
    }
    match size {
        ButtonSize::Md => class.push_str(" px-3 py-1.5 text-sm"),
        ButtonSize::Sm => class.push_str(" px-2 py-0.5 text-xs"),
    }
    if danger {
        class.push_str(" border-evil text-evil hover:bg-evil/10");
    } else {
        class.push_str(" border-input bg-panel hover:bg-panel-hover disabled:hover:bg-panel");
    }

    view! {
        <button class=class disabled=disabled on:click=move |_| on_click.run(())>
            {children()}
        </button>
    }
}
