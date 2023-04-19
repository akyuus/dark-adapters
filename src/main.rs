use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::texture::ImageSampler;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .run();
}

/// sets up a scene with textured entities
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("img/dun/wall1.png");

    if let Some(texture) = images.get_mut(&texture_handle) {
        texture.sampler_descriptor = ImageSampler::linear();
    }

    let aspect = 1.0;
    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 1.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * aspect,
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // textured quad - normal
    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 1.0, -1.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(-0.5, 1.0, -0.5)
            .with_rotation(Quat::from_rotation_y(PI / 2.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.5, 1.0, -0.5)
            .with_rotation(Quat::from_rotation_y(-PI / 2.0)),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.0, 2.0)
            .looking_at(Vec3::new(0.0, 1.0, -1.0), Vec3::Y),
        ..default()
    });
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut translate_diff = Vec3::ZERO;
    let mut rotate_diff = 0_f32;
    if keyboard_input.just_pressed(KeyCode::Up) {
        translate_diff = Vec3::NEG_Z;
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        translate_diff = Vec3::Z;
    } else if keyboard_input.just_pressed(KeyCode::Left) {
        rotate_diff = PI / 2.0;
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        rotate_diff = -PI / 2.0;
    }

    for mut transform in &mut query {
        transform.translation += translate_diff;
        transform.rotate_y(rotate_diff);
    }
}
