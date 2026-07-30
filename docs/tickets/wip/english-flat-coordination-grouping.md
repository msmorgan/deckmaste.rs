---
needs: [english-lint-expressiveness]
---
**Flat coordinations mixing `and` with `or` cannot express their grouping — 9
supported faces.** A single member list carrying both connectives cannot
distinguish `(A or B) and C` from `A or (B and C)`; the grouping is simply
absent from the node, so every consumer re-guesses it.

Measured 2026-07-29 by `cargo xtask english lint --check mixed-conjunction`: 9
faces, down from 13 before `english-exception-rider-quoted-conjunct` landed —
that work fixed four, which is evidence the remainder are the same kind of
defect rather than an intended flat modelling.

- **Elspeth Resplendent** — `Put a +1/+1 counter and a counter from among
  flying, first strike, lifelink, or vigilance on it.` parses as one flat list
  mixing `and` with the `or`-run, with `on it` attached to `vigilance`. The
  `or`-run belongs inside `from among`; the `and` joins two counter NPs.
- Also fires on `CoordinatedIndependentClause.rest` and
  `Coordination.junctions`, so this is not confined to nominals.

`NounPhrase` already has a `Coordinated` variant a member can hold, so nesting
is representable — the fix is likely parser-side rather than an AST change, but
confirm against the real enum shape before assuming it.

Regression gate: `mixed-conjunction` to 0. Standard constraints apply.
