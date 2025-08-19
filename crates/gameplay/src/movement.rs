//! Player movement state and logic.
use bevy::prelude::*;

/// A state machine for player movement, implemented as a Bevy State.
#[derive(States, Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MovementState>();
    }
}
