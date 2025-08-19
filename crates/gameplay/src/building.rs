//! Player building logic and APIs.
use bevy::prelude::*;

/// A unique identifier for a material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u32);

/// Represents a material that can be used for building.
#[derive(Debug, Clone)]
pub struct Material {
    pub id: MaterialId,
    pub base_hp: f32,
}

/// Represents a piece that can be built.
#[derive(Component, Debug, Clone)]
pub struct BuildPiece {
    pub kind: BuildPieceKind,
    pub material_id: MaterialId,
    pub health: f32,
}

/// The type of a build piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildPieceKind {
    Wall,
    Floor,
    Ramp,
    Roof,
}

/// An event triggered when a player wants to build something.
#[derive(Event)]
pub struct BuildEvent {
    pub piece: BuildPieceKind,
    pub position: Vec3,
    pub rotation: Quat,
}

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BuildEvent>();
    }
}
