use crate::utils::utilresources::WindowScaleFactor;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{
    Commands, Component, Entity, EventReader, Query, Res, Sprite, Text, Vec2, With,
};
use bevy::window::WindowResized;

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
pub struct ScalableSpriteComponent {
    pub base_width: f32,
    pub base_height: f32,
}

#[derive(Component)]
pub struct ScalableTextComponent {
    pub base_size: f32,
}

pub fn resize_sprite_system(
    mut q: Query<(&mut Sprite, &ScalableSpriteComponent)>,
    resize_reader: EventReader<WindowResized>,
    scale_factor: Res<WindowScaleFactor>,
) {
    if resize_reader.is_empty() {
        return;
    }

    for (mut sprite, resizable_sprite_dimensions) in q.iter_mut() {
        let (base_width, base_height) = (
            resizable_sprite_dimensions.base_width,
            resizable_sprite_dimensions.base_height,
        );
        sprite.custom_size = Some(Vec2::new(
            base_width * scale_factor.0,
            base_height * scale_factor.0,
        ))
    }
}

pub fn resize_text_system(
    mut q: Query<(&mut Text, &ScalableTextComponent)>,
    resize_reader: EventReader<WindowResized>,
    scale_factor: Res<WindowScaleFactor>,
) {
    if resize_reader.is_empty() {
        return;
    }

    for (mut text, scalable_text) in q.iter_mut() {
        for section in text.sections.iter_mut() {
            section.style.font_size = scalable_text.base_size * scale_factor.0;
        }
    }
}
