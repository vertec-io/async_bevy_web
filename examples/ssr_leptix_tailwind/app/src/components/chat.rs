// use bevy_eventwork::NetworkMessage;

// use bevy_eventwork::NetworkMessage;
// use bevy_eventwork::NetworkPacket; 
use leptos::*;
use leptos_use::core::ConnectionReadyState;
use leptos_use::UseWebsocketReturn;
use leptos_use::use_websocket;
use shared::messages::NetworkMessage;
use shared::messages::NetworkPacket;
use shared::messages::NewChatMessage;
use shared::messages::UserChatMessage;
use crate::components::ui::button::Button;
// use crate::utils::web::{serialize_msg, deserialize_msg};
use shared::messages::{serialize_msg,deserialize_msg};
// use shared::messages::*;

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
        } = use_websocket("ws://127.0.0.1:8081");

        let send_message = move |_| {
            send("Hello, world!");
        };

        let send_byte_message = move |_| {
            let new_chat_message = UserChatMessage {
                                                    message: "Hello, world!".to_string()
                                                    };

            let packet = NetworkPacket {
                kind: UserChatMessage::NAME.to_string(),
                data: serialize_msg(&new_chat_message).unwrap() 
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

        let deserialized_bytes = move || {
            match message_bytes.get() {
                Some(bytes) => match bincode::deserialize::<NetworkPacket>(&bytes) {
                    Ok(packet) => Some(packet),
                    Err(_) => None,                 },
                None => None,             }
        };

        let deserialized_msg = move || {
            match deserialized_bytes() {
                Some(msg) => deserialize_msg::<NewChatMessage>(msg.data),
                None => None,
            }
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
                <p class="text-4xl text-blue-400">"Receive message: " {move || format!("{:?}", deserialized_bytes())}</p>
                <p class="text-4xl text-blue-400">"Receive message: " {move || format!("{:?}", deserialized_msg())}</p>
            </div>
        }
}
