---
needs: []
---
# Read comparative quantity determiners: N or more, one or more, more than one

## Why

Split from `english-v3-scalar-comparisons` (2026-10-06), which bundled four
constructions. This ticket owns one: a comparative quantity expression in
Determiner function of a counted Noun Phrase. On change `wlvwtnppyovn`
(32,828 supported faces, 13,716 covered, 19,112 unread), as faces touched /
faces where the bucket is the only recognised cause (overlapping buckets, not
gain forecasts):

| Item | Touched / sole | Probe (admitted roots) |
|---|---|---|
| *N or more / or fewer* (surface bucket, excluding *one or more*) | 2,095 / 274 | "If you control two or more creatures, draw a card." 0; "… two creatures …" 1 |
| *one or more* | 592 / 123 | "Whenever one or more cards leave your graveyard, draw a card." 0; "Whenever a card leaves …" 1 |
| *more / fewer than N* before a head noun | (in the *than* bucket, 541 / 99) | "This creature can't be blocked by more than one creature." 0 |

The *N or more* surface bucket also counts *power 2 or less* and *is 0 or
less*, whose outer constructions belong to the sibling tickets below; the
touched figure is a surface count, not this ticket's population.

## What is already owned or done

- `english-v3-scalar-cardinals` (done) composes *up to N* as a quantitative
  Preposition Phrase in Determiner function. It names anaphoric and
  scalar-Amount bounds as remaining. This ticket does not reopen *up to*.
- `english-v3-number-transparent-concord` (done) handles *any number of*.

## Goal

*three or more creatures*, *one or more cards*, *more than one creature* and
*fewer than seven cards* read with the comparative quantity expression in
Determiner function, composed through the shared Quantitative Determiner
interface built by `english-v3-scalar-cardinals`. Do not build a second
counted-NP family. The numeral-*or*-comparative coordination built here
(*two or more*, *0 or less*) is the one `english-v3-scalar-property-values`
and `english-v3-copular-scalar-location` reuse; build it as a unit those
hosts can consume, not as a Determiner-only recipe.

## Analysis

*Three or more* coordinates a cardinal numeral with *more*. CGEL uses *one or
more volunteers* to show the numerical status of *one*, citing *three or more*
as the parallel (Ch. 5, §7.6, p. 386, [44iiia]). The head is plural when
determined by the coordination *one or more*, while *one* in a DP headed by
*more* (*more than one*) selects a singular head (Ch. 5, §3.4, p. 353, n. 13;
the split ticket cited p. 354, but the footnote is anchored on p. 353). The
Agreement of the whole NP follows that structure, not the first numeral.

*More than one* and *fewer than twenty* are Determinative Phrases headed by
the comparative determinative, whose *than* + quantifier PP is its Complement
(Ch. 5, §11(d), p. 432, [5]–[6]). *More than one glass* is syntactically
singular (p. 432, [6i]). These are DP-internal comparative Complements in
Determiner function; the predicative and postpositive comparatives (*is less
than*, *other than*) are `english-v3-comparative-complements`.

## Witnesses

- Warcry Phoenix: "Whenever you attack with three or more creatures, you may
  pay {2}{R}."
- Garrison Excavator: "Whenever one or more cards leave your graveyard, create
  a 2/2 red and white Spirit creature token." (plural Agreement with *leave*)
- Ironhoof Ox: "This creature can't be blocked by more than one creature."
  (singular head)
- Eidolon of Rhetoric: "Each player can't cast more than one spell each turn."
- Shadowborn Demon: "At the beginning of your upkeep, if there are fewer than
  six creature cards in your graveyard, sacrifice a creature."

Each witness's only failing unit is the one quoted (recon on `wlvwtnppyovn`).
Also usable: Norwood Riders and Outland Colossus (as Ironhoof Ox), Cunning
Bandit ("if there are two or more ki counters on this creature").

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/quantity-determiners-before.json` on the claim
   parent, stamped with its change id.
2. Write each witness as a test first, with its bare-cardinal twin where one
   reads.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *or more* read as clausal
   *or*-coordination, *than one* read as a free Adjunct, or a plural head
   admitted under *more than one*, is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.
9. A CGEL citation may back only what the cited passage itself says; a project
   or orchestrator ruling is cited as a ruling, never attributed to CGEL.
10. Retire a superseded route on both the lexicon and the grammar side; do not
    leave unreachable declarations.

## Out of scope

- Comparative governors with *to*/*than* Complements outside the Determiner
  (*equal to*, *is less than*, *other than*):
  `english-v3-comparative-complements`.
- *with power 2 or less*, *with mana value X*: `english-v3-scalar-property-values`.
- *its power is 0 or less*: `english-v3-copular-scalar-location`.
- Cost modification (*costs {1} less*), *the greatest/least*, *tied for*.
- Clause-level hosts that fail independently (*as long as*, *only if*); count
  a face only when this construction is its last cause.

## Landing record

In addition to the standard record: per item, before/after counts stamped with
change ids; the Agreement observed for *one or more* and *more than one*
heads; the interface the numeral-*or*-comparative coordination exposes for
the two sibling hosts; timings as integer ns and ns/B with host load and
worker count.


### Landing record — 2026-10-07

Standard constraints apply. Before figures are stamped `zssmqvsw` / covered
17,556: that report was taken before implementation, with runtime declarations
identical to claim `otlzrzzy`. Final figures are stamped `ztnzpnsq` / covered
18,019: that report's empty child has implementation `zssmqvsw`'s runtime tree.
The covered count distinguishes the two measured states of the implementation
change id. Runtime source digests accompany both censuses.

#### PROVE — retained Readings and structural laws

All 32,828 supported faces are enumerated completely on both measured states.
Covered faces increase from 17,556 to 18,019: 463 gains, zero lost faces.
Every previously covered face retains exactly its previous Reading count:
zero decreases and zero increases. Total Readings increase from 343,616 to
386,032, with all 42,416 additions on newly covered faces. No old production,
lexical form, frame or Reading is retired; there is no no-silent-loss debt.
Refresh has brought in no runtime change, so there is no trunk-attributed loss.

Both full censuses report zero issues, internal failures, duplicate Readings,
admission failures, exact-realization failures, lexical-ownership failures and
construction/leaf traversal failures. Independent values establish both
roundtrip laws and exact construction/leaf identity for the pinned quantities,
the internal PP Complement and the Arabic scalar coordination. Each new
quantity constituent has exactly one Reading; it cannot also become a Cardinal
or an ordinary PP Adjunct. No competing NP family was introduced.

Word-named licensing guards added: zero. Every new guard reads declared
features. Lexical source loading and GrammarEnvironment construction succeed;
existing load-error checks remain intact. V3 emits neither the retired parser's
coverage lock nor its permitted-licensing-checker total; these figures are the
complete V3 census, not a claim about that parser.

Assurance counts: added 6; restored 0; re-spelled 0; removed 0; new ignores 0.
The existing `counts_and_measures_interact_with_nominals_frames_and_prepositions`
test exposed the draft's unintended admission of “Draw 2 cards.” It was kept
unchanged; the code fix confines Arabic numerals to Quantity Conjunct use.

#### DISCLOSE — analyses and scope

All 463 newly covered identities, their names, complete Reading counts,
selected Reading identities/costs and quantity analyses are listed in the
ignored `newly-covered-readings.md` and `selected-analysis-audit.json` evidence.
Every selected analysis contains the existing `CountedNounPhrase` and its
`ComparativeQuantityDeterminer`. The inventory comprises 422 faces with
numeral Coordination and 41 with a headed comparative DP; no new gain lacks
one of those quantity analyses. The 463 gains reconcile as 422 coordination
faces plus 41 comparative-DP faces. Of the coordination faces, 143 contain
“one or more” and 279 do not (the named analysis ledger supplies that split).
The ticket’s earlier sole-cause figures, 274 for the other numeral bucket,
123 for “one or more” and 99 for the entire *than* bucket, were overlapping
surface classifiers on `wlvwtnppyovn` / 13,716 covered; the last bucket also
contains comparative hosts outside this ticket. They are not disjoint gain
predictions for the measured landing base of 17,556 covered. The identity and
selected-constituency audit, rather than sums of those older buckets, establishes
the actual 422 + 41 partition. Representative witnesses follow:

| Face | Quantity | Selected constituency | Complete face Readings |
|---|---|---|---:|
| Warcry Phoenix | three or more | numeral Coordination; Quantity Conjunct; Quantitative Determiner of the counted NP | 1 |
| Garrison Excavator | one or more | numeral Coordination; Quantity Conjunct; Quantitative Determiner of the counted NP | 1 |
| Ironhoof Ox | more than one | comparative DP; internal quantitative PP Complement; Quantitative Determiner of the counted NP | 5 |
| Eidolon of Rhetoric | more than one | comparative DP; internal quantitative PP Complement; Quantitative Determiner of the counted NP | 2 |
| Norwood Riders | more than one | comparative DP; internal quantitative PP Complement; Quantitative Determiner of the counted NP | 5 |
| Outland Colossus | more than one | comparative DP; internal quantitative PP Complement; Quantitative Determiner of the counted NP | 5 |
| Rule of Law | more than one | comparative DP; internal quantitative PP Complement; Quantitative Determiner of the counted NP | 2 |
| Familiar Ground | more than one | comparative DP; internal quantitative PP Complement; Quantitative Determiner of the counted NP | 5 |
| Isleback Spawn | twenty or fewer | numeral Coordination; Quantity Conjunct; Quantitative Determiner of the counted NP | 7 |
| Concealed Courtyard | two or fewer | numeral Coordination; Quantity Conjunct; Quantitative Determiner of the counted NP | 2 |
| Battle of Wits | 200 or more | numeral Coordination; Quantity Conjunct; Quantitative Determiner of the counted NP | 2 |
| Sheltered Valley | three or fewer | numeral Coordination; Quantity Conjunct; Quantitative Determiner of the counted NP | 12 |

The selection census is 7,122 unique / 10,434 multiple before, and 7,185
unique / 10,834 multiple after; all previously covered multiplicities remain
unchanged. There is no specificity-resolution admission step. Sample ranking
retains every Reading and is a presentation choice only.

Surface-bucket counts below use reminder-stripped supported text and cardinal
word/digit bounds. Buckets overlap and include outer hosts owned by sibling
tickets; they are not sole-cause or gain forecasts. Each column carries the
before/after stamps given above.

| Surface item | Touched faces | Covered before / after | Readings before / after |
|---|---:|---:|---:|
| N or comparative, excluding the one-or-more occurrence | 1,724 | 1 / 281 | 1 / 30,092 |
| one or more | 592 | 0 / 148 | 0 / 12,030 |
| comparative than numeral | 136 | 1 / 42 | 1 / 506 |

“One or more cards” selects a plural Nominal and plural finite “leave”.
“More than one creature” selects a singular Nominal; its PP Complement contains
cardinal one. Parsed and independently constructed wrong head numbers fail.
These selection facts are supported by CGEL Ch. 5 §3.4, p. 353 n. 13 and
§11(d), p. 432; the numeral Coordination is supported by §7.6, p. 386.
Project licensing and the Quantity Conjunct interface are project choices,
not claims attributed to CGEL.

The reusable interface is `ComparativeQuantity(number)`, with
`NumeralComparativeCoordination { left: QuantityConjunct, marker, right }`.
It admits the attested “three or more”, “one or more”, “twenty or fewer” and
“0 or less” units. The scalar-property and copular-scalar tickets can consume
that same unit. Reuse stops at “4 or greater”: *greater* is adjectival
(CGEL, Ch. 5 §11(d), p. 432 n. 48) and has no `ComparativeQuantityUse`,
so the current `NumeralComparativeCoordination` cannot admit that value.
Its native numeric Conjunct does not acquire unrelated Cardinal
functions. `ComparativePrepositionPhrase(number)` uses the existing quantitative
PP schema; a headed comparative DP feeds the existing counted-NP interface.

Deviations and additions:

- Added five ordinary Constructions: `NumericQuantityConjunct`, the zero-cost
  `CardinalQuantityConjunct` projection, `NumeralComparativeCoordination`,
  `ComparativeDeterminativePhrase`, and the zero-cost
  `ComparativeQuantityDeterminer`. Added one instance of the existing
  quantitative PP schema. The scoped numeric Conjunct is necessary for the
  sibling host's attested Arabic “0 or less”; the shared Conjunct interface
  preserves the compiler's field-category contract and prevents a second
  coordination recipe. No existing Construction or test was removed.
- The draft general `SmallCount` route is completely retired from the final
  grammar. It added 15 unrelated damage-partitive gains (165 Readings) and
  failed the existing numeric-cardinal rejection test. Those draft figures
  are not final gains. No lexical form or frame was introduced for that route,
  so there is no corresponding lexical declaration to orphan or retain.
- The ticket's old recon overstated Shadowborn Demon's sole cause. Its full
  quantified NP now reads, but its full face remains at zero because the
  existential “there are” host is independently unimplemented. Its authentic
  NP remains tested; the host residual is routed to live
  `english-v3-systemic-residuals`, not counted as a gain.
- Added Comparative Quantity, Comparative Quantity Use and Quantity Conjunct
  glossary entries and expanded Quantitative Determiner to include these
  realizations. No Comprehensive Rules claim or citation was added.

STOPs: none; no recorded ruling was contradicted. The draft regression was
fixed inside the ticket before integration; the two-form compiler rejection
was resolved through the shared Conjunct interface.

#### REPORT — provenance and performance

Construction inventory: 192 ordinary + 44 shared schemas before (236 named),
197 ordinary + 44 shared schemas after (241 named). One category instance of
the existing PP schema is added. These figures carry the above before/after
change-id and covered-count stamps. Grammar economy's ultimate 250 ceiling
remains applicable.

Declared-form homograph inventory: unchanged, 1,128 spellings; 1,012 are
attested as exact spellings in raw supported Oracle text, including reminders.
Both named lists, with lexical owners, are in ignored `inventories.json` and
`attested-homographs.json`. Form-literal/vocabulary overlap inventory: empty,
unchanged. No lexical surface or owner was added or removed.

| Measured tree / covered | Workers | Host load (1/5/15 min) | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| `zssmqvsw` / 17,556 (claim runtime) | 12 | 5.91/7.18/5.26 | 55,825,333,148 | 277,189 ns/B |
| `ztnzpnsq` / 18,019 (final runtime) | 12 | 13.67/8.37/8.11 | 64,540,107,969 | 301,487 ns/B |

Both busy-host runs exceed the inherited 16.26s quiet-host ceiling. Wall time
and per-byte CPU rise; host load and simultaneous gate compilation limit a
causal comparison. This is a performance advisory, not an admission gate.
The final all-gains sampled run additionally validates all 463 preferred
analyses and all 42,416 Readings: 13,262,463,016 ns, 669,438 ns/B, 12 workers,
host load 11.56/9.48/8.60, stamp `ztnzpnsq` / whole-corpus covered 18,019.

#### Verification and evidence

Full final census: `target/debug/cargo-xtask english-v3 --all --workers 12
--samples-per-face 0 --output target/english-v3/quantity-determiners-final.json`.
The binary was rebuilt by `cargo xtask gate` before direct invocation; both
launchers execute the same census command. All-gains iteration uses explicit
`--face-id` selectors, one selected sample per face and complete enumeration.

Gate: `cargo xtask gate --changed --from english-v3-comparative-quantity-determiners
--run --clippy`, deriving:

```text
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Gate passes: 769 tests passed, zero failed, one inherited ignored corpus
cross-check (`macro_schema_census_count_matches_21`, whose existing blocker is
“cross-checks the live corpus against the census; run on demand”). Clippy
passes with warnings denied. The six focused quantity tests pass. Nightly
formatting of changed Rust files passes. Citation checks
report zero noncompliant strings and zero stale citations across 16,113 sites;
the piped diff citation audit selects zero new Comprehensive Rules sites.
No citation blessing is needed.

All census, named inventories, identity deltas, analysis ledgers, runtime
digests, probe results and logs were generated under the feature workspace's
ignored `target/english-v3/`. Before workspace retirement, the review evidence
is retained under the coordinator workspace's ignored
`target/english-v3/english-v3-comparative-quantity-determiners/`. No evidence
snapshot or verifier is tracked or embedded in source.


### Post-landing fix (2026-10-07)

Standard constraints apply. The accepted review found an invalid bare
*nor* coordination, an unrecorded residual route, and an overstated reuse
interface. This follow-up requires `NoncorrelativeCoordination = Yes` on
`NumeralComparativeCoordination`, retaining the existing Alternative-kind
requirement. The negative NP list now includes “three nor more creatures”.
*And/or more* remains admitted through the existing Alternative,
noncorrelative coordinator; its licensing is unchanged.

Shadowborn Demon's existential *there are* obligation is now recorded in
`english-v3-systemic-residuals`, which owns cause audit and bounded-ticket
routing; there is no existing implementation ticket for existential *there*.
Its quantified NP remains covered by the unchanged authentic regression test.
The reuse note here and the scalar-property ticket's Analysis now state that
adjectival *greater* has no `ComparativeQuantityUse`, so “4 or greater” cannot
reuse the current `NumeralComparativeCoordination`. CGEL Ch. 5 §11(d),
p. 432 n. 48 supports only the adjective classification; the declared licence
and resulting implementation limit are project facts. The 463 original gains
are reconciled above against the older overlapping sole-cause figures.

#### PROVE / DISCLOSE / REPORT

Full enumeration is unchanged on all 32,828 supported faces: 18,019 covered,
386,032 Readings, 7,185 unique and 10,834 multiple faces, zero gained or lost
faces, zero per-face count changes (including all previously covered faces).
`base.json` is stamped `tzttsvto` / covered 18,019 and has the
runtime declarations of initial base `pwunnqxz`; `after.json` is stamped
`wsvqxusx` / covered 18,019. `delta.json` supplies the identity-level
comparison; runtime digests distinguish the measured trees. Both censuses
report zero issues, duplicate Readings and internal failures, preserving both
roundtrip laws, admission, lexical ownership and construction/leaf traversal.
The initial base also matches every per-face Reading count on the accepted
landing (`accepted-to-initial-base.json`). Refresh before verification was a
no-op; no inherited trunk count change is present.

The invalid full “Destroy three nor more creatures.” probe falls from one
Reading to zero: a wrong analysis retired outside the supported corpus,
with no re-coverage obligation. “Three and/or more creatures” retains one
Reading; “4 or greater” retains zero at `ComparativeQuantity`. All six
before/after probes report zero issues; this negative result does not promise
that the scalar-property host is implemented.

| Measured tree / covered | Workers | Host load (1/5/15 min) | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| `tzttsvto` / 18,019 (base runtime) | 12 | 12.45/8.72/8.43 | 55,018,005,280 | 268,963 ns/B |
| `wsvqxusx` / 18,019 (follow-up runtime) | 12 | 17.20/10.49/9.08 | 186,509,667,173 | 434,876 ns/B |

Performance advisory: both runs exceed the 16.26s quiet-host ceiling; the
final census overlaps the gate build on a busier host. Wall and per-byte CPU
rise; these measurements do not isolate the guard's causal cost.

Construction counts remain 197 ordinary + 44 shared schemas (241 named),
with no instance added or removed. Homograph and form-literal/vocabulary
overlap inventories remain the named lists in the original landing evidence:
1,128 declared spellings, 1,012 attested spellings and zero overlaps. All these
unchanged inventory counts carry the before/after stamps and covered counts
above. V3 has no retired coverage-lock or permitted-checker-total output. No Construction, lexical declaration, surface or frame is
added or removed. The new guard reads a declared feature, with no word-named
licensing checker. No overlapping route is introduced or superseded.

#### Verification

The full-corpus commands are `cargo xtask english-v3 --all --workers 12
--samples-per-face 0 --output target/english-v3/base.json` and the same command
with `after.json`, executed through the rebuilt `target/debug/cargo-xtask`
for the final run while the gate tests compile. The complete per-face
comparison is `target/english-v3/delta.json`.

Derived gate: `cargo xtask gate --changed --from pwunnqxz --run --clippy`:

```text
cargo test -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Gate passes: 714 tests passed, zero failed, one inherited ignored
`macro_schema_census_count_matches_21` with its existing blocker “cross-checks
the live corpus against the census; run on demand”. All six quantity tests
pass; clippy passes with warnings denied. The newly strengthened rejection
first failed on “three nor more creatures” before the guard, then passed
without changing any other assertion.

Citation checks from the workspace root report zero stale sites among 16,113
citations and zero noncompliant strings; the piped diff audit selects zero new
CR sites. No citation blessing is needed. The CGEL footnote was read directly.

Formatting passes for both changed Rust files with nightly rustfmt. The
broader `cargo +nightly fmt --all -- --check` reports inherited formatting
drift in 170 other files, with no diff in either changed Rust file; its named
inventory and log are retained as `inherited-format-drift.json` and
`workspace-fmt.log`. This follow-up does not reformat that unrelated tree.

Assurance: one existing rejection test strengthened with one negative case;
added test functions 0, restored 0, re-spelled 0, removed 0, new ignores 0.
No glossary gap, new Comprehensive Rules citation, scope deviation or STOP.

All full-corpus reports, per-face comparisons, runtime digests, probes and logs
are under this workspace's ignored `target/english-v3/`; retained review
evidence after retirement is under the coordinator workspace's ignored
`target/english-v3/english-v3-cqd-followup/`.
