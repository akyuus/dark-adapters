use bevy::prelude::{
    Commands, Component, DespawnRecursiveExt, Entity, Query, States, SystemSet, With,
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, SystemSet)]
pub enum GameModeState {
    #[default]
    LoadingDungeon,
    InDungeon,
    LoadingBattle,
    InBattle,
    ExitingBattle, // used for tile transition. there might be a better way to do this
}

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
