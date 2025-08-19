//! Voxel/chunk world representation, materials, and destruction logic.

use bevy::prelude::*;
use std::collections::HashMap;

// --- Constants ---

/// The width of a chunk in voxels.
pub const CHUNK_WIDTH: usize = 32;
/// The height of a chunk in voxels.
pub const CHUNK_HEIGHT: usize = 32;
/// The depth of a chunk in voxels.
pub const CHUNK_DEPTH: usize = 32;

// --- Data Structures ---

/// A unique identifier for a material type.
/// For now, `0` is considered "air" or empty.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct MaterialId(pub u16);

/// Defines the properties of a voxel material.
/// This will be expanded later to include things like HP, resistances, etc.
#[derive(Component, Debug, Clone)]
pub struct Material {
    pub name: String,
    pub is_solid: bool,
}

/// Represents a single voxel in the world.
/// It's a wrapper around a material ID for type safety and future expansion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Voxel(pub MaterialId);

/// A chunk of the world, containing a 3D grid of voxels.
/// Chunks are the primary unit of world generation, simulation, and rendering.
#[derive(Component, Debug, Clone)]
pub struct Chunk {
    /// The voxels in this chunk, arranged in [x][y][z] order.
    pub voxels: [[[Voxel; CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
    /// Whether the chunk mesh needs to be rebuilt.
    pub is_dirty: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            // Initialize with air (MaterialId(0))
            voxels: [[[Voxel::default(); CHUNK_DEPTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
            is_dirty: true,
        }
    }
}

// --- World Storage ---

/// A resource that holds all the chunks in the world in a HashMap.
/// The key is the chunk's coordinate in chunk-space (i.e., not world-space).
#[derive(Resource, Debug, Default)]
pub struct WorldData {
    pub chunks: HashMap<IVec3, Chunk>,
}

// --- Coordinate Conversion ---

/// Converts world coordinates (e.g., from a transform) to global voxel coordinates.
pub fn world_to_global_voxel(world_pos: Vec3) -> IVec3 {
    world_pos.floor().as_ivec3()
}

/// Converts global voxel coordinates to the coordinates of the chunk they are in.
pub fn global_voxel_to_chunk_coord(voxel_pos: IVec3) -> IVec3 {
    IVec3::new(
        voxel_pos.x.div_euclid(CHUNK_WIDTH as i32),
        voxel_pos.y.div_euclid(CHUNK_HEIGHT as i32),
        voxel_pos.z.div_euclid(CHUNK_DEPTH as i32),
    )
}

/// Converts global voxel coordinates to local voxel coordinates within a chunk.
pub fn global_voxel_to_local_voxel_coord(voxel_pos: IVec3) -> UVec3 {
    UVec3::new(
        voxel_pos.x.rem_euclid(CHUNK_WIDTH as i32) as u32,
        voxel_pos.y.rem_euclid(CHUNK_HEIGHT as i32) as u32,
        voxel_pos.z.rem_euclid(CHUNK_DEPTH as i32) as u32,
    )
}

// --- API Implementation ---

impl WorldData {
    /// Gets the voxel at the given global voxel coordinate.
    /// Returns `None` if the chunk containing the voxel is not loaded.
    pub fn get_voxel(&self, voxel_pos: IVec3) -> Option<Voxel> {
        let chunk_coord = global_voxel_to_chunk_coord(voxel_pos);
        if let Some(chunk) = self.chunks.get(&chunk_coord) {
            let local_coord = global_voxel_to_local_voxel_coord(voxel_pos);
            Some(
                chunk.voxels[local_coord.x as usize][local_coord.y as usize]
                    [local_coord.z as usize],
            )
        } else {
            None
        }
    }

    /// Sets the voxel at the given global voxel coordinate.
    ///
    /// If the chunk for this voxel doesn't exist, it will be created.
    /// This function marks the modified chunk as dirty.
    pub fn set_voxel(&mut self, voxel_pos: IVec3, voxel: Voxel) {
        let chunk_coord = global_voxel_to_chunk_coord(voxel_pos);
        let chunk = self.chunks.entry(chunk_coord).or_default();
        let local_coord = global_voxel_to_local_voxel_coord(voxel_pos);

        chunk.voxels[local_coord.x as usize][local_coord.y as usize][local_coord.z as usize] =
            voxel;
        chunk.is_dirty = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_to_global_voxel() {
        assert_eq!(
            world_to_global_voxel(Vec3::new(0.5, 1.5, 2.5)),
            IVec3::new(0, 1, 2)
        );
        assert_eq!(
            world_to_global_voxel(Vec3::new(-0.5, -1.5, -2.5)),
            IVec3::new(-1, -2, -3)
        );
    }

    #[test]
    fn test_global_voxel_to_chunk_coord() {
        // Positive coordinates
        assert_eq!(
            global_voxel_to_chunk_coord(IVec3::new(0, 0, 0)),
            IVec3::new(0, 0, 0)
        );
        assert_eq!(
            global_voxel_to_chunk_coord(IVec3::new(31, 31, 31)),
            IVec3::new(0, 0, 0)
        );
        assert_eq!(
            global_voxel_to_chunk_coord(IVec3::new(32, 32, 32)),
            IVec3::new(1, 1, 1)
        );

        // Negative coordinates
        assert_eq!(
            global_voxel_to_chunk_coord(IVec3::new(-1, -1, -1)),
            IVec3::new(-1, -1, -1)
        );
        assert_eq!(
            global_voxel_to_chunk_coord(IVec3::new(-32, -32, -32)),
            IVec3::new(-1, -1, -1)
        );
        assert_eq!(
            global_voxel_to_chunk_coord(IVec3::new(-33, -33, -33)),
            IVec3::new(-2, -2, -2)
        );
    }

    #[test]
    fn test_global_voxel_to_local_voxel_coord() {
        // Positive coordinates
        assert_eq!(
            global_voxel_to_local_voxel_coord(IVec3::new(0, 0, 0)),
            UVec3::new(0, 0, 0)
        );
        assert_eq!(
            global_voxel_to_local_voxel_coord(IVec3::new(31, 31, 31)),
            UVec3::new(31, 31, 31)
        );
        assert_eq!(
            global_voxel_to_local_voxel_coord(IVec3::new(32, 32, 32)),
            UVec3::new(0, 0, 0)
        );

        // Negative coordinates
        assert_eq!(
            global_voxel_to_local_voxel_coord(IVec3::new(-1, -1, -1)),
            UVec3::new(31, 31, 31)
        );
        assert_eq!(
            global_voxel_to_local_voxel_coord(IVec3::new(-32, -32, -32)),
            UVec3::new(0, 0, 0)
        );
        assert_eq!(
            global_voxel_to_local_voxel_coord(IVec3::new(-33, -33, -33)),
            UVec3::new(31, 31, 31)
        );
    }

    #[test]
    fn test_set_and_get_voxel() {
        let mut world_data = WorldData::default();
        let voxel_pos = IVec3::new(10, 20, 30);
        let voxel_to_set = Voxel(MaterialId(42));

        // 1. Get from empty world
        assert_eq!(world_data.get_voxel(voxel_pos), None);

        // 2. Set a voxel
        world_data.set_voxel(voxel_pos, voxel_to_set);

        // 3. Get the voxel back
        assert_eq!(world_data.get_voxel(voxel_pos), Some(voxel_to_set));

        // 4. Check if chunk was created and is dirty
        let chunk_coord = global_voxel_to_chunk_coord(voxel_pos);
        let chunk = world_data.chunks.get(&chunk_coord).unwrap();
        assert!(chunk.is_dirty);

        let local_coord = global_voxel_to_local_voxel_coord(voxel_pos);
        assert_eq!(
            chunk.voxels[local_coord.x as usize][local_coord.y as usize][local_coord.z as usize],
            voxel_to_set
        );
    }

    #[test]
    fn test_set_voxel_creates_chunk() {
        let mut world_data = WorldData::default();
        let voxel_pos = IVec3::new(100, 200, 300);
        let chunk_coord = global_voxel_to_chunk_coord(voxel_pos);

        // Chunk should not exist yet
        assert!(!world_data.chunks.contains_key(&chunk_coord));

        // Set a voxel, which should create the chunk
        world_data.set_voxel(voxel_pos, Voxel(MaterialId(1)));

        // Chunk should now exist
        assert!(world_data.chunks.contains_key(&chunk_coord));
    }
}
