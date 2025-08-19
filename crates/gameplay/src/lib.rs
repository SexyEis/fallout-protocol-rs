//! Gameplay logic, including movement, stats, inventory, building, and missions.

use bevy::prelude::*;

pub mod building;
pub mod inventory;
pub mod movement;

use building::BuildingPlugin;
use inventory::InventoryPlugin;
use movement::MovementPlugin;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementPlugin, BuildingPlugin, InventoryPlugin));
    }
}
