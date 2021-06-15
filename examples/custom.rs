use bevy::prelude::*;
use bevy_osc::{OscEvent, OscSettings, OSC};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(OSC)
        .insert_resource(OscSettings {
            max_log_packets: 20,
            recv_addr: Some("127.0.0.1:34254"),
            log: false,
            ..Default::default()
        })
        .add_system(event_listener_system.system())
        .run();
}

//Make events a type param?
fn event_listener_system(mut events: EventReader<OscEvent>) {
    for my_event in events.iter() {
        info!("My custom osc handler: {:?}", my_event.packet);
    }
}
