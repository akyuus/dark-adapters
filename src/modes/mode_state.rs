use bevy::prelude::{States, SystemSet};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, SystemSet)]
pub enum GameModeState {
    #[default]
    LoadingDungeon,
    InDungeon,
    LoadingBattle,
    InBattle,
}
