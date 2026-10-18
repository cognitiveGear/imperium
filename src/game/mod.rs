use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum Turn {
    #[default]
    PlayerTurn,
    Paused,
    EnemyTurn,
}
