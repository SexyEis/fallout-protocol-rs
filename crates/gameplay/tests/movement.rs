use bevy::input::InputPlugin;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::scene::ScenePlugin;
use bevy_rapier3d::prelude::*;
use gameplay::camera::{CameraPlugin, CameraPerspective, CameraRig};
use gameplay::movement::{MovementPlugin, MovementState, PlayerInput};
use gameplay::player::{Player, PlayerPlugin};

/// A minimal Bevy app setup for testing movement logic.
fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin::default(), // Crucially, add this for transform propagation
        InputPlugin::default(),
        AssetPlugin::default(),
        ScenePlugin::default(), // Add plugin for SceneSpawner
        RapierPhysicsPlugin::<NoUserData>::default(),
        MovementPlugin,
        PlayerPlugin,
        CameraPlugin,
    ));

    // Configure Rapier for fixed timestep testing
    app.insert_resource(RapierConfiguration {
        timestep_mode: TimestepMode::Fixed {
            dt: 1.0 / 60.0,
            substeps: 1,
        },
        ..default()
    });

    // Manually add the resources needed by the `spawn_player` system,
    // since we aren't using the full rendering plugins.
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();

    app
}

fn run_updates(app: &mut App, count: usize) {
    for _ in 0..count {
        app.update();
    }
}

#[test]
fn test_movement_state_changes_to_walking() {
    let mut app = setup_test_app();

    // Simulate a key press
    {
        let mut input = app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyW);
    }

    run_updates(&mut app, 5);

    // Check the state
    let state = app.world.resource::<State<MovementState>>();
    assert_eq!(*state.get(), MovementState::Walking);
}

#[test]
fn test_movement_state_reverts_to_idle() {
    let mut app = setup_test_app();

    // Frame 1: Press key and update
    {
        let mut input = app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyW);
    }
    run_updates(&mut app, 5);

    // Check state is Walking
    let state_walking = app.world.resource::<State<MovementState>>();
    assert_eq!(*state_walking.get(), MovementState::Walking);

    // Frame 2: Release key and update
    {
        let mut input = app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.release(KeyCode::KeyW);
    }
    run_updates(&mut app, 5);

    // Check state is Idle
    let state_idle = app.world.resource::<State<MovementState>>();
    assert_eq!(*state_idle.get(), MovementState::Idle);
}

#[test]
fn test_movement_state_changes_to_sprinting() {
    let mut app = setup_test_app();

    // Simulate key presses for moving and sprinting
    {
        let mut input = app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyW);
        input.press(KeyCode::ShiftLeft);
    }

    run_updates(&mut app, 5);

    // Check the state
    let state = app.world.resource::<State<MovementState>>();
    assert_eq!(*state.get(), MovementState::Sprinting, "State should be Sprinting");
}


#[test]
fn test_player_input_resource_is_updated() {
    let mut app = setup_test_app();

    // Simulate multiple key presses
    {
        let mut input = app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyW);
        input.press(KeyCode::KeyA);
    }

    run_updates(&mut app, 5);

    let player_input = app.world.resource::<PlayerInput>();
    let expected = Vec2::new(-1.0, 1.0).normalize();
    assert!(
        (player_input.move_direction - expected).length_squared() < 1e-6,
        "Expected normalized vector"
    );
}

#[test]
fn test_player_transform_is_changed_by_movement() {
    let mut app = setup_test_app();

    // Run a single update to spawn the player
    app.update();

    // Manually spawn a camera rig for the movement system to use.
    app.world.spawn((
        CameraRig::default(),
        Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Store the initial transform
    let initial_transform = {
        let mut player_query = app.world.query_filtered::<&Transform, With<Player>>();
        *player_query.get_single(&app.world).unwrap()
    };

    // Simulate key press
    {
        let mut input = app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyD);
    }

    // Run updates to process movement
    run_updates(&mut app, 5);

    // Get the final transform
    let final_transform = {
        let mut player_query = app.world.query_filtered::<&Transform, With<Player>>();
        *player_query.get_single(&app.world).unwrap()
    };

    // The transform's translation should have changed.
    assert_ne!(
        initial_transform.translation, final_transform.translation,
        "Player's transform should change after movement input"
    );
}

#[test]
fn test_camera_perspective_toggles() {
    let mut app = setup_test_app();

    // Run startup systems to initialize state
    app.update();
    let initial_state = app.world.resource::<State<CameraPerspective>>();
    assert_eq!(*initial_state.get(), CameraPerspective::ThirdPerson, "Initial state should be ThirdPerson");

    // --- First toggle: to FirstPerson ---
    {
        let mut input = app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyV);
    }
    run_updates(&mut app, 1); // Run systems, including the toggle system

    // Check that the state change has been queued in NextState
    let next_state = app.world.resource::<NextState<CameraPerspective>>();
    assert_eq!(next_state.0, Some(CameraPerspective::FirstPerson), "State change to FirstPerson should be queued");

    // Run another update to apply the state change and check the final state
    run_updates(&mut app, 1);
    let first_person_state = app.world.resource::<State<CameraPerspective>>();
    assert_eq!(*first_person_state.get(), CameraPerspective::FirstPerson, "State should be FirstPerson after toggle is applied");


    // --- Second toggle: back to ThirdPerson ---
    {
        // We need to release the key, so that `just_pressed` is fired again on the next press.
        let mut input = app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.release(KeyCode::KeyV);
    }
    run_updates(&mut app, 1); // Run an update to process the release.
    {
        let mut input = app.world.resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyV);
    }
    run_updates(&mut app, 1); // Run systems to queue the next state change

    // Check that the next state change has been queued
    let next_state_2 = app.world.resource::<NextState<CameraPerspective>>();
    assert_eq!(next_state_2.0, Some(CameraPerspective::ThirdPerson), "State change to ThirdPerson should be queued");
}
