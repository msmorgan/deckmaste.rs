---
needs: []
---
Use the [`Ability` and `Static Ability` glossary meanings](../../contexts/game-model/CONTEXT.md): an Ability is a
rules-defined quality of an Object or Player (or an activated/triggered ability on the stack), and a
Static Ability is one that is simply true while its Object is in the relevant zone — neither
describes a state-based action. Aura's actual `enchant` keyword remains an ordinary Static Ability
([CR#702.5a]), while the Aura type rules remain type rules ([CR#303.4,704.5m]); the two must not be
collapsed. Apply the same classification audit to Equipment, Fortification, Saga, and the other type
and subtype conferrals, not to Aura alone.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

**[design] An SBA is not a static ability — hoist `Sba` out of the `Static`
ability path.** `StaticEffect::Sba` (reachable only as `Ability::Static (Sba …)`) models a
state-based action ([CR#704]) as a kind of static ability ([CR#604]). The two are distinct CR
categories — [CR#704.1] "state-based actions are game actions that happen automatically… don't use
the stack," and [CR#704.1a] pointedly says abilities that watch game state are *triggered*
abilities, "not state-based actions"; [CR#604.1] static abilities "do something all the time." An
SBA is not an ability of any kind, so `Static (Sba …)` is a 704-under-604 category error.

**Where it bites.** `StaticEffect` is the engine's continuous-effect family — `Modify` ([CR#611]),
`Replaces`/`CantHappen` ([CR#614]), `CostModifier`, `AsThough`, and `Sba` ([CR#704]) — all wrapped
by `Ability::Static`. Most members belong: a replacement effect *is* established by a static
ability, a continuous P/T mod *is* one. `Sba` is the outlier — a 704 game action smuggled into the
ability hierarchy. The Aura must-be-attached rule ([CR#704.5m]) is authored (in
`plugins/builtin_v2/macros/stubs/subtypes/enchantment/Aura.ron`) as
`Ability(Innate(Static(Sba(…))))`. That is the shape to remove.

**The clean encoding already exists.**
- `Property::StateBased { condition, effect }` (a `Property` variant DISTINCT from
  `Property::Ability`) is the ability-free "this confers an SBA" flavor. Today every conferring
  subtype uses the `Property::Ability(…Static(Sba…))` form instead — that is the symptom.
- `SbaRule` (`crates/deckmaste_core/src/sba_rule.rs`) is documented as `StaticEffect::Sba` "lifted
  to a global, scoped rule" — SBA-as-game-rule, the taxonomically-correct home (see
  [State-based actions are data](../../decisions/state-based-actions-are-data.md)).

**Fix (design-gated — pair before touching the model).**
1. Remove `Sba` from `StaticEffect` (`crates/deckmaste_core/src/continuous.rs`), so an SBA can no
   longer be reached via `Ability::Static`. The workbench's `Experimental.Effect.StaticEffect` has
   no `Sba` constructor and must not gain one.
2. Re-author the conferring subtypes onto the SBA/state flavors: Aura's falls-off → a
   `Property::StateBased { condition, effect }` (not `Property::Ability(…Static(Sba…))`), and the
   same for the other core-facing subtype stubs. Saga's lore is a `TurnBased`/replacement mechanic,
   not an SBA — keep it out of the SBA path.

**Verify.** `cd idris && ./scripts/build` is green at its module count; workspace green; cite audit
if any CR citation moves. Grep confirms `Sba` is unreachable from `Ability::Static` in
`deckmaste_core` and absent from `idris/src/Experimental/`.

*Exposed by `idris-subtype-open-names` (done): putting conferrals on the subtype value surfaced that
every subtype confer is the `Property::Ability` flavor, i.e. SBAs were being modeled as abilities.
`[design]` — the model change is the soundness gate; open a design dialogue before implementing.*
