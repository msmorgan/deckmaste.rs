---
needs: []
---
# Give every closure row a recorded verdict, region by region

[The closure tables](../../idris-workbench-closure-tables.md) inventory **671
rows** across the three workbench sources — every closed catalog, measured-zero
gate, closed-count table, single-value slot, canonicality gate and marked union
construction — each tagged by what closes it, plus **66 single-value slots**
called out separately as the sharpest overfitting surface. They are a
measurement, not a defect list, and today none of them carries a verdict.

This is an **investigation**, and its output is a decision rather than a diff.
For each row, exactly one of three verdicts, recorded in the docstring at the
site:

- **widen** — the closure is thin and the axis should open now; route the work to
  the family ticket that owns the site, and record the routing here.
- **bless-as-permanent** — a rule shuts the set, so state the rule in the
  docstring and mark the row as not-to-be-re-measured. This is the only verdict
  that ends the row's maintenance.
- **re-measure** — the recorded evidence is stale, its denominator is unstated,
  or the count predates a chapter that moved the construction; take the number
  again and record it with its finding.

A row that already carries a rule and a stated discipline needs only the marker.
The point is that a reader can tell, per cell, whether the zero beside it is a
rule, a measurement, or an accident — which is exactly what the closure tag was
introduced to buy and what nothing today spends.

Authority:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
— a measured phrasing fact is table content, not type-level law. A verdict that
widens must move the measurement into a table, never delete it.

> **Stale anchors (2026-08-21):** the four region boundaries below and the
> closure tables' header line totals predate a large shrink —
> `Experimental.idr` is ~5,500 lines, `Words.idr` ~3,200, `Events.idr` ~1,100;
> three of the four regions no longer exist, and the tables' §7 line refs
> exceed every file. Re-anchor against the tree before partitioning the sweep.

## Do not drop a measured zero

The tables' rows exist because a zero was counted. Carrying a row forward without
its count, or blessing a cell whose only support is an uncounted grep, turns a
measured zero into a silent yes. Every verdict must name the evidence it rests
on, and a row whose evidence cannot be found is a **re-measure**, never a bless.

## Scope, by region

Work it region by region; each is independently claimable and each ends with
every row in it carrying a verdict.

| region | rows | tables §  |
|---|---|---|
| `Experimental/Words.idr` | 166 | §2.1 |
| `Experimental/Events.idr` | 61 | §2.2 |
| `Experimental.idr` 1–4040 (zones, predicates, nouns) | part of 444 | §2.3 |
| `Experimental.idr` 4040–7770 (amounts, conditions, events, tokens) | part of 444 | §2.4 |
| `Experimental.idr` 7770–11600 (`StaticEffect`, `Cost`, the `Effect` head) | part of 444 | §2.5 |
| `Experimental.idr` 11600–17081 (`Effect` tail, projections, `Card`) | part of 444 | §2.6 |
| single-value slots, all three modules | 66 | §3 |

Distribution by closure basis, which is the triage order — printing-backed rows
are the ones a printing can move and therefore the ones a verdict buys most:

| basis | Words | Events | Experimental | total |
|---|---|---|---|---|
| printing-backed | ~63 | ~34 | ~202 | **~300** |
| CR-backed | ~35 | ~4 | ~95 | **~130** |
| CR-closed | ~22 | ~6 | ~81 | **~110** |
| structural | ~42 | ~12 | ~69 | **~120** |

The bases sum slightly above the row count because compound rows carry a dominant
label. `slot` and `table` dominate by kind, `catalog` and `canon` follow, and
`count` is the smallest class (roughly two dozen closed `Nat` tables); the one
region with a full per-kind tally is `Experimental.idr` 1–4040 — slot 46, table
35, canon 17, catalog 5, union 3 of 106.

## What this ticket does not decide

- The **fifteen top-ranked flip risks** and the **ten large attestation grids**
  below them have their own ticket, `workbench-closure-flip-risks`, which carries
  their widening costs and routes the grids to their owning families. Do not
  re-verdict them here; mark them as owned and move on.
- The **ten rows carrying the `union` tag** — `Payload`/`UnionP`,
  `wordReaches JoinW`, `JoinedPlayer`, `JoinedClass` (Words.idr); `KindJoin`,
  `bindFor`, `YouAnd`, `nounIsKindJoin`, `nounIsMixedGroup`, `DamageRecipient`
  (Experimental.idr) — are the union family's, and their measured gates move
  verbatim under `workbench-union-gate-spelling-rehome`. A verdict here would
  pre-empt that move.
- Claims with **no count behind them at all**, and the **seven live
  discrepancies**, are `workbench-docstring-evidence-audit`'s. Where this review
  finds a row whose evidence is missing rather than stale, route it there and
  record the routing.
- A verdict that **refutes a cell** is not this ticket's to fix. Route it to the
  family ticket owning the cell and record the number here, exactly as the
  evidence audit does.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Events.idr` — docstrings and tables. Evidence bench
`idris/src/Experimental/Cards.idr` only where a re-measurement changes an
admission. [The closure tables](../../idris-workbench-closure-tables.md) are
updated in step: a verdict recorded at a site is also recorded in the row.

## Acceptance

- Every row in the region claimed carries one of the three verdicts, in its own
  docstring at the site, with the evidence the verdict rests on named.
- No cell is flipped on the strength of a verdict alone: a widening lands with
  its witness or its pin, and the measurement moves to a table rather than being
  dropped.
- Every routed row names the ticket it was routed to, and every number found
  stale is re-measured rather than reconciled by arithmetic.
- The closure-table rows for the claimed region match the docstrings after the
  pass.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
