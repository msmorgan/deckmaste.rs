---
needs: [core-regions-substrate-closeout]
---
**Stage 2 of [Core is explicit regions](../../decisions/core-explicit-regions.md):
the discourse channel becomes defs, loops and predicates become regions,
and the engine's anaphora record is deleted.** Standard constraints apply.

## Scope

- Instructions with dests replace the binders: `Choose { dest, by,
  quantity, filter }`, `ChooseValue { dest, by, domain }`, `Search { dest,
  by, whose, from, quantity, filter, if_none: Block }`, `Act { dest:
  Option<DefId>, action }` (the `Produce` binder's product [CR#400.7]),
  `RevealUntil { found, passed, whose, matches, body }`, `Let { dest, expr }`
  for a pinned evaluation moment [CR#608.2h]. `With`, `Noting`, `Label`,
  and the `Binder` enum are deleted; a `With` body flattens into the
  enclosing block after its defining instruction.
- Loop bodies are regions: `Each { over, body: Region }` with the element
  as the body's param; `Distribute { amount, over, body: Region }` with
  element and share params. `Repeat`/`Batch` keep the same activation.
- Predicates are regions with a candidate param: `SelectAll`, a
  `TargetSpec` filter, `Where`, `Pick`, `Aggregate`, and continuous
  per-subject reads. Nested predicates have distinct params, so an inner
  predicate may read an outer candidate; the depth-0 restriction is gone.
- Deleted from core: `It`, `That(Sort)`, `Selection::They`/`Them`/
  `AmongNoted`, `Count::ThatMany`/`ThatMuch`/`Allotment`, `Binder`,
  `Noting`, `Label`. A "where X is …" definition lowers to a `Let` at the
  head of the body.
- Engine: the residual discourse record and `Frame.anaphora` are deleted;
  `Frame` is the activation id plus payment state. The `BindChoice`
  continuation writes the deciding instruction's dest directly; no shared
  `chosen` slot, no presence-flag tests, no clearing discipline. Magnitudes
  are defs, so `GameState.that_much` is deleted.
- Lowering: R1 (nearest compatible antecedent) and R2 (refuse on a second
  compatible antecedent) run here over the semantic term, in reading order,
  and emit per-card diagnostics with card context. The computed resolution
  is gate-compared against the Idris mirror's certification for the canon
  slice (the certifier/resolver split of `semantics-spelling-lowering.md`
  §17), and the resolver runs on every card at lower time, wizards included.

## Absorbed tickets (deleted 2026-09-02; their fixtures are gates here)

- `engine-chosen-slot-provenance`: a chooser nested under another chooser,
  under `Random`, or under `AmongNoted` resolves its own picks; there is no
  clear to omit.
- `engine-that-much-frame-scoped`: a card fixing two magnitudes reads the
  semantic antecedent; an `Each` over players followed by "that much" reads
  the per-element amount.

## Gates

Canon re-lowers green with the engine suites and the Idris re-emit gate;
the resolver/certifier differential is empty over the canon slice.
Fixtures: exile-and-return (target param, then a move-product def read by
the return); an insertion negative (a new producing clause between binder
and mention re-resolves at lowering, never silently rebinds); nested
`Where`-in-`Pick` reading both candidates; one multi-sentence anaphora
card from `parse-cross-sentence-anaphora`'s staged shapes; a `Let`-pinned
"that many" that is not re-evaluated after the state changes.
