# deckmaste_english_v4

Oracle English for English v4
([`docs/decisions/english-v4.md`](../../docs/decisions/english-v4.md)): the
grammar data file [`grammar/english.ron`](grammar/english.ron) and its binding
into `deckmaste_syntax`.

The grammar names Categories, features, Schemas and core closed-class lexemes
only. Vocabulary and Subcategorization Frames come from the lexical environment
the caller binds it with; until `deckmaste_lexical_source_v4` exists, the only
environment is the seed in `tests/seed/`.

## Seed proof

`tests/seed_proof.rs` proves the frame engine end to end on a seed of about
fifteen open-class words: active complementation, same-head complement priority,
Host-Category-limited attachment with cross-head ambiguity retained (including
Seedborn Muse), passives with and without `by`, object and stranded-preposition
Gaps across finite, progressive, perfect and passive forms, and Right Node Raising.
Every Reading is admitted and realizes to its input; hand-rebuilt Readings reparse.
`chart_and_forest_sizes` prints chart items, forest nodes, families and Readings
per seed sentence.

## Not yet here

Catalogs (`english-v4-lexical-catalogs`), unit splitting, reminder-text stripping,
the census and complexity strata (`english-v4-corpus-stratification-and-judgment-harness`).
