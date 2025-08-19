# AGENTS.md — Protocol Zero (Rust)

> **Ziel:** Diese Datei gibt Jules klare, durchsetzbare Regeln & Kontext für das Repo *Protocol Zero* (Rust) vor. Halte dich strikt an diese Vorgaben.

---

## 1) Projektüberblick

**Spielname:** Protocol Zero
**Sprache/Stack:** Rust (Cargo Workspace), wgpu‑basierte 3D‑Pipeline, ECS (z. B. bevy‑ökosystem) & Rapier für Physik (falls nötig).
**Zielplattformen:** **Windows 10 x64 (vorerst)**. (wgpu auf Direct3D 12; später Linux optional.)

**Kernfeatures (MVP → Ausbau):**

* **Bewegung:** Bodycam/Omni‑Movement‑ähnlich (Sprint, Slide, Dive, Vault/Mantle, Crouch, Prone, Lean, Wall‑Interaction).
* **Perspektive:** First‑ & Third‑Person (umschaltbar).
* **Grafik:** realistisch (PBR‑Materialien, dynamische Beleuchtung, Schatten, PostFX).
* **Welt:** 100 % veränderbar (abbauen/platzieren/sprengen). Chunk‑/Voxel‑basiert mit Materialeigenschaften.
* **Story:** dynamisch/reaktiv (ähnlich TWD; Entscheidungen beeinflussen Missionen/Ereignisse).
* **Progression:** freie Lösungswege statt „Hard‑Locks“. Entdeckungs‑basierte Freischaltungen.
* **Stats/Training:** trainierbare Werte; **Respec nach 100 %** mit dauerhaften Meta‑Boni.
* **Ausrüstung:** Tools/Materialien/Waffen/Ausrüstung modifizier‑ & upgradbar.
* **Dimensionen (1–10):** je Dimension:

  * 1 Basis‑Welt (Base‑Building & Defense, ähnlich „Rette die Welt“).
  * Mehrere **Missionen (skriptet)** mit fixen Zielen.
  * Mehrere **Missionen (dynamisch/event/story‑getriggert)**.
  * Eigene Materialpools pro Dimension (kleine Chance auf Next‑Tier‑Drops).
  * **Skalierung:** Spieler‑Level passt sich Missions/Dims‑Stärke an (Anti‑Carry & Anti‑Farm).
* **Begleiter/Überlebende:** rekrutierbar; Gruppen geben Stat‑Buffs.
* **Inventare:** Missions‑Inventar **separat**; **Lager** ist global (missionsübergreifend).

  * In Missionen: **Airdrop‑Zugriff aufs Lager** gegen Missions‑Materialkosten → leicht reduzierte Abschlussbelohnung.

---

## 2) Repositorystandards

* **Toolchain:** Rust stable (MSRV im Repo dokumentieren).
* **Workspace:** Cargo‑Workspace mit logisch getrennten Crates (siehe Struktur).
* **Code‑Qualität:** `cargo fmt` (stable), `cargo clippy -D warnings`.
* **Tests:** `cargo test` (plus ggf. `nextest`).
* **Commits/PRs:** kleine, fokussierte Änderungen; PR‑Beschreibung mit *Was/Warum/Wie/Getestet*.
* **Security/Permissions:** ändere nur Dateien innerhalb der explizit dafür vorgesehenen Ordner (unten).

### Vorgeschlagene Struktur

```
protocol-zero/
├─ Cargo.toml            # Workspace
├─ crates/
│  ├─ engine/            # Render, Physik, ECS-Glue, Scene/Camera
│  ├─ world/             # Voxel/Chunks, Materialien, Zerstörung
│  ├─ gameplay/          # Movement, Stats, Inventory, Build, Missions
│  ├─ ai/                # Gegner/Verbündete, Verhalten
│  ├─ narrative/         # Story-State, Entscheidungen, Trigger
│  ├─ net/               # (später) Networking/Sync
│  ├─ tools/             # Importer, CLI, Editor-Stubs
│  └─ common/            # Math, Types, Config, Utilities
├─ assets/               # Modelle, Texturen, Sounds (nur Platzhalter im Repo)
├─ shaders/              # WGSL/GLSL
├─ scripts/              # Dev-Skripte (z. B. build/run)
└─ .github/workflows/    # CI (fmt, clippy, test, build)
```

### Build & Test

* **Build:** `cargo build --workspace --release`
* **Run (Beispiel‑bin):** `cargo run -p engine --release` (bis eigenständiges Game‑crate existiert)
* **Check:** `cargo fmt --all -- --check && cargo clippy --workspace -D warnings && cargo test --workspace`

---

## 3) Agent‑Rollen (für Jules)

> **Jules muss jeden Task einer passenden Rolle zuordnen und den Scope strikt einhalten.**

### A. **Planner** (Pflicht zuerst)

* Liefert/aktualisiert `docs/plan.md` mit Meilensteinen, Risiken, Testplan.
* Erst nach Freigabe weiterarbeiten.

### B. **Engine‑Programmer** (`crates/engine`, `shaders/`)

* Rendering (wgpu/PBR), Kamera (1st/3rd), PostFX, Input‑Abstraktion.
* Physik‑Integration (Rapier o. ä.), Kollision, Bodenhaftung.
* Akzeptanzkriterien: 60 FPS Ziel auf Mid‑Range, deterministische Update‑Order, klare Abstraktionen.

### C. **World‑Systems** (`crates/world`)

* Chunk/VOX‑Datenstruktur, Streaming, Persistenz‑Hooks.
* Bearbeitung: platzieren/abbauen/sprengen; Materialeigenschaften (Dichte/HP/Resistenzen).
* Craft‑Komposits (z. B. Wasser+Sand+Stein→Beton, +30 % HP ggü. Stein).

### D. **Gameplay‑Systems** (`crates/gameplay`)

* **Movement:** Sprint/Slide/Dive/Mantle/Lean + Parameter/Physik‑Tuning.
* **Build‑Menü:** schnelle Platzierung Wände/Böden/Rampen, Edit‑Shapes.
* **Stats/Training/Respec100:** skillbare Werte + Meta‑Boni nach 100 %.
* **Inventorys:** missions‑separat; Lager global + Airdrop‑Kosten/Reward‑Malus.
* **Missionslogik:** Ziele, Timer, Events, Fail/Success, Scaling per Dimension.

### E. **AI & Narrative** (`crates/ai`, `crates/narrative`)

* Gegner‑Archetypen je Dimension, Fähigkeiten, Spawns, Aggro/Utility‑AI.
* Überlebende/Begleiter & Gruppen‑Buffs.
* Story‑State‑Machine, Branching via Flags/Consequences, Missions‑Generator.

### F. **Tools/Pipeline** (`crates/tools`, `assets/`)

* Content‑Importer (GLTF), Asset‑Validation, einfache In‑Engine‑Dev‑UI.

### G. **CI/QA** (`.github/workflows`, Tests)

* Lint/Format/Test Pipelines, Artifacts (nightly builds).

**Verbote:** keine großen Refactors über Crate‑Grenzen ohne Plan/PR; kein Vendoring riesiger Binär‑Assets; keine proprietären Abhängigkeiten.

---

## 4) Systemspezifikation (Umriss)

### 4.1 Bewegung & Kamera

* **States:** Idle/Walk/Run/Sprint/Crouch/Prone/Slide/Dive/Climb/Mantle/Vault/Lean.
* **Physik:** Kapsel‑Collider; Hang‑Limit; Reibung; Sprint‑Ausdauer optional.
* **Kamera:** 1st/3rd toggle, Schulter‑Offset, Kollision/Obstruction, FOV‑Regeln.
* **Waffen‑Handling:** ADS, Recoil‑Platzhalter (später).

### 4.2 Welt & Zerstörung

* **Repräsentation:** chunked voxel grid (konfigurierbare Auflösung).
* **Operationen:** platzieren/abbauen; Explosions‑Ablation; Struktur‑HP.
* **Materialsystem:** Basismaterialien je Dimension + Kombinationsrezepte → neue Stats.

### 4.3 Bauen

* **Schnell‑Bau:** Wände/Böden/Rampen/Dächer; **Edit‑Modi** (Dreieck/Öffnung/Teilstücke).
* **Snapping/Anchors:** Raster/Sockelpunkte; Previews; Kostenanzeige.

### 4.4 Progression

* **Dimensionen (1–10):** eigene Gegner/Materialien; seltene Next‑Tier‑Drops.
* **Level‑Scaling:** Missions‑Matchmaking/Skalierung verhindert Carry/Farm‑Exploits.
* **Unlocks:** durch Entdeckung statt Hard‑Locks.
* **Respec100:** nach 100 % Stat‑Cap Reset→permanente Meta‑Perks (separater Tree).

### 4.5 Inventar & Lager

* **Missions‑Inventar:** isoliert; Loadout‑Regeln pro Mission.
* **Lager (global):** Zugriff via Airdrops in Missionen **gegen Materialkosten** → Abschluss‑Reward leicht reduziert.

### 4.6 Begleiter/Überlebende

* **Rekrutieren:** Einheiten/Squads mit Stat‑Buffs; Synergie‑Sets.

### 4.7 Gegner

* **Pro Dimension neuartige Fähigkeiten/Resistenzen** (Skalierung & Konter durch neue Materialien/Builds).

---

## 5) Datenmodelle (Skizzen; keine finalen Namespaces)

> *Nur als Leitplanken; konkrete Typen beim Implementieren dokumentieren.*

* `MovementParams { speed, accel, friction, slide_coef, mantle_speed, lean_angle, ... }`
* `Material { id, tier, base_hp, density, resistances, recipe_inputs }`
* `Chunk { id, coord, voxels[], dirty_flags, ... }`
* `BuildPiece { kind: Wall|Floor|Ramp|Roof, material_id, hp, edit_shape }`
* `Inventory { slots[], rules }`  |  `Stash { global_capacity, fees }`
* `Mission { id, type, goals[], rewards, dim, scaling }`
* `StoryState { flags, branches, consequences }`
* `Companion { role, perks[], squad_tag }`

---

## 6) Meilensteine (Vorschlag)

* **M0 (Scaffold):** Workspace + Crates, CI, lint/test green; Basic window/render loop; Input‑Mapping.
* **M1 (Core Movement + Camera):** Kapsel‑Physik, Sprint/Slide/Mantle; 1st/3rd Kamera; Testszene.
* **M2 (Voxel‑Welt + Bauen):** Chunk‑IO, Platzieren/Abbauen, einfache Explosionslogik; Build‑Menü MVP.
* **M3 (Progression/Inventar/Story‑Stub):** getrennte Inventare; Lager/Airdrop‑Hook; Story‑Flags/Mission‑Runner.

---

## 7) CI/QA‑Regeln

* GitHub Actions: Format/Lint/Test/Build auf PR.
* Jede PR: **Bench/Perf‑Hinweise**, falls Rendering/Physik betroffen.
* Tests für: Movement‑State‑Maschine, Inventar‑Regeln, Craft‑Rezepte, Chunk‑Mutationen.

---

## 8) PR‑Vorlage (von Jules ausfüllen)

**Titel:** `<crate>: <kurzer Zweck>`
**Was:** …
**Warum:** …
**Wie:** (Kern‑Änderungen/Algorithmen)
**Tests:** (Welche, wie ausgeführt)
**Risiken:** (Render/Physik/Save‑Kompatibilität etc.)

---

## 9) Startaufgaben für Jules (in Reihenfolge)

1. **Scaffold:** Cargo‑Workspace & oben genannte Crates mit `lib.rs`/`mod.rs` + Doku‑Kommentare.
2. **CI:** `.github/workflows/rust.yml` (fmt/clippy/test/build; Cache).
3. **Engine‑Boot:** Fenster, Render‑Loop, Input‑Abstraktion, Kamerastub (1st/3rd).
4. **World‑Stub:** Chunk‑Typen, einfache Speicherung im RAM, API für Platzieren/Abbauen.
5. **Gameplay‑Stub:** Movement‑State‑Maschine (ohne Feintuning), Build‑Menü‑API, Inventar‑Interfaces.
6. **Docs:** `docs/plan.md` & `docs/architecture.md` (aktualisiert).

---

## 10) Grenzen & Nicht‑Ziele (vorerst)

* Kein Multiplayer vor M3.
* Keine High‑End‑RTX‑Spezialeffekte vor stabilem PBR‑Baseline.
* Keine gigantischen Binär‑Assets im Repo.

---

## 11) Hinweise für Jules

* **Plan → Umsetzung → kleine PRs.**
* **Keine unaufgeforderten Framework‑Wechsel.**
* **Frage nur, wenn Spezifikation unklar ist** (ansonsten konservativ umsetzen).
* **Dokumentiere öffentliche APIs** und **füge Minimal‑Tests** hinzu.

---

## 12) Build-/Commit-Policy & Binärartefakte (Windows 10)

**Ziel:** Kein Commit ohne erfolgreichen Build/Tests. Bei Erfolg wird die aktuelle `.exe` dem Repo (via Git LFS) hinzugefügt **oder** als CI‑Artefakt/Releases angehängt.

### Regeln

1. **Pre-Commit:** `cargo fmt` → `clippy -D warnings` → `test` → `build --release`.
2. **Bei Fehlern:** Commit blockieren; Fehler beheben; erneut versuchen.
3. **Bei Erfolg:** `target/release/protocol-zero.exe` nach `bin/ProtocolZero.exe` kopieren und **per Git LFS** tracken **(Standard)**. Alternativ nur als CI‑Artefakt/Releases anhängen.

### Git LFS

`.gitattributes` enthält:

```
*.exe filter=lfs diff=lfs merge=lfs -text
```

> **Hinweis:** Große Binärdateien **nicht** ohne LFS committen.

### Hooks (Windows)

1. **Hooks-Pfad setzen (einmalig):**

```
git config core.hooksPath scripts/hooks
```

2. **`scripts/hooks/pre-commit.ps1`** (PowerShell):

```powershell
# scripts/hooks/pre-commit.ps1
$ErrorActionPreference = 'Stop'
& cargo fmt --all
& cargo clippy --workspace -D warnings
& cargo test --workspace
& cargo build --workspace --release
if ($LASTEXITCODE -ne 0) { Write-Error 'Build failed. Commit blockiert.'; exit 1 }
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

> Optional: Für Git for Windows kann ein schlanker `scripts/hooks/pre-commit` (Bash) die PS1 aufrufen.

### CI (GitHub Actions, Windows)

`.github/workflows/ci-win.yml`:

```yaml
name: ci-win
on: [push, pull_request]
jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { toolchain: stable, components: clippy }
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all -- --check
      - run: cargo clippy --workspace -D warnings
      - run: cargo test --workspace --release
      - run: cargo build --workspace --release
      - uses: actions/upload-artifact@v4
        with:
          name: ProtocolZero-win64
          path: target/release/protocol-zero.exe
```

> **Optional Release:** Bei Tags `v*` zusätzlich Release anlegen und `.exe` anhängen.

---

## 13) Gegner-Archtypen ("Erfolgsformel")

Grundbausteine, die in fast allen erfolgreichen Spielen funktionieren. Jeder Archtyp hat klare **Rolle**, **Verhalten**, **Konter** und **Skalierung über Dimensionen**.

* **Grunt (Standardkämpfer):** Allround, in Gruppen gefährlich. *Konter:* Deckung + präzise Treffer. *Skalierung:* Genauigkeit/HP ↑.
* **Rusher (Nahkampf/Flanker):** sprintet, springt über Deckung, zwingt Distanzkämpfe auf. *Konter:* Fallen/Stop‑Effekte. *Skalierung:* Speed/Gap‑Closers ↑.
* **Tank (Bulwark):** hoher HP/Resist, Schild/Front. *Konter:* Sprengstoff/Piercing/Umgehung. *Skalierung:* Schadensreduktion/Schockwelle.
* **Sniper (Pickoff):** lange Reichweite, Sichtlinien kontrollieren. *Konter:* Rauch/Block‑Bau/Flank. *Skalierung:* Through‑Cover‑Shots/Scanner.
* **Support/Healer:** bufft/heilt Verbündete, Respawn‑Beacon. *Konter:* Fokusfeuer/Interrupt. *Skalierung:* AoE‑Buffs, Totems.
* **Controller/Disruptor:** Zonen, Mines, Stuns, Nebel, EMP. *Konter:* Utility‑Clear, Timing. *Skalierung:* Multi‑Charges, Ketteneffekte.
* **Summoner/Spawner:** ruft Adds, Portale. *Konter:* Zielpriorität Portal. *Skalierung:* Elite‑Adds, schneller Spawn‑Takt.
* **Elite/Champion:** Varianten mit 1–2 einzigartigen Modifikatoren (z. B. „Explosive Tod“, „Phasen‑Dash“).
* **Boss (Dim‑Endboss):** mehrphasig, Mechaniken + Adds + Soft‑Enrage. *Belohnung:* hoher Drop‑Anteil Next‑Tier‑Material.

**Dimensionale Variation:**

* **DMG‑Typen & Resistenzen** je Dimension (z. B. Kälte/Feuer/Korro).
* **Bewegungsregeln** (niedrige Gravitation, Rutschflächen, Nebel).
* **KI‑Ziele** (fokussiert auf Strukturen vs. Spieler).
* **Gegenbau‑Druck** (z. B. Anti‑Bau‑Projectiles in höheren Dimensionen).

---

## 14) Beispiel‑Story (angepasst an Dimensionen)

**Prämisse:** Ein fehlgeschlagenes Energie‑Experiment („Protocol Zero“) zerreißt die Realität in **10 Dimensionen**. 80–95 % der Menschen verschwinden: einige **verdrängt** in andere Schichten, andere **versteinert** in Stasis‑Feldern, viele **umgekommen**. Was bleibt, sind Splitterwelten mit eigenen Regeln.

**Warum gibt es Überlebende?**

* **Schutzräume/Blacksites** mit Notfeldgeneratoren.
* **Autonome AIs** hielten Siedlungen am Laufen – bis Ressourcen knapp wurden.
* **Dimensionen‑Drift:** Menschen wurden versetzt, können zurückgeholt werden.
* **Widerstandszellen** und **Söldner** verteidigen Ressourcenpunkte.

**Fraktionen (Beispiele):**

* **Aegis‑Kollektiv:** Technik‑Rettung, priorisiert Reaktoren und Netzknoten.
* **Freie Sammler:** Nomaden, Handelsrouten zwischen Rissen.
* **Wardens:** Relikt‑Hüter; religiöser Eifer gegen Technologie.
* **Continuity‑Kult:** will Trennung dauerhaft machen.
* **Blacksite‑Operative:** kennen Protokoll‑Geheimnisse, verfolgen eigene Agenda.

**Dimensionen (Skizze):**

1. **Vorstadt‑Bruch** (Tutorial): Ruinen, Drohnen/Raider.
2. **Überflutetes Werk**: Rost, Strom, Leitfähigkeits‑Gefahren.
3. **Borealer Riss**: Kälte, Sicht <20 m; Biester.
4. **Aschegürtel**: Vulkanisch, Korrosion; Feuer‑DMG häufig.
5. **Neon‑Bastion**: Exo‑Stadt, Sentry‑Mechs, Hack‑Events.
6. **Hohlwelt**: Kavernen, Echo‑Schwarm, Einsturz‑Risiko.
7. **Mirage‑Steppe**: Hitze/Optik‑Illusionen; Sniper‑Geister.
8. **Glasozean**: Kristallfelder; Reflexion/Beugung als Gameplay.
9. **Chrono‑Ödnis**: Zeitdilations‑Zonen (langsamer/schneller).
10. **Nullkrone**: Kern der Anomalie; Final‑Boss.

**Dynamische Missionslogik:**

* Entscheidungen (z. B. *Artefakt sichern* vs. *Überlebende retten*) setzen **Story‑Flags** und erzeugen **Event‑Ketten** (Rache, Handel, Hinterhalt).
* Gerettete Überlebende eröffnen **neue Squads/Perks**.
* Fehlentscheidungen können **Dimension‑Intensität** erhöhen (härtere Gegner, bessere Drops).

**Beispiel‑Arc D1:**

* *M1:* „Erste Funken“ – Reaktor stabilisieren **oder** Evakuierung schützen.
* *M2:* „Gesprengte Brücke“ – Material bergen; wähle stille Infiltration oder Spreng‑Umweg.
* *M3 (Boss):* „Kernwächter“ – mehrphasiger Kampf; Entscheidung bestimmt, ob Aegis Basis in D1 dauerhaft besteht.

---

## 15) Performance- & RAM‑Leitlinien (Win10)

**Ziele:** Hohe FPS, **minimaler RAM**.

### Build/Compiler

`Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"
strip = "symbols"
incremental = false
```

`.cargo/config.toml`:

```toml
[target.x86_64-pc-windows-msvc]
rustflags = [
  "-C", "target-cpu=native",
  "-C", "link-arg=/OPT:REF",
  "-C", "link-arg=/OPT:ICF"
]
```

### Runtime

* **ECS & Datenorientierung:** sparsame Komponenten, **SmallVec**/Arenen, Objekt‑Pools.
* **Weltstreaming:** Chunks um Spieler, LOD, **On‑Demand‑Deserialisierung**.
* **Assets:** BCn‑Kompression, Mipmaps, Streaming‑Audio.
* **Renderer:** begrenzte Frames‑in‑Flight, Batch/Instancing, frustum/occlusion culling.
* **Speicherbudgets:** Ziel < **1 GB** Arbeitsspeicher im MVP (ohne große Texturen); Telemetrie `tracing` + Heapsnapshots.

---

## 16) Windows‑Setup (Win10, MSVC)

* Installiere **Rustup** (stable, `x86_64-pc-windows-msvc`).
* Installiere **Visual Studio Build Tools** (C++ Build Tools + **Windows 10 SDK**).
* Installiere **Git LFS** und aktiviere mit `git lfs install`.
* Test: `rustc -V`, `cargo -V`, `cl.exe` verfügbar.
* Build: `cargo build --release` → `target/release/protocol-zero.exe`.

---

## 17) Startaufgaben – Ergänzung

* Hooks & LFS gemäß §12 einrichten.
* CI‑Workflow `ci-win.yml` anlegen.
* Gegner‑Archtypen als `crates/ai/src/archetypes.rs` skizzieren.
* `docs/story.md`: obige Prämisse + Arc D1 detaillieren.
* Performance‑Profile‑Flags in Repo aufnehmen.