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

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
