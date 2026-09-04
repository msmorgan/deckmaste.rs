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
