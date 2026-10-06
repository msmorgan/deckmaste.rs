---
needs: []
---
# Make the complete corpus run affordable without capping enumeration

The complete corpus run is the progress measure for this grammar, and it has
become too slow to use as one. Recorded figures, each for its own tree:

- 207,807 ms wall for all 32,828 supported faces, 1,031,953 ns/B checked-text
  thread CPU, 24 workers, host load 3.31 / 5.25 / 3.98 with concurrent test
  activity (tree `vwtnryos`, covered 13,226);
- 235,015 ms, 960,452 ns/B, 24 workers, load 3.59 / 3.83 / 3.64 (tree
  `ovrpzmym`, covered 11,249);
- on that second tree Avacyn, Guardian Angel alone took 150,638 ms to validate
  its 33,856 Readings after a 0.099 s chart.

The advisory ceiling is 16.26 s on a quiet host. The records conclude that
structural enumeration and validation, not chart admission, dominate. Cost
grows with coverage and with every legitimately ambiguous face, so later
grammar tickets make it worse.

Profile before designing: confirm where the time goes on the current tree and
how it is distributed across faces. Then reduce the cost of the complete run
while keeping it complete. The obvious candidate is to validate at the level of
the packed forest, so a shared substructure is checked once rather than once
per Reading that contains it; choose on the profile, not on this sentence.

Constraints. Every admitted Reading is still validated: declaration admission,
lexical ownership, byte-exact realization, construction and word traversal.
Enumeration is not capped, sampled or truncated to meet a time. If any per-face
budget is introduced, a face that exceeds it is reported as undetermined, never
as covered, and the budget and the list of such faces are disclosed. Admission
and Reading identity do not change. If a faster scheme would change what
"validated" means for a Reading, STOP and report before building it.

`english-v3-packed-preferred-readings` owns returning a preferred Reading
without enumerating; this ticket owns the complete census. They may share forest
machinery; neither substitutes for the other.

Acceptance: the complete run on one tree before and after reports identical
identities, Reading counts and validation results; wall time, per-byte thread
CPU, memory and the slowest faces are reported with host load and worker count;
and the ticket states the corpus-runtime and forest-growth ceiling that
`english-v3-production-cutover` adopts, with the measurement that justifies it.
Standard constraints apply.


## Landing record

### PROVE: completeness and structural validation

This changes census execution, not the admitted language. Construction and
lexical declarations are unchanged. The baseline executable was built from the
claim tree `uwktopko`; the implementation and final evidence are stamped
`rpsrztuw` / covered 13,622. Both use input snapshot
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`
and lexical inventory
`ab63eaed51a42b72ff4c5064b39163e5354f0699ea5d1b4d3d53cfbf1d4cb973`.
The baseline report captured the working change's identity after edits began;
its executable, declaration tree and pre-edit algorithm are recorded separately
here rather than attributing the baseline algorithm to the final source.

The census remains complete: no Reading limit, per-face budget, sampling of
validation or truncation is introduced. Every root performs declaration
admission, declared realization, contextual lexical ownership, byte-exact
comparison, and complete construction and word traversal comparison. Independent
materialization traces now retain structural values instead of hashing each
subtree repeatedly. Equality compares the full values and traces. A diagnostic
hash orders deduplication sets, with full equality on a hash collision.
Admission caches retain the complete grammatical value, including lexical
identity and correlated features, with the immutable Grammar and Lexicon fixed.
Independent lexical analysis is reused only after comparing rendered bytes;
a different rendering receives fresh analysis.

Generated materialization explicitly consumes only the production and ordered
child values. Memoization shares immutable results for that exact tuple, without
combining feature alternatives or pruning forest paths. Every returned Reading
still goes through the root checks. The generic iterator, chart callback and
admission equations are unchanged. Its materializer accessor exposes separate
request/unique counters; report schema 5 adds those counters without redefining
`readings.builds`.

No grammar guards or lexical load rules are added. There are zero new forbidden
word-named licensing checkers. The v3 path does not use the retired coverage-lock
or legacy permitted-checker census; its complete one/multiple census supplies
covered counts. Lexical-source loading and GrammarEnvironment compilation succeed
in the complete run. No linguistic term or Comprehensive Rules citation is added.

### DISCLOSE: implementation scope and assurance

No newly covered or retired identity, new grammatical analysis, construction,
schema, lexical entry or form literal is intended. Unique versus multiple
Reading census is preserved, with no destructive selection or specificity
preference introduced.

Deviations and additions:

- Optimize the v3 grammar, lexical runtime and xtask in the ordinary development
  profile. The baseline profile left these hot paths unoptimized.
- Add a reusable generated realization context and exact admission caches;
  index selected-frame candidates in their existing order and avoid cloning a
  marked argument merely to validate it.
- Expose production/child materialization as a generated method, removing
  unused chart Summary/State values from the census cache key. Add pointer
  reflexivity to structural ordering without changing value equality.
- Share independent materialization traces and compare their full values;
  retain diagnostic fingerprints only when needed for identity ordering or
  report samples. Preserve sample cost/identity ordering.
- Add four regression tests for cached/uncached complete-value equivalence,
  a changed lexical identity inside the same construction, fingerprint collision
  fallback, and independently constructed values with changed rendering and
  invalid correlated form/Number choices.

Assurance counts: restored 0; re-spelled 2; added 4; removed 0; ignored 0.
The re-spelled tests are
`validation_compares_exact_surface_and_both_traversal_identities` and
`type_line_census_uses_its_own_source_and_root_without_relabeling_rules_text`:
they use the new trace constructor/shared storage and retain their full outcome
assertions. No test assertion is weakened or deleted. No STOP occurred, no
validation obligation changed, and no owning-glossary gap was needed.

### REPORT: profiling and measured bounds

Profile before implementation, `uwktopko` / covered 13,622, 24 workers:
aggregate per-face lexical wall 22,078 ms, chart wall 1,226,846 ms and downstream
validation wall 1,227,669 ms. These are sums over concurrent faces, not corpus
elapsed time. Avacyn's chart was 170 ms but its 39,601 Readings took 262,789 ms
to validate. A 512-Reading selector profile on the same baseline source, one
worker (host load not captured; instrumentation, not acceptance timing), separated
materialization (0.423 s),
admission/rendering (1.644 s), subtree fingerprints/traversal (0.847 s) and word
traversal (0.008 s). This evidence selected exact reuse and removal of repeated
subtree fingerprinting, rather than changing what a validated Reading means.

Evidence exports, profiler code and resource measurements stay in
`/tmp/english-v3-census-tractability`; none enter source history. Corpus results
and cutover bounds follow below.


Final comparison (`rpsrztuw` / covered 13,622) preserves all 32,828 supported
face identities, complete enumeration status, per-face Reading counts, issues,
unknown lexical words, construction occurrences, chart metrics and retained
sample identities/costs. Both runs have 19,206 no-Reading faces, 6,653 one-Reading
faces and 6,969 multiple-Reading faces; all 176,171 Readings are checked, with
zero issues, undetermined/limited faces, duplicate Readings, cyclic derivations
or internal failures. No identity loses or gains coverage, so there is no
retirement or re-coverage obligation. Specificity-resolved selection is not
used; the complete unique/multiple census is unchanged.

The independent complete root identity exports contain 32,828 ordered face
records and all 176,171 full-value SHA-256 identities, and compare byte for byte.
Both the plain final iterator and the optimized tracing materializer produce
exactly the original complete identity export. The comparison also checks every
per-face chart/enumeration/construction result
and sample choice against the original executable's report.

Both complete commands use `english-v3 --all --workers 24`, with the default
two diagnostic samples per face and no Reading limit. Timings below exclude
compilation. The host has 24 logical CPUs (12 physical cores), AMD Ryzen
Threadripper 2920X, and 131,766,812 KiB RAM. Corpus wall excludes setup/report output;
process elapsed and peak RSS include them. Host loads are 1 / 5 / 15 minute.

| Tree / covered | Corpus wall | Checked-text thread CPU | Process elapsed | Peak RSS | Report host load | Workers |
|---|---:|---:|---:|---:|---|---:|
| `uwktopko` baseline / 13,622 | 336,892,780,897 ns | 1,501,585 ns/B | 389,241,492,573 ns | 4,723,880 KiB | 21.356 / 19.148 / 11.999 | 24 |
| `rpsrztuw` final / 13,622 | 40,092,252,599 ns | 343,653 ns/B | 48,883,009,191 ns | 7,431,844 KiB | 4.662 / 8.606 / 11.224 | 24 |

Process load start/end: baseline 24.597 / 19.681 / 12.091 to
8.535 / 12.455 / 11.671; final 4.719 / 8.684 / 11.263 to
17.054 / 11.576 / 12.118. The final measurement runs without this session's
other tests or exports executing concurrently. These are observed host loads,
not a controlled quiet-host comparison. Wall improves 8.4 times and the
checked-text CPU measure improves 4.4 times; memory increases 57.3 percent.
Retaining full structural traces and memoized values trades memory for time.
The unchanged source byte count is 5,204,548; checked-text bytes are 1,093,492.
The 16.26 s quiet-host advisory is still exceeded; this landing does not claim
compliance with it.

Slowest final faces, wall = chart plus complete downstream validation, all
`rpsrztuw` / covered 13,622, 24 workers and the final host load above. Baseline
uses `uwktopko` / covered 13,622, 24 workers and the baseline load above:

| Face identity / name | Readings | Baseline wall (ms) | Final wall (ms) |
|---|---:|---:|---:|
| `6da0b6d2-3c5e-49ef-9264-076269c04744#card` — Avacyn, Guardian Angel | 39,601 | 262,959 | 33,919 |
| `892c8fd1-9553-4ced-84ec-9fc35c314c76#card` — Grave Betrayal | 8,250 | 53,690 | 7,157 |
| `7747804f-ca49-4a68-b508-b94b27c52b0f#card` — Cruel Entertainment | 2,688 | 19,596 | 4,912 |
| `1d9b8859-14e3-4bbe-ad92-916b25a37b31#card` — Raiding Party | 3,520 | 42,510 | 3,469 |
| `2d44bdd2-aa31-415a-be71-d9e98ef12334#card` — Noctis, Prince of Lucis | 1,872 | 15,388 | 2,982 |
| `4baa6145-216e-476b-b178-aaaa1e633701#card` — Tempt with Discovery | 2,016 | 28,614 | 2,403 |
| `791cbb7c-7935-4902-b050-9ea9d030e9fa#card` — Osteomancer Adept | 2,538 | 20,107 | 2,053 |
| `27e83bae-0ffe-4cc7-a6c0-cbca25f2ba4e#card` — Gilraen, Dúnedain Protector | 1,300 | 13,273 | 1,966 |
| `0f68ae67-1671-4281-8b6b-3e52fdd4915f#card` — Salvation Swan | 1,254 | 11,841 | 1,873 |
| `18d0e6e1-3824-465d-9245-21e0174f551f#card` — Xira, the Golden Sting | 1,702 | 17,167 | 1,818 |

The final Avacyn selector (`rpsrztuw` / covered 13,622, one worker,
load 5.645 / 9.238 / 11.519) checks all 39,601 Readings in 11,424,905,590 ns
corpus wall and 41,417,939 ns/B; contention in the 24-worker complete run makes
its elapsed validation longer. No selector result replaces complete evidence.
Across the complete census 4,626,951 leaf requests reuse 178,306 unique leaves;
6,641,795 build requests reuse 1,489,665 unique materializations. The unchanged
generic iterator reports 208,999 requests and 176,171 complete derivations.

#### Bounds adopted by production cutover

Use a fixed 48-second corpus-wall acceptance ceiling for the complete pinned
corpus on this 24-worker host, excluding compilation, setup and report output,
with the same complete validation and default diagnostic sampling. Repeat the
measurement on an otherwise idle host (initial one-minute load at most 6) before
cutover; disclose all load values and worker count. The final 40.092-second
observation supplies 19.7 percent headroom, while the fixed threshold prevents
later tickets from ratifying each slower run. The 16.26-second advisory remains
a separate improvement target. Peak process RSS must remain within 12 GiB;
this is 69 percent headroom over the measured 7.09 GiB, not a Reading budget.
A slower machine needs an explicitly recorded calibration, not a silent ceiling
increase. These are acceptance checks, never enumeration cutoffs.

Forest-growth acceptance ceilings for the same pinned corpus are 100,000,000
items/intermediate nodes and 110,000,000 total packed families, against measured
82,606,278 and 87,110,423 respectively (21 and 26 percent headroom). In addition,
per-face ceilings are 20,000 items/intermediate nodes, 24,000 total packed
families, 1,000 completed nodes and 1,100 completed families. The measured maxima
are 14,178 items/intermediate nodes and 15,731 families on Tempt with Discovery
(`4baa6145-216e-476b-b178-aaaa1e633701#card`, 354 analyzed bytes), and 588 completed
nodes / 752 completed families on Avacyn
(`6da0b6d2-3c5e-49ef-9264-076269c04744#card`, 274 analyzed bytes).
All figures are identical before/after, `uwktopko` and `rpsrztuw` respectively,
covered 13,622 in each case. These absolute bounds leave approximately
41–70 percent per-face headroom while preventing a grammar change from hiding
local growth within corpus aggregates. A bound failure requires investigation
and an explicit revised measurement/ruling; it never makes a truncated face
covered. Reading counts themselves are unbounded by this ticket.

Declaration economy (`rpsrztuw` / covered 13,622) is unchanged: 175 ordinary
constructions plus 44 schemas (219 forms), 134 Categories, 2,758 declaration
lines, 537 static and 595 compiled productions; the census uses 166 construction
names. The 19 unsupported lexical frame declarations are unchanged residuals.
The homograph and form-literal/vocabulary overlap inventories are unchanged
from `english-v3-by-complement-functions`: the lexical inventory hash is exactly
the same and no declaration changes. Overlap inventory: empty.

Homograph inventory: 120 surfaces (provenance, not a gate):

`'d`, `'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `time`, `to`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’d`, `’s`, `∞`.


Verification on `rpsrztuw` / covered 13,622:
`cargo xtask gate --changed --run --clippy` exits zero. Its derived command is
`cargo test -p deckmaste -p deckmaste_construction_core -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`,
followed by the same package closure's `cargo clippy --all-targets -- -D warnings`.
All 1,246 tests pass, zero fail, and one pre-existing test remains ignored:
`macros::templates::tests::macro_schema_census_count_matches_21`, whose attribute
names its existing on-demand corpus cross-check. This includes the four new
regressions and the four inherited Lean gate integration tests. The broad
closure also includes packages affected by coordinator changes not yet
incorporated at measurement; it does not make them this ticket's modifications.
`cargo fmt --all -- --check` passes. Citation validation reports zero
non-compliant strings and zero stale citations across 16,057 sites; the diff
adds zero Comprehensive Rules citation sites, so no bless is needed.


After unconditional Kata refresh, `rpsrztuw` retains the same English sources;
coordinator Semantics/helper-parameter changes are incorporated without conflict.
Both the previous executable and the rebuilt declaration reader load the new
plugin files with exactly the same lexical-inventory hash and residual-source
list. The complete Seedborn Muse selector preserves Reading count, chart,
construction and traversal results with zero issues. These checks justify
preserving the complete corpus identities, validation and performance evidence
above for the unchanged English behavior. The final verification amendment is
stamped `qtozkusu` / covered 13,622; it changes this landing record only.

The additional consumed-plugin check `cargo test -p deckmaste_lexical_source`
passes all 55 tests, with zero failures or ignores (`qtozkusu` / covered 13,622).
Formatting and both citation checks pass again after refresh.

Final refreshed `cargo xtask gate --changed --run --clippy` exits zero
(`qtozkusu` / covered 13,622). The derived command is
`cargo test -p deckmaste -p deckmaste_construction_v3_core -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`,
followed by the same closure's `cargo clippy --all-targets -- -D warnings`:
646 tests pass, zero fail, and the same 1 pre-existing on-demand test
remains ignored. The four inherited Lean gate tests pass on the refreshed
workbench. Citation checks report zero non-compliant strings and zero stale
citations across 16,068 sites on this refreshed tree.

The final refresh also incorporates
`semantics-v2-declared-definitions-are-unchecked` (definition-check command,
Semantics Lean emission and workbench checks). No English source or lexical
metadata changes. The final rebuilt reader again preserves the exact lexical
hash, residual list and selector results. Unaffected complete-English evidence
is retained. For the affected code, `cargo test -p xtask` passes 380 tests,
zero failures and the same 1 pre-existing ignore; all seven refreshed Lean gate
integration tests pass. `cargo clippy -p xtask --all-targets -- -D warnings`
exits zero. This final amendment is stamped `qtozkusu` / covered 13,622.
Final citation checks report zero non-compliant strings; checked 16059 citations against cr.txt (eff. 2026-09-25); 0 stale.

### Addendum: census validation fault tests (2026-10-06)

Added four tests using the existing card-draw trace through `validate_in_context`:
`census_validation_reports_a_one_byte_surface_mismatch` (`Roundtrip`),
`census_validation_reports_a_wrong_lexical_identity_at_one_leaf` (`LexicalTraversal`),
`census_validation_reports_a_duplicate_reading` (`DuplicateReading`, using the
census caller's counting key), and
`census_validation_rejects_cached_reading_for_different_source_text` (`Roundtrip`
after warming the same admission context). The tests compare the issues and
counting behavior with the old validation path; no production code or fixtures
are added or changed. Assurance: added 4 / restored 0 / re-spelled 0 / removed 0 /
ignored 0.

Validation: `cargo test -p xtask english_v3` and
`cargo xtask gate --changed --run --clippy` pass. The gate runs
`cargo test -p xtask` and `cargo clippy -p xtask --all-targets -- -D warnings`.
Formatting, Kanban and both citation checks pass.

The opt-level-2 dev-profile change's share of the reported 8.4× speedup was not
measured separately.
