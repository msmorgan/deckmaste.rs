---
needs: [english-v3-lexical-model]
---
# Detach v3 lexical sources from English v2 tooling

Replace the private `xtask::english_v2` lexical-source path with a v3-owned
source module whose small interface returns normalized lexical declarations.
`deckmaste_lexical_model` remains data-only and `deckmaste_lexical` remains the
analysis/realization engine; source discovery, authoring-format adaptation and
inventory assembly sit behind a separately testable adapter. An English v3
consumer must not depend on xtask, `deckmaste_english_v2`, or the deletion-bound
v2 construction compiler to obtain a `Lexicon`.

Move the census to a neutral `cargo xtask lexical` command and remove the
`english_v2 lexical` entry point. Existing v2 declarations may be consumed only
through an explicitly transitional input adapter while the lexical-inventory
ticket consolidates their contents; they are not the durable interface or
source identity. Rust must not name an individual RON source file, directly or
by reconstructing a declaration identity from its filename. Discover source
trees generically and take identities from normalized declarations.

Carry the review follow-ups with the seam repair: replace the two plain
construction/readback model tests with behavioral serialization and consumer
tests, explicitly document that the new `FrameItem` export shape is not
backward-compatible with prototype artifacts, and replace the glob model
re-export with an explicit public surface.

Acceptance proves that the library-owned source interface assembles and indexes
the full current inventory, rejects unresolved supplemental overrides, and
reproduces the lexical census and independent realization/reanalysis law without
an exported-file handoff. Repository searches must show that the new durable
pipeline neither calls `xtask::english_v2` nor names a RON file. Preserve all
licensed readings and source provenance; do not broaden this repair into the
planned lexical inventory's vocabulary expansion. Standard constraints apply.

## Landing record

All figures below were measured on change `xpvwuznw` after refreshing against
the coordinator's flavor-word changes. The final refresh was a no-op, so the
reported test and census results apply to the integrated parent.

### PROVE

`deckmaste_lexical_source` is now the library-owned source boundary. Its public
`load_workspace` interface returns normalized `LexicalSources`; callers do not
observe the private legacy document schemas. The full source-tree test loads
the current inventory, selects a real declaration containing a correlated
marked frame without naming its identity or file, serializes and deserializes
that declaration, indexes it, and proves independent realization/reanalysis.
A separate boundary test proves that an unresolved supplemental override is an
error rather than a silently dropped edit.

The builtin declaration reader now uses directories only to bound discovery
and paths only as provenance. Kind, subtype category, and declaration name
come from normalized content. Its regression deliberately places a keyword
ability under a misleading keyword-action filename and a land subtype under a
nested creature path; both retain their authored content identities. Existing
catalog-completeness tests now compare catalogs with normalized declaration
identities instead of file stems. The durable lexical source and census Rust
paths contain neither an individual RON filename nor an
`xtask::english_v2`/`english_v2::lexical` call.

`cargo xtask lexical` is the sole corpus census entry point. It obtains the
inventory directly from the library adapter, indexes it in memory, checks every
independent value through realization and reanalysis, and only writes a RON
inventory when the explicitly unstable `--export` inspection option is used.
The old `cargo xtask english_v2 lexical` command and its private loader modules
are removed.

### DISCLOSE

The source crate's private `legacy` module remains an explicitly transitional
quarry for the current English-v2-authored source tree. It uses
`deckmaste_construction_core` to normalize declaration content, but `cargo
tree -p deckmaste_lexical_source` contains neither
`deckmaste_english_v2` nor `deckmaste_construction_compiler`; no v3 consumer
depends on the parser or deletion-bound compiler. The planned lexical-inventory
work still owns migration of those source contents into v3-native declarations.

The refreshed `FlavorWord` declaration-kind removal is preserved; this change
does not restore or replace it. No lexical entry, construction, parser
admission, or vocabulary expansion was added. The old path/content mismatch
errors were retired with their subject because filenames no longer participate
in identity validation; malformed source and duplicate content identities
remain errors at their provenance paths.

The two model-only construction/readback tests were removed. Their assurance is
replaced by the source-boundary serialization/consumer test over an actual
authored marked-frame declaration. `deckmaste_lexical` now explicitly re-exports
the shared model surface, and its documentation says prototype RON exports,
including the changed `FrameItem` shape, are not backward-compatible and must
be regenerated. No tests were ignored.

### REPORT

The full current census passed:

- 32,641 supported faces and 914,801 word occurrences across 6,824 distinct
  words;
- 168,473 unknown occurrences across 3,148 distinct words;
- 1,726,873 matched readings;
- 34,447 lexical source owners and 38,018 independently realized/reanalyzed
  values;
- 240 retained unmapped-source/policy reports.

The input SHA-256 was
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`.
Reproduce it with:

```sh
cargo xtask lexical --data data/mtgjson/AtomicCards.json \
  --output /tmp/deckmaste-english-v3-lexical-census.json
```

`cargo xtask gate --changed --clippy --run` derived and passed the test set for
`deckmaste`, `deckmaste_construction_core`, `deckmaste_construction`,
`deckmaste_english_v2`, `deckmaste_lexical_model`, `deckmaste_lexical`,
`deckmaste_lexical_source`, and `xtask`, followed by the matching all-targets
Clippy command with `-D warnings`. An earlier attempt exposed one stale
`map_or` idiom in a touched declaration test; it was mechanically changed to
`is_none_or`, and the complete gate then passed. Focused source and construction
reader suites also passed. STOP: none.
