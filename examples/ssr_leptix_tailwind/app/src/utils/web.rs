
// use bevy_eventwork::{NetworkPacket, NetworkMessage};
// // pub fn serialize_msg(message: NetworkMessage) {
// //     let encoded = match bincode::serialize(&message) {
// //                     Ok(encoded) => encoded,
// //                     Err(err) => {
// //                         error!("Could not encode packet {:?}: {}", message, err);
// //                         continue;
// //                     }
// //                 };
// // }

// // pub fn serialize_msg(message: NetworkMessage) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
// //     bincode::serialize(&message)
// // }

// pub fn deserialize_msg<T>(
//     msg: Vec<u8>,
//     // source: ConnectionId
// ) -> Option<T>
// where
//     T: NetworkMessage,
// {
//     let message = bincode::deserialize::<T>(&msg)
//         .ok()
//         .map(|message| message);
//     message
    
// }