use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

#[derive(Resource, Default)]
struct EulerAngles {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_grab_mode() {
        CursorGrabMode::None => {
            window.set_cursor_grab_mode(CursorGrabMode::Confined);
            window.set_cursor_visibility(false);
        }
        _ => {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
        }
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::Key1) {
            toggle_grab_cursor(window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.037, 2.5, 8.372).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    //we need to zero the y value because the camera look-at is tilting
    //so it fucks up with the forward/backward movement
    forward = forward.normalize();

    let mut left = camera.left();
    left = left.normalize();

    let speed = 3.0;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::LControl) {
        camera.translation -= Vec3::Y * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Space) {
        camera.translation += Vec3::Y * time.delta_seconds() * speed;
    }
}

/// Handles looking around if cursor is locked
fn camera_look(
    mut euler_angles: ResMut<EulerAngles>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        let mut delta_state = euler_angles.as_mut();
        let mut camera_transform = query.single_mut();

        let window_scale = 720.0;
        let sens: f32 = 0.00012;
        for ev in delta_state.reader_motion.iter(&motion) {
            match window.cursor_grab_mode() {
                CursorGrabMode::None => (),
                _ => {
                    let window_scale = 720.0;
                    delta_state.pitch -=
                        (sens * ev.delta.y * window_scale).to_radians();
                    delta_state.yaw -=
                        (sens * ev.delta.x * window_scale).to_radians();
                }
            }

            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);
        }

        // Order is important to prevent unintended roll
        camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
            * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

/// Contains everything needed to add first-person fly camera behavior to your game
pub struct CustomCameraPlugin;
impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EulerAngles>()
            .add_startup_system(camera_spawn)
            .add_startup_system(initial_grab_cursor)
            .add_system(camera_controls)
            .add_system(camera_look)
            .add_system(cursor_grab);
    }
}