# How the EventWork networking system works

We're using the Bevy EventWork crate to handle all of the network requests from the leptos application to the bevy backend. Eventwork sets up a socket server that can accept multiple connections over the network. EventWork doesn't natively implement a websocket connection, it only implements TCP and UDP connections. So connecting to EventWork from the browser isn't possible without a websocket. Luckily, someone wrote a mod for EventWork that implements a Websocket protocol for it (bevy_eventwork_mod_websockets). 

This implementation allows us to connect to the Bevy app via the browser. Since the Leptos app is connected in the browser, this is a key component of the framework. However, this is a key issue in that EventWork's message system is built into the Bevy world integrally, and there is not a way to leverage the serialization and deserialization of messages outside of the Bevy App. We need a way to serialize and deserilaize the same message types on the browser and on the backend, inside of the Bevy world and outside of the Bevy world. 

EventWork implements two types to handle network messages:

```rust
    #[derive(Debug, Event)]
    /// [`NetworkData`] is what is sent over the bevy event system
    ///
    /// Please check the root documentation how to up everything
    pub struct NetworkData<T> {
        source: ConnectionId,
        inner: T,
    }


    #[derive(Serialize, Deserialize)]
    /// [`NetworkPacket`]s are untyped packets to be sent over the wire
    pub struct NetworkPacket {
        kind: String,
        data: Vec<u8>,
    }
```

NetworkData is a struct that is used when deserializing messages passed from a client into the Bevy app, and then used to generate Bevy events. NetworkPacket is an untyped packet of a "kind" string and the "data", which is the binary serialized message type. The Leptos app needs to somehow make use of both of these types in order to send and receive messages from the Bevy world. It will receive NetworkPackets, deserialize them,