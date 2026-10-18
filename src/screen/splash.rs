use bevy::prelude::*;

use super::Screen;

pub struct SplashScreenPlugin;

#[derive(Component)]
struct SplashScreenTag;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Splash), splash_setup);
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon: Handle<Image> = asset_server.load("icons/splash.png");
    commands.spawn((
        DespawnOnExit(Screen::Splash),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        SplashScreenTag,
        children![(
            ImageNode::new(icon),
            Node {
                width: px(200),
                ..default()
            },
        )],
    ));
}
