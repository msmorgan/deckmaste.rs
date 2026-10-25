---
needs: [lean-v3-checker-and-drt-fixes]
---
# Checker fuel exhaustion and the remaining one-purpose flags

Split from [lean-v3-checker-and-drt-fixes](../done/lean-v3-checker-and-drt-fixes.md) (§3, the
§2.9 fuel item and the rest of the §2.13 item) on 2026-10-24, each with the options recorded
there, copied word for word. Both await an owner ruling; implement the option ruled. Done
there already: `predicatePayload` takes no fuel; `librarySearchResult` is gone; `DRS.ofList`
numbers by position (`DRS.ofNumbered`); `historicalMoveFrom` is split; `.addedAbilities`,
`NounPhrase.plur`'s slice rule and the `ownScope` mentions are no longer positional; the
`EmptyCollection DRS` instance is gone; `attachmentHost` is read only by `leaveRelative`.

## 1. Fuel defaults & helper depth limits (Opus §2.9, Astra §7)

Audit silent fuel fallbacks (`NounPhrase.kind?` → `some .object`, `NounPhrase.plur` → `.one`,
`Quantity.plur` → `.many`, `GameEvent.facts` → `{}`, the `numberSlots` family → `[]`) and tight
helper fuel limits (`stepQuantityWith`, `stepTimingWith` and `stepLocationWith`, at 8) so fuel
exhaustion cannot silently produce default classifications.

**Needs ruling 2026-10-24 (the readers over the AST).** The other readers and helpers recurse on
the one mutual block of `Abilities.lean`, which has no `brecOn` (structural recursion on it is
refused: "the type Location does not have a `.brecOn` recursor") and no generated `SizeOf`
(`sizeOf` is the default instance, 0), so fuel stays; what is open is how running out of it is
reported. Of the listed defaults, `NounPhrase.kind?`'s and `NounPhrase.plur`'s (32) are
reachable under an accepted term: the stepper's budget is 64 and theirs is 32; `Quantity.plur`'s
(16) is shadowed by `stepQuantityWith`'s refusal at 8, when the quantity is stepped. Options:
(a) readers return an exhaustion signal (`Option`/`Except`) that the checker turns into
`Refusal.recursionLimit`, changing every caller of `kind?`, `plur`, `facts` and the
`numberSlots` family; (b) one budget: the stepper's helpers (`stepQuantityWith`,
`stepTimingWith`, `stepLocationWith`, at 8) draw on the stepper's own fuel (a `StepOps.fuel`
field), every reader's default is at least that budget, and a pin per reader shows the stepper
refuses `recursionLimit` before the reader's default is reached; (c) as (b), with the
guarantee stated in `CONTRACTS.md` instead of pinned per reader; (d) enable a recursor or
`SizeOf` for the mutual block (`brecOn`/`genSizeOf`) and drop fuel from the readers, at a
build-time cost not yet measured.

## 2. The remaining one-purpose flags and the payment splice (Opus §2.13)

**Needs ruling 2026-10-24 (the remaining flags and the payment splice).**
(1) `announcedTargetResult` and `createdResult` record stepping history the suggested
derivations do not: deriving the loop's target group from `det == .target` publishes the targets
of "reveal target creature" and of an optional ("you may") target per player, which
`SelectionEvidence.ordinaryLocalsAndOptionalTargetsRemainClosed` keeps closed (the flag is set
only by a resolving `choose`); deriving creations from `Origin.token` and a create stamp loses a
created token once it moves, since a move resets its stamp [CR#400.7], against
`DrtCards.creationResultsKeepTheirActualDestinationAfterAMove` and
`movingAnOuterCreationInsideALoopDoesNotCreateAPluralResult`, and a token merely mentioned has
`Origin.token` too (`aTokenMentionIsNotACreationResult`). Options: (a) keep both flags as the
discourse facts they are, documented as such; (b) derive them and retire or rewrite those pins;
(c) move them off `Referent` into one DRS-level record of what each loop body published.
(2) `attachmentHost`'s projection: (a) keep the flag for `leaveRelative` only (as now); (b) step
`.attachHost` as a presupposition that reuses a host already in view and is accommodated in the
outermost box, which needs the host's attachment recorded on the referent to tell "enchanted"
from "equipped" hosts of one type. (3) `stepPlayPaymentWith` restores each scope frame's
`callerPending` frame-for-frame by `zipWith`, keeping the paid stack when the lengths differ:
(a) give `OwnScopeFrame` an identity and restore by it; (b) step a payment in a mode in which
mentions are not buffered, so nothing is restored; (c) keep the frame-for-frame restore, drop
the unequal-length fallback, and pin that stepping a cost leaves the scope stack balanced.
