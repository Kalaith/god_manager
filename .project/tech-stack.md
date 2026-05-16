# God Manager (Gods as Athletes MVP) - Tech Stack

> **Document Location:** `.project/tech-stack.md`
>
> This document outlines the technology choices and rationale for the project.
> All technology decisions should be documented here with reasoning.

---

## Stack Overview

```
Frontend (Rendering/UI): Macroquad + macroquad-toolkit
Backend (Game Logic): Rust (in-process, no server)
Data Layer: JSON files (assets + save file)
Infrastructure: Local builds (no hosted services for MVP)
```

---

## Core Technologies

### Language & Runtime

| Technology | Version | Purpose |
|------------|---------|---------|
| Rust | 2021 edition | Primary language |
| Rust (native/WASM) | stable | Execution environment |

**Rationale:**
- Matches existing studio standards for Macroquad games.
- Single codebase for Windows and WebGL builds.

---

### Framework

| Technology | Version | Purpose |
|------------|---------|---------|
| Macroquad | 0.4.x | Rendering, input, game loop |
| macroquad-toolkit | local path | UI widgets, helpers, palette |

**Rationale:**
- Immediate-mode UI keeps iteration fast and matches the MVP focus.
- Toolkit provides consistent UI primitives and input helpers.

---

### Data Storage

| Technology | Version | Purpose |
|------------|---------|---------|
| JSON (assets) | n/a | Balance constants and static data |
| JSON (save) | n/a | Single in-progress match save |

**Rationale:**
- JSON is lightweight, human-editable, and fast for balance iteration. Prefer toolkit-provided helpers (if available) before adding custom loaders.

**Schema Location:** `assets/constants.json` (planned)

---

## Dependencies

### Production Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| macroquad | 0.4.x | Rendering/input/runtime |
| macroquad-toolkit | local path | UI + helpers (check toolkit for shared utilities first) |
| serde | 1.x | Data serialization |
| serde_json | 1.x | JSON parsing |

### Development Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| (none yet) | - | - |

---

## Build & Tooling

### Build System

| Tool | Version | Purpose |
|------|---------|---------|
| cargo | stable | Build/test |
| rustfmt | stable | Formatting |
| clippy | stable | Linting |

### Build Commands

```bash
# Development
cargo build

# Windows release
cargo build --release

# WebGL/WASM
cargo build --release --target wasm32-unknown-unknown

# Testing
cargo test

# Linting
cargo clippy

# Formatting
cargo fmt
```

---

## Architecture Patterns

### Code Organization

```
project-root/
+-- .project/           # Project documentation
+-- src/
¦   +-- main.rs         # Entry point, game loop, phase transitions
¦   +-- data/            # Data types and JSON loading
¦   +-- engine/          # Game logic services (stateless)
¦   +-- state/           # Game state management
¦   +-- ui/              # UI components and styling
¦   +-- screens/         # Screen-specific rendering (optional)
+-- assets/              # Game data and constants
+-- index.html           # WebGL host page
+-- publish.ps1          # Build/deploy script
```

### Design Patterns Used

| Pattern | Where Used | Purpose |
|---------|------------|---------|
| State machine | `state/` and `game` loop | Clear phase transitions |
| Stateless services | `engine/` | Deterministic calculations |
| Data-driven config | `assets/` JSON | Fast balance iteration |

---

## Environment Configuration

### Required Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| (none) | No external services required | No |

### Configuration Files

| File | Purpose |
|------|---------|
| `assets/constants.json` | Balance values and rules |
| `save.json` | Local in-progress match save |

---

## External Services

### APIs & Integrations

| Service | Purpose | Documentation |
|---------|---------|---------------|
| (none) | MVP is offline/local | - |

---

## Security Considerations

### Authentication
- None for MVP

### Data Protection
- Local-only files

### Dependencies
- Use cargo and lockfile for deterministic builds

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Clash resolution | < 1s | In-game timing |
| UI response | < 200 ms | Manual playtest |

---

## Dependency Policy\r\n\r\n- Check `macroquad-toolkit` for existing utilities (random, JSON loading, asset helpers) before adding new crates or custom implementations.\r\n- If the toolkit does not provide the needed functionality, document the new dependency or custom module here.\r\n\r\n## Toolkit Capabilities (Prefer First)

| Area | Module | Notes |
|------|--------|-------|
| RNG | `rng` | Use `GameRng` for seeded random rolls and selections. |
| Persistence | `persistence` | Use `save_json`, `load_json`, `get_app_data_path` for save/load. |
| Assets | `assets` | Use `AssetManager` and JSON `TextureConfig` loading. |
| UI | `ui`, `colors` | Buttons, panels, windows/modals, progress bars, palette. |
| Input | `input` | Hover/click helpers and input capture. |
| Events | `events` | Generic event bus for decoupled logic. |
| State | `states` | Optional GameState/Transition trait system. |
## Decision Log

| Date | Decision | Rationale | Alternatives Considered |
|------|----------|-----------|------------------------|
| 2026-02-04 | Use Macroquad + macroquad-toolkit | Matches studio standards and MVP speed | Bevy, custom renderer |
| 2026-02-04 | JSON for data and saves | Fast iteration, low complexity | SQLite |
| 2026-02-04 | No backend services | MVP scope and offline play | Client/server split |

---

*Last updated: 2026-02-04*


