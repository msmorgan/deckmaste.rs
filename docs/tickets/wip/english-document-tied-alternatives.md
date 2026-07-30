---
needs: []
---
**`ParseSelection::tied_alternatives` has no doc comment on its public
accessor, and a wrong assumption about its meaning already cost real work.**
An invariant in the new adversarial-corpus tool was built on the assumption
that a non-empty `tied_alternatives` meant the parser had no basis to prefer
the tree it returned. That assumption is wrong.

Evidence:

- 29,490 of 31,685 printed supported faces (93.07%) have at least one tie.
- 28,899 of those tie under rule 176 alone — 98% of all tied faces.
- `deckmaste_english`'s own test
  `provenance_identifies_selected_rules_without_copying_source`
  (`crates/deckmaste_english/src/parse.rs`) asserts
  `!selection.tied_alternatives.is_empty()` for the input `"Draw a card."` —
  a passing, intentional assertion, not an oversight.
- `Forest::best_root_matching`'s doc comment
  (`crates/deckmaste_english/src/forest.rs`) states it "Ranks complete roots
  by the ordinary cost and stable-node tiebreak, returning the first root
  accepted by `matches`."

So the field records cost-equal packed alternatives resolved by a
deterministic, documented tiebreak — normal, expected behavior on the
majority of the corpus, not a defect signal. `ParseSelection::tied_alternatives`
(the `#[must_use] pub fn tied_alternatives` in
`crates/deckmaste_english/src/parse.rs`) currently carries no doc comment
explaining this. Add one, so the next reader who finds it non-empty does not
repeat the mistake of treating it as a parser-uncertainty or defect signal.

Standard constraints apply.
