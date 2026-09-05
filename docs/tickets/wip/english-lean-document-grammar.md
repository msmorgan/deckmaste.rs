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

## Landing record

Implemented in change `pkytsmpy`.

### Prove

- Document rules compose in the same recursive Syntax and grammatical and
  realization judgments as phrases. A quoted Document can fill a lexical
  frame; the nested quotation witness uses that path.
- Annotated Surface atoms preserve word/punctuation binding, symbols and line
  breaks. `Spells` connects them to exact strings, and `Written` connects
  textual admissibility to grammar independently of selection. Keyword
  suffixes bind structurally, including in a second grammatical host.
- Typed witnesses cover ordinary/activated/triggered text, paragraphs, costs,
  bare/parameterized/bound keywords, labels, modes, reminders, quotation,
  type lines, chapter/Class/leveler/Case/die/Station sections and independent
  face/door Documents. Exact-text assertions cover punctuation, casing,
  adjacency and line breaks. Negative proofs reject empty sentence surfaces,
  nested reminders and reversed type-line groups.
- Lean LSP MCP reports the English root clean. Axiom audits of
  `English.Documents.nested_quote_text` and `nested_reminder_rejected` report
  only the standard axioms (`propext`, `Classical.choice`, `Quot.sound`), with
  no source warnings. After refresh, `cd lean && ./scripts/build` passed all
  English and existing Semantics targets (64 jobs), with warnings treated as
  failures. The ticket graph check is clean.

### Disclose

- Added 27 theorem assertions plus concrete typed document witnesses. All 47
  pre-existing theorem statements/outcomes remain checked; their surface
  values now use annotated word atoms. Sixteen existing theorem declarations
  were directly re-spelled with Surface type annotations (13 composition and
  3 initial lexical witnesses). Restored 0; ignored 0; removed 0.
- Deviations and additions: exact spelling exposed quotation terminal-period
  ownership, handled by a shared sentence-finishing operation; reminder
  nesting has an explicit structural check. Both are within the ticket's
  punctuation and embedding scope. No executable parser or full renderer.
- Added linguistic glossary entries for Document, Keyword Line and Notation.
  No CR claims or citations changed. No STOPs or existing-test regressions.
  Other WIP tickets were untouched.

### Report and limits

Production code/data are unchanged. The source-map audit records a decision
or a concrete review question for each document family: full numeral/range
structure, source whitespace, inline reminder fragments, keyword-specific
separators, template population and face/layout metadata are not silently
claimed complete. Examples are synthetic. Exact Lean string spelling is not
an automatic proof of production byte roundtrip or corpus conformance.
