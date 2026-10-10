---
needs: []
---
# Read *N or more* / *N or less* before a measured mass noun: A player has 13 or less life.

## Why

A Comparative Quantity reads before a count plural (*5 or more cards*) but not
before a measured mass noun, although the plain numeral does (*13 life*,
*3 damage*) and so does the bounded cardinal (*at least 7 life*). The
2026-10-10 triage (`docs/memory/english-v3-blocker-triage-2026-10-10.md`,
blocker report at trunk `vwlkmvns`) ranks this fourth, about 82 sole-blocker
faces. `cargo xtask english-v3 grep '\b([0-9]+|X|[a-z]+) or (more|less) (life|damage|mana|combat damage|noncombat damage)\b'`
finds **121** faces; 99 of them have a single failing line of that shape; matches in those lines:
*or less life* 37, *or more life* 29, *or more damage* 18, *or more mana* 16,
*or less damage* 1. *N or fewer life* has 0 faces.

## Goal

*N or more* and *N or less* read as the quantity of a Measured Noun Phrase
(*13 or less life*, *25 or more life*, *3 or more damage*, *five or more
mana*) wherever the plain numeral reads today, with byte-exact realisation.

## Analysis

CGEL Ch. 5 §7.6, p. 386: "The numerical nature of *one* is seen in the
coordination with *or more* in [iiia] (cf. *three or more*)". Ch. 5 §7.11,
p. 393: "***much*** and ***little*** non-count singulars" (the comparative
forms *more*, *less* are theirs). Those back the numeral + comparative
Coordination and *less* / *more* with a non-count head. A numeral measuring a
mass noun (*13 life*) is Oracle register: project ruling, as in the measured
damage landing.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`NumeralComparativeCoordination` (Category `ComparativeQuantity`,
`[left: QuantityConjunct, marker, right: Determinative]`, exports
`number = Plural`), consumed only by `ComparativeQuantityDeterminer` →
`QuantifiedNounPhrase` (count plural heads). The measured routes are
`MeasuredNounPhrase` (`[quantity: MeasurePhrase, head: Nominal]`, reads
*13 life*), `CardinalMeasuredNounPhrase` (`[quantity: Cardinal, …]`, reads
*five mana*) and `CardinalMeasuredNominal` (already takes a
`QuantityConjunct`). *more* and *less* declare `ComparativeQuantityUse: Yes`
and `MeasureModifierUse: Yes` (`lexicon/core.ron`); *fewer* declares only the
first.

Orchestrator rulings 2026-10-10:

- BQ1. Refine the measured routes: their quantity admits a Comparative
  Quantity of the Coordinated structure
  (`ComparativeQuantityStructure = Coordinated`). No new NP Construction; if a
  second consumer row is unavoidable it is a refinement of the existing
  quantity slot, recorded as such.
- BQ2. Which comparative may bound a measured mass head is read from the
  comparative Determinative's declared feature (the existing
  `MeasureModifierUse`, or a declared count / non-count selection), never from
  the word. That is what keeps *fewer* out; do not add a check whose only job
  is to block *N or fewer life* (0 faces, pruning rule).
- BQ3. One analysis per string. *13 or less life* must not also read through
  `QuantifiedNounPhrase` (the mass head stays unlicensed there), nor as an NP
  Coordination *13 or [less life]*. *5 or more cards* keeps exactly its
  Readings.
- BQ4. Number: the measured NP stays singular, as *13 life* is
  (*4 or more damage was dealt*); the Coordination's own `number = Plural`
  export concerns count heads only.
- BQ5. The postpositive forms (*mana value 3 or less*, *power 4 or greater*)
  already read and are controls.
- BQ6. Newly covered faces are gains. Any Reading-count increase on a
  previously covered face is reported with its shape before landing; under
  BQ3 most are defects.

## Witnesses

Each is the face's only failing line at `vwlkmvns`. Probed 2026-10-10 with the
`zrrnvqto` binary (`--category sentence --readings 3 --as-card`): witness 0;
control = the same sentence with the plain numeral, 1 each. Trunk has since
taken `nvotwtqp`; re-probe on the claim parent first.

- Abandoned Campground: "This land enters tapped unless a player has 13 or
  less life." (control "… has 13 life.")
- Divinity of Pride: "This creature gets +4/+4 as long as you have 25 or more
  life." (control "… have 25 life.")
- Innocent Bystander: "Whenever this creature is dealt 3 or more damage,
  investigate." (control "… is dealt 3 damage, …")
- Callous Giant: "If a source would deal 3 or less damage to this creature,
  prevent that damage." (control "… deal 3 damage to …")
- Dragonborn Champion: "Whenever a source you control deals 5 or more damage
  to a player, draw a card." (control "… deals 5 damage to …")

Controls that must not change: "You have 5 or more cards in hand." 2; "A
player has 13 life." 1; "You have at least 7 life." 1; NP "mana value 3 or
less" 1; NP "4 or more counters" 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b([0-9]+|X|[a-z]+) or (more|less|fewer) ([a-z]+ )?(life|damage|mana|cards|counters)\b

(629 faces: the mass heads and two count-head control classes) plus 300 random
covered controls (`cargo xtask english-v3 diff --manifest … --random 300
--seed 20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; BQ6 for increases.
Full `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_english_v3` (and `deckmaste_lexical_source` if a declaration
changes), once, at the end: a rebuild after a grammar edit takes about 2–3
minutes and the test binary about 7, so batch edits. Tests assert the role
(the Comparative Quantity as the measured NP's quantity, its Quantity Conjunct
and comparative Determinative, singular agreement, and the BQ3 negatives),
never an incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- *N or more mana* is in scope only as a by-product of BQ1 (16 lines; *five
  mana* reads through `CardinalMeasuredNounPhrase`); a mana-specific gap is
  listed, not built.
- *more life than NP* (`english-v3-comparative-quantity-than`); *N life more
  than …* (`english-v3-postpositive-comparative-determinatives`).
- *exactly N*, *no more than N*.

## Landing record
