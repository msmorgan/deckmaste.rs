---
needs: []
---
# Pins refuse rules-impossibility only

Ruling (2026-08-22): the semantics layer accepts anything the Comprehensive
Rules make meaningful. A pin (`Unspellable … impossible` in
`idris/src/Experimental/Proofs*.idr`) refuses a term only when a rule makes it
meaningless — a category error or a contradiction — and its docstring names
that rule. "No card has written this" is never a reason to refuse: what English
prints is spelling-boundary knowledge, and a restriction that leaves a printed
card unrepresentable is the worst outcome. Authority:
`docs/memory/rulings/measurements-live-in-pins.md` and the overgeneration
clause of
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md).

The workbench has ~717 pins, many minted on the opposite doctrine ("a measured
zero is a pin"). This ticket reverses that.

## Phase 1 — classify every pin

For each pin, one of:

- **RULES** — the docstring names a CR rule under which the term is
  meaningless. Keep; verify the cite points the direction the claim needs.
- **STRUCTURAL** — the term is meaningless by the model's own algebra (a
  duplicate card type in a set; an order rider on a two-end disjunction; a
  verb that needs an agent used without one; `Quality \/ Outcome`). Keep; cite
  the rule if one exists.
- **CORPUS** — the only justification is a count or an absence ("0 of 440
  lines", "no card writes it", "single witness"). Widen the gate behind it
  and delete the pin.

The table (pin · file · class · rule or reason) lands in this ticket.

## Phase 2 — widen and delete, per file

For every CORPUS pin: admit the cell in the gate table it exercises, delete the
pin, and keep `idris/scripts/build` green. Where the gate was a constructor
index or a trimmed sum, restore the arm with the rules-correct shape (a
duration the CR states, an agent slot) rather than just admitting. Bench any
printed card the refusal had left out — Luxior first. Named first cases:

- `attachHeadOk Equipped PermanentW` → admitted ([CR#301.5]: creatures
  "unless an effect says otherwise"); `Equipped`/`Enchanted` are words for
  `AttachHost`, not constructors.
- `comparableBound` → a summed bound at every comparator.
- `admitsSpan CostModification` → admitted; `SpanUse` row renamed.
- `visibilityOk LookAt WholeHand` → admitted; its backwards [CR#402.3] cite
  fixed.
- `ConferringWord` → `SaddleW`/`AscendW`/`StoriedW`/`RenownW` restored with
  the durations [CR#702.171a,702.131a] and static shape [CR#702.195a] the CR
  states; `GainsDesignation` grows what it needs to carry them.
- `badPutIntoExile`, `badSweepAcrossPlayers`, `badSweepAcrossOpponents`,
  `badGroupCommander`, `badNoHolderNegated`, `badBeforeAttackersEachPlayers`
  → deleted with their gate cells admitted.

This lands before the union-family redesign so its pins are classified once.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Events.idr`, `idris/src/Experimental/Macros.idr`,
`idris/src/Experimental/Proofs*.idr`, `idris/src/Experimental/Cards.idr`. No
Rust crate is touched.

## Acceptance

- Every surviving pin is RULES or STRUCTURAL and its docstring names why; no
  pin's justification is a count.
- Every CORPUS pin's gate admits; Luxior and any other newly representable
  printed card is benched.
- `idris/scripts/build` PASS; `Cards.idr` still binds no implicits; cites 0/0.

Standard constraints apply.
