---
needs: []
---
# Read *up to that many*: Return up to that many target permanents that player controls to their owner's hand.

## Why

*that many* + noun reads, *up to* + cardinal + noun reads, their composition
does not. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): key `at:to that` 13 sole-blocker
faces. `cargo xtask english-v3 grep 'up to that many'` finds **20** faces,
all unread; 20 failing lines match, **17** of them the only failing line of
their face.

## Goal

*up to* bounds the degree Determinative phrase *that many* exactly as it
bounds a cardinal, in every NP position where *up to two* reads (Object,
Subject, *for* Complement, before *target*).

## Analysis

CGEL Ch. 5 §11(c), p. 431: the degree determinatives *many*, *much*, *few*
and *little* "take a very similar range of degree modifiers to gradable
adjectives, including *very*, *so*, *too*, *how*, *this*, *that*". That backs
*that many* as a determinative phrase (`DegreeDeterminativePhrase`); this
ticket relies on no further passage. Letting *up to* take it is a project ruling
(landed for cardinals in `english-v3-scalar-cardinals`, for *that many* in
`english-v3-that-degree-quantifiers`).

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`DegreeDeterminativePhrase: QuantitativeDeterminer` (`form [modifier:
lexical(Determinative), " ", head: lexical(Determinative)]; require
modifier.DegreeUse = Modifier; require head.DegreeUse = Head`) builds *that
many*; the *up to* bound from `english-v3-scalar-cardinals` composes with a
cardinal quantity, not with a `QuantitativeDeterminer`.

Orchestrator rulings 2026-10-10:

- UM1. Refine the *up to* bound so its operand may be the
  `QuantitativeDeterminer` built by `DegreeDeterminativePhrase`; no new
  Construction and no multiword lexeme.
- UM2. The licence reads the operand's declared features (`DegreeUse`, its
  agreement number). No test on *that* or *many*. *up to that much* is
  unattested: no work, and no check to block it.
- UM3. One analysis per string. *up to that many target creatures* is one
  bounded Determiner over the Nominal. It must not also read with *up to* as
  a Preposition taking the NP *that many target creatures* (a goal or limit
  PP), nor with *that* as a demonstrative Determiner of *many target
  creatures*.
- UM4. Newly covered faces are gains. Reading-count increases are possible on
  covered faces with *up to* + cardinal or with *that many*; none is
  expected, and each is reported with its shape.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Cephalid Constable: "Whenever this creature deals combat damage to a
  player, return up to that many target permanents that player controls to
  their owner's hand." (main clause probed: 0)
- Heroic Feast (first sentence): "Whenever you gain life, choose up to that
  many target creatures you control." ("Choose up to that many target
  creatures you control." 0)
- Swashbuckler Extraordinaire (second sentence): "When you do, up to that
  many target creatures gain double strike until end of turn." (main clause
  0)
- Scapeshift (second sentence): "Search your library for up to that many
  land cards, put them onto the battlefield tapped, then shuffle." ("Search
  your library for up to that many land cards." 0)

Controls that must not change: "Return that many target permanents that
player controls to their owner's hand." 2; "Return up to two target
permanents that player controls to their owner's hand." 2; "Search your
library for that many land cards." 3; "Search your library for up to two land
cards." 3; "That many target creatures gain double strike until end of turn."
1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bup to\b` or `\bthat (many|much)\b` case-insensitively
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
UM4 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (*up to* bounding the degree Determinative phrase, that phrase as
Determiner of the Nominal, the UM3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *distribute up to that many +1/+1 counters among …* (Lathiel) and *pay any
  amount of {E} … choose up to that many* where another construction fails.
- *as many … as* comparatives (*draws as many cards as they discarded this
  way*, Forget).
- *up to X*, *up to one* as a modal header count
  (`english-v3-modal-header-quantities`).

## Landing record
