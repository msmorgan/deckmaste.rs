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

## Landing record

All figures below were measured on change `kwyoqxqs`, whose unchanged parser
coverage lock has `covered`: 20,254. This landing extracts lexical data and
does not change construction admission or selection.

### PROVE

`deckmaste_lexical_model` is the data-only vocabulary shared by lexical
analysis and future English v3 grammar code. It owns Lexeme identity, Category,
Word Form, correlated applicable features, Frame/FrameSlot signatures, and
compact declared-word and numeral values. Its only dependency is Serde.
`deckmaste_lexical` depends on and re-exports that vocabulary while retaining
declaration loading and provenance, morphology, tokenization and binding,
indexes, position-indexed occurrences, numeral codecs, and realization. No
construction, parser, catalog-I/O, source-position, lookup, or compiler state
entered the shared crate.

Frame arguments carry an explicit ordered `FrameSlot`; a fixed marker and its
licensed slot remain one correlated `Marked` value rather than an untyped
sequence. The adapter discovers the macro inventory through
`read_builtin_v2`, takes identity and provenance from each normalized
declaration, and neither names nor reconstructs an individual macro RON file.
The full authored-source regression test exercises that interface without
naming a declaration or source file.

There is no silent lexical loss. On the complete Vintage-supported input, exact
realization succeeded for 1,728,377 matched readings and independent
realization/reanalysis succeeded for all 38,648 indexed values. The census
retains 35,077 lexemes and the same 914,801 word occurrences and 167,609
unknown occurrences recorded by the originating lexical-analysis landing.
Focused witnesses retain the `counters` Noun/Verb readings, all 13 licensed
`cast` readings, the Noun/Determinative/numeral alternatives for `one`, feature
applicability, declared spelling variants, capitalization, bound and
overlapping multiword occurrences, and Unicode-derived positions.

The only identities re-spelled during the stopped census were stale
supplemental override targets left by the settled camelCase declaration-name
migration: `Tap`→`tap`, `Untap`→`untap`, `Surveil`→`surveil`, `Fight`→`fight`,
`Behold`→`behold`, `Earthbend`→`earthbend`, `Airbend`→`airbend`,
`CollectEvidence`→`collectEvidence`,
`VentureIntoTheDungeon`→`ventureIntoTheDungeon`, `TimeTravel`→`timeTravel`,
and `ManifestDread`→`manifestDread`. These were invalid references that made
the source join fail before analysis, not removed lexical coverage. No second
lexeme owner or morphology rule was introduced.

No word-naming licensing guard was added. The production no-word-naming census
and environment validation tests pass; names occur only in declarations and
regression fixtures. No construction or covered card identity was added or
removed, and the final census reports zero internal lexical failures.

### DISCLOSE

No parser coverage, construction, selection, or preference result is claimed.
The parser lock, construction inventory, selection census, licensing-checker
total, homograph inventory, and form-literal/vocabulary overlap inventory were
not remeasured because their code and admitted grammar are unchanged. The
lexical census independently confirms the intended overlap witnesses:
`counters` has 2,898 ordinary occurrences, `cast` 6,984, and `one` 3,886;
all 3,886 `one` occurrences also retain a numeral match.

Deviations and additions:

- Added the new shared crate and a one-way dependency from the lexical engine;
  no current deletion-bound construction compiler was wired merely to create a
  consumer.
- Replaced the source adapter's marker-plus-argument sequence with the shared
  correlated `Marked` frame item. Optionality is retained, while unreconciled
  literal markers remain explicitly identified for the planned inventory
  reconciliation.
- Moving `Numeral` into the data-only crate required a re-exported
  `NumeralCodec` trait so parsing and formatting implementation remains in the
  deep lexical module.
- The full census exposed the stale camelCase override targets above. Work
  stopped while their neighboring ticket was claimed; after it returned to
  planned, the settled declaration-name ruling supplied authority for the
  narrow data repair. The full census and source-tree regression then passed.
- Tests restored 0, re-spelled 1, newly ignored 0, added 5, removed 0. The
  re-spelled frame-adapter assertion preserves the same optional marked-role
  outcome against the correlated replacement shape.

No unresolved STOP or Oracle English glossary gap remains. No CR citation site
changed.

### REPORT

The exported inventory contains 35,077 lexemes: 33,358 Catalog, 227 Core, 938
Keyword, and 554 Plugin source owners. The corpus contains 32,641 supported
faces, including 32,285 with text; 914,801 word occurrences across 6,824
distinct words; and 167,609 unknown occurrences across 2,581 distinct words.
The report retains 242 named source issues and policy notes rather than treating
them as missing-word totals.

Lexical telemetry: loading 0.455 s, indexing 0.154 s, independent-value law
0.224 s, and analysis/accounting 11.466 s. These are lexical process timings,
not the parser's accepted-byte metric or its quiet-host 16.26 s ceiling. Parser
performance was not remeasured because parser behavior is unchanged.

Validation: `cargo xtask gate --changed --clippy --run` derived and passed
`cargo test -p deckmaste -p deckmaste_english_v2 -p
deckmaste_lexical_model -p deckmaste_lexical -p xtask` and the matching
all-targets Clippy command with `-D warnings`: 1,026 active tests passed, zero
failed, with one pre-existing on-demand macro-schema census test ignored.

Reproduce the lexical census with:

```sh
cargo xtask english_v2 lexical --data data/mtgjson/AtomicCards.json \
  --output /tmp/english-v3-lexical-model-corpus.json \
  --export /tmp/english-v3-lexical-model-export.ron
```

Measured input SHA-256:
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`.
Session artifacts: `/tmp/english-v3-lexical-model-corpus.json` and
`/tmp/english-v3-lexical-model-export.ron`.
