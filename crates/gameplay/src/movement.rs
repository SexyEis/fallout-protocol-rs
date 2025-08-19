//! Player movement FSM.

/// Represents the different movement states a character can be in.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovementState {
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
