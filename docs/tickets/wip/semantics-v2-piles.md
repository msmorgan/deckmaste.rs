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

## Landing record

Series on the claim `tzmpmmkz`, oldest first: `nryrprzp` (Pile node,
slice/run/place/shuffle, checker cases), `orqrvnuw` (Rust mirror, move riders,
`Pile` kind and renderer), `tvlyvzns` (RON helpers, Lean bench macros, bench
re-spelled), `yqnrpwtq` (keyword bodies, cascade body, ripple STOP, pile pins),
`utxkqwto` (canon), `wqzwnksv` (glossary, this record). Numbers measured on
`utxkqwto` after the second `kata refresh` (onto `semantics-v2-cost-substitutes`;
one conflict in `Phrase.lean`'s deriving line, both sides kept). `lean/` changed.

1. **Census delta**: not run. No english_v3 input changed: grammar, lexicon
   and keyword spellings are byte-identical; only macro bodies and helper
   macros changed. Lost faces: none.
2. **Witnesses**: `cargo xtask lean check`: canon 155/155, testing 8/8, so
   Opt (scry 1), Mental Note (mill 2) and Trumpeting Carnosaur (discover 5,
   a run over `library(actor)`) prove `Card.check = []`. Lean pins:
   `okDiscover` and `badLibraryRunOfAnObject` (same outcome,
   `[kindMismatch player object]`, now a run over a creature's "library",
   which is no pile); graveyard order: `okSliceOfAGraveyard` and
   `sliceOfAGraveyardStaysInIt` (`Proofs/Piles.lean`, the slice binds in the
   graveyard [CR#700.3c]). `lean definitions` 553/553.
3. **Spot checks** (bodies, not faces): scry/fateseal sort a `slice(Top, N,
   library(…))` onto `onBottom` and `onTop` with `InOrder(AnyOrder)`; surveil
   spills to the graveyard with no rider; mill moves `slice(Top, N,
   library(actor))` from `zone(Library, Bare)`; discover runs `library(actor)`
   and puts the rest on the bottom `InOrder(RandomOrder)`; explore and clash
   reveal `slice(Top, 1, library(actor))`; manifest dread looks at the top two;
   shuffle is `Shuffle(library(actor))`; shuffleInto is the enacted move into
   `zone(Library, Bare)` then `Shuffle(Library(Bare))` [CR#701.24c,701.24d];
   cascade exiles a run and puts `allFromAmong(not(wasCast), theVerbed(exile,
   Many))` on the bottom at random (`okCascade`); Galvanoth, Soul Summons and
   Ransom Note read `slice(Top, 1, library(you))`; Spin into Myth keeps
   `onTop`.
4. **Deviations and STOPs**:
   - Nodes (Lean, mirrored): `Pile = library (whose : ZoneScope) | graveyard
     (whose : ZoneScope) | that | exiledWith (source)`; `NounPhrase.slice`,
     `NounPhrase.run`, `ZoneExpr.place (end_) (pile)`, `Instruction.shuffle
     (pile)`. `whose` is a `ZoneScope`, not a noun phrase, so a bare "its
     owner's library" destination stays bare [CR#400.3] as `LibraryPlace`'s
     scope was.
   - `LibraryPlace`'s other content moved to three move riders, since `place`
     is `(end) (pile)` by the ruling: `TokenRider.inOrder (arrangement)`,
     `nthFromEnd (ordinal)` (keeps `ordinalNonZero`), `orOtherEnd`. The
     either-end chooser is dropped: the actor chooses, as the 2026-10-06
     ruling "`choose` has no chooser" has it; Aether Gust and Not Forgotten
     named the actor anyway. `shuffled` became the shuffleInto sequence, so
     `ZoneExpr.shuffles`/`afterMoveTo` (always false) are deleted.
   - Retired RON helpers: `librarySlice`, `libraryRun`, `library` (ZoneExpr),
     `libraryOf`, `onTopIn`, `onBottomIn`, `topOrBottom`,
     `choiceOfTopOrBottom`, `nthFromTop`, `nthFromTopOrBottom`; same in Lean
     (`library`, `libraryOf`, `onBottomIn`, `onTopIn`, `topOrBottom`,
     `choiceOfTopOrBottom`, `nthFromTopOrBottom`, `nthFromTop`). Re-bodied over
     the new nodes, names kept: `topSlice`, `bottomCard`, `onTop`, `onBottom`,
     `yourLibrary`. Kept untouched: `top`, which is `Window.top` (pronoun
     scope), not a library position; the ticket's list names it in error.
   - New helpers: RON `slice`, `run`, `place`, `piles/library(player)`; `move`
     gains a defaulted `riders`; `lookAndSortSlice` gains `spillRiders`. Lean
     `library (player) : Pile`, `putWith` (bench cards cannot write a raw
     move with riders), `cascadeExpansion`, `cascade`.
   - Checker: references and kinds only. `Pile.checkIn` checks the possessor
     (player, `slicePossessor`), "that pile" as a singular pile anaphor, the
     exile pile's source. Dead code deleted: `NounPhrase.libraryOwner`,
     `ZoneExpr.arrangement`, `ZoneExpr.ordinal`. `Binding.isPublic` (also
     unreferenced) reads a pile's face before its zone.
   - Rust: `Pile` registered as a macro kind and param type; xtask
     `expansions` gains a `Pile` renderer and sample, and its `Shuffle` sample
     became `Shuffle(pile: That)`.
   - STOP, ripple [CR#702.60a]: stays bodyless. "Revealed cards" as
     `theVerbed(reveal, Many)` is refused `[anaphor (verbed (action "reveal")
     none attributive) many 0]` (pin `badRippleRevealedCards`); "those cards"
     `[anaphor (word card) many 2]`; the slice re-mentioned by identity checks
     but breaks the 2026-10-08 moved-referent ruling. Routed to
     `docs/tickets/planned/lean-drt-anaphora-refactor.md`. Cascade got its body.
   - Glossary: Slice and Run added beside Pile
     (`docs/contexts/game-model/CONTEXT.md`).
5. **Tests**: `lake build --wfail` (89 jobs) ok; `cargo test -p
   deckmaste_semantics_v2` ok (lib 45, corpus 7, 1, 4, 43);
   `cargo test -p xtask` ok (382 + 1 pre-existing ignore, 14, 4);
   `cargo test -p deckmaste_construction_core` ok; `cargo test -p
   deckmaste_lexical_source` ok (24, 35); `cargo clippy -p <each> --tests --
   -D warnings` clean for all four; `cargo xtask facts check` up to date;
   `cite check` 0 stale, `--list-noncompliant` empty, `cite bless` registered
   701.24c and 701.24d. Restored 0; re-spelled: 22 Lean theorems directly and
   4 shared defs (`exploreBody`, `lookAndSortSlice`, `scryBy`, `libraryCards`)
   plus 30 bench card terms whose theorems are unchanged; Rust 4 corpus
   fixtures and 1 xtask assertion (`shuffle`'s body text). Ignored 0. Added: 8
   Lean pins in `Proofs/Piles.lean`, 4 corpus fixtures. Removed 0.
6. **Timing**: no census run; `cargo xtask lean check` 144.7 s.
