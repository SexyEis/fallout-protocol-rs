# Protocol Zero - Architecture Overview

*This document outlines the high-level architecture of the Protocol Zero game engine and systems. It will be updated as design decisions are made and implemented.*

## 1. Core Principles

- **Data-Oriented Design:** Using Bevy's ECS, we prioritize data layout and transformation over object-oriented hierarchies.
- **Modularity:** The workspace is divided into loosely-coupled crates, each with a specific domain of responsibility.
- **Clear Ownership:** Systems own their data. Cross-crate communication happens through well-defined public APIs and shared types in the `common` crate.

## 2. Crate Breakdown

- **`engine`**: The heart of the application. Manages the main loop, rendering, physics, audio, and input. It ties all other systems together.
- **`world`**: Handles the voxel world representation, chunk management, and material properties.
- **`gameplay`**: Implements all player-facing mechanics like movement, building, stats, and mission logic.
- **`ai`**: Contains logic for non-player characters, including enemies and companions.
- **`narrative`**: Manages the story state, dialogue, and event triggers.
- **`ui`**: Responsible for all user interfaces, including the HUD, menus, and settings screens.
- **`tools`**: A collection of development tools, such as asset importers and command-line utilities.
- **`common`**: A library of shared types and functions used across the workspace.

## 3. System Interaction (Example: Player shoots a wall)

1.  **`engine` (Input System):** Detects mouse click.
2.  **`gameplay` (Action System):** Interprets the input as a "shoot" action.
3.  **`engine` (Physics System):** Performs a raycast from the camera to find what was hit.
4.  **`world` (Destruction System):** The raycast hits a voxel in a chunk. The system calculates damage based on the weapon and material properties.
5.  **`world` (Voxel System):** The voxel's health is reduced. If it reaches zero, it's removed from the chunk, and the mesh is marked as dirty.
6.  **`engine` (Rendering System):** The `world` crate's mesh generation system is triggered for the dirty chunk, and the new mesh is uploaded to the GPU.
