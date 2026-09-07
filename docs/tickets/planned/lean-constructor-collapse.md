---
needs: [semantics-v2-parity]
---
# Simplify the shared Lean/Rust syntax after parity

Use the completed lowering and parity work to remove genuine redundancies in
Lean and its Rust mirror. This is follow-up cleanup, not a prerequisite for
building semantics v2 or retiring v1 (user decision, 2026-09-06). Lean remains
the specification; change both representations and their consumers together.
Standard constraints apply.

A shared checker arm, an unread payload, or absence from the bench does not
establish semantic equivalence. Engine-facing vocabulary can carry meaning
without a distinct admission law. A family fold must retain every meaningful
discriminator and payload; a macro replacement must express the same meaning
and preserve reference scope. There is no constructor-count target.

## Candidates

Recheck these against the then-current model and lowering before changing them:

- Remove `StatusCat` and `Status.category` if still unused and derivable from
  `Status`; preserve the existing status laws.
- Move `ManaUnit` to the macro-authoring layer if `scaledMana` still eliminates
  it and no semantic field carries it. Follow its actual consumers in Rust.
- Consider representing `Predicate.wasCast` through a cast predicate with an
  optional caster. Today `castBy` requires a `NounPhrase`; `castBy none` is not
  an existing expansion. Preserve rank, introductions, and source distinctions.
- Check whether `RollWatch.anyResult` duplicates an enclosing optional value.
  Fold only if omission and an explicit unrestricted watch have the same
  meaning in their consumers.
- Consider sharing the stat axis vocabulary. `CharacteristicStat` has defense
  but no mana value; `Stat` has mana value but no defense. Any consolidation
  must preserve which axes each operation admits.
- Consider a family representation for related outcomes, mana queries, or card
  forms only when the implemented consumers show a real simplification. Keep
  distinct outcomes, source queries, and frame kinds as explicit data. Replacing
  `OutcomeVerb` with unrestricted `CoreDeed` is not a mechanical deduplication.

A candidate may be retained with a brief explanation. Prefer a small justified
change to relocating variants into new enums without simplifying consumers.

## Preserved scope and corrections to the earlier audit

Preserve draw versus restart, split versus modal DFC, the distinct produced-mana
queries, repetition excluding previous choices, lookback windows, and meld
destinations unless a replacement explicitly retains their meaning. Do not
prune Special Action or other vocabulary solely because it lacks a witness or
checker read. Cost symbols and `LoyaltyCost`, including `downX`, remain deferred.
`workbench-last-shapes` remains separate.

The earlier audit's alleged missing status and face laws already exist as
`Status.clash`, `Status.word`, `Status.markable`, `boxSuitsTypes`, `cardBoxOk`,
and `cardCostOk`. `SharedLineHalf.name` is a required `String`; `gameIs` and
`untilCond` have checker reads. Absence of `hasSupertype` from card-type
inference and the differing introductions of `Quantity.upToOf` and `Amount.upTo`
are not by themselves demonstrated defects. Any actual correctness defect
needs a concrete witness and should be fixed when found, without waiting for
this cleanup.

## Completion

For each selected change, record the redundancy removed and demonstrate the
preserved meaning with positive and negative witnesses. Update the Lean syntax,
checker and pins, Rust types and drift mapping, reader/emitter, affected RON
macro bodies and cards, and lowering together. Use the active crate and plugin
paths if cutover has already renamed them.

Run the Lean gate, representation drift checks, emitted-card gate, affected
Rust test closure, and the parity/regression checks exercising the changed
constructs. Preserve existing coverage and report the dispositions of the
remaining candidates. Constructor reduction alone is not completion evidence.
