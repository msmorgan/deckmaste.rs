---
needs: []
---
`idris/src/EmitTables.idr` carries a "DRIFT CONTROL" banner claiming every
`entailments.ron` row value is computed from `Semantics.idr` model functions, but
the Exile, Mill, Fight, and Explore rows hand-write `MkEventCaps` literals
instead of reading `eventKindCaps`. Those four rows can silently disagree with
the model (the committed `entailments.ron` matches today — checked 2026-07-16).
Derive them from the model like the other rows, or scope the banner's
guarantee honestly.
