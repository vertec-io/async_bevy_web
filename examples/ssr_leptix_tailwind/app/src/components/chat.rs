use bevy_eventwork::NetworkMessage;
// use bevy_eventwork::NetworkMessage;
use bevy_eventwork::NetworkPacket;
use leptos::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::UseWebsocketReturn;
use leptos_use::use_websocket;
use crate::components::ui::button::Button;
// use crate::utils::web::{serialize_msg, deserialize_msg};
use shared::messages::{serialize_msg,deserialize_msg};
use shared::messages::*;

#[component]
pub fn WsMessages() -> impl IntoView {
    let UseWebsocketReturn {
    ready_state,
    message,
    message_bytes,
    send,
    send_bytes,
    open,
    close,
    ..
        } = use_websocket("ws://127.0.0.1:4100/ws");

        let send_message = move |_| {
            send("Hello, world!");
        };

        // let send_byte_message = move |_| {
        //     send_bytes(b"Hello, world!\r\n".to_vec());
        // };

        let send_byte_message = move |_| {
            let new_chat_message = NewChatMessage {
                                                    name: "Client".to_string(),
                                                    message: "Hello, world!".to_string()
                                                    };

            let packet = NetworkPacket {
                kind: NewChatMessage::NAME.to_string(),
                data: serialize_msg(&new_chat_message).unwrap()//bincode::serialize(&new_chat_message).unwrap()
            };

            send_bytes(bincode::serialize(&packet).unwrap());
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
                    <Button on_click=send_byte_message disabled=move || !connected()>"Send New Chat Message (bytes)"</Button>
                    <Button on_click=open_connection disabled=connected>"Open"</Button>
                    <Button on_click=close_connection disabled=move || !connected()>"Close"</Button>
                </div>
                <p class="text-4xl text-blue-400">"Receive message: " {move || format!("{:?}", message.get())}</p>
                
                <p class="text-4xl text-blue-400">"Receive message: " {move || format!("{:?}", message_bytes.get())}</p>
                // <p class="text-4xl text-blue-400" >"Receive byte message: " {move || format!("{:?}", message_bytes.get())}</p>
            </div>
        }
}
