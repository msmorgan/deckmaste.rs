---
needs: []
---
**Idris grammar: give the global state-based-action rules a real data
representation.** From the idris↔rust structure audit (2026-06-26).

`idris/src/Core.idr` has the *conferred* SBA — `Sba : Condition b ->
OneShotEffect b -> StaticEffect b` (an Aura's falls-off rule) — but the *global*
[CR#704.5] tier is not modeled. The lethal-damage rule exists only as a loose
`tLethalSba : Condition Base` in `idris/src/Spec.idr`
(`Compare (Damage This) GreaterEq (StatOf This Toughness)`): a bare condition with
**no scope** (it silently assumes `This` is a creature) and **no effect** (the
destroy is gone). It is a member of no list and lives on no game-state, so the
term can say *when* the rule fires but cannot *be* the rule.

Rust models this as `SbaRule { scope: Filter, when: Condition, then: Effect }`
(`crates/deckmaste_core/src/sba_rule.rs`), where `scope` is the binding domain for
`This` (checked before `when`) — exactly what lets a rule that belongs to no
object range over objects. Adopt the shape:

```idris
record SbaRule where
  scope : Filter           -- the binding domain for This (e.g. creature on the battlefield)
  when  : Condition Base
  then  : OneShotEffect Base
```

Represent the [CR#704.5] rules (lethal damage [CR#704.5g], deathtouch
[CR#704.5h], 0-toughness [CR#704.5f] as a `Move` not a destroy, loyalty-0, …) as
a `List SbaRule`, replacing `tLethalSba`. All the ingredients already exist
(`Filter`/`Predicate`, `Condition Base`, `OneShotEffect Base`); this is a
factoring, not new primitives.
