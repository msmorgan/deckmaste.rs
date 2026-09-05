---
needs: [english-lean-grammar-composition]
---
# Extend the formal model to Oracle documents and notation

Complete the document and surface portion of `lean/English/` under the
[Lean design decision](../../decisions/english-lean-design-workbench.md).
Use the scope map established by `english-lean-grammar-model`, consulting style
guide §§1–3, §8, §14, and §15 for editorial structure, ability boundaries,
keywords, and frame-dependent text. Include the mapped type-line order,
symbols, quoted text, labels, modes, and parameterized/bound keyword surfaces.

Express these structures using the general phrase/clause grammar and explicit
notation or document rules. Preserve distinctions required for exact surface
realization without importing semantic validation into English. Resolve each
unimplemented scope-map entry or identify the concrete design question it
leaves for the design review; a currently failing card does not alone define a
new grammatical category.

Acceptance: typed document witnesses and relational surface derivations cover
each agreed document family, including nested/quoted grammar and lexical
parameters in more than one host. Audit the source map for omitted families;
record decisions, not a corpus percentage. Build `English` and the existing
Lean targets without proof placeholders. No automatic translation to Semantics,
card validator, executable renderer, or production integration is required.
Standard constraints apply.
