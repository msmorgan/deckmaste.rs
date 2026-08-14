---
needs: [english-construction-rewrite-design, english-v2-catalog-pipeline]
---
**Hand-write the construction compiler's target output for the first vertical
slice of `english_v2`: five grammatical categories with exact bidirectional
round-trip on a fixed sentence set.** Stage 2 of the rewrite's implementation
sequence.

This ticket writes BY HAND, in `deckmaste_english_v2`, the code the future
construction compiler will generate — struct-per-construction, category
enums, exact renderer, minimal parser — so codegen has a golden target before
it exists. No DSL, no macros, no compiler work here.

Decisions already made (2026-08-13 design dialogue; the ADR produced by
`english-construction-rewrite-design` is the authority if anything here
drifts):

- Transcript AST: one struct per construction wrapped in its category enum
  (`pub struct DealDamage { amount, to }`;
  `VerbPhrase::DealDamage(DealDamage)`).
- Derive, don't store: verb agreement, capitalization, structural
  punctuation, and numeral spelling are computed at render from context; only
  non-derivable surface facts are stored.
- Byte-exact round-trip both ways: `render(parse(s)) == s` for every accepted
  sentence, `parse(render(v)) == v` for constructed values.
- Failure is loud: input the grammar does not cover is an error. NO
  recovered-text or raw-string nodes anywhere in the AST.
- Acceptance tracks grammatical Oracle English, not rules legality: sentence
  2 below is deliberately rules-invalid and must parse.
- Anaphora ("it", "that creature") are unresolved syntax; a `where` clause is
  a syntactic binder node (recognized and attached, never resolved); a
  variable is the same leaf value at every occurrence of the same name.
- Lexical atoms with internal structure store their parts (sign + magnitude),
  never fused strings. The word "hole" is reserved for the future macro
  feature; recursive constituents are plain typed roles.

Scope: categories `Ability`, `Sentence`, `Clause`, `NounPhrase`,
`VerbPhrase` (plus `Amount`), with exactly the constructions this sentence
set needs, each sentence round-tripping byte-exactly:

1. "Destroy target creature."
2. "Whenever a player connives, that creature deals X damage to it."
3. "You gain X life, where X is the number of creatures you control with
   power 2 or less."
4. One real corpus sentence (implementer's choice) using the card's
   self-reference — it must bind the self-name parse-context parameter plus
   at least one catalog through the catalog-pipeline loader, and store the
   abbreviated-vs-full spelling.

Tests: exact round-trip per sentence; a constructed-value render test that
never parses; a loud-failure test on ungrammatical input. Standard
constraints apply.
