---
needs: []
---
# Delete the unheld `InterrogativeClause` value of ComplementKind and ObliqueKind

From `english-v4-child-name-control-and-category-fission-audit`, finding G.

The load report lists ComplementKind and ObliqueKind `InterrogativeClause`
with no holder. Only verbs declare a ContentClause frame
(`crates/deckmaste_lexical_source_v4/lexicon/english/verbs.ron`), and the Verb
Valence never sets ComplementKind. No noun or preposition frame projects the
value, and no `english.ron` equation reads it.

Delete `InterrogativeClause` from the `ComplementKind` and `ObliqueKind`
domains in `crates/deckmaste_english_v4/grammar/english.ron`. The
`InterrogativeClause` Category and its `slot_categories` entry stay. If
binding then fails, because some frame projects the value after all, keep it
and report which frame does.

Standard constraints apply.

Census delta (measured on `mqlxwspx` → `tzlyktoq`): faces read 28,801 → 28,801 (of 32,828), total Readings 237,339 → 237,339, Surplus 101,981 → 101,981 (the `strata.json` report is byte-identical before and after); lost faces: none. Binding succeeded, so no frame projects the value.
Judgment set: every judgment holds (`cargo test -p deckmaste_english_v4`, `judgments.rs`, all tests pass); `strata --check` passes; cheapest tier unchanged (singleton in 15,825 of 17,635 ambiguous units, summed 20,054, largest 8). The S/A/P/F tally has no tool in this tree.
