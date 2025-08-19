//! The core game engine, responsible for rendering, physics integration, and scene management.

use bevy::math::primitives::{Cuboid, Rectangle};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use gameplay::GameplayPlugin;
use world::{MaterialId, Voxel}; // Import world data structures

/// Sets up the core engine plugins and resources.
pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Protocol Zero".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins((
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin::default(),
        GameplayPlugin,
    ))
    .insert_resource(ClearColor(Color::rgb(0.1, 0.4, 0.8)))
    .init_resource::<world::WorldData>() // Initialize the world data resource
    .add_systems(Startup, (setup_scene, setup_world)); // Add setup systems
    app
}

/// Sets up the initial 3D scene for visualization.
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(50.0, 50.0))),
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
        mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1_000_000.0, // Adjusted for Bevy 0.13's new lighting model
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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
