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

**[design] Idris grammar: an SBA is not a static ability — hoist `Sba` out of the `Static`
ability path.** `StaticEffect::Sba` (Rust) / the Idris `Sba` constructor of `StaticEffect`
(reachable only as `Ability::Static (Sba …)`) models a state-based action ([CR#704]) as a
kind of static ability ([CR#604]). The two are distinct CR categories — [CR#704.1] "state-based
actions are game actions that happen automatically… don't use the stack," and [CR#704.1a]
pointedly says abilities that watch game state are *triggered* abilities, "not state-based
actions"; [CR#604.1] static abilities "do something all the time." An SBA is not an ability of
any kind, so `Static (Sba …)` is a 704-under-604 category error.

**Where it bites.** `StaticEffect` is the engine's continuous-effect family — `Modify` ([CR#611]),
`Replaces`/`CantHappen` ([CR#614]), `CostModifier`, `AsThough`, and `Sba` ([CR#704]) — all wrapped
by `Ability::Static : StaticEffect -> Ability`. Most members belong: a replacement effect *is*
established by a static ability, a continuous P/T mod *is* one. `Sba` is the outlier — a 704 game
action smuggled into the ability hierarchy. The Aura must-be-attached rule ([CR#704.5m]) is
authored (in `plugins/builtin/macros/types/enchantment/Aura.ron`) as
`Ability(Innate(Static(Sba(…))))` and the (now-unused, post-`idris-subtype-open-names`)
`subtypeConfers` mirrored it as `Static (Sba …)`. That is the shape to remove.

**The clean encoding already exists.**
- `Property::StateBased { condition, effect }` (a `Property` variant DISTINCT from
  `Property::Ability`) is the ability-free "this confers an SBA" flavor. Today every conferring
  subtype uses the `Property::Ability(…Static(Sba…))` form instead — that is the symptom.
- `SbaRule` (`crates/deckmaste_core/src/sba_rule.rs`) is documented as `StaticEffect::Sba` "lifted
  to a global, scoped rule" — SBA-as-game-rule, the taxonomically-correct home (see
  [State-based actions are data](../../decisions/state-based-actions-are-data.md)).

**Fix (design-gated — pair before touching the grammar).**
1. Remove `Sba` from `StaticEffect` (Rust `continuous.rs`) and from the Idris `StaticEffect`
   constructor list, so an SBA can no longer be reached via `Ability::Static`.
2. Re-author the conferring subtypes onto the SBA/state flavors: Aura's falls-off → a
   `Property::StateBased { condition, effect }` (not `Property::Ability(…Static(Sba…))`); route the
   Idris side so a conferred SBA is represented without the `Static` ability wrapper (its own
   conferral kind, aligned with `Property::StateBased`/`TurnBased` and/or the `SbaRule` game-rule
   path). Saga's lore is a `TurnBased`/replacement mechanic, not an SBA — keep it out of the SBA
   path.
3. `idris_emit`: emit conferred SBAs through the new (ability-free) representation; the confers are
   still sourced from the RON per [Conferrals come from registries](../../decisions/conferrals-come-from-registries.md), just no longer as
   `Static (Sba …)`.

**Verify.** `cargo xtask idris-check` stays green (the Aura/Equipment/etc. cards re-emit through the
new shape); `idris/` builds; workspace green; cite audit if any CR citation moves. Grep confirms `Sba`
is unreachable from `Ability::Static` (Rust and Idris).

*Exposed by `idris-subtype-open-names` (done): putting conferrals on the subtype value surfaced that
every subtype confer is the `Property::Ability` flavor, i.e. SBAs were being modeled as abilities.
Serializes with other `idris-*` grammar tickets (all rewrite `idris/src/Semantics.idr`). `[design]` — the
grammar change is the soundness gate; open a design dialogue before implementing.*
