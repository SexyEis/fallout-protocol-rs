# Protocol Zero - Project Plan

*This document will be maintained by the planning agent (Jules) to track project milestones, risks, and the overall test plan. It will be updated as the project progresses.*

## M0 (Scaffold)

- [x] Workspace + crates scaffolded.
- [x] CI green (fmt/clippy/test/build).
- [x] Engine boot: window, render loop, input abstraction, camera stubs.
- [ ] World stub: chunk types, in-RAM storage, place/mine API.
- [ ] Gameplay stub: movement FSM, build API, inventory interfaces.
- [x] Docs: `plan.md` & `architecture.md` created.

## M1 (Core movement + camera)

- [ ] Capsule controller (Rapier).
- [ ] Sprint, slide, mantle mechanics.
- [ ] 1st/3rd person camera toggle.
- [ ] Basic test scene.

## M2 (Voxel world + building)

- [ ] Chunk I/O and persistence.
- [ ] Place/mine operations.
- [ ] Simple explosions.
- [ ] Build menu MVP.

## M3 (Progression/inventory/story stub)

- [ ] Separate mission/stash inventories.
- [ ] Airdrop system.
- [ ] Story flag and mission runner stubs.
