use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    Overworld,
    #[default]
    Level,
    Paused,
    Menu,
    GameOver,
    GodMode,
}
