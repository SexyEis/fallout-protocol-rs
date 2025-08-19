//! Player movement state and logic.
use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const BASE_MOVE_SPEED: f32 = 5.0;
const GRAVITY: f32 = -9.81;

/// A state machine for player movement, implemented as a Bevy State.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MovementState {
    #[default]
    Idle,
    Walking,
    Running,
    Sprinting,
    Crouching,
    Proning,
    Sliding,
    Diving,
    Climbing,
    Mantling,
    Vaulting,
    Leaning,
}

/// A resource to store the player's processed input.
#[derive(Resource, Default)]
pub struct PlayerInput {
    pub move_direction: Vec2,
}

/// Gathers keyboard input and stores it in the `PlayerInput` resource.
fn gather_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    player_input.move_direction = direction.normalize_or_zero();
}

/// Applies the player's input to the `KinematicCharacterController`.
fn apply_player_movement(
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
    player_input: Res<PlayerInput>,
    time: Res<Time>,
) {
    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let movement = Vec3::new(
        player_input.move_direction.x,
        0.0,
        -player_input.move_direction.y, // Note: Y input is mapped to Z for 3D movement
    ) * BASE_MOVE_SPEED
        * time.delta_seconds();

    // Add gravity to the movement
    let gravity_movement = Vec3::new(0.0, GRAVITY * time.delta_seconds(), 0.0);

    controller.translation = Some(movement + gravity_movement);
}

/// Updates the `MovementState` based on whether the player is moving.
fn update_movement_state(
    player_input: Res<PlayerInput>,
    mut next_state: ResMut<NextState<MovementState>>,
    current_state: Res<State<MovementState>>,
) {
    if player_input.move_direction == Vec2::ZERO {
        if *current_state.get() != MovementState::Idle {
            next_state.set(MovementState::Idle);
        }
    } else if *current_state.get() != MovementState::Walking {
        next_state.set(MovementState::Walking);
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MovementState>()
            .init_resource::<PlayerInput>()
            .add_systems(
                FixedUpdate,
                (
                    gather_player_input,
                    apply_player_movement,
                    update_movement_state,
                )
                    .chain(),
            );
    }
}
