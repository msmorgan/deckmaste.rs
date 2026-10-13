---
needs: [grammar-dead-declarations-and-requirements]
---
# Generic coordination with per-feature combiners

At the prototype baseline, coordination was enumerated per category: 51 `*Series` and `Correlative*` categories held 289 of 676 interface entries (43%); 257 of 415 schema instances (62%) are coordination rows. Replace this with one declaration and per-feature combiners, generating the same construction and category names so readings do not change.

## Change

- `coordinable NounPhrase, Nominal, ...;` The compiler generates the binary, serial and correlative forms and the series helper categories. The exact surface syntax is settled in the landing; the requirements below are not.
- This is elaboration to today's declarations. The generated constructors must reproduce today's constructor shape, field names and order, form indices and optional `category` field exactly, because the census hashes the complete Reading `Debug` output (`crates/xtask/src/english_v3/validation.rs:31`) and the generated values carry `form`, `category` and ordered fields (`crates/deckmaste_construction_v3_core/src/emit.rs:692–711`). Admission must be unchanged as well; matching names alone does not preserve digests.
- Each feature declares how it combines across conjuncts: `all`, `any`, `same`, or a named table. Combiners allow per-category and per-context overrides, because behaviour is category-sensitive today: `number` combines through `cardinal_coordinate_number` for Cardinals (`CardinalAgreement`, `crates/deckmaste_english_v3/src/declarations.rs:1657`) but through `determined_np_coordinate_number` for determined noun phrases (`declarations.rs:1890`), and finite predicates require agreement (`FiniteConcord`, `:1690`). They allow a table to take arguments from several features, and they allow asymmetric restrictions such as `require Right.CliticHost = Free` (`:1698`). The coordination instance rows carry different distribution and boundary policies (`:3778`); each category keeps the forms (binary, serial, correlative) it has today and no others.
- Export-computing AND and OR tables left by `grammar-compiler-expressions-and-tables` are replaced here by `all` and `any`. Settle first, with a script over the tables used by series instances, how many concord tables are uniform; real logic such as `determined_np_coordinate_number` stays a named table.
- Delete the per-category series and correlative declarations, schemas and instance rows the generator replaces.
- The specification includes an inventory of the existing tests that construct coordination readings by hand (for example `crates/deckmaste_english_v3/tests/suite/correlatives.rs`, `correlative_adjectives.rs`, `correlative_finite_complements.rs`) and keeps each, re-spelled if its constructor must change, for the same witnesses; tests are never deleted to pass, and the landing reports restored, re-spelled, added and removed counts.

## Settled authoring model (2026-10-12)

Model B: structural forms, markers and separators remain grammar-owned reusable
schemas. The compiler generates per-category instances and helper summaries.
Feature declarations supply default combiners; Category interface annotations
are exceptions, independent of `head`/`local`. Context overrides take precedence.
`all`, `any` and `same` reject absent inputs. Constants use `always Value`;
real judgments remain named tables, with role-inferred arguments for regular
signatures and explicit arguments otherwise. References use `left`, `right`
and `coordinator`. No Boolean requirements or policy composition.

Profiles and blocks share binary/serial/both/either/neither role names;
`schema(Name)` is the exact-schema escape. Profiles can state common requirements
once. Helpers have default names with explicit exceptions. Distinct member and
result Categories use `coordinable Result of Member using Profile`.

Phase 1's nominal prototype was reviewed independently at `onkkszmp`; equivalence
and its full-corpus gate were confirmed. The revised prototype must pass the same
gate and sorted-contract comparison before Phase 2 continues, one family per
commit. Convert only families whose blocks remain clear with a few ordinary
requirements/overrides/omissions and no additional compiler construct. A further
compiler capability is a STOP; a prospective capability must serve at least
three families. Leave exceptional structures handwritten and explain them in
the compiler README.

## Gate

Every `readings_sha256` identical on every face, checked with the census check mode as an identical-digest landing. The landing updates `crates/deckmaste_construction_v3_core/README.md`. Note: census delta (none); the judgment-set line does not apply yet.

## Phase 1 prototype

Nominal and NounPhrase now use interface combiners and the grammar-owned Standard
form profile. Their fifteen instance rows and three helper interfaces are
compiler-generated; four policies and seven now-unused AND/OR tables are removed.
The revised prototype implements the reviewed defaults, roles, helper names and
member/result distinction. Other families remain pending their separate conversion
and gate. Compiler tests cover the revised syntax and preserve the original
strict-combiner runtime matrix.

## Phase 2 prefix

AdjectivePhrase and CorrelativeAdjectivePhrase use two blocks, with the existing
member/result distinction and the CorrelativeAdjectiveSeries name exception.
Eleven rows and two helper interfaces are generated. AdjectiveStructureMerge,
AdjectiveAttributiveConcord, adjective_attributive_concord and
coordinated_adjective_premodifier have no remaining users and are removed.
The 670 constructor/form contracts and 927 normalized admission contracts match
the original baseline. The two removed tables also agree with strict primitive
combination on all nine present/absent input pairs each. Existing hand-built
Reading tests retain their constructor names, fields and assertions.

AdverbPhrase also uses one block: an ordinary-adjunct requirement, an ordinary-form
licence override, and omission of that licence from correlative helpers. Eleven
rows and two helper interfaces are generated. AdverbPermissions and its three
last-user tables coordinated_frequency_use, coordinated_final_adjunct and
coordinated_duration_use are removed; their present/absent domains were also
exhaustively compared. All constructor and admission contracts remain identical.

PrepositionPhrase and SelectedPrepositionHead replace fifteen instance rows and
three helper interfaces with two blocks. LocativeUse overrides the feature default;
selected-head helpers omit HeadCoordination. ObliquePrepositionConcord and
PrepositionPermissions and SharedHeadStatus have no remaining users. Eight last-user Boolean tables
are removed: complement_marker_concord, coordinated_focus,
coordinated_initial_adjunct, coordinated_intervening_adjunct,
coordinated_locative_use, nominal_exception_concord, selected_preposition_concord
and source_modifier_presence. All thirteen table-to-primitive uses were compared
on all nine present/absent input pairs; the 670 shape and 927 admission contracts
remain identical. Multi-valued tables and tables with other users remain.

FinitePredicate, SecondaryVerbPhrase, FiniteSelectedHead and SecondarySelectedHead
replace forty-four rows and eight helper interfaces. The grammar-owned Clausal
profile retains their lack of a GeneralCoordination requirement. Right-only clitic
requirements and helper omissions remain explicit. FiniteConcord, SecondaryConcord,
SecondaryConjunctProperties, SelectedFrameConcord and SharedFrameConcord lose their
last users, as do auxiliary_ellipsis_concord, auxiliary_pro_form_concord,
complement_presence and coordinated_participial_supplement. Six Boolean-table uses
were compared on all nine present/absent pairs. All 670 shape and 927 admission
contracts remain identical.

FiniteObjectGap, BareObjectGap and PastParticipleObjectGap replace twenty-six rows
and five helper interfaces. Their right-only clitic requirement, boundary omissions
and differing form sets remain explicit. FiniteGapConcord, BareGapConcord and
NoClauseComplementBoundary have no remaining users. The last-user tables
comparative_gap_concord and coordinated_purpose_licence are removed; strict all
was compared with the former on all nine present/absent pairs (the latter was
already checked with the predicate family). Shape and admission contracts match.
The sweep also removes CoordinatorKindSummary, whose final preposition user was
already converted.

Clause serial/either forms and CoordinatedFiniteClause replace fifteen rows and
four helper interfaces. ClauseCoordination binary punctuation stays handwritten.
Clause tail requirements use exact-schema contexts; finite clauses bind FiniteClause
members to CoordinatedFiniteClause results with their existing helper names.
CoordinatedClauseHost, ClauseCoordinationProperties, LeadingConjunct,
UnmarkedConjunct, UnmarkedConjunctHead and MarkedConjunctSeries lose their last users.
All 670 shape and 927 admission contracts match; no additional compiler capability
was needed.

Focused adjunct coordination remains handwritten: six result-only function
features would each require a constant override and a helper omission. Keeping
FocusedAdjunctConcord and FocusedAdjunctFunction avoids repeating that distinction.
No grammar declaration changes in this decision.

Cardinal replaces eleven rows and two helpers with one block, its number-table
annotation and the CardinalComplementUse default. CardinalAgreement, CardinalListEnd
and coordinated_cardinal_complement lose their last users. The Boolean table was
checked on all nine present/absent pairs; all 670 shape and 927 admission contracts
match the baseline.

MeasurePhrase replaces eleven rows and two helper interfaces. CoordinatedMeasureKind
loses its last user. arithmetic_unit_bearing remains used by arithmetic constructions;
its nine present/absent pairs agree with the existing UnitBearing any default.
All 670 shape and 927 admission contracts match the baseline.

Remaining families are pending their separate conversion and gate: GrantedAbility, depictive/predicative complements, comparative governors, quality prepositions, noun/verbal premodifiers, ManaPhrase, Amount, KeywordPhrase, InfinitiveComplement, FrequencyPhrase, QuotedText.
The explicit exceptions and Phase 2 stopping rule above still apply. Remaining
AND/OR twins must go only as their last handwritten users convert.

Census delta (`lunxqpyz`, measure family): faces read 30,794 → 30,794; total Readings 482,041 → 482,041; lost faces: none.
Judgment set (`lunxqpyz`): does not apply yet.
