---
needs: [english-shape-rarity]
---
**Calibrate the rare-production tail against a corpus with known defects.**
`english-shape-rarity` shipped uncalibrated, by circumstance rather than
choice: the plan was to validate the miner by checking its tail contained the
two Bonfire productions, and `english-ast-grouping` fixed that family before
the miner landed, so the labelled pre-fix snapshot no longer reproduces from
trunk.

Without calibration the tail's *ranking* is unvalidated — we know it surfaces
real oddities (a `ComplexClause{matrix: [Subject, Modal, None]}` singleton led
to Lim-Dûl's Vault, whose modal is swallowed by an adverbial), but not that
rare-and-wrong outranks rare-and-fine.

Candidate approaches:

- Reconstruct a labelled snapshot by running the miner at a pre-fix revision
  and checking the then-live Bonfire productions land in the tail.
- Use the families now ticketed from `english-lint-*` as ground truth: their
  exemplars are known-defective faces, so their productions should rank in the
  tail. This needs no history surgery and is probably the cheaper route.
- Report tail precision honestly once measured, and tune `--lexicalize` /
  `--depth` / `--arity` against it rather than by eye.

Until then, treat the tail strictly as a worklist. Standard constraints apply.
