// use bevy_eventwork::NetworkMessage;

// use bevy_eventwork::NetworkMessage;
// use bevy_eventwork::NetworkPacket;
use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::UseWebSocketReturn;
use leptos_use::use_websocket;
use crate::components::ui::button::Button;
use codee::string::FromToStringCodec;

#[component]
pub fn WsMessages() -> impl IntoView {
    let UseWebSocketReturn {
    ready_state,
    message,
    send,
    open,
    close,
    ..
        } = use_websocket::<String, String, FromToStringCodec>("ws://127.0.0.1:8081");

        let send_message = move |_| {
            send(&"Hello, world!".to_string());
        };

        let status = move || ready_state.get().to_string();

        let connected = move || ready_state.get() == ConnectionReadyState::Open;

        let open_connection = move |_| {
            open();
        };

        let close_connection = move |_| {
            close();
        };

        view! {
            <div class="flex flex-col gap-1">
                <p>"status: " {status}</p>
                <div class="flex items-center gap-1">
                    <Button on_click=send_message disabled=move || !connected()>"Send"</Button>
                    <Button on_click=open_connection disabled=connected>"Open"</Button>
                    <Button on_click=close_connection disabled=move || !connected()>"Close"</Button>
                </div>
                <p class="text-4xl text-blue-400">"Receive message: " {move || format!("{:?}", message.get())}</p>
            </div>
        }
}
