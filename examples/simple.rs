use bevy::{
    prelude::*,
};
use bevy_osc::{MyEvent, OSC};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

fn main() {
    App::build()
        .add_plugin(OSC)
        .add_system(event_listener_system.system())
        .run();
}

fn event_listener_system(mut events: EventReader<MyEvent>) {
    println!("Triggered");
    for my_event in events.iter() {
        info!("{}", my_event.message);
    }
}