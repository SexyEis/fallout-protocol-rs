//! Player entity and spawning logic.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
