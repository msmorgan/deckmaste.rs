---
needs: [english-v2-grammar-migration-design, english-v2-concord-class, english-v2-number-feature-unification]
---
> **Migration routing (2026-09-05).** This unclaimed ticket waits on
> `english-v2-grammar-migration-design` under the
> [Lean design decision](../../decisions/english-lean-design-workbench.md).
> That design task must reconcile and repin this ticket before it becomes
> executable. The prior body below preserves examples, regression and
> re-coverage obligations, and proposed mechanisms; its old sequence,
> implementation prescriptions, and coverage-ratchet acceptance do not
> override the new design process or the current landing contract.

**Strengthen English agreement to explicit Person and Number features.** Build
on the weak Concord Class split using the [`Person`, `Number`, `Agreement`, and
`Concord Class` definitions](../../contexts/oracle-english/CONTEXT.md). Carry underlying Person and
Number on the agreeing nominal/pronominal values and derive the morphological
Concord Class used by verb realization.

Update construction constraints, coordination behavior, pronoun realization,
provider rows, parser consistency checks, and direct AST tests. Cover at least
*you cast*, *a player casts*, *players/they cast*, and the supported *was/were*
contrasts. Keep Person and Number available for later pronoun and possessive
agreement rather than collapsing them back into the two-way derived class.
