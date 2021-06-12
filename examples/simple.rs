use bevy::prelude::*;
use bevy_osc::OSC;

fn main() {
   App::build()
       .add_plugin(OSC)
       .run();
}
