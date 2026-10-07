---
needs: []
---
# Read comparative governors with their complements: equal to, less than, other than

## Why

Split from `english-v3-scalar-comparisons` (2026-10-06), which bundled four
constructions. This ticket owns one: an adjective comparative governor with
its expanded comparative Complement (*equal to X*, *less/greater than X*,
*other than X*), in postpositive or predicative function. On change
`wlvwtnppyovn` (32,828 supported faces, 13,716 covered, 19,112 unread), as
faces touched / faces where the bucket is the only recognised cause
(overlapping buckets, not gain forecasts):

| Item | Touched / sole | Probe (admitted roots) |
|---|---|---|
| *equal to* | 1,606 / 285 | "This creature deals damage equal to its power to any target." 0; "… deals 2 damage to any target." 1; "Its power is equal to that creature's power." 0 |
| *less / greater / more / other than* outside a Determiner | (in the *than* bucket, 541 / 99) | "Whenever you cast a spell other than your first spell each turn, draw a card." 0 |

Lexical declarations already exist: `vocab:ScalarDegree/Equal` takes a
To-marked `MeasurePhrase`, and `vocab:ScalarDegree/Greater`, `…/Lesser` and
`vocab:Adjective/Less` take Than-marked `MeasurePhrase`s (the `frame_additions`
block of `crates/deckmaste_lexical_source/lexicon/core.ron`). Deal, Draw, Gain
and Lose declare `Role("ScalarEquality")` frames in `verbs.ron`. The gap is
composition and slot reconciliation, not vocabulary.

## What is already owned or done

- `english-v3-by-complement-functions` (done) handles *by N* scalar-change
  extent Complements. It leaves Spikeshell Harrier uncovered because it also
  needs "comparative and *below*". This ticket owns the comparative sentence.
  Its other failing unit, "This effect can't reduce their speed below 1.", is
  a *below* PP, not a comparative governor; it stays with
  `english-v3-systemic-residuals`.
- From `english-v3-systemic-residuals`: Deal's two `ScalarEquality` rows in
  the unsupported-inventory table of the done
  `english-v3-generic-frame-consumption`, and Discerning Taste's comparison
  body ("You gain life equal to the greatest power among …"); its superlative
  composition stays audited there.

## Goal

Each item reads through the existing declarations, with the comparative
Complement attached to its governor and not as a free PP. *Equal to* after a
verb's Object (*deals damage equal to its power*, *gain life equal to …*)
either postmodifies the Object or fills the verb's `ScalarEquality` slot:
decide which by the frame and record it.

## Analysis

*Equal to*, *more/less than*, ·*er than* (*greater than*) and *other than* are
comparative governors with the prepositions their expanded Complements take
(CGEL, Ch. 13, §1.3, p. 1104, [15]): *equal* is non-scalar equality [15ii],
*more/less*/·*er than* scalar inequality [15iii], *other than* non-scalar
inequality [15iv]. The complement of *to*/*than* is the comparative
Complement, licensed by the governor (p. 1104, [13]–[15]). An AdjP with its own
post-head dependent occurs postpositively in NP structure, as in *members
[dissatisfied with the board's decision]* (Ch. 5, §14.2(b), p. 445), which is
the shape of *damage equal to its power* and *a spell other than your first
spell*. In *its power is less than this creature's power* the same AdjP is the
predicative Complement of *be*; the be + NP scalar-location host (*its power
is 0*) is `english-v3-copular-scalar-location`, not this ticket.

DP-internal *more than one creature* / *fewer than six cards* (comparative
determinative heading a Determinative Phrase) is
`english-v3-comparative-quantity-determiners`.

## Witnesses

- Soul's Fire: "Target creature you control deals damage equal to its power to
  any target."
- Mirkwood Elk: "You gain life equal to that card's power."
- Freedom Fighter Recruit: "Freedom Fighter Recruit's power is equal to the
  number of creatures you control."
- Sage-Eye Avengers: "Whenever this creature attacks, you may return target
  creature to its owner's hand if its power is less than this creature's
  power."
- Arcbound Tracker: "Whenever you cast a spell other than your first spell
  each turn, put a +1/+1 counter on this creature."
- Worldslayer: "Whenever equipped creature deals combat damage to a player,
  destroy all permanents other than this Equipment."

Each witness's only failing unit is the one quoted (recon on `wlvwtnppyovn`).
Also usable: Cosmos Elixir (*greater than*), Spikeshell Harrier's comparative
sentence, and Ghastly Demise / Dispersal Shield (*less than or equal to*,
coordinated governors sharing one Complement).

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/comparative-complements-before.json` on the
   claim parent, stamped with its change id.
2. Land one commit per governor class (equality, scalar inequality, non-scalar
   inequality). Write each witness as a test first.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading, covering every governor
   class. A comparative Complement read as a free Adjunct, or *to its power*
   read as Deal's recipient, is a defect, not a gain.
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

- *the same … as* (non-scalar equality with *same*): stays in
  `english-v3-systemic-residuals` ("Comparative *same … as*").
- *more than one creature*, *three or more creatures*:
  `english-v3-comparative-quantity-determiners`.
- *its power is 0*, *is 1 or less*: `english-v3-copular-scalar-location`.
- *with power 2*, *with mana value X*: `english-v3-scalar-property-values`.
- *the number of* (1,296 touched, 9 sole) inside an *equal to* Complement: not
  a comparison. Note how many become readable, but do not build for it.
- Cost modification (*costs {1} less*), *the greatest/least*, *tied for*,
  *rather than* (also excluded by `english-v3-instead-replacement`).

## Landing record

Standard constraints apply. The three governor slices are `olroklmz`
(equality), `wzqryvzk` (scalar inequality and shared governors), and `vqwmuolk`
(non-scalar inequality). Final runtime evidence is stamped `kruuuuup`, covered
18,646. Evidence lives in ignored `target/english-v3/` and is retained in the
coordinator's `target/english-v3/english-v3-comparative-complements/` before
workspace retirement. No evidence, scripts or generated inventories are tracked.

### PROVE — retained Readings and structural laws

The original full baseline, stamped `olroklmz` before its equality edits, has
18,019 covered faces and 386,032 Readings. The refreshed full baseline is
stamped `mupwkuzv`, an empty child with the same runtime tree as the refreshed
claim `pwunnqxz`, also covered 18,019. Refresh incorporated `plwnlrlq` / `sqoozkvl`
(the instead cleanup and verification): zero trunk-attributed face losses or
per-face Reading decreases, and identical per-face counts.

Against that refreshed base, all 32,828 supported faces are completely
enumerated. Coverage grows from 18,019 to 18,646: 627 gains, zero lost faces.
Exact Readings grow from 386,032 to 475,194: 89,162 additions, zero per-face
Reading decreases. Every previously covered face retains exactly its previous
Reading count; all additions occur on newly covered faces. There is no removed
Reading or re-coverage obligation.
`refresh-delta.json` and `reading-delta.json` retain both comparisons by face.

Both full censuses report zero issues, internal failures, duplicate Readings,
admission failures, exact-realization failures, lexical-ownership failures,
and construction/leaf traversal failures. Independently built equality and
shared-governor values establish both roundtrip laws with complete value and
Word equality. Existing independent numeric comparisons are re-spelled into
the replacement structure. The pinned *equal to its power*, *less than this
creature's power*, *less than or equal to this creature's power*, and *other
than this Equipment* constituents each have exactly one Reading. The new
scalar and nominal projections do not give two labels to one constituency.

Word-named licensing guards added: zero. The marker table reads the declared
frame signature and `ComparisonMarker`; admission reads the declared
`PrepositionFunctionLicence`. Lexical-source and GrammarEnvironment loading
succeed, preserving their error checks. V3 emits neither the retired parser's
coverage lock nor its permitted-licensing-checker total. These are complete V3
census figures, not figures from the retired parser.

Assurance: added 6 tests; restored 0; re-spelled 10 existing tests and 3 fixture
helpers; removed 0; new ignores 0. The re-spelled tests retain their original
cards and complete asserted outcomes: the two independent complemented-
adjective tests in `grammar`; the Other fixtures for restricted genitives,
relative-clause adjuncts, selected-cluster coordination and Noctis's means;
the Deal fixtures for the Avacyn manner and recipient paths, retained-object
passives, and lexical selected-marker/recipient-first frames. The three helper
changes are `head_primitives::signatures`, `selected_complement_clusters::clusters`
and `selected_complement_clusters::any_target`. The first gate found the old
seven-frame Deal census in the lexical fixture; it now asserts the six-frame
inventory and the complete replacement postposed frame, retaining the
recipient-first frame assertion. Clippy also flagged the length of the
independent-value fixture; it was split into equality and shared-governor tests retaining every value and traversal
assertion. No failing assertion was weakened. The gate's
one inherited ignore is `macro_schema_census_count_matches_21`, whose attribute
states that the live-corpus cross-check runs on demand.

### DISCLOSE — analysis and scope

The bare Comparative Complement is the secondary term; the selected marker is
retained with its governor. This follows CGEL Ch. 13 §1.3, pp. 1103–1105.
Complemented AdjPs can be postpositive (Ch. 5 §14.2(b), p. 445); the nominal
postmodifier reads *damage equal to its power*, *life equal to that card's
power*, and *a spell other than your first spell*. The predicative host reads
*its power is less than this creature's power*. CGEL Ch. 15 §4.4, p. 1343 n. 66
supports retaining each marker when comparative governors share a delayed
Complement. `ComparativeGovernorHead` is explicitly a project term for the
partial governor-plus-marker interface, not a new claim about CGEL's category
inventory. The glossary now defines Comparative Governor, Comparative Governor
Head, Comparative Complement and Comparative Phrase; no unresolved term gap.

Deal's contiguous `Object + ScalarEquality + to + Object` row is retired:
its comparison belongs inside the first Object. Draw/Gain/Lose's contiguous
`Object + ScalarEquality` rows and the corresponding shared grammar schema
are retired for the same reason. Deal's second, postposed row is replaced with
`Object NP + to + Object NP + ComparativeAdjectivePhrase`, preserving the
attested recipient-before-comparison order of Massive Raid and Torrent of
Fire. This frame attachment is the project's analysis; the cited CGEL passage
does not adjudicate that specific discontinuous Oracle ordering. The old
`ScalarEquality` alias, ObjectEquality frame/class/table row, equality/ordering
constructions and shared instances are removed on the grammar side as well
as in the lexicon. No superseded comparative route remains unreachable.

Every newly covered identity is named, with its exact Reading count, selected
Reading identity/cost, governor/marker owners, bare Complement owners and
attachment path in `gained-analysis-ledger.json`; `gained-analysis-review.txt`
is the complete named review list. All 627 selected samples were inspected.
Every comparison has its selected marker and bare Complement: none is a free
comparative Adjunct, and none treats *to its power* as Deal's recipient.
The census retains grammatical attachment alternatives, including outer
Genitive Phrase and PP attachments, under the
[user ruling (2026-10-04)](../../decisions/english-lexical-analysis.md):
grammatical game-semantic nonsense remains admissible. A cheapest sample is
presentation preference, not a
semantic Oracle. For example, Mirkwood Elk's cheapest sample attaches the
comparison inside the genitive possessor; its intended full *equal to that
card's power* constituent is also present and asserted by the witness test.
No costs or semantic admission filters were introduced.

Representative selected analyses, all stamped `kruuuuup`, covered 18,646:

| Face | Selected comparative analysis | Complete face Readings |
|---|---|---:|
| Soul's Fire | postpositive AdjP modifying damage; *equal to* + NP *its power* | 3 |
| South Wind Avatar | postpositive AdjP modifying damage; *equal to* + NP *its toughness* | 1 |
| Freedom Fighter Recruit | predicative AdjP; *equal to* + NP *the number of creatures you control* | 2 |
| Massive Raid | postposed Deal frame; *equal to* + NP *the number of creatures you control* | 23 |
| Torrent of Fire | postposed Deal frame; *equal to* + NP with existing superlative mana-value nominal | 42 |
| Discerning Taste | postpositive AdjP modifying life; *equal to* + greatest-power NP | 72 |
| Sage-Eye Avengers | predicative AdjP; *less than* + NP *this creature's power* | 8 |
| Yorvo, Lord of Garenbrig | predicative AdjP; *greater than* + Yorvo's-power NP | 10 |
| Dispersal Shield | shared *less than or equal to* governors + greatest-mana-value NP | 10 |
| Ghastly Demise | shared *less than or equal to* governors + number-of-cards NP | 9 |
| Arcbound Tracker | postpositive AdjP modifying spell; *other than* + NP *your first spell* | 4 |
| Worldslayer | postpositive AdjP modifying permanents; *other than* + NP *this Equipment* | 3 |

The exact source-pattern class census below uses overlapping buckets, not
mutually exclusive gain forecasts. Scalar inequality includes the out-of-scope
DP *more than* pattern; no such DP is recategorized as an Adjective. Before is
`mupwkuzv`, covered 18,019; after is `kruuuuup`, covered 18,646.

| Class | Faces touched | Covered before / after | Readings before / after |
|---|---:|---:|---:|
| equality (*equal to*) | 1,606 | 0 / 581 | 0 / 87,815 |
| scalar inequality (*less/greater/lesser/more than*) | 376 | 44 / 58 | 728 / 2,355 |
| non-scalar inequality (*other than*) | 166 | 0 / 43 | 0 / 1,223 |

Among 613 supported faces containing both *equal to* and *the number of*,
covered faces rise from 0 to 223. Existing number-of nominal composition is
reused; it was not rebuilt.

Spikeshell Harrier's comparative sentence passes the witness test, but its
whole face remains at zero Readings because the final *below 1* PP is outside
scope; that residual remains with `english-v3-systemic-residuals`. Cosmos
Elixir also remains at zero: *your life total* has no nominal Reading. The new
`english-v3-postpositive-comparative-determinatives` ticket owns the measured
NP / postpositive comparative DP host and *life total* nominal composition,
with Cosmos as an additional nominal witness. Its eight named *life more than*
faces remain unread. CGEL Ch. 5 §14.2(a), p. 445 describes the post-head
Determinative use; this is not attributed to the adjective passage. No *more*
Adjective homograph was added.

Selection census: unique faces 7,185 → 7,218; multiple faces 10,834 → 11,428;
unread faces 14,809 → 14,182. All Readings are retained. There is no destructive
specificity-resolved selection count, as required by the lexical-analysis ADR.
No competing new constituency or new specificity preference was introduced.

Deviations and additions:

- Five general comparative constructions replace four governor-specific ones;
  the scalar/nominal bare-Complement projections retain their distinct sources.
- One Coordination instance for governor heads replaces the retired two-instance
  shared ObjectEquality schema. Each shared governor retains its own marker.
- Other gains a selected *than* frame plus an explicit empty frame preserving
  its bare attributive use; this re-spells four independent fixture values.
- Deal's postposed comparison slot is retained, with complete lexical and
  passive fixture identities updated after the obsolete row is removed.
- Six witness/independent-value tests are added; no tests or plugin bodies are
  removed or changed outside the declared lexical frames and grammar consumers.
- The postpositive comparative Determinative follow-up and its systemic-residual
  dependency are added from measured remaining faces. No xtask flags, tracked
  evidence or task-specific verifier code is added.

STOPs: none. No recorded ruling was contradicted. Ordinary test failures were
fixed within scope, preserving their assertions.

### REPORT — inventories and performance

The measured V3 covered count is 18,019 → 18,646; there is no V3 coverage lock.
Named constructions remain 241: 197 ordinary + 44 shared schemas on refreshed
base `mupwkuzv` (covered 18,019), and 198 ordinary + 43 shared schemas on
`kruuuuup` (covered 18,646). The five replacement constructions and retired
schema give no net construction growth.

The declared-form inventory has 38,375 spellings on both trees, with identical
spelling-to-owner maps. `inventories.json` contains the complete named list of
1,128 homographs and the named form-literal inventory; the form-literal /
vocabulary overlap inventory is empty. No homograph or overlap was added or
removed. These are source inventories, not admission gates.

Performance is advisory. Full-census wall time, thread CPU per checked byte,
12 workers and host load (1/5/15-minute values):

| Tree / covered | Corpus wall time (ns) | Thread CPU per byte | Host load |
|---|---:|---:|---|
| original `olroklmz` / 18,019 | 60,665,740,122 | 295,929 ns/B | 7.6821 / 10.7783 / 10.1660 |
| refreshed `mupwkuzv` / 18,019 | 77,315,259,393 | 266,357 ns/B | 10.2466 / 7.6309 / 8.0815 |
| final `kruuuuup` / 18,646 | 176,647,509,862 | 450,008 ns/B | 29.5986 / 14.0166 / 10.2861 |

All three exceed the inherited quiet-host ceiling of 16,260,000,000 ns. The
final census overlapped gate compilation on a busy host and enumerated 89,162
additional Readings; these measurements do not establish a quiet-host speed
comparison. The 627-face selected-analysis census took 9,933,017,151 ns,
748,197 ns/B, 12 workers, host load 17.0425 / 25.3804 / 17.1367, on `kruuuuup`
(covered 18,646). No runtime figure is an admission gate.

### Validation

The derived reverse-dependency command is
`cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`;
the requested lint command is
`cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings`.
Their final outcomes and citation checks follow.

`cargo xtask gate --changed --from english-v3-comparative-complements --run --clippy`
exits 0 on the final tree: 776 tests pass, zero fail, one inherited ignore;
Clippy with warnings denied passes. `gate-verified.log` retains the complete
output. The split independent-value suite separately passes all six tests.
Nightly rustfmt checks all ten changed Rust files successfully.

`cargo xtask cite check --list-noncompliant` reports zero noncompliant strings;
`cargo xtask cite check` checks 16,113 sites with zero stale. The complete
feature diff is piped to `cargo xtask cite audit --diff`: zero new/changed
Comprehensive Rules citation sites. No citation blessing is needed. CGEL
passages were read through the offline reference helper before their claims
were written; the postposed Deal frame and retained game-semantic nonsense
are explicitly attributed to project analysis and the user ruling.
