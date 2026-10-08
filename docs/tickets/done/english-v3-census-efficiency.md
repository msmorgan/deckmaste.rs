---
needs: []
---
# Census efficiency: digest dedup, bounded caches, per-face Readings digest, lexicon-impact incremental mode

## Why

On 2026-10-07 a permissive-admission experiment pushed one census to 27 GB.
A read-only review then found that census memory grows with the number of
Readings (O(Readings)) in six places, none of them bounded (lines as of claim
time):

1. The per-face checked set, `crates/xtask/src/english_v3/mod.rs:220` and
   `:258`. It keeps the hex SHA-256 identity string plus the Reading's AST
   `Arc` for every checked Reading.
2. The enumerator's `seen: BTreeSet<M::Reading>`,
   `crates/deckmaste_english_v3/src/readings.rs:82` and `:218`. It keeps every
   distinct `TracedValue` together with its trace vectors.
3. The enumerator's `pending: Vec<Work<…>>`, `readings.rs:81`. It clones work
   items at `:119`–`:121`.
4. The materialization memo `Tracing.leaves` and `Tracing.builds`,
   `crates/xtask/src/english_v3/validation.rs:96`–`:97` (inserts at `:155`
   and `:182`).
5. The AdmissionCache `summaries` map,
   `crates/deckmaste_construction_v3_core/src/runtime.rs.txt:557` (insert at
   `:590`).
6. The all-faces report collector, `analyze_cards` in `english_v3/mod.rs`
   (`par_iter().map(analyze_face).collect()`), which holds every `FaceReport`
   until the end of the run.

The experiment built the small variant: dedup by digest, drop the retained
traces, and cap the memo and AdmissionCache at 100,000 entries each, clearing
when full. That code is the reference. It lives on the archived line
`archive-english-v3-licence-experiment-ref` (prkrlprq, "experiment (reference,
never integrate)", measured on base lryqtwskmyxs). Its census part is the
saved diff `census-memory.diff`, which touches `readings.rs`,
`validation.rs`, `english_v3/mod.rs` and `runtime.rs.txt`. The experiment
measured it on the baseline grammar with 4 workers:

| | before | after |
|---|---|---|
| peak RSS | 8,313,392 kB | 5,416,292 kB |
| corpus wall | 221,777,967,803 ns | 156,254,326,458 ns |

Results were identical face by face. The user approved landing it
(2026-10-07). Do not integrate the archived line itself: its
permissive-admission half is not part of this ticket.

## Goal

**A. Digest dedup, trace drop and bounded caches.** Land the census-memory
change. Prove census identity face by face against the claim parent: the
covered count, the Reading count for each face, and the inventories. Report
peak RSS and wall time before and after, at 4 workers and at 12. Every Reading
is still fully materialized and validated. Assurance is unchanged and nothing
is sampled. A bounded cache may only cost recomputation, never a result.

**B. A Readings digest for each face in the census JSON.** Hash the sorted list
of per-Reading digests for the face with a stable hash. Two runs can then be
compared by digest, which catches a face whose count stayed the same while
its Readings changed. Define the digest input exactly: which per-Reading
digest is used, how it is encoded, and how the list is sorted and separated.
Put a version field in the JSON so the definition can change later.

**C. An incremental mode.** Given an earlier census JSON and the current tree:

- diff `crates/deckmaste_lexical_source/lexicon/{core.ron,verbs.ron,vocabulary.rs}`;
- find the affected lexemes, then their surface forms;
- re-check only the faces whose text contains one of those forms;
- copy the earlier results for every other face.

Mark the output as incremental and record the path of the base census. A
grammar change (`declarations.rs`) turns incremental mode off, because the
method is sound only for changes confined to the lexicon. The one exception
would be a chart-level record of predicted categories. That record is out of
scope here and is a possible follow-up. One full `--all` run per landing
remains the authority.

## Method

Standard constraints apply. Land as a series: A, then B, then C.

- A: the identity proof above.
- B: a stability test. Running the same tree twice gives identical digests,
  and one lexicon change alters only the expected faces' digests.
- C: a soundness test. On a lexicon-only change, the incremental result must
  equal a full run. Assert this on the diff of a real past landing, e.g.
  `english-v3-combat-interval-noun`.

Timings are integer ns. No new fixtures are checked in. Evidence goes under
`target/english-v3/`.

## Out of scope

Sampled validation; counting at the forest level; tracking predicted
categories at the chart level (the follow-up above).

By-name corpus selection and nickname derivation are owned by `card-corpus-fast-lookup`.

## Landing record

### Provenance and scope

Implemented as A (`woulmsvmqqkpwlqkprtpqympovupzqtv`), B (`mtyqtmtz`),
and C (`uxoypzxwqxvnyoxyqltqrnqxvkuwvtns`). A takes only the census-memory
part of the archived `prkrlprq` reference, not its licence neutralization.
`declarations.rs`, `core.ron`, `verbs.ron`, and `vocabulary.rs` remain
byte-identical to the claim baseline. A/B/C add or remove no Construction, schema, lexical frame,
Lexeme or Word Form; the imported connive delta is recorded below.

The user's 2026-10-07 instruction replaces the ticket's 4/12-worker measurement
request with **6 workers** and limits full corpus runs to the identity proofs.
There are four full enumerations: A before, A after, C before the final
refresh, and the refreshed-tree D authority needed after the connive import.
All evidence, temporary fixtures and measurement helpers are in the workspace's
ignored `target/english-v3/census-efficiency/`; none is tracked. Commands below
use that directory as `E` for brevity. Each full enumeration uses
`english-v3 --all --workers 6 --samples-per-face 0 --output E/<label>.json`,
wrapped by `E/measure.py` for process wall time and peak RSS.

### PROVE: no silent loss and structural laws

A compares every one of **32,828 face identities**, not just totals. Covered
faces stay **21,074** and total checked Readings stay **685,707**. Every face's
source hashes, unknown words, enumeration/census classification, exact and
checked Reading counts, chart and enumeration counters, Construction inventory,
and issues match. The input, support filter, lexical inventory hash, residual
and unknown-word inventories and unmapped source inventory also match.
`E/a-identity-proof.json` records zero differences. Only materialization memo
miss counters change on two faces: bounded eviction causes recomputation.
No identity loses or gains coverage; no re-coverage obligation or regression
needs routing.

Every counted Reading is still materialized and validated: byte-exact
realization, contextual lexical ownership/admission, full Construction traversal
and full Word traversal against independent materialization traces. The census
retains all grammatical Readings. The default enumeration API still deduplicates
by complete value equality, including its fingerprint-collision fallback; only
the census opts into SHA-256 keys. Fault tests retain full value comparisons,
and a changed trace value now demonstrably changes the census key.

Both A corpora have complete enumeration on all faces, zero issues, zero internal
failures, zero cycles and zero duplicate-Reading reports. Independent
construction/realization families in the package gate preserve the second
roundtrip law; the corpus alone is not an exhaustive proof over constructed
values. C before the final refresh repeats this face-by-face comparison against A after: all 32,828
identities and their checked/exact counts and inventories match, every face has
a complete 64-character Readings digest, and issues remain zero. Evidence:
`E/c-identity-proof.json` and `E/c-full.json`.

No admission licence, forbidden word-naming guard or parser literal changed.
The authored lexical source and GrammarEnvironment load successfully in every
corpus/test run. The v3 command does not emit the legacy coverage lock or
licensing-checker totals: those fields are not applicable, rather than invented
zero measurements. There is no new licensing checker in this change.

### DISCLOSE: digest and incremental soundness

Digest version **1**, JSON schema **7**. Each Reading key hashes the complete
value fingerprint (64 lowercase ASCII hex bytes of SHA-256 over generated
Reading Debug), ASCII `|n=`, its node trace encoding, ASCII `|w=`, and its Word
trace encoding. Each trace is ASCII `canonical` only after its complete ordered
values equal the generated visitor sequence; otherwise it is ASCII `raw` plus
the complete trace's Rust Debug encoding. A canonical trace can be reconstructed
from the full root value. Sort the unique 32-byte keys lexicographically as
unsigned bytes, concatenate raw fixed-width keys without separators, and
SHA-256 the result. Output lowercase hex. The empty set hashes zero bytes;
limited or invalid enumeration emits null. Debug identity is diagnostic under
the recorded tree, not a cross-version grammatical serialization contract.

Tests check exact encoding, ordering, empty sets, equal-sized changed sets,
repeated same-tree evaluation, and the real combat-interval-noun landing's
three lexical feature assignments (`wvmukxorkwyxxpsnkzvollmqxokyyqlv`). The seven
cases use authentic Oracle sentences, rather than claiming coverage of each
whole card. Only the Basandra, Dire Fleet Warmonger, Echo Circlet and Glyph of
Destruction sentence cases change digests. Fog, Deep Sight and Reach Through
Mists keep their digests. Selection
remains **7,464 unique / 13,610 multiple**, with **11,754 no Reading** and zero
undetermined; there is no destructive or specificity-based selection census.
There are no newly covered identities or selected analyses to disclose.

Incremental compatibility fixes input bytes, source field, support filter,
enumeration/sample policy, schema/digest version, and all recorded non-lexical
runtime/compiler/dependency sources. A declaration change, any unsupported
source change, an incomplete/invalid/duplicate base, or a missing selected face
forces full evaluation of the selection. Compatible lexical changes compare
complete Lexeme fingerprints and realized forms, select conservative case-folded
substrings of parser input using **both old and new forms**, fully recheck those
faces, and copy all other face payloads. Output records the base path, changed
sources, affected Lexemes/forms and rechecked/reused identities. Copied timing
and counters retain base provenance; `measurement_scope` explicitly labels the
mixed historical/current aggregates.

The seven-sentence combat replay matches full face payloads (excluding four timing
fields) **in both directions**, with five rechecks and two copies. Removed
irregular `dice` triggers a recheck even though remaining `die` is absent.
Removing a multiword form joined by reminder removal also triggers a recheck;
matching raw text would have missed it. JSON roundtrip and incompatible-base
fallback tests cover the serialized reuse path. The real CLI reuses all three selected Basandra/Echo Circlet/Fog face payloads
byte-for-byte from the full schema-7 report (zero rechecks). The old A schema
falls back and matches those full face payloads excluding timing fields.
Evidence: `E/c-cli-proof.json`, `E/c-cli-reuse.json`, `E/c-cli-fallback.json`.

**Deviations and additions:** the reference's node-name-only trace digest could
merge changed trace values; this implementation compares complete traces and
encodes malformed traces distinctly. Leaves are capped along with builds and
admission summaries at 100,000 entries. Source compatibility conservatively
covers runtime/compiler/normalization dependencies in addition to declarations.
Report maps own their strings for JSON deserialization. The parser-input
selection regression is fixed within C. README documents caps, digest/reuse
contracts and historical counter provenance. No grammar/schema/lexical additions
or deletions, checked-in corpus fixtures or new integration test binaries.
Remaining pending-work cloning and all-face report retention are unchanged by
the prescribed small memory variant; this landing does not claim constant total
memory. No semantic STOP, ruling contradiction or glossary gap occurred.

Assurance counts: **8 added unit tests**, one existing trace-fault test
strengthened, **0 restored, 0 re-spelled, 0 ignored, 0 removed**. No assertion
was weakened and no failing subject was retired. The gate runs the final normalization regression. Its xtask unit suite reports
380 passed and one pre-existing ignored test:
`macros::templates::tests::macro_schema_census_count_matches_21`, annotated
"cross-checks the live corpus against the census; run on demand". This ticket
adds no ignored tests.

### REPORT: stamped inventories and performance advisory

A before/after stamps are `woulmsvmqqkpwlqkprtpqympovupzqtv`, covered **21,074**;
"before" is that change's initial baseline tree, and "after" its implemented A
tree. `E/before-sources.json`, the copied baseline executable, the archived
reference diff, and the identity proof distinguish those snapshots. C before the final refresh
stamp is `qnsqxnqkpvlrnxnslzzrvzouzouvspzo`, covered **21,074**. These are
v3 census covered counts; v3 has no legacy lock `covered` field.

| Full enumeration (6 workers) | Peak RSS (kB) | Corpus wall (ns) | Process wall (ns) | Checked-text thread CPU (ns/B) | Host load (1/5/15 min) |
|---|---:|---:|---:|---:|---|
| A baseline | 10,291,988 | 150,344,832,286 | 154,188,731,176 | 295,374 | 6.56591796875 / 12.39306640625 / 10.97705078125 |
| A bounded/digest | 9,935,008 | 162,209,504,921 | 166,483,046,942 | 341,639 | 18.625 / 17.1298828125 / 16.0068359375 |
| C before final refresh | 9,383,388 | 156,266,330,688 | 161,077,166,926 | 339,266 | 21.37158203125 / 17.38818359375 / 14.9609375 |

A reduces peak RSS by 356,980 kB (about 3.5%) on this baseline. Higher-load
wall time does not demonstrate a speedup. Each corpus exceeds the
**16,260,000,000 ns** quiet-host advisory ceiling; these loaded-host measurements
are reported without treating the advisory as a gate. The before/after figures
in the ticket's archived four-worker experiment are historical evidence,
not measurements of this landing.

Lexical inventory SHA-256 is unchanged:
`3f6533411916afd863234508e82c7d3b249a5b7ef48a960c692d4d81877ab642`.
Named inventories with every spelling and its identities are in
`E/inventory-unchanged.json`: `homographs` (**1,129**),
`case_folded_homographs` (**1,088**), and `form_literal_vocabulary_overlaps` (**0**), over
**38,374** spellings. The named `form_literals` list has 23 entries.
No inventory member changed. Declaration counts are unchanged: **205 ordinary
Constructions + 45 shared schemas = 250**, **697 compiled productions**,
**3,328 nonblank declaration lines**. All inventory/count figures share the A
and pre-refresh C stamps and covered counts above. Zero Construction/test deletions
beyond the counts already disclosed.

### Verification and coordination

`cargo xtask gate --changed --from nkrmyrou --run --clippy` derives:

```sh
cargo test -p deckmaste_construction_v3_core -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_construction_v3_core -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

The derived test command passes: **834 tests, zero failures**, and the one
pre-existing ignored unit test disclosed above. Clippy first rejected the
callback field type, a local import placement, and two test idioms. The fix
adds a private alias of the identical function-pointer type, moves the import,
and makes tests compare sorted complete vectors and an explicit empty vector.
No lint was suppressed. The exact derived Clippy command then passes with
`-D warnings` (`E/clippy-final.log`). After those fixes, `cargo test -p deckmaste_english_v3` passes all 366 tests
and `cargo test -p xtask --lib english_v3::` passes all 27 census tests.
Evidence: `E/post-lint-package-tests.log`, `E/post-lint-census-tests.log`.

These post-census lint fixes change no parser, materialization, validation,
digest encoding or incremental behavior. The final corpus evidence is preserved
without another full enumeration; its exact source snapshot is embedded in
`E/c-full.json`. The current snapshot correctly treats such source edits as
unsupported for reuse of that older JSON.
`E/post-lint-source-delta.json` records the three changed source hashes.
A three-card CLI check after these edits confirms the unsupported-source
fallback and unchanged full face payloads apart from timing
(`E/c-cli-post-lint.json`, `E/c-cli-proof.json`).
`cargo fmt --check -p deckmaste_english_v3 -p xtask` passes.
`cargo xtask cite check --list-noncompliant` reports zero noncompliant strings;
`cargo xtask cite check` checks 16,155 citations with **0 stale**. No citations
were added or changed, so no bless/audit of new citation sites is needed.

The first refresh imported coordinator metadata only, with declarations and
all three lexical inputs byte-identical; runtime evidence is unaffected.
The final refresh log and source-hash checks are retained with the ignored evidence.
Before workspace retirement the evidence is copied to the coordinator
`target/english-v3/census-efficiency/`, preserving the same relative paths.

Final refresh encountered protected concurrent-edit exits (69) while sibling
workspaces advanced. Kata preserved those snapshots and this verified tree;
review and retry are the resolution. The refresh logs name each competing
workspace. These are coordination retries, with no source conflict or omitted
verification.

## Refreshed-tree amendment

Final refresh incorporated `plugins-v2-connive-bare-and-n` (`lwyuqtss`),
including a new `lexeme:keyword_action/conniveN` and a changed
`lexeme:keyword_action/connive`. It also imported facts-generation changes
outside the census path. The first post-refresh source check detected this
import, but its failure was not propagated before integration. That execution
mistake is corrected here with a new full identity proof and gate on the
refreshed tree, followed by integration of this amended record. Prior A/C
figures above certify their recorded source snapshots, not the changed lexicon.

D is stamped **`sknznupqkroutwuzmtypzzyxonswulku`, covered 21,074**.
All **32,828** identities retain the same coverage and checked/exact Reading
counts as C: **685,707 Readings**, **7,464 unique / 13,610 multiple / 11,754 no
Reading**, zero undetermined. Every face is complete with a digest and zero
issues; both traversal laws, admission/ownership and byte-exact realization
remain enforced. There are **zero lost or newly covered identities**.

The digest detects three changed Reading sets despite equal counts:

| Face | Before / after Readings | Current lexical analysis |
|---|---:|---|
| Mask of the Schemer | 15 / 15 | Connive with an Amount complement from `lexeme:keyword_action/conniveN` |
| Spymaster's Vault | 8 / 8 | Connive with an Amount complement from `lexeme:keyword_action/conniveN` |
| Raffine, Scheming Seer | 1 / 1 | Connive with an Amount complement from `lexeme:keyword_action/conniveN` |

These are attributed to the imported declaration split, not A/B/C. The full
named identity/digest/source list is `E/d-import-identity-proof.json`; the
Lexeme and source deltas are `E/imported-lexeme-delta.json` and
`E/refresh-source-proof.json`. Complete enumeration of these three faces with one retained sample each
confirms the `conniveN` lexical identity in every sample, and their digests
match D (`E/d-connive-analyses.json`, `E/d-analysis-proof.json`).

| Refreshed authority (6 workers) | Peak RSS (kB) | Corpus wall (ns) | Process wall (ns) | Checked-text thread CPU (ns/B) | Host load (1/5/15 min) |
|---|---:|---:|---:|---:|---|
| D | 10,403,684 | 255,314,203,625 | 263,517,215,902 | 439,930 | 15.27001953125 / 14.8515625 / 16.4443359375 |

D also exceeds the **16,260,000,000 ns** quiet-host advisory ceiling. It ran
with concurrent compiler work; it is not a speed comparison against A or C.
New lexical SHA-256:
`e5631fb2b39207a47675b51a4ca24125c9012e7509e8eccce76341df421812f1`.
On that D stamp, `E/inventory-refreshed.json` has **38,374 spellings**,
**1,136 exact homographs**, **1,091 case-folded homographs**, **zero
form-literal/vocabulary overlaps**, **23 form literals** and **697 compiled
productions**. Declarations remain **205 + 45 = 250**, **3,328 nonblank lines**.
Exact homograph additions: `Connived`, `Connives`, `Conniving`, `connive`,
`connived`, `connives`, `conniving`. Case-folded additions: `connived`,
`connives`, `conniving`. No spelling, form literal or overlap member is lost.
The full named inventories retain each spelling's lexical identities.

The same derived reverse-dependency test and Clippy commands are rerun on
the refreshed tree by `cargo xtask gate --changed --from nkrmyrou --run --clippy`.
It exits **0**: **834 tests passed, zero failed**, the same one pre-existing
ignored test, and Clippy passes with `-D warnings`
(`E/gate-refreshed.log`). Citation checks report zero noncompliant strings and
**16,156 citations / zero stale** (`E/cite-refreshed*.log`).
No test or assertion is weakened to accommodate the import. A/B/C still add
8 unit tests and remove/ignore none. This amendment adds no Construction,
Lexeme, test, fixture, licensing guard or glossary term.

The last refresh imports `uolxmkns` (derived-gate removal) and `mnvpxwqs`
(per-crate test contract). The only runtime-source deltas from D are deletion
of `gate.rs` and removal of its module/CLI command and dedicated tests. The
English CLI branch, census/grammar/compiler/lexical sources, all data and every
Lexeme state match D. The direct test and Clippy commands above already ran
for all four affected packages; `--all-targets` includes their tests. Preserve
those results under the updated per-crate contract rather than rebuilding to
retest an unchanged census. The 834 count is stamped before the gate's test
subjects were retired by that separate landing; A/B/C remove no test. Evidence:
`E/amendment-refresh-source-proof.json` and `E/amendment-current.json`.

A subsequent integration retry refreshed in `builtin-v2-keyword-spelling-templates`
(`sxppvxpn`). Its reader and corpus changes are representation-only for this
census: rebuilding xtask succeeds, the **complete Lexeme snapshot** and lexical
inventory hash equal D, and fresh full enumeration of the three connive faces
matches every D payload field except timing. Preserve D's full corpus proof;
do not spend another full enumeration on an unchanged lexical inventory. The
owner reports 463 construction-core, 56 lexical-source, 96 semantics-v2 and 366
English-v3 tests passed, plus construction-core/lexical-source Clippy and the
plugin Lean checks. No census/compiler-v3/declaration/validation/digest code
changed. Evidence: `E/current-reader-build.log`, `E/template-current.json`,
`E/template-source-proof.json`; the owner's completed ticket records its tests.
