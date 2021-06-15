use bevy::prelude::*;
use bevy_osc::OSC;

// Uses default behavior, logs the 5 latest osc messages
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(OSC)
        .run();
}
