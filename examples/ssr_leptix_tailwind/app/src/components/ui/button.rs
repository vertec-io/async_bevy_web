use leptos::{ev::MouseEvent,*};
// use web_sys::MouseEvent;

#[component]
pub fn Button<F,D>(
    children:Children,
    on_click: F, //impl Fn() + 'static,
    // disabled: bool,
    disabled: D// Option<Box<dyn Fn() -> bool>>,
) -> impl IntoView 
where F: Fn(MouseEvent) + 'static,
      D: Fn() -> bool + 'static
{
    view! {
        <button
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            on:click=on_click
            disabled=disabled
        >
            {children()}
        </button>
    }
}

