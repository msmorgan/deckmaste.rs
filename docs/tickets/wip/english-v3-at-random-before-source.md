---
needs: []
---
# Read *at random* between an Object and the verb's source Complement: Return a creature card at random from your graveyard to your hand.

## Why

*at random* reads at the end of the verb phrase and before a free *from*
phrase ("Discard a card at random." 1; "Target opponent discards a card at
random from their hand." 1; "Return a creature card from your graveyard to
your hand at random." 1). It does not read where the verb selects what
follows: "Return a creature card at random from your graveyard to your
hand." 0, "Return two cards at random from your graveyard to your hand." 0,
"Put a card at random from your graveyard into your hand." 0, "Return a card
from your graveyard at random to your hand." 0, beside "Return a creature
card from your graveyard to your hand." 1. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:random from` 11, part of
failing token `from` 30. `cargo xtask english-v3 grep -i '\bat random
from\b'` finds **41** faces: 26 covered (*reveals / exiles / chooses a card
at random from* …) and **15** unread, **12** with that line as their only
failing line. 13 of the 15 are *return* NP *at random from your graveyard to
your hand / to the battlefield*.

## Goal

The manner phrase *at random* reads between the Object of *return* and its
selected *from* … *to* … Complements, with byte-exact realisation and with
the Object, source and goal still filling the verb's frame.

## Analysis

No CGEL claim is relied on for the position; that a manner Adjunct may stand
between an Object and the verb's PP Complements in this register is a
project ruling, measured on the corpus (13 faces for *return*).

Existing routes: `core-verb:Return`'s frame
(`crates/deckmaste_lexical_source/lexicon/verbs.ron`:
`Argument(Object: SourceFreeObject)`, `Role("SourceComplement")`,
`Lex("Preposition", "To")`, `ObjectNounPhrase`, …), whose slots are adjacent;
the selected-complement tails `ObjectPrepositionTail`, `ObjectLocativeTail`
and `ObjectLocativeMannerTail`
(`crates/deckmaste_english_v3/src/declarations.rs`), the last of which
already places a manner PP after a locative Complement;
`InterveningPrepositionAdjunct` (`[" ", head: PrepositionPhrase]`, `require
head.InterveningAdjunctLicence = Yes`, `licence_adjunct(…)`), the optional
Adjunct that `SelectedGapPrepositionPredicate` admits between a verb and its
marker. *at random* is `vocab:Preposition/At` + `vocab:Adjective/Random`
(both declare `AdjectiveComplementClass: Manner`); only
`vocab:Preposition/As` declares `InterveningAdjunctLicence: Yes` today.

Orchestrator rulings 2026-10-10:

- AR1. Refine the selected-frame realisation, no new Construction: the
  optional `InterveningPrepositionAdjunct` is admitted after the Object slot
  and before a source Complement, as it is admitted after the verb in
  `SelectedGapPrepositionPredicate`.
- AR2. The licence is declared: the manner PP carries
  `InterveningAdjunctLicence` (declared on the manner Complement class of
  *at* / on *random*, whichever the PP's export reads), and the position is
  open only in frames whose next slot is a source Complement, read from the
  frame's declared slot role. No verb, noun or preposition is named.
- AR3. One analysis per string. The 26 covered *at random from* faces keep
  exactly their Readings: where *from their hand* is a free Adjunct or an NP
  postmodifier nothing changes, and *reveals a card at random from their
  hand* must not gain a Reading through the new position. *at random* is not
  a postmodifier of the Object NP (*a card at random* is not an NP): do not
  reach the witnesses by licensing that.
- AR4. Expected shape of increases: none. If `as`-phrases (the other
  declarer of the licence) start reading in the new position on covered
  faces, report them with their shape before landing.
- AR5. Newly covered faces are gains. *put* NP *at random from* … *into* …
  is unattested (0 faces); it may read as a consequence and is not blocked.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Surreal Memoir: "Return an instant card at random from your graveyard to
  your hand."
- Make a Wish: "Return two cards at random from your graveyard to your hand."
- Ghoulraiser: "When this creature enters, return a Zombie card at random
  from your graveyard to your hand."
- Haunted Fengraf: "{3}, {T}, Sacrifice this land: Return a creature card at
  random from your graveyard to your hand."
- Moldgraf Monstrosity: "When this creature dies, exile it, then return two
  creature cards at random from your graveyard to the battlefield."

Controls that must not change: "Return a creature card from your graveyard to
your hand." 1; "Discard a card at random." 1; "Target opponent discards a
card at random from their hand." 1; Merfolk Spy "Whenever this creature deals
combat damage to a player, that player reveals a card at random from their
hand." 3 (census); Fall "Target player reveals two cards at random from
their hand, then discards each nonland card revealed this way." 6 (census);
Ogre Shaman "{2}, Discard a card at random: This creature deals 2 damage to
any target." 1 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bat random\b|\breturns? [^.]* from [^.]* to\b` (184
faces contain *at random*; record the total) plus 300 random covered
controls, `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff` against the claim parent. Zero lost; AR4 for increases. Full
`cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end; a
rebuild after a grammar edit takes about 2–3 minutes, the six test binaries
about 75 seconds; lexicon data loads at runtime. Tests assert the role (*at
random* as an Adjunct of the *return* predicate; Object, source and goal in
their frame slots; the AR3 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Crabomination (*the top card of their library, a card at random from their
  graveyard, and a card at random from their hand*: Object coordination).
- The Deck of Many Things (table rows).
- Kheru Lich Lord (also *anywhere else*, `english-v3-anywhere-else-goal`);
  Fossil Find (*then reorder your graveyard as you choose*); Tomb Tyrant
  (*at least three* … restriction) if those tails block.

## Landing record

To be written by the claimant (six items, `docs/tickets/README.md`).
