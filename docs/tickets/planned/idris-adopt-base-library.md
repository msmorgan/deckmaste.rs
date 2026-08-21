---
needs: []
---
# Adopt `base` in the workbench: declare the dependency and take the plain swaps

Neither `idris/mtg.ipkg` nor `idris/mtg-dev.ipkg` declares `depends = base`,
and the workbench files import nothing but what `idris-gates-reflect-with-so`
added. Everything below is a hand-rolled copy of something `base` 0.8.0 ships,
found by an audit that read each definition against the library source. Only
the items the audit rated NO REASON are in scope; the `sameX` equality family
(`idris-eq-instances-for-sameX`), the `any`/`all`-shaped recursions (left-fold
elaboration cost, unmeasured), and everything rated GENUINE REASON stay as they
are.

## The change

Both `.ipkg`s: `depends = base`. Then, in `idris/src/Experimental.idr` (E) and
`idris/src/Experimental/Words.idr` (W), replace and delete:

- `leNat` W:255, `ltNat` W:2477 → `Data.Nat.lte`/`lt` (byte-identical bodies);
  `eqNat` W:287 → `==`.
- `isNil` E:5011 → `Data.List.isNil` (same name and body).
- `AtLeastOne`/`OneUp` W:318 → `Data.Nat.IsSucc`/`ItIsSucc`.
- `modeCount` E:3843 → `length`.
- `nameUnwritten` E:2644, `ptWritten` E:2614, `ptPrinted` E:5471 →
  `Data.Maybe.isNothing`/`isJust`.
- `lastType` W:2719 → `Data.List.last'`.
- `optCT` E:245 → `toList` at `Foldable Maybe`.
- `negTypes` E:725, `negZones` E:502, `headTysJoin` E:264, `condMintAll`
  E:2355 (all `f p ++ rec ps`) → `concatMap`.
- `manaRunWritten` W:1436 → `Data.List.NonEmpty`; `loyaltyStepWritten` W:1478
  → `IsSucc`. The other non-emptiness gates in that group are
  structure-specific and stay.

Line numbers are from the audit and will have drifted; find by name. Several of
these feed `So` gates, so the replacement must reduce at elaboration exactly as
the original did — `base`'s versions are `public export`, and the audit
confirmed interface methods already reduce inside gates here (`chapterMarksOk`
uses `Ord Nat`). If any swap changes elaboration (a pin stops failing, a witness
stops elaborating), keep the original for that site and list it.

Also in scope, same shape as the finished `So` migration:
`idris/src/Experimental/Events.idr` carries 5 `= True`/`= False` gates; reflect
them through `So` under the same rules, R5(c) included (a wrapper whose
constructor's index is relied on for inference stays a `data`).

## Consumption boundary

`idris/mtg.ipkg`, `idris/mtg-dev.ipkg`, `idris/src/Experimental.idr`,
`idris/src/Experimental/Words.idr`, `idris/src/Experimental/Events.idr`. No
Rust crate is touched.

## Acceptance

- `depends = base` in both `.ipkg`s; none of the named definitions remains
  except those listed with an elaboration reason.
- No `= True`/`= False` gate remains in `Events.idr`.
- `idris/scripts/build` PASS from a clean `build/`, no witness lost, no pin
  silently passing (break one pin, watch it fail, restore).

Standard constraints apply.
