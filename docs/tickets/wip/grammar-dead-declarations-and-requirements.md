---
needs: [grammar-compiler-expressions-and-tables]
---
# Dead declarations, wildcard tables, policy composition, Boolean requirements

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
Full English and xtask tests/clippy will validate the completed prefix.

Census delta (`srltwmmy`, prefix): faces read 30,794 → 30,794; total Readings 482,041 → 482,041; lost faces: none.
Judgment set (`srltwmmy`): does not apply yet.
