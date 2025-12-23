use std::sync::LazyLock;
use bevy::{input::mouse::MouseWheel, prelude::*};
use crate::env::{acquire_num, EnvVariable};
use crate::grid::Grid;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (scroll_events, move_camera));
    }
}

#[derive(Component)]
struct Camera;

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((Camera2d, Camera));
}


fn scroll_events(
    mut evr_scroll: MessageReader<MouseWheel>,
    mut projection: Query<&mut Projection, With<Camera>>,
    windows: Query<&Window>,
) {
    use bevy::input::mouse::MouseScrollUnit::*;
    let window = windows.single().unwrap();
    
    let min_scale_x = 32.0 / window.width();
    let min_scale_y = 32.0 / window.height();
    let min_scale = min_scale_x.max(min_scale_y);

    let mut projection = projection.single_mut().unwrap();
    if let Projection::Orthographic(ref mut ortho) = *projection {
        for ev in evr_scroll.read() {
            let unit_scale = match ev.unit {
                Line => 0.1,
                Pixel => 0.001
            };

            ortho.scale *= 1.0 - ev.y * unit_scale;
            ortho.scale = ortho.scale.max(min_scale);
        }
    }
}

const CAMERA_SPEED: LazyLock<f32> = LazyLock::new(|| acquire_num(EnvVariable::CAMERA_SPEED));

fn move_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    grid: Res<Grid>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = query.single_mut().unwrap();

    // Speed relative to zoom level
    let speed = *CAMERA_SPEED * (grid.cell_size() as f32) * time.delta_secs();

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        transform.translation.x += speed;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        transform.translation.y -= speed;
    }
}