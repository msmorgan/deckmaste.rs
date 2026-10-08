---
needs: []
---
**An object binding records no possessor, so the checker cannot enforce a
verb's own restriction on whose hand a card is in or who controls a
permanent.** Found while prototyping the actor handoff
(`semantics-v2-actor-handoff`, 2026-10-05). Standard constraints apply.

## The defect

Two shapes check clean in the prototype although the rules forbid them (seen
in the prototype, not pinned):

- `sacrifice (a creatureYouDontControl)`. [CR#701.21a]: "A player can’t
  sacrifice something that isn’t a permanent, or something that’s a permanent
  they don’t control."
- `[choose (a (.inZone yourHand)), act (target .opponent) (discard (that
  .card))]`: the discarded card is in the controller's hand, not the
  discarder's. [CR#701.9a]: "To discard a card, move it from its owner’s hand
  to that player’s graveyard."

## Cause

An object binding, `Payload.object ty zone prov orig size`
(`lean/Semantics/Check/Words.lean`), records the card types, the zone, the
provenance stamp, the origin and the size, but no possessor. The possessor
written in `.inZone (handOf p)` is dropped when the binding is made, and so
is a controller predicate such as `.hasPossessor .controller .you`. The
zone half of a verb's restriction is checked (an indefinite patient that
names no zone is selected where the deed's patient lives, 2026-10-05); the
possessor half has nothing to read.

## What a fix needs

1. A possessor or identity field on object bindings, seeded from
   zone-possessor predicates (`.inZone (handOf p)`) and controller predicates
   (`.hasPossessor .controller p`), and kept through moves.
2. Two rules, one per deed role: a sacrifice's patient is controlled by the
   performer [CR#701.21a], and a discard's patient is in the performer's own
   hand [CR#701.9a]. Each takes both forms §7 of
   `docs/decisions/semantics-v2.md` requires: a predicate conjunct on an
   indefinite patient and a demand on a definite referent.
3. Refusal pins for both shapes above, and the existing pins at their old
   outcomes.

## Landing record

Series on claim `qomtrolw`, oldest first: `lzwrwtmq` (bench re-spelling, see
the STOP), `unwrzloq` (Lean: the possessor, the two rules, the pins),
`nrxooyxo` (CONTRACTS, glossary, ADR §7 status sentence), and the commit
carrying this record. Measured on trunk `qwmoymmp`, then re-run after the
refresh onto `sxppvxpn` (keyword spelling templates: `plugins_v2/builtin`,
`construction_core`, `lexical_source`); the last refresh, onto `sknznupq`,
brought one done-ticket file and nothing was re-run for it.

**STOP: a pin outside the named ones changed outcome, and its proposed
resolution.** With the rule in place the bench card Tourach, Dread Cantor
(`lean/Semantics/Cards/Keyword.lean`, a `spelled` card, so its own
`Card.check = []` pin) was refused `patientPossessor`. Its kicked trigger,
"target opponent discards two cards at random", was spelled with no handoff:
`discard (countedAtRandom (exactly 2) (.and [.isCard, .inZone (handOf (target
.opponent))]))`, i.e. the controller discarding cards from the opponent's
hand, which [CR#701.9a] forbids (a card is discarded by its owner, from its
owner's hand). This is the defect the ticket names, found on a real bench
card. `lzwrwtmq` re-spells it as Hymn to Tourach is pinned
(`Proofs/Actor.lean` `okHymnToTourach`): `act (target .opponent)
(Actor.discard (countedAtRandom (exactly 2) .isCard))`. Same card, same
asserted outcome (`[]`), the faithful spelling. It is its own commit so the
orchestrator can rule on it before integrate. It was the only pin to move:
with it re-spelled, the full build passes. The re-spelling's commit was not
built alone under the old checker; its shape is `okHymnToTourach`'s, which
passes there.

**The fix.** `Payload.object` gains a last field `possessor : Option
Possessor` (`Possessor := you | other`, relative to the controller
[CR#109.5]; unknown is `none`). It is seeded by `Predicate.seedPossessor` in
`bindFor` (now taking the context) from `.inZone` of a possessed hand,
graveyard or library and from `.hasPossessor .controller p` and its negation
on `you`, with `actor` read in its handoff (`NounPhrase.possessorSide`).
`NounPhrase.possessor` reads it in both forms (a description's conjunct, a
pronoun's or later mention's binding). `ActFacts.ownPatient` marks sacrifice
and discard; `enactPatientPossessorOk` refuses the new `Refusal.patientPossessor`
only when the patient's possessor is known to differ from the performer's.
This is a reference agreement, in charter; no zone, timing, cost, token,
grant or frame rule was added.

**PROVE.**
- Lean full build (`lean/scripts/build`, `--wfail`): 84 jobs, no errors or
  warnings, after stage (a) and again after refresh.
- `cargo xtask lean check` (after each refresh that brought code): canon
  134/134, testing 7/7.
  `cargo xtask lean definitions`: 553/553 (73 counters, 461 subtypes, 19
  designations). `cargo xtask facts check`: up to date.
- Cite checks: `--list-noncompliant` 0; `cite check` 16,209 citations, 0
  stale; `cite audit --diff` read for every new cite (35 sites); no bless
  needed. `kata kanban check`: OK.
- Lexicon export (`cargo xtask lexical --card-name "Hymn to Tourach"
  --export`): byte-identical to trunk `qwmoymmp` and to `sxppvxpn` (sha256
  `5a273b80…eabf`).
- Rust tests and clippy: none run, because no crate's code or consumed data
  changed (the diff touches only `lean/` and `docs/`). `cargo xtask gate` was
  not run (removed by the owner).

**Stage (b), Rust mirror: no commit.** The ticket names no RON helper, and
the checker's `Payload` mirror is `crates/deckmaste_semantics_v2/src/reads.rs`,
whose mirror gap belongs to `lean-drt-anaphora-refactor`: `Payload::Object`
there has no `possessor`. Expansions are unchanged: no RON or Rust file
changed.

**Assurance counts.** Restored 0; re-spelled 1 (Tourach, Dread Cantor's
kicked discard); ignored 0; added 14 (`Proofs/Possessor.lean`); removed 0.
Every other pin kept its outcome and its text: `Payload.object`'s new field
defaults to unknown, so hand-built bindings in pins are unchanged.

**Deviations and additions.**
- Moves: the ticket says the possessor is "kept through moves". It is kept
  through a deed that leaves the object where it is and through a move
  between one player's own zones (hand, graveyard, library: the owner,
  [CR#400.1,400.3]). A move to or from a shared zone, and a gain of control,
  leave it unknown (`keptPossessor`), because a permanent's possessor is its
  controller, which a move does not carry [CR#110.2]. Keeping it there would
  refuse "gain control of target creature you don't control … sacrifice it".
- Seeding stops at what the ticket names. A library slice ("the top card of
  your library"), a created token, a searched card and a resolved permanent
  view record no possessor.
- `isYou`, `actorView`, `isYouIn` and `opponentOnly` moved earlier
  (`Abilities.lean` to `Phrase.lean`) so `bindFor` can read them. Their text
  is unchanged.
- Glossary: **Binding Possessor** added to the game-model context. The ADR §7
  sentence saying the possessor half "is not yet checked" now says it is.

STOPs: one, above, resolved by re-spelling and pending the orchestrator's
ruling.
