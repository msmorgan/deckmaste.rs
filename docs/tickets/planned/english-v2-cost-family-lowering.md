---
needs: [english-v2-subordinate-clause, english-v2-lexeme-owned-verb-frames]
---
**Lower the cost-family categories to ordinary linguistic constituents.** The
rewrite decision's Plan 09 amendment (`docs/decisions/english-v2-rewrite.md`,
"English-v2 produces a linguistic AST … Printed notation and document
segmentation are the only game-specific surfaces admitted directly by the
grammar") admits `CostSymbol`, `ManaAmount`, `KeywordManaCost` and their kin as
printed notation. It does not admit `ActivationCostComponent`,
`AdditionalCostBody`, `ControlledCostAction`, `CostComparisonPredicate`,
`CastingRestriction`, or `RestrictionTurn`, which name game meaning and are
the largest unticketed block of the game-semantic categories still in the
grammar (~24 when this was written, unstamped; re-measure at claim). The distribution/counter block has its own ticket and the predicative
block is the `Predicative complement as one copular frame (A7)` entry in
`../fog.md`; this one owns the cost family.

Pinned direction, one category at a time, each with its own landing record
entry: an activation cost is a coordination of cost constituents where the
constituents are ordinary imperative clauses, mana notation, or symbols;
"as an additional cost to cast this spell, …" is a subordinate/adverbial
attachment (`additional_cost` is a `ClauseAttachment` and belongs to the
subordinate-clause family); "cast … only …" restrictions are ordinary
adverbial modification with a temporal noun phrase, not a
`CastingRestriction`/`RestrictionTurn` pair; cost comparison is a comparative
predicate over a measure. The AST remains linguistic; the downstream semantic
layer recovers cost-hood from the ability's document position and the verbs.

Fences: a replacement category whose name carries game meaning (`Cost…`,
`Restriction…`); a Rust guard naming a verb, noun, or lexeme; retaining the old
category as an alias. A specificity tie exposed by generalising is a STOP.

Acceptance: the six categories gone, their coverage re-spelled, selection
census before/after with every identity whose winning analysis moved listed,
and the remaining game-semantic-category count in the landing record.

2026-09-04: audited against the ADR; the category count was an unstamped
baseline — was: "the ~24 game-semantic categories" / "the ADR's
game-semantic-category count" (the ADR carries no such count). Standard
constraints apply.
