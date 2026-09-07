---
needs: [english-v3-systemic-residuals]
---
# Cut production English consumers over to v3

Make the v3 all-Readings parser, generated AST and renderer the production
Oracle English interface. Migrate corpus, inspection and plugin-facing
consumers together so no adapter silently reselects a single Reading or routes
some inputs back through v2. Keep preference as an explicit non-destructive
view where a caller needs presentation order.

Retire replaced v2 scanner, parser, selection, compiler and generated-AST paths
after transferring every live regression witness and source owner. Remove
compatibility shims that would make v3 depend on v2 types. Decide whether the
fresh construction crates take the unversioned names only after all downstream
imports use the new contract; the rename is part of this cutover if chosen.

Acceptance proves no silent corpus loss; reports new and removed Readings with
wrong-analysis retirements separated from regressions; checks both roundtrip
laws and complete traversal; and leaves one production parser route. The
supported corpus may still have named long-tail no-Reading failures, but no
consumer may use v2 to conceal them. Every unit that v2 parsed correctly and v3
does not must appear in an explicit accepted-regression list owned by the
long-tail ticket. Retiring a v2 Reading as wrong requires a grammatical witness
showing why it was invalid. Cutover also adopts the corpus-runtime and forest
growth ceiling justified by the systemic report, rather than leaving measured
performance without an operational bound. Standard constraints apply.
