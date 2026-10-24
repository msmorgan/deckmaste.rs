---
needs: []
---
# Stratum 7: enforce the Stratum 7 judgments

Part of `english-v4-stratum-7-discourse-and-long-tail`. Raise `COMPLETED_STRATUM` in `crates/deckmaste_english_v4` from 6 to 7, so every Stratum-7 judgment is enforced and holds, and every Stratum-7 unit still unread is deferred to the open slice ticket that owns it. The umbrella stays open until those slices land.

## Outcome

Six judgments failed once enforced. Each was a judgment defect, so each was rewritten. None was retired.

- `card_noun_premodifier.txt` (Open the Gates): the negative assertion `!Nominal "card or Gate"` is dropped. *[card or Gate] card* is a well-formed coordinated noun premodifier, and only its meaning is vacuous. The entry asserts the intended conjuncts (README: assert the intended analysis, not that a dispreferred one is absent).
- `clause_final_predicative_as.txt` and `focusing_only.txt` (Counterbore): the assertion `!PrepositionPhrase "as that spell" as Adjunct of search` is dropped. *the same name* used anaphorically, followed by a predicative *as* Adjunct, is a grammatical attachment. The intended *same … as* Complement is still asserted, and so is the word-order negative on *exile*.
- `exactly_cardinal_modifier.txt` (Amalia Benavides Aguirre): the predicative *exactly 20* is asserted as the Noun Phrase that is the Predicative Complement of *be*, not as a Determiner Phrase. This follows the README convention that numeric predicatives keep their Noun Phrase analysis.
- `instead_replacement.txt` (Darksteel Colossus): the wide-scope group asserts the coordinated imperative `Clause`, not a `VerbPhrase`. A chain of bare imperatives is coordinated Clauses (english-lexical-analysis §2).
- `keyword_verbs.txt` (Skyway Robber): the entry drops its `Clause` mark. In logical order, the closing period belongs to the sentence, so the exact corpus unit reads only as the root.

Forty unread Stratum-7 units are deferred in `deferred.txt` under their owning tickets: long-tail-edge-cases 11, long-tail-quotes-and-mentions 7, long-tail-pp-coordination 5, long-tail-quantifiers 5, heavy-object-shift 5, long-tail-nonfinite-clauses 3, gapping-remnants 3 and cleave-transparent-brackets 1.

1. **Census delta** (zwktomys): fully read faces 32,769 → 32,769; total Readings 337,348 → 337,348; lost faces: none.
2. **Judgment set** (zwktomys): all entries hold at completed Stratum 7 (3,539 enforced, 3 pending as deferred units).
