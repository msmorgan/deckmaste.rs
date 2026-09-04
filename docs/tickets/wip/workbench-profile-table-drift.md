---
needs: []
---
**Make `Effect.instrProfile` the one instruction context table and derive
every plural-agent read from it.** Cleanroom review 3, 2026-09-04, findings U1
and G2, under the ruling on simultaneous moves (2026-09-04).

- **U1 — the announced context is the pre-move stack.** `Effect.instrProfile`'s
  `Move` row is `sameIntro (nomIntro what) []` and its
  `Enact (Just s) v (Move …)` row builds `doesAnnIntro` over the same, while
  `Effect.instrIntro`'s `Move` row is
  `afterMoveTo to (moveIntro Nothing what …)`. `Effect.SimInstructions.(::)`,
  `replacedCtx`, `otherwiseCtx`, `HeldUntil` and `Unless` all consume
  `annIntro`, so `Sequentially [Simultaneously [exile You (target creature)],
  tap (It OneOf)]` is admitted while the flat `Sequentially` form is refused:
  an exiled object is in the exile zone [CR#701.13a] and only untapped
  permanents can be tapped [CR#701.26a]. Ruling: after
  `Simultaneously [Move …]` the moved object is readable at its destination,
  so `announced` is the post-move stack.
- **Same drift, two more symptoms.** `instrProfile`'s `Search` deed row builds
  the found-card binding with `Nothing` for the stamp while `instrIntro`
  stamps it `"Search"`, so `InsteadOf (searchLibraryFor (exactly 1) creature)
  revealsIt` is refused where the `Sequentially` form is admitted; and
  `instrProfile (ControllerSacrifices n)` announces the object still on the
  battlefield.
- **One table.** Delete `instrIntro`'s own rows and define
  `instrIntro e = deed p ++ announced p where p = instrProfile e`, moving the
  `moveIntro`/`stampIntro` calls the old table used into `instrProfile`. Pin
  the tap-after-exile term and witness the search read through `InsteadOf`.
- **G2 — one plural-agent function.** `Effect.doesInstrIntro`, `doesPreIntro`,
  `doesRiderIntro` and `doesAnnIntro` are four case tables over
  `Plurality × {Move, SetStatus, …}` differing only in the projection they
  build, with `distributedDelta` serving all four. Replace them with
  `doesProfile : Plurality -> (s : Noun bs Player) -> VerbLabel ->
  Instruction (agentIntro s) -> InstrProfile bs`, and make `riderIntro` a
  fourth `InstrProfile` field (it is `preIntro` except for the `Enact`,
  `CantBe`, `Reflexively`, `ThisWay` and `Sequentially` cases).

Size: M. Done when: `instrIntro` has no rows of its own; one plural-agent
function remains; the tap-after-exile term is pinned with its same-module
twin and the `InsteadOf` search read is witnessed; every re-spelled pin is
probed non-vacuous; build at its module count. Standard constraints apply,
including the RON-shaped constraint.

## As landed

- **U1 — the announced context is now the post-move stack.** `instrProfile`'s
  `Move`, `Enact Nothing v (Move …)` and `doesProfile`'s two `Move` rows
  announce `afterMoveTo to (moveIntro … (Just (zoneSort to)))`;
  `ControllerSacrifices` announces
  `MkBinding TheD Player OneOf PlayerP :: moveIntro (Just "Sacrifice") n (Just Graveyard)`;
  `Shuffle` announces `afterShuffle (nomIntro whose)`. Probes:
  `Sequentially [Simultaneously [exile You (target creature)], tap (It OneOf)]`
  is refused on
  `So (zoneIsB (zoneOfReach Bare OneOf (instrIntro (Simultaneously […]))) Battlefield)`,
  and `Simultaneously [ControllerSacrifices (target creature), tap (It OneOf)]`
  on the same goal over `annIntro (ControllerSacrifices …)`; both were admitted
  before.
- **Same drift, two more symptoms.** The `Search` deed binding carries
  `mkStamp (Just "Search") Nothing False`, so
  `InsteadOf (searchLibraryFor (exactly 1) creature) revealsIt` typechecks —
  witnessed as `ProofsZone.okInsteadOfSearchRevealsIt`; the
  `ControllerSacrifices` symptom is the probe above.
- **One table.** `instrIntro` has no rows of its own:
  `instrIntro e = profileIntro (instrProfile e)` over
  `profileIntro p = deed p ++ announced p`. The 108-row table is deleted and
  every `moveIntro`/`stampIntro`/`afterMoveTo`/`afterShuffle`/`costIntro` call
  it carried moved into `instrProfile` (112 rows → 88; `CantBe`,
  `Enact Nothing _ e`, `Reflexively` and `ThisWay` now share the sub-profile
  outright). Pins: `ProofsZone.badTapAfterSimultaneousExile` and its sequential
  twin `ProofsZone.badTapAfterSequentialExile`, both non-vacuous against the
  same-module positive `ProofsZone.okTapAfterSimultaneousDamage`.
- **G2 — one plural-agent function.** `doesInstrIntro`, `doesPreIntro`,
  `doesRiderIntro` and `doesAnnIntro` are deleted; `doesProfile : Plurality ->
  (s : Noun bs Player) -> VerbLabel -> Instruction (agentIntro s) ->
  InstrProfile bs` (8 rows) is the only plural-agent table, and `riderIntro` is
  the fourth `InstrProfile` field.
- Nothing in the ticket was left undone.

## Landing record

Measured on `idris/` at this change, quiet host apart from one sibling
workspace build during the exploratory runs (the two timings below were taken
back to back on an otherwise idle host).

- Numbers before → after: `instrIntro` rows 108 → 0; `instrProfile` rows 112 →
  88; plural-agent case tables 4 → 1; `Effect.idr` 2588 → 2490 lines;
  `InstrProfile` fields 3 → 4; pins across `Proofs*.idr` 619 → 620.
- Gates: `cd idris && ./scripts/build` — `46/46: Building Cards
  (src/Cards.idr)`, exit 0, 0 `Warning` lines, clean build 1m18.536s (pristine
  tree, same host, clean: 1m21.635s). `cargo xtask cite check
  --list-noncompliant` — `0 non-compliant citation-looking string(s)`. `cargo
  xtask cite check` — `checked 14313 citations against cr.txt (eff.
  2026-08-07); 0 stale`. `cargo xtask cite audit --diff` — `audited 4 citation
  site(s)`; [CR#701.13a], [CR#701.26a] and [CR#614.1a] were already in
  `cr-citations.lock`, so no `bless` was needed and the lock is unchanged.
- Performance advisory: `idris2 --find-ipkg --check src/Experimental/Effect.idr`
  12.834s → 4.141s (3.1x) on the matched before/after pair; whole-tree clean
  build 1m21.635s → 1m18.536s. The collapse is only affordable because the
  telescope profiles were made single-traversal (below); the naive shape
  measured 3m22s for `Effect.idr` alone and >8m for the whole change.
- Assurance counts: restored 0; re-spelled 1
  (`ProofsZone.badHeldUntilExileRetag` → `okHeldUntilExileRetag`); ignored 0;
  added 4 (`badTapAfterSimultaneousExile`, `badTapAfterSequentialExile`,
  `okTapAfterSimultaneousDamage`, `okInsteadOfSearchRevealsIt`); removed 0.
  Every added pin was probed non-vacuous by checking the same term without its
  handle in a scratch module and reading the changed message.

### Deviations and additions

- `InstrProfile.rider` is `Maybe Bindings` (`Nothing` = "the `pre` stack"), not
  `Bindings`. Measured: a third `Bindings` copy in every `sameIntro` row takes
  `Effect.idr` from 12.8s to 3m22s, because all four fields are forced. With
  `Maybe`, a row that does not deviate costs nothing. `profileRider` reads it.
- `seqProfile`/`simProfile` (with `lastProfile`, `simLast`, `simCons`,
  `reProfile`) replace `preIntros`, `instrsIntro`, `riderIntros`, `simPres`,
  `simIntro` and `annSims`: one traversal per telescope instead of two or
  three. This is what pays for the collapse.
- `mayProfile`/`offerProfile` replace `mayIntro`. `May`/`IfDone` now carry the
  body's deed in `deed` rather than `[]`, so `annIntro` still withholds it and
  `ProofsCounters.badSimultaneousReadsMayDeed`/`badSimultaneousReadsMayOutcome`
  keep refusing.
- `Macros.exileUntil`'s event slot re-spelled `preIntro` → `annIntro`: it feeds
  `HeldUntil`, whose event reads the announced context.
- `ProofsZone.badHeldUntilExileRetag` became the positive
  `okHeldUntilExileRetag`. Its refusal rested on the pre-move announcement the
  2026-09-04 ruling retires: the exiled card is in the exile zone
  [CR#701.13a] and "that card" reads it there.
- `Choose` announces `chooseIntro by n` (the chooser included), which leaves
  `Phrase.chooseAnn` and `Phrase.chosenAnnBy` unused. They are left in place:
  `Phrase.idr` is a sibling round's region this round, so removing them belongs
  to whoever owns that file next.
- STOP taken: none.
