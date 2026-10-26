---
needs: []
---
# "The rest" and "the other" read their kind from the text

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against `wmyykqnv`. Paths are under `lean/Semantics/`. The whole change was built on a
scratch copy of `lean/` on 2026-10-25 and passes `lake build --wfail` (patch:
`/tmp/claude-1000/-home-msmorgan-Projects-deckmaste-rs/4e5532ee-6640-4aad-9769-888a72f918d1/scratchpad/q11-theRest-kind-inference.patch`).
A first stage, in which the checker ignored the authored kind and used the inferred one, passed
every existing pin unchanged, so inference agrees with the authored kind at all 35 sites.

## Why

In "one pile … the other" and "one of them … the rest", the head noun is elided and recovered
from the partitive (Oracle English glossary, **Ellipsis** and **Antecedent**). Oracle never
prints "the other pile". Today authors write the recovered kind (`theRest .object`,
`theOther .pile`), which is checker plumbing in authored text. The checker reads it instead.

## Change

**Core** (`Abilities.lean:393`): `| theRest (kind : Kind) (plurality : Plurality)` →
`| theRest (plurality : Plurality)`. Doc: "the rest" (`.many`) and "the other" (`.one`); the kind
is that of the part or group the text took from, read by the checker.

**Templates** (`Templates.lean:690-693`):
```lean
/-- "the rest [of them]" -/
semantic_template theRest : NounPhrase := .theRest .many
/-- "the other [one]", "the other [pile]": the one remaining after a choice among two. -/
semantic_template theOther : NounPhrase := .theRest .one
```

**Checker, `Check/Words.lean`.** Add beside `restGroup`/`zoneOfGroup` (`:978-987`):
```lean
/-- The kind "the rest" (`.many`) or "the other" (`.one`) reads, from the text: for "the other",
the group of the current distributive member first; else the newest visible part a selection
took (`det == .part`); else the newest selection in the current selection scope. -/
def DRS.restKind? (pl : Plurality) (drs : DRS) : Option Kind :=
  let distributive : Option Kind :=
    if pl.isOne then
      (drs.distributiveSubject.filter drs.isVisible).bind fun member =>
        ((drs.findById? member).bind (·.memberOf)).bind fun groupId =>
          (drs.findById? groupId).map (·.kind)
    else none
  distributive.orElse fun _ =>
    ((drs.visibleReferents.find? fun b => !b.linked && b.det == .part).map (·.kind)).orElse
      fun _ => (drs.selectionEvidence.find? (·.scope == drs.selectionScope)).map (·.kind)
```
The Kind-taking internals (`theRestOk`, `theOtherOk`, `theRestFits`, `restSource`,
`consumeRest`, `restGroup`, `zoneOfGroup`, `tyOfGroup`, `distributivePair?`, `:850-987`) keep
their signatures; only their callers change.

**Checker, `Check/Phrase.lean`:**
- `:118` (`NounPhrase.kind?`): `| _ + 1, .theRest k _ => some k` → `| _ + 1, .theRest _ => none`.
- `NounPhrase.kindIn` (`:122-131`): add `| _ + 1, .theRest pl => (drs.restKind? pl).getD d`
  before the `| _ + 1, n => n.kind?.getD d` fallback.
- `:334`: `.theRest _ pl` → `.theRest pl`.
- `:740` (`restContext?`): `| _ + 1, .theRest kind plurality => some (kind, plurality, drs)` →
  `| _ + 1, .theRest plurality => some ((drs.restKind? plurality).getD .object, plurality, drs)`.
  The move arm at `:3967` consumes it unchanged.
- `:3275-3283` (stepper arm): `| .theRest pl =>` with `let k := (drs.restKind? pl).getD .object`,
  then the existing body over `k`, and the refusal `.theRestFits pl`.

**Refusal** (`Check/Refusal.lean:102`): `| theRestFits (k : Kind)` → `| theRestFits (plurality :
Plurality)`, doc: "the rest" (`.many`) or "the other" (`.one`) finds no part or group to be the
remainder of. The kind is no longer authored, and where none can be inferred there is none to
report.

**Authored sites** (`theRest .object` → `theRest`; `theOther .object`, `theOther .pile` →
`theOther`):
- `theRest`: `Cards/Anaphora.lean:81` (Impulse), `:87` (Anticipate), `:105`, `:110`, `:120`,
  `:139`, `:142` (Flow State and its pins), `:155`, `:161`; `Cards/Piles.lean:28` (Tezzeret's
  Gatebreaker); `Cards/Mana.lean:799` (Collected Company); `Cards/Choice.lean:210`, `:215` (Green
  Sun's Twilight), `:444` (Murmurs from Beyond), `:1184` (Duneblast), `:1196` (Boreas Charger),
  `:1271` (Commune with the Gods), `:1573` (Stick Together); `Definitions/Actions.lean:352`
  (discover), `:394` (manifestDread); `Definitions/Keywords.lean:1009` (hideaway);
  `Templates.lean:1407` (`lookAndSortSlice`).
- `theOther` (was `.object`): `Cards/Anaphora.lean:282` (Write into Being), `:668` (Codecracker
  Hound); `Cards/Choice.lean:142` (Grim Contest); `Definitions/Actions.lean:209`
  (`fightEachOther`).
- `theOther` (was `.pile`): `Cards/Cost.lean:983` (Unesh); `Cards/Anaphora.lean:706` (Fact or
  Fiction); `Cards/Piles.lean:160` (Death or Glory), `:171` (Steam Augury);
  `Cards/Faces.lean:553`, `:563`, `:578`, `:593` (Riddles in the Dark, Fortune's Favor, Curator
  of Destinies, Atris); `Cards/Keyword.lean:1550` (Sphinx of Uthuun).

**Raw Core spellings** (`.theRest .object .many` → `.theRest .many`; `.theRest .object .one`,
`.theRest .pile .one` → `.theRest .one`): `Check.lean:246`, `Proofs/BindingIdentity.lean:177`,
`Proofs/SelectionEvidence.lean:36,43,47,53,57`.

## What must not change

The Kind-taking internals and `Check/Queries.lean:61,160-161` (which call them with `.object`
directly); the `restSource`/`consumeRest` semantics; every existing pin's claim.

## Pins

- Re-spelled, same claim (kind argument dropped): `okTheRestAfterCoordinatedParts`
  (`Proofs/Anaphora.lean:602`), `okTheOtherAfterCoordinatedParts` (`:611`),
  `okRestAfterAPartition` (`:756`), `okRestAfterCountedChoice` (`:765`); `okChoiceRestStands`
  (`Proofs/Choice.lean:220`), `afterChoiceRestDisposed` (`:224`), `distributedRestOfOwnChoice`
  (`:271`), `okDistributedRestStands` (`:285`),
  `distributedRestPreservesTheUnrelatedOuterGroup` (`:330`),
  `distributedSacrificeConsumesTheOuterRemainder` (`:342`); `theOtherIsTheMembersCounterpart`
  (`Proofs/Damage.lean:125`), `okRestAfterPart` (`:473`), `badRestWithoutPart` (`:483`);
  `badEachOfTheRest` (`Proofs/Zone.lean:337`), `badAgentChooseTheRest` (`:465`);
  `optionalSubsetKeepsItsGroupWithoutTheSelectedCard` (`Proofs/SelectionEvidence.lean:36`).
- Rewritten on the refusal payload only (`.theRestFits .object` → the plurality):
  - to `.theRestFits .one`: `fightEachOtherNeedsAPair` (`Proofs/Damage.lean:103,106`),
    `theOtherIsTheMembersCounterpart` (`:135`), `optionalSubsetCannotPromiseOneRemainingCard`
    (`Proofs/SelectionEvidence.lean:57`);
  - to `.theRestFits .many`: `badRestWithoutAPartition` (`Proofs/Anaphora.lean:760`),
    `badRestAfterTargetChoice` (`:773`), `badChoiceRestDisposedTwice` (`Proofs/Choice.lean:229`),
    `consumingObjectRestPreservesUnrelatedPlayerChoice` (`:257`),
    `badDistributedRestDisposedTwice` (`:294`), `badRestWithoutGroup` (`Proofs/Damage.lean:478`),
    `badRestWithoutPart` (`:484`), `badRestDisposedTwice` (`:493`),
    `badRestOverTwoAnnouncements` (`:500`), `missingSelectionIsStillRefused`
    (`Proofs/SelectionEvidence.lean:43`), `unrelatedOrClosedGroupCannotSupplyTheRemainder`
    (`:47`, `:53`).
- Added (in `Proofs/Anaphora.lean`, each a `decide` over the card's term as spelled in `Cards/`):
  - `theOtherReadsThePileKind`: in *Fact or Fiction*, after "Put one pile into your hand", the
    referent `theOther` mints has kind `.pile`;
  - `theRestReadsThePartKind`: in *Impulse*, after "Put one of them into your hand",
    `restKind? .many = some .object`;
  - `theOtherReadsTheDistributiveGroup`: in *Grim Contest*, `restKind? .one = some .object`.

## Acceptance

- `./lean/scripts/build` (`--wfail`), `cargo xtask lean check`, the CR citation gates.
- `grep -rn 'theRest \.\|theOther \.' lean/Semantics` returns only `.theRest .many` / `.theRest
  .one` hits; `grep -rn 'theRestFits \.object' lean/Semantics` is empty.
