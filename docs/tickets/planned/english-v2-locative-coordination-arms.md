---
needs: []
---
# Locative noun-phrase coordination gets its `or` and `and/or` arms

**R7 — Group R.** Smallest ticket in the group.

Defect. Every other coordination family in the grammar carries the declared
three-way split (`and` / `or` / `and/or`) — predicate
(`crates/deckmaste_english_v2/src/constructions.rs:1623, 1636, 1649`), bare
predicate (`:1662, 1675, 1688`), clause, nominal, full noun phrase, and mana.
`LocativeNounPhraseCoordination` has **one** arm,
`locative_and_noun_phrase_coordination`. `Exile target creature card from your
graveyard or your hand.` therefore has no derivation. `KeywordQualityCoordination`
has the same single-arm shape.

No English fact excludes `or` and `and/or` from a coordination of locative noun
phrases; the arm set stopped where the corpus stopped.

Pinned shape. Complete both triples with the same positional separator algebra
and the same three semantic constructions the other families use. Witness counts
for the new arms are provenance in the landing record, never a condition on
declaring them (rewrite ADR, "Amendment: attachment class is a declared
linguistic property (2026-09-04)").

Fences. Declaring only the arm a witness was found for. A shared arm that stores
the coordinator spelling. Any census used as a gate.

Glossary: Coordination, Coordinator, Locative, Noun Phrase. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` — re-measure at claim. Standard
constraints apply.
