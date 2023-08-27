use bevy::prelude::{EventReader, NextState, Quat, ResMut, Resource, States, Transform};
use bevy_tweening::{Lens, TweenCompleted};
use std::marker::PhantomData;

#[derive(Resource, Default)]
pub struct ExitTweenValues<T: Default + Sized + Send + Sync + 'static> {
    _phantom: PhantomData<T>,
    pub count: u16,
    pub max: u16,
}

impl<T: Default + Sized + Send + Sync + 'static> ExitTweenValues<T> {
    pub fn step_state_when_tweens_completed<U: States + Copy>(
        next: U,
    ) -> impl FnMut(EventReader<TweenCompleted>, ResMut<ExitTweenValues<T>>, ResMut<NextState<U>>)
    {
        move |mut event_reader: EventReader<TweenCompleted>,
              mut exit_tween_values: ResMut<ExitTweenValues<T>>,
              mut next_state: ResMut<NextState<U>>| {
            for _ in event_reader.iter() {
                exit_tween_values.count += 1;
            }

            if exit_tween_values.count == exit_tween_values.max {
                next_state.set(next);
                exit_tween_values.count = 0;
            }
        }
    }
}

pub struct PreserveQuatRotateYLens {
    pub start_quat: Quat,
    pub start: f32,
    pub end: f32,
}

impl Lens<Transform> for PreserveQuatRotateYLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let angle = (self.end - self.start).mul_add(ratio, self.start);
        target.rotation = Quat::from_rotation_y(angle) * self.start_quat;
    }
}