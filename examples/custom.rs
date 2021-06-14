use bevy::{core::CorePlugin, prelude::*};
use bevy_osc::{OSCSettings, OSC};

fn main() {
    App::build()
        .add_plugin(CorePlugin)
        .add_plugin(OSC)
        .insert_resource(OSCSettings {
            max_packets: 20,
            addr: "127.0.0.1:50512",
            ..Default::default()
        })
        .run();
}
