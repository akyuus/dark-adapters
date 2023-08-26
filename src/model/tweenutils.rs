use bevy::prelude::{EventReader, NextState, ResMut, Resource, States};
use bevy_tweening::TweenCompleted;
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
