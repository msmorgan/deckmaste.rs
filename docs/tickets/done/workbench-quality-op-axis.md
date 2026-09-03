---
needs: [workbench-effect-axes]
---
**Replace the eight type/colour statics with one `Becomes n (op : CharOp) (q :
QualityPayload)` carrying per-payload gates.** Ruling 2026-09-02 on audit item
R6.

`Effect.StaticEffect:302-339` has `BecomesAlso` (6 uses), `AddsEveryType` (11),
`LosesEveryType` (5), `LosesType` (1), `SetsColor` (9), `SetsType` (25),
`AddsChosenQuality` (6), `SetsChosenQuality` (12): eight rows spanning op ∈
{Adds, Sets, Loses} × payload ∈ {written bundle, every-type-of-space, one type,
chosen quality, colour spec}, with holes. v1 spells this as
`Modification::{CardTypes,Subtypes,Colors,Supertypes} (CollectionOp
Set/Add/Remove)`.

## The ruling

One row: `Becomes n (op : CharOp) (q : QualityPayload)`, with the per-payload
gates (`AddedFits`, `RetentionOk`, `ColorSpecOk`) selected on `q`. 8 → 1, plus
the two small enums.

This **deliberately reverses** the landed decision in
`done/workbench-choice-d-ascription-payloads.md`, which chose "SetsColor, one
row for the setting". That was a landed choice, not one of the settled rulings,
and it is superseded here: the op axis is data, and a per-payload row for each
op cell is the shape the audit found producing the holes. Record the reversal
on the declaration.

Size: M.

Done when: build is 23/23; all eight named constructors are gone from
`Effect.StaticEffect`; `CharOp` and `QualityPayload` exist and `Becomes` is the
sole spelling; every one of the 75 bench witnesses across the eight rows is
re-spelled and still typechecks; the ascription pins from
`workbench-choice-d-ascription-payloads` are re-spelled against `Becomes` and
remain non-vacuous; the reversal is noted on the declaration and in that done
ticket. Standard constraints apply.

## As landed

- `Effect.StaticEffect` has one row, `Becomes n (op : CharOp) (q : QualityPayload bs)` with `{auto 0 ok : BecomesOk op n q}`; `BecomesAlso`, `AddsEveryType`, `LosesEveryType`, `LosesType`, `SetsColor`, `SetsType`, `AddsChosenQuality`, `SetsChosenQuality` are gone. The reversal of choice-D is on the declaration's docstring and noted at the foot of that done ticket.
- `CharOp = Adds | Sets | Loses`; `QualityPayload = Bundle (t : TokenChars) (ret : Maybe CardType) | EveryTypeOf TypeSpace | ChosenQuality (Predicate Object) | Colored ColorSpec`. The audit's "one type" payload is folded into `Bundle` (Luxior's "isn't a planeswalker" is a one-type line under `Loses`); the retention rider rides the bundle and is admitted under `Sets` alone.
- Gates survive on the payload side as one composite `becomesOk`, selected on `q` then `op`: `Bundle` under `Adds` keeps `addsSomething`/`addedFits`/`tokenCanonical`/`tokenAbilitiesOk`/`tokenQualsFit`/`additionUnnamed` (no zone demand, as before), under `Sets` keeps the battlefield zone, `lineNonEmpty`, `addedFits`, the three token gates and `retentionOk`, under `Loses` asks the battlefield zone and `lossWritesTypes` (a line or supertypes, nothing else); `EveryTypeOf` keeps the zone and `spaceHosted`; `ChosenQuality` keeps `qualityReadOk` and `hostedRead`; `Colored` keeps `colorSpecOk` plus `colorOpOk` (`SomeColors []` is `Sets`-only). `AdditionSaysSomething` is deleted: a colour-only addition is the `Adds × Colored` cell, so the bundle's addition gate is `addsSomething` on the line or a written supertype.
- `staticKind` is `becomesKind op q` (`Colored` → `ColorSet`, else `Adds`/`Sets`/`Loses` → `TypeAddition`/`TypeSet`/`TypeLoss`); no new `StaticKind` member.
- `Macros.becomesAs`/`becomes`/`becomesColor` are thin wrappers over `Becomes` with one `{auto 0 ok : BecomesOk …}`; no macro deleted or added.
- All 75 `Cards.idr` sites re-spelled by a balanced-atom script (25 `SetsType`, 12 `SetsChosenQuality`, 11 `AddsEveryType`, 9 `SetsColor`, 6 `BecomesAlso`, 6 `AddsChosenQuality`, 5 `LosesEveryType`, 1 `LosesType`); Indigo Faerie moves from a colour-only bundle to `Adds (Colored (SomeColors [Blue]))`.
- Pins re-spelled against `Becomes` (`{ok = ok}`): `badBecomesZombieLand`, `badBecomesNothing`, `badBecomesOwnType` (ProofsB); `badStillOnSubtypeSet`, `badStillAnInstant` (ProofsE); `badChosenBasicTypeOnCreature`, `badCreaturesAreMountains`, `badDoubleExtension`, `badAscribedQualityBeforeChoice`, `badEveryCreatureTypeOnLand`, `badEveryBasicLandTypeOnCreature` (ProofsF); `badNamedAddition`, `badRepeatedAdditionColor`, `badRepeatedAdditionType` (ProofsG). New pins: `badAddsNoColor`, `badLosesNoColor` [CR#105.2c], `badLosesPt` [CR#613.1d], `badStillOnAddition` [CR#205.1b]. Every one of the 17 was probed non-vacuous: its corrected sentence typechecks as a positive term (scratch module, deleted).
- New printed witness `arcumsWeathervane` (supported; "is no longer snow" = `Loses (Bundle supers [Snow])`, "becomes snow" = `Adds (Bundle supers [Snow])`, [CR#205.4b]'s gain/loss of a supertype). Synthetic witnesses in ProofsF: `setsEveryBasicLandType`, `losesChosenCreatureType`, `losesAllColors`.

Grid (op × payload), each cell's witness or pin:

| | `Bundle` | `EveryTypeOf` | `ChosenQuality` | `Colored` |
|---|---|---|---|---|
| `Adds` | Ghoulflesh, Blade of the Oni, Arcum's Weathervane ("becomes snow"); ret refused by `badStillOnAddition` | Prismatic Omen, Mistform Ultimus | Ashes of the Fallen, Xenograft-shaped lines | Indigo Faerie; `SomeColors []` refused by `badAddsNoColor` |
| `Sets` | Blood Moon, Darksteel Mutation, Missy | `setsEveryBasicLandType` (synthetic; no printed line drops "in addition") | Reef Shaman, Dream Thrush-shaped lines | Darkest Hour, Thran Lens, Transguild Courier |
| `Loses` | Luxior ("isn't a planeswalker"), Arcum's Weathervane ("is no longer snow"); P/T refused by `badLosesPt` | Amoeboid Changeling, Curse of Conformity | `losesChosenCreatureType` (synthetic) | `losesAllColors` (synthetic); `SomeColors []` refused by `badLosesNoColor` |

- Undone: nothing in the ticket's letter. Observations: Mistform Ultimus's "is every creature type" stays `Adds` (its prior spelling) though [CR#205.1b] reads a bare "is" as a setting — the two coincide for the whole creature space, and re-reading it is not this ticket's; the per-cell zone demands are carried over exactly (no battlefield demand on `Adds × Bundle`, `ChosenQuality` or `Colored`), so choice-D's "kept" note for `BecomesAlso` describes a demand the code never had.

## Landing record

- Construction count: `StaticEffect` −8/+1 (`Becomes`); new `CharOp` (3) and `QualityPayload` (4); `AdditionSaysSomething`/`additionSaysSomething` deleted; `lossWritesTypes`, `bundleOk`, `colorOpOk`, `becomesOk`, `BecomesOk`, `becomesKind` added in `Effect`.
- Coverage and lock state: printed-card bench coverage +1 (Arcum's Weathervane), no witnesses removed; `cr-citations.lock` unchanged (every cited rule was already registered; `bless` would only have dropped 23 rules no file in this tree cites, so its output was not kept).
- Assurance: restored 0; re-spelled 75 card sites + Indigo Faerie + 14 pins; ignored 0; added 8 (1 card, 3 synthetic witnesses, 4 pins); removed 0.
- Positive artifacts: 23/23 Idris build from a clean `build/`; `cite check --list-noncompliant` 0; `cite check` 0 stale of 17893; diff audit 5 sites read against their rule text.
- Deviations and additions: the "one type" payload of the audit's grid is not a `QualityPayload` member (folded into `Bundle`, above); `Adds × Colored` reverses choice-D's "no second row for the colour-only addition" as the ruling's op axis makes it a cell, not a row; the choice-D done ticket gained one superseding line (the ticket's done-when asks for it).
- STOPs: none.
