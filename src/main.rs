use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, basic_setup)
        .run();
    println!("Hello, world!");
}

fn basic_setup() {}
