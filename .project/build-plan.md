# God Manager (Gods as Athletes MVP) Build Plan

> **CRITICAL INSTRUCTIONS FOR ENGINEERS**
>
> ## Project Structure
> All project documentation lives in the `.project/` directory at the repository root:
> ```
> .project/
> +-- prd.md           # Product Requirements Document
> +-- tech-stack.md    # Technology choices and rationale
> +-- build-plan.md    # This file - task tracking
> +-- changelog.md     # Version history and updates
> ```
>
> ## Build Discipline
> 1. **Keep this document up to date** - Mark tasks as completed immediately after finishing them
> 2. **Build after every task** - Run the build command after completing each task
> 3. **Zero tolerance for warnings/errors** - Fix any warnings or errors before moving to the next task
> 4. **Update changelog.md** - Log significant changes, fixes, and milestones
>
> ```bash
> # Build command (run after each task)
> cargo build --release
> ```
>
> If warnings or errors appear, fix them immediately. Do not proceed until the build is clean.

---

## Status Legend

| Icon | Status | Description |
|------|--------|-------------|
| ⬜ | Not Started | Task has not begun |
| 🔄 | In Progress | Currently being worked on |
| ✅ | Completed | Task finished |
| ⛔ | Blocked | Cannot proceed due to external dependency |
| ⚠️ | Has Blockers | Waiting on another task |
| 🔍 | In Review | Pending review/approval |
| 🚫 | Skipped | Intentionally not doing |
| ⏸️ | Deferred | Postponed to later phase/sprint |

---

## Project Progress Summary

```
Phase 1: Planning & Setup [█████████████░░░░░░░]  67%  🔄
Phase 2: Core Loop        [██████████████░░░░░░]  81%  🔄
Phase 3: UI Screens       [██████████████░░░░░░]  77%  🔄
Phase 4: Data & Saves     [░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
Phase 5: AI & Balance     [░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
Phase 6: QA & Polish      [░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
Phase 7: Release Builds   [░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
────────────────────────────────────────────────────
Overall Progress          [█████████░░░░░░░░░░░]  46%
```

| Phase | Tasks | Completed | Blocked | Deferred | Progress |
|-------|-------|-----------|---------|----------|----------|
| Phase 1: Planning & Setup | 12 | 8 | 0 | 0 | 67% |
| Phase 2: Core Loop | 16 | 13 | 0 | 0 | 81% |
| Phase 3: UI Screens | 13 | 10 | 0 | 0 | 77% |
| Phase 4: Data & Saves | 8 | 0 | 0 | 0 | 0% |
| Phase 5: AI & Balance | 6 | 0 | 0 | 0 | 0% |
| Phase 6: QA & Polish | 7 | 0 | 0 | 0 | 0% |
| Phase 7: Release Builds | 4 | 0 | 0 | 0 | 0% |
| **Total** | **66** | **31** | **0** | **0** | **46%** |

---

## Phase 1: Planning & Setup

### 1.1 Documentation

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 1.1.1 | Finalize `prd.md` from GDD MVP |
| ✅ | 1.1.2 | Finalize `tech-stack.md` mapped to PRD |
| ✅ | 1.1.3 | Build `build-plan.md` from PRD + tech stack |
| ⬜ | 1.1.4 | Define game name constants and window config |
| ⬜ | 1.1.5 | Create `assets/` structure and placeholders |
| ⬜ | 1.1.6 | **BUILD CHECK** - Verify clean release build |

### 1.2 Project Skeleton

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 1.2.1 | Initialize Rust project structure (`src/`, modules) |
| ✅ | 1.2.2 | Add dependencies in `Cargo.toml` |
| ✅ | 1.2.3 | Create base module files (data/engine/state/ui) |
| ✅ | 1.2.4 | Wire minimal main loop and state machine |
| ✅ | 1.2.5 | Implement empty screen stubs |
| ⬜ | 1.2.6 | **BUILD CHECK** - Verify clean release build |

---

## Phase 2: Core Loop (MVP Rules)

### 2.1 Data Models

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 2.1.1 | Define universe stats and season data structs |
| ✅ | 2.1.2 | Define god roster, roles, and trait structs |
| ✅ | 2.1.3 | Implement stat recalculation rules |
| ✅ | 2.1.4 | Implement HP calculation rules |
| ✅ | 2.1.5 | Implement cultivation allocation rules |
| ⬜ | 2.1.6 | **BUILD CHECK** - Verify clean release build |

### 2.2 Clash Simulation

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 2.2.1 | Implement turn ordering by Speed |
| ✅ | 2.2.2 | Implement damage formula and defeat logic |
| ✅ | 2.2.3 | Implement 3-round combat loop |
| ✅ | 2.2.4 | Emit combat log events for UI |
| ⬜ | 2.2.5 | **BUILD CHECK** - Verify clean release build |

### 2.3 Fallout & Win Conditions

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 2.3.1 | Implement stability loss per defeated god |
| ✅ | 2.3.2 | Implement 1d6 fallout roll table |
| ✅ | 2.3.3 | Apply trait outcomes per season |
| ✅ | 2.3.4 | Implement win conditions and tie-breaker clash |
| ⬜ | 2.3.5 | **BUILD CHECK** - Verify clean release build |

---

## Phase 3: UI Screens

### 3.1 Match Setup

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 3.1.1 | Build Match Setup screen UI |
| ✅ | 3.1.2 | Mode selection (PVP / AI) |
| ✅ | 3.1.3 | Display initial universe and roster |
| ⬜ | 3.1.4 | **BUILD CHECK** - Verify clean release build |

### 3.2 Cultivation

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 3.2.1 | Implement stat sliders or +/- controls |
| ⬜ | 3.2.2 | Points remaining indicator |
| ⬜ | 3.2.3 | Confirm allocation flow |
| ⬜ | 3.2.4 | **BUILD CHECK** - Verify clean release build |

### 3.3 Clash & Fallout Views

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 3.3.1 | God selection panel (3 of 4) |
| ✅ | 3.3.2 | Combat log or simple animation display |
| ✅ | 3.3.3 | Fallout summary screen |
| ✅ | 3.3.4 | End screen with winner summary |
| ⬜ | 3.3.5 | **BUILD CHECK** - Verify clean release build |

---

## Phase 4: Data & Saves

### 4.1 Data-Driven Config

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 4.1.1 | Create `assets/constants.json` schema |
| ⬜ | 4.1.2 | Load balance constants at startup |
| ⬜ | 4.1.3 | Refactor logic to use config values |
| ⬜ | 4.1.4 | **BUILD CHECK** - Verify clean release build |

### 4.2 Save/Load

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 4.2.1 | Define save data struct |
| ⬜ | 4.2.2 | Save in-progress match to `save.json` |
| ⬜ | 4.2.3 | Load save on boot and resume |
| ⬜ | 4.2.4 | **BUILD CHECK** - Verify clean release build |

---

## Phase 5: AI & Balance

### 5.1 AI Logic

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 5.1.1 | Implement AI cultivation rules |
| ⬜ | 5.1.2 | Implement AI god selection (top 3) |
| ⬜ | 5.1.3 | **BUILD CHECK** - Verify clean release build |

### 5.2 Balance Pass

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 5.2.1 | Define initial trait list and weights |
| ⬜ | 5.2.2 | Adjust constants to avoid dominant stat |
| ⬜ | 5.2.3 | **BUILD CHECK** - Verify clean release build |

---

## Phase 6: QA & Polish

### 6.1 Tests

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 6.1.1 | Unit tests for stat calculations |
| ⬜ | 6.1.2 | Unit tests for clash resolution |
| ⬜ | 6.1.3 | Unit tests for fallout outcomes |
| ⬜ | 6.1.4 | **BUILD CHECK** - All tests passing |

### 6.2 Playtest & UX

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 6.2.1 | Playtest checklist and notes |
| ⬜ | 6.2.2 | Improve clarity of logs and summaries |
| ⬜ | 6.2.3 | **BUILD CHECK** - Verify clean release build |

---

## Phase 7: Release Builds

### 7.1 Builds

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 7.1.1 | Windows release build |
| ⬜ | 7.1.2 | WebGL/WASM build |
| ⬜ | 7.1.3 | Verify `publish.ps1` output |
| ⬜ | 7.1.4 | Update `index.html` with correct WASM name |

---

## Changelog Reference

See `.project/changelog.md` for detailed version history.

---

## Notes & Decisions

### Architecture Decisions
- State machine for screen transitions
- Stateless engine services for calculations
- Data-driven constants from JSON

### Known Issues
- [ ] None yet

### Dependencies
- macroquad, macroquad-toolkit, serde, serde_json

---

*Last updated: 2026-02-04*
*Current Phase: Phase 2 - Core Loop*
*Next Milestone: Phase 2 build checks (2.1.6, 2.2.5, 2.3.5)*







