use bevy::prelude::{AppBuilder, IntoSystem, Plugin};

pub struct OSC;
impl Plugin for OSC {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system(hello_world_system.system());
    }
}

fn hello_world_system() {
    println!("hello world");
 }