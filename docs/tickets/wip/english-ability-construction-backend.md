---
needs: [english-derived-family-inventory]
---
**Derive the handwritten English ability layer.**

Extend the construction compiler with the backend required by the `Cost`,
`KeywordLine`, and `Ability` families inventoried by
`english-derived-family-inventory`. Keep the decision's same declaration
contract, `(ast, surface)` laws, explicit dominance, surface-witness handling,
and sealed build ingress; this is not a second ability-specific grammar.

Migrate each inventoried ability family as an atomic vertical slice: generated
parse/reduction/lowering, render, and build projections; `inspect` provenance;
affected frame/spelling and serialized consumers; one registry-owner flip; and
deletion of every handwritten mirror. A backend capability may land with its
first family, but generic machinery without a migrating family does not close
the ticket.

Gates: both construction laws and negative combinations cover every migrated
family; direct AST and `inspect` fixtures preserve intended selection and
ambiguity; registration permutation is semantic-neutral; round-trip remains
clean; structural audit finds no handwritten ability-family authority or
validation bypass.

Standard constraints apply.
