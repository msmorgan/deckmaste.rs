---
needs: [engine-combatant-capability]
---
The removal dual of the layer-4 confer fold. `engine-combatant-capability` made card
types confer capabilities (`Creature` → `May(Attack)`/`May(Block)`) and folded
layer-4-**added** type & subtype confers into an object's derived abilities (close the
L4→L6 seam). That fold is **add-only**. The symmetric REMOVAL case is unhandled: when a
layer-4 effect strips a type or subtype (`CardTypes(Set([<non-creature>]))` /
`Subtypes(Set([...]))` — Song of the Dryads, Imprisoned in the Moon, Kasmina's
Transmutation, "loses all creature types"), the capabilities that type/subtype conferred
are NOT removed. The printed-face confers live in the base `abilities` (via
`printed_of_face`) and nothing strips them on type loss, so a permanent turned into a
land still carries `Innate(May(Attack))` and reads as a combatant — able to attack when
it should not.

This is a genuine correctness gap the capability-as-data model introduces (pre-feature,
combat gated on `has_type(Creature)` over the LAYER view, so type loss correctly stopped
combat). No current canon card exercises it, so no live bug ships — but it must be closed
before any type-stripping card enters canon.

**Why it's separate from the add fold (harder):** removal needs ATTRIBUTION. The add fold
just appends the current types'/subtypes' confers (deduped). Removal must strip ONLY the
abilities a now-absent type/subtype conferred, without touching the object's genuinely
printed/intrinsic abilities or abilities conferred by still-present types. The base
`abilities` mixes printed-intrinsic + printed-type-conferred + printed-subtype-conferred
with no provenance tag. Options to design: (a) tag conferred abilities with their source
type/subtype so removal is precise; (b) rebuild the type/subtype-conferred slice from the
CURRENT (post-L4) `card_types`/`subtypes` set each pass instead of carrying printed
confers in the base — i.e. move ALL type/subtype conferral (printed + added) to the L4
seam fold, so the base carries only intrinsic abilities and the fold is the single source
of truth (printed types are just types that happen to be present). Option (b) unifies add
and remove and is likely cleaner, but reworks how `printed_of_face` seeds the base.

Symmetric for subtypes — a printed subtype conferring an ability, removed at layer 4, has
the identical gap. Solve both at once (same seam / same attribution model).
