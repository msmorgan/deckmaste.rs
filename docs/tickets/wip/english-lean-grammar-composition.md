---
needs: [english-lean-grammar-model]
---
# Model composition across phrases and clauses

Implement the general phrase and clause grammar in `lean/English/` from
`docs/english-grammar-design.md`, refining the model when explicit witnesses
expose a flaw. Follow the [Lean design decision](../../decisions/english-lean-design-workbench.md).
This is one composition task: do not serialize an implementation ticket per
card, subordinator, inflection, or adjunct/host combination.

Cover the mapped relationships among Nominal, Noun Phrase, Determiner,
Modifier, grammatical Subject/Object, lexical Verb Frame and its selected
Complements, Adjunct, agreement, inflection, finite/nonfinite Clause,
auxiliaries, negation, relative gaps, coordination, and ellipsis. Include
comparative and quantity structure. Keep grammatical features separate from
Game Model classification. Design lexical admissibility from linguistic facts,
with corpus attestation as evidence rather than the acceptance domain.

Add surface derivations and structural witnesses that combine independently
modeled capabilities: a relative body with auxiliary and adjunct, a frame with
coordinated complements, and an agreeing noun phrase in several grammatical
relations. Add near-miss exclusions with positive twins and explicit
assumptions. Select further witnesses from style guide §§4–7 and §§10–13 and
the wayfinder's inherited regression cases; verify corpus wording when used.

Acceptance: each mapped core capability is represented or has an explicitly
resolved scope decision; witnesses inspect structure, not mere inhabitation;
the combined examples use the general rules rather than new constructors for
their combinations. Update the design map and build `English` without proof
placeholders. No global ambiguity theorem or executable recognizer is required.
Standard constraints apply.
