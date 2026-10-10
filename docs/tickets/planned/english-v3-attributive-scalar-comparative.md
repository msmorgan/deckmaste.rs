---
needs: []
---
# Read an attributive scalar comparative before a property noun: a spell with equal or lesser mana value

## Why

*mana value 3 or less* and *power less than X* read; the attributive order
*lesser mana value*, *greater power*, *equal or lesser power* does not.
Blocker report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`):
key `at:equal or` 10 sole-blocker faces is one corner. `cargo xtask
english-v3 grep -i '\b(equal or )?(lesser|greater|equal) (power|toughness|
mana value|mana values|life|loyalty)\b'` finds **45** faces (47 lines), all
unread; **43** faces have that line as their only failing line. By shape:
*with lesser mana value / power / toughness* about 20; *with greater power /
mana value* about 9; *with equal or lesser …* 10 (all sole); *with equal
toughness* 1; with an overt standard after the noun (*has greater power than
this creature*, *lesser mana value than it*, *greater power or toughness
than Hulkling*) about 7.

## Goal

*lesser*, *greater* and *equal* read as attributive Modifiers of a scalar
property noun with the standard left implicit (*a creature with lesser
power*), coordinate there (*equal or lesser mana value*), and take an overt
*than* standard after the noun through the existing discontinuous route
(*greater power than this creature*), with byte-exact realisation.

## Analysis

CGEL Ch. 13 §4.1.1, p. 1126, [14]: "i *They too had felt the influence of
Christianity to a greater or lesser extent.* ii *We think this is a lesser
risk than taking no action at all.*"; "*Lesser* is found only in attributive
function". That backs attributive *lesser* (and *greater*), their
coordination, and an overt *than* phrase after the noun. Ch. 5 §11(d), p.
432 n. 48: "*greater* is an adjective". That the implicit standard is
grammatical in Oracle text, and the routes below, are project rulings.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`PremodifiedNominal` (`[modifier: AdjectivePhrase, " ", head: Nominal]`,
`require modifier.AdjectiveStructure = Simple`, `require
modifier.AdjectivePremodifierUse = Yes`); schema
`DiscontinuousComparativeNominal` with the `(Nominal, Adjective, …,
DiscontinuousComparativeModifier)` instance, which today requires
`governor.frame = DiscontinuousEquality` and an Equality marker (*same name
as that land*); `ComparativeAdjectivePhrase` / `ComparativeScalarValue`
(*power less than X*, postpositive) and `AdjectivalScalarCoordination` (*4
or greater*). Lexicon (`lexicon/core.ron`): `vocab:ScalarDegree/Lesser`,
`/Greater`, `/Equal`, each with a single frame selecting a *than* / *to*
Comparative Complement, so none is a Simple Adjective Phrase. Glossary:
**Discontinuous Comparative Nominal**, **Scalar Comparative Use**.

Orchestrator rulings 2026-10-10:

- AC1. Data first: the three Adjectives declare a complement-free
  attributive use (the frame / feature `PremodifiedNominal` already reads
  through the Simple Adjective route). No new Construction. *equal or
  lesser* is then ordinary Adjective coordination.
- AC2. Overt standard: add the Ordering case to the existing
  `DiscontinuousComparativeModifier` policy (or a sibling policy row in the
  same schema instance) so an Adjective whose declared frame selects *than*
  can govern `[governor Nominal than NP]`. The licence is the governor's
  declared frame and the marker's `ExpandedComparisonMarker`, as for *same …
  as*. No new schema.
- AC3. The licence reads declared features only. Whether the noun is a
  scalar property noun is read from its declared `ScalarDenotation` if a
  restriction is needed at all; do not add a check whose only job is to
  block *lesser creature*. No test on the three words or on *power*.
- AC4. One analysis per string. The twins to avoid: (a) *power greater than
  3*, *mana value less than or equal to X* keep exactly their postpositive
  Readings (the complement-free use must not let *greater* premodify what
  follows *than*, nor read *greater than 3* as Modifier + stray PP);
  (b) *4 or greater*, *five or greater* keep the
  `AdjectivalScalarCoordination` Reading only; (c) *equal to X* keeps its
  Readings (*equal* complement-free must not strand *to X* as an Adjunct);
  (d) *equal or lesser mana value than that spell* (Saruman of Many Colors)
  has one governor coordination sharing the *than* standard, not *equal*
  alone with an implicit standard beside it.
- AC5. Newly covered faces are gains. Reading-count increases on covered
  faces are not expected; `(?i)\b(lesser|greater|equal)\b` matches 2,091
  faces, so every one is in the selector. An increase on a
  covered *N or greater* / *greater than* / *equal to* face is an AC4
  defect; report each with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--readings 3`): witness 0. Re-probe on the claim parent first.

- Reinterpret: "Counter target spell. You may cast a spell with equal or
  lesser mana value from your hand without paying its mana cost." (control
  "You may cast a spell with mana value 3 or less from your hand without
  paying its mana cost." 3+)
- Body Launderer: "When this creature dies, return another target non-Rogue
  creature card with equal or lesser power from your graveyard to the
  battlefield." (control "… with power 2 or less …" 3+)
- Prehistoric Pet: "This creature can't be blocked by creatures with greater
  power."
- Rushed Rebirth, the clause "search your library for a creature card with
  lesser mana value, put it onto the battlefield …"
- Conformer Shuriken, the clause "If that creature has greater power than
  this creature, …" (AC2)

Also 0 today (`noun-phrase`): "a creature with lesser power"; "a creature
with greater power"; "a creature with equal power"; "a permanent card with
lesser mana value". Controls that must not change: "a permanent card with
mana value 3 or less" (`noun-phrase`) 2; "Its power is 5 or greater." 1;
"Its power is five or greater." 1; "the greatest power" (`noun-phrase`) 1;
"Its power is less than 5." 1. Prehistoric Pet, Rushed Rebirth and Conformer
Shuriken were confirmed as sole failing lines from the blocker report, not
re-probed (the trunk moved under the prebuilt binary).

## Method

Standard constraints apply. Bounded census: every supported face matching
`(?i)\b(lesser|greater|equal)\b` (2,091 faces; the Lexemes gain a declared
use) plus 300 random covered controls (`--random 300 --seed 20261010`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; AC5 for increases. Full `cargo
test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_english_v3` and `deckmaste_lexical_source`, once, at the end: a
rebuild after a grammar edit takes about 2–3 minutes and the six test
binaries about 75 seconds, so do AC1 (data, loaded at runtime) and measure
before touching AC2. Tests assert the role (the comparative Adjective as
Modifier of the property noun; the coordination; the *than* phrase as the
governor's Comparative Complement; the AC4 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only. Glossary: extend **Discontinuous Comparative Nominal** with the
Ordering Adjective governor (CGEL p. 1126 [14ii]) in the landing.

## Out of scope

- *greater power or toughness than Hulkling* (2 faces): coordinated head
  nouns under one governor, unless AC2 reads it unaided.
- *the lesser of …*, *the greater of …* fused heads.
- Postpositive comparative Determinatives
  (`english-v3-postpositive-comparative-determinatives`).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
