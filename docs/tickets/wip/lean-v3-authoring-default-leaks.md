---
needs: []
---
# Template defaults that leak raw constructors, and small authoring-surface residue

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against the tree at `wmyykqnv`. Paths are under `lean/Semantics/`; line numbers are as
of that change.

The authoring boundary (`Authoring.inspect`, `Authoring.lean:505`; `Form.onlyAuthored`,
`:42-51`; enforced by `elabSpelled`, `Check/Card.lean:340-357`) refuses a raw constructor in a
`spelled` card. It covers only `Spelled` cards; Definitions, Templates, `Cards/` fragments and
`Proofs/` are trusted or fixtures (owner, 2026-10-23,
`docs/tickets/done/lean-v3-catalog-items-self-contained.md:56-63`; `lean/CONTRACTS.md:177-182`).
That boundary is not changed here.

## 1. Default arguments written as raw constructors

Lean inserts a default argument at the call site, so `inspect` reads a raw default as the
card's own raw constructor. Two Templates have one:

- `putOntoBattlefieldTappedAttacking` (`Templates.lean:1186-1188`): `(from_ : Location :=
  .anyLocation)`. Write `(from_ : Location := wherever)` (`Templates.lean:147`).
- `gainControl` (`Templates.lean:1463-1465`): `(controller : NounPhrase := .actor)`. Write
  `(controller : NounPhrase := theActor)` (`Templates.lean:1640`). `theActor` is declared after
  `gainControl`; move `theActor` (with its docstring) above `gainControl`, unchanged.

Both edits leave every expanded tree unchanged. Then the five spelled `gainControl` sites that
write `(controller := you)` only because the default was refused, and whose Oracle text names no
controller (an imperative "gain control of"), drop it:

- `Cards/Anaphora.lean:633` and `:647` (Zidane, Tantalus Thief)
- `Cards/Description.lean:79`
- `Cards/Static.lean:496` (Blatant Thievery; `lean-v3-card-head-noun-sweep` §1 row 2 edits the
  same line's target, so whichever lands second rebases onto the other)
- `Cards/Choice.lean:1025`

Before each, read the card's Oracle text in `data/derived/cards.jsonl`. If it prints a controller
("target player gains control of …"), keep the argument and say so. Dropping it changes the tree
from `.you` to `.actor`; the spelled card must still check `[]`.

## 2. Fragments that write a default raw

Four fragments pass the default `RollWatch` raw. Drop the argument (it defaults to `anyResult`,
`Templates.lean:1137`):

- `Cards/Anaphora.lean:326` (`wyllExtraDie`): `rollDice (plus thatMuch 1) .thoseDice`
- `Cards/Anaphora.lean:335` (`vedalkenSquirrelWhackerReroll`): `rollDice thatMuch .thoseDice`
- `Cards/Cost.lean:705` (`bamboozlingBeebleIgnore`): `rollDice (plus thatMuch 1) .thoseDice`
- `Cards/Description.lean:474` (`ironMastiffIgnore`): drop the trailing `.anyResult`

## 3. A Template that mentions its parameter twice without a binding

**Moved 2026-10-25** to [lean-v3-binding-across-or-arms](../planned/lean-v3-binding-across-or-arms.md): the spelling below is refused (`bindingIdentity`) because the two mentions sit in different `.or` arms.

`objectsOfPlayer` (`Templates.lean:479-480`) writes `player` twice. ADR
`docs/decisions/semantics-v3.md` §2.2 binds a Template parameter mentioned more than once. Write

```lean
semantic_template objectsOfPlayer (player : NounPhrase) : Predicate :=
  .or [ controlledBy (.firstMention "objectsOfPlayer.player" player),
        .and [ ownedBy (.laterMention "objectsOfPlayer.player" player),
               .not (controlledBy (a .isPlayer)) ] ]
```

using the identity form the other Templates use (`Templates.lean:836-837`, `:1404-1405`); match
their exact spelling (with or without `.inCaller`). Its only caller is
`Cards/Choice.lean:1402`; that card must still check `[]`.

## 4. Dead fixture

`dazzlingBeautyCastRestriction` (`Cards/Turn.lean:451-453`) has no user and no pin, and its
docstring defers "machinery outside this ticket". Delete it with its docstring.

## 5. Explicit `some` the coercion makes redundant

The core coercion `Coe α (Option α)` (Lean `Init/Data/Option/Coe.lean:21`) was adopted by owner
ruling C, 2026-10-21 (`docs/tickets/done/lean-v3-authoring-surface.md:45-62`). A leading-dot
argument keeps its `some` (`lean/CONTRACTS.md:123-129`, pinned at `Proofs/Authoring.lean:518`).
Each site below was found removable on 2026-10-25 by deleting the word and rebuilding the file
with its pins; 182 other `some` in `Cards/` must stay and are not listed. Delete the word `some`
(and its now-redundant parentheses only where they wrap nothing else) at exactly these
`file:line:column` positions:

- `Cards/Anaphora.lean`: 53:19 234:19 242:19 829:28 830:32 871:12 872:12 882:12 883:12
- `Cards/Choice.lean`: 78:40 78:69 241:35 1140:42 1152:13 1283:24 1292:24 1353:61 1354:92
  1369:50 1373:47 1378:35 1421:47
- `Cards/Copy.lean`: 127:46
- `Cards/Cost.lean`: 382:70 738:45 816:68 830:23 846:69
- `Cards/Counters.lean`: 22:34 202:20 258:40 299:71 425:18 458:26 568:12 571:12 841:40 900:6
  929:41 1035:40 1087:64
- `Cards/Damage.lean`: 939:76 1230:48 1289:45
- `Cards/Deontic.lean`: 224:18 283:61 443:42
- `Cards/Faces.lean`: 241:13 330:56 336:21
- `Cards/Keyword.lean`: 25:84 94:27 212:19 305:110 785:83 952:48 1022:42 1102:48 1478:23
  1488:43 1619:18
- `Cards/Mana.lean`: 726:48 846:16 846:57 888:13 1001:42
- `Cards/Static.lean`: 39:15 784:84 836:23
- `Cards/Trigger.lean`: 98:13 392:19 409:39 484:34 561:73 563:73 668:86 685:37 768:22 789:16
  905:30 990:19
- `Cards/Turn.lean`: 475:72 519:23 531:23 568:23 580:23 620:17 632:29 636:19 650:19 666:19
  670:23 673:14

Apply this section first, before the line numbers above drift, or apply it in the tree it was
measured on (`wmyykqnv`) and rebase. If a listed site no longer holds a `some` (another ticket
moved it), skip it and say so. The 12 `.some` in `Cards/` are `ColorSpec.some`
(`Words.lean:657`), not `Option`; leave them.

## What must not change

- The authoring boundary and its contract; Definition and Template bodies other than §1 and §3.
- Every kernel term except the five §1 sites (`.you` → `.actor`) and §3's binding.

## Pins

- Added: `templateDefaultsAreAuthored` in `Proofs/Authoring.lean`, a `spelled` card (a real,
  Vintage-legal card) that calls `putOntoBattlefieldTappedAttacking` and `gainControl` without
  the defaulted arguments and elaborates with no refusal. Preeminent Captain ("Whenever this
  creature attacks, you may put a Soldier creature card from your hand onto the battlefield
  tapped and attacking") is one; spell it whole, from `Cards/Choice.lean:349-352`.
- Must stay green: every spelled card in §1, `okStolenUniformOpening` (`Cards/Choice.lean:190`),
  `okPreeminentCaptain` (`:353`), the four §2 fragments' `ok…` pins, the `Cards/Choice.lean:1402`
  card, `Proofs/Authoring.lean:385-390` and `:518`.
- Retired: none (the deleted fixture has no pin).

## Acceptance

- `./lean/scripts/build` (`--wfail`), `cargo xtask lean check`.
- `grep -rn ':= \.anyLocation\|:= \.actor' lean/Semantics/Templates.lean` is empty;
  `grep -rn '\.anyResult' lean/Semantics/Cards` is empty.
- CR citation gates.
