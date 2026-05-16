Perfect. Below is a **complete, ultra-lean MVP spec** you can actually build in a weekend.
No fluff, no lore bloat — just enough structure to test if this is *fun*.

---

# ULTRA-LEAN MVP: **Gods as Athletes**

## High-Level Goal

Two players (or player vs AI) manage gods by cultivating universes.
Universes generate stats → gods fight → fallout alters next round.
After **3 Seasons**, the universe that collapses loses.

---

## CORE LOOP (LOCKED)

Each **Season** has 3 phases:

1. **Cultivation Phase** – allocate universe focus
2. **Clash Phase** – gods fight automatically
3. **Fallout Phase** – persistent changes

Repeat ×3.

---

# 1️⃣ DATA MODEL (MINIMUM)

## Universe

```json
{
  "belief": 5,
  "conflict": 5,
  "innovation": 5,
  "stability": 20
}
```

* Belief → defense & sustain
* Conflict → attack & aggression
* Innovation → abilities & tempo
* Stability → universe HP (loss = closer to collapse)

All start at **5**, Stability at **20**.

---

## God

Each side has **4 gods**.

```json
{
  "name": "Gogus",
  "role": "Tank",
  "domain": "War",
  "attack": 0,
  "defense": 0,
  "speed": 0,
  "traits": []
}
```

Roles are *very important* (no custom builds yet).

### Roles (Fixed)

| Role    | Scales From         |
| ------- | ------------------- |
| Tank    | Belief ×2           |
| Striker | Conflict ×2         |
| Control | Innovation ×2       |
| Support | Belief + Innovation |

Stats are recalculated **every season** from universe values.

---

# 2️⃣ CULTIVATION PHASE

Player has **6 points** to distribute.

Options (simple sliders):

* +1 Belief
* +1 Conflict
* +1 Innovation

Rules:

* Max +4 to any one stat per season
* No negatives
* Stability does NOT increase here

### Example Choice

> “I need defense”
> → +4 Belief, +2 Innovation

---

## God Stat Calculation

For each god:

```text
Attack  = Conflict scaling
Defense = Belief scaling
Speed   = Innovation scaling
```

Example:

* Tank God

  * Defense = Belief ×2
  * Attack = Conflict
  * Speed = Innovation

This is deterministic and transparent.

---

# 3️⃣ CLASH PHASE (AUTOMATED)

Each side selects **3 of 4 gods** to deploy.

Order is based on **Speed**.

---

## Combat Rules (VERY SIMPLE)

Combat runs for **3 Rounds**.

### On Each Round:

1. Fastest god acts first
2. Target = random enemy god
3. Damage formula:

```text
Damage = Attack - (Defense × 0.5)
Minimum damage = 1
```

4. If a god reaches 0 HP → defeated for this season

God HP = `10 + Defense`

No healing. No revives.

---

## Traits (Tiny but Powerful)

Gods gain **1 trait** per season depending on performance.

Examples:

* **Battle-Hardened**: +1 Defense permanently
* **Overextended**: −1 Defense next season
* **Inspired**: +1 Speed next season
* **Fractured Faith**: −1 Belief contribution

Traits stack and persist.

This is where narrative starts to emerge.

---

# 4️⃣ FALLOUT PHASE

After clash:

### Stability Damage

Universe loses:

```text
2 Stability per defeated god
```

Example:

* Lost 2 gods → −4 Stability

If Stability ≤ 0 → **Universe Collapses (Loss)**

---

### Additional Fallout (One Roll)

Roll 1d6:

| Roll | Effect                          |
| ---- | ------------------------------- |
| 1    | −1 Belief                       |
| 2    | −1 Conflict                     |
| 3    | −1 Innovation                   |
| 4    | Random god gains negative trait |
| 5    | Nothing                         |
| 6    | Random god gains positive trait |

Cheap, effective chaos.

---

# 5️⃣ WIN CONDITIONS

Game ends when:

* One universe Stability ≤ 0
  **OR**
* After 3 seasons, higher Stability wins

Tie → sudden death clash.

---

# 6️⃣ AI OPPONENT (DUMB BUT FINE)

AI Cultivation Logic:

```text
If lost last season:
  +4 to stat it was weakest in
Else:
  +3 Conflict, +3 Innovation
```

AI God Selection:

* Always deploy top 3 gods by total stats

This is enough for testing.

---

# 7️⃣ WHAT YOU’LL LEARN FROM THIS MVP

This version will immediately answer:

* Is the **Universe → God → Fight loop satisfying?**
* Do players feel responsible for wins/losses?
* Are gods memorable after only 3 seasons?
* Does delayed fallout feel meaningful?

If *yes*, you scale.
If *no*, you pivot cheaply.

---

# HARD CONSTRAINTS (DO NOT BREAK)

These are design load-bearing walls:

1. ❌ No manual god actions in combat
2. ❌ No direct stat upgrades on gods
3. ❌ No hero units, armies, or lore systems
4. ✅ Everything flows from universe stats

If you break these, it stops being a manager game.

---

# NEXT STEP AFTER MVP (ONLY ONE)

Once MVP works, add **ONE** of these:

* God abilities (once per clash)
* Universe asymmetry (starting bonuses)
* Leader god passive

Only one. Not all.

---

If you want, next I can:

* Turn this into **exact JSON schemas**
* Write **pseudocode for the clash loop**
* Help you **decide which stat is broken first** (it will be Conflict)

Just say the word.
