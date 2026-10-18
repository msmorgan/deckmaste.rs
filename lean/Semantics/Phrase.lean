import Semantics.Abilities

/-!
# Semantics.Phrase

The phrase grammar (`ZoneScope`, `Pile`, `ZoneExpr`, `Predicate`, `NounPhrase`, `Amount`,
`Quantity`, `Condition`, `GameEvent`, `DidClause`, `LookbackSpan`, `Subject`).

In Semantics v3 (`docs/decisions/semantics-v3.md` §3), `GameEvent.does` and `DidClause`
(`Predicate.did`, `Amount.tally`, `Condition.did`) embed `action : Action` directly, so the
phrase, trigger, and ability grammars are elaborated together in the unified `mutual` block of
`Semantics.Abilities` and re-exported here.
-/
