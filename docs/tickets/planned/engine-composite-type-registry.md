---
needs: []
---
Make card types data-with-`confers`, like subtypes already are, so engines gate on a
conferred capability instead of a literal `Type::X`. Authoring goes open (types are
`Ident`-named, plugin-declared, macro-expanded to full structs); the Idris soundness core
keeps its closed `Type_` enum ("open authoring, closed core").

Shape: a `TypeDef { name, permanent, confers }` struct mirroring `Subtype` (the `types`
field replaced by a `permanent` bool). `CardFace.types` becomes `Vec<TypeDef>` — expanded
from bare names (`types: [Creature]`) by a `kinds: [Type]` macro pass, exactly as
`subtypes: [Goblin]` expands today. A `GameState.types` registry (parallel to
`GameState.subtypes`) resolves the bare `Ident`s that layer-4 `CardTypes(Add/Remove/Set)`
ops carry, via a `resolve_type` mirroring `resolve_subtype` (absent name → empty def →
fizzle, never crash). The `Type(Type)` predicate becomes `Type(Ident)`;
`CardTypes(CollectionOp<Type>)` becomes `CollectionOp<Ident>` — a public RON break, so every
type-changing effect (Turn-to-Frog) re-authors. A load-time validator (alongside
`validate_counter_refs`) checks every authored type name resolves in the registry. Idris is
untouched: its `emit_type` maps each `TypeDef.name` → the closed `Type_` variant, gapping
unknowns exactly as it gaps `Dungeon` today; type-conferred capabilities are Idris-invisible,
consistent with `ConferralRule`/`SbaRule`/`DamageResultRule`.

Prove the substrate by migrating three consumers in the same change: casting-window (the
`Instant` type confers `May(Cast(window: InstantSpeed))`; drop the `instant` literal so its
default flows the same `may_cast_rows` collector as flash); permanent-flag
(`is_permanent_spell` reads `TypeDef.permanent`; the battlefield-entry-vs-resolve control
flow stays hardcoded); and land-play (the `Land` type confers `May(PlayLand)`, one capability
serving both "offer PlayLand" and "not castable as a spell", correct per face for MDFC
land//spell cards). Every other `Type::X` gate keeps its behavior by name-matching the
canonical type through the registry-resolved predicate. Combatant is a separate ticket
(`engine-combatant-capability`).
