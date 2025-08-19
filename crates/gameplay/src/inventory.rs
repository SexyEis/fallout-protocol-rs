//! Player inventory and stash interfaces.
use bevy::prelude::*;

const DEFAULT_INVENTORY_SLOTS: usize = 10;
const DEFAULT_STASH_SLOTS: usize = 100;

/// A component representing a player's mission-specific inventory.
#[derive(Component, Debug, Clone)]
pub struct Inventory {
    pub items: Vec<Option<ItemId>>,
    pub capacity: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: vec![None; DEFAULT_INVENTORY_SLOTS],
            capacity: DEFAULT_INVENTORY_SLOTS,
        }
    }
}

/// A resource representing the global stash, accessible across missions.
#[derive(Resource, Debug, Clone)]
pub struct Stash {
    pub items: Vec<Option<ItemId>>,
    pub capacity: usize,
}

impl Default for Stash {
    fn default() -> Self {
        Self {
            items: vec![None; DEFAULT_STASH_SLOTS],
            capacity: DEFAULT_STASH_SLOTS,
        }
    }
}

/// A unique identifier for an item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemId(pub u32);

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Stash>();
    }
}
