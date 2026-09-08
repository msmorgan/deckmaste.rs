---
needs: [english-v3-generated-roundtrip-slice, english-v3-lexical-inventory]
---
# Declare the complete interconnected grammar and turn it on

Translate the whole intended Oracle English grammar from the reviewed source
map and corrected Lean model into v3 construction declarations, then run that
grammar against the complete supported corpus. This is deliberately one
top-down activation. Do not schedule or declare victory family by family before
their interactions exist.

Every planned family must have real general Productions and constraints:
documents; sentences and clauses; predicates, lexical frames and agreement;
nominals, determiners and adjectives; subordination; prepositions; relatives
and extraction; coordination, sharing and scope; measures; anaphoric forms and
locally licensed ellipsis; Type Lines; and Target Verb/Targeting Marker
homography. Each family must interact with at least one other family. Missing
lexemes or uncommon variants may remain explicit residuals; permissive
catch-alls, unused declarations and opaque source leaves do not count as
grammar.

Port useful v2 declarations and regression witnesses by meaning, never by
compatibility shape. The grammar consumes the independent lexical layer and
fresh compiler exclusively. Preserve all grammatical Readings, distinguish
duplicate derivations, and enforce every grammatical constraint during
admission or as an explicit deferred constraint before a Reading is exposed.
One declaration continues to drive parsing, checked construction, rendering
and traversal.

The first complete run is an experiment, not a coverage ratchet. Report no,
one and multiple Readings; lexical gaps; invalid-reading findings; byte-exact
and constructed-value roundtrip failures; chart/forest metrics; compiler/build
cost; and corpus runtime. Compute the Reading census only after every deferred
constraint has run; a forest whose candidate families all fail deferral counts
as no Reading. Sample successes and failures across every family and group
failures by shared grammatical, lexical, compiler or parser cause. The full
inherited obligation register is acceptance evidence for the activation; no
named witness may disappear because its former family ticket was retired.

Acceptance is a compiled whole grammar, an actual v3 supported-corpus command,
a reproducible baseline, and a cause-grouped residual report. It does not
require full corpus success. Standard constraints apply.

Note (2026-09-07): `flavor-words-are-vocabulary` deleted the enumerated
flavor-word declarations, and with them english_v2's flavor-word codec and
its mode-marker and label-term constructions. The 196 corpus identities that
stopped being covered are listed in that ticket's landing record and are
this activation's to re-cover: the italic head is one construction over an
ability word [CR#207.2c] or a flavor word [CR#207.2d], and the flavor word's
label is the run itself, analysed by `deckmaste_lexical`.

User clarifications: ellipsis preserves local form/voice and leaves its discourse
antecedent unresolved. `english-v3-systemic-residuals` owns general failures
exposed by activation; activation does not require closing every inherited gap.

## Landing record

Refreshed tree `kkmxslkn`: 134 Constructions, 52 Categories; all planned families
have connected Productions. V2 production and its coverage lock are unchanged.

| Field | No Reading | One | Multiple | Incomplete/errors |
|---|---:|---:|---:|---:|
| Rules text | 31,027 | 1,167 | 447 | 0 |
| Type Line | 0 | 32,641 | 0 | 0 |

Every counted Reading passes admission, exact realization and both traversal
checks. Independent constructed-value tests pass. The representative linguistic
audit found no confirmed syntactic invalidity; nine frame-coordination cases
parse without discharging their required shared-frame structure. All 20 named
register cases and 196 flavor-head cases remain explicit no-Reading obligations.
The complete identity accounting, cause groups, family audit, named inventories
and measurements are archived under `data/reports/english-v3/kkmxslkn/manifest.json`
(ignored, shared data), with general gaps owned by `english-v3-systemic-residuals`.

Reproduce: `cargo build -p xtask --release --timings`, then
`target/release/cargo-xtask english-v3 --field text --output /tmp/text.json --workers 1`;
repeat with `--field type-line --output /tmp/types.json`. Reports pin both input
and lexical-inventory hashes. No full run occurred before the refreshed baseline.

`cargo xtask gate --changed --clippy --run`: 595 passed, 0 failed, 1 preexisting
ignore; strict clippy passed. `english/scripts/axioms`: 2,631 theorems, zero
disallowed axioms. Rust: 25 tests added, none removed, weakened or ignored.
Lean: 10 statements replaced, 3 added; context-helper retirement is recorded in
`english/DECLARATIONS.md`. Compiler additions supply derived features, numeral
summaries and smaller recursive methods fixing the stack overflow. Ellipsis
scope and the typed-error ticket implement the user's explicit corrections.
No new glossary gap or word-named guard was found: 117 feature requirements,
13 agreement equations, 873 homographic spellings, zero literal/vocabulary overlaps.
Release text/types runs took 8.66/2.53 s versus the 16.26 s advisory, one worker:
35,923/3,899 ns/B; host loads 6.55/5.55/3.54 and 4.42/5.13/3.48. The cached
release build took 82.87 s. Full telemetry and validation logs are archived.
