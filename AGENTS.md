# AGENTS.md — Protocol Zero (Rust)

> **Purpose:** A clear, enforceable spec for the *Protocol Zero* repository. Jules should follow this document for planning, code structure, quality gates, and feature scope.

---

## 1) Project Overview

**Game:** Protocol Zero
**Language/Stack:** Rust (Cargo workspace).
**Engine:** **Bevy** (ECS + renderer via wgpu, UI, input), **Rapier 3D** (physics), **bevy\_kira\_audio** (audio, mixing/effects), **wgpu** backend (D3D12 on Win10).
**Target platform:** **Windows 10 x64 (initially)**. Linux/macOS later as optional targets.

**Core Features (MVP → expand):**

* **Movement:** Omni-movement inspired by Bodycam / BO6 (sprint, slide, dive, vault/mantle, crouch, prone, lean, wall interaction).
* **Perspectives:** Toggle **First** / **Third person**.
* **Graphics:** Realistic PBR materials, dynamic lighting/shadows, postFX.
* **World:** 100% editable/destructible: mine, place, cut, explode (chunk/voxel-based with material stats).
* **Story:** Dynamic/branching (TWD-like) with consequences, events, and emergent missions.
* **Progression:** Discovery-based unlocks (no hard locks), player skill & creativity rewarded.
* **Stats/Training:** Trainable player stats; **100% cap respec** grants permanent meta-bonuses (hidden metatree).
* **Equipment:** Tools/materials/weapons/gear moddable & upgradeable.
* **Dimensions (1–10):** per dimension:

  * 1 **Base World** for building/defense (RtW-style).
  * Multiple **scripted missions** (fixed goals).
  * Multiple **dynamic/event/story missions**.
  * Own material pools per dimension (small chance of next-tier drops).
  * **Scaling:** Player level adapts to dimension/mission strength (anti-carry & anti-easy-farm).
* **Companions/Survivors:** recruitable; squads give stat buffs.
* **Inventories:** Mission inventory **separate**; **Stash** is global (cross-mission). Airdrop access during missions costs materials and slightly reduces end-rewards.

---

## 2) Engine & Tech Choices

* **Game engine:** Bevy (renderer via wgpu, ECS, input, scheduling, UI for runtime menus).
* **Physics:** Rapier 3D (character controller, colliders, queries, fixed timestep).
* **Audio:** bevy\_kira\_audio (3D spatialization, mixers, ducking, snapshots).
* **UI:** Bevy UI for in-game/menu; **bevy\_egui** for developer overlays.
* **File formats:** GLTF 2.0 for models; PNG/BCn for textures; OGG/FLAC for audio.
* **Save/Config:** RON/TOML for settings; bincode + LZ4/Zstd for saves.
* **Scripting:** Rust-first; mission/story logic as data-driven state machines; optional light DSL later.
* **Future net:** (post-M3) QUIC-based (e.g., bevy\_quinnet) if needed.

---

## 3) Repository Standards

* **Toolchain:** Rust stable (MSRV documented).
* **Workspace:** Cargo workspace with cohesive crates (see structure).
* **Quality gates:** `cargo fmt` (stable), `cargo clippy -D warnings`, unit/integration tests.
* **Commits/PRs:** small focused changes; PR description includes What/Why/How/Tests/Risks.
* **Security:** edit only within designated folders.

### Proposed Structure

```
protocol-zero/
├─ Cargo.toml                  # Workspace
├─ crates/
│  ├─ engine/                  # Rendering, physics glue, scene/camera, audio init
│  ├─ world/                   # Voxels/chunks, materials, destruction
│  ├─ gameplay/                # Movement, stats, inventory, build, missions
│  ├─ ai/                      # Enemies/companions, behavior trees/utility
│  ├─ narrative/               # Story state, branching, triggers
│  ├─ ui/                      # Menus, HUD, pause/settings, loading
│  ├─ tools/                   # Importers, CLI, editor stubs
│  └─ common/                  # Math, types, config, utilities
├─ assets/                     # Models, textures, audio (placeholders)
├─ shaders/                    # WGSL/GLSL
├─ scripts/                    # Dev scripts (build/run/hooks)
└─ .github/workflows/          # CI (fmt, clippy, test, build)
```

### Build & Test

* **Build:** `cargo build --workspace --release`
* **Run (temp bin):** `cargo run -p engine --release` until a top-level game bin exists.
* **Checks:** `cargo fmt --all -- --check && cargo clippy --workspace -D warnings && cargo test --workspace`

---

## 4) Agent Roles (Jules)

> Assign each task to a role and keep scope tight.

### A. Planner (first)

* Maintain `docs/plan.md` with milestones, risks, test plan. Continue only after approval.

### B. Engine Programmer (`crates/engine`, `shaders/`)

* Rendering (PBR), camera (1st/3rd), postFX, input abstraction.
* Physics integration (Rapier), collision, grounding, fixed timestep.
* Audio bootstrap (mixers, categories).
* Acceptance: 60 FPS target on mid-range; deterministic update order.

### C. World Systems (`crates/world`)

* Chunk/voxel data, streaming, persistence hooks.
* Edit ops: place/mine/explode; material properties (density/HP/resists).
* Craft composites (e.g., water+sand+stone → concrete, +30% HP vs stone).

### D. Gameplay Systems (`crates/gameplay`)

* **Movement:** sprint/slide/dive/mantle/lean; tunables & physics.
* **Build menu:** fast walls/floors/ramps/roofs + edit shapes.
* **Stats/Training/Respec100:** skill trees + meta perks.
* **Inventories:** mission-isolated; stash global with airdrop cost/reward malus.
* **Missions:** goals, timers, events, fail/success, dimension scaling.

### E. AI & Narrative (`crates/ai`, `crates/narrative`)

* Enemy archetypes per dimension; abilities, spawns, utility-AI.
* Companions/squads & buffs.
* Story state machine, flags, consequences, mission generator.

### F. UI/UX Engineer (`crates/ui`)

* **Main menu**, **loading screen/arm**, **pause menu**, **settings** (graphics/audio/controls/gameplay/accessibility), **HUD**.
* Navigation with gamepad/mouse/keyboard; localization-ready text.
* Save/load slots & confirmations.

### G. Tools/Pipeline (`crates/tools`, `assets/`)

* GLTF importer/validator, asset audit, in-engine dev UI.

### H. CI/QA (`.github/workflows`, tests)

* Lint/format/test pipelines, Windows artifacts/releases.

**Forbidden:** cross-crate mega-refactors without plan/PR; vendoring large binaries; proprietary deps.

---

## 5) System Specification (outline)

### 5.1 Movement & Camera

* **States:** idle/walk/run/sprint/crouch/prone/slide/dive/climb/mantle/vault/lean.
* **Physics:** capsule controller; slope limits; friction; optional stamina.
* **Camera:** 1st/3rd toggle, shoulder offset, obstruction handling, FOV rules.
* **Weapon handling:** ADS stub; recoil placeholder.

### 5.2 World & Destruction

* **Representation:** chunked voxel grid (configurable resolution).
* **Ops:** place/mine; explosive ablation; structural HP.
* **Materials:** base materials per dimension + combination recipes → new stats.

### 5.3 Building

* **Quick build:** walls/floors/ramps/roofs; **edit modes** (holes/triangles/partials).
* **Snapping/Anchors:** grid & anchors; placement preview; cost readout.

### 5.4 Progression

* **Dimensions 1–10:** unique enemies/materials; rare next-tier drops.
* **Level scaling:** matchmaking/mission scaling to prevent carry/farm exploits.
* **Unlocks:** discovery-driven, not gated by hard locks.
* **Respec100:** reset at 100% caps for permanent meta-perks (separate tree).

### 5.5 Inventory & Stash

* **Mission inventory:** isolated; loadout rules per mission.
* **Stash (global):** access via mission airdrops **for material cost** → slight reward reduction.

### 5.6 Companions/Survivors

* **Recruitment:** units/squads with stat buffs; synergy sets.

### 5.7 Enemies

* **Per-dimension unique abilities/resists** (escalating complexity & counters via new materials/builds).

---

## 6) Data Models (sketches)

* `MovementParams { speed, accel, friction, slide_coef, mantle_speed, lean_angle, ... }`
* `Material { id, tier, base_hp, density, resistances, recipe_inputs }`
* `Chunk { id, coord, voxels[], dirty_flags, ... }`
* `BuildPiece { kind: Wall|Floor|Ramp|Roof, material_id, hp, edit_shape }`
* `Inventory { slots[], rules }`  |  `Stash { global_capacity, fees }`
* `Mission { id, type, goals[], rewards, dim, scaling }`
* `StoryState { flags, branches, consequences }`
* `Companion { role, perks[], squad_tag }`

---

## 7) Milestones (proposal)

* **M0 (Scaffold):** workspace + crates, CI green; basic window/render loop; input mapping.
* **M1 (Core movement + camera):** capsule controller, sprint/slide/mantle; 1st/3rd camera; test scene.
* **M2 (Voxel world + building):** chunk I/O, place/mine, simple explosions; build menu MVP.
* **M3 (Progression/inventory/story stub):** separate inventories; stash/airdrop; story flags/mission runner.

---

## 8) CI/QA Rules

* GitHub Actions: format/lint/test/build on PR.
* Each PR: note perf impacts if touching render/physics/world.
* Tests: movement state machine, inventory rules, craft recipes, chunk mutations.

---

## 9) PR Template

**Title:** `<crate>: <short purpose>`
**What:** …
**Why:** …
**How:** (core changes/algorithms)
**Tests:** (which/how run)
**Risks:** (render/physics/save compat etc.)

---

## 10) Starter Tasks (order)

1. Scaffold workspace and crates with `lib.rs`/`mod.rs` & docs.
2. CI: `.github/workflows/ci-win.yml` (fmt/clippy/test/build; cache).
3. Engine boot: window, render loop, input abstraction, camera stubs (1st/3rd).
4. World stub: chunk types, in-RAM storage, place/mine API.
5. Gameplay stub: movement FSM, build API, inventory interfaces.
6. Docs: `docs/plan.md` & `docs/architecture.md`.

---

## 11) Non-Goals (for now)

* No multiplayer before M3.
* No high-end RTX-specific features before stable PBR baseline.
* No huge binary assets in the repo.

---

## 12) Build/Commit Policy & Binary Artifacts (Windows 10)

**Rule:** No commit without a successful local build/tests. On success, the current `.exe` is added via Git LFS **or** uploaded as a CI artifact/release.

### Rules

1. **Pre-commit chain:** `cargo fmt` → `cargo clippy -D warnings` → `cargo test` → `cargo build --release`.
2. **On errors:** block commit; fix; retry.
3. **On success:** copy `target/release/protocol-zero.exe` to `bin/ProtocolZero.exe` and LFS-track it **(default)**. Alternatively prefer CI artifacts/releases to keep the repo slim.

### Git LFS

`.gitattributes` must include:

```
*.exe filter=lfs diff=lfs merge=lfs -text
```

### Hooks (Windows PowerShell)

Configure once:

```
git config core.hooksPath scripts/hooks
```

Create **`scripts/hooks/pre-commit.ps1`**:

```powershell
# scripts/hooks/pre-commit.ps1
$ErrorActionPreference = 'Stop'
& cargo fmt --all
& cargo clippy --workspace -D warnings
& cargo test --workspace
& cargo build --workspace --release
if ($LASTEXITCODE -ne 0) { Write-Error 'Build failed. Commit blocked.'; exit 1 }
$exe = 'target/release/protocol-zero.exe'
if (Test-Path $exe) {
  New-Item -ItemType Directory -Force -Path 'bin' | Out-Null
  Copy-Item $exe 'bin/ProtocolZero.exe' -Force
  if (-not (Test-Path '.gitattributes') -or -not (Select-String -Path '.gitattributes' -Pattern '\.exe' -Quiet)) {
    Add-Content .gitattributes "`n*.exe filter=lfs diff=lfs merge=lfs -text"
    git lfs track "*.exe" | Out-Null
    git add .gitattributes
  }
  git add bin/ProtocolZero.exe
}
```

---

## 13) Enemy Archetypes ("Success formula")

Core roles common in successful games. Each archetype defines **role**, **behavior**, **counter**, **dimension scaling**.

* **Grunt:** generalist; dangerous in packs. *Counter:* cover + precision. *Scale:* accuracy/HP ↑.
* **Rusher:** melee/flanker; forces close fights. *Counter:* traps/control. *Scale:* speed/gap-closers ↑.
* **Tank:** high HP/resists; frontal pressure. *Counter:* explosives/piercing/flank. *Scale:* DR/shockwave.
* **Sniper:** long-range pickoffs; lane control. *Counter:* smoke/build/flank. *Scale:* wallbang/scanner.
* **Support/Healer:** heals/buffs/respawns. *Counter:* focus/interrupt. *Scale:* AoE buffs/totems.
* **Controller/Disruptor:** zones/mines/EMP/stuns. *Counter:* utility clear/timing. *Scale:* multi-charges/chains.
* **Summoner/Spawner:** portals/adds. *Counter:* portal focus. *Scale:* elite adds/spawn rate.
* **Elite/Champion:** modifiers (e.g., explosive death, phase dash).
* **Boss (dimension end):** multi-phase; mechanics + adds + soft enrage; drops substantial next-tier mats.

**Dimensional Variation:** damage types/resists; movement rules (low-g, ice, fog); AI priorities (structures vs players); anti-build pressure.

---

## 14) Example Story (dimension-aware)

**Premise:** A failed energy experiment (**Protocol Zero**) fractures reality into **10 dimensions**. 80–95% of humans vanish—some displaced, some petrified in stasis, many dead. Shards remain, each with rules.

**Why survivors exist:** hardened shelters/blacksites; autonomous AIs kept enclaves alive; dimension drift displaced people; resistance cells/mercs protect resource nodes.

**Factions (samples):** Aegis Collective (tech salvage), Free Scavengers (nomads), Wardens (relic zealots), Continuity Cult (make the split permanent), Blacksite Operatives (agenda).

**Dimensions (sketch):**

1. **Suburban Fracture** (tutorial): ruins, drones/raiders.
2. **Drowned Works:** conductivity hazards, rust, power arcs.
3. **Boreal Rift:** cold, <20 m visibility, beasts.
4. **Ash Belt:** volcanic/corrosive; fire damage common.
5. **Neon Bastion:** exo-city, sentry mechs, hack events.
6. **Hollowworld:** caverns, echo-swarms, cave-ins.
7. **Mirage Steppe:** heat/optical illusions; ghost snipers.
8. **Glass Ocean:** crystal fields; reflection/refraction gameplay.
9. **Chrono Wastes:** time dilation zones.
10. **Nullcrown:** anomaly core; final boss.

**Dynamic mission logic:** choices (e.g., *secure artifact* vs *evacuate survivors*) set flags and spawn event chains (revenge, trade, ambush). Rescues unlock squads/perks. Failures can raise dimension intensity (harder enemies, better drops).

**Sample D1 arc:** M1 "First Sparks" (stabilize reactor **or** shield evac); M2 "Blown Bridge" (salvage route—stealth vs demo). M3 "Core Warden" boss decides if Aegis keeps a foothold in D1.

---

## 15) Performance & RAM Guidelines (Win10)

**Goal:** high FPS with **minimal RAM**.

**Release profile (Cargo.toml):**

```toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"
strip = "symbols"
incremental = false
```

**.cargo/config.toml:**

```toml
[target.x86_64-pc-windows-msvc]
rustflags = [
  "-C", "target-cpu=native",
  "-C", "link-arg=/OPT:REF",
  "-C", "link-arg=/OPT:ICF"
]
```

**Runtime:** data-oriented ECS; SmallVec/arenas; object pools.
World streaming; LOD; on-demand deserialization.
BCn texture compression & mipmaps; streamed audio.
Renderer: limited frames-in-flight; batching/instancing; frustum/occlusion culling.
**Budget:** MVP target < **1 GB RAM** (without huge textures). Telemetry via `tracing` & heap snapshots.

---

## 16) Windows Setup (Win10, MSVC)

* Install **rustup** (stable, `x86_64-pc-windows-msvc`).
* Install **Visual Studio Build Tools** (C++ + **Windows 10 SDK**).
* Install **Git LFS** (`git lfs install`).
* Verify: `rustc -V`, `cargo -V`, `cl.exe` on PATH.
* Build: `cargo build --release` → `target/release/protocol-zero.exe`.

---

## 17) UI/UX: Menus, Loading, Settings, HUD

### 17.1 App States

`AppState = { Boot, MainMenu, Settings, Loading, InGame, PauseMenu, Credits }`
State transitions are explicit; loading screens wrap async asset/world preparation.

### 17.2 Main Menu (keyboard/mouse/gamepad)

* **Continue** / **New Game** / **Load Game** / **Settings** / **Credits** / **Quit**.
* Rotating scene background (lightweight) with simple camera path (locked to 60 FPS cap while in menu).

### 17.3 Loading Screen

* Async asset preloading & world-chunk warmup.
* **Progress bar** + step labels (Assets, Shaders, World, AI, Audio).
* Tips & input hints; fallback if step stalls (graceful messages, no fake 100%).

### 17.4 Settings Menu

**Graphics:** display mode (fullscreen/borderless/windowed), resolution, VSync, FPS cap, FOV slider, motion blur (on/off/amount), film grain (on/off), camera shake (0–100), brightness/gamma, HDR toggle (future), DLSS/FSR toggles (future), texture quality (streaming budget), shadows (off/low/med/high), AO (off/on), SSR (off/on), anisotropic filtering, LOD bias.
**Audio:** master/music/SFX/voice sliders, dynamic range (night mode), output device, channel config (stereo/5.1/7.1), subtitles (on/off) + size, hitmarker/UISFX toggles, ducking for VO.
**Controls:** keybinds (rebindable, conflicts detected), mouse sensitivity & ADS multiplier, invert Y, controller deadzones & curves, toggle vs hold (crouch/aim/sprint), lean on Q/E.
**Gameplay:** difficulty presets, aim assist (controller), auto-pickup toggles, building snap strength, damage numbers toggle.
**Accessibility:** colorblind filters (deuter/protan/tritan), subtitle bg, UI scale (75–150%), high-contrast mode, reduce camera motion.

### 17.5 Pause Menu

Resume / Settings / Controls / Photo Mode (later) / Return to Main Menu.

### 17.6 HUD (MVP)

Minimal crosshair (toggle), health/armor bars, stamina (if enabled), mission goals tracker, quick build wheel, compass, interaction prompts.

### 17.7 QoL Features (target list)

* Safe area calibration; auto-detect best display mode on first run.
* Cloud-safe config format; per-profile save slots.
* Skippable intros; hold-to-skip confirmations for destructive actions.
* Recenter 3rd-person camera; camera collision smoothing.
* Boot-time shader pre-warm; optional reduced load screens on SSDs.
* Minimal-mode renderer for low-spec fallback.
* Screenshot key; photo mode later.
* In-game bug report that collects logs (opt-in, local).

---

## 18) Auto-Improvement Policy (ask before change)

* An **Auto-Improver** routine may detect better defaults (perf, memory, quality) or safer APIs.
* It must **open a proposal issue/PR** titled `Auto-Improver: <subject>` describing: current state, suggested change, risk, expected benefit, how to revert.
* **No automatic changes** are merged without explicit approval.
* For tiny risk-free fixes (typos, dead code), pre-approved category may auto-merge, but still logs a PR.

---

## 19) Engine Configuration & Ticks

* **Fixed physics tick** (e.g., 60 Hz) decoupled from render; interpolation for visuals.
* **Bevy schedules:** Core → FixedUpdate (physics/AI) → Update (gameplay/UI) → Render.
* **Audio buses:** master/music/SFX/VO/UI; category ducking; snapshot presets per state.
* **Asset pipeline:** GLTF import to internal formats; texture streaming budgets; hot-reload in dev.

---

## 20) Performance Budgets & Metrics

* **Frame time:** ≤ 16.6 ms target (60 FPS) in gameplay scenes on mid-range GPU (Win10/D3D12).
* **VRAM:** texture streaming keeps peak under budget configured in settings.
* **RAM:** < 1 GB in MVP (excluding large optional packs).
* **Telemetry:** `tracing` spans for frame stages; periodic memory gauges; optional CSV export in dev.

---

## 21) Windows Packaging

* Post-build copies `protocol-zero.exe` to `bin/ProtocolZero.exe`; embed version/build hash; include `configs/` defaults; write crash logs to `%LOCALAPPDATA%/ProtocolZero/logs`.

---

## 22) Notes for Jules

* **Plan → implement → small PRs.**
* **No unsolicited framework swaps.**
* **Ask only if spec is unclear** (otherwise implement conservatively).
* **Document public APIs** and **add minimal tests**.

---

## 23) Detailed Requirements from the Original Brief (Checklist)

> Use this as a hard acceptance list. Each item must be implemented or stubbed with clear TODOs.

**Movement (Bodycam/BO6 omni-movement):** sprint, slide, dive, vault/mantle, crouch, prone, lean, wall interaction.
**Graphics:** realistic PBR look comparable to Bodycam (materials, lighting, postFX).
**Perspectives:** fully supported **first-person** and **third-person** toggle.
**Editable world:** 100% player-editable (mine/place/explode/etc.) with chunk/voxel backing.
**Story:** dynamic/branching (TWD-like), choices influence events/missions.
**Objectives with freedom:** objective system proposes *what*, player chooses *how* (stealth/build/demo/companion tactics).
**Stats:** trainable player stats with progression and **Respec at 100%** → permanent meta bonuses (hidden meta tree).
**Equipment:** tools/materials/weapons/gear can be modified, upgraded, customized.
**No hard locks:** unlocks by discovery; grinders advance faster, casual players are **not artificially slowed**.
**Dimensions 1–10:** each contains multiple worlds: 1 base world (build/defend), multiple scripted mission worlds, multiple event/story mission worlds.
**Materials per dimension:** unique material pools; **low chance** to drop next-dimension mats (configurable).
**Level scaling:** player level adapts to mission/dimension strength; prevents carries and easy-farm exploits.
**Layouts & survivors:** players can extend their layouts with other characters and **recruit survivors** to fight alongside (improves player stats).
**Survivor groups:** survivors may form groups granting **specific stat boosts**.
**Goal:** finish story; kill each dimension endboss; drops substantial next-dimension materials.
**Mission inventory separation:** mission inventory is separate from global stash.
**Stash access in missions:** via airdrops; costs materials found **in the mission**; slightly reduces final rewards; stash persists across missions.
**Mission availability:** playable missions are offered based on **player level or group level**.
**New enemies each dimension:** with **new/complex abilities**.
**Build menu:** fast walls/floors/ramps/roofs with **edit options** (cutouts, diagonals, partials).
**Material combos:** players can craft better composites (e.g., water+sand+stone → concrete with \~+30% HP vs. stone).
**Windows 10 focus:** build, run, and CI target Win10 x64 initially.
**Compile/commit rule:** compile first, only commit on success; if build fails, fix errors first (pre-commit & CI).
**Enemy base models:** create **base models** for each enemy archetype type common in other games as foundations.
**Main menu + loading + settings:** full UI suite on Bevy UI (see §17).
**Auto-improvements:** allowed to **recommend** automatic improvements but must **ask before changing** (see §18).
**QoL features:** implement target list in §17.7.

---

## 24) Objective System & Player Freedom

* **Objective types:** Reach, Defend, Escort, Retrieve, Build, Scan, Survive, Boss.
* **Multiple solution paths:** every major objective supports at least two approaches (e.g., stealth infiltration vs. demolition entry; pure combat vs. build-and-hold).
* **Tracking:** HUD tracker shows *what* and optional *suggested methods* (toggleable).
* **Scoring:** rewards adapt to chosen method; no punitive scoring for slower but creative approaches.

---

## 25) Mission Gating, Scaling, and Anti-Carry

* **Group Power:** group lobby computes **Recommended Power** from members; mission queues require within ±Bracket (config).
* **Downscaling:** overpowered players get normalized stats for low-tier missions to keep challenge; drop quality adjusts to prevent easy farming.
* **Upscaling:** underpowered players receive survivability cushions (minor) but not enough to trivialize.
* **Event rotation:** dynamic/event missions appear based on story flags and world states.

---

## 26) Materials, Crafting, and Next-Tier Drops

* **Material tiers per dimension:** materials have **tier = dimension index** by default.
* **Next-tier drop chance:** config key `loot.next_tier_drop_rate` (default **0.02** = 2%) applied to eligible nodes/enemies.
* **Composite crafting examples:**

  * `Concrete = Water + Sand + Stone` → **+30% HP** over Stone walls.
  * Future composites defined via `recipes.ron` (data-driven).
* **Material stats:** density, HP, blast resistance, elemental resistances; serialized in `materials.ron`.

---

## 27) Survivors, Squads, and Stat Boosts

* **Recruitment:** survivors discovered in missions or events; require resources/quests.
* **Traits:** simple trait system (e.g., Medic, Engineer, Scout) affecting buffs.
* **Groups/Squads:** forming a squad enables **group buffs** (e.g., +stability for structures, +stamina regen).
* **Contribution:** active survivors can **participate in combat** (AI companions) or **boost passively** at base.
* **Persistence:** survivors live in base world and can be assigned roles.

---

## 28) Enemy Base Models — Content & Tech Specs

* **Deliverables per archetype:**

  * **GLTF** mesh with **LOD0/1/2** (mid-poly budget suitable for 60 FPS).
  * **Rigged skeleton** (Humanoid or custom if necessary), animation clips: idle, move, attack, hit, death; optional special.
  * **Collision shapes** (capsule/convex) + Rapier physics parameters.
  * **PBR textures** (albedo/normal/RMA), budgeted texture sizes; BCn compressed mip chains.
  * **Config**: behavior profile (utility scores), resistances, damage, spawn weights.
* **Naming:** `enemy_<archetype>_v01.gltf`, textures under `assets/textures/enemies/<archetype>/`.

---

## 29) UI/UX Details — Main Menu, Loading "Arm", Settings

### Main Menu Layout

* **Top-level items:** Continue, New Game, Load Game, Settings, Credits, Quit.
* **Visual:** light 3D scene w/ slow camera pan; logo + build hash; safe-area aware.

### Loading Screen

* **Radial "arm" progress indicator** (pie arm) + textual stage (Assets → Shaders → World → AI → Audio).
* Async chunk/asset warmup; cancel-safe before entering gameplay.
* Tips rotate; do not fake progress.

### Settings (Apply/Cancel/Restore Defaults, preview where possible)

* **Graphics:** fullscreen/borderless/windowed, resolution, VSync, FPS cap, FOV, motion blur, film grain, camera shake, brightness/gamma, texture quality (stream budget MB), shadows (off/low/med/high), AO, SSR, anisotropic, LOD bias.
* **Audio:** master/music/SFX/VO/UI sliders, dynamic range (night mode), device select, channel layout, subtitles (on/off/size), ducking.
* **Controls:** full rebinds, mouse sensitivity + ADS multiplier, invert Y, controller curves/deadzones, toggle/hold options, lean on Q/E.
* **Gameplay:** difficulty presets, aim assist (controller), auto-pickup toggles, building snap strength, damage numbers toggle.
* **Accessibility:** colorblind modes, UI scale, high contrast, reduce motion.
* **Save:** settings persist per user profile in `configs/settings.ron`.

---

## 30) Control Defaults (can be rebinded)

* **Movement:** WASD, Sprint=Left Shift, Crouch=C, Prone=Z, Slide=C while Sprint, Dive=Space while Sprint + direction, Jump=Space, Mantle=Space near ledge, Lean=Q/E.
* **Build menu:** B (hold to open radial), rotate Q/E (when in build), edit G.
* **Interact:** F, **Inventory (mission)**=I, **Stash** via airdrop prompt.
* **Camera toggle:** V.
* **Map/Log:** M/J.

---

## 31) Build Menu & Editing — Acceptance

* Place **walls/floors/ramps/roofs** quickly with snap/grid; ghost preview shows cost in materials.
* **Edit operations:** cut window/door, triangular halves, flip/rotate.
* **Costing:** deduct on confirm; refund partial on dismantle (config).
* **Trap/Defense** slots separate from structure pieces (future extension ok).

---

## 32) Inventory Separation & Airdrop Rules

* **Mission inventory:** isolated; items acquired in-mission stay until extract; converts to rewards on completion.
* **Stash (global):** cross-mission; **Airdrop** grants one-time **stash access** in mission for **material cost** (config `airdrop.cost`) and applies a **reward penalty** (config `airdrop.reward_penalty`).
* **UI:** using airdrop shows cost and penalty confirmation.

---

## 33) Compile-then-Commit — Enforcement

* **Pre-commit hook** compiles/tests; blocks on failure.
* On success, copies `.exe` to `bin/ProtocolZero.exe` and tracks via **Git LFS** (or CI artifact).
* CI on Windows builds and uploads artifact; optional release on tags.

---

## 34) Performance & RAM — Hard Targets

* **Gameplay**: ≥60 FPS on mid-range Win10 D3D12.
* **RAM (MVP)**: < 1 GB without high-res packs.
* **World streaming** keeps active chunk memory bounded.
* **Profiling** via `tracing` + frame/mem CSV in dev builds.

---

## 35) Anti-Frustration & Fairness Policies

* No artificial delays/timers that block progress (no hard locks).
* Grinders can accelerate via discovery & skill; relaxed players progress steadily without punitive pacing.
* Matchmaking avoids carry; rewards scale fairly; difficulty options available.

---

## 36) Deliverables for M0–M2 (expanded)

* **M0:** workspace, crates, CI green; `AppState` with Boot/MainMenu/Settings/Loading/InGame/Pause; main menu UI skeleton; settings persistence.
* **M1:** omni-movement core states; 1st/3rd cameras; test map; HUD basics; loading arm works; graphics settings applied at runtime.
* **M2:** voxel world edit (mine/place), basic explosions; build menu + edits; materials file + one composite; enemy archetype base models greyboxed.

---

## 37) Open Config Keys (defaults subject to tuning)

```
loot.next_tier_drop_rate = 0.02
airdrop.cost = { stone: 20, metal: 10 }     # example; data-driven
airdrop.reward_penalty = 0.10               # 10%; data-driven
inventory.mission_size = 32                 # slots; data-driven
build.refund_rate = 0.25                    # partial refund
scaling.power_bracket = 10                  # allowed delta
```

> All values are data-driven in RON files; tune with telemetry.

---

## 38) Out of Scope (confirm later)

* Online co-op/PvP until post-M3.
* Ray-traced features until performance budget is met.
* Ultra-high-res textures in core repo (optional packs only).
