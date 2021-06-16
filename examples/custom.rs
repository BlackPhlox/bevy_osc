use bevy::prelude::*;
use bevy_osc::{Osc, OscEvent, OscSettings};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(Osc)
        .insert_resource(OscSettings {
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
