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

## Landing record

Implemented in change `rvvnltsx`.

### Prove

- One recursive Syntax and mutual grammatical judgments compose phrases,
  clauses and ordered gap resources. Closed derivation remains independent of
  realization and selection. General coordination replaces the initial
  dedicated constructor through a transparent abbreviation.
- Lexical frames carry grammatical relations and fixed lexical markers;
  determinative, nominal/NP, PP, VP and clause categories compose through
  explicit productions. Finite licensing is declared by the overt head;
  morphology, agreement, voice and polarity remain separate.
- Cross-capability structural and surface witnesses cover relative + auxiliary
  + adjunct, coordinated frame complements, the identical NP in three
  grammatical relations, quantities, arithmetic, comparison, passive selection,
  nonfinite clauses, ellipsis and a shared coordination gap. Negative twins
  inspect agreement, marker identity, frame arity and gap closure.
- After refresh, `cd lean && ./scripts/build English` passed (6 jobs), with
  warnings treated as failures. Lean LSP MCP checked the root and edited files,
  exposed elaboration errors during development, and checked the resulting
  proofs. LSP axiom audits of `relative_derives` and `perfect_rejects_passive`
  report only standard `propext`, with no source warnings.

### Disclose

- Added 38 theorems (including the frame-arity helper); re-spelled 4 existing
  proofs, preserving all 9 initial theorem statements and asserted outcomes.
  Restored 0; ignored 0; removed 0.
- Added glossary definitions for the linguistic categories/features used by
  this expansion. Determiner remains a function; Cardinal Numeral and
  Determinative Phrase are the grammatical categories. No CR claims or
  citations changed.
- Deviations and additions: passive/perfect selection and invariant-modal
  agreement received explicit witnesses when design exposed the risk of
  conflating form with voice or agreement. These are within the mapped scope.
- No STOPs or existing-test regressions. Other WIP tickets were untouched.

### Report and limits

No production source/data changed and no production coverage, selection or
performance claims are made. The design map records explicit composition
scope decisions and review challenges for extraction islands/relative forms,
countability and modifier order, mixed agreement, bare temporal NPs, and
antecedent accessibility. The review ticket now names those inputs. The model
is a formal composition proposal under stated lexical assumptions, not a
proof of whole-grammar adequacy. Witness wording is synthetic.
