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

2026-09-03 cleanroom review (F16) re-measured it: 16,013 lines, `Cards.ttc`
44.7 MB against `Effect.ttc`'s 2.0 MB, 14 s to re-check after any edit (the
whole core plus bench is ≈55 s), and only 30 intra-file reference edges —
nothing outside imports `Experimental.Cards`. A six-way *contiguous* cut at
existing definition starts (`rakshasaVizier` @2674, `extinction` @5340,
`sickeningShoal` @8002, `ertaisTrickery` @10673, `melekIzzetParagon` @13350)
gives six 2660–2680-line modules with 3 crossing edges — 36 lines to move
(`autarchMammothLine`, `veilingOddityLine`, `kitsuneMysticFlip` down beside
their users) — with `Cards.idr` kept as an `import public` shim for the
`.ipkg` target; a by-family reorder is the larger, separate pass. This ticket
runs **last**, after every other ticket in the 2026-09-03 review batch: each
of those re-spells bench witnesses, and re-spelling across a just-split file
is needless conflict.
