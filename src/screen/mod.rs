use bevy::prelude::*;

pub mod splash;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum Screen {
    #[default]
    Splash,
    MenuMain,
    MenuSettings,
    MenuNewGame,
    MenuCredits,
    LoadNewGame,
    GameStarmap,
    GameStarsystem,
    GamePlanet,
    GameScience,
    GamePolicy,
    GameDiplomacy,
    GameSettings,
}

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(splash::SplashScreenPlugin);
    }
}
