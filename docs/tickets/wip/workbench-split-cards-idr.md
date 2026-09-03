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

## As landed

`Experimental/Cards.idr` (16,016 lines, 1,578 top-level defs) became fifteen
`Experimental/Cards/<Family>.idr` modules plus an `import public` shim at the
old path. Family assignment is by content signal over each definition's body
with the printed header (name, mana cost, type line) masked out, resolved in a
fixed priority order; the module order below is also the import order — a
family module imports only families above it, so the graph is a DAG and no
module imports a later one.

| module | lines | defs | residue tail | imports |
| --- | ---: | ---: | ---: | --- |
| `Experimental.Cards.Description` | 566 | 81 | 12 | — |
| `Experimental.Cards.Anaphora` | 814 | 115 | 15 | Description |
| `Experimental.Cards.Trigger` | 1183 | 127 | 4 | Description, Anaphora |
| `Experimental.Cards.Damage` | 1663 | 168 | 12 | Description, Anaphora, Trigger |
| `Experimental.Cards.Keyword` | 1929 | 176 | 7 | Trigger |
| `Experimental.Cards.Counters` | 1550 | 146 | 9 | Anaphora, Keyword |
| `Experimental.Cards.Mana` | 1427 | 114 | 2 | Anaphora, Keyword |
| `Experimental.Cards.Deontic` | 930 | 96 | 1 | Description |
| `Experimental.Cards.Choice` | 1614 | 165 | 10 | Anaphora |
| `Experimental.Cards.Static` | 1174 | 132 | 9 | Choice |
| `Experimental.Cards.Cost` | 1372 | 124 | 7 | Description, Keyword, Static |
| `Experimental.Cards.Faces` | 670 | 49 | 0 | — |
| `Experimental.Cards.Turn` | 552 | 52 | 3 | Anaphora, Keyword, Faces |
| `Experimental.Cards.Copy` | 256 | 18 | 0 | — |
| `Experimental.Cards.Piles` | 243 | 15 | 1 | — |

Largest module 1,929 lines (`Keyword`); total 1,578 defs, unchanged. Every
definition moved verbatim — the whole-corpus body comparison against the
pre-round file reports 0 bodies present only before and 0 only after, once the
`Cards.`-qualifier rewrite below is normalised away.

The shim keeps `Experimental.Cards` a real module: it is `import public` over
all fifteen family modules and nothing else, so the `.ipkg` target and any
external reference resolve as before. Nothing in `src/` imports it today
(`src/Bridge.idr`'s `Cards.` references are the unrelated top-level `Cards`
module, and Bridge is parked out of the build).

**Residue trailer.** Each module ends with the witnesses whose bodies carry no
family signal at all; they follow the family's own witnesses, in original file
order, and are placed by their nearest preceding classified neighbour. The
trailer is positional, with no section comment, per the round's near-zero-comment
rule. Seven residue witnesses that other definitions reference keep their
in-order position instead, so declaration-before-use holds.

**Qualifier rewrite.** 79 sites inside the bench referred to sibling
definitions as `Cards.<name>`. `Cards.` is no longer a namespace suffix of
`Experimental.Cards.<Family>`, so each was rewritten to `<Family>.<name>` —
the only textual change to any moved line. No line carrying a CR citation is
affected: `Experimental/Cards.idr` had none.

**Proofs renamed by family.** The seven size-split files plus `ProofsAnaphora`
were re-partitioned into fourteen `Experimental.Proofs<Family>` modules over
the bench vocabulary plus `Zone`. A rename alone could not satisfy the
Done-when: measured per definition, no letter file had a dominant family (the
best was `ProofsF` at 36/113 damage), so the names would have been arbitrary.
Twin-beside-pin adjacency is preserved by moving whole groups — a group is a
maximal run of non-pin definitions followed by the pins they cover, cut at each
pin→non-pin boundary — so no pin is separated from its twin.

| module | lines | defs | residue tail |
| --- | ---: | ---: | ---: |
| `Experimental.ProofsAnaphora` | 1952 | 283 | 8 |
| `Experimental.ProofsDescription` | 493 | 77 | 4 |
| `Experimental.ProofsZone` | 785 | 124 | 12 |
| `Experimental.ProofsDamage` | 696 | 100 | 11 |
| `Experimental.ProofsTrigger` | 277 | 38 | 3 |
| `Experimental.ProofsStatic` | 237 | 34 | 2 |
| `Experimental.ProofsCounters` | 487 | 68 | 7 |
| `Experimental.ProofsMana` | 445 | 65 | 6 |
| `Experimental.ProofsKeyword` | 369 | 46 | 6 |
| `Experimental.ProofsDeontic` | 405 | 50 | 0 |
| `Experimental.ProofsChoice` | 270 | 40 | 2 |
| `Experimental.ProofsTurn` | 400 | 59 | 2 |
| `Experimental.ProofsFaces` | 777 | 107 | 7 |
| `Experimental.ProofsPiles` | 187 | 25 | 0 |

Where the old files went (defs):

| old | → new |
| --- | --- |
| `Proofs` (115) | Zone 32, Damage 23, Choice 14, Turn 14, Description 11, Counters 10, Mana 5, Trigger 4, Faces 2 |
| `ProofsB` (97) | Zone 23, Damage 22, Description 18, Counters 17, Deontic 6, Choice 6, Anaphora 5 |
| `ProofsC` (110) | Zone 32, Deontic 17, Turn 16, Mana 12, Counters 7, Damage 7, Anaphora 6, Keyword 5, Description 4, Choice 2, Trigger 2 |
| `ProofsD` (107) | Faces 19, Deontic 14, Zone 13, Counters 11, Anaphora 9, Static 9, Turn 8, Description 8, Keyword 7, Damage 5, Mana 2, Trigger 2 |
| `ProofsE` (111) | Faces 26, Description 19, Keyword 11, Static 11, Mana 10, Zone 9, Trigger 8, Counters 8, Turn 3, Anaphora 2, Deontic 2, Choice 2 |
| `ProofsF` (113) | Damage 36, Choice 13, Zone 12, Keyword 11, Mana 11, Faces 11, Turn 6, Static 4, Deontic 3, Trigger 3, Piles 3 |
| `ProofsG` (290) | Anaphora 97, Faces 49, Mana 25, Piles 22, Trigger 19, Description 18, Counters 15, Keyword 10, Static 10, Turn 10, Damage 7, Deontic 5, Choice 3 |
| `ProofsAnaphora` (174) | Anaphora 164, Zone 3, Deontic 3, Turn 2, Keyword 2 |

No pin module imports another — the fourteen import lists are identical
(`Experimental`, `Experimental.Macros`, `Experimental.Unspellable`, plus
`Data.List.Elem` where a name from it is used), as before the split. Ten
`ProofsG.<name>` self-qualified references were rewritten to their new module.

**Not done.** Nothing in the ticket's letter is outstanding. The build-time
share the ticket attributes to `Experimental.Phrase` is untouched, as is
`Experimental/Effect.idr` (a sibling round holds it).

## Landing record

**Numbers before/after.** `Experimental/Cards.idr`: 16,016 lines / 1,578
top-level defs → 15 modules totalling 15,943 lines and 1,578 defs, plus a
19-line shim; largest module 1,929 lines. Pin corpus: 8 modules / 8,457 lines /
1,117 defs → 14 modules / 7,780 lines / 1,116 defs, of which 594 `Unspellable`
pins before and 594 after. `mtg.ipkg` module count 23 → 44; `mtg-dev.ipkg`
15 → 30. No coverage lock, construction count, or Rust artifact is in scope.

**Gates** (foreground, from `idris/` and the workspace root):

- `./scripts/build` (full gate, `mtg.ipkg`, clean `build/`): exit 0,
  `44/44: Building Cards (src/Cards.idr)`, 0 `Error` and 0 `Warning` lines.
  **44.0 s** clean, against **43.8 s** clean for the pre-round tree built the
  same way from a pristine copy — the split is time-neutral. Every
  `Unspellable` pin still refutes: a pin that stopped refuting fails the build
  with `<name> Oh impossible` not being impossible, and the build is green.
- `idris2 --build mtg-dev.ipkg` (inner loop, no `Proofs*`): 22.6 s.
- `cargo xtask cite check --list-noncompliant`:
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check`:
  `checked 17798 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/sc.diff && cargo xtask cite audit --diff < /tmp/sc.diff`:
  `audited 8 citation site(s) — read each rule text against its claim`. All 8
  are moved docstring lines in the re-partitioned pin modules; each added line
  is byte-identical to a line in the pre-round tree and each removed line is
  byte-identical to a line in the new tree (checked mechanically, 0 mismatches
  either way). No citation was added, deleted, or edited.

**Assurance counts.** restored 0; re-spelled 0; ignored 0; added 0; removed 1;
moved 2,694 (1,578 bench witnesses + 1,116 pin-corpus definitions). The one
removal is `okEachOfTargetGroup`, which the size-split had defined
byte-identically in both `ProofsC` and `ProofsE`
(`EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature)`); the split
brings both into `ProofsDescription`, where two copies of one name do not
compile. The surviving copy is in the same module as both pin groups it covers,
which is what the pin-hygiene rule asks ("one twin covers the pins that share
its obligation"). Neither copy was referenced by anything. No assertion, pin, or
witness lost coverage: the pin count is 594 before and after.

**Deviations and additions.**

1. *Proofs re-partitioned, not just renamed.* The ticket says "rename … by the
   family each refutes". Measured, no letter file has a family to be named
   after (best 36/113), so the names had to be earned by moving the pins. The
   move is by whole twin+pin group and the pin count is unchanged.
2. *Three pin definitions renamed.* `okDamageCreature` (in `ProofsB`, one
   damage) → `okDamageOneToCreature`; `okCompareCountSubject` (in `ProofsG`,
   creature count) → `okCompareCreatureCountSubject`; `badFortifiedCreature`
   (in `ProofsE`, the `Noun` form) → `badFortifiedCreatureNoun`. Each name was
   defined twice across the old size-split files with *different* bodies, and
   the family split brings each pair into one module. In every case the earlier
   file keeps the original name, the later gets a name describing its own term,
   and neither was referenced anywhere. No term, type, docstring, or pin body
   changed.
3. *One deletion*, `okEachOfTargetGroup` — see assurance counts above.
4. *`Cards.<name>` → `<Family>.<name>`* at 79 sites, and
   `ProofsG.<name>` → `Proofs<Family>.<name>` at 10 sites: forced, since the
   old qualifier is no longer a namespace suffix of the new module. These are
   the only edits to any moved line.
5. *Sixteen bench definitions were placed in their user's module rather than
   their own family's*, to keep the import graph a DAG and to avoid making a
   private definition `public export`: `bioplasmAfterExile`,
   `bioplasmCardTest`, `castCreatureSpellsFromTop`, `companyContext`,
   `companyDescribedSlice`, `counterspell`, `engulfingFlamesRider`,
   `forkedBolt`, `hymnOfRebirth`, `masterChefGrantedAbility`, `moonlitWake`,
   `nekrataalRider`, `phyrexianIngesterPump`, `pirDistributive`,
   `tourachDiscardTrigger`, `veilingOddityLine`. No visibility modifier was
   changed anywhere.
6. *`import public Experimental.Unspellable` → `import`* in the one pin module
   that had it (old `Proofs.idr`). Nothing imports a pin module, so the
   re-export was a no-op; all fourteen now carry identical import lists.
7. *Residue trailers are positional*, with no section comment, per the round's
   near-zero-comment rule.
8. `idris/VERIFY.md` gained one paragraph naming the two new module families
   and their import discipline.

**STOP taken.** None.
