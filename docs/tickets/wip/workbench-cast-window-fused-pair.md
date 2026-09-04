---
needs: []
---
**Collapse the two spellings of a spell's cast window.** Residue of
`workbench-fused-residues` (2026-09-04), which gave `Effect.AbilityAt.Spell`
a positional `(window : Maybe (Timing bs))` ahead of its instruction so
"Cast this spell only before the combat damage step" reads (Berserk, Blood
Frenzy). The slot also admits `DuringPart p w`, which spells the same
sentence as the deontic static `Static (OnlyDuring p w (Macros.deontic This
Permit ["Cast"] Patient NoDeonticPatient))` that `teleport`, `festival` and
`dazzlingBeautyCastRestriction` use — a fused pair.

Fix: decide which spelling is the core row (the timing slot on `Spell` is
positional and RON-shaped; the deontic static restates a permission the CR
gives as a timing restriction [CR#307.1] for sorceries and as "cast only"
text otherwise), retire the other by re-spelling its three sites, and keep
`dazzlingBeautyCastRestriction` spellable (a bare cast-restriction fragment
with no instruction — decide whether that is a `Spell` with an empty body
or belongs elsewhere). If the deontic clause `staticOnSpellCardOk` loses its
last user, delete it. Pin the retired spelling, probed non-vacuous.

Size: S. Done when: one spelling remains; the three sites and Berserk read
through it; build at its module count. Standard constraints apply,
including the RON-shaped constraint.
