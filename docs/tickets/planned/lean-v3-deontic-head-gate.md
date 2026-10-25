---
needs: []
---
# The deontic head gate refuses printed "can't become the monarch"

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("engine
creep"), verified against the tree at `wmyykqnv`. Paths are under `lean/Semantics/`.

## BLOCKING open question (owner)

Which fix?

- **(a) Delete the separate deontic gate** and gate `cant`/`must`/`can`/`gatedBy`/`asThough`
  with the event gate (`Action.eventHeadOk`). This amends ADR `docs/decisions/semantics-v3.md`
  §3.7 (`:485`, "Deontic clauses use a separate closed head gate …"). **Recommended:** the
  closed list is an observation of which heads cards happened to constrain, not text meaning or
  anaphora, and it refuses printed text; compound and control-flow actions are still refused by
  the event gate.
- **(b) Keep the gate and admit `gainDesignation`** in `Action.deonticHead`
  (`Check/Events.lean:54-66`), with the ADR list at `:485` updated to match.

Implement only the option ruled.

## The defect

*Jared Carthalion, True Heir* prints "You can't become the monarch this turn." *Fioran
Reformist*'s Democracy reminder prints "Players can't become the monarch." Both are Vintage-legal.
A probe on 2026-10-25 of `cant you (gainDesignation you monarch)` (and of `cant (each isPlayer)
…`) returned `[.eventHead]`: `Action.deonticHead` (`Check/Events.lean:54-66`) admits a closed
list that omits `gainDesignation`. Its other omissions relative to `Action.eventHead`
(`:31-44`) are `markDamage`, `clearDamage`, the `flipped`/`unflipped`/`faceUp`/`faceDown`
statuses, `turnFaceDown`, `combat`, `attackUnblocked`, `attackWith`, `commitCrime`, `gameGains`,
`choose`, `addMana`, `expose`, `shuffle`, `flipCoins`, `rollDice`, `createObject`, `distribute`,
`moveCounters`, `relate`, `merge`, `skipUntap`, `skipPart`, `addTurnPart` and `exchange`.

## Change under (a)

- Delete `Action.deonticHead` (`Check/Events.lean:50-66`, with its docstring) and
  `Action.deonticHeadOk` (`:68-71`).
- Replace `Action.deonticHeadOk` with `Action.eventHeadOk` at its four call sites:
  `Check/Phrase.lean:4579`, `:4597`, `:4609`, `:4627`.
- Pins:
  - `headerDeedIsReadByTheGateAndTheStepper` (`Proofs/DefinitionsAndTemplates.lean:77-82`): drop
    the `spell.deonticHeadOk = true` conjunct.
  - `headGatesAgreeOnADefinitionHeader` (`:86-88`): retire; with one gate it is vacuous.
  - `cantBePreventedIsADeonticOverPreventDamage` (`Proofs/Deontic.lean:289-300`): its last
    conjunct becomes `(Action.preventDamage .all none none).eventHeadOk = true`.
  - `whippoorwillCantIsADeonticOverRedirectDamage` (`:306-322`): its two `deonticHeadOk`
    conjuncts become `eventHeadOk`, keeping `.replace … = false` (a self-replacement is no event
    head either [CR#614.15]).
  - `prohibitionAdmitsAnyZoneChange` (`:686-690`): drop the third conjunct.
- Docs: rewrite ADR `:485` to say deontic clauses use the event gate (keep its sentences on
  `anyCost`, `anyBallot` and pattern entry), and `:331`'s "§3.7 for deontic rules" reference.

## Change under (b)

- Add `| .gainDesignation ..` to `Action.deonticHead` (`Check/Events.lean:54-62`) and to the ADR
  list at `:485`. No pin is rewritten.

## Pins (either option)

- Added: `jaredCarthalionTrueHeir` and `fioranReformist` spelled in full (`Cards/Deontic.lean`),
  each checking `[]`. Fioran Reformist's Democracy is a designation the card names ("Democracy
  is in effect"); if the bench cannot write "Democracy is in effect until …", pin only the
  reminder sentence's static `cant (each isPlayer) (gainDesignation it monarch)` as a fragment
  and say why.

## What must not change

- The event gate `Action.eventHead`; the `ownScope` header admission (`Action.headGate`).
- Every other pin in `Proofs/Deontic.lean` and `Cards/Deontic.lean`.

## Acceptance

`./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates.
