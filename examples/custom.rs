use bevy::prelude::*;
use bevy_osc::{Osc, OscEvent, OscSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Osc)
        .insert_resource(OscSettings {
            recv_addr: Some("127.0.0.1:34254"),
            log: false,
            ..Default::default()
        })
        .add_systems(Update, event_listener_system)
        .run();
}

//Make events a type param?
fn event_listener_system(mut events: EventReader<OscEvent>) {
    for my_event in events.read() {
        info!("My custom osc handler: {:?}", my_event.packet);
    }
}
