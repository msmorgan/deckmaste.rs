Make keyword-line admission generative (parameter-landing-review F1-F3).
The landed design welded cost-shape categorization into parse admission:
abstract sum KeywordLineItem grew to 12 members, one per payload-vector x
separator-layout pair with hardcoded separator literals, so admissible
variants ("Ward—{2}", "Cycling—{2}", "Reinforce 3 {1}{G}") reject. Built
faithfully from a pre-amendment ruling text; the amended ruling: admission
admits separator variants (space or em dash with a mana cost; em dash
with a sentence-shaped cost; a sentence cost without a dash stays
ill-formed — the dash is a constituent boundary there).

- F1: introduce the separator as a constituent (a KeywordCostSeparator
  sum), collapsing the 12-member cross product. IMPORTANT STOP FENCE:
  construction_core has no parse-many/render-one canonicalization
  mechanism; if the fix appears to require building one, STOP and report
  the design question before writing compiler capability. Byte-exact laws
  on attested (house-styled) units must hold throughout; expect near-zero
  coverage change — this ticket buys generative fidelity and structural
  economy, not units.
- F2: KeywordSubject takes the general reference-phrase category — the
  landed fix added a third enumerated member (Coordination) instead; 13
  Enchant units still fail on postmodifiers the ordinary reference stack
  already parses.
- F3: 10 authored parameter vectors are inert ([Ability] x8, [Condition],
  [Cost, Power, Toughness]) with no consuming codec, and ParameterType is
  an unvalidated String — validate declared types against the consumable
  set so an inert vector is a load error, and either consume or explicitly
  defer the three shapes (deferral = a named unsupported class, not
  silence).
- Carried loose ends: planar-controller stub note and the vacuous
  VerbLexeme assertion (xtask report.rs:799) from earlier reviews.

Stub-side separator/layout fields remain banned (the relapse test stays).
Standard constraints apply.

## Landing record

Measured on change `xuqqprqv` with 16,401 covered lock identities. The
measured lock SHA-256 is
`34edbca953d57d086520893d8da36f4f29ac771f5d6d982413aabb3610a2aff4`.

- Coverage: the ticket baseline was 16,337 selected and covered units. The
  required refresh incorporated the concurrent adjunct-class change, whose
  measured parent has 16,385; this feature has 16,401. Thus this feature's
  lock diff against its refreshed parent is **+16/-0 rows**; the complete
  ticket-baseline-to-measured-tree movement is +64/-0, comprising the
  adjunct change's recorded +48 and this change's +16. Parse failures are
  16,304 at the ticket baseline, 16,256 at the refreshed parent, and 16,240
  after this change. Selected-uncovered units, unresolved ties, internal
  failures, exception uses, round-trip mismatches, ownership failures, gaps,
  overlaps, synthetic claims, and provenance-plan mismatches are all zero.
- Selection census: unique selections are 10,843 -> 10,962 -> 10,967 and
  specificity-resolved selections are 5,494 -> 5,423 -> 5,434 for ticket
  baseline, refreshed parent, and measured tree respectively. The feature's
  +5 unique/+11 specificity movement is exactly its +16 selected units.
  Exceptions and unresolved ties remain zero throughout the final feature
  measurement.
- Structural census: construction declarations are 384 -> 388 -> 393 across
  the same three trees. Relative to the refreshed parent, nine payload/layout
  cross-product constructions and the bespoke relative-subject construction
  were replaced by seven shared cost/separator constructions and eight
  reference/modifier constructions, net +5. Literal/lexicon collisions stay
  at the refreshed parent's 59 (ticket baseline 60).
- Separator admission: `KeywordCostSeparator` stores `Space` versus `Dash`
  as a real AST constituent. Space accepts only a symbol-run mana cost; dash
  accepts mana, sentence-shaped, and mana-plus-sentence costs. This admits
  `Cycling—{2}`, `Ward—{2}`, and `Reinforce 3 {1}{G}` while preserving exact
  rendering, and rejects `Ward Pay 3 life.`. No parse-many/render-one compiler
  capability was added, so the F1 STOP fence was not crossed.
- Subject admission: `KeywordSubject` now has a general `NounPhrase` adapter
  plus singular bare-nominal adapters for the determinerless keyword surface.
  Its local modifier category delegates to the existing relative,
  prepositional, scalar, object, and keyword-ability phrase categories. No
  PP, adjunct, or general noun-phrase construction was edited by this change.
- Parameter admission: `ParameterType` is a closed eight-member enum. The
  seven consumed keyword signatures have named codec classes; `[Ability]`,
  `[Condition]`, and `[Cost, Power, Toughness]` have named unsupported classes.
  Any other type or vector is a load error, and declaration-term codec
  parameter names are validated through the same closed vocabulary.
- Gates after the final changed refresh: `cargo fmt --all -- --check`; strict
  all-target Clippy for `deckmaste_construction_core`,
  `deckmaste_english_v2`, and `xtask`; the five keyword-line tests; both
  builtin-v2 integration suites; both parameter/codec validation tests; all
  twelve xtask report tests; `cargo xtask english_v2 ambiguity --json
  --require-resolved`; and `cargo xtask english_v2 coverage --check` exited
  zero. The corpus gates emitted only the busy-host common-path performance
  warning.
- Assurance census: restored 0; re-spelled 5 existing test functions; ignored
  0; added 3; removed 0. The additions pin the mandatory sentence-cost dash,
  closed-or-explicitly-deferred keyword signatures, and the ten authored
  unsupported declarations. Existing tests gained both separator spellings,
  general subject postmodifiers, unknown codec types, the planar-controller
  omission, and the corrected VerbLexeme assertion.
- STOPs: none.

### Deviations and additions

- The ticket expected near-zero coverage movement, but F2 produces sixteen
  whole-unit unlocks: ten Champion lines (`Boggart Mob`, `Changeling
  Berserker`, `Changeling Hero`, `Changeling Titan`, `Lightning Crafter`,
  `Nova Chaser`, `Supreme Exemplar`, `Thoughtweft Trio`, `Unstoppable Ash`,
  and `Wren's Run Packmaster`) and six Enchant lines (`Roots`, `Runner's
  Bane`, `Spellweaver Volute`, `Threads of Disloyalty`, `Twisted Embrace`,
  and the `Strangling Grasp` face of `Vengeful Strangler`). The +16/-0 lock
  diff authenticates that this is additive rather than replacement coverage.
- The general reference adapter alone does not express the established
  determinerless keyword surface, so singular bare and locally postmodified
  nominal adapters were retained. Their singular guards prevent duplicate
  singular/plural declaration readings; final unresolved ties are zero.
- F3 adds public `KeywordParameterClass` and
  `UnsupportedKeywordParameterClass` enums so explicit deferral is observable
  rather than silent. Unknown type spellings and unclassified vectors are
  distinct validation errors.
- The carried loose ends add no grammar: the designations test documents and
  pins the intentional absence of an unused `PlanarController` declaration,
  and the xtask report assertion now permits only `Have` and `Be` among verb
  irregulars. The existing stub unknown-field test explicitly retains
  `separator` and `layout` as rejected fields.
- `cargo test --workspace` was rerun after refresh but is blocked in the
  unchanged `deckmaste_construction` compiled-consumer fixture because its
  local `ParserEnvironment` lacks `declaration_noun_features`; the same
  failure reproduces on `default`. A broader scoped run reached 416 passing
  xtask tests and then hit the refreshed parent's stale collision assertion
  (expected 60, measured 59), which also reproduces on `default`. Neither
  failing file is changed here; the feature-specific and affected-package
  gates listed above are green.
