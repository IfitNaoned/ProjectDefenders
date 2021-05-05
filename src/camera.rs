use crate::layers::*;
use crate::{app_state::*, map::MAP_SIZE};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::render::camera::RenderLayers;
use bevy_mod_picking::*;

fn camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0., MAP_SIZE as f32, MAP_SIZE as f32)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default())
        .insert(
            RenderLayers::layer(DEBUG_LAYER)
                .with(TILE_LAYER)
                .with(GAME_ENTITY_LAYER),
        );

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0., 7.0, 2.)),
        ..Default::default()
    });
}

pub fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Camera, &mut Transform)>,
    time: Res<Time>,
    _windows: Res<Windows>,
) {
    for (mut _camera, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Q) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            direction -= Vec3::new(0.0, 0.0, 1.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }

        if keyboard_input.pressed(KeyCode::Space) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::LControl) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        let speed = 5.;
        transform.translation += time.delta_seconds() * direction * speed;
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(camera.system()))
            .add_system_set(
                SystemSet::on_update(AppState::Game).with_system(camera_movement.system()),
            );
    }
}
