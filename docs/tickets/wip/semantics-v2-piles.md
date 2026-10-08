---
needs: []
---
**One ordered pile shape over libraries, graveyards and minted piles.** Ruling
2026-10-08 ("A pile is one ordered shape", `docs/decisions/semantics-v2.md`
§7). It replaces `NounPhrase.libraryRun` and the library-only operations.
Standard constraints apply.

## Shape

- `Pile` noun: `library (whose)` (face down [CR#401.2]), `graveyard (whose)`
  (face up, still ordered [CR#404.2,700.3c]), `that` ("that pile"), and a
  face-down exile pile [CR#406.4]. A pile is not an object [CR#700.3b].
  The pile carries its `PileFace` (`faceDown` hidden / `faceUp` public). The
  two zones fix their face. A pile an instruction makes (exile face down,
  `separateIntoPiles`) takes the face its instruction gives it. Visibility
  reads the face, never the zone.
- Operations:
  - `slice (end_ : LibraryEnd) (amount : Amount) (pile : Pile)`, an ordered
    object group;
  - `run (end_) (stop : Predicate) (pile)`, from an end until the first
    object matching `stop`, which is included;
  - `place (end_) (pile)`, a move destination;
  - `shuffle (pile)`. A bare "shuffle" is `shuffle(library(actor))`
    [CR#701.24a].
- Checker: references and kinds only. A slice or run introduces an ordered
  group binding; "that pile" resolves to a pile binding.
- Rust mirror, RON helpers, and bench macros aligned to the RON spelling.

## Retires

`NounPhrase.librarySlice`, `NounPhrase.libraryRun`, `ZoneExpr.library`,
`LibraryPlace`, the operand-less `Instruction.shuffle`, and the library
position helpers under `plugins_v2/builtin/macros/` (`librarySlice`,
`libraryRun`, `topSlice`, `top`, `bottomCard`, `onTop`, `onTopIn`,
`onBottom`, `onBottomIn`, `nthFromTop`, `nthFromTopOrBottom`,
`choiceOfTopOrBottom`, `library`, `libraryOf`). Each is replaced by a helper
over the new operations, or deleted when the general form says the same.
Name every deletion in the landing record.

## Re-spells (same asserted outcome)

The bodies of mill, scry, surveil, fateseal, explore, clash, discover,
manifestDread, lookAndSortSlice, shuffleInto and shuffle. The canon cards
reading a library (Opt, Consider, Preordain, Demonic Tutor, Mental Note,
Trumpeting Carnosaur, Threats Around Every Corner, and any others the build
names). Pins: `lean/Semantics/Proofs/Piles.lean`, and in
`KeywordActionResidues.lean` `okDiscover` and `badLibraryRunOfAnObject`
(re-spelled as a run over a non-pile).

## Bodies

cascade [CR#702.85a] (exile a run from your library, then "cards exiled this
way that weren't cast" to the bottom at random) and ripple [CR#702.60a].
Each gets its body if the current anaphora can write "exiled this way";
otherwise it is STOP-noted to `lean-drt-anaphora-refactor` with the refusal.

## Witnesses

Opt (scry), Mental Note (mill), Trumpeting Carnosaur (discover run), and a
graveyard-order pin: a slice of a graveyard keeps its order [CR#700.3c].
Glossary: Pile is already defined in `docs/contexts/game-model/CONTEXT.md`.
Amend it if the build shows a flaw.

## Deltas

Runs in parallel with cost-substitutes and temporal-suite. All three touch
`Words`/`Phrase`/`Abilities`. Refresh before integrate.
