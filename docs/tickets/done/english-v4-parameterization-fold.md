---
needs: []
---
# Fold `Parameterization` into the `KeywordParameters` Category

From `english-v4-child-name-control-and-category-fission-audit`, finding G.

`Parameterization` (`Supplied`, `Omitted`) has one holder per value:
CompleteKeywordPhrase sets `Supplied` and BareKeywordMention sets `Omitted`
(`crates/deckmaste_english_v4/grammar/english.ron`, about lines 446–448). Its
only reader is CompleteKeywordItem, which requires `head.Parameterization`
`Supplied`. That is the phrase built from `KeywordParameters`, the Category
that already is a keyword with its parameters supplied.

Change:
- CompleteKeywordItem takes `Cat("head", "KeywordParameters")` directly.
- Remove the `Parameterization` feature, its equations, and its entry on
  `KeywordPhrase`.

A coordination of whole `KeywordPhrase`s can then no longer be a Keyword Line
item through CompleteKeywordItem. Parameters already coordinate inside the
`Parameters` schema, and Oracle separates keyword-line items with commas. If a
face is lost, STOP and report it.

Standard constraints apply.

Census delta (measured on `orvowoml` → `mqlxwspx`): faces read 28,800 → 28,801 (of 32,828), total Readings 237,320 → 237,339, Surplus 101,963 → 101,981; lost faces: none. Gained: Shay Cormac (*lose hexproof, indestructible, protection, shroud, and ward*, one Reading), removed from `deferred.txt` and from the stratum-7 long-tail ticket's one-off list (38 → 37 units). Removing the feature also lifted its accidental bar on coordinating complete with bare `KeywordPhrase`s, so six read units gained Readings (Katilda's Rising Dawn 10 → 20, Nahiri, Storm of Stone 7 → 10, Akroma's Memorial 4 → 5, Opal Guardian 2 → 4, Grotesque Hybrid 4 → 5, Akroma's Will 1 → 2): a bare *protection* in the list with *from …* attached as an ordinary Preposition Phrase. The cheapest tier is unchanged (singleton in 15,825 of 17,635 ambiguous units, summed 20,054, largest 8); the 28 judgments that asserted the removed `KeywordPhrase` node of a Keyword Line item now assert `KeywordParameters`.
Judgment set: every judgment holds (`cargo test -p deckmaste_english_v4`, `judgments.rs`, all tests pass); `strata --check` passes. The S/A/P/F tally has no tool in this tree.
