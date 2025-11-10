// use leptos::html::html;
use leptos::prelude::*;
use leptos_use::{
    use_color_mode_with_options, use_cycle_list_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn, UseCycleListOptions, UseCycleListReturn,
};
use leptos_meta::*;

#[component]
pub fn ToggleTheme() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            // .custom_modes(vec![
            //     "rust".into(),
            //     "coal".into(),
            //     "navy".into(),
            //     "ayu".into(),
            // ])
            .initial_value(ColorMode::Dark)
            .cookie_enabled(true),
    );

    let UseCycleListReturn { state, next, .. } = use_cycle_list_with_options(
        vec![
            ColorMode::Light,
            ColorMode::Dark,
            // ColorMode::Custom("rust".into()),
            // ColorMode::Custom("coal".into()),
            // ColorMode::Custom("navy".into()),
            // ColorMode::Custom("ayu".into()),
        ],
        UseCycleListOptions::default().initial_value(Some((mode, set_mode).into())),
    );

    view! {
        <Html attr:lang="en" attr:class=move || mode.get().to_string() />
        <button class="bg-primary text-primary-foreground rounded p-2 font-extrabold" on:click=move |_| next()>
        {move || format!("Color Theme: {}", state.get())}
        </button>
    }
}