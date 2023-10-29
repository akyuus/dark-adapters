use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::{
    AssetServer, Color, Entity, Font, FromWorld, Handle, Image, NextState, Query, ResMut, Resource,
    Sprite, Transform, Vec3, With, Without, World,
};
use bevy_tweening::lens::{SpriteColorLens, TransformScaleLens};
use bevy_tweening::{Animator, EaseFunction, EaseMethod, Tracks, Tween};

use crate::modes::pause::pausemenucard::{CardTween, PauseMenuCardType, PauseMenuText};
use crate::modes::pause::pausemode::PauseMenuState;
use crate::modes::sharedassets::shared::FontAssets;
use crate::utils::tweenutils::{ExitTweenValues, RotatePauseMenuCardLens, TransformZValueLens};
use crate::utils::utilresources::WindowScaleFactor;

pub const PAUSE_BUTTON_CARD_WIDTH: f32 = 194.;
pub const PAUSE_BUTTON_CARD_HEIGHT: f32 = 114.;

const CARD_ROTATION_DURATION: f32 = 0.35;

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Copy, Clone)]
pub enum RotationDirection {
    Clockwise,
    Counterclockwise,
}

#[derive(Resource)]
pub struct PauseMenuCardTracker {
    pub cards: [Entity; 5],
    pub text_nodes: [Entity; 5],
    pub anchor_point: Vec3,
    pub angles: [f32; 5],
    pub transforms: [Transform; 5],
    pub colors: [Color; 5],
    pub image_handle: Handle<Image>,
    pub font_handle: Handle<Font>,
}

impl FromWorld for PauseMenuCardTracker {
    fn from_world(world: &mut World) -> Self {
        //region setting arrays
        let mut angles = [0_f32; 5];
        let mut transforms = [Transform::default(); 5];
        let mut colors = [Color::NONE; 5];
        let scale_factor = world.get_resource::<WindowScaleFactor>().unwrap().0;
        for i in 0..5 {
            // idk why i have to do it like this
            let z_value = if i == 2 {
                12.0
            } else {
                9.0 - 4.0 * (2.0 - i as f32).abs()
            };
            let scale_value = if i == 2 {
                1.0
            } else {
                1.0 - 0.15 * (2.0 - i as f32).abs()
            };
            let angle = (2.0 - i as f32) * PI / 20.0;
            let color = if i == 2 {
                Color::WHITE
            } else {
                let tint = 0.85 - 0.1 * ((2.0 - i as f32).abs()).powi(2);
                Color::rgb(tint, tint, tint)
            };
            angles[i] = angle;
            colors[i] = color;
            let card_transform = Transform::from_xyz(
                (PAUSE_BUTTON_CARD_WIDTH / 2.0 - 20.0) * scale_factor,
                0.,
                z_value,
            )
            .with_scale(Vec3::new(scale_value, scale_value, 1.0));
            transforms[i] = card_transform;
        }
        //endregion

        // we can load the image here, but the font comes from FontAssets
        let font_assets = world.get_resource::<FontAssets>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        Self {
            cards: [Entity::from_raw(7777777); 5],
            text_nodes: [Entity::from_raw(7777777); 5],
            anchor_point: Vec3::ZERO,
            angles,
            transforms,
            colors,
            font_handle: font_assets.ui_font.clone(),
            image_handle: asset_server.load("pause/card.png"),
        }
    }
}

impl PauseMenuCardTracker {
    pub fn rotate(
        &mut self,
        rotation_direction: RotationDirection,
        card_query: &mut Query<
            (
                &mut Animator<Transform>,
                &mut Animator<Sprite>,
                &mut Transform,
            ),
            (With<PauseMenuCardType>, Without<PauseMenuText>),
        >,
        text_query: &mut Query<&mut Transform, With<PauseMenuText>>,
        exit_tween_values: &mut ExitTweenValues<CardTween>,
        next_state: &mut ResMut<NextState<PauseMenuState>>,
    ) {
        next_state.set(PauseMenuState::RotatingCard);
        // 0 -> 4, 1 -> 0, 2 -> 1, 3 -> 2, 4 -> 3
        // tween z-values and scales
        for (i, (mut animator_t, mut animator_s, transform)) in card_query
            .get_many_mut(self.cards)
            .unwrap()
            .into_iter()
            .enumerate()
        {
            if i == 0 && rotation_direction == RotationDirection::Counterclockwise {
                // we don't actually need to do anything except the angle tween
                *animator_t = Animator::new(
                    Tween::new(
                        EaseFunction::ExponentialInOut,
                        Duration::from_secs_f32(CARD_ROTATION_DURATION),
                        RotatePauseMenuCardLens {
                            pivot: self.anchor_point,
                            start_transform: *transform,
                            start: 0.,
                            end: (self.angles[4] - self.angles[0]) + 2.0 * PI,
                        },
                    )
                    .with_completed_event(i as u64),
                );
                continue;
            }

            if i == 4 && rotation_direction == RotationDirection::Clockwise {
                // we don't actually need to do anything except the angle tween
                *animator_t = Animator::new(
                    Tween::new(
                        EaseFunction::ExponentialInOut,
                        Duration::from_secs_f32(CARD_ROTATION_DURATION),
                        RotatePauseMenuCardLens {
                            pivot: self.anchor_point,
                            start_transform: *transform,
                            start: 0.,
                            end: (self.angles[0] - self.angles[4]) - 2.0 * PI,
                        },
                    )
                    .with_completed_event(i as u64),
                );
                continue;
            }

            let new_index = match rotation_direction {
                RotationDirection::Clockwise => i + 1,
                RotationDirection::Counterclockwise => i - 1,
            };

            let mut text_transform = text_query.get_mut(self.text_nodes[i]).unwrap();

            text_transform.translation.z = self.transforms[new_index].translation.z;

            let z_tween = Tween::new(
                EaseMethod::Linear,
                Duration::from_secs_f32(CARD_ROTATION_DURATION),
                TransformZValueLens {
                    start: transform.translation.z,
                    end: self.transforms[new_index].translation.z,
                },
            )
            .with_completed_event(i as u64);
            let angle_tween = Tween::new(
                EaseMethod::Linear,
                Duration::from_secs_f32(CARD_ROTATION_DURATION),
                RotatePauseMenuCardLens {
                    pivot: self.anchor_point,
                    start_transform: *transform,
                    start: 0.,
                    end: (self.angles[new_index] - self.angles[i]),
                },
            )
            .with_completed_event(i as u64);
            let scale_tween = Tween::new(
                EaseMethod::Linear,
                Duration::from_secs_f32(CARD_ROTATION_DURATION),
                TransformScaleLens {
                    start: Vec3::new(self.transforms[i].scale.x, self.transforms[i].scale.y, 1.0),
                    end: Vec3::new(
                        self.transforms[new_index].scale.x,
                        self.transforms[new_index].scale.y,
                        1.0,
                    ),
                },
            )
            .with_completed_event(i as u64);

            let color_tween = Tween::new(
                EaseMethod::Linear,
                Duration::from_secs_f32(CARD_ROTATION_DURATION),
                SpriteColorLens {
                    start: self.colors[i],
                    end: self.colors[new_index],
                },
            )
            .with_completed_event(i as u64);

            let combined = Tracks::new([angle_tween, scale_tween, z_tween]);
            *animator_t = Animator::new(combined);
            *animator_s = Animator::new(color_tween);
        }
        // 4 * 4 + 1
        exit_tween_values.max = 17;
        match rotation_direction {
            RotationDirection::Clockwise => {
                self.cards.rotate_right(1);
                self.text_nodes.rotate_right(1);
            }
            RotationDirection::Counterclockwise => {
                self.cards.rotate_left(1);
                self.text_nodes.rotate_left(1);
            }
        }
    }
}
