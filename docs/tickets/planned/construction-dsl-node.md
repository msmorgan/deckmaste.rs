---
needs: [construction-core-final-constituent-literal-opacity]
---
**Rename the construction DSL's `element` declaration to `node`.** In this
DSL, the declaration always emits an AST product struct; graph-theoretically it
is a node. `Element` says nothing useful, while Construction Grammar's
`Construct` denotes a token instance licensed by a Construction rather than the
AST product type declared here. Keep [`Construction`, `Category`,
`Constituent`, and `Production`](../../contexts/oracle-english/CONTEXT.md) for the grammatical
schema, class, part, and
grammar-rule concepts: each DSL `form` is a production or linearization
alternative, and the construction identity supplies the category variant.

Rename the declaration, parser token, diagnostics, generated-code vocabulary,
examples, and tests after the final-constituent/literal-opacity WIP lands. The
serialized declaration language may break cleanly; do not retain `element` as
an alias. Acceptance confirms that every `node` declaration still emits one
product struct and that `form` retains its existing grammatical job.
