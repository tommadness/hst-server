use simple_websockets::{Event, Responder};
use std::collections::HashMap;

fn main() {
    // listen for WebSockets on port 8069:
    let event_hub = simple_websockets::launch(8069).expect("failed to listen on port 8069");
    // map between client ids and the client's `Responder`:
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    loop {
        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                println!("A client connected with id #{}", client_id);
                // add their Responder to our `clients` map:
                clients.insert(client_id, responder);
            }
            Event::Disconnect(client_id) => {
                println!("Client #{} disconnected.", client_id);
                // remove the disconnected client from the clients map:
                clients.remove(&client_id);
            }
            Event::Message(client_id, message) => {
                println!(
                    "Received a message from client #{}: {:?}",
                    client_id, message
                );

                for (_, responder) in &clients {
                    responder.send(message.clone());
                }
            }
        }
    }
}
