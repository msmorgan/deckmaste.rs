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

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
