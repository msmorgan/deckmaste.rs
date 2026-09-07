---
needs: []
---
**Rehome the designation storage columns the v2 declarations used to carry.**
`facts-generator-sheds-v1` (2026-09-07) retyped every
`plugins_v2/builtin/macros/stubs/designations/*.ron` body from the v1
`DesignationDecl(definition: Stored(scope, shape, uniqueness, persistence))`
onto the `deckmaste_semantics_v2::facts` mirror of
`lean/Semantics/Check/FactTypes.lean`. `DesignationFacts` has no column for
`shape`, `uniqueness` or `persistence`, so those three dropped out of the tree.
Seventeen of the nineteen rows existed only there — `plugins/builtin` (v1)
declares only `Commander` and `Monarch` — so this ticket is the record of what
was dropped and the decision of where it goes when an engine needs it.

`scope` survives as `DesignationFacts.scope`; an `Enum` shape survives as one
`DesignationFacts` row per member (`DayNight` → "day"/"night", `Sector` →
"alpha sector"/"beta sector"/"gamma sector"). What was dropped:

| declaration | shape | uniqueness | persistence |
|---|---|---|---|
| CitysBlessing | Flag | PerPlayer | Permanently |
| Commander | Flag | None | Permanently |
| DayNight | Enum(["Day", "Night"]) | PerGame | Permanently |
| EnduringStory | Flag | PerPlayer | Permanently |
| Goaded | Relation | None | EffectSupplied |
| Harnessed | Flag | None | ObjectLifetime |
| Initiative | Flag | PerGame | Permanently |
| LeftHalfUnlocked | Flag | None | ObjectLifetime |
| Level | Number | None | ObjectLifetime |
| Monarch | Flag | PerGame | Permanently |
| Monstrous | Flag | None | ObjectLifetime |
| Prepared | Flag | None | ObjectLifetime |
| Renowned | Flag | None | ObjectLifetime |
| RightHalfUnlocked | Flag | None | ObjectLifetime |
| RingBearer | Relation | PerPlayer | EffectSupplied |
| Saddled | Flag | None | UntilEndOfTurn |
| Sector | Enum(["Alpha", "Beta", "Gamma"]) | None | EffectSupplied |
| Solved | Flag | None | ObjectLifetime |
| Suspected | Flag | None | ObjectLifetime |

The decision to make: whether these are Lean columns (`DesignationFacts` grows
`shape`/`uniqueness`/`persistence`, and the checker gains a law that reads
them — the initiative and the monarch move off their prior holder
[CR#725.3,726.3], a Ring-bearer designation ends when another creature becomes
that player's Ring-bearer [CR#701.54a]) or engine-only state that belongs in a
`plugins_v2/*/rules/` table rather than in the declaration. Lean is the spec, so
a column no law reads should not be a Lean column; a designation law that needs
uniqueness or persistence is the trigger for growing them. Related but distinct:
`core-getdesignation-scopes-and-eviction` and `core-designation-card-scope` are
the v1 `deckmaste_core` half of the same question.

Standard constraints apply.
