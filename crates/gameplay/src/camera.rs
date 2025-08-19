//! A third-person camera that orbits and follows the player.

use crate::player::Player;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

const CAMERA_MIN_DISTANCE: f32 = 2.0;
const CAMERA_MAX_DISTANCE: f32 = 10.0;
const CAMERA_SENSITIVITY: f32 = 0.5;
const SCROLL_SENSITIVITY: f32 = 0.5;

/// The camera's perspective state.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CameraPerspective {
    #[default]
    ThirdPerson,
    FirstPerson,
}

/// A marker component for the main camera.
#[derive(Component)]
pub struct MainCamera;

/// A marker component for the camera's orbital rig.
#[derive(Component)]
pub struct CameraRig {
    /// The focus point of the camera.
    pub focus: Vec3,
    /// The distance from the focus point.
    distance: f32,
    /// The yaw angle (horizontal rotation).
    yaw: f32,
    /// The pitch angle (vertical rotation).
    pitch: f32,
}

impl Default for CameraRig {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            distance: 5.0,
            yaw: 0.0,
            pitch: std::f32::consts::FRAC_PI_4,
        }
    }
}

/// Spawns the camera rig and the main camera.
fn spawn_camera(mut commands: Commands) {
    bevy::log::info!("Spawning camera...");
    let rig = CameraRig::default();
    let transform =
        Transform::from_translation(rig.focus) * Transform::from_rotation(rig.rotation());

    commands
        .spawn((
            rig,
            transform,
            GlobalTransform::default(),
            Name::new("Camera Rig"),
        ))
        .with_children(|parent| {
            parent.spawn((
                MainCamera,
                // Camera3dBundle is commented out for headless mode
                // Camera3dBundle {
                //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, camera_distance)),
                //     ..default()
                // },
                Name::new("Main Camera"),
            ));

            // Add a light to the rig
            // parent.spawn(PointLightBundle {
            //     point_light: PointLight {
            //         intensity: 1_000_000.0,
            //         shadows_enabled: true,
            //         ..default()
            //     },
            //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
            //     ..default()
            // });
        });
}

/// System to update the camera rig's focus to follow the player.
fn update_camera_focus(
    mut rig_query: Query<&mut CameraRig>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    let Ok(mut rig) = rig_query.get_single_mut() else {
        return;
    };

    rig.focus = player_transform.translation;
}

/// System to handle mouse input for orbiting and zooming the camera.
fn handle_camera_input(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut rig_query: Query<&mut CameraRig>,
) {
    let Ok(mut rig) = rig_query.get_single_mut() else {
        return;
    };

    // Orbit
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }
    if delta.length_squared() > 0.0 {
        rig.yaw -= delta.x.to_radians() * CAMERA_SENSITIVITY;
        rig.pitch = (rig.pitch - delta.y.to_radians() * CAMERA_SENSITIVITY)
            .clamp(0.05, std::f32::consts::FRAC_PI_2 - 0.05);
    }

    // Zoom
    let mut scroll = 0.0;
    for event in mouse_wheel_events.read() {
        scroll += event.y;
    }
    if scroll.abs() > 0.0 {
        rig.distance = (rig.distance - scroll * SCROLL_SENSITIVITY)
            .clamp(CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE);
    }
}

/// Toggles the camera perspective between first and third person.
fn toggle_camera_perspective(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_perspective: Res<State<CameraPerspective>>,
    mut next_perspective: ResMut<NextState<CameraPerspective>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        let next = match current_perspective.get() {
            CameraPerspective::ThirdPerson => CameraPerspective::FirstPerson,
            CameraPerspective::FirstPerson => CameraPerspective::ThirdPerson,
        };
        bevy::log::info!("Toggling camera to: {:?}", next);
        next_perspective.set(next);
    }
}

/// System to apply the rig's state to the camera's transform.
fn update_camera_transform(mut rig_query: Query<(&CameraRig, &mut Transform)>) {
    for (rig, mut transform) in rig_query.iter_mut() {
        transform.translation = rig.focus;
        transform.rotation = rig.rotation();
    }
}

/// Updates the camera's local transform to reflect the current perspective (1st/3rd person).
fn update_camera_position(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    rig_query: Query<&CameraRig>,
    perspective: Res<State<CameraPerspective>>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return; };
    let Ok(rig) = rig_query.get_single() else { return; };

    let target_translation = match perspective.get() {
        CameraPerspective::ThirdPerson => Vec3::new(0.0, 0.0, rig.distance),
        // First person view is at the rig's center.
        CameraPerspective::FirstPerson => Vec3::ZERO,
    };

    // Using a small lerp factor for a smooth transition.
    camera_transform.translation = camera_transform.translation.lerp(target_translation, 0.2);
}

impl CameraRig {
    fn rotation(&self) -> Quat {
        Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch)
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CameraPerspective>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    handle_camera_input.run_if(in_state(CameraPerspective::ThirdPerson)), // Only zoom/orbit in 3rd person
                    toggle_camera_perspective,
                    update_camera_focus,
                    update_camera_transform,
                    update_camera_position,
                )
                    .chain(),
            );
    }
}
