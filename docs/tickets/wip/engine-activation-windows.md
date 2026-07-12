---
needs: []
---
Close the P0.W1 activation-window seam (`activate.rs` `todo!("P0.W1:
activation window …")`): the [CR#602.5d..602.5e] "Activate only …" gate
evaluates `Timing::InstantSpeed`/`SorcerySpeed` but trips `todo!` on
`DuringTurn(WhoseTurn)` ("Activate only during your turn") and
`DuringStep(PhaseStep, WhoseTurn)` (forecast-style "Activate only during the
upkeep step of the card's owner", [CR#702.57b]).

Both arms are pure predicates over existing state: `turn.active_player`
against the controller relation (the `WhoseTurn` eval precedent in `eval.rs`)
and `turn.current` against the named step. No new machinery, decisions, or
Idris changes.
