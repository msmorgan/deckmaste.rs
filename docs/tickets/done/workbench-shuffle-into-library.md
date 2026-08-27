---
needs: []
---
# The shuffle-into-library move

Routed from `workbench-event-zone-3-targeting-and-disjunction-arms` (close,
2026-08-26). "Shuffle [card] into its owner's library" is a move whose
destination takes a randomizing arrangement [CR#701.24a], and no move verb
spells it — `Move`'s library destinations are positional (`LibraryAt`), and
`Shuffle` is the bare whole-library act. Fblthp, the Lost's second ability
is the found carrier; MEASURE the "shuffle … into … library" surface before
shaping (it is a common printed verb — expect a large family; decide slot vs
row from the measured shapes).

## Consumption boundary

`idris/src/Experimental/Effect.idr` (or `Phrase.idr` if the destination
arrangement is the seat), `Macros.idr`, `Cards.idr` bench, `Proofs*.idr`.
No Rust crate.

## Acceptance

- Fblthp, the Lost benches whole; the verb is closed by design, not
  per-card.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed

### The measured surface

229 "shuffle … into … library" sentences over 220 cards in
`data/derived/cards.jsonl`. By what is shuffled: a single card, named or
anaphoric (`it` 47, `this card` 13, CARDNAME 10, `that card` 3, `them` 8,
`the rest` 13 + 7 reveal-until variants) ≈ 100; a whole zone read as a
mass (`their/your graveyard` 24, `their/your hand and graveyard` 21,
`the cards from their/your hand` 7) ≈ 55; a described or targeted set
from a zone (`up to N target cards from their graveyard` 13, `any number
of target cards …` 3, `all creature cards from your graveyard`, `all
nonland cards …`, …) ≈ 30. By destination: `their library` 87, `your
library` 73, `its owner's library` 53, plurals and third-party
possessives ≈ 12. By subject: bare imperative ≈ 70/`then` 38, `each
player` 25, `target player` 16, `its owner`/`this creature's owner`/`the
owner of target …` ≈ 17.

Two conclusions the shape follows from: the object slot is the whole of
`Move`'s existing noun surface (nothing shuffle-specific about it), and
roughly half the family writes an explicit subject.

### The shape: a `LibPlace`, not an `Arrangement` and not an `Effect` row

`LibPlace` gains `Shuffled` — "where in a library the card lands" answered
with *no position*, because the act that puts it there randomizes the pile
[CR#701.24a,701.24c]. `Move` composes over it unchanged, so every noun,
rider and enclosure the move already had comes free; a dedicated row would
have had to re-derive them. It is NOT `Arrangement`: an arrangement says in
what order several cards enter one position [CR#401.4], and `RandomOrder`
(bottom in a random order, [MTR 3.10]) stays the separate construct the
`scry-arrangement-design` note settled.

Two gates ride the new place, both rule-backed refusals:
`placeArrangementOk Shuffled (Just _) = False` ([CR#401.4] arranges cards
put "in a specific position"; a shuffle puts them in none) and a new
`placeOrdinalOk`/`PlaceOrdinalFits` gate refusing an offset ([CR#401.7]
counts down from the top card, and a shuffle names no position to count
from). `LibraryAt` takes the second proof as `nf`, and `DestOk`'s
`LibraryPosOk` covers the new place with no new row.

The verb is closed in both voices at the macro layer: `shuffleInto`
(`Enact "Shuffle"`, the imperative) and `shufflesInto` (`Does agent
"Shuffle"`, the agentive), on `destroy`/`puts`' model. `verbFacts` gains a
`"Shuffle"` row — participle, patient and destination all absent
([CR#701.24a] shuffles a library, and a "cards shuffled into your library
this way" marking would have to name the destination, which is the
documented reason `"Put"` has none). The label also makes the shuffle
EVENT expressible, which [CR#701.24b,701.24e,701.24f] all contemplate.

### The shuffle stamp reaches it

`effIntro (Move …)` — and the `Enact`/`Does` move clauses beside it — now
run the destination through `afterMoveTo`, which applies `afterShuffle`
when `zoneShuffles` holds. Verified by evaluation, not by inspection: the
discourse after "Look at the top card of your library" holds 1 binding,
after `Shuffle` 0, and after `shuffleInto This` 0. The moved card's own
mention goes with the rest — [CR#701.24a] leaves no player knowing where
it went. `preIntro`/`annIntro` are unchanged: they read the discourse
BEFORE the effect.

### Witnesses benched

- `fblthpTargeted : Ability` — Fblthp, the Lost's second ability whole,
  replacing the header-only `fblthpTargetedHeader`.
- `loamingShaman : Card` — whole card; [CR#701.24d]'s own example, the
  agentive voice over a targeted set from a graveyard.
- `blessedRespiteShuffle : Effect []` — the mass-zone form, [CR#400.12].

New pins in `ProofsG`: `badShuffledArranged` (a shuffle takes no
arrangement), `badShuffledOrdinal` (a shuffle takes no offset), and
`badReadsShuffledIntoLibraryCard` (the stamp reaches the move; the
destination alone says so, with no `Shuffle` clause written).

### Fblthp's first ability: named blockers

"When Fblthp enters, draw a card. If it entered from your library or was
cast from your library, draw two cards instead." does NOT write, and the
blocker is the entry event's origin, not the shuffle:

- `Enters` (`Triggers.idr`) carries no source-zone slot, so the header
  introduces no origin for "it entered from your library" to read.
- `HappenedTo Entry` cannot supply one either: `lookbackOriginOk` admits
  an origin for `SpellCast` alone, and `lookbackComplementOk Entry _ _`
  is closed, so the `FromZones` complement is refused for `Entry`.
- The other disjunct writes today (`CastFrom Macros.yourLibrary`;
  `playableFrom (Just Library) = True`), as does `InsteadOf`. It is the
  coordinate that has no seat.

### Tolerated overgeneration, named

`PutInto n shuffledIntoZ` is now spellable — "whenever [n] is put into a
library shuffled". Rules-meaningful ([CR#701.24c] does move the objects
there) and unprinted; left ungated.

### Follow-ups (not routed)

- The coordinated mass object, "shuffles their hand and graveyard into
  their library" (21 sentences). `Both (AllOf (InZone …)) (AllOf (InZone
  …))` may already write it; unverified, unbenched.
- An entry-origin seat for `Enters`/`Entry`, which is what Fblthp's first
  ability and the "entered from" family need. Its own ticket.
