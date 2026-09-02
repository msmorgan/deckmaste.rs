# workbench-pile-contents-and-faces

The pile round's two big residues (close 2026-09-02; full residue list in
Cards.idr under "THE PILE PARTITION'S RESIDUES"):

- **The pile-CONTENTS predicate** — 13 lines / 12 cards, the family's single
  largest blocker (six more whole cards behind it). [CR#700.3b] leaves a pile
  containing nothing (not an object) and [CR#700.3c] blocks `InZone`, so
  membership needs its own read against the `PileP` payload.
- **The FACE-DOWN pile** — 17 lines / 16 cards; `FaceDown` is
  `OnBattlefield`-gated and these mark exile/library piles. Also carries the
  exile idiom's shuffle ([CR#701.24a] shuffles "a library or a face-down
  pile" in one sentence — the grammar's shuffle is library-typed; that gap is
  ours, not the rules').

Re-measure at claim.
