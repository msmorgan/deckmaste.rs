---
needs: []
---
# Read irrealis *were* with a measured comparative: This creature crews Vehicles as though its power were 2 greater.

## Why

The Pilot line fails, and two shortened variants show two gaps. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.json`): key
`at:greater .` 20 sole-blocker faces (17 of them this line; the other 3 are
*with power 3 or greater* after *only to*, and *power or toughness 4 or
greater*). `cargo xtask english-v3 grep 'were \d+ greater'` finds **18**
faces, all unread; 18 failing lines match, **17** of them the only failing
line of their face (13 inside a quoted token ability).

Gap 1: "This creature crews Vehicles as though its power were 2." 0, while
"Its power is 2." 1 and "This creature attacks as though it weren't tapped."
2. Gap 2: "Its power is 2 greater." 0, while "Its power is greater than 2."
1.

## Goal

(1) Irrealis *were* takes the scalar Complements realis *be* takes. (2) A
comparative Adjective with a measure Modifier and no *than* Complement (*2
greater*) reads as a predicative Complement. Together: the witness line.

## Analysis

CGEL Ch. 13 §5.5, p. 1152: "*… as though he were remembering painful
things*": irrealis *were* in the Complement of *as though*. That backs the
irrealis clause already landed; it says nothing about which Complements
*were* takes, and this ticket relies on no further passage. The measured bare comparative is a
project ruling. Glossary: **Irrealis Clause**, **Irrealis Clause
Complement**, **Measure Modifier Use**, **Scalar Location**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`IrrealisClause` (`form [subject: NominativePhrase, " ", predicate:
IrrealisPredicate]`), `IrrealisPredicate` built by `SelectedPredicate` with
`IrrealisHeadAgreement` (`require head.form = Irrealis`) and by
`BareAuxiliaryPredicate` with `ParticipialComplement`;
`IrrealisClauseComplementPreposition` (`require
head.IrrealisClauseComplement = Yes`) for *as though*. The realis copular
scalar route is `MeasuredScalarLocation: ScalarLocationComplement`
(`english-v3-copular-scalar-location`). **Measure Modifier Use** is a
comparative head's declared permission to take a measure as degree Modifier
(`english-v3-cost-scalar-complement`).

Orchestrator rulings 2026-10-10:

- IS1. Gap 1 is an instance gap: give `IrrealisPredicate` the scalar-location
  Complement instance the finite copula has. No new Construction; the licence
  is the copula's frame and `form = Irrealis`.
- IS2. Gap 2: refine the comparative AdjP so a head with `MeasureModifierUse`
  takes its measure Modifier when the *than* Complement is absent. The licence
  reads that declared feature. No test on *greater* or on the numeral.
- IS3. One analysis per string. *2 greater* is measure Modifier + comparative
  head. It must not also read as the scalar value *2* followed by a stray
  Adjective, and "Its power is greater than 2." keeps exactly 1. *power 3 or
  greater* (postpositive bounded property) is a different construction and
  keeps its Readings.
- IS4. If Gap 2 needs more than the refinement in IS2 (for instance an
  understood standard of comparison as a feature), STOP after landing Gap 1
  and report: Gap 1 alone covers no face.
- IS5. Newly covered faces are gains. Reading-count increases are possible on
  covered faces with a measure before a comparative (*{1} less*, *2 more*)
  and on covered *as though … were* faces; none is expected, and each is
  reported with its shape.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Hotshot Mechanic: "This creature crews Vehicles as though its power were 2
  greater." (0)
- Deathless Pilot, Dynamite Diver, Cloudspire Captain: "This creature saddles
  Mounts and crews Vehicles as though its power were 2 greater." (not probed
  separately; "This creature saddles Mounts and crews Vehicles." 1)
- Prodigy's Prototype: "… create a 1/1 colorless Pilot creature token with
  "This token crews Vehicles as though its power were 2 greater."" (quoted
  form; not probed)

Also 0 today: "This creature can crew Vehicles as though its power were 2
greater."; "Its power is 2 greater than its toughness." (unattested;
localisation only). Controls that must not change: "This creature crews
Vehicles." 1; "Its power is 2." 1; "Its power is greater than 2." 1; "You may
cast it as though it had flash." 2; "This creature attacks as though it
weren't tapped." 2; "Destroy target creature with power 4 or greater." 2.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bwere\b|\bweren't\b` or `\b(greater|less|more|fewer)\b` case-insensitively
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
IS5 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (*were* as irrealis copula with a scalar Complement, the measure as
Modifier of the comparative head, the IS3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *can be attached only to a creature with power 3 or greater* (Gate
  Smasher, O-Naginata, Konda's Banner): "This Equipment can be attached only
  to a creature." 0, a focused PP Complement of a passive.
- *with power or toughness 4 or greater* (Repel Calamity): coordinated
  property heads.
- Semantics of crewing with modified power.

## Landing record
