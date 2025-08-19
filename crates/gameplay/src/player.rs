//! Player entity and spawning logic.

use crate::movement::MovementState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const PLAYER_MOVE_SPEED: f32 = 5.0;

/// A marker component for the player entity.
#[derive(Component)]
pub struct Player;

/// Spawns the player character in the world.
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        MovementState::default(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(Capsule3d::new(0.5, 1.0))), // Visual representation
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        // --- Rapier components ---
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(1.0, 0.5), // Physics shape
        KinematicCharacterController::default(),
        GravityScale(1.0),
    ));
}

/// A system to move the player based on keyboard input.
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction -= Vec3::Z;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction += Vec3::Z;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec3::X;
    }

    let movement = direction.normalize_or_zero() * PLAYER_MOVE_SPEED * time.delta_seconds();
    controller.translation = Some(movement);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}
