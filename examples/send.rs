use bevy::prelude::*;
use bevy_osc::{Osc, OscEvent, OscSender, OscSettings};
use nannou_osc::{Message, Packet, Type};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Osc)
        .insert_resource(OscSettings {
            recv_addr: Some("127.0.0.1:34254"),
            send_addr: Some("127.0.0.1:34254"),
            log: false,
            ..Default::default()
        })
        .add_system(event_listener_system)
        .add_system(event_sender_system)
        .run();
}

//Make events a type param?
fn event_listener_system(mut events: EventReader<OscEvent>) {
    for my_event in events.iter() {
        info!("OSC Package: {:?}", my_event.packet);
    }
}

//Notice: This is system sends a packet roughly every second, look at the fixed_timestep bevy example for a higher accuracy
fn event_sender_system(events: Option<Res<OscSender>>, time: Res<Time>, mut last_time: Local<f64>) {
    let lst = time.seconds_since_startup() - *last_time;
    if lst > 1.0 {
        if let Some(sender) = events {
            let _ = sender.send(Packet::Message(Message {
                addr: "/c".to_string(),
                args: Some(vec![Type::Int(1), Type::Int(2), Type::Int(3)]),
            }));
        }
        *last_time = time.seconds_since_startup();
    }
}
