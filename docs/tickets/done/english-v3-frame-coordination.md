---
needs: [english-v3-keyword-labels, english-v3-generic-frame-consumption]
---
# Recover shared frame-complement coordination and bound invalid derivations

Add v3 composition for coordination of corresponding ordered frame segments
under a shared verbal head, including paired objects/measures and destinations.
Use Lexeme-owned Frame Slots, selected markers and explicit sharing/discharge
relationships; the historical `VerbPhraseAndFrameComplementPairCoordination`
name identifies an obligation, not a required new Rust identifier. Preserve
separate versus shared surface material and every grammatical scope alternative.

## Re-baseline first (2026-10-05)

The baseline below is historical. A recipient-cluster host has landed since it
was written: one selected-tail host checks the complete governing verb frame and
requires coordination (record: "Selected complement coordination and PP
licensing" in [English v3 residual batch records](../../english-v3-residual-batch-records.md)),
and `crates/deckmaste_english_v3/tests/selected_complement_clusters.rs` asserts
the full structure for Arc Trail, Char and Fireslinger. Probed on the default
line at change `lnrvzkvl`, Fireslinger's ability and Forge Devil's ability each
have exactly one Reading, not 128 and 184.

So the first step of this ticket is measurement, not implementation: re-probe
all nine witnesses in the table and the destination-sharing constituents of
Cavalier of Thorns, Animal Magnetism and Genesis Ultimatum, and report which
already have the intended structure. What remains after that is this ticket's
scope. Do not rebuild the landed host or add a second route to the same
structure. If nothing remains but reconciling the 85 inherited identities, say
so and do only that.

The `english-v3-generic-frame-consumption` need is real, not sequencing: the
pinned shape validates segments against selected Frame Slots, which that ticket
redefines.

The activation tree `kkmxslkn` had 85 inherited frame-coordination obligations:
76 no-Reading faces and the following nine multiple-Reading faces, none with
the required shared-frame analysis. Fresh focused enumeration corroborates
these counts; high multiplicity alone is not proof of invalid grammar.

| Witness | Baseline Readings |
|---|---:|
| Brothers of Fire; Fireslinger; Goblin Artillery; Orcish Artillery; Orcish Cannoneers | 128 each |
| Forge Devil | 184 |
| Psionic Entity; Reckless Embermage | 64 each |
| Spicy Oatmeal Pizza | 736 |

Fireslinger's sampled tree puts `any target and 1 damage` inside one PP
complement and attaches the final recipient PP separately. Pin the intended
two complete amount/recipient frame segments; a render-identical tree with
that cross-segment grouping does not discharge it. Numeral codec alternatives
also multiply these counts (handled by `english-v3-lexical-measures`); do not
credit their removal as restored frame structure.

Pinned shape: a generated grammatical relation validates each coordinated
segment against corresponding selected slots, preserves slot order and marker
ownership, and represents open sharing dependencies until discharge. Packing
must retain those dependencies and their correlations in incomplete and
complete nodes. Admission summaries/deferred checks reject invalid combinations
before paths count as Readings; materialize ASTs only on request. General NP
and PP coordination remains available where independently grammatical.

Acceptance requires complete structural Reading-set assertions for all nine
named recipient witnesses. Reconcile all 85 inherited identities against their
recorded structures, not old selected winners. Fetch exact supported text and
preserve focused positive and negative witnesses. Test mismatched frame arity,
reversed slots, markers paired with the wrong segment, undischarged sharing and
cross-segment NP/PP consumption. Enumerate an independently justified allowed
structural set and assert zero extra invalid members for the recipient cases,
preserving legitimate attachment/scope alternatives. The destination
zero-invalid-members assertions for Cavalier of Thorns, Animal Magnetism and
Genesis Ultimatum are owed to `english-v3-selected-preposition-nominal-licensing`,
under the orchestrator scope decision below (user review pending). Record an
explicit resulting bound on distinct structures and measured forest growth
for these finite fixtures; fail on renewed degenerate growth. Never meet a
bound by capping enumeration, truncating valid readings, ranking one winner,
or asserting that every face must have one Reading.

Apply [Chart admission and ambiguity](../../decisions/english-lexical-analysis.md#chart-admission-and-ambiguity)
and [Source and roundtripping](../../decisions/english-lexical-analysis.md#source-and-roundtripping):
Earley parsing, one declaration for generated parsing/checked construction/
rendering/total traversal, retained lexical identities and both roundtrip laws.
Validate sharing and correlation with independent Rust witnesses. STOP and report
rather than substitute word-named frame guards, semantic target filtering,
eager AST products or destructive selection. V2 remains untouched.

This batch does not absorb all extraction, tense, passive-temporal, scope,
flat-list and layout work. Route unmatched causes from the 85 identities and
the remaining named register to `english-v3-systemic-residuals`; they require
measured owners before cutover. Standard constraints apply.

## Re-baseline and STOP (2026-10-06)

The following measurements and STOP preceded the scope decision below.
They describe work in change `qyqkqmlw` before its final landing verification.

The complete baseline on claim tree `uutxrkxt` measures 32,828 supported
faces: 19,206 no Reading, 6,653 one, 6,969 multiple; 13,622 covered;
zero internal, admission, roundtrip or traversal issues. Eight workers,
host load 12.26 / 16.49 / 17.60, corpus wall 467,225,473,972 ns,
checked-text thread CPU 1,162,890 ns/B. This busy-host measurement exceeds
the inherited 16,260,000,000 ns advisory. Evidence: `/tmp/frame-coordination-before.json`.

All nine named recipient faces have exactly one Reading **before this work**,
each with two complete `ObjectPrepositionTail` segments under the landed
`SelectedComplementClustersPredicate`. No restored recipient coverage is
claimed. The new regression independently constructs the complete Document
for each of the nine, asserts the whole singleton set, both roundtrip laws,
node and lexical traversal identity, and bounds chart items/families at
12,000 / 13,000. Baseline maxima among these finite fixtures are 4,208 items
and 4,367 families. Arc Trail and Char's previous exact witnesses remain.

The refinement replaces the host's fixed frame/marker catalogue check with
three declaration-driven relations: `segment Predicate`, `share_segments`
and `discharge_segments`. Compilation matches the segment's ordered fields
against supported typed Frame Slot layouts and their declared marker classes.
Packed summaries retain candidate frame dependencies; discharge requires the
first segment's selected frame and corresponding slots in frames declared by
the same lexical owner. Marker spellings are never an admission condition.
An independent compiler-consumer witness for Animal Magnetism's destination
constituent checks both roundtrip laws, differing licensed markers, lexical
homographs with incompatible slot relations, arity, order, wrong marker pairing,
and undischarged dependencies. This adds no second recipient-cluster route.

The exact destination constituent of Animal Magnetism,
`put that card onto the battlefield and the rest into your graveyard`,
has six baseline Readings, none with the required segment structure. The
refinement adds the intended plain and past-participial structures, but the
six pre-existing NP/PP groupings remain. They cannot be credited as valid
scope alternatives or silently retained to satisfy acceptance.

**STOP: supporting noun-licensing repair reaches beyond the segment relation.**
The recorded "preposition classes and noun complement licensing" amendment
(2026-09-02) in `docs/decisions/english-v2-rewrite.md` requires:
"Only adjunct-capable PPs enter the free predicate-adjunct / NP-postmodifier
attachment rule." It also requires selected-only prepositions to occur under
a licensing verb frame or noun valence. The current `PostmodifiedNominal`
checks neither the preposition's attachment permission nor selected noun
valence; it consumes the destination markers inside NPs. The supporting
repair was stopped for a scope decision.

**Resolution: orchestrator decision (2026-10-06), user review pending.** Keep
noun licensing separate; do not change `PostmodifiedNominal` or preposition
attachment permissions here. The six invalid Animal Magnetism groupings
pre-date this ticket (six baseline Readings, none correct). This refinement adds
the intended segment structures and no new wrong Reading in that constituent.
Implementing the recorded amendment restricts NP postmodification corpus-wide
and must have its own reviewed landing. The new planned ticket
`english-v3-selected-preposition-nominal-licensing` owns the destination
zero-invalid-members assertions and cause discrimination; it is now a production
cutover correctness prerequisite. This is an orchestrator decision, not a user
ruling. The revised scope authorizes landing after full-corpus verification.

Cavalier of Thorns and Genesis Ultimatum's complete destination constituents
also have no Reading. Discriminating probes find `among them` and `the rest`
readable, but `from among them` and both source-bearing Object NPs unreadable,
with zero vocabulary gaps. The missing PP-complement composition remains an
explicit supporting cause, rather than a frame-coordination gain. The shorter
Genesis Object also requires nominal cause discrimination. Evidence:
`/tmp/frame-coordination-components-*.json`.

The historical 85-gain table is recoverable from the v2 ticket without its old
ignored activation artifacts. Its names select 88 current faces; excluding
Insult, Carnage and Disciples of the Inferno preserves the 85 relevant faces,
including Injury, Carnival and Invasion of Regatha. The staged complete census
has 42 no, 34 one and 9 multiple Readings. At this stage all 43 successful faces contained
cluster structure in the retained samples; those samples did not discharge
the complete structural audit. The exhaustive final audit is recorded below. All 42 whole-face
failures remain owned by `english-v3-systemic-residuals` for cause discrimination.
Durable identities, current exact source, counts and sample fingerprints are
in `/tmp/frame-coordination-inherited-reconciliation.json`; the selector is
`/tmp/frame-coordination-inherited.json`. These are ignored evidence, not
tracked gate authority. These staged artifacts are superseded by the final identity accounting below;
the separate licensing ticket owns destination expected sets.

Staged verification on `qyqkqmlw` (baseline covered 13,622): the twelve-face
focused corpus run retains 3 no / 9 one / 0 multiple, with zero issues.
The nine recipient fixtures now reach at most 4,202 items / 4,361 families;
the finite 12,000 / 13,000 test bounds therefore have explicit headroom.
Four workers, load 2.33 / 9.80 / 14.74, wall 85,447,798 ns,
315,959 ns/B; evidence `/tmp/frame-coordination-staged-witnesses.json`.
This is subset verification, not a final whole-corpus landing census.
The compiler's relation-validation test and the two independent segment tests
pass, including public rejection of undischarged constructed values.
Formatting passes; citation checks find zero noncompliant strings and zero
stale citations. Assurance so far: added 4 tests, restored 0, re-spelled 0,
newly ignored 0, removed 0. No production Construction was added or deleted.

The reverse-dependency test gate passes: 1,238 passed, 0 failed, one pre-existing
xtask census ignore, across 98 test/doc-test suites. Strict clippy passes after
extracting the compiler's segment-relation validation helper and the test's
reflexive-pronoun helper, and replacing a needless test Box allocation. The
22 compiler-core tests, two segment-consumer tests and four recipient tests
pass again after those mechanical repairs. No assertion was weakened.

```text
cargo test -p deckmaste_construction_core -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_construction_core -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

The baseline and staged reports have identical input and lexical-inventory
hashes (`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`
and `ab63eaed51a42b72ff4c5064b39163e5354f0699ea5d1b4d3d53cfbf1d4cb973`).
These staged figures are superseded for landing by the full-corpus record below.
The separate licensing ticket retains the destination correctness obligation.


## Landing record

### PROVE

Baseline source is claim code `uutxrkxt`, covered 13,622 (the baseline report
was emitted in measuring change `qyqkqmlw`). Final source is `qyqkqmlw`, covered
13,622. Input and lexical-inventory hashes match. There is no v3 coverage lock;
13,622 is the measured covered count, not a claim that the legacy lock gates v3.

The final **full** supported-corpus command is:

```text
cargo xtask english-v3 --all --workers 8 --samples-per-face 0 --output /tmp/frame-coordination-final.json
```

All 32,828 faces are completely enumerated, without a Reading limit. All
176,171 counted Readings pass declaration admission, lexical ownership/context,
byte-exact realization and construction/word traversal identity. Internal
failures, failed or limited enumerations, cyclic derivations, duplicates and
validation issues are all zero. The independent constructed-value roundtrip
law is exercised by the nine-recipient expected-set test and independent
segment-consumer test, alongside retained Arc Trail, Char and Avacyn witnesses;
corpus realization alone is not claimed as proof of that law.

Lost covered identities: **[]**. Newly covered identities: **[]**; there is no
new face analysis to credit. Every identity's Reading count and construction
tally is unchanged. Telemetry alone does not independently judge the grammar
of every alternative. Full comparison: `/tmp/frame-coordination-final-comparison.json`;
complete reports: `/tmp/frame-coordination-before.json` and
`/tmp/frame-coordination-final.json`.

All nine named recipient Documents have independently justified singleton
expected sets with zero extra members, both roundtrip laws and traversal checks.
Their maximum measured final forest is 4,202 items / 4,361 families; test bounds
are 12,000 / 13,000. The independent segment witness stays below 500 items and
families. Negatives reject wrong arity, slot order, marker/frame pairing, mixed
slot relations and open sharing at the root, including checked construction.
No bound is met by truncating enumeration.

The inherited 85 identities reconcile as 42 no / 34 one / 9 multiple. All
**134 Readings** across the 43 successes are retained in the exhaustive focused
report, with no failures or truncated samples. Each was compared with its exact
source and retains two complete ordered amount/To/recipient segments per shared
Deal host (two hosts in Soul of Shandalar). Recipient coordination, relative
attachment, With-PP attachment and unrelated-clause alternatives remain
represented; this discharges the inherited frame-segment obligation, not all
wider linguistic obligations. The 42 no-Reading faces are named and routed as
one batch in `english-v3-systemic-residuals`' “Batch records and what they leave
open” list. Whole-face failure alone establishes no sole cause. Durable IDs,
exact sources and every successful Reading's signature are in
`/tmp/frame-coordination-inherited-reconciliation.json`; all complete trees
are in `/tmp/frame-coordination-inherited-final.json`.

| Readable inherited face | Complete Readings |
|---|---:|
| Granger Guildmage | 2 |
| Conduct Electricity | 1 |
| Volcanic Offering | 42 |
| Goblin Artillery | 1 |
| Punish the Enemy | 1 |
| Fireslinger | 1 |
| Burning Sun's Avatar | 1 |
| Boulder Dash | 1 |
| Fear, Fire, Foes! | 12 |
| Bellowing Fiend | 2 |
| Psionic Entity | 1 |
| Chandra's Outrage | 1 |
| Injury | 1 |
| Invasion of Regatha | 1 |
| Shocker, Unshakable | 4 |
| Reckless Embermage | 1 |
| Lunge | 1 |
| Cuombajj Witches | 2 |
| Daredevil's Billy Club | 1 |
| Seismic Wave | 2 |
| Psionic Blast | 1 |
| Spicy Oatmeal Pizza | 1 |
| Radiating Lightning | 1 |
| Unleash Shell | 1 |
| Orcish Cannoneers | 1 |
| Cunning Strike | 1 |
| Hungry Flames | 1 |
| Shower of Sparks | 1 |
| Explosive Welcome | 1 |
| Soul of Shandalar | 32 |
| Orcish Cannonade | 1 |
| First Volley | 1 |
| Carnival | 1 |
| Forge Devil | 1 |
| The Fall of Kroog | 1 |
| Orcish Artillery | 1 |
| Reckless Rage | 1 |
| Char | 1 |
| Chandra's Fury | 2 |
| Brothers of Fire | 1 |
| Dagger Caster | 1 |
| Arc Trail | 1 |
| Rakdos Firewheeler | 1 |

Word-named licensing checkers added: **0**. Admission reads declared Frame Slot
layouts, marker classes and the lexical head's owned frames. No verb, noun,
preposition, construction or card identity is a guard. The unchanged lexical
inventory loads through `deckmaste_lexical_source::load_workspace`; its gate
retains load-error tests. V3 has no legacy `environment.rs` and emits no legacy
licensing-checker census. No invented total substitutes for missing telemetry.

### DISCLOSE

| Measured source / covered | No Reading | One Reading | Multiple Readings | Exact Readings |
|---|---:|---:|---:|---:|
| baseline claim source `uutxrkxt` / 13,622 | 19,206 | 6,653 | 6,969 | 176,171 |
| final `qyqkqmlw` / 13,622 | 19,206 | 6,653 | 6,969 | 176,171 |

Specificity-resolved count is 0 on both trees; optional preference never decides
admission. No newly counted whole-face Reading is claimed. Animal Magnetism's
exact fragment adds the two intended segment structures (plain and past
participle) beside six pre-existing invalid NP/PP groupings. Its whole face,
Cavalier of Thorns and Genesis Ultimatum still have no Reading.

**STOP and resolution:** noun-selected PP licensing reached outside the segment
relation. The orchestrator decision of 2026-10-06, **user review pending**, keeps
that repair separate; it is not a user ruling. The six wrong Animal Magnetism
groupings pre-date this ticket, and that constituent gains only the intended
segment structures. Implementing the recorded 2026-09-02 amendment restricts
NP postmodification corpus-wide and needs its own reviewed landing. Acceptance
was amended explicitly: destination zero-invalid-members assertions and the
source-bearing Object/`from among them` cause discrimination are owed to
`english-v3-selected-preposition-nominal-licensing`, now a production-cutover
correctness prerequisite. `PostmodifiedNominal` and preposition attachment
permissions are unchanged.

**Deviations and additions:** production Constructions/schemas added 0, deleted
0; existing segment and shared-head declarations were refined. Added tests:
the compiler's segment-relation validation test; independent segment discharge
and roundtrip; invalid-frame/open-dependency negatives; and the complete
nine-recipient Document expected-set test. The compiler-consumer grammar is
test-only, not a second production route. The separate planned licensing ticket,
cutover dependency and 42-face systemic handoff implement the scope decision.

Assurance: restored 0, re-spelled 0, newly ignored 0, added **4**, removed **0**.
The one pre-existing ignore is
`macros::templates::tests::macro_schema_census_count_matches_21`, with the named
on-demand live-corpus census cross-check reason. No assertion was weakened.
No new glossary concept was needed: Frame Slots, shared Complements,
Coordination and Anchors keep their Oracle English glossary meanings.

`cargo xtask gate --changed --run --clippy` exits 0: **1,238 passed, 0 failed,
1 pre-existing ignored**, across 98 unit/integration/doc-test suites. Strict
clippy passes. Derived reverse-dependency commands:

```text
cargo test -p deckmaste_construction_core -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_construction_core -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p deckmaste_semantics_v2 -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Full gate output: `/tmp/frame-coordination-final-gate.log`.

Formatting and citation checks pass: 0 noncompliant citation-looking strings,
16,047 citations checked, 0 stale. No CR citation was added or changed.
`kata kanban check` reports no duplicate slugs, cycles or dangling needs.

### REPORT

Final measurements below are stamped `qyqkqmlw`, covered 13,622; baseline
measurements describe claim source `uutxrkxt`, also covered 13,622. The stable
work change ID alone does not distinguish successive measured source snapshots.

| Quantity | Baseline | Final |
|---|---:|---:|
| Shared schemas | 44 | 44 |
| Ordinary Constructions | 175 | 175 |
| Named Reading constructors | 219 | 219 |
| Declared Categories | 134 | 134 |
| Static Productions | 537 | 537 |
| Compiled Productions | 595 | 595 |
| Declaration lines | 2,758 | 2,757 |
| Constructions observed in full corpus | 166 | 166 |
| Chart items | 82,649,303 | 82,606,278 |
| Forest families | 87,161,176 | 87,110,423 |
| Completion work | 14,823,824 | 14,735,263 |
| Reading builds | 6,641,795 | 6,641,795 |

Exact declared-form homographs across lexical owners: **120**, unchanged
(excludes Catalog/Symbol categories and positional Initial variants). The named
inventory is the complete 119-row list in
[the generic-frame landing](../done/english-v3-generic-frame-consumption.md#construction-economy-and-lexical-inventories),
plus `up`: `vocab:Adverb/Up`, `vocab:Preposition/Up`. The complete current named
list and runtime production inventory are `/tmp/frame-coordination-final-inventory.tsv`.
Form-literal/vocabulary overlap inventory: **[]**, count 0 on both trees.
Lexical declarations and form literals did not change.

Performance advisory: final full-corpus wall **327,377,380,374 ns**,
checked-text thread CPU **1,102,008 ns/B**, eight workers, host load
**0.35 / 1.81 / 7.52**. Baseline wall **467,225,473,972 ns**,
**1,162,890 ns/B**, eight workers, load **12.26 / 16.49 / 17.60**. Both exceed
the inherited 16,260,000,000 ns quiet-host advisory; unequal host loads do not establish a
speedup. `english-v3-census-tractability` retains the runtime owner. These
figures come from the full reports, not the twelve-face staged run.
