# God Manager (Gods as Athletes MVP) - Product Requirements Document

> **Document Location:** `.project/prd.md`
>
> This document defines the product requirements, features, and specifications.
> Keep this document as the single source of truth for what we're building.

---

## Overview

### Problem Statement
Players need a fast, strategic manager game loop that is fun to iterate in under 30 minutes, without complex combat controls or heavy content requirements.

### Solution
A 2 player (or player vs AI) manager game where players cultivate universe stats each season, which deterministically generate god stats, then an automated clash resolves combat, followed by fallout that permanently shifts the next season.

### Target Users
- **Primary:** Players who like short-form strategy/manager games with visible cause and effect.
- **Secondary:** Designers testing a loop quickly and iterating balance values.

### Success Metrics
- [ ] A full 3-season match completes in 15-25 minutes in playtest.
- [ ] Players can explain why they won or lost in a short debrief.
- [ ] At least 70% of playtesters want to run a second match.

---

## Features

### Core Features (MVP)

#### Feature 1: Season Loop
**Priority:** P0 (Must Have)

**Description:**
Three-phase season loop: Cultivation, Clash, Fallout. Repeat for 3 seasons.

**User Story:**
> As a player, I want a clear turn structure so I can make decisions and see outcomes quickly.

**Acceptance Criteria:**
- [ ] Each match runs exactly 3 seasons unless a universe collapses earlier.
- [ ] Each season runs in the fixed order: Cultivation, Clash, Fallout.
- [ ] A season summary displays outcomes and persistent effects.

**Technical Notes:**
- Keep the loop deterministic where possible to support balance testing.

---

#### Feature 2: Universe Stats and Cultivation
**Priority:** P0 (Must Have)

**Description:**
Players allocate 6 points per season to Belief, Conflict, Innovation (max +4 to any one stat). Stability is separate and only changes through Fallout.

**User Story:**
> As a player, I want to allocate points to shape my universe so I can influence my gods next season.

**Acceptance Criteria:**
- [ ] Player has 6 points to allocate each season.
- [ ] Belief, Conflict, Innovation each start at 5.
- [ ] Max +4 to any one stat per season; no negatives.
- [ ] Stability starts at 20 and cannot be increased in Cultivation.

---

#### Feature 3: God Roster and Stat Calculation
**Priority:** P0 (Must Have)

**Description:**
Each side has 4 gods with fixed roles. Stats are recalculated each season from universe values.

**User Story:**
> As a player, I want my gods to update based on my universe so I can plan around their roles.

**Acceptance Criteria:**
- [ ] Each side has exactly 4 gods.
- [ ] Roles are fixed: Tank, Striker, Control, Support.
- [ ] Attack, Defense, Speed are derived from universe stats per role.
- [ ] God HP is `10 + Defense`.

**Technical Notes:**
- Role scaling rules:
  - Tank: Defense = Belief * 2; Attack = Conflict; Speed = Innovation
  - Striker: Attack = Conflict * 2; Defense = Belief; Speed = Innovation
  - Control: Speed = Innovation * 2; Attack = Conflict; Defense = Belief
  - Support: Defense = Belief + Innovation; Attack = Conflict; Speed = Innovation
- Recalculate stats every season before Clash.

---

#### Feature 4: Clash Phase (Automated Combat)
**Priority:** P0 (Must Have)

**Description:**
Players select 3 of 4 gods. Combat runs automatically for 3 rounds; speed determines order; random target; simple damage formula.

**User Story:**
> As a player, I want fast automated fights so my strategic choices drive the outcomes.

**Acceptance Criteria:**
- [ ] Each side selects 3 gods to deploy.
- [ ] Turn order is descending Speed.
- [ ] Each action targets a random enemy god.
- [ ] Damage formula: `Attack - (Defense * 0.5)` with minimum 1.
- [ ] Combat runs for 3 rounds or until all gods on one side are defeated.

---

#### Feature 5: Traits and Fallout
**Priority:** P0 (Must Have)

**Description:**
After clash, stability is reduced based on defeated gods and a simple 1d6 fallout roll applies additional effects. Gods gain 1 trait per season based on performance.

**User Story:**
> As a player, I want lasting consequences so each season feels different and builds a narrative.

**Acceptance Criteria:**
- [ ] Stability loss: 2 per defeated god.
- [ ] Universe collapses if Stability <= 0.
- [ ] Apply a 1d6 fallout roll with the defined outcomes.
- [ ] Each god gains 1 trait per season based on performance.

**Technical Notes:**
- Fallout table:
  - 1: -1 Belief
  - 2: -1 Conflict
  - 3: -1 Innovation
  - 4: Random god gains negative trait
  - 5: No effect
  - 6: Random god gains positive trait
- Trait examples:
  - Battle-Hardened: +1 Defense permanently
  - Overextended: -1 Defense next season
  - Inspired: +1 Speed next season
  - Fractured Faith: -1 Belief contribution

---

#### Feature 6: Win Conditions
**Priority:** P0 (Must Have)

**Description:**
Match ends on universe collapse or after 3 seasons; higher stability wins; ties go to sudden death clash.

**User Story:**
> As a player, I want a clear end condition so matches stay short and replayable.

**Acceptance Criteria:**
- [ ] If Stability <= 0, that player loses immediately.
- [ ] After 3 seasons, higher Stability wins.
- [ ] Ties resolve by sudden death clash.

---

#### Feature 7: AI Opponent
**Priority:** P0 (Must Have)

**Description:**
Simple AI: if it lost last season, invests in its weakest stat; otherwise invests in Conflict and Innovation. Deploys top 3 gods by total stats.

**User Story:**
> As a solo player, I want a functional AI so I can test the loop.

**Acceptance Criteria:**
- [ ] AI cultivation logic follows the MVP rules.
- [ ] AI selects the top 3 gods by total stats.

---

### Secondary Features (Post-MVP)

#### Feature 8: One Post-MVP Extension
**Priority:** P1 (Should Have)

**Description:**
Add exactly one of the following: god abilities (once per clash), universe asymmetry (starting bonuses), or leader god passive.

---

## User Interface

### Screens/Views

#### Screen 1: Match Setup
**Purpose:** Choose PVP or vs AI, view starting universe and god roster.

**Components:**
- Mode selection (PVP / AI)
- Universe stats summary
- God roster list

**User Actions:**
- Select mode -> starts Season 1

---

#### Screen 2: Cultivation
**Purpose:** Allocate 6 points into Belief, Conflict, Innovation.

**Components:**
- Stat sliders or +/- controls
- Points remaining indicator
- Confirm allocation button

**User Actions:**
- Allocate points -> confirm -> moves to Clash

---

#### Screen 3: Clash
**Purpose:** Select 3 gods, then observe automated combat.

**Components:**
- God selection panel
- Combat log or simple animation
- Round indicator

**User Actions:**
- Select 3 gods -> start clash -> view results

---

#### Screen 4: Fallout Summary
**Purpose:** Show defeated gods, stability loss, traits gained, and fallout roll effects.

**Components:**
- Stability tracker
- Trait list updates
- 1d6 roll result

**User Actions:**
- Continue -> next season or end screen

---

#### Screen 5: End Screen
**Purpose:** Show winner and final stability values.

**Components:**
- Winner banner
- Final universe stats
- Play again button

**User Actions:**
- Start new match

---

### Design Guidelines

#### Color Palette
Use the default macroquad-toolkit dark palette where possible:
- Background: dark::BACKGROUND
- Surface/Panel: dark::PANEL
- Text: dark::TEXT / dark::TEXT_BRIGHT
- Accent: dark::ACCENT

#### Typography
- **Headings:** Readable sans serif, 600+ weight
- **Body:** Readable sans serif, 400-500 weight
- **Code:** Monospace font

---

## Technical Requirements

### Platform
- Desktop (Windows) first
- WebGL (WASM) target supported

### Performance
- Clash resolution under 1 second
- UI updates under 200 ms after user actions

### Security
- No authentication for MVP
- Local-only data

### Data
- Game constants and balance values stored in JSON under `assets/`
- Local save of a single in-progress match in JSON
- No long-term persistence required for MVP

---

## Constraints & Assumptions

### Constraints
- 3-season matches only
- No manual combat actions
- No direct stat upgrades on gods
- Everything flows from universe stats

### Assumptions
- Deterministic calculations aid balance testing
- Randomness only in target selection and fallout roll

### Out of Scope
- Lore systems and story progression
- Multiplayer networking
- Extensive AI strategy

---

## Glossary

| Term | Definition |
|------|------------|
| Universe | A set of stats that generate god performance each season. |
| Season | One full cycle of Cultivation, Clash, Fallout. |
| Stability | Universe HP; loss condition when it reaches 0. |
| Trait | A modifier applied to a god based on performance or fallout. |

---

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.1 | 2026-02-04 | Codex | Expanded requirements from GDD MVP |
| 1.0 | 2026-02-04 | Codex | Initial draft based on gdd_mvp.md |

---

*Last updated: 2026-02-04*
