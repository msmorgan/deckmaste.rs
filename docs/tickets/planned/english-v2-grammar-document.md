---
needs: [english-v2-grammar-migration-design, english-v2-relative-clause, english-v2-grammar-measure-phrases]
---
# Migrate documents, keyword hosts and printed templates

Replace game-semantic envelopes/cost categories with flat typed document,
paragraph, sentence, modal, keyword and cost collections as in
`DocumentShape`, `DocumentCollections` and `BoundaryInteractions`. Ordinary
and activated text share the same body. Trigger-shaped initial adverbials are
ordinary clause structure; preserve condition order and punctuation without
encoding game execution. Retire `ActivationCostComponent`, `AdditionalCostBody`,
`ControlledCostAction` and remaining semantic cost wrappers. Printed cost/mana
notation and the colon-bearing document structure remain grammatical surfaces.

Canonicalize singleton/list arities; no generic Document coordination. Own
nested sentence quotation, literal-word quotation, sentence-final supplements,
inline clause/fragment Parentheticals, comma/semicolon keyword lines, granted
lines mixing keywords and quotations, named/batch/parameterized/bound keyword
surfaces, and textual labels/chapters/levels/classes/cases/rooms/die tables/
stations/modal weights. Preserve exact bars, dashes, ranges, symbols and separate
text boxes; do not validate physical layout or game legality. Extend the Lean
surface/collection witnesses for forms beyond its checked fragment.

Delete `KeywordSubjectModifier`'s five-member enumeration. Keyword hosts take
the same nominal/reference stages, postmodifiers, participles and relatives as
ordinary hosts, with only declaration-backed host distribution. Verify
`Enchant creature with flying`, `... without flying`, `... without a counter
on it`, stacked modifiers and subject-gap relatives. Replace the inherited
vacuous F7 `VerbLexeme` assertion with exact membership of the irregular verb
inventory actually supplied. Preserve bound-keyword casing and quality ownership.

Acceptance includes multi-sentence trigger-shaped paragraphs, ordinary and
activated modal bodies, empty permitted text collections, exact singleton and
multi-item keyword forms, nested quoted sentence termination, supplementary
comma termination and fragment reminders. Include the keyword/template rows
and level-range `1-9 |` witness in the register. Document parenthetical/quotation
boundaries expose grammatical context hooks for the later ellipsis ticket;
they do not consume Semantics. Source evidence: style-guide §§2–3, 8, 14–15.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
