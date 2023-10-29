use std::time::Duration;

use bevy::prelude::{
    default, BuildChildren, ChildBuilder, Color, Component, Entity, Font, Handle, Image, Quat,
    Sprite, SpriteBundle, Text, Text2dBundle, Transform, Vec2, Vec3,
};
use bevy::text::TextStyle;
use bevy_tweening::lens::{SpriteColorLens, TransformPositionLens};
use bevy_tweening::{Animator, AnimatorState, EaseMethod, Tween};

use crate::modes::pause::pausemenucardtracker::{
    PauseMenuCardTracker, PAUSE_BUTTON_CARD_HEIGHT, PAUSE_BUTTON_CARD_WIDTH,
};
use crate::utils::utilsystems::{ScalableSpriteComponent, ScalableTextComponent};

#[derive(Component, Default)]
pub struct CardTween;

#[derive(Component, Copy, Clone, PartialEq)]
pub enum PauseMenuCardType {
    Resume,
    Exit,
    Options,
}

#[derive(Component, Default)]
pub struct PauseMenuText;

pub fn spawn_cards(
    button_types: &[PauseMenuCardType],
    anchor: &mut ChildBuilder,
    pause_menu_card_tracker: &mut PauseMenuCardTracker,
    scale_factor: f32,
) {
    if button_types.len() != 5 {
        panic!("button type array is not of length 5");
    }

    for (i, &button_type) in button_types.iter().enumerate() {
        let angle = pause_menu_card_tracker.angles[i];
        let mut card_transform = pause_menu_card_tracker.transforms[i];
        card_transform.rotate_around(
            pause_menu_card_tracker.anchor_point,
            Quat::from_rotation_z(angle),
        );
        let (card_e, text_e) = spawn_card(
            button_type,
            anchor,
            card_transform,
            pause_menu_card_tracker.image_handle.clone(),
            pause_menu_card_tracker.colors[i],
            pause_menu_card_tracker.font_handle.clone(),
            scale_factor,
        );
        pause_menu_card_tracker.cards[i] = card_e;
        pause_menu_card_tracker.text_nodes[i] = text_e;
    }
}

fn spawn_card(
    button_type: PauseMenuCardType,
    menu_anchor: &mut ChildBuilder,
    root_transform: Transform,
    card: Handle<Image>,
    color: Color,
    font: Handle<Font>,
    scale_factor: f32,
) -> (Entity, Entity) {
    let card = SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(
                PAUSE_BUTTON_CARD_WIDTH * scale_factor,
                PAUSE_BUTTON_CARD_HEIGHT * scale_factor,
            )),
            ..default()
        },
        texture: card,
        transform: root_transform,
        ..default()
    };

    let mut animator_transform = Animator::new(Tween::new(
        EaseMethod::Linear,
        Duration::from_secs_f32(0.4),
        TransformPositionLens {
            start: default(),
            end: default(),
        },
    ));
    animator_transform.state = AnimatorState::Paused;

    let mut animator_sprite: Animator<Sprite> = Animator::new(Tween::new(
        EaseMethod::Linear,
        Duration::from_secs_f32(0.4),
        SpriteColorLens {
            start: default(),
            end: default(),
        },
    ));
    animator_sprite.state = AnimatorState::Paused;

    let label = match button_type {
        PauseMenuCardType::Resume => "Resume",
        PauseMenuCardType::Exit => "Exit",
        PauseMenuCardType::Options => "Options",
    };
    let mut text_entity = Entity::from_raw(7777777);
    let card_entity = menu_anchor
        .spawn((
            card,
            animator_transform,
            animator_sprite,
            button_type,
            ScalableSpriteComponent {
                base_width: PAUSE_BUTTON_CARD_WIDTH,
                base_height: PAUSE_BUTTON_CARD_HEIGHT,
            },
        ))
        .with_children(|p| {
            let text_bundle = Text2dBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font,
                        font_size: 28.0 * scale_factor,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform {
                    // offset to make it look nicer
                    translation: Vec3::new(
                        5.0 * scale_factor,
                        0.0,
                        root_transform.translation.z + 1.,
                    ),
                    ..default()
                },
                ..default()
            };
            let t = p
                .spawn((
                    text_bundle,
                    PauseMenuText,
                    ScalableTextComponent { base_size: 28.0 },
                ))
                .id();
            text_entity = t;
        })
        .id();
    (card_entity, text_entity)
}
