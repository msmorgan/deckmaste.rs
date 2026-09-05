---
needs: []
---
**Retire `Effect.Unless`; spell "unless" as a macro over the deontic offer.**
Ruling 2026-09-04 (cleanroom review 3, D-Q9).

- `Effect.Unless` and `Effect.May offer (Pay …) Nothing (Just e)` are two core
  spellings of the same "unless a player pays" sentence — `Cards/Choice.idr`
  writes one, `Cards/Deontic.idr` the other. The offer is the row: `Unless`
  becomes a macro over `May` with a cost, and the constructor goes.
- `Unless` is one of the constructors that consumes `annIntro`; the macro
  threads the context the offer already threads, so nothing new is introduced.
- Re-spell both families of witnesses through the macro, as printed, and
  re-spell the pins that refuted through the constructor.

Size: S. Done when: `grep` finds no `Effect.Unless`; every "unless" witness
reads through the macro; the re-spelled pins probe non-vacuous; build at its
module count. Standard constraints apply, including the RON-shaped constraint.

## As landed

- `Effect.Unless` is gone: the constructor and its four function rows
  (`reflexEncloseUse`, `costActionOk`, `replacedCtx`, and the `instrProfile`
  row) are deleted; `grep` finds no `Effect.Unless` (the remaining `Unless`
  tokens are `Events.CondMarking.Unless`, a different lexeme).
- `Macros.unless who e c = May who (Pay (agentRef who ar) c PaidOnce) Nothing
  (Just e)` — the offer with a cost, named for the RON macro
  `plugins/builtin/macros/effect/Unless.ron`, cited to [CR#118.12a]. The payer
  is re-read inside the offer through `Macros.agentRef` (the settled window
  read), which reduces to the agent itself when it introduced nothing.
- `annIntro` threading is unchanged: the macro adds nothing the offer did not
  already thread, and no new `Effect` row was introduced.
- Witnesses re-spelled through the macro, as printed (14 bench cards):
  `Anaphora.manaLeak`; `Choice.rhysticStudy`, `moltingHarpy`, `carnophage`,
  `runeSnag`, `override`, `rakshasasDisdain`, `megatherium`, `killingWave`,
  `fadeAway`; `Counters.cycloneUpkeepPayment`; `Damage.quenchableFire`;
  `Deontic.stasis`; `Turn.fettergeist`.
- Pins re-spelled: `ProofsMana.badUnlessTapSymbol` (pin) with its adjacent twin
  `ProofsMana.okUnlessManaCost`, both now written through `Macros.unless`.
  Probe: swapping `TapSymbol` for `Mana [generic 3]` turns the pin into
  `Error: badUnlessTapSymbol Oh is not a valid impossible case.` — non-vacuous.
- Not re-spelled, and why: `Choice.tidalFlats` and `Damage.yawgmothDemon` print
  the offer order ("… may pay {1}. If that player doesn't, …"), so they stay
  the core `May`; `Choice.solitaryConfinement` prints "unless you discard a
  card" but its offer body is a `discard` instruction, not a `Pay` of a cost,
  and turning it into a `Do` cost is a separate ruling;
  `Macros.cumulativeUpkeepExpansion`, because [CR#702.24a] spells cumulative
  upkeep in the offer order ("you may pay [cost] … If you don't, sacrifice
  it"), not the unless order; `ProofsChoice.okMatchedPayer`/
  `badMismatchedPayer` refute the `PayAgrees` obligation on `Pay` itself, so
  they stay at the constructor they refute.

## Landing record

Measured on change `oxtzuuxl` (this commit), workspace `workbench-unless-macro`.

Numbers before → after:

- `Effect.idr` constructor declarations (`^ {4}[A-Z]… :`): 205 → 204.
- `Macros.idr` top-level exports (`^[a-z]… :`): 399 → 400.
- `Effect.Unless` sites in `idris/src/Experimental/`: 9 → 0.
- `Macros.unless` call sites: 0 → 16 (14 bench cards, 1 pin, 1 twin).
- Modules: 46/46 (unchanged).

Gate lines:

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`; 0 Error lines, 0 Warning lines; `real 1m40.196s`.
  `scripts/check-pin-twins` and the implicit-handle lint pass inside it.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14384 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → `audited 6
  citation site(s)` (7 after this record cites the rule once more): one in code, `Macros.idr:1636 [CR#118.12a]`, whose text is
  the CR's own paraphrase of the unless order as the offer — it says exactly
  what the macro does; the other five are this record's own [CR#118.12a] and
  [CR#702.24a] mentions, each read against the rule text. No `cargo xtask cite
  bless` was needed: both rules are already registered in
  `cr-citations.lock`.

Assurance counts: restored 0; re-spelled 16; ignored 0; added 0; removed 0.

Deviations and additions:

- The ticket's "both families" is read as "every witness the card prints in the
  unless order". Four offer-form sites were deliberately left at the core
  `May` — `tidalFlats`, `yawgmothDemon` and `cumulativeUpkeepExpansion` are
  printed in the offer order, and `solitaryConfinement`'s offer body is not a
  `Pay`. Listed above. `cumulativeUpkeepExpansion` was first re-spelled through
  the macro and then reverted when `cite audit` showed [CR#702.24a]'s own text
  reads "you may pay [cost] … If you don't, sacrifice it".
- Macro argument order is `unless who e c`: `e` and `c` are both indexed by
  `who`'s context, so the payer must come first; the RON macro's residual
  `effect` → `unless` order is preserved after that hoist.

STOP taken: none.
