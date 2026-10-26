---
needs: []
---
# The deontic head gate refuses printed "can't become the monarch"

Standard constraints apply. Evidence corrected 2026-10-25 (Q18 follow-up); the question above is
still the owner's. Found by the 2026-10-25 triage of an external audit ("engine
creep"), verified against the tree at `wmyykqnv`. Paths are under `lean/Semantics/`.

## BLOCKING open question (owner)

Which fix?

- **(a) Delete the separate deontic gate** and gate `cant`/`must`/`can`/`gatedBy`/`asThough`
  with the event gate (`Action.eventHeadOk`). This amends ADR `docs/decisions/semantics-v3.md`
  §3.7 (`:485`, "Deontic clauses use a separate closed head gate …") and the reference at
  `:331` ("§3.7 for deontic rules"; keep the 2026-10-24 ruling there as history). **Recommended:** the
  closed list is an observation of which heads cards happened to constrain, not text meaning or
  anaphora, and it refuses printed text; compound and control-flow actions are still refused by
  the event gate. A "can't" effect takes precedence over an effect that allows or directs the
  thing [CR#101.2], so a prohibition has no reason to admit fewer actions than the events the
  checker already knows.
- **(b) Keep the gate and admit `gainDesignation`** in `Action.deonticHead`
  (`Check/Events.lean:54-66`), with the ADR list at `:485` updated to match.

Implement only the option ruled.

## The defect

*Jared Carthalion, True Heir* prints "You can't become the monarch this turn." *Fioran
Reformist*'s Democracy reminder prints "Players can't become the monarch." Both are Vintage-legal. The monarch is a designation a player can have, and the rule that makes a
player the monarch is [CR#725.1,725.2] (an earlier draft of this ticket cited the wrong
rule).
A probe on 2026-10-25 of `cant you (gainDesignation you monarch)` (and of `cant (each isPlayer)
…`) returned `[.eventHead]`: `Action.deonticHead` (`Check/Events.lean:50-71`, its docstring and `deonticHeadOk` included) is
the event gate minus about 25 heads: it omits `gainDesignation`, and its other omissions relative
to `Action.eventHead` (`:31-44`) are `markDamage`, `clearDamage`, the `flipped`/`unflipped`/`faceUp`/`faceDown`
statuses, `turnFaceDown`, `combat`, `attackUnblocked`, `attackWith`, `commitCrime`, `gameGains`,
`choose`, `addMana`, `expose`, `shuffle`, `flipCoins`, `rollDice`, `createObject`, `distribute`,
`moveCounters`, `relate`, `merge`, `skipUntap`, `skipPart`, `addTurnPart` and `exchange`.

## Change under (a)

- Delete `Action.deonticHead` (`Check/Events.lean:50-66`, with its docstring) and
  `Action.deonticHeadOk` (`:68-71`, with its docstring).
- Replace `Action.deonticHeadOk` with `Action.eventHeadOk` at its four call sites, all in
  `Check/Phrase.lean`: `:4579` (`… Action.deonticHeadOk false (between := (stepDeonticModsWith
  ops · mods))`), `:4597` (`(between := permitting)`), `:4609` (`stepActionClauseWith ops drs
  subj action .preEvent Action.deonticHeadOk false`) and `:4627` (`(between := gated)`).
  `grep -rn deonticHead lean/Semantics` is empty afterwards.
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
- Docs, in `docs/decisions/semantics-v3.md`: rewrite `:485` to say deontic clauses use the event
  gate (keep its sentences on `anyCost`, `anyBallot` and pattern entry, drop the closed list),
  and `:331`'s "§3.7 for deontic rules" reference to point at the event gate, keeping the
  2026-10-24 ruling it records as history. Cite [CR#101.2] for why a prohibition is gated like an
  event.
- These five existing pins are the ones that change under (a): `headerDeedIsReadByTheGateAndTheStepper`,
  `headGatesAgreeOnADefinitionHeader` (retired), `cantBePreventedIsADeonticOverPreventDamage`,
  `whippoorwillCantIsADeonticOverRedirectDamage`, `prohibitionAdmitsAnyZoneChange`.

## Change under (b)

- Add `| .gainDesignation ..` to `Action.deonticHead` (`Check/Events.lean:54-62`) and to the ADR
  list at `:485`. No pin is rewritten.

## Pins (either option)

- Added: `jaredCarthalionTrueHeir` and `fioranReformist` spelled in full (`Cards/Deontic.lean`),
  each checking `[]`. The probed spellings (both refused `[.eventHead]` today; they pass after
  (a), and after (b) too):
  - Jared Carthalion, True Heir, Oracle text from `data/derived/cards.jsonl`: "When Jared
    Carthalion enters, target opponent becomes the monarch. You can't become the monarch this
    turn." The second sentence is `cant (.actor .you) (gainDesignation .actor monarch)` with
    this-turn timing.
  - Fioran Reformist: "Democracy is in effect until Fioran Reformist leaves the battlefield. (At
    the beginning of each player's end step, if they control more creatures than each other
    player, they draw a card. Players can't become the monarch.)" The reminder's last sentence is
    `cant (.actor (bare isPlayer)) (gainDesignation .actor monarch)`. Fioran Reformist's Democracy is a designation the card names ("Democracy
  is in effect"); if the bench cannot write "Democracy is in effect until …", pin only the
  reminder sentence's static `cant (each isPlayer) (gainDesignation it monarch)` as a fragment
  and say why.

## What must not change

- The event gate `Action.eventHead`; the `ownScope` header admission (`Action.headGate`).
- Every other pin in `Proofs/Deontic.lean` and `Cards/Deontic.lean`.

## Acceptance

`./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates
(`cargo xtask cite check --list-noncompliant` empty, `cargo xtask cite check` 0 stale,
`jj --no-pager diff --git | cargo xtask cite audit --diff` read rule by rule). Under (a),
`grep -rn 'deonticHead' lean/ docs/decisions` is empty.
