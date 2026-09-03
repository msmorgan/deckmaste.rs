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
