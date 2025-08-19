//! The core game engine, responsible for rendering, physics integration, and scene management.

use bevy::app::App;
use bevy::asset::{AssetPlugin, Assets};
use bevy::input::InputPlugin;
use bevy::log::LogPlugin;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::scene::ScenePlugin;
use bevy::MinimalPlugins;
use bevy_rapier3d::prelude::*;
use gameplay::GameplayPlugin;
use world::{MaterialId, Voxel}; // Import world data structures

/// Sets up the core engine plugins and resources.
pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default()) // For printing info messages
        .add_plugins(AssetPlugin::default()) // For loading assets
        .add_plugins(InputPlugin) // For keyboard input
        .add_plugins(ScenePlugin) // For SceneSpawner resource
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            GameplayPlugin,
        ))
        .init_resource::<world::WorldData>() // Initialize the world data resource
        .init_resource::<Assets<Mesh>>() // Manually init for Rapier
        .init_resource::<Assets<StandardMaterial>>() // Manually init for player spawn
        .add_systems(Startup, setup_world); // Add setup systems
    app
}

/// Sets up the initial 3D scene for visualization.
#[allow(dead_code)] // This is not used in headless mode
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::math::primitives::Rectangle::new(50.0, 50.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                ..default()
            }),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
        Collider::cuboid(25.0, 0.1, 25.0),
    ));

    // Cube (for visual reference)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(bevy::math::primitives::Cuboid::new(1.0, 1.0, 1.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // Light and camera are now spawned via the CameraPlugin
}

/// A startup system to create some initial voxels in the world data.
fn setup_world(mut world_data: ResMut<world::WorldData>) {
    // Create a flat plane of voxels to represent the ground.
    for x in -16..=16 {
        for z in -16..=16 {
            // Use MaterialId(1) which we can define as "stone" later.
            world_data.set_voxel(IVec3::new(x, -1, z), Voxel(MaterialId(1)));
        }
    }
}
