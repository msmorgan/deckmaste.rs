---
needs: [idris-cards-bind-no-implicits]
---
# Replace the `sameX` equality family with `Eq` instances

`idris/src/Experimental/Words.idr` and `idris/src/Experimental.idr` define 42
`sameX : X -> X -> Bool` functions (36 in Words, 6 in Experimental; ~700
clauses, 79 call sites) instead of `Eq` instances. The audit that found them
rated this PLAUSIBLE REASON, not NO REASON, and the reason must be weighed
before the work: Idris 2 0.8.0 `base` has no `Eq`/`DecEq` deriving, so an
instance body is the same clause count as today's function — the migration is
LOC-neutral on the definitions. What it buys is at the use sites:

- `==` at the 79 call sites, in types and in values alike (interface methods
  reduce inside `So` gates here — `chapterMarksOk` already uses `Ord Nat`'s
  `<` under one).
- `Eq (Maybe X)` and `Eq (List X)` for free, retiring `sameMaybePart`,
  `sameSeedZone`, `sameSeedType`, `sameDomainOpt`.
- `elem`, retiring the 7 membership functions `lineHasType`, `colorMember`,
  `zoneMember`, `typeMember`, `keywordElem`, `supertypeMember`,
  `subtypeMember`.

Out of scope: the heterogeneous comparisons `sameStatusVal`/`statusClash`
(indices differ; `Eq` cannot express them) and the `predEq` family on indexed
types, which stay as functions.

## Decision to make first

Write one instance (`Eq Kind`, the largest table, ~100 clauses with the new
`ObjectOrPlayer` row) and confirm two things before converting the rest:
`sameKindRefl`/`joinable`/`(\/)` and their laws still elaborate with `==` in
place of `sameKind`, and a clean `idris/scripts/build` does not measurably slow
(`Experimental.Proofs*` dominate the build; record before/after wall time). If
either fails, stop and record the measurement in this ticket instead of
converting.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`. No Rust
crate is touched.

## Acceptance

- Every `sameX` in scope is an `Eq` instance; the `Maybe`-lifted and membership
  functions are gone; no call site spells a `sameX` name.
- `idris/scripts/build` PASS from a clean `build/`, no witness lost, no pin
  silently passing; build wall time recorded in the done ticket.

Standard constraints apply.

## Measured (2026-08-21): GO

Clean `idris/scripts/build`, 18/18, exit 0.

| Run | Wall |
| --- | --- |
| Baseline #1 | 39.22 s |
| Baseline #2 | 38.09 s |
| After `Eq Kind` only | 38.55 s |
| Final (all conversions) | 37.48 s |

Within run-to-run noise; no new explicit argument, `rewrite`, or `believe_me`
was needed, and nothing hit strict positivity. `Eq` methods reduce inside `So`
gates: `sameQRefl : (q : QualitySort) -> So (q == q)` and `sameKindRefl` are
still proven by `Oh`.

## As landed (2026-08-21)

34 `Eq` instances in `Experimental/Words.idr` (including `Eq Kind` and the
namespaced `Counter.Eq Delta`) and `Eq (NameSource bs)` in `Experimental.idr`.
Retired: `sameMaybePart`, `sameSeedZone`, `sameSeedType` (`Eq (Maybe a)`), and
`lineHasType`, `colorMember`, `zoneMember`, `typeMember`, `keywordElem`,
`supertypeMember`, `subtypeMember` (`elem`). Backticked `` `elem` `` binds
looser than `||`, so call sites use the prefix form.

Kept as functions:

- `sameStatusVal`, `statusClash` — heterogeneous (`StatusVal c` vs
  `StatusVal c'`); `Eq` cannot express them.
- `predEq` family — indexed types, out of scope.
- `sameChoiceDomain` / `sameDomainOpt` — **elaboration regression.** An
  `Eq (ChoiceDomain q)` instance inside `Experimental.idr`'s single `mutual`
  block makes the size-change checker lose the `predEq` ↔ equality cycle: 58
  definitions (`predEq`, `negates`, `contradictionFree`, `flattenPs`,
  `nomIntro`, …) fail totality. An interface implementation is opaque where a
  plain function is not. Reverted; `sameDomainOpt` stays with it.
- `sameDirection` — direction agreement on `PtShift`, not structural equality;
  an `Eq` instance would name a different relation.
