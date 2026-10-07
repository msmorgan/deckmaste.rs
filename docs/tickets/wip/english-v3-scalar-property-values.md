---
needs: [english-v3-comparative-quantity-determiners]
---
# Read scalar property values: with power N, with mana value X

## Why

Split from `english-v3-scalar-comparisons` (2026-10-06), which bundled four
constructions. This ticket owns one: a scalar property noun (*power*,
*toughness*, *mana value*) immediately followed by its value, inside a Noun
Phrase (*with power 2*, *with mana value X*, *with power 4 or greater*). This
is not a comparison; the value may be a bare numeral or a
numeral-*or*-comparative coordination. On change `wlvwtnppyovn` (32,828
supported faces, 13,716 covered, 19,112 unread), as faces touched / faces
where the bucket is the only recognised cause (overlapping buckets, not gain
forecasts):

| Item | Touched / sole | Probe (admitted roots) |
|---|---|---|
| *mana value* (surface bucket) | 1,124 / 39 | "… with mana value 2." 0; "… with mana value X." 0 |
| *with power N or less/greater* | (in the *N or more* bucket) | "Destroy target creature with power 2." 1; "… with power 2 or less." 0 |

The probes show the bare-numeral value already reads after *power* but not
after *mana value*, and no value reads as a coordination.

## Goal

*with power 2 or less*, *with toughness 4 or greater*, *with mana value 1* and
*with mana value X* read with the value as a dependent of the property noun,
the PP headed by *with* modifying the target Nominal. The *2 or less* value is
the numeral-*or*-comparative coordination built by
`english-v3-comparative-quantity-determiners` (the `needs:` edge): reuse it,
do not build a second one. Make *mana value* and *power* take their value
through one route; record whether the existing *power 2* route is that route
or is superseded (Method 10).

## Analysis

CGEL recognises NP post-head modifiers denoting age, size and similar
properties (*a man my age*, *shoes this size*; Ch. 5, §14.2(c), p. 446, [13i]),
but that passage does not discuss a bare numeral following a property noun,
and no CGEL passage located for this ticket does. The structure of *power 2*
is therefore a decision this landing makes and records as a project ruling,
not a CGEL citation. The numeral-*or*-comparative coordination is cited in
the quantity-determiner ticket (Ch. 5, §7.6, p. 386, [44iiia]); here it fills
the value position, not a Determiner.

Reuse stops at *4 or greater*: *greater* is adjectival (CGEL, Ch. 5,
§11(d), p. 432 n. 48) and has no `ComparativeQuantityUse`, so the current
`NumeralComparativeCoordination` cannot admit that value.

## Witnesses

- Disembowel: "Destroy target creature with mana value X."
- Mental Misstep: "Counter target spell with mana value 1."
- Easy Prey: "Destroy target creature with mana value 2 or less."
- Kor Line-Slinger: "{T}: Tap target creature with power 3 or less."
- Retribution of the Meek: "Destroy all creatures with power 4 or greater."

Each witness's only failing unit is the one quoted (recon on `wlvwtnppyovn`).
Also usable: Valorous Stance (*toughness 4 or greater*), Here Comes a New
Hero! (*mana value X or less*), Rigo, Streetwise Mentor (with *one or more*,
after the prerequisite lands).

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/scalar-property-values-before.json` on the claim
   parent, stamped with its change id.
2. Write each witness as a test first, with its bare-numeral twin.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *or less* read as clausal
   *or*-coordination, or the value read as a Determiner of a following noun,
   is a defect, not a gain.
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

- *its power is 2*, *its mana value is 2 or less* (be + value):
  `english-v3-copular-scalar-location`.
- *equal to*, *less than*, *other than*: `english-v3-comparative-complements`.
- *three or more creatures*, *more than one creature*:
  `english-v3-comparative-quantity-determiners`.
- *base power and toughness 0/2*, *the greatest mana value among*, cost
  modification (*costs {1} less*).

## Landing record

In addition to the standard record: before/after counts per property noun
stamped with change ids; the ruling recorded for the *power 2* structure and
which route survived; timings as integer ns and ns/B with host load and
worker count.


### Project ruling: scalar property values

Landing decision (2026-10-07). In Oracle English, a singular scalar property
noun followed by a value forms a Noun Phrase with that value as its selected
Complement. The noun retains its lexical Countability and is licensed for
this bare value-bearing use by its declared measure position. The value is
not a Determiner and does not determine a following noun. The enclosing
*with* Preposition Phrase can Postmodify the target Nominal through the
existing PP Modifier licence.

The existing `MeasuredAttribute` route for *power 2* and *toughness 2*
survives, refined to consume `ScalarPropertyValue`; *mana value* receives
that same lexical licence. There is no parallel property frame and no
superseded lexical or grammatical route left to retire. Bare scalar values
reuse `MeasurePhrase`; determinative numeral Coordination reuses the
prerequisite's `NumeralComparativeCoordination`. A headed comparative DP
such as *more than one* is not a scalar property value of this ticket.

*4 or greater* has an Adjective as its comparative Conjunct and is licensed
separately from determinative Coordination; it supplies no quantity
Determiner licence. Only that declared adjectival use is added, so *or less*
has one value Reading and does not acquire a second adjective analysis.
This extension and the property-Complement decision are project rulings.
CGEL, Ch. 5 §7.6, p. 386, [44iiia] backs numeral Coordination; Ch. 5 §11(d),
p. 432 n. 48 backs only the Adjective classification of *greater*.
Ch. 5 §14.2(c), p. 446, [13i] discusses property-denoting NP Postmodifiers,
not bare numerals after a property noun; it does not establish this ruling.


### Landing record — 2026-10-07

Standard constraints apply. Before figures are stamped `yqupppnwlkwk` /
covered 18,646, with runtime source identical to claim `kypxyllkrrqv`.
After figures are stamped `yqupppnwlkwk` / covered 19,170.
The covered count and runtime digests distinguish the two measured states of
that working-copy change id. The before run precedes implementation; both
censuses enumerate all 32,828 supported faces without a Reading limit.

#### PROVE — retained Readings and structural laws

Coverage increases from 18,646 to 19,170: 524 newly covered
faces, zero lost faces. Every previously covered face retains exactly its
prior Reading count: zero decreases and zero increases. Total Readings rise
from 475,194 to 542,359, with all
67,165 additions on newly covered faces. There is no silent face or Reading
loss and no re-coverage debt. `census-delta.json` names every changed identity.
Refresh attribution and verification are recorded below before integration.

The full censuses and all-gains sample run report zero issues, internal
failures, duplicate Readings, admission failures, byte-exact realization
failures, lexical-ownership failures and construction/leaf traversal failures.
The independent-value test establishes both roundtrip laws and exact node and
leaf identity for bare numeric and variable values, determinative and
adjectival Coordination, the property NP and its enclosing PP. Every pinned
property NP and value has exactly one Reading. The `or less` subtree equals
the prerequisite's Coordination; values are rejected as Clauses and Cardinals,
and `or greater` has no quantitative Determiner Reading. The property licence
is unavailable on unrelated nouns and plural property heads.

Word-named licensing guards added: zero. All new guards read declared
features; lexical loading and GrammarEnvironment construction succeed and
existing load-error tests remain in the gate. V3 emits no legacy coverage lock
or permitted-licensing-checker total; these are complete V3 census figures.

Assurance counts: added 4 tests; restored 0; re-spelled 0; removed 0; new
ignores 0. The two witness tests first fail against the base. The unchanged
six comparative-quantity tests also pass; neither the independent values nor
existing value assertions were weakened.

#### DISCLOSE — analyses, census and scope

The optional presentation census is 7,218 unique / 11,428
multiple before and 7,224 unique / 11,946 multiple after.
Specificity resolution is not an admission rule or a V3 report field. All
Readings are retained; Construction Costs were not tuned to coverage.

Every newly covered identity, full Reading count, preferred Reading identity
and cost, and property-value subtree is listed in ignored
`newly-covered-readings.md` and `selected-analysis-audit.json`. All 524 preferred
analyses were inspected for the value constituent: 286 determinative
Coordination occurrences, 218 adjectival Coordination occurrences, and 39
scalar Measure occurrences (543 occurrences across 524 faces). None uses
clausal `or` or a Determiner for the value. Existing licensed outer attachment
alternatives remain retained; the witness tests independently establish PP
Postmodification of the target Nominal.

Representative preferred analyses, all stamped with the after tree / covered
count above:

| Face | Complete Readings | Property / value Reading |
|---|---:|---|
| Disembowel | 3 | ManaValue / ScalarPropertyMeasure |
| Mental Misstep | 3 | ManaValue / ScalarPropertyMeasure |
| Easy Prey | 3 | ManaValue / ScalarPropertyCoordination |
| Kor Line-Slinger | 3 | Power / ScalarPropertyCoordination |
| Retribution of the Meek | 3 | Power / AdjectivalScalarCoordination |
| Valorous Stance | 6 | Toughness / AdjectivalScalarCoordination |
| Here Comes a New Hero! | 12 | ManaValue / ScalarPropertyCoordination |
| Rigo, Streetwise Mentor | 45 | Power / ScalarPropertyCoordination |
| Smother | 3 | ManaValue / ScalarPropertyCoordination |
| Reprisal | 3 | Power / AdjectivalScalarCoordination |
| Fatal Push | 136 | ManaValue / ScalarPropertyCoordination |
| Austere Command | 9 | ManaValue / AdjectivalScalarCoordination, ManaValue / ScalarPropertyCoordination |

Per-property surface census (overlapping whole-face buckets, not forecasts),
under the same before/after stamps:

| Property noun | Faces containing the noun | Covered before → after | Readings before → after |
|---|---:|---:|---:|
| power | 1,794 | 328 → 581 | 34,247 → 54,527 |
| toughness | 672 | 67 → 88 | 3,484 → 3,771 |
| mana value | 1,139 | 92 → 344 | 10,316 → 56,957 |

The narrower adjacent-value census in `census-delta.json` matches the noun
immediately followed by Arabic digits or X/Y, optionally `or` + comparative.
Its covered-face counts are power 1 → 253 of 418, toughness 0 → 20 of 240,
and mana value 0 → 252 of 576. Its Reading counts are respectively
1 → 20,254, 0 → 271 and 0 → 46,641. These surface buckets can include
independently failing hosts; they are not a parsing gate.

The [project ruling above](#project-ruling-scalar-property-values) records the
property Complement structure, the surviving `MeasuredAttribute` route and
the adjective licence. CGEL citations support only their stated passages;
the bare-value structure and adjective Coordination extension are project
rulings. No superseded lexical or grammatical route is left unreachable.

Deviations and additions:

- Added `ScalarPropertyMeasure` and `ScalarPropertyCoordination`, zero-cost
  projections into the value interface, plus `AdjectivalScalarCoordination`.
  Their inputs are disjoint; the first two reuse existing scalar composition
  and numeral Coordination. The third reflects the Adjective Category and
  applies only a declared lexical licence. No existing Construction, frame
  or lexical form was removed.
- Added `ComparativeQuantityStructure` to distinguish Coordination from a
  headed comparative DP, excluding *more than one* from this value interface.
  Added `ScalarComparativeUse` for the attested adjective use. The existing
  *power*/*toughness* route is refined and *mana value* receives its licence.
- Existing scalar composition also supplies attested numeral alternatives,
  including Frodo's *mana value 2 or 3*, and the existing NP can occur as
  *has mana value 2 or less* (Fatal Push). No additional frame, coordination
  recipe, copular host, xtask command, flag or fixture was introduced.
- Added four authentic regression tests with bare-value controls and
  independent construction. Added glossary entries for Scalar Property
  Value, Scalar Comparative Use, Comparative Quantity Structure and Measure
  Position; no glossary gap remains. No Comprehensive Rules citation changed.

STOPs: none; no recorded ruling was contradicted and no invalid gain was
accepted.

#### REPORT — provenance, inventories and performance

Named declaration inventory, under the before/after stamps: 198 ordinary +
43 shared schemas = 241 before; 201 ordinary + 43 shared schemas = 244 after.
No schema instance was added. The 250 final-breadth ceiling remains applicable.

The complete spelling-to-owner maps are identical: 38,375 spellings and
1,128 named homographs before and after. `inventories-before.json` and
`inventories-after.json` contain the full named homograph lists and form
literal lists. Form-literal/vocabulary overlap inventories are empty (`{}`),
unchanged. No spelling or lexical owner was added or retired.

Performance advisory (host load: one/five/fifteen minutes):

| Tree / whole-corpus covered | Workers | Host load | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| Before `yqupppnwlkwk` / 18,646 | 12 | `4.65/6.51/5.58` | 60,676,277,673 | 292,846 ns/B |
| After `yqupppnwlkwk` / 19,170 | 12 | `13.49/8.87/7.61` | 110,164,049,106 | 434,858 ns/B |

Both runs exceed the inherited quiet-host ceiling of 16,260,000,000 ns.
The after run overlaps gate compilation on a busier host; wall time and
per-byte CPU rise, so these figures do not isolate the change's cost. This
is an advisory, not an admission gate or a fitted target. The all-gains run
also validates all 67,165 new Readings with one preferred sample per face.
Its measured wall time is 13,993,912,489 ns and thread CPU 707,753 ns/B,
with 12 workers and host load `19.90/14.33/9.87`; stamp
`yqupppnwlkwk` / whole-corpus covered 19,170.

#### Verification and evidence

Full corpus command: `target/debug/cargo-xtask english-v3 --all --workers 12
--samples-per-face 0 --output target/english-v3/scalar-property-values-after.json`.
The binary is rebuilt through `cargo xtask gate` before direct invocation;
the before run uses `cargo xtask english-v3` with the same corpus flags.
Iteration uses explicit `--face-id` selectors for the eight named witnesses
and all 524 gains, with complete enumeration and selected samples.

Gate: `cargo xtask gate --changed --from english-v3-scalar-property-values
--run --clippy`, deriving:

```text
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Gate passes: 780 tests passed, zero failed, one inherited ignored
`macros::templates::tests::macro_schema_census_count_matches_21` (blocker:
“cross-checks the live corpus against the census; run on demand”). Strict
clippy passes with warnings denied.

Changed Rust files pass nightly formatting. A whole-workspace format check
also exposes inherited formatting differences in unrelated files; none was
included in this change. Citation checks report zero noncompliant strings
and zero stale citations across 16,113 sites; the piped diff audit selects
zero new Comprehensive Rules sites, so no blessing is needed.

All census, named inventories, deltas, analysis ledgers, runtime digests,
probe results and logs are generated under this feature workspace's ignored
`target/english-v3/`. Before retirement, this ticket's evidence is retained in
the coordinator's ignored `target/english-v3/english-v3-scalar-property-values/`.
No evidence snapshot or verifier is tracked or embedded in crate source.
