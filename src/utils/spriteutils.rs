use std::cmp::max;

use bevy::prelude::{TextureAtlasSprite, Vec3, Window};
use bevy_tweening::Lens;

pub struct TextureAtlasSpriteLens {
    pub start_index: i8,
    pub end_index: i8,
}

impl Lens<TextureAtlasSprite> for TextureAtlasSpriteLens {
    fn lerp(&mut self, target: &mut TextureAtlasSprite, ratio: f32) {
        let t = (self.end_index - self.start_index) as f32 * ratio;
        target.index = max(self.start_index + t.round() as i8, 0) as usize;
    }
}

pub fn get_top_left_of_window(window: &Window) -> (i32, i32) {
    let window_width = window.resolution.width() as i32;
    let window_height = window.resolution.height() as i32;
    (-window_width / 2, window_height / 2)
}

pub fn get_top_right_of_window(window: &Window) -> (i32, i32) {
    let window_width = window.resolution.width() as i32;
    let window_height = window.resolution.height() as i32;
    (window_width / 2, window_height / 2)
}

pub fn get_bottom_left_of_window(window: &Window) -> (i32, i32) {
    let window_width = window.resolution.width() as i32;
    let window_height = window.resolution.height() as i32;
    (-window_width / 2, -window_height / 2)
}

pub fn get_middle_left_of_window(window: &Window) -> Vec3 {
    let window_width = window.resolution.width();
    Vec3::new(-window_width / 2.0, 0., 0.)
}
