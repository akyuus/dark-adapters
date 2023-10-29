use bevy::prelude::{States, SystemSet};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, SystemSet)]
pub enum GameModeState {
    #[default]
    LoadingSharedAssets,
    LoadingDungeon,
    InDungeon,
    LoadingBattle,
    InBattle,
    ExitingBattle, // used for tile transition. there might be a better way to do this
    Paused,
}

impl GameModeState {
    pub fn can_pause(&self) -> bool {
        match self {
            GameModeState::InDungeon => true,
            _ => false,
        }
    }
}
