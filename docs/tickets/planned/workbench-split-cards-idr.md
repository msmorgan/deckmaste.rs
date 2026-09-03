---
needs: []
---
**Split `Experimental/Cards.idr` into per-family modules and rename the
size-split proof files by family.** 2026-09-02 workbench audit (F14). Distinct
from `idris-retire-cards-idr`, which retires the unrelated 1,271-line
`idris/src/Cards.idr` bridge corpus.

`Experimental/Cards.idr` is 16,127 lines and 1,532 defs (781 `Card`, 303
`Ability`, 300 `Effect []`), with zero section comments and no ordering
(metrics report Part A.5); it costs 21.3 s per elaboration, so every bench edit
re-elaborates the whole file. It is a maintenance problem more than a
compile-time one — the clean build is 65 s and `Experimental.Phrase` (one
3.6k-line `mutual` block, 140-clause `predEq`) is the larger share at 29.6 s.

Fix: `Experimental/Cards/<Family>.idr`, 10-15 modules split by grammar family
matching the ticket families, each importing `Experimental.Macros`, all listed
in `mtg.ipkg`; one residue trailer per module for the witnesses that do not fit
a family.

`Proofs.idr … ProofsG.idr` are size-split, not subject-split: identical import
lists, no module imports another, the zone family is refuted in six of the
seven files and plurality in five, and `ProofsG` (1,430 lines) holds five
unrelated subjects (proofs catalog). Rename them by family in the same pass so
a pin can be found from the constructor it refutes.

Size: S.

Done when: the full build is green at its new module count with 0 errors and 0
warnings; no `Experimental/Cards/` module exceeds ~2k lines; `mtg.ipkg` and
`mtg-dev.ipkg` list every new module; every bench witness that was in
`Experimental/Cards.idr` is present in exactly one new module (def count
unchanged at 1,532); each `Proofs*` module is named for the family it refutes
and every pin still refutes. Standard constraints apply.
