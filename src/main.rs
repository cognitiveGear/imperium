use bevy::prelude::*;

#[warn(unused_imports)]
pub mod core;
pub mod feature;
pub mod game;
pub mod screen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(core::camera::CameraPlugin)
        .add_plugins(screen::ScreensPlugin)
        .init_state::<screen::Screen>()
        .init_state::<game::Turn>()
        .run();
}
