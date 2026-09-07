---
needs: [english-v2-lexical-analysis]
---
# Rebuild the English workbench around the v3 grammar relation

Rewrite the independent `english/` Lake project to describe the grammar that
English v3 will implement. English remains an NLP model with no import from or
interaction with Semantics. Preserve useful definitions and witnesses from the
current workbench only when they express the v3 design directly; compatibility
with its old `Analysis`, selection or packaging layers is not a goal.

Model declared Lexemes, licensed Word Forms and correlated Feature Bundles as a
non-contextual Lexical Analysis relation. Model grammatical admission separately:
Categories and Productions constrain agreement, lexical frames, countability,
voice, tense, extraction, coordination, recoverability and document boundaries.
Every admitted Reading remains available; optional preference is a separate
relation and cannot remove a Reading. Equal text does not collapse distinct
lexical identities or constituent structures, while duplicate derivations of
one Reading do not manufacture linguistic ambiguity.

Give the whole intended grammar one connected top-down shape. It must cover the
families in the reviewed source map: documents, sentences and clauses; lexical
predicates and frames; nominals, determiners and adjectives; subordination;
prepositions; relatives and extraction; coordination and sharing; measures;
contextual recoverability and ellipsis; Type Lines; and the Target Verb/Targeting
Marker rivalry. Each family needs at least one inhabitant, one exclusion where
the family has a grammatical constraint, and one interaction with another
family. This is grammar design, not a census claim or a license for permissive
catch-all productions.

State analysis-to-realization and independently-constructed-value roundtrip
relations without making either true by definition. Preserve declared spelling
variants and capitalization. Model the grammatical information that a parent
must inspect and the correlations that cannot be projected independently, but
leave Earley scheduling, chart indexes and packed-node layout to Rust.

Acceptance names every old workbench declaration kept, replaced or retired;
demonstrates noun/verb and noun/determinative homographs, agreement-sensitive
forms, a jointly invalid combination of individually valid alternatives, two
unrelated Readings of one surface, and an ambiguity-free control; and leaves no
v3 rule resting on the superseded uniqueness or destructive-selection policy.
Use Lean LSP MCP, `english/scripts/build`, and the standard axiom audit. Standard
constraints apply.

## Landing record

The v3 workbench now admits trees over declared, correlated lexical alternatives
through one grammar relation. Admission retains every Reading independently of
preference. The [workbench README](../../../english/README.md) records the model's
entry points, each required family's inhabitant/exclusion/interaction, and its
limits. The [declaration disposition](../../../english/DECLARATIONS.md) names all
789 declarations from the 30 original modules as kept, replaced or retired.

### Prove

Measured on change `lpmxsvro`, after a no-op Kata refresh:

- Lean LSP diagnostics passed for the changed definitions and witnesses.
- `english/scripts/build` passed with warnings as errors: 44 jobs.
- The compiled axiom audit imported `English`, enumerated every theorem whose
  public or private user name begins `English.`, and collected transitive
  axioms. All 2,400 compiled theorems, including private/generated declarations,
  use only `propext`, `Classical.choice` and `Quot.sound`. Source scanning found
  no proof holes or custom axioms. LSP `lean_verify` separately checked the
  genuine ambiguity and crossed-agreement exclusion witnesses.
- The declaration inventory has exactly 789 unique old-declaration rows; all
  1,029 named current source declarations were also resolved in the compiled
  environment. Documentation links were checked.
- The admission import closure consists of `Analysis`, `Readings`,
  `SurfaceRelations`, `Lexical`, `Dependencies`, `FeatureConstraints`,
  `CaseConstraints`, `Grammar`, `DocumentShape` and `Surface`. It contains no
  selection, scope-quotient or role-preference module. English imports only
  its own modules and `Init`; its Lake manifest has no package dependencies
  or Semantics interaction.
- The three valid *was/were* sentences survive at v3 admission. Their three
  formerly admitted crossed-form counterexamples now prove exclusion of the
  same wrong combinations. Distinct finite Word Forms have distinct values.
  Count/mass, lexical frame, selected voice/finiteness, relative Subject,
  shared-gap and recoverability constraints have connected positive and
  negative witnesses. The family map names the exact bounded claims.
- “I saw her duck” has both possessive-NP and pronoun-plus-VP Readings.
  “Artifact” is an ambiguity-free control over the entire admission relation
  at its Type category in that declared environment. Proof multiplicity is
  separate from grammatical-value identity; optional preference preserves
  both endpoints.

The workbench exception in `english-lean-design-workbench.md` applies: no Rust
or declaration data changed, and no production coverage census, lexical-loader
check, performance measurement or source-byte correctness claim is made here.
The lexical adapters test declared features; lexical identity comparisons occur
in fixture declarations, not word-specific grammatical licensing guards.

### Disclose

Named source-theorem accounting: 344 before, 472 after. Of the old theorems,
301 have unchanged proof text, 33 are re-spelled or replaced, and 10 are retired.
Restored: 0; ignored: 0. There are 148 new theorem names, including replacement
names (that count overlaps re-spelled old declarations). Every retired theorem
is named and justified in the disposition: the removed subjects are the global
pruning/package policy and the artificial Echo preference fixture. The lexical
counterexamples are repaired wrong analyses, not deleted regression evidence.
All other old linguistic witness outcomes remain under their stated contracts.

Deviations and additions:

- Added a common declared lexical environment and connected family witnesses;
  retained reusable generic schemas without preserving the old `Analysis`
  interface. Old lower-level witnesses remain bounded schema evidence rather
  than being relabeled as proofs of full v3 admission.
- Renamed the two other public `Reading` carriers to
  `GrammaticalScope.SchemaWitness` and `Scope.AnchorPattern`. Added the glossary
  terms Lexical Environment, Morphology and Capitalization. These resolve part
  of the already-scheduled proof audit; no other claimed ticket was edited.
- Added a shared-gap multiplicity/category constraint and an explicit nonfinite
  use constraint so ordinary unary wrapping and a finite lexical use cannot
  impersonate the corresponding constructions. Added capitalization evidence
  to lexical values and preserved it through document surface relations.
- Replaced the artificial Echo ambiguity fixture with “I saw her duck” after
  the user challenged its linguistic meaning. No Echo-based natural-language
  ambiguity claim survives.
- Removed the provisional executable renderer and its implementation checks
  after the user clarified that the workbench's purpose is proving properties
  of the proposed grammar structure. The two requested roundtrip obligations
  remain statements over independently supplied operations. No parser or
  renderer implementation is delivered, and neither roundtrip is claimed as
  an implementation theorem.

No unresolved ruling contradiction or regression STOP remains. No glossary gap
used by this landing remains. The separate `english-v3-lean-proof-audit` ticket
retains its inherited obligations, including flat serial-comma coordination,
fuller tense propagation, genitive countability, extraction/anchor projections,
payload boundaries and broader nonvacuity challenges. This ticket does not claim
that the entire Oracle grammar or those subsequent audit obligations are proved.

### Report

This is a Lean grammar-model landing. Corpus counts, construction counts,
selection census, vocabulary overlap inventories and Rust throughput are not
measured or claimed under the workbench exception. The evidence is the named
structural proofs, declaration dispositions, LSP checks and complete Lake build.
