---
needs: [english-v2-lexical-analysis, english-v3-lean-proof-audit]
---
# Extract the shared lexical model for English v3

Create the data-only `deckmaste_lexical_model` crate as the stable vocabulary
between lexical analysis, generated constructions and English v3. Move only
types whose meaning is shared: Lexeme identity, lexical Category, Word Form,
applicable grammatical features, lexical Frame signatures and the compact
lexical values needed by grammar and realization. A Frame value identifies its
ordered grammatical slots and their licensed complement categories without
embedding a grammar Production or admission algorithm. Preserve correlations
within one Lexical Analysis.

Keep tokenization, morphology algorithms, declaration loading, catalogs,
indexes, occurrences, source provenance and realization behavior in the deep
`deckmaste_lexical` module. The shared crate contains no I/O, lookup service,
parser state, source positions, alternative container invented for one caller,
or construction/compiler concepts. `deckmaste_lexical` re-exports shared public
types where that gives callers one coherent lexical interface.

Pin the dependency graph so the lexical engine and construction compiler can
both depend on the model without depending on each other. Migrate the current
v3 prototype, its position-indexed occurrence interface and lexical tests
without changing the set of licensed analyses or realizations. That interface
must continue to emit bound, overlapping and multiword alternatives before the
packed chart depends on it; the later inventory ticket expands and reconciles
their declarations rather than introducing the capability. Acceptance includes
the existing `counters`, `cast` and `one` alternatives, feature applicability,
Frame values, declared spelling variants, capitalization, overlapping multiword
occurrences and independent realization/reanalysis tests. Standard constraints
apply.
