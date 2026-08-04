**Prune and re-home the chart parser's closed vocabulary.**

Three lexicon defects, one gate set:

1. **Lexicalize the measured-value nominals.** `mana value` and `life total`
   parse compositionally today — `mana`/`value` and `life`/`total` are
   independent `noun_count` rows in `regular-vocabulary.tsv`, composed by the
   noun-modifier nominal machinery (the grammar tests' "property nominal"
   handling). Both are rules-defined terms, not descriptions, and the
   compositional encoding over-generates: bare `value` as a head noun,
   `mana` as a general-purpose modifier. Lexicalize each as a single unit —
   the hand-curated rules-defined catalog kind already documents "the
   measured value nominal" as intended membership — while keeping the
   property-nominal family's uniform treatment of `power` and `toughness`,
   which are genuinely single words.

2. **Census and delete dead `Vocab` entries.** The closed word list in
   `crates/deckmaste_english/src/word.rs` (~179 registrations) carries
   entries with no consumer: `abandon` is registered as a regular verb,
   referenced nowhere else in the crate, and still live in the scanner (any
   `abandon` in input matches it). Script a per-variant reference census and
   delete the zero-reference rows rather than hand-picking.

3. **Migrate regular-declining keyword actions to the catalog path.**
   `adapt` is hand-listed as a regular `Vocab` verb even though
   `CatalogKind::KeywordAction` and `Verb::KeywordAction` already provide
   catalog-driven keyword-action verbs with their own rendering and
   predicate frames. Migrate such entries to the catalog path, remove the
   bespoke registrations, and check whether a bespoke entry currently
   shadows (or races) the catalog reading of the same surface.

Gates: `cargo xtask english roundtrip --require-clean`; recovery census
byte-identical (no supported face may lose its parse to a deletion); the
full `deckmaste_english` suite including the registration-permutation
fixtures; `deckmaste_spelling` compile/unify/render gates stay green.

Standard constraints apply.
