---
needs: []
---
**The deathtouch-strike flag is reset too late, so a creature that regenerates
from deathtouch damage is wrongly destroyed.** Surfaced by the idris↔rust
structure audit (2026-06-26).

`crates/deckmaste_engine/src/object.rs` carries `struck_by_deathtouch: bool`, set
when damage is dealt by a deathtouch source (`step.rs`), and read by the
lethal-damage SBA via `DamagedByDeathtouch(This)`
(`condition.rs`, `plugins/builtin/rules/sba/lethal-damage.ron`). Today it is
cleared **only at cleanup** (`step.rs`, [CR#514.2]) and is **not** cleared after
each state-based-action check, nor when damage is removed (regeneration's heal
zeroes `damage` but leaves the flag set).

[CR#704.5h]: a creature is destroyed if it has been dealt damage this turn by a
source with deathtouch **"since the last time state-based actions were checked."**
The window is per-SBA-check, not until-cleanup.

Failure case: Drudge Skeletons (regenerate) blocks a deathtouch attacker. First
SBA check → `WillDestroy` → regeneration replaces it (shield consumed). The flag
is still set, so the **next** SBA check sees `DamagedByDeathtouch` true →
`WillDestroy` again → no shield left → wrongly destroyed.

Fix: clear `struck_by_deathtouch` at the end of each SBA check (and when damage is
removed), so it reflects only damage dealt since the previous check.
