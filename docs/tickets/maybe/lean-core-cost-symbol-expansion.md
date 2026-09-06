---
needs: []
---
# Revisit cost-symbol expansion only with a simpler representation

Parked by the user on 2026-09-06: leave `Cost.tapSymbol`, `untapSymbol`,
`loyaltySymbol`, and their payloads as they are for the time being. Reducing
constructor count does not justify overcomplicating the model. This work was
removed from `lean-core-actions` and is not a prerequisite for the
planned action folds.

The intended direction remains that a loyalty cost is a macro over putting
or removing loyalty counters, including zero and X [CR#107.7]. Tap/untap
symbol expansion over ordinary payment actions is also a deferred candidate.
Neither expansion is an implementation commitment while this ticket is parked.

Before promoting this ticket, demonstrate a representation that reduces
complexity while preserving the symbols' semantic distinctions:

- A loyalty-symbol cost classifies the enclosing ability as a loyalty
  ability; ordinary loyalty-counter removal does not. Heart of Kiran is the
  counterexample to inferring that classification from the payment alone.
- Tap/untap-symbol origin carries the applicable creature continuous-control
  restriction; a written-out tap payment does not supply that property.
- Source, counter kind, amount, direction, X, and payment obligations remain
  expressed by the payment body and cost context where possible.

The earlier proposal for general macro-property propagation is a design
candidate, not required infrastructure. Retaining a macro name in
`Authoring.Form.call` does not itself propagate semantic properties, and
name-specific checker guards are not a substitute. Reopen with concrete
expansions, the minimum required metadata, and a comparison against keeping
the current constructors; obtain a design ruling before implementation.

Standard constraints apply.
