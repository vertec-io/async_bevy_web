use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;
pub mod components;
pub mod utils;
use components::toggle_theme::ToggleTheme;
use components::chat::WsMessages;

#[component]
pub fn MyApp() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Meta name="description" content="A static website generated using Leptos and Bevy ECS"/>
        <Meta name="color-scheme" content="dark light" />
        <Stylesheet id="start-axum-workspace" href="/pkg/start-axum-workspace.css"/>
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let doubled = move || count()*2;
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="flex flex-col items-center gap-2">
        <h1 class="text-4xl text-blue-500 items-center">"Welcome to Leptos + Bevy!"</h1>
        <button class="bg-primary text-primary-foreground rounded p-2 font-extrabold" on:click=on_click>"Click Me: " {count}</button>
        <ToggleTheme/>
        <div> This is a Div {doubled}</div>
        <WsMessages />
        </div>
    }
}

