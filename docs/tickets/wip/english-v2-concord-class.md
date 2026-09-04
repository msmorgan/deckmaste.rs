---
needs: [english-v2-np-postmodifiers, english-v2-require-through-optional-role]
---
**Replace the overloaded weak `Agreement::{Bare, ThirdPersonSingular}` axis
with `ConcordClass::{Other, ThirdPersonSingular}`.** Use the [`Agreement`,
`Concord Class`, `Finiteness`, and `Inflectional Form`
definitions](../../contexts/oracle-english/CONTEXT.md). Concord Class is the derived two-way
morphological equivalence class needed by relevant present-tense paradigms; it
is not underlying Person and Number, Finiteness, or an Inflectional Form.

Delete the old Agreement domain rather than retaining redundant aliases.
Preserve the English-v2 decision's existing one-member `Participle` axis and
its rule that no authored or public AST form tag is stored. If the compiler
needs a transient morphological feature, use an `InflectionalForm` inventory
such as plain, third-person-singular present, preterite, gerund-participle, and
past participle, with Finiteness represented separately when required. Do not
introduce a `Finite | Base | PastParticiple` pseudo-paradigm.

Append a dated superseding amendment to the English-v2 rewrite decision rather
than rewriting its history. Pin every currently supported *was/were* path: the
weak two-class stage may keep exceptional copula handling until explicit Person
and Number land, but it may not misclassify those forms to make the enum look
uniform. Acceptance shows the same Inflectional Form dimension can combine with
the applicable Concord Classes and that plain or participial forms do not
acquire spurious agreement values.

## Landing record

- Measured after the required refresh on the tree ending at landing-record
  change `quyrvqmw`; implementation change `vnkoqqpw` is based on refreshed
  claim `utostlpl`. The coverage-lock `covered` count is 16,824. The schema-9
  coverage lock is
  byte-identical before and after: 49,474 lines, SHA-256
  `d7b889de6b5c1b79163ea5b3dfe9e18efde5bde1480aadcef4aa8f04b659148e`,
  and movement **+0/-0 rows, +0/-0 bytes**.
- Vocabulary and compiler: the authored, semantic-plan, emitted, generated,
  visitor, renderer, scanner, diagnostics, compiled-consumer, and licensing
  checker seams now use `ConcordClass::{Other, ThirdPersonSingular}` and
  `concord_class`. No `Agreement` alias remains. The unrelated legacy
  Person/Number product is re-spelled `PersonNumber`, and legacy render
  normalization follows that underlying-domain name rather than being
  misclassified as Concord Class.
- Morphology: transient compiler morphology uses one
  `InflectionalForm::{Plain, ThirdPersonSingularPresent, Preterite,
  GerundParticiple, PastParticiple}` inventory. Its acceptance matrix proves
  Plain with no Concord Class or `Other`, third-person-singular present with
  `ThirdPersonSingular`, preterite with either Concord Class, and both
  participles with no Concord Class. Finiteness remains separate. The
  one-member authored `Participle` axis remains, and no Inflectional Form tag
  enters the public or authored AST.
- Homogeneous sequences: every prior `derive members.agreement = ...` and
  `derive agreement = members.agreement` path is re-spelled through the
  existing per-feature carrier as `members.concord_class` and
  `concord_class`. The licence-reader census still classifies a `checked by`
  reader of that feature as permitted.
- Copula pins: nine exact selected Construction witnesses retain every
  currently supported *was/were* family: finite passive (two), finite copular
  (two), existential (two), copular subject-gap relative (two), and irrealis
  *were* (one). `Was` and `Were` are classified as Preterite, never forced into
  a Concord Class. The former member-specific irrealis guard was removed; its
  lexical form already selects the path without a `require` naming a verb
  form.
- Decision: the 2026-09-04 superseding amendment to
  `docs/decisions/english-v2-rewrite.md` records the Concord Class rename,
  sequence equations, Inflectional Form applicability, separate Finiteness,
  retained Participle/public-AST rule, and exceptional preterite copula
  selection. Historical decision text is retained.
- Coverage: 16,824 selected and covered units / 15,817 parse failures before
  and after. Selected-uncovered units, newly covered identities, coverage
  drops, round-trip mismatches, ownership failures, internal failures,
  unresolved ties, exception resolutions/uses, traversal failures, gaps,
  overlaps, synthetic claims, and provenance-plan mismatches are all zero.
  Newly covered identities: none, so there are no new selected analyses to
  enumerate.
- Selection census: 11,527 unique and 5,297 specificity-resolved selections
  before and after; exception-resolved selections and unresolved ties remain
  zero. Full `ambiguity --json` reports from a reflink baseline and this tree
  were compared for all 32,641 units by identity, status, internal-failure
  kind, resolution mode, and selected Construction path. Exactly 70 selected
  paths carry the required mechanical segment rename
  `FiniteClauseBareAgreementAuxiliaryFiniteClause` ->
  `FiniteClauseOtherConcordClassAuxiliaryFiniteClause`; after that vocabulary
  normalization, the per-unit projections are byte-identical and **0 units
  change analysis**.
- Structural and licensing census: 395 Construction declarations before and
  after. Coverage reports 25 permitted licensing checkers and 0 forbidden
  checkers before and after. No Construction was added or deleted, and no
  `checked by` or `require` clause names a verb form, lexeme, Construction, or
  card.
- Positive gates: `cargo fmt --all -- --check`; strict all-target Clippy for
  `deckmaste_construction_core`, `deckmaste_construction`,
  `deckmaste_english_v2`, `deckmaste_english`, `deckmaste_spelling`,
  `deckmaste_legacy_render`, and `xtask`; and `cargo test --workspace` are
  green. The refreshed workspace run completed 127 suites with 6,111 passed, 0 failed,
  and 6 pre-existing ignored tests. Representative artifacts are `test result:
  ok. 1 passed; 0 failed` for the renamed `trybuild` suite, `test result: ok. 39
  passed; 0 failed` for the generated consumer, `test result: ok. 398 passed; 0
  failed` for construction core, `test result: ok. 101 passed; 0 failed` for
  predicate grammar, and `test result: ok. 444 passed; 0 failed; 1 ignored` for
  `xtask`. `coverage --check` reports schema 9, 16,824 selected, 16,824 covered,
  no lock movement, 25 permitted/0 forbidden licence readers, and every
  failure counter zero. `ambiguity --require-resolved` reports 16,824 selected,
  11,527 unique, 5,297 specificity-resolved, and 0 unresolved ties. The
  accepted-set two-law gate reports 16,824 parse-accepted / 16,824 clean / 0
  mismatched. `cargo xtask map enums` shows `ConcordClass`,
  `InflectionalForm`, and `Participle`, with no `Agreement`. The required grep
  finds `Agreement` only in its glossary definition and retained/superseding
  historical decision prose, never in `crates/` or `plugins/builtin_v2`.
  No citation changed, so citation gates were not required.
- Performance advisory: all corpus commands used 8 workers. The final refreshed
  coverage run took 106.124999044 s at 154,859 ns/B with host load
  12.97/10.04/8.63; ambiguity took 94.939689223 s at 120,409 ns/B with load
  6.85/8.95/8.51; roundtrip took 90.106497497 s at 113,190 ns/B with load
  9.39/9.53/8.59. Each was one foreground gate process. Sibling-process
  visibility is unavailable in the sandbox; the reviewer supplies the
  contention count. The loaded-host readings exceed the 16.26 s quiet-host
  ceiling and are advisory, not a STOP.
- Assurance census: restored 0; re-spelled 40 (38 existing `#[test]` function
  names plus 2 `trybuild` compile-fail cases), with their AST, feature,
  sequence, morphology, parser, renderer, visitor, ownership, diagnostic, and
  rejection assertions migrated in place; ignored with blockers 0; added 3
  test functions (the Inflectional Form applicability matrix, complete finite
  copula morphology inventory, and nine-path preterite copula selection pin);
  removed 0. The feature-local `#[test]` delta is therefore +3; the six ignored
  workspace tests are pre-existing.
- Refresh: `kata refresh` completed without conflict. It rewrote the stack over
  newer coordinator claim records but introduced no sibling source overlap;
  pre-refresh and post-refresh ambiguity JSON is byte-identical. Formatting,
  strict Clippy, the full workspace suite, coverage, ambiguity, roundtrip, enum
  census, lock check, construction count, and vocabulary grep were all rerun on
  the refreshed tree, and the figures above are the refreshed measurements.
- Deviations and additions: no Construction was added or deleted. The three
  tests above are the ticket's required acceptance and copula pins. Re-spelling
  the unrelated legacy Person/Number product was necessary to satisfy the
  ticket's no-alias/no-old-domain requirement without applying Concord Class
  to a different grammatical domain.
- STOPs: none. Glossary gaps: none. Decision wanted: none.
