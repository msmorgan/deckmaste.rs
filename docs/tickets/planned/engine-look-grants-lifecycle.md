---
needs: []
---
**Engine: look grants end when the looked-at cards are shuffled or reordered,
reveals last only as long as [CR#701.20a] allows, and the face-down-exile look
permission of [CR#406.3] becomes its own ledger.** Latent: nothing reads
`look_grants` as a permission yet (only tests: `resolve/action.rs:5857`,
`tests/payment_replay.rs:2784-2785`), but `runner-player-view` is built on it.

Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-engine.md` #15,
#30, #37.

## What is wrong at head (crates/deckmaste_engine/src)

- **Shuffle does not end grants (#15).** `state.rs:473-477`: grants are "keyed by
  object IDENTITY (`ObjectId`), which is freshly minted on every zone change
  ([CR#400.7]) — so grants expire for free on shuffle". A shuffle is not a zone
  change: `step/zone.rs:90-101` `handle_shuffled` only permutes the deque ("Revealed-
  state reset ([CR#701.20d]) is unbuilt"), and nothing removes a grant (inserted only
  at `payment.rs:1016` and `payment/replay.rs:1537`). A scry grant follows the card
  to its random new position. `resolve/effect.rs:820-823` ("expires free on the next
  remint/shuffle") repeats the claim. [CR#701.24a]: shuffling randomizes "so that no
  player knows their order".
- **Reordering keeps the id (#30).** `agenda.rs:311-322` `RepositionLibrary` and
  `step/mod.rs:1718-1749` `reposition_library` keep the `ObjectId` and the grant when
  scry, surveil or fateseal puts cards back. [CR#701.20e]: "Looking at a card follows
  the same rules as revealing a card"; [CR#701.20d]: "any revealed cards that are
  reordered stop being revealed and become new objects." (No ruling addresses this;
  it is the CR chain.) The test `scry_reposition_keeps_id_and_fires_no_zone_change`
  (`resolve/action.rs:5831` ff.) pins the kept id; rewrite it to assert a new id and
  still no `ZoneChange`. These sites also cite [CR#401.7] (the too-few-cards rule):
  `agenda.rs:311-322`, `step/mod.rs:179`, `:1718-1725`, the test doc. Drop it there;
  keep the correct one at `step/mod.rs:652`.
- **Reveals are permanent (#37).** `state.rs:473-477` calls the ledger "[CR#406.3]
  generalized … Persistent"; `GameEvent::Revealed` (`step/mod.rs:454-470`) grants
  every viewer a permanent entry, so a card revealed from hand for a cost stays
  "granted" to opponents while it stays in hand. [CR#701.20a]: a revealed card
  "remains revealed for as long as necessary to complete the parts of the effect
  that card is relevant to". [CR#406.3]'s persistence is only for a card exiled
  face down ("may continue to look at that card until it leaves the exile zone or
  is part of a pile of cards that are shuffled"), and must survive the [CR#400.7]
  remint at the exile, which an `ObjectId`-keyed, look-time ledger drops.

## Change

1. On `Shuffled(player)`, remint every card in that library (which also gives the
   [CR#701.20d] reset) or at least drop every grant on them. Do the same for
   face-down exile piles when they land.
2. When an effect reorders looked-at or revealed library cards (scry, surveil's top
   pile, fateseal, random-bottom remainders), remint them in place, with no zone
   change and no `ZoneChange` event; their grants end with the old ids. Record the
   put-back-in-the-same-position case in the UD register (`docs/engine-adrs.md`),
   since the CR chain is interpretive.
3. Cite peeks and reveals to [CR#701.20e,701.20a], end reveal grants when the
   revealing effect finishes, and document the remaining entries as the engine's
   knowledge model, not a [CR#406.3] permission. Correct the `state.rs` and
   `resolve/effect.rs` comments.
4. When face-down exile lands (`engine-face-down`), keep a separate [CR#406.3] look
   ledger written as part of the exile and keyed on the exiled object, ending when
   it leaves exile or its pile is shuffled; back it with live predicates for
   hideaway (the controller of the permanent that exiled it, [CR#702.75a]) and
   foretell ([CR#702.143a]). Gate [CR#601.3f] and [CR#406.4] on that ledger only.

## Tests

- Scry 1 keeps the card on top: the scrying player's grant is gone after a shuffle.
- A scried card put back gets a new id and fires no `ZoneChange`.
- A card revealed from hand for a cost is not visible to the opponent afterwards.
