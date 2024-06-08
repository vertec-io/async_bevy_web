use bevy_eventwork::NetworkMessage;
use serde::{Serialize, Deserialize};

/////////////////////////////////////////////////////////////////////
// In this example the client sends `UserChatMessage`s to the server,
// the server then broadcasts to all connected clients.
//
// We use two different types here, because only the server should
// decide the identity of a given connection and thus also sends a
// name.
//
// You can have a single message be sent both ways, it simply needs
// to implement both `NetworkMessage" and both client and server can
// send and recieve
/////////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserChatMessage {
    pub message: String,
}

impl NetworkMessage for UserChatMessage {
    const NAME: &'static str = "example:UserChatMessage";
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewChatMessage {
    pub name: String,
    pub message: String,
}

impl NetworkMessage for NewChatMessage {
    const NAME: &'static str = "example:NewChatMessage";
}

// pub fn serialize_msg(message: NetworkMessage) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
//     bincode::serialize(&message)
// }


pub fn serialize_msg<T: NetworkMessage + Serialize>(message: &T) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
    bincode::serialize(message)
}

// pub fn deserialize_msg<T: NetworkMessage + for<'de> Deserialize<'de>>(msg: &[u8]) -> Result<T, Box<bincode::ErrorKind>> {
//     bincode::deserialize(msg).map_err(|e| Box::new(e))
// }

pub fn deserialize_msg<T>(
    msg: Vec<u8>,
    // source: ConnectionId
) -> Option<T>
where
    T: NetworkMessage,
{
    let message = bincode::deserialize::<T>(&msg)
        .ok()
        .map(|message| message);
    message
    
}