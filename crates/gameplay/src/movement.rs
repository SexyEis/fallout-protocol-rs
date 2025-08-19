//! Player movement state and logic.
use crate::camera::CameraRig;
use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const BASE_MOVE_SPEED: f32 = 5.0;
const SPRINT_SPEED: f32 = 10.0;

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
    mut player_query: Query<(&mut KinematicCharacterController, &mut Transform), With<Player>>,
    rig_query: Query<&Transform, (With<CameraRig>, Without<Player>)>,
    player_input: Res<PlayerInput>,
    movement_state: Res<State<MovementState>>,
    time: Res<Time>,
) {
    let Ok((mut controller, mut player_transform)) = player_query.get_single_mut() else {
        return;
    };
    let Ok(rig_transform) = rig_query.get_single() else {
        return;
    };

    if player_input.move_direction == Vec2::ZERO {
        controller.translation = Some(Vec3::ZERO);
        return;
    }

    // --- Camera-relative movement ---
    // Get the camera's forward direction on the XZ plane.
    let mut forward: Vec3 = rig_transform.forward().into();
    forward.y = 0.0;
    let forward = forward.normalize();

    // Get the camera's right direction based on the new forward vector.
    let right = Vec3::new(forward.z, 0.0, -forward.x);

    // Calculate the desired movement direction based on player input.
    let desired_move = (forward * player_input.move_direction.y
        + right * player_input.move_direction.x)
        .normalize_or_zero();

    // --- Player rotation ---
    if desired_move.length_squared() > 0.0 {
        player_transform.rotation = Quat::from_rotation_y(desired_move.x.atan2(desired_move.z));
    }

    // --- Apply movement ---
    let speed = match movement_state.get() {
        MovementState::Sprinting => SPRINT_SPEED,
        _ => BASE_MOVE_SPEED,
    };

    // Let Rapier's character controller handle gravity and collisions.
    let movement = desired_move * speed * time.delta_seconds();
    controller.translation = Some(movement);
}

/// Updates the `MovementState` based on whether the player is moving and sprinting.
fn update_movement_state(
    player_input: Res<PlayerInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<MovementState>>,
    current_state: Res<State<MovementState>>,
) {
    let current_state = *current_state.get();

    // Check for stopping
    if player_input.move_direction == Vec2::ZERO {
        if current_state != MovementState::Idle {
            next_state.set(MovementState::Idle);
        }
        return;
    }

    // Player is moving, check for sprint
    let is_sprinting = keyboard_input.pressed(KeyCode::ShiftLeft);

    if is_sprinting {
        if current_state != MovementState::Sprinting {
            next_state.set(MovementState::Sprinting);
        }
    } else if current_state != MovementState::Walking {
        // Not sprinting, so should be walking (if not already)
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
