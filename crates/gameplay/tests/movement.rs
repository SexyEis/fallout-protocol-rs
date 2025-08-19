use bevy::input::InputPlugin;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::scene::ScenePlugin;
use bevy_rapier3d::prelude::*;
use gameplay::movement::{MovementPlugin, MovementState, PlayerInput};
use gameplay::player::{Player, PlayerPlugin};

/// A minimal Bevy app setup for testing movement logic.
fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        InputPlugin::default(),
        AssetPlugin::default(),
        ScenePlugin::default(), // Add plugin for SceneSpawner
        RapierPhysicsPlugin::<NoUserData>::default(),
        MovementPlugin,
        PlayerPlugin,
    ));

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
