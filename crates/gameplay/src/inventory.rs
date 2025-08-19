//! Inventory system interfaces.

/// A placeholder for an item.
pub struct Item;

/// Represents a player's inventory.
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: u32,
}

impl Inventory {
    /// Creates a new, empty inventory with a given capacity.
    pub fn new(capacity: u32) -> Self {
        Self {
            items: Vec::new(),
            capacity,
        }
    }
}
