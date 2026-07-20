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

**Confirmed root cause (2026-07-11): the confers are CACHED, not recomputed.**
`printed_of_face` (`derive.rs:41-55`) folds `face.types`/`face.subtypes` confers into
`CardInstance.printed` ONCE at card-push (`object.rs:124`); `base_values` (`layer.rs:263`)
re-seeds the layer abilities from that frozen cache every pass. Type-conferred capabilities
are never recomputed from the object's CURRENT (post-L4) `card_types`/`subtypes`. Strip the
Creature type at layer 4 and the cached `Innate(May(Attack))` persists.

**The fix is (b) recompute — and it is CLEAN, not attribution-tagging.** `base_map`
(`layer.rs:1650`) builds view entries for EVERY card-backed object in EVERY zone, and
`fold_conferred_abilities` (`layer.rs:1756`) already runs at the L4 seam for all of them —
so the fold covers hand/stack cards (casting-window `May(Cast)`, land-play `May(Play)`) too,
not just the battlefield. So:
1. Make `printed_of_face` fold ONLY intrinsic `face.abilities` — drop the
   `.chain(subtypes.confers).chain(types.confers)`. `CardInstance.printed` becomes
   intrinsic-only.
2. `fold_conferred_abilities` becomes the SINGLE SOURCE OF TRUTH for type/subtype conferral,
   recomputed from CURRENT `card_types`/`subtypes` each pass. Its dedup (`!contains`) is no
   longer load-bearing (the base has no type confers to collide with) — DELETE it.

Consequences: stripped type → not in current `card_types` → not folded → grant gone
(removal works for free); the T3b dedup band-aid is removed; printed + layer-4-added
conferral unify into one path; symmetric for subtypes. Zero canon reachability today, so no
live bug, but the cache is a convenient-but-wrong shortcut: the architecture must
derive conferrals from current characteristics.

**One nuance to handle:** the first-pass effect-source `gather` reads raw `printed_abilities`
(`layer.rs:476`, `player_statics.rs:67`) to break the `layers()` recursion. A type/subtype
that confers a LAYER-AFFECTING static (not a deontic) would be gathered one fixpoint pass
later instead of at pass 0. Irrelevant for the deontic confers we have (May/Cant produce no
layer effect); the ≤16-pass fixpoint converges for the general case — the same path
layer-4-ADDED confers already ride. Add a test if a layer-affecting type-confer ever lands.

**Tests:** a battlefield creature that LOSES its Creature type (`CardTypes(Set([<non-creature>]))`)
is NOT a combatant (no `May(Attack)`, not a legal attacker, combat damage not marked);
regression — a printed creature's grant still appears exactly once (the existing
`printed_creature_grant_is_not_doubled_by_the_fold`); casting-window (hand Instant →
`May(Cast)`) and land-play (Land → `May(Play)`) still work via the fold; the
`animated_enchantment_can_attack` + `layer_added_subtype_confers_its_keyword` stay green.
