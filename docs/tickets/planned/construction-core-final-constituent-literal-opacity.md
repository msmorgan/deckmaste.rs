---
needs: [construction-core-final-constituent-primitive]
---
Final-constituent primitive: trailing form literals must be opaque (final-
constituent landing review F1, HIGH). In the compiler's rightmost-leaf fold
(`crates/deckmaste_construction_core/src/emit/final_constituent.rs`), the
`AtomPlan::Literal` (and `SentenceInitialLiteral`) arm emits `fallback`, so a
trailing literal is transparent and the predicate reports the leaf BEFORE it.
Live defect: `Target creature gains "When this creature dies, draw a card."
instead` — no final period, quoted block not surface-final — is SELECTED via
`AbilityBodyQuoteTerminatedStatement` and round-trips. The deleted walker's
`_ => false` rejected it. Zero corpus exposure, so every gate is green.

Fix: `Literal`/`SentenceInitialLiteral` → `false` (a literal IS a leaf and is
never the sought category); keep `Bound`/`Circumfix` transparent (that is what
makes `quoted_ability` resolve). Adjacent `VerbFixed`/`OpenDeclaration` arms
already return `false`. Add an emitter fixture with a separate trailing literal
asserting the predicate is false there, and an english_v2 `is_err()` negative
on the sentence above (this closes the property the prior F3 left unpinned:
the existing two `is_err()` lines reach the predicate but carry no quoted
block). Record erratum: the landing record's "token-creation analysis the
walker omitted" is the WINNER (already admitted); the newly reachable loser is
`PredicateAdjunctPredicate` (elliptical fused determinative + duration
adjunct); the `//` card named is the Pest Problem face.

Gate scope: emit/ change → `cargo test --workspace`. Coverage must not move
(zero corpus exposure); a drop is a STOP. Standard constraints apply.
