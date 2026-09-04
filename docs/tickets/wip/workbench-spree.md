---
needs: []
---
**Give `Effect.Modal` a positional per-mode `Maybe (Cost bs)` so Spree's
per-mode additional costs have a spelling [CR#702.172a].** Fresh workbench
review 2026-09-03, R3, resolved by ruling.

**Ruling (settled 2026-09-03): Spree is effect-level, not a new card shape.**
`Effect.Modal (q : Quantity bs) (modes : List (Effect bs))`
(`Effect.idr:1307`) carries no cost per mode, so a Spree spell's "+ [cost] —
[effect]" modes cannot be written. The mode list becomes a list of
`(Maybe (Cost bs), Effect bs)` pairs — one positional slot, `Nothing` for
every ordinary modal spell, so no separate `SpreeModal` constructor and no
second modal mechanism.

21 supported cards print Spree modes (e.g. Caught in the Crossfire).

The slot is positional and required at the constructor, with the ordinary
modal spelling supplied by a wrapping macro that passes `Nothing`, per
`docs/decisions/card-authoring-binds-no-implicits.md`. Existing modal bench
sites go through that macro and do not change shape.

The mode-count gate is unaffected: P10 (`chooseModes (exactly 3) [two modes]`)
is refused today and stays refused.

Size: M.

Done when: Caught in the Crossfire is a typechecking bench witness with a cost
on each mode; an ordinary modal spell still reads through the unchanged macro;
P10's refusal is still pinned and probed non-vacuous; a pin refuses a Spree
mode list in which no mode carries a cost, or the CR-meaningless shape the
round identifies, probed non-vacuous; the build is 44/44 with 0 errors and 0
warnings. Standard constraints apply, plus the RON-shaped constraint: a core
constructor is admissible only if the RON re-emitter can produce it from a RON
node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
