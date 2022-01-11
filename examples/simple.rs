use bevy::prelude::*;
use bevy_osc::Osc;

// Uses default behavior, logs the 5 latest osc messages
fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugin(Osc).run();
}
