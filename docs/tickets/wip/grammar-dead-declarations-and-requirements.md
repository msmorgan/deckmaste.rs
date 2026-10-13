---
needs: [grammar-compiler-expressions-and-tables]
---
# Dead declarations and wildcard tables

What remains of the authoring-layer work after every Category interface was converted to head and local modes. In this order, each step a commit that keeps every reading digest identical on every face (`cargo xtask english-v3 diff --check identical-digests`). The aim is a grammar that reads more clearly; a step that does not make it clearer is skipped and said so.

1. Dead declarations. `PurposeConcord`, `PurposeHostProjection` and `coordinate_number` have no users. Then interface slots nothing reads, one feature at a time: a (feature, Category) slot is dead only if no `require`, `agree` or table argument reads it, directly or through a forward to a slot that is read, AND removing it cannot change admission. Some unread case exports still constrain admission through the table lookup that computes them; those are not dead. `Polarity` sits in 16 interfaces and is read from none. For each removed slot the landing says how it was shown dead; any candidate that cannot be shown dead stays and is listed. There were 91 candidates when the interfaces were converted.
2. Wildcard table rows. They already exist (`_` parses as `TablePattern::Any`, first matching row wins). Convert tables where that makes them clearer, keeping the complete domain: a table such as `coordinated_property_value_position`, 75 rows of which 74 yield `None`, becomes a few rows.
3. Policy composition: a policy may use policies. Cycles are rejected; precedence and conflicting overrides are defined and tested in `crates/deckmaste_construction_v3_core`.
4. Boolean predicates in `require` (`and`, `or`, `not`, `in`), only if step 1 to 3 leave tables that a predicate would state more clearly. Decided: a comparison with an absent operand is unknown, connectives are three-valued, and a requirement admits only when definitely true. A direct requirement on an optional element keeps today's behaviour and applies only when the element is present. A table becomes a predicate only if the two agree on every input including absent ones, checked mechanically per table; `clitic_complement` does not qualify. The 24 AND and 11 OR tables all compute exported values and belong to `grammar-generic-coordination`.

Each completed compiler capability updates `crates/deckmaste_construction_v3_core/README.md`. Note: census delta; the judgment-set line does not apply yet.

## Dead-declaration pass

Removed PurposeConcord, PurposeHostProjection and coordinate_number: each had
exactly one identifier occurrence, its declaration, across the grammar and no
references elsewhere in Rust/compiler sources. No rules or feature domains were
removed.

The expanded schema/policy reference graph found 91 unread interface slots.
The pass removes 38, one feature at a time; each intermediate declaration
validates. No requirement, agreement, selected literal or table argument reads
these slots, including transitively through forwarding. Retained unread slots
were treated as roots before choosing removals.

| Removed feature | Categories | Admission proof |
|---|---|---|
| AuxiliaryEllipsisLicence | ManaPassivePredicate | Defaulted lexical projection or forwarding from complete interfaces; all authored lexical values are in the declared domain. No computing table is removed. |
| AuxiliaryProFormLicence | ManaPassivePredicate | Defaulted lexical projection or forwarding from complete interfaces; all authored lexical values are in the declared domain. No computing table is removed. |
| ClauseComplementBoundary | FiniteManaPassivePredicate, GerundParticipleObjectGap, ManaPassivePredicate, ScalarLocationPredicate | Every producer supplies a constant; no input check or table lookup is removed. |
| ComparisonMarker | ComparativeAdjectivePhrase | Constants or forwarding from complete retained child interfaces; no input admission check is removed. |
| FocusedUse | FocusedAdjunctCoordination, FocusedAdjunctMember, FocusedAdjunctSeries | Every producer supplies a constant; no input check or table lookup is removed. |
| MatrixComplementLicence | ExistentialPredicate, GerundParticipleObjectGap | Defaulted lexical projection or forwarding from complete interfaces; all authored lexical values are in the declared domain. No computing table is removed. |
| NominalAdjunctHost | FiniteManaPassivePredicate, ManaPassivePredicate, ScalarLocationPredicate | Every producer supplies a constant; no input check or table lookup is removed. |
| Polarity | CentralAdverbSelectedHead, CorrelativeFiniteObjectGapSeries, CorrelativeFinitePredicateSeries, CorrelativeFiniteSelectedHeadSeries, ExistentialPredicate, FiniteDepictiveHost, FiniteManaPassivePredicate, FiniteObjectGap, FiniteObjectGapSeries, FinitePredicate, FinitePredicateSeries, FiniteSelectedHead, FiniteSelectedHeadSeries, NegatedAuxiliaryHead, QuantifiedAuxiliaryHead, ScalarLocationPredicate | Defaulted lexical projection or forwarding from complete interfaces; all authored lexical values are in the declared domain. No computing table is removed. |
| PredicativeAdjunctUse | NominativePhrase | Constants or forwarding from complete retained child interfaces; no input admission check is removed. |
| PurposeAdjunctLicence | ExistentialPredicate, GerundParticipleObjectGap | Defaulted lexical projection or forwarding from complete interfaces; all authored lexical values are in the declared domain. No computing table is removed. |
| SelectedPrepositionUse | NominativePhrase | Constants or forwarding from complete retained child interfaces; no input admission check is removed. |
| SourceComplementSelection | GerundParticipleObjectGap | Defaulted lexical projection or forwarding from complete interfaces; all authored lexical values are in the declared domain. No computing table is removed. |
| SourceModifierPresent | NominativePhrase | Constants or forwarding from complete retained child interfaces; no input admission check is removed. |
| WholeObjectSourcePresent | NominativePhrase | Constants or forwarding from complete retained child interfaces; no input admission check is removed. |

The lexical `require head.Polarity = Positive` remains. The removed slot
exports do not enforce that restriction. All construction identities, fields,
surface pieces and form ordering remain identical (326 constructions, 927
lowered rules).

Candidates retained:

- export shared with retained interfaces; splitting policy would obscure it: `AdjectivePhraseSeries.PremodifierCoordination`, `AdverbPhraseSeries.NominalAdjunctHost`, `AdverbPhraseSeries.UnmarkedConjunctLicence`, `ClauseSeries.ClauseCoordinationHost`, `CorrelativeAdverbPhraseSeries.NominalAdjunctHost`, `CorrelativeClauseSeries.ClauseCoordinationHost`, `CorrelativeNounPhraseSeries.BareGenitiveHost`, `CorrelativeNounPhraseSeries.ComparativePostmodifierUse`, `CorrelativeNounPhraseSeries.Definiteness`, `CorrelativeNounPhraseSeries.LocativeUse`, `CorrelativeNounPhraseSeries.NominalDependentPresent`, `CorrelativeNounPhraseSeries.NounPhraseStructure`, `CorrelativeNounPhraseSeries.SubjectStructure`, `CorrelativePrepositionPhraseSeries.NominalAdjunctHost`, `CorrelativePrepositionPhraseSeries.NominalComplementMarker`, `ExistentialPredicate.number`, `ExistentialPredicate.person`, `FiniteManaPassivePredicate.MatrixComplementLicence`, `ManaPassivePredicate.HeadCoordination`, `NominalSeries.BareGenitiveHost`, `NominalSeries.BarePrepositionUse`, `NominalSeries.ComparativeModifierPresent`, `NominalSeries.NominalComplementMarker`, `NominalSeries.ScalarMeasurementHeadUse`, `NominalSeries.UnmarkedConjunctLicence`, `NounPhraseSeries.BareGenitiveHost`, `NounPhraseSeries.ComparativePostmodifierUse`, `NounPhraseSeries.Definiteness`, `NounPhraseSeries.LocativeUse`, `NounPhraseSeries.NominalDependentPresent`, `NounPhraseSeries.NounPhraseStructure`, `NounPhraseSeries.SubjectStructure`, `NounPremodifierSeries.PremodifierCoordination`, `PastParticipleObjectGap.ClauseComplementBoundary`, `PastParticipleObjectGap.NominalAdjunctHost`, `PrepositionPhraseSeries.NominalAdjunctHost`, `PrepositionPhraseSeries.NominalComplementMarker`, `ScalarLocationPredicate.MatrixComplementLicence`, `ScalarLocationPredicate.PurposeAdjunctLicence`, `SelectedComplementTailSeries.HeadCoordination`, `SelectedPrepositionHeadSeries.NominalComplementMarker`.
- forwarded to a retained unread slot: `BareScalarLocationComplement.MatrixComplementLicence`, `BareScalarLocationComplement.PurposeAdjunctLicence`, `BareScalarLocationPredicate.MatrixComplementLicence`, `BareScalarLocationPredicate.PurposeAdjunctLicence`, `ManaPassiveComplement.MatrixComplementLicence`, `ManaPassivePredicate.MatrixComplementLicence`.
- computing table retained; admission-neutrality not established for removal: `AccusativePhrase.CaseUse`, `FocusedAdjunctCoordination.PrepositionPhraseComplementMarker`, `FocusedAdjunctCoordination.SourceComplementUse`, `ManaPassivePredicate.FrameUse`, `ManaPassivePredicate.form`, `NominativePhrase.CaseUse`.

The shared-policy candidates are intentionally skipped: splitting an otherwise
useful coordination policy just to omit an unread constant or forward would
make its uses harder to follow. No slot computed by a table is claimed dead.

Dead-declaration gate (`srltwmmy`): all 32,828 faces have identical digests;
no lost/gained faces or Reading changes. Counts: declarations.rs 6,897 → 6,855
lines; exports 991 → 975; policies 119 → 117; tables 216 → 215; interface
entries 714 → 676. The subset check also passes (11 faces, 40 Readings).
Full English and xtask tests/clippy pass for the completed prefix.

## Wildcard-row pass

Use the existing first-matching-row semantics to state the exception, then the
default. `coordinated_property_value_position` changes from 75 explicit rows
to `(Additive, First, Second) => Pair` followed by `(_, _, _) => None`.
The 24 AND tables now state `(Yes, Yes) => Yes` then `(_, _) => No`; the
11 OR tables state `(No, No) => No` then `(_, _) => Yes`. They still compute
exported feature values and remain for the coordination ticket.

Every converted table was mechanically compared with its old rows over its
complete declared input domain plus absence in every argument: 459 combinations
(35 times 9, plus 144). Outputs and missing results are identical; wildcards
still reject absent arguments. The compiler validates the converted grammar
with the same 326 constructions and 927 lowered rules. No compiler capability
is introduced by these row conversions.

Converted tables:

- AND: `coordinated_frequency_use`, `coordinated_participial_supplement`, `coordinated_targeting_premodifier`, `connective_adjunct_concord`, `predicative_adjunct_concord`, `coordinated_purpose_licence`, `coordinated_source_marker`, `coordinated_source_complement_use`, `comparative_gap_concord`, `coordinated_intervening_adjunct`, `auxiliary_pro_form_concord`, `auxiliary_ellipsis_concord`, `selected_preposition_complement`, `coordinated_attribute_premodifier_head`, `complement_marker_concord`, `coordinated_transparency`, `coordinated_scalar_denotation`, `nominal_exception_concord`, `coordinated_cardinal_complement`, `slash_premodifier_concord`, `coordinated_locative_use`, `coordinated_final_adjunct`, `coordinated_initial_adjunct`, `coordinated_duration_use`.
- OR: `coordinated_focus`, `source_selection_union`, `source_modifier_presence`, `nominal_source_modifier`, `whole_object_source_presence`, `arithmetic_unit_bearing`, `selected_preposition_concord`, `complement_presence`, `internalised_complement_presence`, `coordinated_targeting`, `nominal_dependent_presence`.
- Property position: `coordinated_property_value_position`.

Other tables retain their explicit judgments; this pass does not obscure
multi-valued selection by introducing broad defaults.

## Remaining compiler work

Policy composition, then Boolean requirements, remain unimplemented. Both
absence rulings above are settled; compound predicates over optional-element
features remain unsupported. `clitic_complement` stays a table. No provisional
compiler patch was applied in this prefix.

The earlier selected-frame regressions have an identified cause: the provisional
Boolean runtime indexed a new per-symbol predicate_reads vector by dot, while
dynamic selected-frame expansion resized the symbols and existing per-symbol
vectors without resizing that vector. Longer expanded frames indexed past the
template vector. The resumed implementation must preserve symbol-index
alignment across expansion, parsing and independent admission, and retain the
existing selected-frame regression tests.

## Prefix gates and measurements

Both steps pass `cargo xtask english-v3 diff <baseline> <after> --check
identical-digests` against the fresh baseline taken before this ticket's edits:
32,828 faces compared, all unchanged, no source-hash changes and no lost/gained
faces or Reading-count changes. Both smoke subsets pass (11 faces, 40 Readings).

| Step | Change | Lines | Exports | Policies | Tables | Interface entries |
|---|---|---:|---:|---:|---:|---:|
| Baseline | | 6,897 | 991 | 119 | 216 | 714 |
| Dead declarations | srltwmmy | 6,855 | 975 | 117 | 215 | 676 |
| Wildcard rows | wumvkqzo | 6,845 | 975 | 117 | 215 | 676 |

Clippy for all tests in deckmaste_english_v3 and xtask passes with warnings
denied. Citation checks report 0 stale and 0 noncompliant references; the diff
audit selects no citation sites. The changed declarations file passes rustfmt.
Full `cargo test -p deckmaste_english_v3 -p xtask` passes: English 1,165 and
xtask 439, total 1,604 passed, 0 failed, 2 pre-existing ignores. They are
`mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`
(blocked by english-v3-mana-symbol-verbless-subject MS4) and
`macros::templates::tests::macro_schema_census_count_matches_21`
(the existing on-demand live-corpus cross-check). No test source changed:
restored 0, re-spelled 0, newly ignored 0, added 0, removed 0.

Census delta (`wumvkqzo`, prefix): faces read 30,794 → 30,794; total Readings 482,041 → 482,041; lost faces: none.
Judgment set (`wumvkqzo`): does not apply yet.

## Not done, deliberately

Steps 3 and 4 (policy composition, Boolean requirements) are dropped from this ticket rather than deferred into it. Nothing in the converted grammar currently needs a policy to use a policy, and the Boolean experiment found only two tables a predicate could replace exactly, at the cost of new absence and optional-element semantics in the compiler. Neither earns its place yet. If the coordination work or a later step shows a concrete need, it is specified then, with the decisions already recorded above.
