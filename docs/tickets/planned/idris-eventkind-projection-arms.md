---
needs: []
---
**`idris/scripts/build` is red: five EventKind constructors have no projection
arms.** `Core.idr` declares `Mill`, `Scry`, `Surveil`, `Fateseal`, and `Fight`
as `EventKind` constructors (around `Core.idr:473`), but the projection
functions over `EventKind` were never extended to cover them, so idris reports
each as *not covering*:

```
actionEventCaps, costCaps, costsCaps, eventKindCaps, eventKindHasAmount,
eventKindObjectSort, eventQueryCaps, eventQueryHasAmount, kindsHaveAmount,
queryObjectSort, queryRoles
```

There is no `_` fallthrough in these functions (deliberately — the total
enumeration is the soundness gate, so a new kind must be handled explicitly,
[[idris-probe-soundness-gate]]), so the missing arms are hard build errors.

Landed by the `engine-act-fight-patient` / `idris-act-parity` trunk work, which
added the EventKind data and the `KeywordActionSpec` shapes (`Scry : Count b ->
…`, the Mill/Draw slice family) but not the `EventKind`-projection arms. Found
while refreshing [[parse-positional-target-reads]] onto trunk: that feature's
idris changes are orthogonal and green on their own base, but the merged
`idris/scripts/build` fails on these pre-existing gaps.

## What each arm needs (author's call, not a guess)

Each function reads a different facet of the kind; the values are real semantics,
not defaults:

- `eventKindCaps` → the `EventCaps` record (hasObject / hasActor / hasAmount /
  patientKind / …). Scry/Surveil act on YOUR library by definition (count only,
  no performer — the comment at `Core.idr:2197` already says so); Fateseal names
  the fatesealed player; Fight carries two fighter objects.
- `eventKindObjectSort` / `queryObjectSort` → the object sort the kind exposes.
- `eventKindHasAmount` / `eventQueryHasAmount` / `kindsHaveAmount` → whether the
  kind carries an amount (Mill/Scry/Surveil/Fateseal count; Fight none).
- `actionEventCaps` / `costCaps` / `costsCaps` / `eventQueryCaps` / `queryRoles`
  → the derived caps/roles, following the existing arms' pattern.

Fill each from the mechanic's actual rules (mill, scry, surveil, fateseal, and
fight — look up the current CR subrules via the mtg-rules skill and cite them in
the arms/comments) rather than copying a neighbouring kind.

Verify: `idris/scripts/build` green; `cargo xtask idris-check plugins/canon` runs
(no emitter regressions); the added arms match the `KeywordActionSpec` shapes the
same trunk work introduced.
