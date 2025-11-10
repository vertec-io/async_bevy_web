use crate::error_template::{AppError, ErrorTemplate};

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_router::components::*;

pub mod error_template;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/start-axum-workspace.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body>
                <MyApp/>
            </body>
        </html>
    }
}

#[component]
pub fn MyApp() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors/> }.into_view()
                }>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="flex flex-col items-center gap-2">
        <h1 class="text-4xl text-blue-500 items-center">"Welcome to Leptos!"</h1>
        <button class="bg-primary text-primary-foreground rounded p-2 font-extrabold" on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
