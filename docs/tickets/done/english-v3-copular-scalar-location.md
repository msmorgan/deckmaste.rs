---
needs: [english-v3-comparative-quantity-determiners]
---
# Read copular scalar location: its power is N, is N or less

## Why

Split from `english-v3-scalar-comparisons` (2026-10-06), which bundled four
constructions. This ticket owns one: *be* with a scalar value as predicative
Complement (*its power is 2*, *its mana value is 2 or less*, *The number of
cards in your hand is three.*). On change `wlvwtnppyovn` (32,828 supported
faces, 13,716 covered, 19,112 unread), the surface bucket overlaps the *N or
more* bucket (2,095 touched / 274 sole) and has no count of its own; probes
(admitted roots):

- "If that creature's power is 0, destroy it." 0
- "If that creature's power is 0 or less, destroy it." 0
- "The number of cards in your hand is three." 0 in either number on
  2026-10-06; the *becomes* version reads. A copula gap, not a concord one.

This ticket takes over from `english-v3-systemic-residuals` its "Copular
scalar location" item.

## Goal

A scalar value (numeral, *X*, or the numeral-*or*-comparative coordination
built by `english-v3-comparative-quantity-determiners`, the `needs:` edge)
reads as the predicative Complement of *be* when the Subject denotes a
quantity or scalar property. Reuse that coordination; do not build a second
one. Do not admit a value Complement after a Subject that denotes neither.

## Analysis

Simple location on a scale is commonly expressed by *be* with an NP as
predicative Complement: *The price is $12*, *The temperature is 10°*, *This
case is over 20 kilos* (CGEL, Ch. 8, §5.4, p. 693, [11]–[12]). *Its power is 0*
and *The number of cards in your hand is three* are this construction. The
numeral-*or*-comparative coordination is cited in the quantity-determiner
ticket (Ch. 5, §7.6, p. 386, [44iiia]).

The split ticket grouped *is equal to* and *is less than* with this host.
They are be + comparative AdjP, a different predicative Complement, and belong
to `english-v3-comparative-complements`.

## Witnesses

- Savage Swipe: "Target creature you control gets +2/+2 until end of turn if
  its power is 2."
- Stature, Size Shifter: "Stature can't be blocked if her power is 1 or less."
- Depressurize: "Then if that creature's power is 0 or less, destroy it."
- Domestication: "At the beginning of your end step, if enchanted creature's
  power is 4 or greater, sacrifice this Aura."
- Guidelight Pathmaker: "Put it onto the battlefield if its mana value is 2 or
  less."
- Technodrome: "This creature can't attack or block unless its power is 6 or
  greater."

Each witness's only failing unit is the one quoted (recon on `wlvwtnppyovn`).
Also pin the synthetic "The number of cards in your hand is three." in both
numbers.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/copular-scalar-location-before.json` on the
   claim parent, stamped with its change id.
2. Write each witness as a test first, with a negative probe whose Subject is
   not scalar (for example "If that creature is 2, destroy it.").
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *or less* read as clausal
   *or*-coordination, or a value Complement admitted after a non-scalar
   Subject, is a defect, not a gain.
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

- *is equal to*, *is less than*: `english-v3-comparative-complements`.
- *with power 2*, *with mana value X*: `english-v3-scalar-property-values`.
- *where X is* (1,119 touched / 114 sole) and *the number of* (1,296 / 9) as
  constructions: note how many become readable, but do not build for them.
  Magma Sliver's scalar Subject *X* (from the done
  `english-v3-number-transparent-concord`) stays with
  `english-v3-systemic-residuals`.
- Life-total comparisons (*your life total is less than …*) are comparatives,
  not this host.

## Landing record

In addition to the standard record: before/after counts stamped with change
ids; the Subject feature that licenses the value Complement; the status of
the synthetic number-of probe in both numbers; timings as integer ns and ns/B
with host load and worker count.

### PROVE — refreshed base and retained Readings

Whole-corpus stamps: the claim-parent runtime measured before editing is
`srovlwxp` / covered **19,170**; the refreshed base census is `xrrvrpoz` /
covered **19,170**, inherited by claim `uytxnvoo`; the implementation census
is `lxupmwno` / covered **19,205**. Both base censuses agree for every face.
All reports enumerate the same 32,828 supported faces without a Reading limit.

| Census | Unread | Unique | Multiple | Undetermined | Complete Readings |
|---|---:|---:|---:|---:|---:|
| Refreshed base `xrrvrpoz` / 19,170 | 13,658 | 7,224 | 11,946 | 0 | 542,359 |
| After `lxupmwno` / 19,205 | 13,623 | 7,233 | 11,972 | 0 | 543,312 |

All 19,170 previously covered faces retain exactly their former Reading counts:
zero lost faces, zero removed Readings, zero existing-face count increases.
The 35 new faces contribute all 953 new Readings. `census-delta.json` names
every changed identity; no re-coverage debt is created.

Trunk-attributed Reading-count decreases: none; refresh imports `yryvqxsx`
and `xrrvrpoz`, which change comparative attribution comments and documentation
only. Their refreshed census agrees face by face with our original baseline.

Every counted Reading passes admission, lexical ownership, byte-exact realization
and construction/leaf traversal identity: zero issues, duplicate Readings,
cyclic derivations or internal failures. Independent values cover the new
copular host, the shared numeral Coordination and the spelled cardinal value;
negative independent values reject a non-scalar Subject and the wrong frame.
Pinned scalar clauses and value fragments have one Reading each, and *or less*
retains the prerequisite Coordination rather than a clausal *or* analysis.

Word-named licensing guards added: **0**. Licensing reads declared grammatical
features; the existing environment load-error tests pass in the gate. V3 emits
neither a legacy coverage lock nor a permitted-licensing-checker total; these
are complete retained-Reading census figures.

### DISCLOSE — analysis and scope

`ScalarDenotation` licenses the Subject. It is declared on *power*, *toughness*,
*mana value* and *number*, retained through ordinary headed phrases and
intersected through Coordination. Measured amounts also denote quantities;
number-transparent quantificational uses instead denote their Oblique and
do not acquire this licence. Non-scalar Subjects and the deferred scalar
variable Subject *X* remain excluded from this host.

The `ScalarLocation` frame selects `ScalarValue` and composes only with the
licensed Subject through the finite copular host. Existing full, negative
and clitic *be* declarations retain their shared frame inventory; the existing
clitic frame-parity test is preserved. No Lexeme or Word Form was added.
CGEL Ch. 8 §5.4, p. 693 supports *be* with an NP predicative Complement for
scalar location. The scalar-Subject restriction and the typed frame/value
interfaces are this ticket’s project contract, not claims attributed to CGEL.

The synthetic singular “The number of cards in your hand is three.” has two
Readings, differing in the attachment of *in your hand*. Its plural-*are*
counterpart has zero. Existing number-of composition and concord are reused.

Every newly covered face is listed below. Each preferred Reading contains
`ScalarLocation` with `ScalarLocationPredicate`, an NP Subject headed by the
listed property, and the listed value Reading. These stamps are `lxupmwno` /
covered 19,205. `gain-analysis-ledger.json` records each preferred fingerprint,
complete source and scalar subtree; all 953 new Readings are validated again
in the all-gains run. No wrong analysis is accepted as a gain.

| Face identity | Readings | Selected scalar Complement |
|---|---:|---|
| Zaffai, Thunder Conductor (`02d529df-6299-4e8f-9794-70c8061efdc4#card`) | 32 | ManaValue: `AdjectivalScalarCoordination` |
| Containment Breach (`043dd2e5-21e4-49b9-90b2-831e587b2238#card`) | 5 | ManaValue: `ScalarPropertyCoordination` |
| Overload (`07159efc-c69f-4164-a8ca-9da641dbf702#card`) | 12 | ManaValue: `ScalarPropertyCoordination` |
| Perilous Voyage (`0ab090db-f8fc-42ff-95db-e9370796db97#card`) | 4 | ManaValue: `ScalarPropertyCoordination` |
| Eshki, Temur's Roar (`1b162dd3-3be6-406d-bf86-f7cc9eff098d#card`) | 10 | Power: `AdjectivalScalarCoordination` |
| Writ of Passage (`1ea7f839-e3b8-4aff-b850-c0a98b06ae14#card`) | 36 | Power: `ScalarPropertyCoordination` |
| Reject Imperfection (`25781dc5-d6ea-4273-9fe3-06a8808b3718#card`) | 1 | ManaValue: `ScalarPropertyCoordination` |
| Witch-Maw Nephilim (`26edbbb8-331e-42a6-a550-acc7e9efb32c#card`) | 12 | Power: `AdjectivalScalarCoordination` |
| Eagle of Deliverance (`4446e59c-29f7-4722-92be-2625c1d3dab9#card`) | 4 | Power: `ScalarPropertyCoordination` |
| Deathknell Berserker (`4658ae19-d203-4c12-b56e-42cd8f0db91d#card`) | 1 | Power: `AdjectivalScalarCoordination` |
| Technodrome (`51c0b46c-c379-420a-a4a4-d18724c759aa#card`) | 7 | Power: `AdjectivalScalarCoordination` |
| Soul Search (`58f12f0d-e2c1-48f9-9e27-d1de8db743cb#card`) | 32 | ManaValue: `ScalarPropertyCoordination` |
| Tainted Treats (`60826714-b9fd-4c89-b9f2-1a3c3229686b#card`) | 1 | ManaValue: `ScalarPropertyCoordination` |
| Fading Hope (`64883ef2-c536-42e2-b52f-cfd1984fcdfd#card`) | 2 | ManaValue: `ScalarPropertyCoordination` |
| Ent's Fury (`6bc3c7fd-e371-467f-973c-4c9f0d1ba0bc#card`) | 3 | Power: `AdjectivalScalarCoordination` |
| Extinguish the Light (`6dbef43d-40d3-4894-86a5-d31a5026604e#card`) | 1 | ManaValue: `ScalarPropertyCoordination` |
| Tribute to the World Tree (`72deedab-7c17-4505-aeca-4bc8596d80a5#card`) | 3 | Power: `AdjectivalScalarCoordination` |
| Cyclonus, the Saboteur (`7ead01b9-def3-4f9d-a860-af64e94cb45e#face:0`) | 12 | Power: `AdjectivalScalarCoordination` |
| Thieves' Tools (`7fe361ef-a168-4847-92a2-21c1661aac06#card`) | 4 | Power: `ScalarPropertyCoordination` |
| Domestication (`86da33ac-f8ac-48c8-a50b-22fd761dff8f#card`) | 1 | Power: `AdjectivalScalarCoordination` |
| Thunderous Velocipede (`899007b5-6a07-4491-9104-bb537106b07d#card`) | 600 | ManaValue: `ScalarPropertyCoordination` |
| Reptilian Recruiter (`a12a66b5-ea4a-4a58-b0f0-13152598b7e6#card`) | 31 | Power: `ScalarPropertyCoordination` |
| Depressurize (`abcb5321-c295-4abf-8622-7de79679d27a#card`) | 2 | Power: `ScalarPropertyCoordination` |
| Carnivorous Canopy (`b3360259-19df-43c6-95a9-e76770f2fb77#card`) | 4 | ManaValue: `ScalarPropertyCoordination` |
| Stature, Size Shifter (`bd49a05c-ed51-4d69-ab9b-11192442ec17#card`) | 32 | Power: `ScalarPropertyCoordination` |
| Seedship Impact (`c6d89c4a-351f-4d63-ba57-b978a695191e#card`) | 1 | ManaValue: `ScalarPropertyCoordination` |
| Savage Swipe (`c74c87f7-3120-4fd7-8735-1d7a7a429643#card`) | 3 | Power: `ScalarPropertyMeasure` |
| Vindictive Triumph (`d6b478d1-5015-49ba-b1aa-cbc2b08107a6#card`) | 28 | ManaValue: `ScalarPropertyCoordination` |
| Prohibit (`d95f1797-56f9-41be-a5fe-8398961f4b8b#card`) | 12 | ManaValue: `ScalarPropertyCoordination` |
| Sound the Trumpets (`dbcf2a79-b455-43ac-9cf0-9e63708800f4#card`) | 1 | ManaValue: `ScalarPropertyCoordination` |
| Gore Vassal (`e4f44a37-1e66-4f60-9af2-1e67ba58445b#card`) | 1 | Toughness: `AdjectivalScalarCoordination` |
| Raze to the Ground (`efe24eed-e1e2-4807-ae61-2e5be529ae3c#card`) | 1 | ManaValue: `ScalarPropertyCoordination` |
| Prismari Pianist (`f58117e4-ba85-41b8-8fd5-4245716a84dc#card`) | 12 | ManaValue: `AdjectivalScalarCoordination` |
| Viv Vision, Teen Synthezoid (`f67a599c-200a-4831-9e73-126c7435a396#card`) | 6 | Power: `AdjectivalScalarCoordination` |
| Guidelight Pathmaker (`f8103865-5086-4f16-a35a-a56acdab1547#card`) | 36 | ManaValue: `ScalarPropertyCoordination` |

Unique/multiple census changes are +9/+26, all on the 35 new faces. No
specificity arbitration or Construction Cost change was introduced. The
600 Readings of Thunderous Velocipede retain surrounding composition/scopes;
its local scalar clause uses the same single value route as the pinned probes.

Case-insensitive source buckets, under the same before/after stamps:
*where X is*: 1,119 faces, covered 0 → 0; *the number of*: 1,316 faces,
covered 246 → 246 and 68,423 Readings unchanged. No construction was built
for either bucket. Scalar variable Subjects remain routed to
`english-v3-systemic-residuals`.

**Deviations and additions:**

- Added `ScalarLocation`, `ScalarLocationPredicate` and `ScalarCardinalValue`.
  The first two separate a Subject-restricted frame from ordinary predication;
  the third supplies the ticket’s spelled-cardinal probe. Its numeral-kind
  licence is disjoint from the existing scalar measure and Coordination routes.
- Renamed the shared Category `ScalarPropertyValue` to `ScalarValue` on both
  grammar and lexicon sides. The noun-specific glossary term keeps its meaning;
  both noun and copula reuse the existing value/Coordination Readings. No old
  Category declaration or frame reference remains. No existing Construction
  or lexical route was retired.
- Added Scalar Location, Scalar Denotation and Scalar Value to the owning
  glossary, and reconciled its existing Scalar Property Value definition with
  the general value Category. No glossary gap remains.
- Re-spelled two existing scalar-property tests against the renamed Category,
  retaining their same cards, structure, value equality and outcomes. Added
  six tests with authentic fragments, the expressly required synthetic probe,
  negative controls and independent values. Counts: restored 0, re-spelled 2,
  newly ignored 0, added 6, removed 0. No test was weakened.

STOPs: none; no recorded ruling was contradicted. The initial expected
copula/cardinal failures were fixed within this ticket. Clippy’s test-only
needless-box replacement finding was fixed and the focused tests rerun.

### REPORT — inventories, performance and gate

Under the whole-corpus stamps above: 201 ordinary + 43 shared schemas = 244
named Constructions before; 204 ordinary + 43 shared schemas = 247 after.
The 43 instance matrices and their 315 Category instances are unchanged.
After `lxupmwno` / covered 19,205 has 144 declared Categories and 681 compiled
Productions in the shared lexical environment. The final-breadth ceiling
remains 250 named Constructions.

Both spelling inventories have 38,375 declared spellings and identical
1,128 named homographs. `inventories-before.json` and `inventories-after.json`
contain the complete homograph lists and identical form-literal inventories;
form-literal/vocabulary overlap is empty (`{}`) in both. No spelling owner
was added or retired.

Performance advisory; host load is one/five/fifteen minutes:

| Tree / whole-corpus covered | Workers | Host load | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| Before `srovlwxp` / 19,170 | 12 | `9.11/9.61/9.58` | 79,577,367,905 | 351,475 ns/B |
| After `lxupmwno` / 19,205 | 12 | `4.36/4.92/7.92` | 104,087,538,429 | 408,377 ns/B |

Both exceed the inherited quiet-host ceiling of 16,260,000,000 ns. The after
run overlapped gate compilation; these host-dependent figures do not isolate
the change’s cost and are advisory rather than admission gates.
The all-gains run (`lxupmwno` / whole-corpus covered 19,205) uses 4 workers
at host load `12.91/10.72/9.82`: 344,295,348 ns and 232,836 ns/B.

`cargo xtask gate --changed --from uytxnvoo --run` exits 0, deriving:

```sh
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
```

All 786 tests pass, zero fail. The one inherited ignore is
`macro_schema_census_count_matches_21`, whose attribute names its live-corpus
cross-check blocker. The post-Clippy focused suite passes all six tests,
including the added independent cardinal assertion. Strict Clippy for the
same four packages, with `--all-targets -- -D warnings`, exits 0. Changed
Rust files pass nightly rustfmt. Citation checks report zero noncompliant
strings and zero stale citations across 16,113 sites; the piped diff audit
selects zero changed Comprehensive Rules sites, so no blessing is needed.

The first refresh imports the comparative attribution/documentation follow-up,
with no conflicts. Final refresh also imports `lryqtwsk`, which changes
ignore rules and an anaphor ticket only; all 1,832 recorded runtime digests
remain identical (`final-refresh-digests.json`), so all per-face counts and
verification results above are retained. No trunk-attributed Reading decrease
is introduced. Against its 1,832 runtime/declaration dependency digests,
only our grammar declaration and core lexicon differ. All census files,
named inventories, probes, ledgers, scripts and logs remain under this
workspace’s ignored `target/english-v3/`; evidence is retained before retirement
in the coordinator’s ignored `target/english-v3/english-v3-copular-scalar-location/`.
No evidence snapshot, process verifier or temporary file is tracked.

### Review notes (2026-10-07)

- **Undisclosed reach, disclosed here.** `ScalarCardinalValue` joined the
  shared `ScalarValue` category, which is also the `quantity` slot of
  `MeasuredAttribute`, so spelled values after scalar property nouns
  ("power three") now read. Corpus effect: zero faces.
- **Unattested exports.** The measured-amount `ScalarDenotation = Yes` exports
  from `CardinalMeasuredNominal`, `CardinalMeasuredNounPhrase` and
  `MeasuredNounPhrase` have no gained face with a measured-amount Subject
  ("Two damage is 3." reads 0). Routed to [english-v3-systemic-residuals](../planned/english-v3-systemic-residuals.md) for removal under the pruning
  rule.
- **Reconciliation.** 77 faces containing "(power|toughness|mana value) is"
  remain unread. About 62 belong to other constructions (*equal to the
  number* 21, *less than or equal* 9, *equal to your devotion* 5, …). The rest
  are blocked outside the scalar clause (Greenhilt Trainee: activation
  restriction; Phyrexian Devourer, Aradesh: another sentence).
