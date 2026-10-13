//! Connected Oracle English declarations under whole-grammar activation.
//!
//! Lexical distribution and complete frame signatures govern composition.
//! The activation ticket records the remaining families and corpus obligations.

use deckmaste_construction_v3::constructions;

constructions! {
    pub mod grammar {
        capitalization Positional;

        feature DeterminerUse { SingularCount, Unrestricted, PluralOrMass, PluralCount, Mass,
            Singular }
        feature DeterminerKind { Ordinary, Indefinite }
        feature SuperlativeQuantifierUse { No, Yes } default No;
        feature DegreeUse { None, Head, Modifier } default None;
        feature QuantitativeExceptionUse { No, Yes } default No;
        feature ComparativeDegreeModifierUse { No, Yes } default No;
        feature FrequencyUse { No, Yes } default No combine all;
        feature PredeterminerUse { No, Multiplier, Fraction, UniversalQuantifier } default No;
        table predeterminer_host(PredeterminerUse, Definiteness, NounPhraseStructure) -> Selection {
            (Multiplier, Definite, Other) => Yes, (Fraction, Definite, Other) => Yes,
            (Multiplier, Definite, ProperName) => Yes, (Fraction, Definite, ProperName) => Yes,
            (Multiplier, Definite, IndefiniteArticle) => Yes, (Fraction, Definite, IndefiniteArticle) => Yes,
            (Multiplier, Definite, Predeterminer) => Yes, (Fraction, Definite, Predeterminer) => Yes,
            (UniversalQuantifier, Definite, Other) => Yes,
            (UniversalQuantifier, Definite, ProperName) => Yes,
            (UniversalQuantifier, Definite, IndefiniteArticle) => Yes,
        }
        table predeterminer_structure(PredeterminerUse) -> NounPhraseStructure {
            (Multiplier) => Predeterminer, (Fraction) => Predeterminer,
            (UniversalQuantifier) => UniversalPredeterminer,
        }
        table predeterminer_number(DeterminerUse, number, PartitiveCountability) -> Selection {
            (PluralOrMass, Plural, Count) => Yes, (PluralOrMass, Singular, Mass) => Yes,
            (PluralCount, Plural, Count) => Yes,
        }
        feature ParticipialSupplementUse { No, Yes } default No combine all;
        feature Definiteness { Unspecified, Nondefinite, Definite } default Unspecified combine always Unspecified;
        feature NounPhraseStructure { Other, ProperName, IndefiniteArticle,
            Predeterminer, UniversalPredeterminer } default Other combine always Other;
        feature DeterminerRequirement { No, Yes, Definite } default No combine coordinated_determiner_requirement;
        feature NumberTransparency { No, Yes } default No combine all;
        feature SubjectRelativeNumber { None, Singular, Plural } default None combine relative_number_concord;
        table relative_number_concord(SubjectRelativeNumber, SubjectRelativeNumber) -> SubjectRelativeNumber {
            (None, None) => None, (None, Singular) => Singular, (Singular, None) => Singular,
            (None, Plural) => Plural, (Plural, None) => Plural,
            (Singular, Singular) => Singular, (Plural, Plural) => Plural,
        }
        table deferred_relative_number(NominalAgreementNumber, number) -> SubjectRelativeNumber {
            (Head, Singular) => None, (Head, Plural) => None,
            (Plural, Singular) => Singular, (Plural, Plural) => Plural,
            (SingularOrPlural, _) => None,
        }
        table singular_relative_number(SubjectRelativeNumber) -> Selection {
            (None) => Yes, (Singular) => Yes,
        }
        table relative_number_chain(SubjectRelativeNumber, number) -> Selection {
            (None, _) => Yes, (Singular, Singular) => Yes, (Plural, Plural) => Yes,
        }
        table resolved_ordinary_relative_number(SubjectRelativeNumber, NominalAgreementNumber, number) -> Selection {
            (None, _, _) => Yes, (Singular, Head, Singular) => Yes,
            (Plural, Head, Plural) => Yes, (Plural, Plural, _) => Yes,
            (_, SingularOrPlural, _) => Yes,
        }
        table resolved_relative_number(SubjectRelativeNumber, NominalAgreementNumber, number, DeterminerAgreementNumber) -> Selection {
            (None, _, _, _) => Yes,
            (Singular, _, _, Singular) => Yes,
            (Singular, Head, Singular, Nominal) => Yes,
            (Plural, Head, Plural, Nominal) => Yes,
            (Plural, Plural, _, Nominal) => Yes,
            (_, SingularOrPlural, _, _) => Yes,
        }
        feature NominalAgreementNumber { Head, Plural, SingularOrPlural } default Head
            combine coordinated_nominal_agreement(coordinator.CoordinationKind, right.NominalAgreementNumber);
        feature DeterminerAgreementNumber { Nominal, Singular, SingularOrPlural } default Nominal combine coordinated_determiner_agreement;
        feature QuantificationalDeterminer { No, Yes } default No;
        feature QuantificationalAdjunctUse { No, Yes } default No;
        feature AuxiliaryUse { No, Yes } default No;
        feature Polarity { Positive, Negative } default Positive;
        feature PrimaryNegation { Yes }
        feature ObliqueMarker { No, Yes } default No;
        feature ObliqueNumber { None, Singular, Plural } default None
            combine coordinated_oblique(coordinator.CoordinationKind, left.ObliqueNumber, right.ObliqueNumber);
        feature PartitiveCountability { Count, Mass, Unrestricted } default Unrestricted combine coordinated_partitive_countability;
        feature CaseUse { Common, Nominative, Accusative, Genitive } combine common_case;
        feature Targeting { No, Yes } combine any;
        feature TargetingPremodifierUse { No, Yes } default No combine all;
        table targeting_premodifier(TargetingPremodifierUse, Targeting) -> Selection {
            (No, No) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        table coordinated_targeting_premodifier(TargetingPremodifierUse, TargetingPremodifierUse)
            -> TargetingPremodifierUse {
            (Yes, Yes) => Yes,
            (_, _) => No,
        }
        feature ClauseCoordinationHost { Simple, Coordinated } default Simple combine always Coordinated;
        feature ClauseComplementBoundary { None, Finite, Verbless } default None combine coordinated_clause_complement_boundary;
        feature ConnectiveAdjunctUse { No, Yes } default No combine all;
        table nonconnective_trailing_adjunct_host(ClauseComplementBoundary) -> Selection {
            (None) => Yes, (Verbless) => Yes,
        }
        table trailing_adjunct_host(ClauseComplementBoundary, ConnectiveAdjunctUse) -> Selection {
            (None, No) => Yes, (None, Yes) => Yes,
            (Verbless, No) => Yes, (Verbless, Yes) => Yes,
            (Finite, Yes) => Yes,
        }
        table coordinated_clause_complement_boundary(ClauseComplementBoundary,
            ClauseComplementBoundary) -> ClauseComplementBoundary {
            (None, None) => None, (None, Verbless) => Verbless,
            (Verbless, None) => Verbless, (Verbless, Verbless) => Verbless,
            (None, Finite) => Finite, (Finite, None) => Finite,
            (Verbless, Finite) => Finite, (Finite, Verbless) => Finite,
            (Finite, Finite) => Finite,
        }
        table connective_adjunct_concord(ConnectiveAdjunctUse, ConnectiveAdjunctUse)
            -> ConnectiveAdjunctUse {
            (Yes, Yes) => Yes,
            (_, _) => No,
        }
        feature CoordinationKind { Additive, Alternative, Adversative }
        feature PrepositionComplement { NounPhrase, PredicativeNounPhrase, PrepositionPhrase, Intransitive } set;
        table noun_phrase_complement(PrepositionComplement) -> Selection contains NounPhrase => Yes;
        table preposition_phrase_complement(PrepositionComplement) -> Selection contains PrepositionPhrase => Yes;
        table predicative_noun_phrase_complement(PrepositionComplement) -> Selection contains PredicativeNounPhrase => Yes;
        table intransitive_preposition_complement(PrepositionComplement) -> Selection contains Intransitive => Yes;
        feature AdjectiveComplementClass { None, Manner, ColorWord } default None combine coordinated_adjective_complement_class;
        feature AdverbComplementClass { Duration }
        feature DurationUse { No, Yes } default No combine all;
        feature FocusingModifierUse { Restrictive }
        // A focused conjunct blocks focus on the whole coordination.
        feature FocusedUse { No, Yes } default No combine any;
        // CF8: two focused Adjuncts use the common member route exclusively.
        table ordinary_adjunct_coordination(FocusedUse, FocusedUse) -> Selection {
            (No, No) => Yes, (No, Yes) => Yes, (Yes, No) => Yes,
        }
        feature NominalAdjunctHost { No, Yes } default Yes combine always Yes;
        feature PredicativeAdjunctUse { No, Yes } default No combine all;
        table predicative_adjunct_concord(PredicativeAdjunctUse, PredicativeAdjunctUse)
            -> PredicativeAdjunctUse {
            (Yes, Yes) => Yes,
            (_, _) => No,
        }
        feature PurposeAdjunctLicence { No, Yes } default Yes combine all;
        feature FusedHeadUse { No, Yes } default No;
        feature ImmediatePostheadDependentUse { No, Yes } default No;
        feature FusedHeadPostmodifierUse { No, Yes } default No;
        table licensed_posthead_dependent(present, ImmediatePostheadDependentUse) -> Selection {
            (No, _) => Yes, (Yes, Yes) => Yes,
        }
        feature ComparativePostmodifierUse { No, Yes } default No combine always No;
        feature QuantitativeComplement { None, Cardinal, CardinalPrepositionPhrase, AdjectivePhrase, ExceptionQuantity } default None;
        feature QuantitativeAdjectiveComplementUse { No, Yes } default No;
        feature ComparativeQuantityUse { No, Yes } default No;
        feature QuantitativeConjunctUse { No, Yes } default No;
        feature ComparativeDeterminerUse { No, Yes } default No;
        feature ComparativeQuantityStructure { Coordinated, Headed }
        feature ScalarComparativeUse { No, Yes } default No;
        feature ImplicitComparativeUse { No, Yes } default No combine all;
        feature AdjectiveAttributiveOnlyUse { No, Yes } default No combine any;
        table implicit_comparative_attributive_use(ImplicitComparativeUse) -> AdjectiveAttributiveOnlyUse {
            (No) => No, (Yes) => Yes,
        }
        feature ComparativeModifierPresent { No, Yes } default No combine always No;
        table comparative_modifier_present(ImplicitComparativeUse, ComparativeModifierPresent)
            -> ComparativeModifierPresent {
            (Yes, _) => Yes, (No, Yes) => Yes, (No, No) => No,
        }
        feature MeasureModifierUse { No, Yes } default No;
        feature AdditiveModifierUse { No, Yes } default No;
        feature PrepositionFunctionLicence { Adjunct, Modifier, NounComplement, VerbComplement,
            PredicativeComplement, Compound, ComparativeComplement, PreposedAdjunct,
            Supplement, MedialSupplement } set combine coordinated_preposition_licence;
        feature MatrixComplementGovernor { No, Yes } default No;
        feature MatrixComplementRequirement { None, Internalised, Nominal, Content } default None combine matrix_complement_concord;
        feature MatrixComplementLicence { None, Nominal, Content } default None combine matrix_host_concord;
        feature NominalExceptionComplementUse { No, Yes } default No combine all;
        table matrix_host_concord(MatrixComplementLicence, MatrixComplementLicence)
            -> MatrixComplementLicence {
            (None, None) => None, (None, Nominal) => None, (None, Content) => None,
            (Nominal, None) => None, (Nominal, Nominal) => Nominal, (Nominal, Content) => None,
            (Content, None) => None, (Content, Nominal) => None, (Content, Content) => Content,
        }
        table licensed_matrix_complement(MatrixComplementLicence, MatrixComplementRequirement)
            -> Selection { (Nominal, Nominal) => Yes, (Content, Content) => Yes, }
        table matrix_pp_requirement(InternalisedComplementMarker, NominalExceptionComplementUse)
            -> MatrixComplementRequirement {
            (Yes, No) => Internalised, (No, Yes) => Nominal,
        }
        table matrix_complement_concord(MatrixComplementRequirement, MatrixComplementRequirement)
            -> MatrixComplementRequirement {
            (None, None) => None, (Internalised, Internalised) => Internalised,
            (Nominal, Nominal) => Nominal, (Content, Content) => Content,
        }
        feature CompoundComplementMarker { Of }
        feature PrepositionPhraseComplementLicence { Unrestricted, Source, With }
        feature PrepositionPhraseComplementMarker { None, With } default None combine coordinated_preposition_phrase_complement_marker;
        feature SourceComplementUse { No, Yes } default No combine all;
        feature SourceMarker { No, Yes } default No combine all;
        feature SourceComplementSelection { No, Yes } default No combine any;
        table source_selection_union(SourceComplementSelection, SourceComplementSelection)
            -> SourceComplementSelection {
            (No, No) => No,
            (_, _) => Yes,
        }
        // Orchestrator ruling RS2: a source-selected PP is a Complement.
        table source_adjunct_admission(SourceComplementSelection, SourceMarker) -> Selection {
            (No, No) => Yes, (No, Yes) => Yes, (Yes, No) => Yes,
        }
        feature SourceModifierPresent { No, Yes } default No combine any;
        table nominal_source_modifier(SourceModifierPresent, SourceMarker) -> SourceModifierPresent {
            (No, No) => No,
            (_, _) => Yes,
        }
        feature WholeObjectSourcePresent { No, Yes } default No combine any;
        table whole_object_source_presence(WholeObjectSourcePresent, WholeObjectSourcePresent) -> WholeObjectSourcePresent {
            (No, No) => No,
            (_, _) => Yes,
        }
        table whole_object_source_modifier(WholeObjectSourcePresent, WholeObjectSourcePresent, SourceMarker) -> WholeObjectSourcePresent {
            (No, No, No) => No, (Yes, _, _) => Yes,
            (No, Yes, _) => Yes, (No, No, Yes) => Yes,
        }
        // Orchestrator ruling RS7: only the transparent Oblique projects it.
        table whole_object_oblique_source(ObliqueMarker, WholeObjectSourcePresent) -> WholeObjectSourcePresent {
            (No, _) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        table coordinated_source_marker(SourceMarker, SourceMarker) -> SourceMarker {
            (Yes, Yes) => Yes,
            (_, _) => No,
        }
        table licensed_preposition_phrase_complement(PrepositionPhraseComplementLicence,
            SourceComplementUse, PrepositionPhraseComplementMarker) -> Selection {
            (Unrestricted, _, _) => Yes, (Source, Yes, _) => Yes, (With, _, With) => Yes,
        }
        table coordinated_preposition_phrase_complement_marker(PrepositionPhraseComplementMarker,
            PrepositionPhraseComplementMarker) -> PrepositionPhraseComplementMarker {
            (With, With) => With, (_, _) => None,
        }
        table coordinated_source_complement_use(SourceComplementUse, SourceComplementUse)
            -> SourceComplementUse {
            (Yes, Yes) => Yes,
            (_, _) => No,
        }
        table selected_compound_complement(CompoundComplementMarker, NominalComplementMarker)
            -> Selection { (Of, Of) => Yes, }
        table licence_adjunct(PrepositionFunctionLicence) -> Selection contains Adjunct => Yes;
        table licence_supplement(PrepositionFunctionLicence) -> Selection contains Supplement => Yes;
        table licence_medial_supplement(PrepositionFunctionLicence) -> Selection contains MedialSupplement => Yes;
        table licence_preposed_adjunct(PrepositionFunctionLicence) -> Selection contains PreposedAdjunct => Yes;
        table licence_modifier(PrepositionFunctionLicence) -> Selection contains Modifier => Yes;
        table licence_noun_complement(PrepositionFunctionLicence) -> Selection contains NounComplement => Yes;
        table licence_verb_complement(PrepositionFunctionLicence) -> Selection contains VerbComplement => Yes;
        table licence_comparative_complement(PrepositionFunctionLicence) -> Selection contains ComparativeComplement => Yes;
        table licence_predicative_complement(PrepositionFunctionLicence) -> Selection contains PredicativeComplement => Yes;
        table licence_compound(PrepositionFunctionLicence) -> Selection contains Compound => Yes;
        table coordinated_preposition_licence(PrepositionFunctionLicence, PrepositionFunctionLicence)
            -> PrepositionFunctionLicence intersection;
        frame_marker_licence Predicate(Preposition) = licence_verb_complement;
        feature RelativeGapKind { Object, TemporalAdjunct } default Object;
        feature RelativisableAdjunctClass { None, Temporal } default None combine coordinated_relativisable_adjunct_class;
        table relative_head_licence(RelativeGapKind, RelativisableAdjunctClass) -> Selection {
            (Object, None) => Yes, (Object, Temporal) => Yes,
            (TemporalAdjunct, Temporal) => Yes,
        }
        table coordinated_relativisable_adjunct_class(RelativisableAdjunctClass,
            RelativisableAdjunctClass) -> RelativisableAdjunctClass {
            (None, None) => None, (None, Temporal) => None,
            (Temporal, None) => None, (Temporal, Temporal) => Temporal,
        }
        feature FrequencyUnit { Yes }
        feature FiniteClauseComplement { Yes, Nonsequencing }
        table finite_clause_complement(FiniteClauseComplement) -> Selection {
            (Yes) => Yes, (Nonsequencing) => Yes,
        }
        feature FiniteObjectGapComplement { No, Yes } default No;
        feature ComparativeObjectGapLicence { No, Yes } default No combine all;
        feature InterveningAdjunctLicence { No, Yes } default No combine all;
        feature IrrealisClauseComplement { No, Yes } default No;
        feature VerblessClauseComplement { No, Yes, OvertSubject } default No;
        feature VerblessPredicateUse { No, Yes } default No;
        feature VerblessComplementOmissionUse { No, Yes } default No;
        table verbless_ordinary_frame_use(framing, VerblessComplementOmissionUse) -> VerblessPredicateUse {
            (Unframed, _) => Yes, (_, Yes) => Yes, (_, _) => No,
        }
        feature GerundClauseComplement { Yes }
        feature ComplementFunctionRestriction { None, Adjunct, Compound, PreposedOrPredicative } default None combine coordinated_complement_function_restriction(left.ComplementFunctionRestriction, right.ComplementFunctionRestriction, left.PrepositionFunctionLicence, right.PrepositionFunctionLicence);
        table coordinated_complement_function_restriction(ComplementFunctionRestriction, ComplementFunctionRestriction,
            PrepositionFunctionLicence, PrepositionFunctionLicence)
            -> ComplementFunctionRestriction {
            (None, None, _, _) => None, (None, Adjunct, _, _) => Adjunct,
            (Adjunct, None, _, _) => Adjunct,
            (None, Compound, _, _) => Compound, (Compound, None, _, _) => Compound,
            (Adjunct, Adjunct, _, _) => Adjunct, (Compound, Compound, _, _) => Compound,
            (None, PreposedOrPredicative, contains VerbComplement, contains PredicativeComplement) => PreposedOrPredicative,
            (PreposedOrPredicative, None, contains PredicativeComplement, contains VerbComplement) => PreposedOrPredicative,
            (PreposedOrPredicative, PreposedOrPredicative, contains PredicativeComplement, contains PredicativeComplement) => PreposedOrPredicative,
        }
        table adjunct_complement_function(ComplementFunctionRestriction, PredicativeAdjunctUse)
            -> PredicativeAdjunctUse {
            (None, No) => Yes, (None, Yes) => Yes,
            (Adjunct, No) => Yes, (Adjunct, Yes) => Yes,
            (Compound, No) => No, (Compound, Yes) => No,
            (PreposedOrPredicative, No) => No, (PreposedOrPredicative, Yes) => Yes,
        }
        table preposed_complement_function(ComplementFunctionRestriction) -> Selection {
            (None) => Yes, (Adjunct) => Yes, (PreposedOrPredicative) => Yes,
        }
        table compound_complement_function(ComplementFunctionRestriction) -> Selection {
            (None) => Yes, (Compound) => Yes,
        }
        feature InternalisedComplementMarker { No, Yes } default No combine all;
        feature InternalisedComplementPresent { No, Yes } default No combine any;
        feature ExtentMarker { Yes }
        feature LocativeUse { No, Yes } default No combine always No;
        feature AdjectiveOrigin { Ordinary, Participial } default Ordinary combine coordinated_adjective_origin;
        feature AdjectivePremodifierUse { No, Yes } default Yes combine all;
        feature AdjectiveStructure { Simple, Complemented } combine coordinated_adjective_structure;
        feature RelativeSubordinator { Yes }
        feature ParticipialUse { Ordinary, BarePassive, Mixed } combine coordinated_participial_use;
        feature PastParticipialPremodifier { No, Yes } default No;
        feature AuxiliaryComplementRealization { Overt, Elided, ProForm }
        feature ProFormComplementUse { No, Yes } default No;
        feature AuxiliaryProFormLicence { No, Yes } default No combine all;
        feature AuxiliaryEllipsisLicence { No, Yes } default No combine all;
        table licensed_auxiliary_complement(AuxiliaryEllipsisLicence,
            AuxiliaryComplementRealization) -> Selection {
            (No, Overt) => Yes, (Yes, Overt) => Yes, (Yes, Elided) => Yes,
            (Yes, ProForm) => Yes,
        }
        table licensed_auxiliary_pro_form(AuxiliaryProFormLicence,
            AuxiliaryComplementRealization) -> Selection {
            (No, Overt) => Yes, (No, Elided) => Yes,
            (Yes, Overt) => Yes, (Yes, Elided) => Yes, (Yes, ProForm) => Yes,
        }
        feature Clitic { No, Yes } default No;
        feature CliticHost { Free, Subject, PronounSubject } combine left.CliticHost;
        feature SubjectStructure { Phrase, Pronoun } combine always Phrase;
        feature PredicateFrameUse { No, Yes } default Yes;
        feature ScalarVariable { Yes }
        feature ScalarSign { Absent, Plus, Hyphen, Minus } default Absent;
        table loyalty_sign(ScalarSign) -> Selection { (Plus) => Yes, (Minus) => Yes, }
        feature MeasureOperator { No, Yes } default No;
        feature ScalarDenotation { No, Yes } default No combine all;
        feature MeasurePosition { None, Before } default None combine coordinated_measure_position;
        feature UnitBearing { No, Yes } default No combine any;
        table nominal_unit_bearing(MeasurePosition) -> UnitBearing {
            (None) => No, (Before) => Yes,
        }
        table arithmetic_unit_bearing(UnitBearing, UnitBearing) -> UnitBearing {
            (No, No) => No,
            (_, _) => Yes,
        }
        feature ScalarMeasurementHeadUse { No, Yes } default No combine always No;
        feature PropertyValuePosition { None, Single, First, Second, Pair } default None
            combine coordinated_property_value_position(
                coordinator.CoordinationKind, left.PropertyValuePosition, right.PropertyValuePosition);
        feature PropertyEqualityForm { None, Unquantified, Quantified } default None;
        feature CardinalMeasurementUse { No, DeterminerRequired, DeterminerProjection, Yes } default No combine coordinated_cardinal_measurement;
        feature MeasureKind { Scalar, Pair } combine same;
        feature MatrixComparisonUse { No, Yes } default No;
        feature ComparativeComplementForm { Nominal, BareInfinitival, GerundParticipial }
        table nonfinite_comparative_form(form) -> ComparativeComplementForm {
            (Plain) => BareInfinitival, (GerundParticiple) => GerundParticipial,
        }
        table matched_nonfinite_comparison(form, ComparativeComplementForm) -> Selection {
            (Plain, BareInfinitival) => Yes, (GerundParticiple, GerundParticipial) => Yes,
        }
        feature ComparisonMarker { Equality, Ordering } combine coordinated_comparison_marker;
        feature ExpandedComparisonMarker { Equality, Ordering }
        feature KeywordParameterClass { Nullary, Amount, Cost, Quality, Subject, Name, AmountCost,
            QualityCost, QualityAmount, Ability, Condition, CostPowerToughness, Instruction }
        table bare_keyword_use(KeywordParameterClass) -> Selection {
            (Nullary) => Yes, (Amount) => Yes, (Cost) => Yes, (AmountCost) => Yes,
        }
        feature CostComponentForm { Empty, Symbols, Action } default Empty;
        table final_cost_component(CostComponentForm, CostComponentForm) -> CostComponentForm {
            (Symbols, Empty) => Symbols, (Action, Empty) => Action,
            (_, Symbols) => Symbols, (_, Action) => Action,
        }
        separator keyword_cost_terminal(CostComponentForm, terminal_punctuation) {
            (Symbols, None) => "", (Action, None) => ".", (Action, QuotedFullStop) => "",
        }
        feature NominalBareClass { None, Interval, Boundary }
        feature NominalAdjunctClass { None, Temporal, Manner } default None combine nominal_adjunct_concord;
        feature SlashPremodifierUse { No, Yes } default Yes combine all;
        feature NominalDependentPresent { No, Yes } default No combine any;
        feature NominalAdjunctDependent { None, Demonstrative, Temporal, Definite, DeicticTemporal } default None;
        feature BareSingularUse { No, Yes } default No;
        feature RelativeUse { No, Yes } default No;
        feature ExistentialUse { No, Yes } default No;
        feature KeywordMarker { None, Among, For, From, Onto, To, Into, At, With, Of, On, Under } combine same;
        feature KeywordPayloadOrder { AfterHead, BeforeHead, BoundSuffix }
        feature BoundKeyword { No, Yes }
        feature CreatureSubtypeStem { No, Yes } default No;
        feature CreatureSubtypeStemUse { No, Yes } default No;
        feature KeywordSeparator { Space, Dash, SpacedDash }
        feature KeywordParameterSeparator { Space, Dash, SpacedDash }
        feature KeywordQualityNumber { Any, Singular, Plural } combine coordinated_quality;
        feature Selection { Yes }
        feature Function { Genitive }
        feature HostEnding { Default, PluralS }
        feature BareGenitiveHost { No, Yes } default No combine always No;
        feature LabelKind { None, AbilityWord, FlavorWord }
        feature SymbolUse { Cost }
        feature ManaSymbolUse { No, Yes } default No;
        feature SymbolNominalUse { No, Count, Mass } default No;
        table symbol_nominal_use(SymbolNominalUse) -> Selection {
            (Count) => Yes, (Mass) => Yes,
        }
        table symbol_nominal_sequence(SymbolNominalUse, SymbolNominalUse) -> Selection {
            (Count, No) => Yes, (Count, Count) => Yes, (Mass, No) => Yes,
        }
        table symbol_nominal_countability(SymbolNominalUse) -> countability {
            (Count) => Count, (Mass) => Mass,
        }
        // QS2/QS7: notation licensed as mana or a mass symbol has one
        // number-neutral quantified use; generic cost notation retains Head.
        table symbol_nominal_agreement(SymbolNominalUse, ManaSymbolUse) -> NominalAgreementNumber {
            (Count, Yes) => SingularOrPlural, (Mass, _) => SingularOrPlural,
            (Count, No) => Head,
        }
        table symbol_measure_position(NominalAgreementNumber) -> MeasurePosition {
            (Head) => None, (SingularOrPlural) => Before,
        }
        feature TypeLineRole { Supertype, CardType, Subtype }
        feature NameNumber { Singular, Plural } default Singular;
        table name_number(NameNumber) -> number {
            (Singular) => Singular, (Plural) => Plural,
        }
        feature IdentityUse { Name }
        feature NominalComplementMarker { None, Of, To } default None combine always None;
        feature TaxonomicComplementUse { No, Yes } default No;
        table nominal_complement_marker(NominalComplementMarker, NominalComplementMarker) -> Selection {
            (None, None) => Yes, (Of, Of) => Yes, (To, To) => Yes,
        }
        feature BarePrepositionUse { No, Location, SpatialEdge } default No combine always No;
        feature BareNominalComplement { No, Location, SpatialEdge } default No;
        table bare_preposition_use(BarePrepositionUse) -> Selection {
            (Location) => Yes, (SpatialEdge) => Yes,
        }
        table bare_nominal_complement(BareNominalComplement, BarePrepositionUse) -> Selection {
            (Location, Location) => Yes, (SpatialEdge, SpatialEdge) => Yes,
        }
        feature SelectedPrepositionUse { No, Yes } default No combine any;
        table selected_preposition_complement(SelectedPrepositionUse, SelectedPrepositionUse)
            -> SelectedPrepositionUse {
            (Yes, Yes) => Yes,
            (_, _) => No,
        }
        table nominal_postmodifier_licence(NominalComplementMarker, NominalComplementMarker)
            -> Selection {
            (None, None) => Yes, (None, Of) => Yes, (Of, None) => Yes,
            (None, To) => Yes, (Of, To) => Yes, (To, None) => Yes, (To, Of) => Yes,
        }
        feature NounPremodifier { Yes, Attribute, Keyword, None } combine coordinated_noun_premodifier;
        feature PremodifierCoordination { No, Yes } default No combine always Yes;
        feature PremodifierCategory { Nominal, Adjectival, Verbal, Mixed }
        feature SyndeticPremodifier { No, Yes }
        table premodifier_category(PremodifierCategory, PremodifierCategory)
            -> PremodifierCategory {
            (Nominal, Nominal) => Nominal, (Adjectival, Adjectival) => Adjectival,
            (Verbal, Verbal) => Verbal, (_, _) => Mixed,
        }
        table serial_premodifier_licence(PremodifierCategory, PremodifierCategory,
            SyndeticPremodifier) -> Selection {
            (Nominal, Nominal, No) => Yes, (Nominal, Adjectival, No) => Yes,
            (Adjectival, Nominal, No) => Yes, (Adjectival, Adjectival, No) => Yes,
            (Nominal, Mixed, No) => Yes, (Adjectival, Mixed, No) => Yes,
            (Adjectival, Verbal, Yes) => Yes, (Verbal, Adjectival, Yes) => Yes,
            (Adjectival, Mixed, Yes) => Yes, (Verbal, Mixed, Yes) => Yes,
        }
        table unlike_premodifiers(PremodifierCategory, PremodifierCategory) -> Selection {
            (Adjectival, Verbal) => Yes, (Verbal, Adjectival) => Yes,
        }
        table syndetic_premodifier_member(PremodifierCategory) -> Selection {
            (Adjectival) => Yes, (Verbal) => Yes,
        }
        feature AttributePremodifierHead { No, Yes } default No combine all;
        feature KeywordPremodifierHead { No, Yes } default No combine all;
        table noun_premodifier_use(NounPremodifier) -> Selection {
            (Yes) => Yes, (Attribute) => Yes,
        }
        table licensed_noun_premodifier(NounPremodifier, AttributePremodifierHead,
            KeywordPremodifierHead) -> Selection {
            (Yes, _, _) => Yes, (Attribute, Yes, _) => Yes,
            (Keyword, _, Yes) => Yes, (None, _, _) => Yes,
        }
        table coordinated_noun_premodifier(NounPremodifier, NounPremodifier)
            -> NounPremodifier {
            (Yes, Yes) => Yes, (Yes, Attribute) => Attribute,
            (Attribute, Yes) => Attribute, (Attribute, Attribute) => Attribute,
            (None, None) => None, (None, Yes) => Yes, (Yes, None) => Yes,
            (None, Attribute) => Attribute, (Attribute, None) => Attribute,
            (Keyword, Keyword) => Keyword, (Keyword, Yes) => Keyword,
            (Yes, Keyword) => Keyword, (Keyword, None) => Keyword, (None, Keyword) => Keyword,
        }
        feature VPFinalAdjunct { Yes, No } combine all;
        feature ClauseInitialAdjunct { Yes, No } default No combine all;
        feature CardinalModifierUse { Yes, No } default No;
        feature CentralAdjunctLicence { Preverbal, PostCopula, PostAuxiliary, Imperative } set;
        table central_preverbal(CentralAdjunctLicence) -> Selection contains Preverbal => Yes;
        table central_postcopula(CentralAdjunctLicence) -> Selection contains PostCopula => Yes;
        table central_postauxiliary(CentralAdjunctLicence) -> Selection contains PostAuxiliary => Yes;
        table central_imperative(CentralAdjunctLicence) -> Selection contains Imperative => Yes;
        feature CentralAdjunctPosition { Preverbal, PostCopula, PostAuxiliary } default Preverbal;
        feature InfinitivalMarker { Yes }
        feature SecondaryNegation { Yes }
        feature FrameUse { Object, Predicative, Locative, Mana, Amount, Measure, SlashMeasure,
            GrantedAbility, AuxiliaryBare, AuxiliaryParticiple, AuxiliaryPerfect, ObjectName,
            QuantitativeDeterminer, Infinitive, PassiveNominal, PassiveInternalised } combine shared_frame_concord;
        feature HeadCoordination { No, Yes } combine always Yes;
        feature SelectedTailLicence { ParallelFrames, DeclaredFrames } default ParallelFrames;
        feature BareCoordination { No, Yes } default Yes;
        feature CommaCoordination { No, Yes } default No;
        feature GeneralCoordination { No, Yes } default Yes combine coordinator.GeneralCoordination;
        feature UnmarkedConjunctLicence { No, Yes } default Yes combine always Yes;
        feature NoncorrelativeCoordination { Yes, No }
        feature CorrelativeKind { Both, Either, Neither }
        feature CorrelativeCoordinator { And, Or, Nor }
        // The default partial judgments reject like pairs; helpers use the total tables.
        feature PredicativeKind { Adjectival, Nominal, Prepositional, Mixed } combine unlike_predicative_kind;
        feature DepictiveKind { Adjectival, Participial, Mixed } combine unlike_depictive_kind;
        feature NominalLicense { AnyNominal }
        feature KeywordComplement { Yes }
        feature QuotedComplement { Yes }
        feature QuotationStructure { Simple, Coordinated } combine always Coordinated;
        feature GrantedAbilityKind { Keyword, Quoted } combine combined_ability_kind;
        feature AbilityTermination { Complete, Unterminated } default Complete;
        feature SentenceLineList { No, Yes }
        feature ModeListLayout { DashHeader, SentenceHeader }

        category Document();
        category Ability(head LabelKind, head AbilityTermination, head SentenceLineList);
        category AbilityContinuation();
        category Paragraph();
        category ParagraphItem();
        category ParagraphContinuation();
        category SentenceParagraph();
        category SentenceContinuation();
        category Sentence(head AbilityTermination);
        category Clause(
            head ClauseCoordinationHost, head UnmarkedConjunctLicence, head SourceComplementSelection
        );
        category ImperativeClause(
            head ClauseCoordinationHost, head UnmarkedConjunctLicence, head SourceComplementSelection
        );
        category FiniteClause(head SourceComplementSelection);
        category FiniteGappedClause();
        category InterveningPrepositionAdjunct();
        category IrrealisClause();
        category IrrealisPredicate(head number, head person, head ScalarDenotation);
        category CentralAdverbSelectedHead(
            head number, head person, head CliticHost, head PurposeAdjunctLicence,
            head MatrixComplementLicence, head SourceComplementSelection, head frame
        );
        category ExistentialExtension();
        category DummyPronoun();
        category ExistentialPredicate(
            local NominalAdjunctHost, local ClauseComplementBoundary, head number,
            head person, head CliticHost, head SourceComplementSelection
        );
        category FinitePredicate(
            local NominalAdjunctHost,
            local ClauseComplementBoundary combine always None,
            head number combine same,
            head person combine same,
            head CliticHost, head PurposeAdjunctLicence, head MatrixComplementLicence,
            head SourceComplementSelection
        );
        category QuantifiedAuxiliaryHead(
            head number, head person, head frame, head CliticHost,
            head PurposeAdjunctLicence, head MatrixComplementLicence, head SourceComplementSelection
        );
        category ScalarLocationPredicate(
            head number, head person, head CliticHost, head PurposeAdjunctLicence, head MatrixComplementLicence,
            head SourceComplementSelection
        );
        category BareScalarLocationPredicate(
            head PurposeAdjunctLicence, head MatrixComplementLicence, head SourceComplementSelection
        );
        category BareScalarLocationComplement(
            head AuxiliaryComplementRealization, head PurposeAdjunctLicence, head MatrixComplementLicence,
            head SourceComplementSelection
        );
        category SecondaryVerbPhrase(
            head ParticipialSupplementUse, local NominalAdjunctHost,
            local ClauseComplementBoundary combine always None,
            head form combine same,
            head ParticipialUse, head InternalisedComplementPresent, head PurposeAdjunctLicence,
            head MatrixComplementLicence, head SourceComplementSelection
        );
        category FiniteDepictiveHost(
            head number, head person, head CliticHost, head PurposeAdjunctLicence,
            head MatrixComplementLicence, head SourceComplementSelection
        );
        category SecondaryDepictiveHost(
            head ParticipialSupplementUse, head form, head ParticipialUse,
            head InternalisedComplementPresent, head PurposeAdjunctLicence, head MatrixComplementLicence,
            head SourceComplementSelection
        );
        category BarePredicate(
            head PurposeAdjunctLicence, head MatrixComplementLicence, head SourceComplementSelection
        );
        category ParticipialPredicate(
            head PurposeAdjunctLicence, head MatrixComplementLicence, head SourceComplementSelection,
            head CentralAdjunctPosition
        );
        category PastParticiplePredicate(
            head PurposeAdjunctLicence, head MatrixComplementLicence, head SourceComplementSelection
        );
        category PredicativeComplement(head PredicativeKind, head PredicativeAdjunctUse);
        category DepictivePhrase(head DepictiveKind);
        category Ellipsis(head form);
        category BareComplement(
            head AuxiliaryComplementRealization, head PurposeAdjunctLicence, head MatrixComplementLicence,
            head SourceComplementSelection
        );
        category ParticipialComplement(
            head PurposeAdjunctLicence, head MatrixComplementLicence, head SourceComplementSelection
        );
        category PerfectComplement(
            head PurposeAdjunctLicence, head MatrixComplementLicence, head SourceComplementSelection
        );
        category Nominal(
            head ComparativeModifierPresent,
            head AttributePremodifierHead,
            head KeywordPremodifierHead,
            head number combine same,
            head NominalAgreementNumber,
            head countability combine same,
            head Targeting combine same,
            head NominalAdjunctClass,
            local NominalDependentPresent,
            head RelativisableAdjunctClass,
            head SlashPremodifierUse,
            head NumberTransparency,
            head ObliqueNumber,
            head BareGenitiveHost,
            head DeterminerRequirement,
            head MeasurePosition,
            local ScalarMeasurementHeadUse,
            local PropertyValuePosition,
            head CardinalMeasurementUse,
            head BarePrepositionUse,
            head NominalComplementMarker,
            head SelectedPrepositionUse,
            head PredicativeAdjunctUse,
            head ScalarDenotation,
            head SourceModifierPresent,
            head WholeObjectSourcePresent,
            head SubjectRelativeNumber,
            head UnmarkedConjunctLicence
        );
        category VerbalPremodifier();
        category NounPremodifier(head NounPremodifier, local PremodifierCoordination);
        category PartitiveModifier();
        category NounPhrase(
            head NominalDependentPresent combine always Yes,
            head PartitiveCountability,
            local NounPhraseStructure,
            local FocusedUse,
            head UnitBearing,
            head number combine determined_np_coordinate_number(
                coordinator.CoordinationKind, left.number, right.number,
                left.DeterminerAgreementNumber, right.DeterminerAgreementNumber),
            head person combine coordinate_person(coordinator.CoordinationKind, left.person, right.person),
            head CaseUse,
            head Targeting,
            head BareGenitiveHost,
            head SubjectStructure,
            head BarePrepositionUse combine same,
            head SelectedPrepositionUse,
            head PredicativeAdjunctUse,
            head ScalarDenotation,
            head SourceModifierPresent,
            head WholeObjectSourcePresent,
            head DeterminerAgreementNumber,
            head ComparativePostmodifierUse,
            head Definiteness,
            head LocativeUse
        );
        category NominativePhrase(
            head number, head person, head DeterminerAgreementNumber, head CaseUse, head SubjectStructure,
            head ScalarDenotation
        );
        category AccusativePhrase(
            head NominalDependentPresent, head PartitiveCountability, head number, head person,
            head CaseUse, head SelectedPrepositionUse, head PredicativeAdjunctUse,
            head SourceModifierPresent, head WholeObjectSourcePresent
        );
        category AdjectivePhrase(
            head AdjectiveComplementClass, head AdjectiveAttributiveOnlyUse, head ImplicitComparativeUse,
            head AdjectiveStructure, head AdjectiveOrigin, head AdjectivePremodifierUse,
            head TargetingPremodifierUse, local PremodifierCoordination
        );
        category Premodifier(
            head PremodifierCategory, head SyndeticPremodifier, head NounPremodifier, head AdjectiveOrigin,
            head TargetingPremodifierUse, head SelectedPrepositionUse, head PredicativeAdjunctUse
        );
        category PremodifierCoordination(
            head PremodifierCategory, head SyndeticPremodifier, head NounPremodifier, head AdjectiveOrigin,
            head TargetingPremodifierUse, head SelectedPrepositionUse, head PredicativeAdjunctUse
        );
        category VerblessClause(head VerblessClauseComplement);
        category Name(head number);
        category NamePredicate();
        category PrepositionPhrase(
            local PartitiveCountability, head InterveningAdjunctLicence, head ConnectiveAdjunctUse,
            local NominalAdjunctHost, local ClauseComplementBoundary,
            head LocativeUse combine all,
            head PrepositionFunctionLicence, head ComplementFunctionRestriction, local ObliqueNumber,
            head InternalisedComplementMarker, head NominalComplementMarker, head SelectedPrepositionUse,
            head PredicativeAdjunctUse, head ClauseInitialAdjunct, local FocusedUse,
            head PrepositionPhraseComplementMarker, head SourceComplementUse, head SourceModifierPresent,
            head WholeObjectSourcePresent, head SourceMarker, head MatrixComplementRequirement,
            head NominalExceptionComplementUse
        );
        category SourceComplement();
        category LocativeComplement();
        category MannerComplement();
        category FrequencyPhrase();
        category NominalAdjunctPhrase(head NominalAdjunctClass, head NominalAdjunctDependent);
        category BareTemporalNominal(head NominalBareClass);
        category BoundaryComplement();
        category SubjectRelativeClause(head number);
        category ObjectRelativeClause(head RelativeGapKind);
        category FiniteObjectGap(
            local NominalAdjunctHost,
            local ClauseComplementBoundary combine always None,
            head number combine same,
            head person combine same,
            head CliticHost, head PurposeAdjunctLicence, head MatrixComplementLicence,
            head SourceComplementSelection, head ComparativeObjectGapLicence
        );
        category BareObjectGap(
            local NominalAdjunctHost,
            local ClauseComplementBoundary combine always None,
            head PurposeAdjunctLicence,
            head MatrixComplementLicence, head SourceComplementSelection, head ComparativeObjectGapLicence
        );
        category PastParticipleObjectGap(
            local NominalAdjunctHost,
            local ClauseComplementBoundary combine always None,
            head PurposeAdjunctLicence,
            head MatrixComplementLicence, head SourceComplementSelection, head ComparativeObjectGapLicence
        );
        category GerundParticipleObjectGap(
            head ComparativeObjectGapLicence
        );

        category Cardinal(
            head number combine cardinal_coordinate_number(coordinator.CoordinationKind, left.number, right.number),
            head CardinalComplementUse
        );
        category CardinalModifier();
        category SecondaryNegator();
        category SecondaryNegatorHead();
        category PrimaryNegator();
        category NegatedAuxiliaryHead(
            head PredicateFrameUse, head number, head person, head CliticHost,
            head PurposeAdjunctLicence, head MatrixComplementLicence, head SourceComplementSelection,
            head frame
        );
        category QuantitativeAdjectiveComplement();
        category QuantitativeDeterminer(
            head DeterminerUse, head DeterminerAgreementNumber, head Definiteness,
            head NominalAdjunctDependent, head DegreeUse
        );
        category CardinalPrepositionPhrase(head number);
        category ArithmeticModifier();
        category QuantitativePrepositionPhrase(head number);
        category DegreePrepositionPhrase(head DeterminerUse, head DeterminerAgreementNumber);
        category QuantitativeDegreePrepositionPhrase(head DeterminerUse, head DeterminerAgreementNumber);
        category ExceptionPrepositionPhrase(head DeterminerUse, head DeterminerAgreementNumber);
        category ExceptionDegreePrepositionPhrase(head DeterminerUse, head DeterminerAgreementNumber);
        category ComparativePrepositionPhrase(head number);
        category CountedFrequencyPrepositionPhrase();
        category AdverbFrequencyPrepositionPhrase();
        category QuantityConjunct(head number, head CardinalComplementUse, head CardinalMeasurementUse);
        category ComparativeQuantity(
            head number, head ComparativeQuantityStructure, head MeasureModifierUse, head FrequencyUse
        );
        category ScalarLocationComplement();
        category HollowInfinitivalComplement();
        category PurposeInfinitival(local FocusedUse);
        category ScalarValue(head PredicativeKind, head PropertyEqualityForm);
        category PropertyValue(head MeasureKind, head PropertyEqualityForm);
        category Amount();
        category MeasurePhrase(head MeasureKind, head UnitBearing);
        category ArithmeticOperand(head UnitBearing);
        category ScalarMeasurePhrase();
        category ScalarExtentComplement();
        category ScalarExtentMeasure();
        category UnsignedScalar();
        category ScalarComponent(head ScalarSign);
        category SlashPair(head AdditiveModifierUse);
        category ComparativeGovernorHead(head ComparisonMarker);
        category ComparativeComplement(head AdjectivePremodifierUse);
        category MatrixComparativeComplement(head ComparisonMarker, head ComparativeComplementForm);
        category MatrixComparativePhrase(head ComparativeComplementForm);
        category ComparativeAdjectivePhrase(
            head AdjectivePremodifierUse, head PropertyEqualityForm
        );
        category DiscontinuousComparativeGovernor(
            head frame combine coordinated_discontinuous_frame,
            head ImplicitComparativeUse
        );
        category ComparativeDeterminerHead(
            head ComparativeDeterminerUse, head DeterminerUse, head NominalAdjunctDependent
        );
        category KeywordPhrase();
        category SelectedKeywordPreposition(head NominalComplementMarker);
        category KeywordQuality(head KeywordQualityNumber);
        category KeywordQualityPreposition(head KeywordMarker, head KeywordQualityNumber);
        category SelectedComplementTail(head HeadCoordination);
        category SelectedComplementTailSeries(head HeadCoordination);
        category KeywordContinuation();
        category Cost(head CostComponentForm);
        category CostComponent(head CostComponentForm);
        category CostContinuation(head CostComponentForm);
        category CostSymbols();
        category CostSymbol();
        category ManaPhrase();
        category ManaSubject(
            head number, head person, head SubjectStructure, head DeterminerAgreementNumber
        );
        category ManaPassivePredicate(
            head FrameUse, head HeadCoordination, head form, head PurposeAdjunctLicence,
            head MatrixComplementLicence, head SourceComplementSelection
        );
        category ManaPassiveComplement(
            head PurposeAdjunctLicence, head MatrixComplementLicence, head SourceComplementSelection
        );
        category FiniteManaPassivePredicate(
            head number, head person, head CliticHost, head PurposeAdjunctLicence, head MatrixComplementLicence,
            head SourceComplementSelection
        );
        category ManaSymbol();
        category CountNominalSymbol(head SymbolNominalUse);
        category NominalSymbol(head SymbolNominalUse, head NominalAgreementNumber);
        category Parenthetical();
        category QuotedText(head QuotationStructure);
        category GrantedAbility(head GrantedAbilityKind);
        category Mode(head LabelKind);
        category Modes(head ModeListLayout);
        category ModeContinuation();
        category NamedModes(head ModeListLayout);
        category NamedModeContinuation();
        category TypeLine();
        category SupertypePrefix();
        category CardTypes();
        category CardTypeContinuation();
        category Subtypes();
        category SubtypeContinuation();
        category AdverbPhrase(
            local NominalAdjunctHost, head VPFinalAdjunct, head ClauseInitialAdjunct, head DurationUse,
            head UnmarkedConjunctLicence, local FocusedUse, head FrequencyUse
        );
        category InfinitiveComplement();
        category FiniteSelectedHead(
            head number combine same,
            head person combine same,
            head FrameUse, head HeadCoordination, head CliticHost,
            head AuxiliaryEllipsisLicence, head AuxiliaryProFormLicence, head PurposeAdjunctLicence,
            head MatrixComplementLicence, head SourceComplementSelection
        );
        category SecondarySelectedHead(
            head form combine same,
            head FrameUse, head HeadCoordination, head AuxiliaryEllipsisLicence,
            head AuxiliaryProFormLicence, head PurposeAdjunctLicence, head MatrixComplementLicence,
            head SourceComplementSelection
        );
        category FocusedAdjunctMember(
            head NominalAdjunctHost, head ClauseComplementBoundary,
            head ConnectiveAdjunctUse, head SourceMarker, head PrepositionPhraseComplementMarker,
            head SourceComplementUse
        );
        category FocusedAdjunctSeries(
            head NominalAdjunctHost, head ClauseComplementBoundary,
            head ConnectiveAdjunctUse, head SourceMarker, head PrepositionPhraseComplementMarker,
            head SourceComplementUse
        );
        category FocusedAdjunctCoordination(
            head NominalAdjunctHost, head ClauseComplementBoundary,
            head ConnectiveAdjunctUse, head SourceMarker, head PrepositionPhraseComplementMarker,
            head SourceComplementUse, head PrepositionFunctionLicence, head ComplementFunctionRestriction,
            head PredicativeAdjunctUse, head SelectedPrepositionUse, head MatrixComplementRequirement,
            head InternalisedComplementMarker
        );
        category CoordinatedFiniteClause(head GeneralCoordination);
        category CorrelativeAdjectivePhrase(head AdjectiveAttributiveOnlyUse);
        category SelectedPrepositionHead(
            head LocativeUse combine all,
            head PrepositionFunctionLicence, head HeadCoordination,
            head InternalisedComplementMarker, head NominalComplementMarker, head SelectedPrepositionUse,
            head PredicativeAdjunctUse, head PrepositionPhraseComplementMarker, head SourceComplementUse,
            head SourceModifierPresent, head WholeObjectSourcePresent, head SourceMarker
        );

        frame_category NounPhrase = AccusativePhrase;
        // Owner rulings HO1/HO4 and HO8 (2026-10-11): frame shape and
        // the Object’s declared internal structure license postposing.
        // Heaviness is structural; the licence does not test verb identity.
        frame_category HeavyObject = AccusativePhrase where NominalDependentPresent = Yes;
        frame_postposing Predicate object HeavyObject after [PrepositionPhrase, SourceComplement, LocativeComplement] with DepictivePhrase;
        // PW-R1 (orchestrator ruling, 2026-10-10): a selected frame
        // takes the overt subject–predicate form, not reducible if able.
        frame_category VerblessClause = VerblessClause where VerblessClauseComplement = OvertSubject;
        frame_category SourceFreeObject = AccusativePhrase where SourceModifierPresent = No;
        // AR1–AR2 (orchestrator ruling, 2026-10-10): an intervening
        // Adjunct follows the Object only before this declared source role.
        frame_adjunct SourceComplement = InterveningPrepositionAdjunct;
        frame_category WholeSourceFreeObject = AccusativePhrase where WholeObjectSourcePresent = No;
        frame_category NominalPredicativePreposition = PrepositionPhrase where ComplementFunctionRestriction = PreposedOrPredicative;
        frame_category PowerToughnessAdjustment = SlashPair;
        frame_category MeasurePhrase = ScalarMeasurePhrase;
        // Copular scalar-location frames select a nominal value. Comparative
        // AdjPs retain their existing adjectival predicative route.
        frame_category ScalarValue = ScalarValue where PredicativeKind = Nominal;

        schema SelectedPredicate {
            form [head: lexical(Verb), complements: selected_frame(head, Predicate)];
            require head.PredicateFrameUse = Yes;
        }
        instance SelectedPredicate<Result, Properties>: [
            (FinitePredicate, FiniteHeadAgreement),
            (IrrealisPredicate, IrrealisHeadAgreement),
            (SecondaryVerbPhrase, OrdinarySecondaryHead),
        ] {
            use Properties;
        }
        // NB1–NB4: analytic verbal negation belongs to a positive finite
        // auxiliary head; its ordinary selected Complement is unchanged.
        construction NegatedAuxiliaryHead: NegatedAuxiliaryHead {
            form [head: lexical(Verb), " ", negator: PrimaryNegator];
            export frame = head.frame;
            require head.AuxiliaryUse = Yes;
            require head.Polarity = Positive;
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        construction NegatedSelectedPredicate: FinitePredicate {
            require head.PredicateFrameUse = Yes;
            form [head: NegatedAuxiliaryHead, complements: selected_frame(head, Predicate)];
        }

        construction NegatedParticipialPredicate: FinitePredicate {
            form [head: NegatedAuxiliaryHead, complement: ParticipialComplement];
            require head.frame = ParticipialAuxiliary;
            use SharedAuxiliaryAgreement;
        }

        // The ticket's central-position analysis: the lexical declaration
        // chooses the site; the Adverb is an Adjunct of the predicate head.
        schema CentralAdverbHead {
            form [modifier: lexical(Adverb), " ", head: lexical(Verb)]
                require head.CentralAdjunctPosition = Preverbal
                require central_preverbal(modifier.CentralAdjunctLicence) = Yes;
            form [head: lexical(Verb), " ", modifier: lexical(Adverb)]
                require head.CentralAdjunctPosition = PostCopula
                require central_postcopula(modifier.CentralAdjunctLicence) = Yes;
            require head.PredicateFrameUse = Yes;
            export frame = head.frame;
        }
        instance CentralAdverbHead<Result, Properties>: [
            (CentralAdverbSelectedHead, FiniteHeadAgreement),
        ] { use Properties; }
        construction CentralAdverbPredicate: FinitePredicate {
            form [head: CentralAdverbSelectedHead, complements: selected_frame(head, Predicate)];
        }

        construction ScalarMeasurePhrase: ScalarMeasurePhrase {
            form [head: MeasurePhrase];
            require head.MeasureKind = Scalar;
        }

        frame Intransitive = Predicate();
        frame Transitive = Predicate(Object(NounPhrase));
        frame ManaComplement = Predicate(Complement(ManaPhrase));
        frame ObjectName = Predicate(Object(NounPhrase), Complement(Name));
        frame BareNominal = Nominal();
        frame BoundKeywordStem = BoundKeywordStem();
        frame NominalSymbols = Nominal(Marked(Preposition, Of, Complement(CostSymbols)));
        frame NominalInfinitive = Nominal(Complement(InfinitiveComplement));
        frame NominalKeywordPreposition = Nominal(Complement(SelectedKeywordPreposition));
        frame NominalPreposition = Nominal(Marked(Preposition, Of, Complement(NounPhrase)));
        frame NominalToPreposition = Nominal(Marked(Preposition, To, Complement(NounPhrase)));
        frame NominalColorWordPreposition = Nominal(Marked(Preposition, To, Complement(AdjectivePhrase)));
        frame Predicative = Predicate(Complement(PredicativeComplement));
        frame NominalPredicativePreposition = Predicate(Complement(NominalPredicativePreposition));
        frame Existential = Existential(Complement(NounPhrase), Optional(Complement(LocativeComplement)));
        frame AdjectivalPrepositionComplement = Adjectival(
            Marked(Preposition, By, Complement(NounPhrase)));
        frame AdjectivalInfinitive = Adjectival(Complement(InfinitiveComplement));
        frame Locative = Predicate(Complement(LocativeComplement));
        frame ObjectLocative = Predicate(Object(NounPhrase), Complement(LocativeComplement));
        frame BareAuxiliary = Auxiliary(Complement(BarePredicate));
        frame ParticipialAuxiliary = Auxiliary(Complement(ParticipialPredicate));
        frame PerfectAuxiliary = Auxiliary(Complement(PastParticiplePredicate));
        frame Measure = Predicate(Complement(MeasurePhrase));
        frame SlashMeasure = Predicate(Complement(PowerToughnessAdjustment));
        frame Equality = Predicate(Marked(Preposition, To, Complement(ComparativeComplement)));
        frame DiscontinuousEquality = Predicate(Marked(Preposition, As, Complement(ComparativeComplement)));
        frame Ordering = Predicate(Marked(Preposition, Than, Complement(ComparativeComplement)));
        frame GrantedAbilityComplement = Predicate(Complement(GrantedAbility));
        frame AmountComplement = Predicate(Complement(Amount));
        frame QuantitativeDeterminerComplement = Predicate(Complement(QuantitativeDeterminer));
        frame InfinitiveSelection = Predicate(Complement(InfinitiveComplement));

        frame SourceFreeObjectSourceToObject = Predicate(Object(SourceFreeObject), Complement(SourceComplement), Preposition(To), Object(NounPhrase), Optional(Marked(Preposition, Under, Complement(NounPhrase))));
        frame SourceFreeObjectToObject = Predicate(Object(SourceFreeObject), Preposition(To), Object(NounPhrase));
        frame ObjectToObject = Predicate(Object(NounPhrase), Preposition(To), Object(NounPhrase));
        frame ObjectIntoObject = Predicate(
            Object(NounPhrase), Preposition(Into), Object(NounPhrase)
        );
        frame ObjectOntoObject = Predicate(
            Object(NounPhrase), Preposition(Onto), Object(NounPhrase)
        );
        frame ObjectForObject = Predicate(Object(NounPhrase), Preposition(For), Object(NounPhrase));
        frame ObjectAmongComplement = Predicate(Object(NounPhrase), Marked(Preposition, Among, Complement(NounPhrase)));
        frame AtObject = Predicate(Preposition(At), Object(NounPhrase));
        frame Ditransitive = Predicate(Object(NounPhrase), Object(NounPhrase));
        frame ObjectExtent = Predicate(Object(NounPhrase), Complement(ScalarExtentComplement));

        table tensed_form(form) -> Selection {
            (Present) => Yes, (Preterite) => Yes,
        }

        // Plain form is shared by finite imperatives and nonfinite infinitivals.
        table secondary_form(form) -> form {
            (Plain) => Plain,
            (GerundParticiple) => GerundParticiple,
            (PastParticiple) => PastParticiple,
        }

        // The ticket's corpus restriction retires single-referent additive
        // Subject agreement; each coordinate keeps its determiner-selection Number.
        table coordinated_nominal_agreement(CoordinationKind, NominalAgreementNumber)
            -> NominalAgreementNumber {
            (Additive, Head) => Plural, (Additive, Plural) => Plural,
            (Alternative, Head) => Head, (Alternative, Plural) => Plural,
            (Additive, SingularOrPlural) => Plural,
            (Alternative, SingularOrPlural) => SingularOrPlural,
        }
        table nominal_agreement_number(NominalAgreementNumber, number) -> number {
            (Head, Singular) => Singular, (Head, Plural) => Plural,
            (Plural, Singular) => Plural, (Plural, Plural) => Plural,
            (SingularOrPlural, Singular) => Singular,
            (SingularOrPlural, Plural) => Plural,
        }

        table nominal_relative_agreement(NominalAgreementNumber, number, number) -> Selection {
            (Head, Singular, Singular) => Yes, (Head, Plural, Plural) => Yes,
            (Plural, Singular, Plural) => Yes, (Plural, Plural, Plural) => Yes,
            (Plural, Singular, Singular) => Yes, (Plural, Plural, Singular) => Yes,
            (SingularOrPlural, _, _) => Yes,
        }

        table coordinate_person(CoordinationKind, person, person) -> person {
            (Additive, First, First) => First,
            (Additive, First, Second) => First,
            (Additive, First, Third) => First,
            (Additive, Second, First) => First,
            (Additive, Second, Second) => Second,
            (Additive, Second, Third) => Second,
            (Additive, Third, First) => First,
            (Additive, Third, Second) => Second,
            (Additive, Third, Third) => Third,
            (Alternative, First, First) => First,
            (Alternative, First, Second) => Second,
            (Alternative, First, Third) => Third,
            (Alternative, Second, First) => First,
            (Alternative, Second, Second) => Second,
            (Alternative, Second, Third) => Third,
            (Alternative, Third, First) => First,
            (Alternative, Third, Second) => Second,
            (Alternative, Third, Third) => Third,
        }

        table common_case(CaseUse, CaseUse) -> CaseUse {
            (Common, Common) => Common,
            (Common, Nominative) => Nominative,
            (Nominative, Common) => Nominative,
            (Nominative, Nominative) => Nominative,
            (Common, Accusative) => Accusative,
            (Accusative, Common) => Accusative,
            (Accusative, Accusative) => Accusative,
            (Genitive, Genitive) => Genitive,
            (Common, Genitive) => Genitive, (Genitive, Common) => Genitive,
            (Nominative, Genitive) => Nominative, (Genitive, Nominative) => Nominative,
            (Accusative, Genitive) => Accusative, (Genitive, Accusative) => Accusative,
        }

        table nominative_case(CaseUse) -> CaseUse {
            (Common) => Nominative,
            (Nominative) => Nominative,
            (Genitive) => Nominative,
        }

        table accusative_case(CaseUse) -> CaseUse {
            (Common) => Accusative,
            (Accusative) => Accusative,
            (Genitive) => Accusative,
        }

        // Local complement selection, not the voice of an auxiliary chain.
        table internalised_complement_presence(InternalisedComplementPresent,
            InternalisedComplementMarker) -> InternalisedComplementPresent {
            (No, No) => No,
            (_, _) => Yes,
        }
        table coordinated_participial_use(ParticipialUse, ParticipialUse) -> ParticipialUse {
            (Ordinary, Ordinary) => Ordinary,
            (BarePassive, BarePassive) => BarePassive,
            (Ordinary, BarePassive) => Mixed,
            (BarePassive, Ordinary) => Mixed,
            (Mixed, Ordinary) => Mixed,
            (Mixed, BarePassive) => Mixed,
            (Ordinary, Mixed) => Mixed,
            (BarePassive, Mixed) => Mixed,
            (Mixed, Mixed) => Mixed,
        }

        table determiner_license(DeterminerUse, number, countability,
            DeterminerRequirement, NominalAdjunctDependent) -> Selection {
            // FH7 (orchestrator, 2026-10-10): noun-side definite selection.
            (Unrestricted, Singular, Count, Definite, Definite) => Yes,
            (Unrestricted, Plural, Count, Definite, Definite) => Yes,
            (Unrestricted, Singular, Mass, Definite, Definite) => Yes,
            (SingularCount, Singular, Count, No, _) => Yes,
            (SingularCount, Singular, Count, Yes, _) => Yes,
            (Unrestricted, Singular, Count, No, _) => Yes,
            (Unrestricted, Singular, Count, Yes, _) => Yes,
            (Unrestricted, Plural, Count, No, _) => Yes,
            (Unrestricted, Plural, Count, Yes, _) => Yes,
            (Unrestricted, Singular, Mass, No, _) => Yes,
            (Unrestricted, Singular, Mass, Yes, _) => Yes,
            (PluralOrMass, Plural, Count, No, _) => Yes,
            (PluralOrMass, Plural, Count, Yes, _) => Yes,
            (PluralOrMass, Singular, Mass, No, _) => Yes,
            (PluralOrMass, Singular, Mass, Yes, _) => Yes,
            (PluralCount, Plural, Count, No, _) => Yes,
            (PluralCount, Plural, Count, Yes, _) => Yes,
            (Mass, Singular, Mass, No, _) => Yes,
            (Mass, Singular, Mass, Yes, _) => Yes,
            (Singular, Singular, Count, No, _) => Yes,
            (Singular, Singular, Count, Yes, _) => Yes,
            (Singular, Singular, Mass, No, _) => Yes,
            (Singular, Singular, Mass, Yes, _) => Yes,
        }

        // CGEL Ch. 5 §§3.3, 18.2: the oblique controls concord only in the
        // declared quantificational use; the syntactic head keeps its Number.
        table quantificational_number(QuantificationalDeterminer, NumberTransparency,
            ObliqueNumber, number, NominalAgreementNumber, DeterminerAgreementNumber) -> number {
            (_, No, None, Singular, SingularOrPlural, _) => Singular,
            (No, No, None, Singular, Head, Nominal) => Singular,
            (No, No, None, Plural, Head, Nominal) => Plural,
            (No, No, Singular, Singular, Head, Nominal) => Singular,
            (No, No, Singular, Plural, Head, Nominal) => Plural,
            (No, No, Plural, Singular, Head, Nominal) => Singular,
            (No, No, Plural, Plural, Head, Nominal) => Plural,
            (No, Yes, None, Singular, Head, Nominal) => Singular,
            (No, Yes, None, Plural, Head, Nominal) => Plural,
            (No, Yes, Singular, Singular, Head, Nominal) => Singular,
            (No, Yes, Singular, Plural, Head, Nominal) => Plural,
            (No, Yes, Plural, Singular, Head, Nominal) => Singular,
            (No, Yes, Plural, Plural, Head, Nominal) => Plural,
            (Yes, No, None, Singular, Head, Nominal) => Singular,
            (Yes, No, None, Plural, Head, Nominal) => Plural,
            (Yes, No, Singular, Singular, Head, Nominal) => Singular,
            (Yes, No, Singular, Plural, Head, Nominal) => Plural,
            (Yes, No, Plural, Singular, Head, Nominal) => Singular,
            (Yes, No, Plural, Plural, Head, Nominal) => Plural,
            (Yes, Yes, None, Singular, Head, Nominal) => Singular,
            (Yes, Yes, None, Plural, Head, Nominal) => Plural,
            (Yes, Yes, Plural, Singular, Head, Nominal) => Plural,
            (Yes, Yes, Plural, Plural, Head, Nominal) => Plural,
            (No, No, None, Singular, Plural, Nominal) => Plural, (No, No, None, Plural, Plural, Nominal) => Plural,
            (No, No, Singular, Singular, Plural, Nominal) => Plural, (No, No, Singular, Plural, Plural, Nominal) => Plural,
            (No, No, Plural, Singular, Plural, Nominal) => Plural, (No, No, Plural, Plural, Plural, Nominal) => Plural,
            (No, Yes, None, Singular, Plural, Nominal) => Plural, (No, Yes, None, Plural, Plural, Nominal) => Plural,
            (No, Yes, Singular, Singular, Plural, Nominal) => Plural, (No, Yes, Singular, Plural, Plural, Nominal) => Plural,
            (No, Yes, Plural, Singular, Plural, Nominal) => Plural, (No, Yes, Plural, Plural, Plural, Nominal) => Plural,
            (Yes, No, None, Singular, Plural, Nominal) => Plural, (Yes, No, None, Plural, Plural, Nominal) => Plural,
            (Yes, No, Singular, Singular, Plural, Nominal) => Plural, (Yes, No, Singular, Plural, Plural, Nominal) => Plural,
            (Yes, No, Plural, Singular, Plural, Nominal) => Plural, (Yes, No, Plural, Plural, Plural, Nominal) => Plural,
            (Yes, Yes, None, Singular, Plural, Nominal) => Plural, (Yes, Yes, None, Plural, Plural, Nominal) => Plural,
            (Yes, Yes, Singular, Singular, Plural, Nominal) => Plural, (Yes, Yes, Singular, Plural, Plural, Nominal) => Plural,
            (Yes, Yes, Plural, Singular, Plural, Nominal) => Plural, (Yes, Yes, Plural, Plural, Plural, Nominal) => Plural,
            // Owner ruling (2026-10-08): singular-selecting Determiners fix
            // Agreement across Nominal coordination. CGEL p. 508 [28] supports
            // singular agreement for coordinated NPs containing each/every.
            (No, No, None, Singular, Head, Singular) => Singular, (No, No, None, Singular, Plural, Singular) => Singular,
            (No, No, Singular, Singular, Head, Singular) => Singular, (No, No, Singular, Singular, Plural, Singular) => Singular,
            (No, No, Plural, Singular, Head, Singular) => Singular, (No, No, Plural, Singular, Plural, Singular) => Singular,
            (No, Yes, None, Singular, Head, Singular) => Singular, (No, Yes, None, Singular, Plural, Singular) => Singular,
            (No, Yes, Singular, Singular, Head, Singular) => Singular, (No, Yes, Singular, Singular, Plural, Singular) => Singular,
            (No, Yes, Plural, Singular, Head, Singular) => Singular, (No, Yes, Plural, Singular, Plural, Singular) => Singular,
        }

        table quantificational_scalar_denotation(QuantificationalDeterminer,
            NumberTransparency, ScalarDenotation) -> ScalarDenotation {
            (No, No, No) => No, (No, No, Yes) => Yes,
            (No, Yes, No) => No, (No, Yes, Yes) => Yes,
            (Yes, No, No) => No, (Yes, No, Yes) => Yes,
            (Yes, Yes, No) => No, (Yes, Yes, Yes) => No,
        }

        table oblique_number(ObliqueMarker, number) -> ObliqueNumber {
            (No, Singular) => None, (No, Plural) => None,
            (Yes, Singular) => Singular, (Yes, Plural) => Plural,
        }
        // The first of-Complement supplies the oblique. Later PP modifiers
        // preserve that dependency instead of replacing its concord source.
        table nominal_oblique(ObliqueNumber, ObliqueNumber) -> ObliqueNumber {
            (None, None) => None, (Singular, None) => Singular, (Plural, None) => Plural,
            (None, Singular) => Singular, (Singular, Singular) => Singular,
            (Plural, Singular) => Plural,
            (None, Plural) => Plural, (Singular, Plural) => Singular, (Plural, Plural) => Plural,
        }
        table coordinated_oblique(CoordinationKind, ObliqueNumber, ObliqueNumber)
            -> ObliqueNumber {
            (Additive, None, None) => None,
            (Additive, None, Singular) => None,
            (Additive, None, Plural) => None,
            (Additive, Singular, None) => None,
            (Additive, Singular, Singular) => Plural,
            (Additive, Singular, Plural) => Plural,
            (Additive, Plural, None) => None,
            (Additive, Plural, Singular) => Plural,
            (Additive, Plural, Plural) => Plural,
            (Alternative, None, None) => None,
            (Alternative, None, Singular) => None,
            (Alternative, None, Plural) => None,
            (Alternative, Singular, None) => None,
            (Alternative, Singular, Singular) => Singular,
            (Alternative, Singular, Plural) => Plural,
            (Alternative, Plural, None) => None,
            (Alternative, Plural, Singular) => Singular,
            (Alternative, Plural, Plural) => Plural,
            (Adversative, None, None) => None,
            (Adversative, None, Singular) => None,
            (Adversative, None, Plural) => None,
            (Adversative, Singular, None) => None,
            (Adversative, Singular, Singular) => Singular,
            (Adversative, Singular, Plural) => Plural,
            (Adversative, Plural, None) => None,
            (Adversative, Plural, Singular) => Singular,
            (Adversative, Plural, Plural) => Plural,
        }

        table selected_frame_use(frame) -> FrameUse {
            (Transitive) => Object,
            (Predicative) => Predicative,
            (Locative) => Locative,
            (ManaComplement) => Mana,
            (AmountComplement) => Amount,
            (Measure) => Measure,
            (SlashMeasure) => SlashMeasure,
            (GrantedAbilityComplement) => GrantedAbility,
            (BareAuxiliary) => AuxiliaryBare,
            (ParticipialAuxiliary) => AuxiliaryParticiple,
            (PerfectAuxiliary) => AuxiliaryPerfect,
            (ObjectName) => ObjectName,
            (QuantitativeDeterminerComplement) => QuantitativeDeterminer,
            (InfinitiveSelection) => Infinitive,
        }

        // CGEL Ch. 18 §6.2, pp. 1615–1616: the third-person singular present
        // and preterite clitics admit phrase hosts; other present cells need a
        // pronoun that is itself the Subject.
        table clitic_host(Clitic, form, number, person) -> CliticHost {
            (No, Present, Singular, First) => Free,
            (No, Present, Singular, Second) => Free,
            (No, Present, Singular, Third) => Free,
            (No, Present, Plural, First) => Free,
            (No, Present, Plural, Second) => Free,
            (No, Present, Plural, Third) => Free,
            (No, Preterite, Singular, First) => Free,
            (No, Preterite, Singular, Second) => Free,
            (No, Preterite, Singular, Third) => Free,
            (No, Preterite, Plural, First) => Free,
            (No, Preterite, Plural, Second) => Free,
            (No, Preterite, Plural, Third) => Free,
            (Yes, Present, Singular, First) => PronounSubject,
            (Yes, Present, Singular, Second) => PronounSubject,
            (Yes, Present, Singular, Third) => Subject,
            (Yes, Present, Plural, First) => PronounSubject,
            (Yes, Present, Plural, Second) => PronounSubject,
            (Yes, Present, Plural, Third) => PronounSubject,
            (Yes, Preterite, Singular, First) => Subject,
            (Yes, Preterite, Singular, Second) => Subject,
            (Yes, Preterite, Singular, Third) => Subject,
            (Yes, Preterite, Plural, First) => Subject,
            (Yes, Preterite, Plural, Second) => Subject,
            (Yes, Preterite, Plural, Third) => Subject,
        }
        table clitic_subject(SubjectStructure, CliticHost) -> Selection {
            (Phrase, Free) => Yes, (Pronoun, Free) => Yes,
            (Phrase, Subject) => Yes, (Pronoun, Subject) => Yes,
            (Pronoun, PronounSubject) => Yes,
        }
        table relative_clitic(CliticHost) -> Selection {
            (Free) => Yes, (Subject) => Yes,
        }
        separator clitic_separator(CliticHost) {
            (Free) => " ", (Subject) => "", (PronounSubject) => "",
        }
        separator lexical_clitic_separator(Clitic) {
            (No) => " ", (Yes) => "",
        }
        table clitic_complement(Clitic, AuxiliaryComplementRealization) -> Selection {
            (No, Overt) => Yes, (No, Elided) => Yes, (No, ProForm) => Yes, (Yes, Overt) => Yes,
        }

        policy FiniteHeadAgreement {
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        policy IrrealisHeadAgreement {
            export ScalarDenotation = No;
            require head.form = Irrealis;
            require head.finiteness = Finite;
            require head.Clitic = No;
            export number = head.number;
            export person = head.person;
        }

        policy SecondaryHeadForm {
            export form = secondary_form(head.form);
        }

        policy NominalHeadFeatures {
            export NominalDependentPresent = Yes;
        }

        policy FiniteGapHeadAgreement {
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        policy FiniteAuxiliaryAgreement {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export SourceComplementSelection = complement.SourceComplementSelection;
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        policy SharedAuxiliaryAgreement {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export SourceComplementSelection = complement.SourceComplementSelection;
        }

        policy SecondaryAuxiliaryProperties {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export SourceComplementSelection = complement.SourceComplementSelection;
            export form = secondary_form(head.form);
            export ParticipialSupplementUse = No;
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy SelectedAuxiliaryProperties {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export SourceComplementSelection = complement.SourceComplementSelection;
            export ParticipialUse = Ordinary;
        }

        policy NoFeatures {
        }

        policy NominativeCase {
            export CaseUse = nominative_case(head.CaseUse);
        }

        policy AccusativeCase {
            export CaseUse = accusative_case(head.CaseUse);
        }

        policy SecondaryProjection {
            export form = secondary_form(head.form);
        }

        table internalised_adjunct_admission(InternalisedComplementPresent, InternalisedComplementMarker) -> Selection {
            (No, No) => Yes, (No, Yes) => Yes, (Yes, No) => Yes,
        }
        policy SecondaryPrepositionProjection {
            require internalised_adjunct_admission(head.InternalisedComplementPresent,
                modifier.InternalisedComplementMarker) = Yes;
            export InternalisedComplementPresent = internalised_complement_presence(
                head.InternalisedComplementPresent, modifier.InternalisedComplementMarker);
            export form = secondary_form(head.form);
        }

        policy OrdinarySecondaryHead {
            export form = secondary_form(head.form);
            export ParticipialSupplementUse = No;
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy OrdinarySelectedPredicate {
            export ParticipialUse = shared_participial_use(head.FrameUse);
            export InternalisedComplementPresent = shared_internalised_presence(head.FrameUse);
        }

        table partitive_countability(countability) -> PartitiveCountability {
            (Count) => Count, (Mass) => Mass,
        }
        table coordinated_partitive_countability(PartitiveCountability, PartitiveCountability)
            -> PartitiveCountability {
            (Count, Count) => Count, (Mass, Mass) => Mass,
            (_, _) => Unrestricted,
        }

        policy NounPhraseHeadAgreement {
            export PartitiveCountability = partitive_countability(head.countability);
            export SubjectStructure = Phrase;
            export number = nominal_agreement_number(head.NominalAgreementNumber, head.number);
            export person = Third;
            export CaseUse = Common;
        }

        policy ThirdPersonCommonCase {
            export SubjectStructure = Phrase;
            export person = Third;
            export CaseUse = Common;
        }

        table coordinated_adjective_complement_class(AdjectiveComplementClass,
            AdjectiveComplementClass) -> AdjectiveComplementClass {
            (ColorWord, ColorWord) => ColorWord, (Manner, Manner) => Manner, (_, _) => None,
        }
        policy PremodifierConcord<Right, Source> {
            require left.SyndeticPremodifier = No;
            require serial_premodifier_licence(left.PremodifierCategory,
                Right.PremodifierCategory, Right.SyndeticPremodifier) = Yes;
            export PremodifierCategory = premodifier_category(left.PremodifierCategory,
                Right.PremodifierCategory);
            export SyndeticPremodifier = Right.SyndeticPremodifier;
            export NounPremodifier = coordinated_noun_premodifier(
                left.NounPremodifier, Right.NounPremodifier);
            export AdjectiveOrigin = coordinated_adjective_origin(left.AdjectiveOrigin, Right.AdjectiveOrigin);
            export TargetingPremodifierUse = coordinated_targeting_premodifier(
                left.TargetingPremodifierUse, Right.TargetingPremodifierUse);
            export SelectedPrepositionUse = selected_preposition_complement(
                left.SelectedPrepositionUse, Right.SelectedPrepositionUse);
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
        }

        policy NoConcord<Right, Source> {

        }

        table coordinated_determiner_requirement(DeterminerRequirement, DeterminerRequirement)
            -> DeterminerRequirement {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
            (Definite, _) => Definite, (_, Definite) => Definite,
        }
        table coordinated_measure_position(MeasurePosition, MeasurePosition) -> MeasurePosition {
            (Before, Before) => Before,
            (None, None) => None, (None, Before) => None, (Before, None) => None,
        }
        table coordinated_property_value_position(CoordinationKind, PropertyValuePosition,
            PropertyValuePosition) -> PropertyValuePosition {
            (Additive, First, Second) => Pair,
            (_, _, _) => None,
        }
        table premodified_property_value_position(AdjectiveOrigin, PropertyValuePosition)
            -> PropertyValuePosition {
            (Ordinary, None) => None, (Ordinary, Single) => Single,
            (Ordinary, First) => First, (Ordinary, Second) => Second, (Ordinary, Pair) => Pair,
            (Participial, None) => None, (Participial, Single) => None,
            (Participial, First) => None, (Participial, Second) => None, (Participial, Pair) => None,
        }
        table coordinated_adjective_origin(AdjectiveOrigin, AdjectiveOrigin) -> AdjectiveOrigin {
            (Ordinary, Ordinary) => Ordinary, (Ordinary, Participial) => Participial,
            (Participial, Ordinary) => Participial, (Participial, Participial) => Participial,
        }
        // PE1–PE4 (orchestrator, property-pair-each-equal-to, 2026-10-10):
        // equality may predicate a coordinated property head; quantification
        // agrees with that predicand, independently of the value's scalar kind.
        table property_value_kind(PropertyValuePosition, MeasureKind, PropertyEqualityForm) -> Selection {
            (Single, Scalar, _) => Yes, (First, Scalar, _) => Yes, (Second, Scalar, _) => Yes,
            (Pair, Pair, _) => Yes,
            (Pair, Scalar, Unquantified) => Yes, (Pair, Scalar, Quantified) => Yes,
        }
        table property_equality_form(ComparisonMarker) -> PropertyEqualityForm {
            (Equality) => Unquantified, (Ordering) => None,
        }
        table property_equality_agreement(PropertyEqualityForm, NominalAgreementNumber, number) -> Selection {
            (None, _, _) => Yes, (Unquantified, _, _) => Yes,
            (Quantified, Plural, _) => Yes, (Quantified, Head, Plural) => Yes,
        }
        table unquantified_comparative(PropertyEqualityForm) -> Selection {
            (None) => Yes, (Unquantified) => Yes,
        }
        table coordinated_cardinal_measurement(CardinalMeasurementUse, CardinalMeasurementUse)
            -> CardinalMeasurementUse {
            (Yes, Yes) => Yes, (No, No) => No, (No, Yes) => No, (Yes, No) => No,
            (DeterminerRequired, DeterminerRequired) => DeterminerRequired,
            (DeterminerRequired, Yes) => DeterminerRequired,
            (Yes, DeterminerRequired) => DeterminerRequired,
            (DeterminerRequired, No) => No, (No, DeterminerRequired) => No,
            (DeterminerProjection, DeterminerProjection) => DeterminerProjection,
        }


        table coordinated_determiner_agreement(DeterminerAgreementNumber, DeterminerAgreementNumber)
            -> DeterminerAgreementNumber {
            (Singular, Singular) => Singular, (Singular, Nominal) => Nominal,
            (Nominal, Singular) => Nominal, (Nominal, Nominal) => Nominal,
            (SingularOrPlural, _) => Nominal, (_, SingularOrPlural) => Nominal,
        }
        table determined_np_coordinate_number(CoordinationKind, number, number,
            DeterminerAgreementNumber, DeterminerAgreementNumber) -> number {
            (Additive, Singular, Singular, Nominal, Nominal) => Plural,
            (Additive, Singular, Singular, Singular, Nominal) => Plural,
            (Additive, Singular, Singular, Nominal, Singular) => Plural,
            (Additive, Singular, Singular, Singular, Singular) => Singular,
            (Additive, Singular, Plural, Nominal, Nominal) => Plural,
            (Additive, Singular, Plural, Singular, Nominal) => Plural,
            (Additive, Singular, Plural, Nominal, Singular) => Plural,
            (Additive, Singular, Plural, Singular, Singular) => Singular,
            (Additive, Plural, Singular, Nominal, Nominal) => Plural,
            (Additive, Plural, Singular, Singular, Nominal) => Plural,
            (Additive, Plural, Singular, Nominal, Singular) => Plural,
            (Additive, Plural, Singular, Singular, Singular) => Singular,
            (Additive, Plural, Plural, Nominal, Nominal) => Plural,
            (Additive, Plural, Plural, Singular, Nominal) => Plural,
            (Additive, Plural, Plural, Nominal, Singular) => Plural,
            (Additive, Plural, Plural, Singular, Singular) => Singular,
            (Alternative, Singular, Singular, Nominal, Nominal) => Singular,
            (Alternative, Singular, Singular, Singular, Nominal) => Singular,
            (Alternative, Singular, Singular, Nominal, Singular) => Singular,
            (Alternative, Singular, Singular, Singular, Singular) => Singular,
            (Alternative, Singular, Plural, Nominal, Nominal) => Plural,
            (Alternative, Singular, Plural, Singular, Nominal) => Plural,
            (Alternative, Singular, Plural, Nominal, Singular) => Plural,
            (Alternative, Singular, Plural, Singular, Singular) => Singular,
            (Alternative, Plural, Singular, Nominal, Nominal) => Singular,
            (Alternative, Plural, Singular, Singular, Nominal) => Singular,
            (Alternative, Plural, Singular, Nominal, Singular) => Singular,
            (Alternative, Plural, Singular, Singular, Singular) => Singular,
            (Alternative, Plural, Plural, Nominal, Nominal) => Plural,
            (Alternative, Plural, Plural, Singular, Nominal) => Plural,
            (Alternative, Plural, Plural, Nominal, Singular) => Plural,
            (Alternative, Plural, Plural, Singular, Singular) => Singular,
            (Additive, _, _, _, _) => Plural,
            (Alternative, _, Singular, _, _) => Singular,
            (Alternative, _, Plural, _, _) => Plural,
        }

        // DR6: passive selected heads share an NP, preserving whether either
        // member licenses an Internalised Complement. Ordinary signatures still agree.
        table shared_frame_concord(FrameUse, FrameUse) -> FrameUse {
            (Object, Object) => Object,
            (Predicative, Predicative) => Predicative,
            (Locative, Locative) => Locative,
            (Mana, Mana) => Mana,
            (Amount, Amount) => Amount,
            (Measure, Measure) => Measure,
            (SlashMeasure, SlashMeasure) => SlashMeasure,
            (GrantedAbility, GrantedAbility) => GrantedAbility,
            (AuxiliaryBare, AuxiliaryBare) => AuxiliaryBare,
            (AuxiliaryParticiple, AuxiliaryParticiple) => AuxiliaryParticiple,
            (AuxiliaryPerfect, AuxiliaryPerfect) => AuxiliaryPerfect,
            (ObjectName, ObjectName) => ObjectName,
            (QuantitativeDeterminer, QuantitativeDeterminer) => QuantitativeDeterminer,
            (Infinitive, Infinitive) => Infinitive,
            (PassiveNominal, PassiveNominal) => PassiveNominal,
            (PassiveNominal, PassiveInternalised) => PassiveInternalised,
            (PassiveInternalised, PassiveNominal) => PassiveInternalised,
            (PassiveInternalised, PassiveInternalised) => PassiveInternalised,
        }
        table shared_nominal_frame(FrameUse) -> Selection {
            (Object) => Yes, (PassiveNominal) => Yes, (PassiveInternalised) => Yes,
        }
        table shared_participial_use(FrameUse) -> ParticipialUse {
            (PassiveNominal) => BarePassive, (PassiveInternalised) => BarePassive,
            (_) => Ordinary,
        }
        table shared_internalised_presence(FrameUse) -> InternalisedComplementPresent {
            (PassiveInternalised) => Yes, (_) => No,
        }

        policy GeneralCoordinatorDistribution<Right, Source> {
            require Source.GeneralCoordination = Yes;
        }

        // left and right are conjuncts; at a serial tail right is the helper summary.
        // coordinator is the overt marker, or its carried summary at a serial tail.
        coordination_forms Standard {
            binary Coordination;
            serial CoordinationSeriesEnd, CoordinationSeriesContinuation, SerialCoordination;
            correlative_helper CorrelativeSeriesEnd, CorrelativeSeriesContinuation;
            both BothCoordination;
            either EitherCoordination, EitherSerialCoordination;
            neither NeitherCoordination, NeitherSerialCoordination;
            on ordinary_overt { require coordinator.GeneralCoordination = Yes; }
        }

        coordinable Nominal using Standard {
            forms binary, serial;
            // R15d/R15e: an asyndetic shared-head group cannot resegment
            // an unmarked member of a coordinator-ending Nominal list.
            on serial { require left.UnmarkedConjunctLicence = Yes; }
        }

        coordinable NounPhrase using Standard {
            forms binary, serial, both, either, neither;
        }

        coordinable AdjectivePhrase using Standard {
            forms binary, serial;
        }

        coordinable CorrelativeAdjectivePhrase of AdjectivePhrase using Standard {
            forms both, either, neither;
            correlative_helper CorrelativeAdjectiveSeries;
        }

        coordinable AdverbPhrase using Standard {
            forms binary, serial, both, either, neither;
            require ordinary_adjunct_coordination(left.FocusedUse, right.FocusedUse) = Yes;
            on binary, serial { combine UnmarkedConjunctLicence = left.UnmarkedConjunctLicence; }
            // Overt ordinary joins copy the left conjunct's unmarked licence.
            // Correlative results are licensed by their markers; their helpers need no licence.
            on correlative_helper { omit UnmarkedConjunctLicence; }
        }

        coordinable PrepositionPhrase using Standard {
            forms binary, serial, both, either, neither;
            require ordinary_adjunct_coordination(left.FocusedUse, right.FocusedUse) = Yes;
        }

        coordinable SelectedPrepositionHead using Standard {
            forms binary, serial;
            on helper { omit HeadCoordination; }
        }

        // These forms do not require the coordinator to license GeneralCoordination.
        coordination_forms WithoutGeneralCoordinatorRequirement {
            binary Coordination;
            serial CoordinationSeriesEnd, CoordinationSeriesContinuation, SerialCoordination;
            correlative_helper CorrelativeSeriesEnd, CorrelativeSeriesContinuation;
            both BothCoordination;
            either EitherCoordination, EitherSerialCoordination;
            neither NeitherCoordination, NeitherSerialCoordination;
        }

        coordinable FinitePredicate using WithoutGeneralCoordinatorRequirement {
            forms binary, serial, both, either, neither;
            require right.CliticHost = Free;
            on helper { omit NominalAdjunctHost; omit ClauseComplementBoundary; }
        }

        coordinable SecondaryVerbPhrase using WithoutGeneralCoordinatorRequirement {
            forms binary, serial, both, either, neither;
            helper SecondaryPredicateSeries;
            on helper { omit NominalAdjunctHost; omit ClauseComplementBoundary; }
        }

        coordinable FiniteSelectedHead using WithoutGeneralCoordinatorRequirement {
            forms binary, serial, both, either, neither;
            require right.CliticHost = Free;
            on helper { omit HeadCoordination; }
        }

        coordinable SecondarySelectedHead using WithoutGeneralCoordinatorRequirement {
            forms binary, serial, both, either, neither;
            on helper { omit HeadCoordination; }
        }

        coordinable FiniteObjectGap using WithoutGeneralCoordinatorRequirement {
            forms binary, serial, both, either, neither;
            require right.CliticHost = Free;
            on helper { omit NominalAdjunctHost; omit ClauseComplementBoundary; }
        }

        coordinable BareObjectGap using WithoutGeneralCoordinatorRequirement {
            forms binary, serial, both, either, neither;
            on helper { omit NominalAdjunctHost; omit ClauseComplementBoundary; }
        }

        coordinable PastParticipleObjectGap using WithoutGeneralCoordinatorRequirement {
            forms binary, serial;
            on helper { omit NominalAdjunctHost; omit ClauseComplementBoundary; }
        }

        coordinable Clause using WithoutGeneralCoordinatorRequirement {
            forms serial, either;
            on serial, correlative_helper {
                combine UnmarkedConjunctLicence = left.UnmarkedConjunctLicence;
            }
            // A following series must be licensed as an unmarked conjunct.
            on tail {
                require right.UnmarkedConjunctLicence = Yes;
            }
        }

        coordinable CoordinatedFiniteClause of FiniteClause using WithoutGeneralCoordinatorRequirement {
            forms binary, serial, either;
            helper FiniteClauseSeries;
            correlative_helper CorrelativeFiniteClauseSeries;
        }

        coordinable Cardinal using Standard {
            forms binary, serial, both, either, neither;
        }

        coordinable MeasurePhrase using Standard {
            forms binary, serial, both, either, neither;
        }

        coordinable GrantedAbility using Standard {
            forms binary, serial;
            on result {
                require quoted_ability_combination(left.GrantedAbilityKind, right.GrantedAbilityKind) = Yes;
            }
        }

        coordinable DepictivePhrase using Standard {
            forms binary, serial;
            on helper { combine DepictiveKind = combined_depictive_kind; }
        }

        coordinable PredicativeComplement using Standard {
            forms binary, serial;
            on helper { combine PredicativeKind = combined_predicative_kind; }
        }

        coordinable ComparativeGovernorHead using Standard {
            forms binary;
        }

        coordinable KeywordQualityPreposition using Standard {
            forms binary, serial;
        }

        coordinable NounPremodifier using Standard {
            forms binary, serial;
        }

        coordinable VerbalPremodifier using Standard {
            forms binary, serial;
        }

        coordinable ManaPhrase using Standard {
            forms binary, serial, both, either, neither;
        }

        coordinable Amount using Standard {
            forms binary, serial, both, either, neither;
        }

        coordinable KeywordPhrase using Standard {
            forms binary, serial, both, either, neither;
        }

        coordinable InfinitiveComplement using Standard {
            forms binary, serial, both, either, neither;
        }

        coordinable FrequencyPhrase using Standard {
            forms binary, serial;
        }

        coordinable QuotedText using Standard {
            forms binary, serial, both, either, neither;
            on helper { omit QuotationStructure; }
        }

        schema CoordinationSeriesEnd {
            head none;
            form [left: node, ", ", coordinator: lexical(Coordinator), " ", right: node];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        schema CoordinationSeriesContinuation {
            head none;
            form [left: node, ", ", rest: node];
        }

        schema SerialCoordination {
            head none;
            form [left: node, ", ", rest: node];
        }

        separator coordination_separator(CommaCoordination) {
            (No) => " ", (Yes) => ", ",
        }
        schema Coordination {
            head none;
            form [left: node, coordination_separator(coordinator.CommaCoordination),
                coordinator: lexical(Coordinator), " ", right: node];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        schema CasePhrase {
            form [head: node];
        }

        // User ruling 2026-10-06, resolved by the orchestrator 2026-10-07:
        // omitted auxiliary Complements require a declared lexical licence.
        // Inflection and clitic constraints remain independently enforced.
        table auxiliary_realization(form, AuxiliaryComplementRealization) -> Selection {
            (Plain, Overt) => Yes, (Plain, Elided) => Yes, (Plain, ProForm) => Yes,
            (Present, Overt) => Yes, (Present, Elided) => Yes, (Present, ProForm) => Yes,
            (Preterite, Overt) => Yes, (Preterite, Elided) => Yes,
            (PastParticiple, Overt) => Yes, (PastParticiple, Elided) => Yes,
            (GerundParticiple, Overt) => Yes,
        }
        policy AuxiliarySelectedPredicate {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export SourceComplementSelection = complement.SourceComplementSelection;
            export ParticipialUse = Ordinary;
            require auxiliary_realization(head.form, complement.AuxiliaryComplementRealization)
                = Yes;
        }

        schema BareAuxiliaryPredicate {
            require licensed_auxiliary_pro_form(head.AuxiliaryProFormLicence,
                complement.AuxiliaryComplementRealization) = Yes;
            require licensed_auxiliary_complement(head.AuxiliaryEllipsisLicence,
                complement.AuxiliaryComplementRealization) = Yes;
            // CGEL p. 1614: stranding requires a stressed auxiliary.
            require clitic_complement(head.Clitic, complement.AuxiliaryComplementRealization)
                = Yes;
            form [head: lexical(Verb), complement: node];
            require head.frame = BareAuxiliary;
            require auxiliary_realization(head.form, complement.AuxiliaryComplementRealization)
                = Yes;
        }

        schema ParticipialAuxiliaryPredicate {
            form [head: lexical(Verb), complement: node];
            require head.frame = ParticipialAuxiliary;
        }

        schema PerfectAuxiliaryPredicate {
            form [head: lexical(Verb), complement: node];
            require head.frame = PerfectAuxiliary;
        }

        // The adjunct precedes the overt auxiliary Complement's predicate,
        // rather than modifying a following NP or predicative Complement.
        schema CentralAdverbComplement {
            head predicate;
            form [" ", modifier: lexical(Adverb), " ", predicate: node];
            form [" ", modifier: lexical(Preposition), " ", predicate: node]
                require modifier.PrepositionComplement = Intransitive;
        }
        table central_participial_position(CentralAdjunctLicence, CentralAdjunctPosition) -> Selection {
            (contains PostCopula, PostCopula) => Yes,
            (contains PostAuxiliary, PostAuxiliary) => Yes,
        }
        policy PostAuxiliaryCentralLicence {
            require central_postauxiliary(modifier.CentralAdjunctLicence) = Yes;
        }
        policy ParticipialCentralLicence {
            require central_participial_position(modifier.CentralAdjunctLicence,
                predicate.CentralAdjunctPosition) = Yes;
        }
        instance CentralAdverbComplement<Result, Predicate: predicate, Licence,
            Properties = NoComplementProperties>: [
            (BareComplement, BarePredicate, PostAuxiliaryCentralLicence, OvertComplementRealization),
            (ParticipialComplement, ParticipialPredicate, ParticipialCentralLicence),
            (PerfectComplement, PastParticiplePredicate, PostAuxiliaryCentralLicence),
        ] {
            use Licence;
            use Properties;
        }

        schema NominalAdjunctPredicate {
            require head.NominalAdjunctHost = Yes;
            require postposed_nominal_adjunct(modifier.NominalAdjunctDependent) = Yes;
            form [head: node, " ", modifier: NominalAdjunctPhrase];
        }

        // CGEL Ch. 8 §12.2, pp. 728–729: an ungoverned positive infinitival
        // is a purpose Adjunct, including in a passive clause.
        schema PurposePredicate {
            require head.PurposeAdjunctLicence = Yes;
            form [head: node, " ", modifier: PurposeInfinitival];
        }

        schema FrequencyPredicate {
            form [head: node, " ", modifier: node];
            form [head: node, " ", modifier: ComparativeQuantity]
                require modifier.FrequencyUse = Yes;
        }

        schema PrepositionPredicate {
            require source_adjunct_admission(head.SourceComplementSelection,
                modifier.SourceMarker) = Yes;
            require trailing_adjunct_host(head.ClauseComplementBoundary,
                modifier.ConnectiveAdjunctUse) = Yes;
            export ClauseComplementBoundary = modifier.ClauseComplementBoundary;
            export NominalAdjunctHost = modifier.NominalAdjunctHost;
            form [head: node, " ", modifier: node];
            form [head: node, " ", modifier: FocusedAdjunctCoordination];
            require adjunct_complement_function(modifier.ComplementFunctionRestriction,
                modifier.PredicativeAdjunctUse) = Yes;
            require licence_adjunct(modifier.PrepositionFunctionLicence) = Yes;
            require modifier.MatrixComplementRequirement = None;
            require modifier.SelectedPrepositionUse = No;
        }

        // CGEL Ch. 16 §10.1.1: this function belongs to the embedded Bare Passive.
        // CGEL Ch. 7 §5.1(g), pp. 641–643: the matrix licenses the
        // embedded PP. Here its internalised Complement distribution requires
        // an unfilled Bare Passive host, independently of lexical identity.
        // Project analysis requested by the except-matrix-complements ticket:
        // nominal and copy-description exceptions use declared host licences.
        // CGEL's matrix account alone does not establish these Oracle uses.
        schema LicensedMatrixComplementPredicate {
            // Owner ruling, 2026-10-08 (licence-hygiene ticket): both
            // separators occur with each licensed exception Complement shape.
            form [head: node, " ", modifier: PrepositionPhrase];
            form [head: node, ", ", modifier: PrepositionPhrase];
            require licensed_matrix_complement(head.MatrixComplementLicence,
                modifier.MatrixComplementRequirement) = Yes;
        }
        instance LicensedMatrixComplementPredicate<Result, Head: head>: [
            (FinitePredicate, Self),
        ] {}
        instance LicensedMatrixComplementPredicate<Result, Head: head, Properties>: [
            (SecondaryVerbPhrase, Self, SecondaryProjection),
        ] {
            use Properties;
        }

        construction MatrixComplementPredicate: SecondaryVerbPhrase {
            form [head: SecondaryVerbPhrase, " ", modifier: PrepositionPhrase];
            require modifier.MatrixComplementRequirement = Internalised;
            require head.ParticipialUse = BarePassive;
            require head.InternalisedComplementPresent = No;
            export InternalisedComplementPresent = Yes;
        }

        construction InternalisedComplementPredicate: SecondaryVerbPhrase {
            form [head: SecondaryVerbPhrase, " ", modifier: PrepositionPhrase];
            require head.ParticipialUse = BarePassive;
            require head.InternalisedComplementPresent = No;
            require modifier.InternalisedComplementMarker = Yes;
            export InternalisedComplementPresent = Yes;
        }

        policy NoComplementProperties {
        }

        policy OvertComplementRealization {
            export AuxiliaryComplementRealization = Overt;
        }

        schema OvertComplement {
            head predicate;
            form [" ", predicate: node];
        }

        // Complete lexical hosts keep auxiliary ellipsis outside depictive attachment.
        schema DepictivePredicate {
            form [head: lexical(Verb), " ", modifier: DepictivePhrase];
            require head.frame = Intransitive;
        }

        schema ComplementedDepictivePredicate {
            form [head: node, " ", modifier: DepictivePhrase];
        }

        schema AdverbPredicate {
            form [head: node, " ", modifier: node];
            require modifier.VPFinalAdjunct = Yes;
        }

        schema SelectedVerbHead {
            // An uncomplemented coordinate head requires the strong form.
            require head.Clitic = No;
            form [head: lexical(Verb)];
            export FrameUse = selected_frame_use(head.frame);
            export HeadCoordination = No;
        }

        schema SharedObjectComplement {
            form [head: node, " ", object: node];
            require head.HeadCoordination = Yes;
            require shared_nominal_frame(head.FrameUse) = Yes;
        }

        schema SharedPredicativeComplement {
            form [head: node, " ", complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Predicative;
        }

        schema SharedLocativeComplement {
            form [head: node, " ", complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Locative;
        }

        schema SharedManaComplement {
            form [head: node, " ", complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Mana;
        }

        schema SharedAmountComplement {
            form [head: node, " ", amount: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Amount;
        }

        schema SharedMeasureComplement {
            form [head: node, " ", measure: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Measure;
            require measure.MeasureKind = Scalar;
        }

        schema SharedSlashMeasureComplement {
            form [head: node, " ", measure: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = SlashMeasure;
        }

        schema SharedGrantedAbilityComplement {
            form [head: node, " ", complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = GrantedAbility;
        }

        schema SharedAuxiliaryBareComplement {
            require licensed_auxiliary_pro_form(head.AuxiliaryProFormLicence,
                complement.AuxiliaryComplementRealization) = Yes;
            require licensed_auxiliary_complement(head.AuxiliaryEllipsisLicence,
                complement.AuxiliaryComplementRealization) = Yes;
            form [head: node, complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = AuxiliaryBare;
        }

        schema SharedAuxiliaryParticipleComplement {
            form [head: node, complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = AuxiliaryParticiple;
        }

        schema SharedAuxiliaryPerfectComplement {
            form [head: node, complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = AuxiliaryPerfect;
        }

        schema SharedObjectNameComplement {
            form [head: node, " ", object: node, " ", complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = ObjectName;
        }

        schema EitherCoordination {
            head none;
            form [marker: lexical(Determinative), " ", left: node, " ",
                coordinator: lexical(Coordinator), " ", right: node];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
        }

        schema CorrelativeSeriesEnd {
            head none;
            form [left: node, ", ", coordinator: lexical(Coordinator), " ", right: node];
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
        }

        schema CorrelativeSeriesContinuation {
            head none;
            form [left: node, ", ", rest: node];
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
        }

        schema EitherSerialCoordination {
            head none;
            form [marker: lexical(Determinative), " ", left: node, ", ", rest: node];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
        }

        schema BothCoordination {
            head none;
            form [marker: lexical(Determinative), " ", left: node, " ",
                coordinator: lexical(Coordinator), " ", right: node];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
        }

        schema NeitherCoordination {
            head none;
            form [marker: lexical(Determinative), " ", left: node, " ",
                coordinator: lexical(Coordinator), " ", right: node];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
        }

        schema NeitherSerialCoordination {
            head none;
            form [marker: lexical(Determinative), " ", left: node, ", ", rest: node];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
        }

        schema SharedQuantitativeDeterminerComplement {
            form [head: node, " ", complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = QuantitativeDeterminer;
        }

        schema SharedInfinitiveComplement {
            form [head: node, " ", complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Infinitive;
        }

        construction EmptyDocument: Document {
            form [];
        }

        construction TypeLine: TypeLine {
            form [supertypes: repeat(SupertypePrefix, ""), types: CardTypes];
        }

        construction SubtypedLine: TypeLine {
            form [supertypes: repeat(SupertypePrefix, ""), types: CardTypes, " — ",
                subtypes: Subtypes];
        }

        construction SupertypePrefix: SupertypePrefix {
            form [head: lexical(Catalog), " "];
            require head.TypeLineRole = Supertype;
        }

        construction CardTypes: CardTypes {
            form [head: lexical(Catalog), rest: repeat(CardTypeContinuation, "")];
            require head.TypeLineRole = CardType;
        }

        construction CardTypeContinuation: CardTypeContinuation {
            form [" ", head: lexical(Catalog)];
            require head.TypeLineRole = CardType;
        }

        construction Subtypes: Subtypes {
            form [head: lexical(Noun), rest: repeat(SubtypeContinuation, "")];
            require head.TypeLineRole = Subtype;
            require head.form = Singular;
        }

        construction SubtypeContinuation: SubtypeContinuation {
            form [" ", head: lexical(Noun)];
            require head.TypeLineRole = Subtype;
            require head.form = Singular;
        }

        construction Document: Document {
            form [first: Ability, rest: repeat(AbilityContinuation, "")];
            require first.AbilityTermination = Complete;
        }

        construction AbilityContinuation: AbilityContinuation {
            form ["\n", ability: Ability];
            require ability.AbilityTermination = Complete;
        }

        construction OrdinaryAbility: Ability {
            head body;
            export LabelKind = None;
            export SentenceLineList = No;
            form [body: Paragraph];
            form [body: Sentence] require body.AbilityTermination = Unterminated
                export AbilityTermination = Unterminated;
        }

        policy CompleteSentenceFirst {
            require first.AbilityTermination = Complete;
        }
        schema Paragraph {
            form [first: node, rest: repeat(node, "")];
        }
        instance Paragraph<Result, First: first, Rest: rest, Properties = NoFeatures>: [
            (Paragraph, ParagraphItem, ParagraphContinuation),
            (SentenceParagraph, Sentence, SentenceContinuation, CompleteSentenceFirst),
        ] { use Properties; }

        policy CompleteSentenceMember {
            require item.AbilityTermination = Complete;
        }
        schema ParagraphContinuation {
            form [" ", item: node];
        }
        instance ParagraphContinuation<Result, Item: item, Properties = NoFeatures>: [
            (ParagraphContinuation, ParagraphItem),
            (SentenceContinuation, Sentence, CompleteSentenceMember),
        ] { use Properties; }

        construction SentenceItem: ParagraphItem {
            form [sentence: Sentence];
            require sentence.AbilityTermination = Complete;
        }

        construction ParentheticalItem: ParagraphItem {
            form [parenthetical: Parenthetical];
        }

        construction Parenthetical: Parenthetical {
            form ["(", body: Paragraph, ")"];
        }

        construction QuotedText: QuotedText {
            head text;
            export QuotationStructure = Simple;
            form ["\"", text: Document, "\""];
        }

        construction QuotedClause: QuotedText {
            head clause;
            export QuotationStructure = Simple;
            boundary Interior;
            form ["\"", clause: Clause, "\""];
            form ["\"", clause: Ability, "\""]
                require clause.AbilityTermination = Unterminated
                require clause.terminal_punctuation = None;
        }

        construction QuotedKeyword: QuotedText {
            head keyword;
            export QuotationStructure = Simple;
            boundary Interior;
            form ["\"", keyword: KeywordPhrase, "\""];
            form ["\"", keyword: KeywordPhrase, ".\""];
        }

        construction KeywordGrantedAbility: GrantedAbility {
            head keyword;
            form [keyword: KeywordPhrase];
            export GrantedAbilityKind = Keyword;
        }

        construction QuotedGrantedAbility: GrantedAbility {
            head quotation;
            form [quotation: QuotedText];
            require quotation.QuotationStructure = Simple;
            export GrantedAbilityKind = Quoted;
        }

        table combined_ability_kind(GrantedAbilityKind, GrantedAbilityKind) -> GrantedAbilityKind {
            (Keyword, Keyword) => Keyword,
            (Keyword, Quoted) => Quoted, (Quoted, Keyword) => Quoted, (Quoted, Quoted) => Quoted,
        }
        table quoted_ability_combination(GrantedAbilityKind, GrantedAbilityKind) -> Selection {
            (Keyword, Quoted) => Yes, (Quoted, Keyword) => Yes, (Quoted, Quoted) => Yes,
        }
        construction ActivatedAbility: Ability {
            head body;
            export LabelKind = None;
            export SentenceLineList = No;
            form [cost: Cost, ": ", body: Paragraph];
            form [cost: Cost, ": ", body: Sentence]
                require body.AbilityTermination = Unterminated
                export AbilityTermination = Unterminated;
        }

        construction AbilityWordHead: Ability {
            boundary Initial;
            form [head: lexical(Keyword), " — ", body: Ability];
            require head.LabelKind = AbilityWord;
            require body.LabelKind = None;
            require body.SentenceLineList = No;
            export LabelKind = AbilityWord;
            export SentenceLineList = No;
            export AbilityTermination = body.AbilityTermination;
        }

        construction FlavorWordHead: Ability {
            cost 100;
            boundary Initial;
            form [head: lexical(FlavorWord), " ", body: Ability];
            require body.LabelKind = None;
            require body.SentenceLineList = No;
            export LabelKind = FlavorWord;
            export SentenceLineList = No;
            export AbilityTermination = body.AbilityTermination;
        }

        construction Cost: Cost {
            head none;
            form [first: CostComponent, rest: repeat(CostContinuation, "")];
            export CostComponentForm = final_cost_component(first.CostComponentForm,
                last(rest).CostComponentForm);
        }

        construction CostContinuation: CostContinuation {
            head component;
            form [", ", component: CostComponent];
        }

        construction ActionCost: CostComponent {
            head action;
            boundary Initial;
            form [action: BarePredicate];
            export CostComponentForm = Action;
        }

        construction SymbolCost: CostComponent {
            head symbols;
            form [symbols: CostSymbols];
            export CostComponentForm = Symbols;
        }

        // Oracle document notation, under the signed-numeral-cost project analysis.
        construction SignedScalarCost: Cost {
            head value;
            form [value: ScalarComponent];
            require loyalty_sign(value.ScalarSign) = Yes;
            export CostComponentForm = Symbols;
        }

        construction ZeroScalarCost: Cost {
            head none;
            form ["0"];
            export CostComponentForm = Symbols;
        }

        schema SymbolSequence {
            form [first: node, rest: repeat(node, "")];
        }
        instance SymbolSequence<Result, First: first, Rest: rest>: [
            (CostSymbols, CostSymbol, CostSymbol),
            (ManaPhrase, ManaSymbol, ManaSymbol),
        ] {}

        construction NamedManaSymbol: ManaSymbol {
            form ["{", symbol: lexical(Symbol), "}"];
            require symbol.ManaSymbolUse = Yes;
        }

        construction NamedCostSymbol: CostSymbol {
            form ["{", symbol: lexical(Symbol), "}"];
            require symbol.SymbolUse = Cost;
        }

        construction NumericCostSymbol: CostSymbol {
            form ["{", number: lexical(Numeral), "}"];
            require number.numeral_kind = Arabic;
        }

        construction KeywordLine: Ability {
            head none;
            export LabelKind = None;
            export SentenceLineList = No;
            export AbilityTermination = Complete;
            boundary Initial;
            form [first: KeywordPhrase, rest: repeat(KeywordContinuation, "")];
        }

        construction KeywordContinuation: KeywordContinuation {
            form [", ", keyword: KeywordPhrase];
            form ["; ", keyword: KeywordPhrase];
        }

        construction BareKeyword: KeywordPhrase {
            form [head: lexical(Keyword)];
            require bare_keyword_use(head.KeywordParameterClass) = Yes;
        }

        construction AmountKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", amount: MeasurePhrase];
            require head.KeywordParameterClass = Amount;
        }

        construction CostKeyword: KeywordPhrase {
            require head.KeywordParameterClass = Cost;
            form [head: lexical(Keyword), " ", cost: CostSymbols];
            // Project rulings KD1–KD3: the dash selects a Cost parameter;
            // its final component determines the selected terminal literal.
            form [head: lexical(Keyword), "—", cost: Cost,
                keyword_cost_terminal(cost.CostComponentForm, cost.terminal_punctuation)];
        }

        table quality_number(number) -> KeywordQualityNumber {
            (Singular) => Singular, (Plural) => Plural,
        }
        table compatible_quality(KeywordQualityNumber, KeywordQualityNumber) -> Selection {
            (Any, Any) => Yes, (Any, Singular) => Yes, (Any, Plural) => Yes,
            (Singular, Singular) => Yes, (Plural, Plural) => Yes,
        }
        table coordinated_quality(KeywordQualityNumber, KeywordQualityNumber)
            -> KeywordQualityNumber {
            (Any, Any) => Any, (Any, Singular) => Any, (Any, Plural) => Any,
            (Singular, Any) => Any, (Singular, Singular) => Singular,
            (Singular, Plural) => Any, (Plural, Any) => Any,
            (Plural, Singular) => Any, (Plural, Plural) => Plural,
        }
        separator keyword_separator(KeywordSeparator) {
            (Space) => " ", (Dash) => "—", (SpacedDash) => " — ",
        }
        separator keyword_parameter_separator(KeywordParameterSeparator) {
            (Space) => " ", (Dash) => "—", (SpacedDash) => " — ",
        }
        policy SelectedTailConcord<Right, Source> {
            require left.HeadCoordination = No;
            share_segments left, Right;
            export HeadCoordination = Yes;
        }
        policy PrimitiveRightTail<Right, Source> {
            require Right.HeadCoordination = No;
        }
        construction AdjectivalKeywordQuality: KeywordQuality {
            head phrase;
            form [phrase: AdjectivePhrase];
            require phrase.AdjectiveAttributiveOnlyUse = No;
            require phrase.AdjectiveStructure = Simple;
            export KeywordQualityNumber = Any;
        }
        construction NominalKeywordQuality: KeywordQuality {
            head phrase;
            form [phrase: Nominal];
            require phrase.Targeting = No;
            export KeywordQualityNumber = quality_number(phrase.number);
        }
        construction NounPhraseKeywordQuality: KeywordQuality {
            head phrase;
            require phrase.BarePrepositionUse = No;
            form [phrase: NounPhrase];
            require phrase.Targeting = No;
            require accusative_case(phrase.CaseUse) = Accusative;
            export KeywordQualityNumber = quality_number(phrase.number);
        }
        table keyword_quality_marker(KeywordMarker) -> Selection {
            (For) => Yes, (From) => Yes, (Onto) => Yes,
        }
        construction KeywordQualityPreposition: KeywordQualityPreposition {
            form [head: lexical(Preposition), " ", quality: KeywordQuality];
            require keyword_quality_marker(head.KeywordMarker) = Yes;
            export KeywordMarker = head.KeywordMarker;
            export KeywordQualityNumber = quality.KeywordQualityNumber;
        }
        construction QualityKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", quality: KeywordQualityPreposition];
            require head.KeywordParameterClass = Quality;
            require head.KeywordPayloadOrder = AfterHead;
            require head.KeywordSeparator = Space;
            agree head.KeywordMarker = quality.KeywordMarker;
            require compatible_quality(head.KeywordQualityNumber,
                quality.KeywordQualityNumber) = Yes;
        }
        // Name parameter and selected marker: orchestrator ruling PW2 (2026-10-11).
        construction NameKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", marker: lexical(Preposition), " ", name: Name];
            require head.KeywordParameterClass = Name;
            require head.KeywordPayloadOrder = AfterHead;
            require head.KeywordSeparator = Space;
            agree head.KeywordMarker = marker.KeywordMarker;
        }
        construction BeforeHeadQualityKeyword: KeywordPhrase {
            form [quality: KeywordQuality, " ", head: lexical(Keyword)];
            require head.KeywordParameterClass = Quality;
            require head.KeywordPayloadOrder = BeforeHead;
            require head.KeywordMarker = None;
            require compatible_quality(head.KeywordQualityNumber,
                quality.KeywordQualityNumber) = Yes;
        }

        // Oracle keyword formation: ticket analysis and owner ruling, 2026-10-08.
        table licensed_creature_stem(CreatureSubtypeStem, CreatureSubtypeStemUse) -> Selection {
            (No, No) => Yes, (No, Yes) => Yes, (Yes, Yes) => Yes,
        }
        construction BoundQualityKeyword: KeywordPhrase {
            form [modifier: optional(lexical(Adjective), " "), quality: lexical(Noun), head: lexical(Keyword)];
            require modifier.TypeLineRole = Supertype;
            require quality.frame = BoundKeywordStem;
            require head.KeywordParameterClass = Quality;
            require head.KeywordPayloadOrder = BoundSuffix;
            require head.BoundKeyword = Yes;
            require licensed_creature_stem(quality.CreatureSubtypeStem,
                head.CreatureSubtypeStemUse) = Yes;
        }

        // Typecycling's bound Quality and mandatory Cost: orchestrator ruling R8a.
        construction BoundQualityCostKeyword: KeywordPhrase {
            form [modifier: optional(lexical(Adjective), " "), quality: lexical(Noun), head: lexical(Keyword), " ", cost: CostSymbols];
            require modifier.TypeLineRole = Supertype;
            require quality.frame = BoundKeywordStem;
            require head.KeywordParameterClass = QualityCost;
            require head.KeywordPayloadOrder = BoundSuffix;
            require head.BoundKeyword = Yes;
            require licensed_creature_stem(quality.CreatureSubtypeStem,
                head.CreatureSubtypeStemUse) = Yes;
        }

        // The keyword's parameter recipe selects its article (CH1, 2026-10-10).
        feature KeywordIndefiniteArticle { No, Yes } default No;
        separator keyword_subject_article(KeywordIndefiniteArticle, onset) {
            (No, _) => " ", (Yes, Consonant) => " a ", (Yes, Vowel) => " an ",
        }
        table keyword_subject_license(KeywordIndefiniteArticle, number, countability) -> Selection {
            (No, _, _) => Yes, (Yes, Singular, Count) => Yes,
        }
        construction SubjectKeyword: KeywordPhrase {
            form [head: lexical(Keyword),
                keyword_subject_article(head.KeywordIndefiniteArticle, subject.onset), subject: Nominal];
            require head.KeywordParameterClass = Subject;
            require keyword_subject_license(head.KeywordIndefiniteArticle,
                subject.number, subject.countability) = Yes;
            require head.KeywordSeparator = Space;
            require subject.Targeting = No;
        }
        construction AmountCostKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", amount: MeasurePhrase,
                keyword_parameter_separator(head.KeywordParameterSeparator), cost: CostSymbols];
            require head.KeywordParameterClass = AmountCost;
            require head.KeywordSeparator = Space;
            require amount.MeasureKind = Scalar;
        }
        table quality_cost_marker_concord(KeywordMarker, KeywordMarker) -> Selection {
            (For, For) => Yes, (From, From) => Yes,
            (Onto, Onto) => Yes,
        }
        construction QualityCostKeyword: KeywordPhrase {
            require head.KeywordParameterClass = QualityCost;
            require head.KeywordSeparator = Space;
            require compatible_quality(head.KeywordQualityNumber,
                quality.KeywordQualityNumber) = Yes;
            form [head: lexical(Keyword), " ", quality: KeywordQualityPreposition,
                keyword_parameter_separator(head.KeywordParameterSeparator), cost: CostSymbols]
                require quality_cost_marker_concord(head.KeywordMarker, quality.KeywordMarker) = Yes;
            form [head: lexical(Keyword), " ", quality: KeywordQuality,
                keyword_parameter_separator(head.KeywordParameterSeparator), cost: CostSymbols]
                require head.KeywordMarker = None;
        }
        // A bare quality then a number, no preposition: "Devour artifact 1"
        // (ruling 2026-10-10, `semantics-v2-devour-quality-keyword`).
        construction QualityAmountKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", quality: KeywordQuality, " ",
                amount: MeasurePhrase];
            require head.KeywordParameterClass = QualityAmount;
            require head.KeywordSeparator = Space;
            require amount.MeasureKind = Scalar;
        }
        category ClausalKeywordPayload(head KeywordParameterClass);
        construction AbilityKeywordPayload: ClausalKeywordPayload {
            head body;
            form [body: Ability];
            require body.LabelKind = None;
            require body.AbilityTermination = Complete;
            export KeywordParameterClass = Ability;
        }
        construction ConditionKeywordPayload: ClausalKeywordPayload {
            head body;
            form [body: Sentence];
            require body.AbilityTermination = Complete;
            export KeywordParameterClass = Condition;
        }
        construction ClausalKeyword: KeywordPhrase {
            form [head: lexical(Keyword), keyword_separator(head.KeywordSeparator),
                body: ClausalKeywordPayload];
            agree head.KeywordParameterClass = body.KeywordParameterClass;
        }
        construction CostPowerToughnessKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", cost: CostSymbols,
                keyword_parameter_separator(head.KeywordParameterSeparator), size: SlashPair];
            require size.AdditiveModifierUse = No;
            require head.KeywordParameterClass = CostPowerToughness;
            require head.KeywordSeparator = Space;
        }

        construction RemindedKeyword: KeywordPhrase {
            form [keyword: KeywordPhrase, " ", reminder: Parenthetical];
        }

        construction Mode: Mode {
            head body;
            form ["• ", body: Paragraph] export LabelKind = None;
            form ["• ", body: Ability] require body.LabelKind = FlavorWord
                export LabelKind = FlavorWord;
        }

        policy NamedModeMember {
            require mode.LabelKind = FlavorWord;
        }

        schema ModeContinuation {
            form ["\n", mode: Mode];
        }
        instance ModeContinuation<Result, Properties = NoFeatures>: [
            (ModeContinuation), (NamedModeContinuation, NamedModeMember),
        ] { use Properties; }

        policy DashHeaderModes {
            export ModeListLayout = DashHeader;
        }
        policy NamedModeFirst {
            require first.LabelKind = FlavorWord;
            export ModeListLayout = SentenceHeader;
        }

        schema Modes {
            head none;
            form [first: Mode, rest: repeat(node, "")];
        }
        instance Modes<Result, Rest: rest, Properties = NoFeatures>: [
            (Modes, ModeContinuation, DashHeaderModes),
            (NamedModes, NamedModeContinuation, NamedModeFirst),
        ] { use Properties; }

        policy DashHeaderModalItem {
            require modes.ModeListLayout = DashHeader;
        }
        policy SentenceLineListAbility {
            require modes.ModeListLayout = SentenceHeader;
            export LabelKind = None;
            export AbilityTermination = Complete;
            export SentenceLineList = Yes;
        }
        schema ModalItem {
            head none;
            boundary Initial;
            form [header: Clause, " —\n", modes: Modes];
            form [header: SentenceParagraph, "\n", modes: NamedModes];
        }
        instance ModalItem<Result, Properties>: [
            (ParagraphItem, DashHeaderModalItem),
            (Ability, SentenceLineListAbility),
        ] { use Properties; }

        separator sentence_terminal(terminal_punctuation) {
            (None) => ".", (QuotedFullStop) => "",
        }
        construction Sentence: Sentence {
            head clause;
            boundary Initial;
            form [clause: Clause, sentence_terminal(clause.terminal_punctuation)];
            form [clause: Clause] require clause.terminal_punctuation = None
                export AbilityTermination = Unterminated;
        }

        // Project analysis: a variable-defining PP supplements the completed
        // clause or keyword label once. CGEL Ch. 11 §4.8, p. 971 supports
        // content-clause selection, not this variable-defining application.
        schema Supplementation {
            head anchor;
            boundary Initial;
            form [anchor: node, ", ", supplement: PrepositionPhrase, "."];
            require licence_supplement(supplement.PrepositionFunctionLicence) = Yes;
            require anchor.terminal_punctuation = None;
        }
        policy UnlabelledSupplementation {
            export LabelKind = None;
            export SentenceLineList = No;
        }
        instance Supplementation<Result, Anchor: anchor>: [
            (Sentence, Clause),
        ] {}
        instance Supplementation<Result, Anchor: anchor, Properties>: [
            (Ability, KeywordPhrase, UnlabelledSupplementation),
        ] {
            use Properties;
        }

        // HR4: an embedded participial Supplement has a Clause or action
        // Cost anchor. Unlike terminal supplementation, it has no initial
        // capitalization boundary and leaves punctuation to its container.
        schema ParticipialSupplementation {
            head anchor;
            form [anchor: node, ", ", supplement: SecondaryVerbPhrase];
            require supplement.ParticipialUse = BarePassive;
            require supplement.form = PastParticiple;
            require supplement.ParticipialSupplementUse = Yes;
            require anchor.terminal_punctuation = None;
        }
        policy CostSupplement {
            require anchor.CostComponentForm = Action;
            export CostComponentForm = Action;
        }
        instance ParticipialSupplementation<Result, Anchor: anchor>: [
            (FiniteClause, FiniteClause),
            (Clause, ImperativeClause),
        ] {}
        instance ParticipialSupplementation<Result, Anchor: anchor, Properties>: [
            (CostComponent, CostComponent, CostSupplement),
        ] {
            use Properties;
        }

        construction Declarative: Clause {
            head clause;
            form [clause: FiniteClause];
        }

        // HR4: a disjoint Clause anchor keeps the finite and imperative
        // supplementation routes from duplicating a declarative Reading.
        construction SupplementAnchorImperative: ImperativeClause {
            head predicate;
            form [predicate: BarePredicate];
        }

        construction Imperative: Clause {
            head predicate;
            form [predicate: BarePredicate];
        }

        // CGEL Ch. 8 §20.1, p. 781: without a Subject, front and
        // pre-verbal central position are not overtly distinct. The licensed
        // central analysis of Oracle imperative also is the project application.
        construction CentralAdverbImperative: Clause {
            head predicate;
            form [modifier: lexical(Adverb), " ", predicate: BarePredicate];
            require central_imperative(modifier.CentralAdjunctLicence) = Yes;
        }


        table finite_subject_number(number, number, DeterminerAgreementNumber) -> Selection {
            (Singular, Singular, _) => Yes, (Plural, Plural, _) => Yes,
            (_, Singular, SingularOrPlural) => Yes,
            (_, Plural, SingularOrPlural) => Yes,
        }
        table finite_subject_person(person, person) -> Selection {
            (First, First) => Yes, (Second, Second) => Yes, (Third, Third) => Yes,
        }
        construction FiniteClause: FiniteClause {
            head predicate;
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: FinitePredicate]
                require finite_subject_number(subject.number, predicate.number, subject.DeterminerAgreementNumber) = Yes
                require finite_subject_person(subject.person, predicate.person) = Yes
                require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
            form [subject: DummyPronoun, clitic_separator(predicate.CliticHost),
                predicate: ExistentialPredicate];
            form [subject: ManaSubject, clitic_separator(predicate.CliticHost),
                predicate: FiniteManaPassivePredicate]
                require finite_subject_number(subject.number, predicate.number, subject.DeterminerAgreementNumber) = Yes
                require finite_subject_person(subject.person, predicate.person) = Yes
                require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        // CGEL Ch. 5 §9.2, p. 413: implicit-partitive adjuncts quantify
        // the Subject. Clause attachment scopes over the whole Predicate.
        construction QuantifiedClause: FiniteClause {
            head predicate;
            form [subject: NominativePhrase, " ", quantifier: lexical(Determinative),
                " ", predicate: FinitePredicate];
            require quantifier.QuantificationalAdjunctUse = Yes;
            require subject.number = Plural;
            require predicate.CliticHost = Free;
            require finite_subject_number(subject.number, predicate.number, subject.DeterminerAgreementNumber) = Yes;
            agree subject.person = predicate.person;
        }

        // CGEL Ch. 3 §2.1.5, p. 102: the adjunct can follow an auxiliary,
        // never a lexical verb. Copular be is auxiliary (Ch. 2 §3, p. 51).
        construction QuantifiedAuxiliaryHead: QuantifiedAuxiliaryHead {
            form [head: lexical(Verb), " ", quantifier: lexical(Determinative)];
            require quantifier.QuantificationalAdjunctUse = Yes;
            require head.AuxiliaryUse = Yes;
            require head.number = Plural;
            export frame = head.frame;
            use FiniteHeadAgreement;
        }
        construction PostAuxiliaryQuantification: FinitePredicate {
            form [head: QuantifiedAuxiliaryHead, complements: selected_frame(head, Predicate)];
        }
        construction QuantifiedScalarLocation: ScalarLocationPredicate {
            form [head: QuantifiedAuxiliaryHead, complements: selected_frame(head, ScalarLocation)];
        }

        construction QuantifiedBareAuxiliaryPredicate: FinitePredicate {
            require licensed_auxiliary_pro_form(head.AuxiliaryProFormLicence,
                complement.AuxiliaryComplementRealization) = Yes;
            form [head: lexical(Verb), " ", quantifier: lexical(Determinative),
                complement: BareComplement];
            require quantifier.QuantificationalAdjunctUse = Yes;
            require head.frame = BareAuxiliary;
            require head.number = Plural;
            require licensed_auxiliary_complement(head.AuxiliaryEllipsisLicence,
                complement.AuxiliaryComplementRealization) = Yes;
            require clitic_complement(head.Clitic, complement.AuxiliaryComplementRealization) = Yes;
            require auxiliary_realization(head.form, complement.AuxiliaryComplementRealization) = Yes;
            use FiniteAuxiliaryAgreement;
        }

        // IS1 (orchestrator ruling, 2026-10-10): retain the scalar-location
        // Subject restriction when extending the copula's irrealis instance.
        table scalar_subject_licence(ScalarDenotation, ScalarDenotation) -> Selection {
            (_, No) => Yes, (Yes, Yes) => Yes,
        }
        // CGEL Ch. 3 §§1.7–1.8.1, pp. 86–88: finite irrealis is non-tensed
        // and distinct from the ordinary agreeing preterite.
        construction IrrealisClause: IrrealisClause {
            form [subject: NominativePhrase, " ", predicate: IrrealisPredicate];
            require scalar_subject_licence(subject.ScalarDenotation, predicate.ScalarDenotation) = Yes;
            require finite_subject_number(subject.number, predicate.number, subject.DeterminerAgreementNumber) = Yes;
            agree subject.person = predicate.person;
        }

        // CGEL Ch. 16 §6.1, p. 1391: dummy Subject and displaced Subject
        // (an internal Complement, not an Object). Agreement inheritance:
        // Ch. 4 §3.2.2, p. 242. Strict concord also with clitic forms is the
        // existential-there ticket's Oracle-English requirement.
        // Carrier completed under orchestrator ruling T1, 2026-10-10.
        construction ExistentialPredicate: ExistentialPredicate {
            form [head: lexical(Verb), " ", displaced_subject: AccusativePhrase,
                extension: optional(ExistentialExtension)];
            require head.frame = Existential;
            require tensed_form(head.form) = Yes;
            agree displaced_subject.number = head.number;
            agree displaced_subject.person = head.person;
            use FiniteHeadAgreement;
        }

        // CGEL Ch. 16 §6.2.1, pp. 1393–1394: the locative extension is a
        // Complement of the existential verb, outside the displaced NP.
        construction ExistentialExtension: ExistentialExtension {
            form [" ", complement: LocativeComplement];
        }

        // CGEL Ch. 8 §5.4, p. 693: scalar location with be and an NP
        // predicative Complement. The Subject restriction is this ticket's
        // Oracle-English contract, not a restriction asserted by that passage.
        construction ScalarLocation: FiniteClause {
            head predicate;
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: ScalarLocationPredicate];
            require subject.ScalarDenotation = Yes;
            require finite_subject_number(subject.number, predicate.number, subject.DeterminerAgreementNumber) = Yes;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        schema ScalarLocationPredicate {
            form [head: lexical(Verb), complements: selected_frame(head, ScalarLocation)];
        }
        // SL1 (orchestrator ruling, 2026-10-11): an auxiliary selects the
        // plain scalar predicate; ScalarLocation still checks its Subject.
        policy PlainScalarHeadProperties {
            require head.form = Plain;
        }
        policy IrrealisScalarHeadAgreement {
            require head.form = Irrealis;
            require head.finiteness = Finite;
            require head.Clitic = No;
            export number = head.number;
            export person = head.person;
            export ScalarDenotation = Yes;
        }
        // IS1 (orchestrator ruling, 2026-10-10): the irrealis copula
        // selects the same scalar-location frame as the finite copula.
        instance ScalarLocationPredicate<Result, Properties>: [
            (ScalarLocationPredicate, FiniteHeadAgreement),
            (IrrealisPredicate, IrrealisScalarHeadAgreement),
            (BareScalarLocationPredicate, PlainScalarHeadProperties),
        ] { use Properties; }

        // Preposed temporal NP position: project ruling R11a (2026-10-09).
        // Keep the existing PP forms and their Reading identities.
        construction InitialPreposition: Clause {
            head clause;
            export ClauseCoordinationHost = Simple;
            form [dependent: PrepositionPhrase, ", ", clause: Clause]
                require licence_preposed_adjunct(dependent.PrepositionFunctionLicence) = Yes
                require preposed_complement_function(dependent.ComplementFunctionRestriction) = Yes
                require dependent.MatrixComplementRequirement = None
                require dependent.SelectedPrepositionUse = No;
            form [dependent: PrepositionPhrase, " ", clause: Clause]
                require dependent.ClauseInitialAdjunct = Yes
                require licence_preposed_adjunct(dependent.PrepositionFunctionLicence) = Yes
                require preposed_complement_function(dependent.ComplementFunctionRestriction) = Yes
                require dependent.MatrixComplementRequirement = None
                require dependent.SelectedPrepositionUse = No;
            form [dependent: NominalAdjunctPhrase, ", ", clause: Clause]
                require dependent.NominalAdjunctClass = Temporal;
            export UnmarkedConjunctLicence = Yes;
        }

        // Project ruling, trailing-adjunct-height (2026-10-09): only a
        // Clause Coordination licenses trailing Clause-level Adjunct scope.
        construction ClausalPreposition: Clause {
            head clause;
            require source_adjunct_admission(clause.SourceComplementSelection,
                dependent.SourceMarker) = Yes;
            require clause.ClauseCoordinationHost = Coordinated;
            form [clause: Clause, " ", dependent: PrepositionPhrase];
            require licence_adjunct(dependent.PrepositionFunctionLicence) = Yes;
            require adjunct_complement_function(dependent.ComplementFunctionRestriction,
                dependent.PredicativeAdjunctUse) = Yes;
            require dependent.MatrixComplementRequirement = None;
            require dependent.SelectedPrepositionUse = No;
        }

        construction MedialSupplementation: Clause {
            head none;
            export SourceComplementSelection = source_selection_union(
                anchor.SourceComplementSelection, continuation.SourceComplementSelection);
            export ClauseCoordinationHost = Simple;
            form [anchor: Clause, ", ", supplement: PrepositionPhrase, ", ",
                coordinator: lexical(Coordinator), " ", continuation: Clause];
            require licence_medial_supplement(supplement.PrepositionFunctionLicence) = Yes;
            require coordinator.NoncorrelativeCoordination = Yes;
            require anchor.terminal_punctuation = None;
            export UnmarkedConjunctLicence = anchor.UnmarkedConjunctLicence;
        }

        construction ClauseCoordination: Clause {
            head none;
            export SourceComplementSelection = source_selection_union(
                left.SourceComplementSelection, right.SourceComplementSelection);
            export ClauseCoordinationHost = Coordinated;
            form [left: Clause, ", ", coordinator: lexical(Coordinator), " ", right: Clause];
            form [left: Clause, " ", coordinator: lexical(Coordinator), " ", right: Clause]
                require coordinator.BareCoordination = Yes;
            require coordinator.NoncorrelativeCoordination = Yes;
            export UnmarkedConjunctLicence = left.UnmarkedConjunctLicence;
        }
        instance CoordinationSeriesEnd<Result, Member, Agreement = NoConcord,
            Properties = NoConcord, Distribution = GeneralCoordinatorDistribution>: [
            (SelectedComplementTailSeries, SelectedComplementTail, SelectedTailConcord,
                PrimitiveRightTail),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
            use Distribution(right, coordinator);
        }
        instance CoordinationSeriesContinuation<
            Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord>: [
            (PremodifierCoordination, Premodifier, Self, PremodifierConcord, AsyndeticPremodifierTail),
            (SelectedComplementTailSeries, SelectedComplementTail, Self, SelectedTailConcord),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
        }
        instance SerialCoordination<Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord, BoundaryProperties = NoFeatures>: [
            (PremodifierCoordination, Premodifier, Premodifier, PremodifierConcord),
            (SelectedComplementTail, Self, SelectedComplementTailSeries, SelectedTailConcord),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
            use BoundaryProperties;
        }


        // CF8 (orchestrator, 2026-10-10): like and unlike focused Adjuncts
        // share one member Category. Each form checks its own function licence.
        construction FocusedAdjunctMember: FocusedAdjunctMember {
            require head.FocusedUse = Yes;
            form [head: PrepositionPhrase]
                require licence_adjunct(head.PrepositionFunctionLicence) = Yes
                require adjunct_complement_function(head.ComplementFunctionRestriction,
                    head.PredicativeAdjunctUse) = Yes
                require head.SelectedPrepositionUse = No
                require head.MatrixComplementRequirement = None
                export NominalAdjunctHost = Yes;
            form [head: AdverbPhrase]
                require head.VPFinalAdjunct = Yes
                export NominalAdjunctHost = No;
        }
        // A bounded frequency stays inside its coordinate. The standalone
        // predicate route is retained; no extra standalone AdvP is admitted.
        instance NominalAdjunctPredicate<Result, Head: head>: [
            (FocusedAdjunctMember, AdverbPhrase),
        ] {
            require head.FocusedUse = Yes;
            require head.VPFinalAdjunct = Yes;
            export NominalAdjunctHost = No;
        }
        policy FocusedAdjunctConcord<Right, Source> {
            export NominalAdjunctHost = Right.NominalAdjunctHost;
            export ClauseComplementBoundary = coordinated_clause_complement_boundary(
                left.ClauseComplementBoundary, Right.ClauseComplementBoundary);
            export ConnectiveAdjunctUse = connective_adjunct_concord(
                left.ConnectiveAdjunctUse, Right.ConnectiveAdjunctUse);
            export SourceMarker = coordinated_source_marker(left.SourceMarker, Right.SourceMarker);
            export PrepositionPhraseComplementMarker = coordinated_preposition_phrase_complement_marker(
                left.PrepositionPhraseComplementMarker, Right.PrepositionPhraseComplementMarker);
            export SourceComplementUse = coordinated_source_complement_use(
                left.SourceComplementUse, Right.SourceComplementUse);
        }
        policy FocusedAdjunctFunction {
            export PrepositionFunctionLicence = Adjunct;
            export ComplementFunctionRestriction = Adjunct;
            export PredicativeAdjunctUse = No;
            export SelectedPrepositionUse = No;
            export MatrixComplementRequirement = None;
            export InternalisedComplementMarker = No;
        }
        instance Coordination<Result, Member: (left, right)>: [
            (FocusedAdjunctCoordination, FocusedAdjunctMember),
        ] {
            use FocusedAdjunctConcord(right, coordinator);
            use FocusedAdjunctFunction;
            use GeneralCoordinatorDistribution(right, coordinator);
        }
        instance CoordinationSeriesEnd<Result, Member: (left, right)>: [
            (FocusedAdjunctSeries, FocusedAdjunctMember),
        ] {
            use FocusedAdjunctConcord(right, coordinator);
            use GeneralCoordinatorDistribution(right, coordinator);
        }
        instance CoordinationSeriesContinuation<Result, Member: left, Tail: rest>: [
            (FocusedAdjunctSeries, FocusedAdjunctMember, Self),
        ] {
            use FocusedAdjunctConcord(rest, rest);
        }
        instance SerialCoordination<Result, Member: left, Tail: rest>: [
            (FocusedAdjunctCoordination, FocusedAdjunctMember, FocusedAdjunctSeries),
        ] {
            use FocusedAdjunctConcord(rest, rest);
            use FocusedAdjunctFunction;
        }

        construction Noun: Nominal {
            export UnmarkedConjunctLicence = Yes;
            export SubjectRelativeNumber = None;
            form [head: lexical(Noun)];
            require head.framing = Unframed;
            export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
            export PropertyValuePosition = head.PropertyValuePosition;
            export Targeting = No;
            export number = head.number;
            export countability = head.countability;
        }

        construction BareFramedNoun: Nominal {
            export UnmarkedConjunctLicence = Yes;
            export SubjectRelativeNumber = None;
            form [head: lexical(Noun)];
            require head.frame = BareNominal;
            export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
            export PropertyValuePosition = head.PropertyValuePosition;
            export Targeting = No;
            export number = head.number;
            export countability = head.countability;
        }

        // MS1–MS2: the declared symbol class supplies a nominal head.
        construction NominalSymbol: NominalSymbol {
            form ["{", head: lexical(Symbol), "}"]
                require symbol_nominal_use(head.SymbolNominalUse) = Yes
                export NominalAgreementNumber = symbol_nominal_agreement(
                    head.SymbolNominalUse, head.ManaSymbolUse);
            form ["{", head: lexical(Numeral), "}"]
                require head.numeral_kind = Arabic
                export SymbolNominalUse = Count
                export NominalAgreementNumber = Head;
        }

        // A count-symbol continuation cannot borrow the measured mass use.
        construction CountNominalSymbol: CountNominalSymbol {
            form [head: NominalSymbol];
            require head.SymbolNominalUse = Count;
            export SymbolNominalUse = Count;
        }

        // QE1–QE2 and MS1–MS5: one determined or measured symbol-sequence head.
        construction MeasuredSymbolNominal: Nominal {
            form [head: NominalSymbol, rest: repeat(CountNominalSymbol, "")];
            require symbol_nominal_sequence(head.SymbolNominalUse,
                last(rest).SymbolNominalUse) = Yes;
            export DeterminerRequirement = Yes;
            export MeasurePosition = symbol_measure_position(head.NominalAgreementNumber);
            // QS4/QS7: CardinalDeterminer and QuantifiedNounPhrase own all
            // quantified symbols; there is no parallel cardinal-measure route.
            export number = Singular;
            export countability = symbol_nominal_countability(head.SymbolNominalUse);
            export Targeting = No;
        }

        // CGEL Ch. 14 §8.2, pp. 1259–1260: selected subjectless
        // to-infinitivals are internal Complements of nouns.
        construction SelectedComplementNominal: Nominal {
            export UnmarkedConjunctLicence = Yes;
            export NominalDependentPresent = Yes;
            export SubjectRelativeNumber = None;
            export BareGenitiveHost = No;
            form [head: lexical(Noun), " ", complement: InfinitiveComplement]
                require head.frame = NominalInfinitive;
            // CK1–CK3: the noun frame selects the whole keyword-complemented PP.
            form [head: lexical(Noun), " ", complement: SelectedKeywordPreposition]
                require head.frame = NominalKeywordPreposition
                require complement.NominalComplementMarker = Of;
            export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
            export PropertyValuePosition = head.PropertyValuePosition;
            export Targeting = No;
            export number = head.number;
            export countability = head.countability;
        }

        construction SymbolComplementNominal: Nominal {
            export UnmarkedConjunctLicence = Yes;
            export NominalDependentPresent = Yes;
            export SubjectRelativeNumber = None;
            export BareGenitiveHost = No;
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ",
                complement: CostSymbols];
            require head.frame = NominalSymbols;
            require marker.NominalComplementMarker = Of;
            require licence_noun_complement(marker.PrepositionFunctionLicence) = Yes;
            export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
            export PropertyValuePosition = head.PropertyValuePosition;
            export Targeting = No;
            export number = head.number;
            export countability = head.countability;
        }

        table nominal_preposition_frame(frame) -> Selection {
            (NominalPreposition) => Yes, (NominalToPreposition) => Yes,
        }
        // Orchestrator rulings DV1–DV4: the selected colour Complement stays
        // inside the noun's frame, without a general colour-word NP route.
        construction PrepositionComplementNominal: Nominal {
            export UnmarkedConjunctLicence = Yes;
            export NominalDependentPresent = Yes;
            export SubjectRelativeNumber = None;
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase]
                require nominal_preposition_frame(head.frame) = Yes
                require nominal_complement_marker(head.NominalComplementMarker,
                    marker.NominalComplementMarker) = Yes;
            // KC1–KC3 (kind-of-counter ruling, 2026-10-10): the licensed
            // head selects a bare count Nominal; Postmodifiers attach outside it.
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ",
                complement: Nominal]
                require head.framing = Unframed
                require head.TaxonomicComplementUse = Yes
                require marker.NominalComplementMarker = Of
                require complement.number = Singular
                require complement.countability = Count
                require complement.NominalDependentPresent = No
                require complement.DeterminerRequirement = No;
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ",
                complement: AdjectivePhrase]
                require head.frame = NominalColorWordPreposition
                require complement.AdjectiveComplementClass = ColorWord
                require nominal_complement_marker(head.NominalComplementMarker,
                    marker.NominalComplementMarker) = Yes;
            require licence_noun_complement(marker.PrepositionFunctionLicence) = Yes;
            export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
            export PropertyValuePosition = head.PropertyValuePosition;
            export Targeting = No;
            export BareGenitiveHost = No;
            export number = head.number;
            export countability = head.countability;
        }

        construction Adjective: AdjectivePhrase {
            export AdjectiveAttributiveOnlyUse = No;
            export ImplicitComparativeUse = No;
            form [head: lexical(Adjective)];
            require head.framing = Unframed;
            export AdjectiveStructure = Simple;
        }

        construction IntransitiveAdjective: AdjectivePhrase {
            export AdjectiveAttributiveOnlyUse = implicit_comparative_attributive_use(
                head.ImplicitComparativeUse);
            form [head: lexical(Adjective)];
            require head.frame = Intransitive;
            export AdjectiveStructure = Simple;
        }

        construction ComplementedAdjective: AdjectivePhrase {
            export AdjectiveComplementClass = None;
            export AdjectiveAttributiveOnlyUse = No;
            export ImplicitComparativeUse = No;
            form [head: lexical(Adjective), " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require head.frame = AdjectivalPrepositionComplement;
            require marker.InternalisedComplementMarker = Yes;
            export AdjectiveStructure = Complemented;
            export TargetingPremodifierUse = No;
        }

        // The Adjective selects its infinitival Complement; DS3 licenses
        // the resulting complemented phrase on the existing PostpositiveNominal route.
        construction InfinitivalAdjective: AdjectivePhrase {
            export AdjectiveComplementClass = None;
            export AdjectiveAttributiveOnlyUse = No;
            export ImplicitComparativeUse = No;
            form [head: lexical(Adjective), " ", complement: InfinitiveComplement];
            require head.frame = AdjectivalInfinitive;
            export AdjectiveStructure = Complemented;
            export TargetingPremodifierUse = No;
        }

        construction NounPremodifier: NounPremodifier {
            export NounPremodifier = head.NounPremodifier;
            form [head: lexical(Noun)]
                require noun_premodifier_use(head.NounPremodifier) = Yes
                require head.form = Singular
                require head.framing = Unframed;
            form [head: lexical(Keyword)]
                require head.NounPremodifier = Keyword
                require head.BoundKeyword = No;
        }
        instance Coordination<Result, Member, Agreement = NoConcord, Properties = NoConcord, Distribution = GeneralCoordinatorDistribution, BoundaryProperties = NoFeatures>: [
            (SelectedComplementTail, Self, SelectedTailConcord, PrimitiveRightTail),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
            use Distribution(right, coordinator);
            use BoundaryProperties;
        }

        // CGEL Ch. 15 §3.2, pp. 1326–1328 permits unlike categories sharing a
        // function, including pre-head modifiers. R15e (2026-10-09) supplies
        // the Oracle grouping and keeps coordinator-ending lists unchanged.
        construction NounPremodifierMember: Premodifier {
            export PremodifierCategory = Nominal;
            export SyndeticPremodifier = No;
            form [head: NounPremodifier];
            require head.PremodifierCoordination = No;
        }

        construction AdjectivePremodifierMember: Premodifier {
            export PremodifierCategory = Adjectival;
            export SyndeticPremodifier = No;
            form [head: AdjectivePhrase];
            require head.PremodifierCoordination = No;
            require head.AdjectiveStructure = Simple;
            require head.AdjectivePremodifierUse = Yes;
            export NounPremodifier = None;
            export SelectedPrepositionUse = Yes;
            export PredicativeAdjunctUse = Yes;
        }

        table coordinated_premodifier_property_value(NounPremodifier, AdjectiveOrigin,
            PropertyValuePosition) -> PropertyValuePosition {
            (Yes, _, _) => None, (Attribute, _, _) => None, (Keyword, _, _) => None,
            (None, Participial, _) => None,
            (None, Ordinary, None) => None, (None, Ordinary, Single) => Single,
            (None, Ordinary, First) => First, (None, Ordinary, Second) => Second,
            (None, Ordinary, Pair) => Pair,
        }

        // CP1–CP4 (orchestrator, coordinated-participial-premodifier,
        // 2026-10-10): one shared Modifier retains each coordinate's Category.
        policy AsyndeticPremodifierTail<Right, Source> {
            require Right.SyndeticPremodifier = No;
        }
        policy SyndeticPremodifierConcord<Right, Source> {
            export PremodifierCategory = premodifier_category(left.PremodifierCategory,
                Right.PremodifierCategory);
            export SyndeticPremodifier = Yes;
            export NounPremodifier = None;
            export AdjectiveOrigin = coordinated_adjective_origin(left.AdjectiveOrigin,
                Right.AdjectiveOrigin);
            export TargetingPremodifierUse = coordinated_targeting_premodifier(
                left.TargetingPremodifierUse, Right.TargetingPremodifierUse);
            export SelectedPrepositionUse = selected_preposition_complement(
                left.SelectedPrepositionUse, Right.SelectedPrepositionUse);
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
        }
        instance Coordination: PremodifierCoordination {
            bind left, right = Premodifier;
            require left.SyndeticPremodifier = No;
            require right.SyndeticPremodifier = No;
            require unlike_premodifiers(left.PremodifierCategory, right.PremodifierCategory) = Yes;
            use SyndeticPremodifierConcord(right, coordinator);
            use GeneralCoordinatorDistribution(right, coordinator);
        }
        instance CoordinationSeriesEnd: Premodifier {
            bind left, right = Premodifier;
            require left.SyndeticPremodifier = No;
            require right.SyndeticPremodifier = No;
            require syndetic_premodifier_member(left.PremodifierCategory) = Yes;
            require syndetic_premodifier_member(right.PremodifierCategory) = Yes;
            use SyndeticPremodifierConcord(right, coordinator);
            use GeneralCoordinatorDistribution(right, coordinator);
        }
        instance CoordinationSeriesContinuation: Premodifier {
            bind left, rest = Premodifier;
            require left.SyndeticPremodifier = No;
            require rest.SyndeticPremodifier = Yes;
            require syndetic_premodifier_member(left.PremodifierCategory) = Yes;
            use SyndeticPremodifierConcord(rest, rest);
        }

        construction CoordinatedPremodifiedNominal: Nominal {
            export UnmarkedConjunctLicence = No;
            form [modifier: PremodifierCoordination, " ", head: Nominal];
            require licensed_noun_premodifier(modifier.NounPremodifier,
                head.AttributePremodifierHead, head.KeywordPremodifierHead) = Yes;
            require targeting_premodifier(modifier.TargetingPremodifierUse, head.Targeting) = Yes;
            export NominalDependentPresent = Yes;
            export PropertyValuePosition = coordinated_premodifier_property_value(
                modifier.NounPremodifier, modifier.AdjectiveOrigin, head.PropertyValuePosition);
            export SelectedPrepositionUse = selected_preposition_complement(
                modifier.SelectedPrepositionUse, head.SelectedPrepositionUse);
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                modifier.PredicativeAdjunctUse, head.PredicativeAdjunctUse);
        }

        construction NounPremodifiedNominal: Nominal {
            form [modifier: NounPremodifier, " ", head: Nominal];
            require licensed_noun_premodifier(modifier.NounPremodifier,
                head.AttributePremodifierHead, head.KeywordPremodifierHead) = Yes;
            export NominalDependentPresent = Yes;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            require head.Targeting = No;
        }

        construction PremodifiedNominal: Nominal {
            form [modifier: AdjectivePhrase, " ", head: Nominal];
            export NominalDependentPresent = Yes;
            export ComparativeModifierPresent = comparative_modifier_present(
                modifier.ImplicitComparativeUse, head.ComparativeModifierPresent);
            export PropertyValuePosition = premodified_property_value_position(
                modifier.AdjectiveOrigin, head.PropertyValuePosition);
            require modifier.AdjectiveStructure = Simple;
            require modifier.AdjectivePremodifierUse = Yes;
            require targeting_premodifier(modifier.TargetingPremodifierUse, head.Targeting) = Yes;
        }

        table superlative_quantifier_license(DeterminerUse, number, countability) -> Selection {
            (PluralOrMass, Plural, Count) => Yes, (PluralOrMass, Singular, Mass) => Yes,
            (PluralCount, Plural, Count) => Yes, (Mass, Singular, Mass) => Yes,
            (Unrestricted, Singular, Count) => Yes, (Unrestricted, Plural, Count) => Yes,
            (Unrestricted, Singular, Mass) => Yes,
        }
        // SM1–SM4 and the owner ruling (2026-10-11) assign internal Modifier
        // function to the declared superlative class. CGEL p. 1166 [15] backs
        // the superlative meaning, not this project choice of internal structure.
        construction SuperlativePremodifiedNominal: Nominal {
            form [modifier: lexical(Determinative), " ", head: Nominal];
            form [modifier: lexical(Adjective), " ", head: Nominal];
            require modifier.SuperlativeQuantifierUse = Yes;
            require targeting_premodifier(modifier.TargetingPremodifierUse, head.Targeting) = Yes;
            require superlative_quantifier_license(modifier.DeterminerUse,
                head.number, head.countability) = Yes;
            export NominalDependentPresent = Yes;
            export DeterminerRequirement = coordinated_determiner_requirement(
                modifier.DeterminerRequirement, head.DeterminerRequirement);
            export PropertyValuePosition = head.PropertyValuePosition;
        }

        // CGEL pp. 393, 395 gives comparative Determiner function and head
        // selection; CQ1–CQ4 (2026-10-10) supplies the shared representation
        // and immediate NP Complement analysis. CGEL p. 1139 describes same/as.
        schema DiscontinuousComparativeNominal {
            form [governor: node, " ", head: Nominal, " ",
                marker: lexical(Preposition), " ", complement: AccusativePhrase];
            require licence_comparative_complement(marker.PrepositionFunctionLicence) = Yes;
            require head.Targeting = No;
        }

        // AC2/AC4: the declared selected frame survives governor coordination,
        // with one expanded Comparative Complement after the Nominal.
        table discontinuous_governor_frame(frame, ImplicitComparativeUse) -> Selection {
            (DiscontinuousEquality, _) => Yes, (Ordering, Yes) => Yes,
            (Equality, Yes) => Yes,
        }
        table coordinated_discontinuous_frame(frame, frame) -> frame {
            (DiscontinuousEquality, DiscontinuousEquality) => DiscontinuousEquality,
            (Ordering, Ordering) => Ordering,
            (Equality, Ordering) => Ordering, (Ordering, Equality) => Ordering,
        }
        table discontinuous_comparative_marker(frame, ExpandedComparisonMarker) -> Selection {
            (DiscontinuousEquality, Equality) => Yes, (Ordering, Ordering) => Yes,
        }
        construction DiscontinuousComparativeGovernorHead: DiscontinuousComparativeGovernor {
            form [head: lexical(Adjective)];
            require discontinuous_governor_frame(head.frame, head.ImplicitComparativeUse) = Yes;
            export frame = head.frame;
        }
        coordinable DiscontinuousComparativeGovernor using WithoutGeneralCoordinatorRequirement {
            forms binary;
        }
        construction ComparativeDeterminerHead: ComparativeDeterminerHead {
            form [head: lexical(Determinative)];
            require head.ComparativeDeterminerUse = Yes;
            export DeterminerUse = head.DeterminerUse;
        }

        policy DiscontinuousComparativeModifier {
            require discontinuous_comparative_marker(governor.frame,
                marker.ExpandedComparisonMarker) = Yes;
            export ComparativeModifierPresent = comparative_modifier_present(
                governor.ImplicitComparativeUse, head.ComparativeModifierPresent);
        }

        policy DiscontinuousComparativeDeterminer {
            export Definiteness = Nondefinite;
            require governor.ComparativeDeterminerUse = Yes;
            require determiner_license(governor.DeterminerUse, head.number, head.countability,
                head.DeterminerRequirement, governor.NominalAdjunctDependent) = Yes;
            require marker.ExpandedComparisonMarker = Ordering;
            require head.DeterminerRequirement = No;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber,
                head.NominalAgreementNumber, head.number) = Yes;
            export UnitBearing = nominal_unit_bearing(head.MeasurePosition);
            export BarePrepositionUse = No;
        }

        instance DiscontinuousComparativeNominal<Result, Governor: governor,
            HeadFeatures, Properties>: [
            (Nominal, DiscontinuousComparativeGovernor, NominalHeadFeatures,
                DiscontinuousComparativeModifier),
            (NounPhrase, ComparativeDeterminerHead, NounPhraseHeadAgreement,
                DiscontinuousComparativeDeterminer),
        ] { use HeadFeatures; use Properties; }

        // CGEL Ch. 5 §7.6, p. 386: numerical cardinals can modify under
        // an outer Determiner; without it the cardinal is the Determiner.
        construction CardinalPremodifiedNominal: Nominal {
            export NominalDependentPresent = Yes;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            form [quantity: Cardinal, " ", head: Nominal];
            require head.DeterminerRequirement = No;
            require head.countability = Count;
            agree quantity.number = head.number;
            export DeterminerRequirement = Yes;
        }

        // Orchestrator ruling 2026-10-09: this licence supplies a measured Nominal
        // under a Determiner; it does not open the closed bare measured NP.
        table determined_cardinal_measurement(CardinalMeasurementUse) -> Selection {
            (DeterminerRequired) => Yes, (Yes) => Yes,
        }
        table measured_nominal_cardinal_use(CardinalMeasurementUse, CardinalMeasurementUse)
            -> CardinalMeasurementUse {
            (Yes, No) => No,
            (Yes, DeterminerRequired) => DeterminerProjection,
            (DeterminerRequired, No) => DeterminerProjection,
            (DeterminerRequired, DeterminerRequired) => DeterminerProjection,
        }

        // Amounts remain singular measurements; the lexical mass noun does
        // not acquire a bare count-plural use. CGEL Ch. 5 §3.4, p. 354
        // distinguishes the singular conceptualization of a measured quantity.
        construction CardinalMeasuredNominal: Nominal {
            export NominalDependentPresent = Yes;
            export NominalAgreementNumber = Head;
            export ScalarDenotation = Yes;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            form [quantity: QuantityConjunct, " ", head: Nominal];
            require head.MeasurePosition = Before;
            require determined_cardinal_measurement(head.CardinalMeasurementUse) = Yes;
            require head.number = Singular;
            require head.countability = Mass;
            require head.DeterminerRequirement = No;
            export number = Singular;
            export countability = Count;
            export DeterminerRequirement = Yes;
            export MeasurePosition = None;
            export CardinalMeasurementUse = measured_nominal_cardinal_use(
                head.CardinalMeasurementUse, quantity.CardinalMeasurementUse);
            export NominalAdjunctClass = None;
            export SlashPremodifierUse = No;
            export NumberTransparency = No;
            export BareGenitiveHost = No;
        }

        // QE6: quantification satisfies an unmeasured mass head's declared
        // Determiner Requirement, without determining an already measured Nominal.
        table quantifying_determiner_requirement(DeterminerRequirement, MeasurePosition,
            countability) -> Selection {
            (No, _, _) => Yes, (Yes, Before, Mass) => Yes, (Yes, Before, Count) => Yes,
        }

        construction CardinalMeasuredNounPhrase: NounPhrase {
            export PartitiveCountability = partitive_countability(head.countability);
            export UnitBearing = Yes;
            export ScalarDenotation = Yes;
            export BarePrepositionUse = No;
            form [quantity: Cardinal, " ", head: Nominal];
            require head.MeasurePosition = Before;
            require head.CardinalMeasurementUse = Yes;
            require head.number = Singular;
            require head.countability = Mass;
            require quantifying_determiner_requirement(head.DeterminerRequirement,
                head.MeasurePosition, head.countability) = Yes;
            export number = Singular;
            export BareGenitiveHost = No;
            use ThirdPersonCommonCase;
            require singular_relative_number(head.SubjectRelativeNumber) = Yes;
        }

        schema PostpositiveNominal {
            form [head: node, " ", modifier: node];
        }
        policy PostpositiveNominalProperties {
            export BareGenitiveHost = No;
            require modifier.AdjectiveStructure = Complemented;
            require modifier.AdjectiveAttributiveOnlyUse = No;
        }
        // Pre-integrate review ruling (2026-10-09): a declared fused-head NP
        // licence admits the comparative Postmodifier without pruning nominal scope.
        policy PostpositiveNounPhraseProperties {
            export NominalDependentPresent = Yes;
            require unquantified_comparative(modifier.PropertyEqualityForm) = Yes;
            require head.ComparativePostmodifierUse = Yes;
            export ComparativePostmodifierUse = No;
        }
        instance PostpositiveNominal<Result, Head: head, Modifier: modifier,
            HeadFeatures, Properties>: [
            (Nominal, Self, AdjectivePhrase, NominalHeadFeatures,
                PostpositiveNominalProperties),
            (NounPhrase, Self, ComparativeAdjectivePhrase, NoFeatures,
                PostpositiveNounPhraseProperties),
        ] { use HeadFeatures; use Properties; }

        // CGEL Ch. 5 §14.2, p. 444: participial VPs can premodify a noun.
        table verbal_premodifier(form, frame, PastParticipialPremodifier) -> Selection {
            (GerundParticiple, Intransitive, No) => Yes,
            (GerundParticiple, Intransitive, Yes) => Yes,
            (PastParticiple, Transitive, Yes) => Yes,
        }
        schema VerbalPremodifier {
            form [head: lexical(Verb)];
            require verbal_premodifier(head.form, head.frame, head.PastParticipialPremodifier) = Yes;
        }
        instance VerbalPremodifier: VerbalPremodifier {}
        instance VerbalPremodifier: Premodifier {
            export PremodifierCategory = Verbal;
            export SyndeticPremodifier = No;
            export NounPremodifier = None;
            export AdjectiveOrigin = Participial;
            export TargetingPremodifierUse = No;
            export SelectedPrepositionUse = Yes;
            export PredicativeAdjunctUse = Yes;
        }

        construction ParticipialPremodifier: Nominal {
            form [modifier: VerbalPremodifier, " ", head: Nominal];
            export NominalDependentPresent = Yes;
            require head.Targeting = No;
        }

        construction CatalogName: Name {
            export number = name_number(head.NameNumber);
            form [head: lexical(Catalog)];
            require head.IdentityUse = Name;
        }

        // CGEL p. 516: a referential proper name has NP status.
        // Owner ruling (2026-10-08): Oracle names carry the grammatical
        // agreement declared on their lexeme.
        construction ProperNameNounPhrase: NounPhrase {
            export PartitiveCountability = Count;
            export NounPhraseStructure = ProperName;
            export Targeting = No;
            form [head: Name];
            use ThirdPersonCommonCase;
        }

        // CGEL Ch. 15 §5.2(b), pp. 1357–1358: an indefinite NP can
        // supplement a proper-name anchor. NA1–NA4 (orchestrator ruling,
        // 2026-10-10) license this unclosed NP shape wherever a name occurs.
        construction AscriptiveNounPhraseSupplement: NounPhrase {
            head anchor;
            export NominalDependentPresent = Yes;
            form [anchor: NounPhrase, ", ", supplement: NounPhrase];
            require anchor.NounPhraseStructure = ProperName;
            require supplement.NounPhraseStructure = IndefiniteArticle;
            require supplement.Definiteness = Nondefinite;
            export FocusedUse = anchor.FocusedUse;
        }

        construction PassiveNamePredicate: NamePredicate {
            form [head: lexical(Verb), " ", complement: Name];
            require head.form = PastParticiple;
            require head.frame = ObjectName;
        }

        construction NamedNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: NamePredicate];
            export NominalDependentPresent = Yes;
        }

        construction TargetedNominal: Nominal {
            export NominalDependentPresent = Yes;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            form [marker: lexical(Determinative), " ", head: Nominal];
            require marker.Targeting = Yes;
            require head.Targeting = No;
            export Targeting = Yes;
            export NominalAdjunctClass = None;
        }

        construction PostmodifiedNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: PrepositionPhrase];
            require licence_modifier(modifier.PrepositionFunctionLicence) = Yes;
            require modifier.ComplementFunctionRestriction = None;
            require modifier.SelectedPrepositionUse = No;
            require nominal_postmodifier_licence(head.NominalComplementMarker,
                modifier.NominalComplementMarker) = Yes;
            export NominalDependentPresent = Yes;
            export SourceModifierPresent = nominal_source_modifier(head.SourceModifierPresent, modifier.SourceMarker);
            export WholeObjectSourcePresent = whole_object_source_modifier(
                head.WholeObjectSourcePresent, modifier.WholeObjectSourcePresent, modifier.SourceMarker);
            export ObliqueNumber = nominal_oblique(head.ObliqueNumber, modifier.ObliqueNumber);
        }

        table partitive_oblique(ObliqueNumber) -> Selection {
            (Singular) => Yes, (Plural) => Yes,
        }
        construction PartitiveModifier: PartitiveModifier {
            form [" ", phrase: PrepositionPhrase];
            require licence_modifier(phrase.PrepositionFunctionLicence) = Yes;
            require phrase.ComplementFunctionRestriction = None;
            require phrase.SelectedPrepositionUse = No;
        }
        // PQ1: the quantitative fused head retains its declared agreement.
        table partitive_quantity_number(DeterminerAgreementNumber, DeterminerUse) -> number {
            (Singular, SingularCount) => Singular, (Singular, Singular) => Singular,
            (Nominal, SingularCount) => Singular, (Nominal, Singular) => Singular,
            (Nominal, PluralCount) => Plural, (Nominal, Mass) => Singular,
        }
        construction PartitiveNounPhrase: NounPhrase {
            head quantity;
            export NominalDependentPresent = Yes;
            export DeterminerAgreementNumber = Nominal;
            form [quantity: QuantitativeDeterminer, " ", complement: PrepositionPhrase,
                modifiers: repeat(PartitiveModifier, "")]
                export number = partitive_quantity_number(quantity.DeterminerAgreementNumber,
                    quantity.DeterminerUse)
                export Definiteness = quantity.Definiteness;
            form [quantity: QuantityConjunct, " ", complement: PrepositionPhrase,
                modifiers: repeat(PartitiveModifier, "")]
                require quantity.CardinalMeasurementUse = DeterminerRequired
                require complement.ObliqueNumber = Singular
                export number = quantity.number
                export Definiteness = Nondefinite;
            require partitive_oblique(complement.ObliqueNumber) = Yes;
            require licence_noun_complement(complement.PrepositionFunctionLicence) = Yes;
            require complement.ComplementFunctionRestriction = None;
            export Targeting = No;
            use ThirdPersonCommonCase;
        }

        table determiner_partitive_oblique(DeterminerUse, ObliqueNumber, PartitiveCountability) -> Selection {
            (SingularCount, Plural, _) => Yes,
            (Unrestricted, Singular, Mass) => Yes,
            (Unrestricted, Singular, Unrestricted) => Yes,
            (Unrestricted, Plural, _) => Yes,
            (PluralOrMass, Singular, Mass) => Yes,
            (PluralOrMass, Singular, Unrestricted) => Yes,
            (PluralOrMass, Plural, _) => Yes,
            (PluralCount, Plural, _) => Yes,
            (Mass, Singular, Mass) => Yes,
            (Mass, Singular, Unrestricted) => Yes,
        }
        table determiner_partitive_number(DeterminerUse, ObliqueNumber) -> number {
            (SingularCount, Plural) => Singular,
            (Unrestricted, Singular) => Singular, (Unrestricted, Plural) => Plural,
            (PluralOrMass, Singular) => Singular, (PluralOrMass, Plural) => Plural,
            (PluralCount, Plural) => Plural, (Mass, Singular) => Singular,
        }
        // One NP Reading; the finite predicate selects agreement for an
        // unrestricted head over a plural oblique (PD2 and owner ruling W9-A).
        table partitive_agreement(DeterminerUse, ObliqueNumber) -> DeterminerAgreementNumber {
            (Unrestricted, Plural) => SingularOrPlural,
            (SingularCount, Plural) => Nominal,
            (Unrestricted, Singular) => Nominal,
            (PluralOrMass, Singular) => Nominal, (PluralOrMass, Plural) => Nominal,
            (PluralCount, Plural) => Nominal, (Mass, Singular) => Nominal,
        }

        // CGEL Ch. 5 §§9.1–9.2, pp. 411–413: explicitly partitive determiner-head.
        // PD2 (orchestrator, 2026-10-10) specifies oblique and agreement selection.
        construction DeterminerHeadPartitiveNounPhrase: NounPhrase {
            export NominalDependentPresent = Yes;
            export LocativeUse = No;
            export PartitiveCountability = complement.PartitiveCountability;
            export Definiteness = Unspecified;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = partitive_agreement(head.DeterminerUse,
                complement.ObliqueNumber);
            form [head: lexical(Determinative), " ", complement: PrepositionPhrase,
                modifiers: repeat(PartitiveModifier, "")];
            require head.NominalComplementMarker = Of;
            agree head.NominalComplementMarker = complement.NominalComplementMarker;
            require determiner_partitive_oblique(head.DeterminerUse, complement.ObliqueNumber,
                complement.PartitiveCountability) = Yes;
            require licence_noun_complement(complement.PrepositionFunctionLicence) = Yes;
            require complement.ComplementFunctionRestriction = None;
            export number = determiner_partitive_number(head.DeterminerUse, complement.ObliqueNumber);
            export Targeting = No;
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            use ThirdPersonCommonCase;
        }

        // CGEL Ch. 5 §9.2(c), pp. 414–415: a singular demonstrative
        // can fuse Determiner and Head without an overt nominal.
        construction FusedDeterminerHeadNounPhrase: NounPhrase {
            // AE1–AE2 and AE6 (orchestrator, 2026-10-10/11):
            // immediate dependent and locative use; CGEL Ch. 5 §9.6, p. 423.
            export NominalDependentPresent = posthead_dependent_presence(dependent.present);
            export PartitiveCountability = Unrestricted;
            export UnitBearing = No;
            form [head: lexical(Determinative),
                dependent: optional(lexical(Preposition), before(" "))]
                require dependent.FusedHeadPostmodifierUse = Yes
                require licensed_posthead_dependent(dependent.present,
                    head.ImmediatePostheadDependentUse) = Yes
                require head.FusedHeadUse = Yes
                require head.DeterminerUse = Singular
                export Definiteness = head.Definiteness
                export ComparativePostmodifierUse = head.ComparativePostmodifierUse
                export DeterminerAgreementNumber = Nominal
                export CaseUse = Common;
            // CGEL Ch. 5 §9.1, p. 410: independent Genitives have head use.
            // IG7 reuses the one-Reading partitive agreement convention.
            form [head: lexical(Pronoun),
                dependent: optional(lexical(Preposition), before(" "))]
                require dependent.present = No
                require head.case = Genitive
                require head.form = IndependentGenitive
                export Definiteness = Definite
                export ComparativePostmodifierUse = No
                export DeterminerAgreementNumber = SingularOrPlural
                export CaseUse = Genitive;
            export number = Singular;
            export Targeting = No;
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            export SubjectStructure = Phrase;
            export person = Third;
        }

        // CGEL Ch. 5 §12(b–c), p. 434: fractions and multipliers are
        // external Modifiers; HR1–HR2 require a declared definite host.
        construction PredeterminerNounPhrase: NounPhrase {
            export NounPhraseStructure = predeterminer_structure(modifier.PredeterminerUse);
            form [modifier: lexical(Adverb), " ", head: NounPhrase];
            form [modifier: lexical(Noun), " ", head: NounPhrase];
            form [modifier: lexical(Determinative), " ", head: NounPhrase]
                require predeterminer_number(modifier.DeterminerUse, head.number,
                    head.PartitiveCountability) = Yes;
            require predeterminer_host(modifier.PredeterminerUse, head.Definiteness,
                head.NounPhraseStructure) = Yes;
        }

        construction DeterminedNounPhrase: NounPhrase {
            export PartitiveCountability = partitive_countability(head.countability);
            export Definiteness = determiner.Definiteness;
            export DeterminerAgreementNumber = determiner.DeterminerAgreementNumber;
            export ScalarDenotation = quantificational_scalar_denotation(
                determiner.QuantificationalDeterminer, head.NumberTransparency, head.ScalarDenotation);
            export BarePrepositionUse = No;
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Ordinary;
            require determiner_license(determiner.DeterminerUse, head.number,
                head.countability, head.DeterminerRequirement, determiner.NominalAdjunctDependent) = Yes;
            export number = quantificational_number(determiner.QuantificationalDeterminer,
                head.NumberTransparency, head.ObliqueNumber, head.number, head.NominalAgreementNumber,
                determiner.DeterminerAgreementNumber);
            use ThirdPersonCommonCase;
            require resolved_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number, determiner.DeterminerAgreementNumber) = Yes;
        }

        table nondefinite_determiner_requirement(DeterminerRequirement) -> Selection {
            (No) => Yes, (Yes) => Yes,
        }

        construction IndefiniteNounPhrase: NounPhrase {
            export PartitiveCountability = partitive_countability(head.countability);
            export NounPhraseStructure = IndefiniteArticle;
            export Definiteness = Nondefinite;
            export BarePrepositionUse = No;
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Indefinite;
            require determiner.number = Singular;
            require head.number = Singular;
            require head.countability = Count;
            agree determiner.article_onset = head.onset;
            export number = determiner.number;
            use ThirdPersonCommonCase;
            require singular_relative_number(head.SubjectRelativeNumber) = Yes;
            require nondefinite_determiner_requirement(head.DeterminerRequirement) = Yes;
        }

        table bare_noun_phrase_license(number, countability, ScalarDenotation,
            ComparativeModifierPresent) -> Selection {
            (Plural, Count, _, _) => Yes, (Singular, Mass, _, _) => Yes,
            (Singular, Count, Yes, Yes) => Yes,
        }
        construction BareNounPhrase: NounPhrase {
            export Definiteness = Nondefinite;
            export BarePrepositionUse = No;
            form [head: Nominal];
            require bare_noun_phrase_license(head.number, head.countability,
                head.ScalarDenotation, head.ComparativeModifierPresent) = Yes;
            require head.DeterminerRequirement = No;
            use NounPhraseHeadAgreement;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
        }

        // CGEL Ch. 7 §3.1, pp. 620–623: the bare NP is internal to the PP.
        construction BarePrepositionNounPhrase: NounPhrase {
            form [head: Nominal];
            require bare_preposition_use(head.BarePrepositionUse) = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.DeterminerRequirement = No;
            use NounPhraseHeadAgreement;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
        }

        construction TargetNounPhrase: NounPhrase {
            export PartitiveCountability = partitive_countability(head.countability);
            export Definiteness = Nondefinite;
            export BarePrepositionUse = No;
            form [marker: lexical(Determinative), " ", head: Nominal];
            require marker.Targeting = Yes;
            require head.Targeting = No;
            require head.number = Singular;
            require head.DeterminerRequirement = No;
            export number = nominal_agreement_number(head.NominalAgreementNumber, head.number);
            use ThirdPersonCommonCase;
            export Targeting = Yes;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
        }

        schema NominativePronoun { form [head: lexical(Pronoun)]; }
        policy OrdinaryNominativePronoun {
            export NominalDependentPresent = No;
            export LocativeUse = No;
            export PartitiveCountability = Unrestricted;
            export Definiteness = Unspecified;

            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export BarePrepositionUse = No;
            export SubjectStructure = Pronoun;
            export BareGenitiveHost = No;
            export Targeting = No;
            export number = head.number;
            export person = head.person;
            require head.case = Nominative;
            require head.RelativeUse = No;
            require head.ExistentialUse = No;
            export CaseUse = Nominative;
                }
        policy DummyPronounLicence { require head.ExistentialUse = Yes; }
        instance NominativePronoun<Result, Properties>: [
            (NounPhrase, OrdinaryNominativePronoun),
            (DummyPronoun, DummyPronounLicence),
        ] { use Properties; }
        instance CasePhrase<Result, Head: head, Properties>: [
            (NominativePhrase, NounPhrase, NominativeCase),
            (AccusativePhrase, NounPhrase, AccusativeCase),
        ] {
            require head.BarePrepositionUse = No;
            use Properties;
        }

        construction AccusativePronoun: NounPhrase {
            export NominalDependentPresent = No;
            export LocativeUse = No;
            export PartitiveCountability = Unrestricted;
            export Definiteness = Unspecified;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export BarePrepositionUse = No;
            export SubjectStructure = Phrase;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: lexical(Pronoun)];
            export number = head.number;
            export person = head.person;
            require head.case = Accusative;
            require head.ExistentialUse = No;
            export CaseUse = Accusative;
        }

        construction PrepositionPhrase: PrepositionPhrase {
            export PartitiveCountability = complement.PartitiveCountability;
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: AccusativePhrase];
            require noun_phrase_complement(head.PrepositionComplement) = Yes;
            export ObliqueNumber = oblique_number(head.ObliqueMarker, complement.number);
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export SelectedPrepositionUse = selected_preposition_complement(head.SelectedPrepositionUse,
                complement.SelectedPrepositionUse);
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = whole_object_oblique_source(
                head.ObliqueMarker, complement.WholeObjectSourcePresent);
        }

        construction BareNominalPreposition: PrepositionPhrase {
            export PartitiveCountability = complement.PartitiveCountability;
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: NounPhrase];
            require bare_nominal_complement(head.BareNominalComplement,
                complement.BarePrepositionUse) = Yes;
            require noun_phrase_complement(head.PrepositionComplement) = Yes;
            export ObliqueNumber = oblique_number(head.ObliqueMarker, complement.number);
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export SelectedPrepositionUse = selected_preposition_complement(head.SelectedPrepositionUse,
                complement.SelectedPrepositionUse);
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
        }

        // CGEL Ch. 7 §5.1, pp. 636–637: a predicative NP Complement
        // differs from an Object; adjunct use excludes adjective Complements.
        // Enter selects this PP by the project ruling of 2026-10-06.
        // Final Adjunct position follows the orchestrator ruling of 2026-10-07.
        // The nominal's declared use licenses that position; selected and
        // preposed uses survive independently (CGEL Ch. 4 §5.3, p. 263).
        construction PredicativeComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export PredicativeAdjunctUse = complement.PredicativeAdjunctUse;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = PreposedOrPredicative;
            export SelectedPrepositionUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            form [head: lexical(Preposition), " ", complement: PredicativeComplement];
            require predicative_noun_phrase_complement(head.PrepositionComplement) = Yes;
            require complement.PredicativeKind = Nominal;
            require licence_preposed_adjunct(head.PrepositionFunctionLicence) = Yes;
            require licence_predicative_complement(head.PrepositionFunctionLicence) = Yes;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // CGEL Ch. 6 §7.3.1, p. 587, [42i,ii,iv]: restrictive focusing
        // Adverbs can modify NPs, PPs and AdvPs. NP distribution follows FO1–FO3
        // in english-v3-focus-only-object (orchestrator ruling, 2026-10-10).
        schema FocusedPhrase {
            form [modifier: lexical(Adverb), " ", head: node];
            require modifier.FocusingModifierUse = Restrictive;
            require head.FocusedUse = No;
        }
        policy FocusedPrepositionPermissions {
            export ClauseComplementBoundary = head.ClauseComplementBoundary;
            require adjunct_complement_function(head.ComplementFunctionRestriction,
                head.PredicativeAdjunctUse) = Yes;
            require licence_adjunct(head.PrepositionFunctionLicence) = Yes;
            require head.SelectedPrepositionUse = No;
            export PrepositionFunctionLicence = Adjunct;
            export FocusedUse = Yes;
            export ClauseInitialAdjunct = No;
            export SelectedPrepositionUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export LocativeUse = No;
            export NominalComplementMarker = None;
            export InternalisedComplementMarker = No;
        }
        policy FocusedAdverbPermissions {
            export FocusedUse = Yes;
            require head.VPFinalAdjunct = Yes;
            export ClauseInitialAdjunct = No;
        }
        policy FocusedNounPhrasePermissions {
            export FocusedUse = Yes;
        }
        policy FocusedPurposePermissions {
            export FocusedUse = Yes;
        }
        // Project analysis: focusing extends to an attested purpose-Clause
        // constituent; the PP/AdvP citation above does not establish this case.
        instance FocusedPhrase<Result, Head: head, Properties>: [
            (NounPhrase, Self, FocusedNounPhrasePermissions),
            (PurposeInfinitival, Self, FocusedPurposePermissions),
            (PrepositionPhrase, Self, FocusedPrepositionPermissions),
            (AdverbPhrase, Self, FocusedAdverbPermissions),
        ] {
            use Properties;
        }

        construction IntransitivePreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition)];
            require intransitive_preposition_complement(head.PrepositionComplement) = Yes;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // CGEL Ch. 7 §3.2, p. 626: preposition + adjective idioms.
        construction AdjectiveComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = complement.InterveningAdjunctLicence;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            form [head: lexical(Preposition), " ", complement: lexical(Adjective)];
            require head.AdjectiveComplementClass = Manner;
            agree head.AdjectiveComplementClass = complement.AdjectiveComplementClass;
            require complement.framing = Unframed;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // CGEL Ch. 14 §10(b), p. 1267: reducible Clause Complements.
        // The lexical permission for an unrealized infinitival Complement
        // and the clause-final distribution are declared project decisions.
        // VW8 (2026-10-10): a native participial state uses its bare or
        // intransitive frame, never an unsupplied selected Complement.
        construction VerblessClause: VerblessClause {
            head predicate;
            form [predicate: lexical(Adjective)]
                require predicate.AdjectiveOrigin = Ordinary
                require verbless_ordinary_frame_use(predicate.framing,
                    predicate.VerblessComplementOmissionUse) = Yes;
            form [predicate: lexical(Adjective)]
                require predicate.AdjectiveOrigin = Participial
                require predicate.framing = Unframed;
            form [predicate: lexical(Adjective)]
                require predicate.AdjectiveOrigin = Participial
                require predicate.frame = Intransitive;
            form [predicate: lexical(Adjective)]
                require predicate.AdjectiveOrigin = Ordinary
                require predicate.frame = Intransitive
                require predicate.VerblessComplementOmissionUse = No;
            require predicate.VerblessPredicateUse = Yes;
            export VerblessClauseComplement = Yes;
        }

        // CGEL Ch. 14 §10(a), p. 1267: with takes subject–predicate
        // clause Complements, including nonfinite and verbless forms.
        // The participial/adjectival restriction is project ruling PW2/PW6
        // (english-v3-play-with-absolute, 2026-10-10).
        construction SubjectAdjectiveVerblessClause: VerblessClause {
            head predicate;
            form [subject: AccusativePhrase, " ", predicate: AdjectivePhrase];
            export VerblessClauseComplement = OvertSubject;
            require predicate.AdjectiveAttributiveOnlyUse = No;
        }

        construction SubjectParticipialClause: VerblessClause {
            head predicate;
            form [subject: AccusativePhrase, " ", predicate: SecondaryVerbPhrase];
            require predicate.form = PastParticiple;
            require predicate.ParticipialUse = BarePassive;
            export VerblessClauseComplement = OvertSubject;
        }

        construction VerblessClauseComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export ClauseComplementBoundary = Verbless;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: VerblessClause];
            require head.VerblessClauseComplement = Yes;
            require complement.VerblessClauseComplement = Yes;
            require licence_adjunct(head.PrepositionFunctionLicence) = Yes;
            export PrepositionPhraseComplementMarker = None;
            export SourceComplementUse = No;
            export SourceMarker = No;
            export ClauseInitialAdjunct = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export PrepositionFunctionLicence = Adjunct;
        }

        // CGEL Ch. 13 §5.5, pp. 1151–1152: compound prepositions take
        // content clauses, including modal preterites and irrealis.
        // Their Oracle function and irrealis licences are lexical declarations.
        construction ClauseComplementPreposition: PrepositionPhrase {
            export ConnectiveAdjunctUse = No;
            export ClauseComplementBoundary = Finite;
            export NominalExceptionComplementUse = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            form [head: lexical(Preposition), " ", complement: FiniteClause]
                require finite_clause_complement(head.FiniteClauseComplement) = Yes;
            // Orchestrator ruling R12f (2026-10-09), licensing detail.
            form [head: lexical(Preposition), " ", complement: FiniteGappedClause]
                require head.FiniteObjectGapComplement = Yes;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // CGEL Ch. 13 §5.5, p. 1152: irrealis content-clause Complement.
        construction IrrealisClauseComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export ClauseComplementBoundary = Finite;
            export NominalExceptionComplementUse = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            form [head: lexical(Preposition), " ", complement: IrrealisClause];
            require head.IrrealisClauseComplement = Yes;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // CGEL Ch. 7 §5.1(d), p. 640: restricted AdvP Complements
        // include duration use. The lexical licence and its application to
        // the Oracle equative duration are declared project analyses.
        construction AdverbComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = Adjunct;
            form [head: lexical(Preposition), " ", complement: AdverbPhrase];
            require head.AdverbComplementClass = Duration;
            require complement.DurationUse = Yes;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            export InternalisedComplementMarker = No;
            export LocativeUse = No;
            // PD1–PD2 (orchestrator, preposed-duration-for, 2026-10-10):
            // retain the head's position licence within Adjunct function.
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // CGEL Ch. 14 §8.3, p. 1262: gerund-participials freely complement
        // prepositions. Complement-specific function restriction is project policy,
        // declared by the lexicon, independently of Complement availability.
        construction GerundComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            form [head: lexical(Preposition), " ", complement: SecondaryVerbPhrase];
            require head.GerundClauseComplement = Yes;
            require complement.form = GerundParticiple;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        construction PredicativePreposition: PredicativeComplement {
            head phrase;
            export PredicativeAdjunctUse = No;
            form [phrase: PrepositionPhrase];
            require licence_predicative_complement(phrase.PrepositionFunctionLicence) = Yes;
            require phrase.ComplementFunctionRestriction = None;
            require phrase.LocativeUse = No;
            export PredicativeKind = Prepositional;
        }
        // CGEL Ch. 7 §2.4, pp. 616–617: the selected PP is a constituent;
        // omission removes its head together with its internal Complement.
        construction CompoundPrepositionPhrase: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export ClauseInitialAdjunct = No;
            form [head: lexical(Preposition), " ", complement: PrepositionPhrase];
            require selected_compound_complement(head.CompoundComplementMarker,
                complement.NominalComplementMarker) = Yes;
            require licence_compound(head.PrepositionFunctionLicence) = Yes;
            require compound_complement_function(complement.ComplementFunctionRestriction) = Yes;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // Project analysis: the numeric nominal exception retains its
        // cardinal Complement inside the selected PP, without licensing a
        // free duration, purpose or nominal-modifier use.
        construction QuantitativeExceptionPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: Cardinal];
            require head.NominalExceptionComplementUse = Yes;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = Yes;
            export PrepositionFunctionLicence = Compound;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export NominalComplementMarker = None;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export ClauseInitialAdjunct = No;
            export PrepositionPhraseComplementMarker = None;
            export SourceComplementUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export SourceMarker = No;
        }
        construction MatrixComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: PrepositionPhrase];
            require head.MatrixComplementGovernor = Yes;
            require complement.MatrixComplementRequirement = None;
            export MatrixComplementRequirement = matrix_pp_requirement(
                complement.InternalisedComplementMarker, complement.NominalExceptionComplementUse);
            export NominalExceptionComplementUse = No;
            export PrepositionFunctionLicence = Adjunct;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export NominalComplementMarker = None;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export ClauseInitialAdjunct = No;
            export PrepositionPhraseComplementMarker = None;
            export SourceComplementUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export SourceMarker = No;
        }

        // CGEL Ch. 7 §2.4, p. 617 permits PP Complements; Ch. 8 §4.3,
        // p. 687 describes locative PP Complements to a source preposition.
        // The supported source-head eligibility is a declared project decision.
        construction PrepositionComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: PrepositionPhrase];
            require preposition_phrase_complement(head.PrepositionComplement) = Yes;
            require complement.MatrixComplementRequirement = None;
            require complement.ComplementFunctionRestriction = None;
            require licensed_preposition_phrase_complement(head.PrepositionPhraseComplementLicence,
                complement.SourceComplementUse, complement.PrepositionPhraseComplementMarker) = Yes;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }


        construction SourceComplement: SourceComplement {
            form [phrase: PrepositionPhrase];
            require phrase.SourceMarker = Yes;
            require licence_verb_complement(phrase.PrepositionFunctionLicence) = Yes;
            require phrase.ComplementFunctionRestriction = None;
        }

        construction LocativeComplement: LocativeComplement {
            form [phrase: PrepositionPhrase]
                require phrase.SelectedPrepositionUse = No
                require licence_predicative_complement(phrase.PrepositionFunctionLicence) = Yes
                require phrase.ComplementFunctionRestriction = None;
            // AE2: NP function follows its head's declared locative use.
            form [phrase: NounPhrase];
            require phrase.LocativeUse = Yes;
        }

        construction MannerComplement: MannerComplement {
            form [phrase: AccusativePhrase];
            require phrase.SelectedPrepositionUse = Yes;
        }

        construction AdjectivalComplement: PredicativeComplement {
            head phrase;
            form [phrase: AdjectivePhrase];
            export PredicativeKind = Adjectival;
            require phrase.AdjectiveAttributiveOnlyUse = No;
        }

        // CGEL pp. 262, 1265: optional depictives are adjuncts.
        construction AdjectivalDepictive: DepictivePhrase {
            head phrase;
            form [phrase: AdjectivePhrase];
            export DepictiveKind = Adjectival;
            require phrase.AdjectiveAttributiveOnlyUse = No;
        }

        construction ParticipialDepictive: DepictivePhrase {
            head phrase;
            form [phrase: SecondaryVerbPhrase];
            require phrase.form = GerundParticiple;
            require phrase.ParticipialUse = Ordinary;
            export DepictiveKind = Participial;
        }

        construction NominalComplement: PredicativeComplement {
            head phrase;
            form [phrase: AccusativePhrase];
            export PredicativeKind = Nominal;
        }
        instance BareAuxiliaryPredicate<Result, Complement: complement, Properties>: [
            (FinitePredicate, BareComplement, FiniteAuxiliaryAgreement),
            (SecondaryVerbPhrase, BareComplement, SecondaryAuxiliaryProperties),
            (ScalarLocationPredicate, BareScalarLocationComplement, FiniteAuxiliaryAgreement),
        ] {
            use Properties;
        }
        instance ParticipialAuxiliaryPredicate<Result, Complement: complement, Properties>: [
            (FinitePredicate, ParticipialComplement, FiniteAuxiliaryAgreement),
            (FiniteManaPassivePredicate, ManaPassiveComplement, FiniteAuxiliaryAgreement),
            (IrrealisPredicate, ParticipialComplement, IrrealisHeadAgreement),
            (SecondaryVerbPhrase, ParticipialComplement, SecondaryAuxiliaryProperties),
        ] {
            use Properties;
        }
        instance PerfectAuxiliaryPredicate<Result, Complement: complement, Properties>: [
            (FinitePredicate, PerfectComplement, FiniteAuxiliaryAgreement),
            (SecondaryVerbPhrase, PerfectComplement, SecondaryAuxiliaryProperties),
        ] {
            use Properties;
        }

        table selected_object_marker(frame, KeywordMarker) -> Selection {
            (ObjectToObject, To) => Yes, (ObjectIntoObject, Into) => Yes,
            (ObjectOntoObject, Onto) => Yes, (ObjectForObject, For) => Yes,
            (ObjectAmongComplement, Among) => Yes,
            (SourceFreeObjectToObject, To) => Yes,
        }
        table selected_object_marker_with_object(frame, KeywordMarker, SourceModifierPresent) -> Selection {
            (ObjectToObject, To, No) => Yes, (ObjectToObject, To, Yes) => Yes,
            (ObjectIntoObject, Into, No) => Yes, (ObjectIntoObject, Into, Yes) => Yes,
            (ObjectOntoObject, Onto, No) => Yes, (ObjectOntoObject, Onto, Yes) => Yes,
            (ObjectForObject, For, No) => Yes, (ObjectForObject, For, Yes) => Yes,
            (SourceFreeObjectToObject, To, No) => Yes,
        }
        // CGEL Ch15 §4.3 pp1341–1343: parallel NP+PP tails under one selected verb.
        construction ObjectPrepositionTail: SelectedComplementTail {
            head none;
            form [object: AccusativePhrase, " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require noun_phrase_complement(marker.PrepositionComplement) = Yes;
            require licence_verb_complement(marker.PrepositionFunctionLicence) = Yes;
            segment Predicate;
            export HeadCoordination = No;
        }
        construction ObjectLocativeTail: SelectedComplementTail {
            head none;
            form [object: AccusativePhrase, " ", complement: LocativeComplement];
            segment Predicate;
            export HeadCoordination = No;
        }
        // GC7: each coordinate selects its own Manner Complement.
        construction ObjectLocativeMannerTail: SelectedComplementTail {
            head none;
            form [object: AccusativePhrase, " ", complement: LocativeComplement,
                " ", marker: lexical(Preposition), " ", manner: MannerComplement];
            require noun_phrase_complement(marker.PrepositionComplement) = Yes;
            require licence_verb_complement(marker.PrepositionFunctionLicence) = Yes;
            segment Predicate;
            export HeadCoordination = No;
        }
        schema SelectedComplementClustersPredicate {
            form [head: lexical(Verb), " ", tail: SelectedComplementTail];
            require tail.HeadCoordination = Yes;
            discharge_segments head, tail;
        }
        instance SelectedComplementClustersPredicate<Result, Properties>: [
            (FinitePredicate, FiniteHeadAgreement),
            (SecondaryVerbPhrase, OrdinarySecondaryHead),
        ] {
            use Properties;
        }
        // The depictive precedes the selected predicative PP (Vesuva).
        schema DepictiveSelectedPredicativePredicate {
            form [head: lexical(Verb), " ", modifier: DepictivePhrase, " ",
                complement: PrepositionPhrase];
            require head.frame = NominalPredicativePreposition;
            require complement.ComplementFunctionRestriction = PreposedOrPredicative;
            require licence_predicative_complement(complement.PrepositionFunctionLicence) = Yes;
        }
        instance DepictiveSelectedPredicativePredicate<Result, Properties>: [
            (FinitePredicate, FiniteHeadAgreement),
            (SecondaryVerbPhrase, OrdinarySecondaryHead),
        ] {
            use Properties;
        }

        schema SelectedSourceDepictivePredicate {
            form [head: lexical(Verb), complements: selected_frame(head, Predicate)];
            require head.frame = SourceFreeObjectSourceToObject;
        }
        instance SelectedSourceDepictivePredicate<Result, Properties>: [
            (FiniteDepictiveHost, FiniteHeadAgreement),
            (SecondaryDepictiveHost, OrdinarySecondaryHead),
        ] {
            use Properties;
        }

        schema SelectedObjectPrepositionPredicate {
            form [head: lexical(Verb), " ", object: AccusativePhrase, " ",
                marker: lexical(Preposition), " ", complement: AccusativePhrase];
            require selected_object_marker_with_object(head.frame, marker.KeywordMarker, object.SourceModifierPresent) = Yes;
            require licence_verb_complement(marker.PrepositionFunctionLicence) = Yes;
        }
        instance SelectedObjectPrepositionPredicate<Result, Properties>: [
            (FiniteDepictiveHost, FiniteHeadAgreement),
            (SecondaryDepictiveHost, OrdinarySecondaryHead),
        ] {
            use Properties;
        }
        // Orchestrator ruling R12f: this site precedes the selected marker,
        // never an Object NP. The Adjunct's lexical head declares its licence.
        construction InterveningPrepositionAdjunct: InterveningPrepositionAdjunct {
            form [" ", head: PrepositionPhrase];
            require head.InterveningAdjunctLicence = Yes;
            require licence_adjunct(head.PrepositionFunctionLicence) = Yes;
        }
        schema SelectedGapPrepositionPredicate {
            form [head: lexical(Verb), modifier: optional(InterveningPrepositionAdjunct), " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require selected_object_marker(head.frame, marker.KeywordMarker) = Yes;
            require licence_verb_complement(marker.PrepositionFunctionLicence) = Yes;
        }
        policy PlainGapHead {
            require head.form = Plain;
        }
        policy BarePassiveHead {
            require head.form = PastParticiple;
            export form = PastParticiple;
            export ParticipialUse = BarePassive;
            export InternalisedComplementPresent = No;
        }
        instance SelectedGapPrepositionPredicate<Result, Properties>: [
            (FiniteObjectGap, FiniteGapHeadAgreement),
            (BareObjectGap, PlainGapHead),
            (SecondaryVerbPhrase, BarePassiveHead),
        ] {
            use Properties;
        }
        construction BareNominalPrepositionPassive: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", marker: lexical(Preposition), " ",
                complement: NounPhrase];
            require selected_object_marker(head.frame, marker.KeywordMarker) = Yes;
            require licence_verb_complement(marker.PrepositionFunctionLicence) = Yes;
            require bare_nominal_complement(marker.BareNominalComplement,
                complement.BarePrepositionUse) = Yes;
            use BarePassiveHead;
        }

        schema SelectedGapLocativePredicate {
            form [head: lexical(Verb), " ", complement: LocativeComplement];
            require head.frame = ObjectLocative;
        }
        instance SelectedGapLocativePredicate<Result, Properties>: [
            (FiniteObjectGap, FiniteGapHeadAgreement),
            (BareObjectGap, PlainGapHead),
            (SecondaryVerbPhrase, BarePassiveHead),
        ] {
            use Properties;
        }
        construction SelectedExtentPassive: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", complement: ScalarExtentComplement];
            require head.frame = ObjectExtent;
            use BarePassiveHead;
        }
        construction SelectedFramePassive: SecondaryVerbPhrase {
            form [head: lexical(Verb), complements: selected_frame(head, Passive)];
            use BarePassiveHead;
        }
        construction RetainedObjectPassive: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", object: AccusativePhrase];
            require head.frame = Ditransitive;
            use BarePassiveHead;
        }
        // CGEL pp. 1264–1266: gerund-participial and bare-passive noun modifiers.
        table participial_postmodifier(form, ParticipialUse) -> Selection {
            (GerundParticiple, Ordinary) => Yes,
            (PastParticiple, BarePassive) => Yes,
        }
        construction ParticipialPostmodifiedNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: SecondaryVerbPhrase];
            require participial_postmodifier(modifier.form, modifier.ParticipialUse) = Yes;
            export NominalDependentPresent = Yes;
            export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
        }

        // CGEL pp. 698 and 671: licensed temporal and manner NPs as adjuncts.
        // CGEL p. 698: definite temporal NPs require another dependent.
        table nominal_adjunct_license(NominalAdjunctDependent, NominalAdjunctClass,
            NominalDependentPresent) -> Selection {
            (Demonstrative, Temporal, No) => Yes, (Demonstrative, Temporal, Yes) => Yes,
            (Demonstrative, Manner, No) => Yes, (Demonstrative, Manner, Yes) => Yes,
            (Temporal, Temporal, No) => Yes, (Temporal, Temporal, Yes) => Yes,
            (Definite, Temporal, Yes) => Yes,
            (DeicticTemporal, Temporal, No) => Yes, (DeicticTemporal, Temporal, Yes) => Yes,
        }
        // Definite temporal NPs are preposed only: project ruling R11d.
        table postposed_nominal_adjunct(NominalAdjunctDependent) -> Selection {
            (Demonstrative) => Yes, (Temporal) => Yes, (DeicticTemporal) => Yes,
        }
        table posthead_dependent_presence(present) -> NominalDependentPresent {
            (No) => No, (Yes) => Yes,
        }
        table nominal_adjunct_concord(NominalAdjunctClass, NominalAdjunctClass)
            -> NominalAdjunctClass {
            (None, None) => None, (None, Temporal) => None, (None, Manner) => None,
            (Temporal, None) => None, (Temporal, Temporal) => Temporal,
            (Temporal, Manner) => None, (Manner, None) => None,
            (Manner, Temporal) => None, (Manner, Manner) => Manner,
        }
        construction NominalAdjunctPhrase: NominalAdjunctPhrase {
            export NominalAdjunctDependent = determiner.NominalAdjunctDependent;
            form [determiner: lexical(Determinative), " ", head: Nominal]
                require determiner_license(determiner.DeterminerUse, head.number, head.countability,
                    head.DeterminerRequirement, determiner.NominalAdjunctDependent) = Yes;
            // Deictic adjective dependent: orchestrator rulings R14a/R14e (2026-10-09).
            form [determiner: lexical(Adjective), " ", head: Nominal]
                require determiner.NominalAdjunctDependent = DeicticTemporal
                require head.countability = Count;
            require head.Targeting = No;
            require head.number = Singular;
            require nominal_adjunct_license(determiner.NominalAdjunctDependent,
                head.NominalAdjunctClass, head.NominalDependentPresent) = Yes;
        }
        instance NominalAdjunctPredicate<Result, Head: head>: [
            (ExistentialPredicate, Self),
            (FinitePredicate, Self),
        ] {
            require nonconnective_trailing_adjunct_host(head.ClauseComplementBoundary) = Yes;
        }
        instance NominalAdjunctPredicate<Result, Head: head, Properties>: [
            (SecondaryVerbPhrase, Self, SecondaryProjection),
        ] {
            require nonconnective_trailing_adjunct_host(head.ClauseComplementBoundary) = Yes;
            use Properties;
        }
        instance NominalAdjunctPredicate<Result, Head: head>: [
            (FiniteObjectGap, Self),
            (BareObjectGap, Self),
        ] {
            require nonconnective_trailing_adjunct_host(head.ClauseComplementBoundary) = Yes;
        }

        // CGEL pp. 409–410: restricted bare count NPs in selected PP frames.
        construction BareIntervalNominal: BareTemporalNominal {
            form [head: lexical(Noun)];
            require head.NominalBareClass = Interval;
            require head.number = Singular;
            export NominalBareClass = Interval;
        }
        construction OrdinaryBoundaryComplement: BoundaryComplement {
            form [phrase: AccusativePhrase];
        }
        construction BareBoundaryComplement: BoundaryComplement {
            form [phrase: BareTemporalNominal];
            require phrase.NominalBareClass = Interval;
        }
        construction BareBoundaryNominal: BareTemporalNominal {
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ",
                complement: BoundaryComplement];
            require head.NominalBareClass = Boundary;
            require head.number = Singular;
            require marker.NominalComplementMarker = Of;
            require licence_noun_complement(marker.PrepositionFunctionLicence) = Yes;
            export NominalBareClass = Boundary;
        }
        construction BareTemporalPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: BareTemporalNominal];
            agree head.NominalBareClass = complement.NominalBareClass;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // The declared modifier licenses this Oracle-register bare singular NP.
        construction BareStatusNounPhrase: NounPhrase {
            export BarePrepositionUse = No;
            form [modifier: lexical(Adjective), " ", head: Nominal];
            require modifier.BareSingularUse = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.Targeting = No;
            require head.DeterminerRequirement = No;
            use NounPhraseHeadAgreement;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
        }

        construction BareParticipialStatusNounPhrase: NounPhrase {
            export BarePrepositionUse = No;
            form [modifier: lexical(Verb), " ", head: Nominal];
            require verbal_premodifier(modifier.form, modifier.frame,
                modifier.PastParticipialPremodifier) = Yes;
            require modifier.BareSingularUse = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.Targeting = No;
            require head.DeterminerRequirement = No;
            use NounPhraseHeadAgreement;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
        }

        construction PronounSubjectRelative: SubjectRelativeClause {
            head predicate;
            form [marker: lexical(Pronoun), clitic_separator(predicate.CliticHost),
                predicate: FinitePredicate];
            require marker.RelativeUse = Yes;
            require marker.case = Nominative;
            agree marker.person = predicate.person;
            agree marker.number = predicate.number;
        }

        construction QuantifiedSubjectRelative: SubjectRelativeClause {
            head predicate;
            form [marker: lexical(Subordinator), " ", quantifier: lexical(Determinative),
                " ", predicate: FinitePredicate];
            require marker.RelativeSubordinator = Yes;
            require quantifier.QuantificationalAdjunctUse = Yes;
            require predicate.CliticHost = Free;
            require predicate.number = Plural;
            require predicate.person = Third;
        }

        construction CountedFrequency: FrequencyPhrase {
            form [quantity: Cardinal, " ", head: lexical(Noun)];
            require head.FrequencyUnit = Yes;
            require head.countability = Count;
            agree quantity.number = head.number;
        }

        instance FrequencyPredicate<Result, Head: head, Modifier: modifier>: [
            (FinitePredicate, Self, FrequencyPhrase),
        ] {}
        instance FrequencyPredicate<Result, Head: head, Modifier: modifier, Properties>: [
            (SecondaryVerbPhrase, Self, FrequencyPhrase, SecondaryProjection),
        ] {
            use Properties;
        }

        instance PrepositionPredicate<Result, Head: head, Modifier: modifier>: [
            (ExistentialPredicate, Self, PrepositionPhrase),
            (FinitePredicate, Self, PrepositionPhrase),
        ] {}
        instance PrepositionPredicate<Result, Head: head, Modifier: modifier, Properties>: [
            (SecondaryVerbPhrase, Self, PrepositionPhrase, SecondaryPrepositionProjection),
        ] {
            use Properties;
        }
        instance PrepositionPredicate<Result, Head: head, Modifier: modifier>: [
            (FiniteObjectGap, Self, PrepositionPhrase),
            (BareObjectGap, Self, PrepositionPhrase),
        ] {}

        instance PurposePredicate<Result, Head: head>: [
            (FinitePredicate, Self),
            (SecondaryVerbPhrase, Self),
            (FiniteManaPassivePredicate, Self),
            (ManaPassivePredicate, Self),
            (FiniteObjectGap, Self),
            (BareObjectGap, Self),
        ] {}

        construction BarePredicate: BarePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = Plain;
        }

        construction ProgressiveComplement: ParticipialPredicate {
            export CentralAdjunctPosition = PostAuxiliary;
            form [head: SecondaryVerbPhrase];
            require head.form = GerundParticiple;
        }

        // CGEL p. 1327: unlike AdjP and progressive complements coordinated under be.
        // The shared phrase describes their constituent structure; here it is a complement.
        construction MixedCopularProgressiveComplement: ParticipialPredicate {
            head phrase;
            export CentralAdjunctPosition = PostAuxiliary;
            form [phrase: DepictivePhrase];
            require phrase.DepictiveKind = Mixed;
            export PurposeAdjunctLicence = No;
        }

        construction PassiveComplement: ParticipialPredicate {
            export CentralAdjunctPosition = PostCopula;
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            require head.ParticipialUse = BarePassive;
        }

        construction PerfectComplement: PastParticiplePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            require head.ParticipialUse = Ordinary;
        }

        construction PassivePredicate: SecondaryVerbPhrase {
            form [head: lexical(Verb)];
            require head.form = PastParticiple;
            require head.frame = Transitive;
            export form = PastParticiple;
            export ParticipialSupplementUse = No;
            export ParticipialUse = BarePassive;
            export InternalisedComplementPresent = No;
        }

        construction SubjectRelativeClause: SubjectRelativeClause {
            head predicate;
            form [marker: lexical(Subordinator), clitic_separator(predicate.CliticHost),
                predicate: FinitePredicate];
            require marker.RelativeSubordinator = Yes;
            require relative_clitic(predicate.CliticHost) = Yes;
            require predicate.person = Third;
        }

        construction SubjectRelativeNominal: Nominal {
            require relative_number_chain(head.SubjectRelativeNumber, relative.number) = Yes;
            export SubjectRelativeNumber = deferred_relative_number(head.NominalAgreementNumber, relative.number);
            export BareGenitiveHost = No;
            form [head: Nominal, " ", relative: SubjectRelativeClause];
            export NominalDependentPresent = Yes;
            require nominal_relative_agreement(head.NominalAgreementNumber, head.number, relative.number) = Yes;
        }

        construction FiniteObjectGap: FiniteObjectGap {
            require head.PredicateFrameUse = Yes;
            form [head: lexical(Verb)];
            use FiniteHeadAgreement;
            require head.frame = Transitive;
        }

        construction BareObjectGap: BareObjectGap {
            form [head: lexical(Verb)];
            require head.form = Plain;
            require head.frame = Transitive;
        }

        construction PastParticipleObjectGap: PastParticipleObjectGap {
            form [head: lexical(Verb)];
            require head.PredicateFrameUse = Yes;
            require head.form = PastParticiple;
            require head.frame = Transitive;
        }

        construction GerundParticipleObjectGap: GerundParticipleObjectGap {
            form [head: lexical(Verb)];
            require head.PredicateFrameUse = Yes;
            require head.form = GerundParticiple;
            require head.frame = Transitive;
        }

        construction AuxiliaryObjectGap: FiniteObjectGap {
            export ComparativeObjectGapLicence = complement.ComparativeObjectGapLicence;
            form [head: lexical(Verb), " ", complement: BareObjectGap]
                require head.frame = BareAuxiliary;
            // The perfect selects an Object gap in a transitive past-participle
            // Complement, strong or clitic.
            form [head: lexical(Verb), " ", complement: PastParticipleObjectGap]
                require head.frame = PerfectAuxiliary;
            // PG1–PG3: the progressive selects a transitive gerund-participle
            // Object gap through the existing strong and clitic auxiliary route.
            form [head: lexical(Verb), " ", complement: GerundParticipleObjectGap]
                require head.frame = ParticipialAuxiliary;
            use FiniteHeadAgreement;
        }

        construction ObjectRelativeClause: ObjectRelativeClause {
            head predicate;
            form [marker: lexical(Subordinator), " ", subject: NominativePhrase,
                clitic_separator(predicate.CliticHost), predicate: FiniteObjectGap];
            require marker.RelativeSubordinator = Yes;
            require finite_subject_number(subject.number, predicate.number, subject.DeterminerAgreementNumber) = Yes;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        construction ZeroObjectRelativeClause: ObjectRelativeClause {
            head predicate;
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: FiniteObjectGap];
            require finite_subject_number(subject.number, predicate.number, subject.DeterminerAgreementNumber) = Yes;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        // R12f: the Subject and finite Object-gap Predicate form the
        // comparative-clause Complement, without a relative marker.
        construction FiniteGappedClause: FiniteGappedClause {
            require predicate.ComparativeObjectGapLicence = Yes;
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: FiniteObjectGap];
            require finite_subject_number(subject.number, predicate.number, subject.DeterminerAgreementNumber) = Yes;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        // CGEL Ch. 12 §3.3(e), pp. 1045–1046: a bare relative can have
        // an Adjunct gap. The temporal head licence is declared by the noun.
        construction AdverbialGapRelativeClause: ObjectRelativeClause {
            head clause;
            form [clause: FiniteClause];
            export RelativeGapKind = TemporalAdjunct;
        }

        construction ObjectRelativeNominal: Nominal {
            require relative_head_licence(relative.RelativeGapKind,
                head.RelativisableAdjunctClass) = Yes;
            export BareGenitiveHost = No;
            form [head: Nominal, " ", relative: ObjectRelativeClause];
            export NominalDependentPresent = Yes;
        }

        construction OmittedPlain: Ellipsis {
            head none;
            form [];
            export form = Plain;
        }


        instance OvertComplement<Result, Predicate: predicate, Properties = NoComplementProperties>: [
            (BareComplement, BarePredicate, OvertComplementRealization),
            (BareScalarLocationComplement, BareScalarLocationPredicate, OvertComplementRealization),
            (ParticipialComplement, ParticipialPredicate),
            (PerfectComplement, PastParticiplePredicate),
            (ManaPassiveComplement, ManaPassivePredicate),
        ] {
            use Properties;
        }

        // DS1–DS2: an overt pro-form fills the stranded auxiliary's Complement.
        construction ProFormComplement: BareComplement {
            form [" ", head: lexical(Adverb)];
            require head.ProFormComplementUse = Yes;
            export AuxiliaryComplementRealization = ProForm;
            export PurposeAdjunctLicence = No;
            export MatrixComplementLicence = None;
            export SourceComplementSelection = No;
        }

        construction BareEllipsis: BareComplement {
            head omission;
            form [omission: Ellipsis];
            require omission.form = Plain;
            export AuxiliaryComplementRealization = Elided;
            export PurposeAdjunctLicence = No;
        }




        // EX1: a licensed Adverb modifies the numeral, not its enclosing NP.
        construction AdverbCardinalModifier: CardinalModifier {
            form [head: lexical(Adverb), " "];
            require head.CardinalModifierUse = Yes;
        }

        construction Cardinal: Cardinal {
            form [modifier: optional(CardinalModifier), head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
            export number = head.number;
            export CardinalComplementUse = cardinal_complement_use(head.numeral_kind);
        }

        construction LargeCount: Cardinal {
            form [modifier: optional(CardinalModifier), head: lexical(Numeral)];
            require head.numeral_kind = GroupedArabic;
            require head.numeral_size = Large;
            export number = head.number;
            export CardinalComplementUse = cardinal_complement_use(head.numeral_kind);
        }

        construction NumericQuantityConjunct: QuantityConjunct {
            export CardinalMeasurementUse = DeterminerRequired;
            form [head: lexical(Numeral)];
            require head.numeral_kind = Arabic;
            require head.numeral_size = Small;
            export number = head.number;
            export CardinalComplementUse = cardinal_complement_use(head.numeral_kind);
        }

        construction CardinalQuantityConjunct: QuantityConjunct {
            head value;
            cost 0;
            form [value: Cardinal];
        }

        policy VariableCountProperties {
            export number = Plural;
            export CardinalComplementUse = No;
        }
        policy ScalarVariableProperties {
            export UnitBearing = No;
            export MeasureKind = Scalar;
        }
        policy VariableSubjectProperties {
            export DeterminerAgreementNumber = Nominal;
            export number = Singular;
            export person = Third;
            export CaseUse = Nominative;
            export SubjectStructure = Phrase;
            export ScalarDenotation = head.ScalarDenotation;
        }
        schema ScalarVariable {
            form [head: lexical(Numeral)];
            require head.ScalarVariable = Yes;
        }
        instance ScalarVariable<Result, Properties>: [
            (Cardinal, VariableCountProperties),
            (UnsignedScalar, NoFeatures),
            (MeasurePhrase, ScalarVariableProperties),
            (NominativePhrase, VariableSubjectProperties),
        ] {
            use Properties;
        }

        construction QuantitativeAdjectiveComplement: QuantitativeAdjectiveComplement {
            form [head: lexical(Adjective)];
            require head.QuantitativeAdjectiveComplementUse = Yes;
            require head.framing = Unframed;
        }

        separator quantitative_modifier_separator(QuantitativeComplement) {
            (None) => "", (Cardinal) => "", (CardinalPrepositionPhrase) => "",
            (AdjectivePhrase) => " ",
            (ExceptionQuantity) => "",
        }
        separator arithmetic_modifier_separator(MeasureOperator) {
            (No) => "", (Yes) => " ",
        }
        schema QuantitativePrepositionPhrase {
            form [arithmetic_modifier_separator(head.MeasureOperator), head: lexical(Preposition), " ", complement: node,
                quantitative_modifier_separator(head.QuantitativeComplement)];
        }

        policy CardinalPrepositionComplement {
            export number = complement.number;
            require head.QuantitativeComplement = Cardinal;
        }

        // DQ3: the Complement licence comes from the numeral's declared kind.
        feature CardinalComplementUse { No, Yes } default No combine all;
        table cardinal_complement_use(numeral_kind) -> CardinalComplementUse {
            (Cardinal) => Yes, (Arabic) => Yes, (GroupedArabic) => Yes,
        }
        policy ArithmeticCardinalPrepositionComplement {
            require head.MeasureOperator = Yes;
            require complement.CardinalComplementUse = Yes;
        }

        policy QuantitativePrepositionComplement {
            export number = complement.number;
            require head.QuantitativeComplement = CardinalPrepositionPhrase;
        }

        policy QuantitativeDegreePrepositionComplement {
            export DeterminerUse = complement.DeterminerUse;
            export DeterminerAgreementNumber = complement.DeterminerAgreementNumber;
            require head.QuantitativeComplement = CardinalPrepositionPhrase;
        }

        // AB2–AB3: digit quantities retain their measurement use, whereas
        // spelled cardinals select count heads as in their bare use.
        table exception_determiner_use(CardinalMeasurementUse, number) -> DeterminerUse {
            (No, Singular) => SingularCount, (No, Plural) => PluralCount,
            (DeterminerRequired, Singular) => Singular,
            (DeterminerRequired, Plural) => PluralOrMass,
        }
        policy ExceptionQuantityComplement {
            require head.QuantitativeComplement = ExceptionQuantity;
            export DeterminerUse = exception_determiner_use(
                complement.CardinalMeasurementUse, complement.number);
            export DeterminerAgreementNumber = cardinal_agreement_number(complement.number);
        }
        policy ExceptionDegreeComplement {
            require head.QuantitativeComplement = ExceptionQuantity;
            require complement.DegreeUse = Head;
            export DeterminerUse = complement.DeterminerUse;
            export DeterminerAgreementNumber = complement.DeterminerAgreementNumber;
        }

        // UM1–UM2: the bound also takes a degree Determinative Phrase,
        // retaining its declared head selection and Agreement.
        policy DegreePrepositionComplement {
            require head.QuantitativeComplement = Cardinal;
            require complement.DegreeUse = Head;
            export DeterminerUse = complement.DeterminerUse;
            export DeterminerAgreementNumber = complement.DeterminerAgreementNumber;
        }

        policy ComparativePrepositionComplement {
            export number = complement.number;
            require head.ComparisonMarker = Ordering;
        }

        // NM3: frequency Complements belong to the same Comparative Quantity.
        policy CountedFrequencyPrepositionComplement {
            require head.ComparisonMarker = Ordering;
        }
        policy AdverbFrequencyPrepositionComplement {
            require head.ComparisonMarker = Ordering;
            require complement.FrequencyUse = Yes;
        }

        // CGEL Ch. 5 §11(e), p. 432: a bounding PP modifies a numeral.
        // Its adjectival Complement licence follows ticket rulings R9a–R9d.
        policy QuantitativeAdjectiveComplementLicence {
            require head.QuantitativeComplement = AdjectivePhrase;
        }

        instance QuantitativePrepositionPhrase<Result, Complement: complement, Properties>: [
            (CardinalPrepositionPhrase, Cardinal, CardinalPrepositionComplement),
            (ArithmeticModifier, QuantityConjunct, ArithmeticCardinalPrepositionComplement),
            (QuantitativePrepositionPhrase, CardinalPrepositionPhrase,
                QuantitativePrepositionComplement),
            (ComparativePrepositionPhrase, Cardinal, ComparativePrepositionComplement),
            (CountedFrequencyPrepositionPhrase, FrequencyPhrase, CountedFrequencyPrepositionComplement),
            (AdverbFrequencyPrepositionPhrase, AdverbPhrase, AdverbFrequencyPrepositionComplement),
            (CardinalModifier, QuantitativeAdjectiveComplement, QuantitativeAdjectiveComplementLicence),
            (DegreePrepositionPhrase, QuantitativeDeterminer, DegreePrepositionComplement),
            (QuantitativeDegreePrepositionPhrase, DegreePrepositionPhrase,
                QuantitativeDegreePrepositionComplement),
            (ExceptionPrepositionPhrase, QuantityConjunct, ExceptionQuantityComplement),
            (ExceptionDegreePrepositionPhrase, QuantitativeDeterminer, ExceptionDegreeComplement),
        ] {
            use Properties;
        }

        // CGEL Ch. 5 §7.6, p. 386: a numeral and comparative are Conjuncts.
        // §3.4, p. 353 n. 13: with a count noun, it selects a plural head.
        construction NumeralComparativeCoordination: ComparativeQuantity {
            head none;
            export FrequencyUse = No;
            export MeasureModifierUse = right.MeasureModifierUse;
            form [left: QuantityConjunct, " ", marker: lexical(Coordinator), " ",
                right: lexical(Determinative)];
            require marker.CoordinationKind = Alternative;
            require marker.NoncorrelativeCoordination = Yes;
            require right.ComparativeQuantityUse = Yes;
            export ComparativeQuantityStructure = Coordinated;
            export number = Plural;
        }

        // CGEL Ch. 5 §11(d), p. 432: the quantifier in the PP Complement
        // controls head selection, including singular more than one.
        construction ComparativeDeterminativePhrase: ComparativeQuantity {
            form [modifier: optional(lexical(Determinative), " "),
                head: lexical(Determinative), " ", complement: ComparativePrepositionPhrase]
                export number = complement.number
                export FrequencyUse = No;
            form [modifier: optional(lexical(Determinative), " "),
                head: lexical(Determinative), " ", complement: CountedFrequencyPrepositionPhrase]
                export number = Plural
                export FrequencyUse = Yes;
            form [modifier: optional(lexical(Determinative), " "),
                head: lexical(Determinative), " ", complement: AdverbFrequencyPrepositionPhrase]
                export number = Plural
                export FrequencyUse = Yes;
            require modifier.ComparativeDegreeModifierUse = Yes;
            require head.ComparativeQuantityUse = Yes;
            export ComparativeQuantityStructure = Headed;
        }

        // MH2: a declared Determinative may coordinate with a cardinal count.
        construction CardinalDeterminativeCoordination: QuantitativeDeterminer {
            head none;
            export DegreeUse = None;
            form [left: Cardinal, " ", coordinator: lexical(Coordinator), " ",
                right: lexical(Determinative)];
            require coordinator.CoordinationKind = Alternative;
            require coordinator.NoncorrelativeCoordination = Yes;
            require right.QuantitativeConjunctUse = Yes;
            export DeterminerUse = right.DeterminerUse;
            export DeterminerAgreementNumber = right.DeterminerAgreementNumber;
            export Definiteness = Nondefinite;
            export NominalAdjunctDependent = None;
        }

        construction ComparativeQuantityDeterminer: QuantitativeDeterminer {
            head value;
            export Definiteness = Nondefinite;
            cost 0;
            form [value: ComparativeQuantity];
            require value.FrequencyUse = No;
            export DeterminerUse = cardinal_determiner_use(value.number);
            export DeterminerAgreementNumber = cardinal_agreement_number(value.number);
        }

        construction CardinalDeterminer: QuantitativeDeterminer {
            head value;
            export Definiteness = Nondefinite;
            cost 0;
            form [value: Cardinal];
            export DeterminerUse = cardinal_determiner_use(value.number);
            export DeterminerAgreementNumber = cardinal_agreement_number(value.number);
        }

        construction PrepositionDeterminer: QuantitativeDeterminer {
            head value;
            export Definiteness = Nondefinite;
            cost 0;
            form [value: QuantitativePrepositionPhrase]
                export DeterminerUse = cardinal_determiner_use(value.number)
                export DeterminerAgreementNumber = cardinal_agreement_number(value.number);
            form [value: QuantitativeDegreePrepositionPhrase];
        }

        table cardinal_agreement_number(number) -> DeterminerAgreementNumber {
            (Singular) => Singular, (Plural) => Nominal,
        }
        table quantified_nominal_number(DeterminerAgreementNumber, NominalAgreementNumber, number) -> number {
            (Singular, Head, Singular) => Singular, (Singular, Head, Plural) => Singular,
            (Singular, Plural, Singular) => Singular, (Singular, Plural, Plural) => Singular,
            (Nominal, Head, Singular) => Singular, (Nominal, Head, Plural) => Plural,
            (Nominal, Plural, Singular) => Plural, (Nominal, Plural, Plural) => Plural,
            (_, SingularOrPlural, Singular) => Singular,
            (_, SingularOrPlural, Plural) => Plural,
        }
        table cardinal_determiner_use(number) -> DeterminerUse {
            (Singular) => SingularCount, (Plural) => PluralCount,
        }

        // CGEL Ch. 5 §11(c), p. 431: degree modification within a DP.
        construction DegreeDeterminativePhrase: QuantitativeDeterminer {
            export Definiteness = modifier.Definiteness;
            export NominalAdjunctDependent = None;
            // DQ1: the optional addition/subtraction PP is internal to the DP.
            form [modifier: lexical(Determinative), " ", head: lexical(Determinative),
                arithmetic_modifier: optional(ArithmeticModifier)];
            require modifier.DegreeUse = Modifier;
            require head.DegreeUse = Head;
            export DeterminerUse = head.DeterminerUse;
        }

        // CGEL Ch. 5 §11(f), p. 433: an exception PP depends on all.
        // AB1–AB3 license its quantity Complement and partitive-head use.
        construction ExceptionDeterminativePhrase: QuantitativeDeterminer {
            form [head: lexical(Determinative), " ", exception: ExceptionPrepositionPhrase];
            form [head: lexical(Determinative), " ", exception: ExceptionDegreePrepositionPhrase];
            require head.QuantitativeExceptionUse = Yes;
            export DegreeUse = None;
            export Definiteness = Nondefinite;
            export NominalAdjunctDependent = None;
            export DeterminerUse = exception.DeterminerUse;
            export DeterminerAgreementNumber = exception.DeterminerAgreementNumber;
        }

        // Scalar Denotation includes measured amounts. Quantification of a
        // head with declared Measure Position supplies that amount; ordinary
        // quantified objects retain their head's Scalar Denotation.
        table quantified_scalar_denotation(ScalarDenotation, MeasurePosition) -> ScalarDenotation {
            (No, None) => No, (No, Before) => Yes,
            (Yes, None) => Yes, (Yes, Before) => Yes,
        }

        // QS7: a number-neutral Nominal accepts mass and count Determiners
        // without expanding its lexical analysis or its grammatical Number.
        table quantified_determiner_license(DeterminerUse, number, countability,
            DeterminerRequirement, NominalAdjunctDependent, NominalAgreementNumber) -> Selection {
            // FH7 (orchestrator, 2026-10-10): noun-side definite selection.
            (_, Singular, Count, Yes, _, SingularOrPlural) => Yes,
            (_, Singular, Mass, Yes, _, SingularOrPlural) => Yes,
            (Unrestricted, Singular, Count, Definite, Definite, _) => Yes,
            (Unrestricted, Plural, Count, Definite, Definite, _) => Yes,
            (Unrestricted, Singular, Mass, Definite, Definite, _) => Yes,
            (SingularCount, Singular, Count, No, _, _) => Yes,
            (SingularCount, Singular, Count, Yes, _, _) => Yes,
            (Unrestricted, Singular, Count, No, _, _) => Yes,
            (Unrestricted, Singular, Count, Yes, _, _) => Yes,
            (Unrestricted, Plural, Count, No, _, _) => Yes,
            (Unrestricted, Plural, Count, Yes, _, _) => Yes,
            (Unrestricted, Singular, Mass, No, _, _) => Yes,
            (Unrestricted, Singular, Mass, Yes, _, _) => Yes,
            (PluralOrMass, Plural, Count, No, _, _) => Yes,
            (PluralOrMass, Plural, Count, Yes, _, _) => Yes,
            (PluralOrMass, Singular, Mass, No, _, _) => Yes,
            (PluralOrMass, Singular, Mass, Yes, _, _) => Yes,
            (PluralCount, Plural, Count, No, _, _) => Yes,
            (PluralCount, Plural, Count, Yes, _, _) => Yes,
            (Mass, Singular, Mass, No, _, _) => Yes,
            (Mass, Singular, Mass, Yes, _, _) => Yes,
            (Singular, Singular, Count, No, _, _) => Yes,
            (Singular, Singular, Count, Yes, _, _) => Yes,
            (Singular, Singular, Mass, No, _, _) => Yes,
            (Singular, Singular, Mass, Yes, _, _) => Yes,
        }
        table quantified_agreement(DeterminerAgreementNumber, NominalAgreementNumber)
            -> DeterminerAgreementNumber {
            (_, SingularOrPlural) => SingularOrPlural,
            (Nominal, _) => Nominal, (Singular, _) => Singular,
            (SingularOrPlural, _) => SingularOrPlural,
        }

        construction QuantifiedNounPhrase: NounPhrase {
            export PartitiveCountability = partitive_countability(head.countability);
            export Definiteness = quantity.Definiteness;
            export UnitBearing = nominal_unit_bearing(head.MeasurePosition);
            export DeterminerAgreementNumber = quantified_agreement(
                quantity.DeterminerAgreementNumber, head.NominalAgreementNumber);
            export BarePrepositionUse = No;
            form [quantity: QuantitativeDeterminer, " ", head: Nominal];
            require quantified_determiner_license(quantity.DeterminerUse, head.number, head.countability,
                head.DeterminerRequirement, quantity.NominalAdjunctDependent,
                head.NominalAgreementNumber) = Yes;
            require quantifying_determiner_requirement(head.DeterminerRequirement,
                head.MeasurePosition, head.countability) = Yes;
            export ScalarDenotation = quantified_scalar_denotation(head.ScalarDenotation, head.MeasurePosition);
            export SubjectStructure = Phrase;
            export number = quantified_nominal_number(quantity.DeterminerAgreementNumber,
                head.NominalAgreementNumber, head.number);
            export person = Third;
            export CaseUse = Common;
            require resolved_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number, quantity.DeterminerAgreementNumber) = Yes;
        }

        construction OrdinalPremodifier: AdjectivePhrase {
            export AdjectiveComplementClass = None;
            export AdjectiveAttributiveOnlyUse = No;
            export ImplicitComparativeUse = No;
            form [head: lexical(Numeral)];
            require head.numeral_kind = Ordinal;
            export AdjectiveStructure = Simple;
            export AdjectiveOrigin = Ordinary;
            export AdjectivePremodifierUse = Yes;
            export TargetingPremodifierUse = Yes;
        }

        construction GroupedScalarNumeral: MeasurePhrase {
            export UnitBearing = No;
            form [modifier: optional(CardinalModifier), head: lexical(Numeral)];
            require head.numeral_kind = GroupedArabic;
            require head.numeral_size = Large;
            export MeasureKind = Scalar;
        }

        construction SmallUnsignedScalar: UnsignedScalar {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Arabic;
            require head.numeral_size = Small;
            require head.numeral_sign = Nonnegative;
        }

        construction LargeUnsignedScalar: UnsignedScalar {
            form [head: lexical(Numeral)];
            require head.numeral_kind = GroupedArabic;
            require head.numeral_size = Large;
            require head.numeral_sign = Nonnegative;
        }

        construction UnsignedScalar: ScalarComponent {
            head value;
            form [value: UnsignedScalar];
        }

        construction PositiveScalar: ScalarComponent {
            head value;
            export ScalarSign = Plus;
            onset Consonant;
            form ["+", value: UnsignedScalar];
        }

        construction NegativeScalar: ScalarComponent {
            head value;
            onset Consonant;
            form ["-", value: UnsignedScalar] export ScalarSign = Hyphen;
            form ["−", value: UnsignedScalar] export ScalarSign = Minus;
        }

        // AD1–AD7: the optional dependent belongs to the adjustment Complement.
        // Its article follows ordinary lexical Onset agreement.
        table additive_modifier_presence(present) -> AdditiveModifierUse {
            (No) => No, (Yes) => Yes,
        }
        construction SlashPair: SlashPair {
            head none;
            form [determiner: optional(lexical(Determinative), " "),
                modifier: optional(lexical(Adjective), " "),
                left: ScalarComponent, "/", right: ScalarComponent]
                require determiner.article_onset = Vowel
                require modifier.onset = Vowel;
            form [determiner: optional(lexical(Determinative), " "),
                modifier: optional(lexical(Adjective), " "),
                left: ScalarComponent, "/", right: ScalarComponent]
                require modifier.present = Yes
                require determiner.article_onset = Consonant
                require modifier.onset = Consonant;
            agree determiner.present = modifier.present;
            require determiner.DeterminerKind = Indefinite;
            require determiner.number = Singular;
            require modifier.AdditiveModifierUse = Yes;
            export AdditiveModifierUse = additive_modifier_presence(modifier.present);
        }

        schema SlashMeasure {
            head pair;
            form [pair: SlashPair];
            require pair.AdditiveModifierUse = No;
            export MeasureKind = Pair;
        }
        instance SlashMeasure<Result>: [
            (MeasurePhrase),
            (PropertyValue),
        ] {}


        construction SlashModifiedNominal: Nominal {
            form [modifier: SlashPair, " ", head: Nominal];
            require modifier.AdditiveModifierUse = No;
            require head.SlashPremodifierUse = Yes;
            export NominalDependentPresent = Yes;
            require head.Targeting = No;
            require head.countability = Count;
        }

        construction UngroupedScalarNumeral: MeasurePhrase {
            export UnitBearing = No;
            form [modifier: optional(CardinalModifier), head: lexical(Numeral)];
            require head.numeral_kind = Arabic;
            require head.numeral_size = Small;
            export MeasureKind = Scalar;
        }

        construction ScalarMeasureOperand: ArithmeticOperand {
            head value;
            form [value: MeasurePhrase];
            require value.MeasureKind = Scalar;
        }
        // Spelled scalar operands follow ticket rulings SC1–SC4 (2026-10-10).
        construction CardinalScalarOperand: ArithmeticOperand {
            export UnitBearing = No;
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }
        construction ScalarNominalOperand: ArithmeticOperand {
            head value;
            form [value: NounPhrase];
            require value.ScalarDenotation = Yes;
        }

        // Arithmetic nominal operands follow ticket rulings R10a–R10d
        // (2026-10-09). CGEL's PP complementation does not supply this route.
        construction ArithmeticMeasure: MeasurePhrase {
            head none;
            export UnitBearing = arithmetic_unit_bearing(left.UnitBearing, right.UnitBearing);
            form [left: ArithmeticOperand, " ", operator: lexical(Preposition), " ",
                right: ArithmeticOperand];
            require operator.MeasureOperator = Yes;
            export MeasureKind = Scalar;
        }

        construction ScalarExtentMeasure: ScalarExtentMeasure {
            form [value: ScalarMeasurePhrase];
        }
        construction CardinalExtentMeasure: ScalarExtentMeasure {
            form [value: Cardinal];
        }
        // CGEL Ch. 8 §§5.2–5.3: scalar extent is selected by its lexical host.
        construction ScalarExtentComplement: ScalarExtentComplement {
            form [marker: lexical(Preposition), " ", complement: ScalarExtentMeasure];
            require marker.ExtentMarker = Yes;
        }

        construction MeasuredNounPhrase: NounPhrase {
            export PartitiveCountability = partitive_countability(head.countability);
            export UnitBearing = Yes;
            export ScalarDenotation = Yes;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [quantity: MeasurePhrase, " ", head: Nominal]
                require quantity.MeasureKind = Scalar
                require quantity.UnitBearing = No;
            // Orchestrator rulings BQ1–BQ4 (2026-10-10): refine the quantity
            // slot, retaining the measured head's singular conceptualization.
            form [quantity: ComparativeQuantity, " ", head: Nominal]
                require quantity.ComparativeQuantityStructure = Coordinated
                require quantity.MeasureModifierUse = Yes;
            require head.MeasurePosition = Before;
            require head.ScalarMeasurementHeadUse = Yes;
            require head.DeterminerRequirement = No;
            require head.Targeting = No;
            require head.number = Singular;
            require head.countability = Mass;
            export number = Singular;
            use ThirdPersonCommonCase;
        }

        policy ScalarValueMeasure {
            export PredicativeKind = Nominal;
            require value.MeasureKind = Scalar;
        }
        policy ScalarPropertyValueKind {
            export MeasureKind = Scalar;
        }
        schema ScalarMeasureValue {
            head value;
            cost 0;
            form [value: node];
        }
        instance ScalarMeasureValue<Result, Value: value, Properties>: [
            (ScalarValue, MeasurePhrase, ScalarValueMeasure),
            (ScalarLocationComplement, CostSymbols, NoFeatures),
            (PropertyValue, ScalarValue, ScalarPropertyValueKind),
        ] {
            use Properties;
        }

        // Orchestrator rulings R5a (2026-10-08) and R13a (2026-10-09):
        // ordering and equality comparative AdjPs enter the existing
        // ScalarMeasureValue route to PropertyValue.
        // CGEL Ch. 6 §3.3(c), p. 552 supports their postpositive function,
        // not this value-slot analysis.
        construction ComparativeScalarValue: ScalarValue {
            head value;
            export PredicativeKind = Adjectival;
            form [value: ComparativeAdjectivePhrase];
        }

        construction ScalarCardinalValue: ScalarValue {
            export PropertyEqualityForm = None;
            export PredicativeKind = Nominal;
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }

        construction ScalarPropertyCoordination: ScalarValue {
            head value;
            export PredicativeKind = Nominal;
            cost 0;
            form [value: ComparativeQuantity];
            require value.ComparativeQuantityStructure = Coordinated;
        }

        // CGEL Ch. 5 §11(d), p. 432 n. 48 classifies greater as an
        // Adjective. Its permission in this value position is a project ruling.
        construction AdjectivalScalarCoordination: ScalarValue {
            head none;
            export PropertyEqualityForm = None;
            export PredicativeKind = Nominal;
            form [left: QuantityConjunct, " ", marker: lexical(Coordinator), " ",
                right: lexical(Adjective)];
            require marker.CoordinationKind = Alternative;
            require marker.NoncorrelativeCoordination = Yes;
            require right.ScalarComparativeUse = Yes;
            require right.frame = Ordering;
        }

        // Project rulings: scalar-property-values (2026-10-07) and
        // coordinated-property-values (2026-10-08). A property Nominal selects
        // its following value Complement through declared position and kind.
        construction MeasuredAttribute: NounPhrase {
            export PartitiveCountability = partitive_countability(head.countability);
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: Nominal, " ", quantity: PropertyValue];
            require property_value_kind(head.PropertyValuePosition, quantity.MeasureKind,
                quantity.PropertyEqualityForm) = Yes;
            require property_equality_agreement(quantity.PropertyEqualityForm,
                head.NominalAgreementNumber, head.number) = Yes;
            require head.DeterminerRequirement = No;
            require head.Targeting = No;
            require head.number = Singular;
            export number = nominal_agreement_number(head.NominalAgreementNumber, head.number);
            use ThirdPersonCommonCase;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
        }

        // CGEL Ch. 13 §1.3, pp. 1103–1104 distinguishes bare/expanded
        // comparative Complements and lists governors in [15]; its constituency
        // is [governor [P X]]. Our [governor marker][bare Complement] grouping
        // is a project convention, following SelectedPrepositionHead and
        // FiniteSelectedHead for shared delayed Complements. CGEL Ch. 15 §4.4,
        // p. 1343 n. 66 supports coordinating governors each keeping its marker.
        table comparative_marker(frame, ComparisonMarker) -> Selection {
            (Equality, Equality) => Yes, (Ordering, Ordering) => Yes,
        }

        // CGEL Ch. 13 §1.3, p. 1104 [15iii]: an Adverb can govern a
        // comparative Complement. Ch. 14 §8.4(b), p. 1263 [27i] describes
        // matching infinitival forms in would-rather. The postposed Oracle
        // nominal and gerund analyses are the rather-than ticket's project
        // analysis; neither citation asserts those extensions.
        construction NominalMatrixComparativeComplement: MatrixComparativeComplement {
            form [head: lexical(Preposition), " ", complement: AccusativePhrase];
            require licence_comparative_complement(head.PrepositionFunctionLicence) = Yes;
            export ComparisonMarker = head.ComparisonMarker;
            export ComparativeComplementForm = Nominal;
        }
        construction NonfiniteMatrixComparativeComplement: MatrixComparativeComplement {
            form [head: lexical(Preposition), " ", complement: SecondaryVerbPhrase];
            require licence_comparative_complement(head.PrepositionFunctionLicence) = Yes;
            export ComparisonMarker = head.ComparisonMarker;
            export ComparativeComplementForm = nonfinite_comparative_form(complement.form);
        }
        construction MatrixComparativePhrase: MatrixComparativePhrase {
            head governor;
            form [governor: lexical(Adverb), " ", complement: MatrixComparativeComplement];
            require governor.MatrixComparisonUse = Yes;
            agree governor.ComparisonMarker = complement.ComparisonMarker;
            export ComparativeComplementForm = complement.ComparativeComplementForm;
        }
        schema MatchedComparativeConstituent {
            form [head: node, " ", comparison: MatrixComparativePhrase];
        }
        policy NominalComparisonHost {
            export NominalDependentPresent = Yes;
            export ComparativePostmodifierUse = No;
            require comparison.ComparativeComplementForm = Nominal;
        }
        policy NonfiniteComparisonHost {
            require matched_nonfinite_comparison(head.form, comparison.ComparativeComplementForm) = Yes;
            export ParticipialSupplementUse = No;
        }
        instance MatchedComparativeConstituent<Result, Head: head, Properties>: [
            (NounPhrase, Self, NominalComparisonHost),
            (SecondaryVerbPhrase, Self, NonfiniteComparisonHost),
        ] { use Properties; }

        table coordinated_comparison_marker(ComparisonMarker, ComparisonMarker)
            -> ComparisonMarker {
            (Equality, Equality) => Equality, (Equality, Ordering) => Ordering,
            (Ordering, Equality) => Ordering, (Ordering, Ordering) => Ordering,
        }
        construction ComparativeGovernorHead: ComparativeGovernorHead {
            export ComparisonMarker = marker.ComparisonMarker;
            form [head: lexical(Adjective), " ", marker: lexical(Preposition)];
            require comparative_marker(head.frame, marker.ComparisonMarker) = Yes;
            require licence_comparative_complement(marker.PrepositionFunctionLicence) = Yes;
        }

        construction ScalarComparativeComplement: ComparativeComplement {
            head value;
            form [value: ScalarMeasurePhrase];
        }

        // SC3 excludes a second, adjective-premodified quantity analysis.
        construction CardinalComparativeComplement: ComparativeComplement {
            export AdjectivePremodifierUse = No;
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }

        construction NominalComparativeComplement: ComparativeComplement {
            head value;
            form [value: AccusativePhrase];
        }

        construction ComparativeAdjectivePhrase: ComparativeAdjectivePhrase {
            head governor;
            export PropertyEqualityForm = property_equality_form(governor.ComparisonMarker);
            export AdjectivePremodifierUse = complement.AdjectivePremodifierUse;
            form [governor: ComparativeGovernorHead, " ", complement: ComparativeComplement];
        }

        // PE1–PE5 extend the implicit-partitive Adjunct to a reduced
        // predication. The existing clause routes contain a Subject or a
        // copular Verb; this prefix AdjP shape has neither. Its plural
        // predicand requirement is discharged only by MeasuredAttribute.
        construction QuantifiedComparativeAdjectivePhrase: ComparativeAdjectivePhrase {
            head value;
            form [quantifier: lexical(Determinative), " ", value: ComparativeAdjectivePhrase];
            require quantifier.QuantificationalAdjunctUse = Yes;
            require value.PropertyEqualityForm = Unquantified;
            export PropertyEqualityForm = Quantified;
            export AdjectivePremodifierUse = No;
        }

        construction ComparativeAdjective: AdjectivePhrase {
            head phrase;
            require unquantified_comparative(phrase.PropertyEqualityForm) = Yes;
            form [phrase: ComparativeAdjectivePhrase];
            export AdjectiveStructure = Complemented;
        }

        construction CardinalAmount: Amount {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }

        construction ScalarAmount: Amount {
            form [value: UnsignedScalar];
        }

        // CGEL Ch. 13 §1.3, p. 1104 describes the scalar-equality governor
        // and expanded Complement; Ch. 6 §5.2, p. 569 identifies duration
        // use of the adverb. Applying that analysis to Oracle duration
        // Complements is the project analysis tested in this ticket.
        construction EquativeAdverb: AdverbPhrase {
            export FrequencyUse = No;
            form [governor: lexical(Adverb), " ", head: lexical(Adverb), " ",
                marker: lexical(Preposition), " ", complement: FiniteClause];
            require governor.ComparisonMarker = Equality;
            require head.DurationUse = Yes;
            require marker.ExpandedComparisonMarker = Equality;
            export VPFinalAdjunct = head.VPFinalAdjunct;
            export UnmarkedConjunctLicence = governor.UnmarkedConjunctLicence;
        }

        construction Adverb: AdverbPhrase {
            require head.MatrixComparisonUse = No;
            form [head: lexical(Adverb)];
            export VPFinalAdjunct = head.VPFinalAdjunct;
        }
        instance DepictivePredicate<Result, Properties>: [
            (FinitePredicate, FiniteHeadAgreement),
            (SecondaryVerbPhrase, OrdinarySecondaryHead),
        ] {
            use Properties;
        }
        instance ComplementedDepictivePredicate<Result, Head: head>: [
            (FinitePredicate, FiniteDepictiveHost),
            (SecondaryVerbPhrase, SecondaryDepictiveHost),
        ] {}
        instance AdverbPredicate<Result, Head: head, Modifier: modifier>: [
            (FinitePredicate, Self, AdverbPhrase),
            (SecondaryVerbPhrase, Self, AdverbPhrase),
        ] {}

        construction InitialAdverb: Clause {
            head clause;
            export ClauseCoordinationHost = Simple;
            form [modifier: AdverbPhrase, " ", clause: Clause];
            form [modifier: AdverbPhrase, ", ", clause: Clause];
            require modifier.ClauseInitialAdjunct = Yes;
            export UnmarkedConjunctLicence = modifier.UnmarkedConjunctLicence;
        }

        // Project analysis: a mana measure modifies the comparative determinative.
        schema MeasuredScalarLocation {
            form [modifier: node, " ", head: lexical(node)];
            require head.MeasureModifierUse = Yes;
        }
        // IS2–IS3 (orchestrator ruling, 2026-10-10): the measure modifies
        // the comparative head when its standard is unexpressed.
        policy MeasuredComparativeAdjectiveProperties {
            require modifier.MeasureKind = Scalar;
            require head.frame = Intransitive;
            export PredicativeKind = Adjectival;
            export PredicativeAdjunctUse = No;
        }
        instance MeasuredScalarLocation<Result, Modifier: modifier, Head: head, Properties>: [
            (ScalarLocationComplement, CostSymbols, Determinative, NoFeatures),
            (PredicativeComplement, MeasurePhrase, Adjective, MeasuredComparativeAdjectiveProperties),
        ] { use Properties; }

        // A selected hollow internal Complement links its Object Gap to the Subject
        // (CGEL Ch. 14 §6.3(c), p. 1250); ordinary infinitivals keep a full predicate.
        schema ToInfinitive {
            form [negator: optional(SecondaryNegator), marker: lexical(Subordinator), " ", predicate: node];
            require marker.InfinitivalMarker = Yes;
        }
        schema Negator {
            form [head: lexical(Adverb)];
        }
        policy SecondaryNegatorLicence {
            require head.SecondaryNegation = Yes;
        }
        policy PrimaryNegatorLicence {
            require head.PrimaryNegation = Yes;
        }
        instance Negator<Result, Licence>: [
            (SecondaryNegatorHead, SecondaryNegatorLicence),
            (PrimaryNegator, PrimaryNegatorLicence),
        ] { use Licence; }
        construction SecondaryNegator: SecondaryNegator {
            form [head: SecondaryNegatorHead, " "];
        }
        instance ToInfinitive<Result, Predicate: predicate>: [
            (InfinitiveComplement, BarePredicate),
            (HollowInfinitivalComplement, BareObjectGap),
        ] {}

        instance ToInfinitive<Result, Predicate: predicate>: [
            (PurposeInfinitival, BarePredicate),
        ] {}

        // DR6 fallback: optional lexical fields cannot participate in the
        // cross-field frame/marker licence or export its completion distinction.
        // These members are incomplete; only shared-head completion admits their NP.
        construction PassiveSelectedHead: SecondarySelectedHead {
            form [head: lexical(Verb), " ", marker: lexical(Preposition)]
                require selected_object_marker(head.frame, marker.KeywordMarker) = Yes
                export FrameUse = PassiveNominal;
            form [head: lexical(Verb), " ", marker: lexical(Preposition)]
                require head.frame = Transitive
                require marker.InternalisedComplementMarker = Yes
                export FrameUse = PassiveInternalised;
            require head.form = PastParticiple;
            require head.Clitic = No;
            require noun_phrase_complement(marker.PrepositionComplement) = Yes;
            require licence_verb_complement(marker.PrepositionFunctionLicence) = Yes;
            export HeadCoordination = No;
            use SecondaryHeadForm;
        }

        // MP2–MP3: the declared Mana Complement frame selects this passive
        // head, independently of the verb's lexical identity.
        instance SelectedVerbHead<Result, Properties>: [
            (ManaPassivePredicate, SecondaryHeadForm),
        ] {
            require head.frame = ManaComplement;
            require head.form = PastParticiple;
            use Properties;
        }

        // MP2/MP6: a governed Subject projection supplies singular Agreement
        // without turning bare Mana Phrase notation into a Noun Phrase.
        construction ManaSubject: ManaSubject {
            form [head: ManaPhrase];
            export number = Singular;
            export person = Third;
            export SubjectStructure = Phrase;
        }

        // Shared head primitives: exact selected signatures; no complement or coordination yet.

        instance SelectedVerbHead<Result, Properties>: [
            (FiniteSelectedHead, FiniteHeadAgreement),
            (SecondarySelectedHead, SecondaryHeadForm),
        ] {
            use Properties;
        }

        table cardinal_coordinate_number(CoordinationKind, number, number) -> number {
            (Additive, Singular, Singular) => Plural,
            (Additive, Singular, Plural) => Plural,
            (Additive, Plural, Singular) => Plural,
            (Additive, Plural, Plural) => Plural,
            (Alternative, Singular, Singular) => Singular,
            (Alternative, Singular, Plural) => Plural,
            (Alternative, Plural, Singular) => Plural,
            (Alternative, Plural, Plural) => Plural,
            (Adversative, Singular, Singular) => Singular,
            (Adversative, Singular, Plural) => Plural,
            (Adversative, Plural, Singular) => Singular,
            (Adversative, Plural, Plural) => Plural,
        }

        instance SharedObjectComplement<Result, Head: head, Object: object>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase),
        ] {}
        instance SharedObjectComplement<Result, Head: head, Object: object, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedPredicativeComplement<Result, Head: head, Complement: complement>: [
            (FinitePredicate, FiniteSelectedHead, PredicativeComplement),
        ] {}
        instance SharedPredicativeComplement<Result, Head: head, Complement: complement, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, PredicativeComplement, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedLocativeComplement<Result, Head: head, Complement: complement>: [
            (FinitePredicate, FiniteSelectedHead, LocativeComplement),
        ] {}
        instance SharedLocativeComplement<Result, Head: head, Complement: complement, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, LocativeComplement, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedManaComplement<Result, Head: head, Complement: complement>: [
            (FinitePredicate, FiniteSelectedHead, ManaPhrase),
        ] {}
        instance SharedManaComplement<Result, Head: head, Complement: complement, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, ManaPhrase, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAmountComplement<Result, Head: head, Amount: amount>: [
            (FinitePredicate, FiniteSelectedHead, Amount),
        ] {}
        instance SharedAmountComplement<Result, Head: head, Amount: amount, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, Amount, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedMeasureComplement<Result, Head: head, Measure: measure>: [
            (FinitePredicate, FiniteSelectedHead, MeasurePhrase),
        ] {}
        instance SharedMeasureComplement<Result, Head: head, Measure: measure, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, MeasurePhrase, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedSlashMeasureComplement<Result, Head: head, Measure: measure>: [
            (FinitePredicate, FiniteSelectedHead, SlashPair),
        ] {}
        instance SharedSlashMeasureComplement<Result, Head: head, Measure: measure, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, SlashPair, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedGrantedAbilityComplement<Result, Head: head, Complement: complement>: [
            (FinitePredicate, FiniteSelectedHead, GrantedAbility),
        ] {}
        instance SharedGrantedAbilityComplement<Result, Head: head, Complement: complement, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, GrantedAbility, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAuxiliaryBareComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, BareComplement, SharedAuxiliaryAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, BareComplement,
                AuxiliarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAuxiliaryParticipleComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, ParticipialComplement, SharedAuxiliaryAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, ParticipialComplement,
                SelectedAuxiliaryProperties),
        ] {
            use Properties;
        }
        instance SharedAuxiliaryPerfectComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, PerfectComplement, SharedAuxiliaryAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, PerfectComplement,
                SelectedAuxiliaryProperties),
        ] {
            use Properties;
        }
        instance SharedObjectNameComplement<Result, Head: head, Object: object, Complement: complement>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, Name),
        ] {}
        instance SharedObjectNameComplement<Result, Head: head, Object: object, Complement: complement, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase, Name, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        table coordinated_adjective_structure(AdjectiveStructure,
            AdjectiveStructure) -> AdjectiveStructure {
            (Simple, Simple) => Simple,
            (Simple, Complemented) => Complemented,
            (Complemented, Simple) => Complemented,
            (Complemented, Complemented) => Complemented,
        }

        construction CoordinatedClauseComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export ClauseComplementBoundary = Finite;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            form [head: lexical(Preposition), " ", complement: CoordinatedFiniteClause]
                require complement.GeneralCoordination = Yes;
            form [head: lexical(Preposition), " ", complement: CoordinatedFiniteClause]
                require complement.GeneralCoordination = No
                require head.FiniteClauseComplement = Yes;
            require finite_clause_complement(head.FiniteClauseComplement) = Yes;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        table combined_predicative_kind(PredicativeKind, PredicativeKind) -> PredicativeKind {
            (Adjectival, Adjectival) => Adjectival,
            (Nominal, Nominal) => Nominal,
            (Adjectival, Nominal) => Mixed,
            (Nominal, Adjectival) => Mixed,
            (Mixed, Adjectival) => Mixed,
            (Mixed, Nominal) => Mixed,
            (Adjectival, Mixed) => Mixed,
            (Nominal, Mixed) => Mixed,
            (Mixed, Mixed) => Mixed,
            (Prepositional, Prepositional) => Prepositional,
            (Prepositional, Adjectival) => Mixed, (Adjectival, Prepositional) => Mixed,
            (Prepositional, Nominal) => Mixed, (Nominal, Prepositional) => Mixed,
            (Prepositional, Mixed) => Mixed, (Mixed, Prepositional) => Mixed,
        }
        table unlike_predicative_kind(PredicativeKind, PredicativeKind) -> PredicativeKind {
            (Adjectival, Nominal) => Mixed,
            (Nominal, Adjectival) => Mixed,
            (Mixed, Adjectival) => Mixed,
            (Mixed, Nominal) => Mixed,
            (Adjectival, Mixed) => Mixed,
            (Nominal, Mixed) => Mixed,
            (Mixed, Mixed) => Mixed,
        }

        table combined_depictive_kind(DepictiveKind, DepictiveKind) -> DepictiveKind {
            (Adjectival, Adjectival) => Adjectival,
            (Participial, Participial) => Participial,
            (Adjectival, Participial) => Mixed,
            (Participial, Adjectival) => Mixed,
            (Mixed, Adjectival) => Mixed,
            (Mixed, Participial) => Mixed,
            (Adjectival, Mixed) => Mixed,
            (Participial, Mixed) => Mixed,
            (Mixed, Mixed) => Mixed,
        }
        table unlike_depictive_kind(DepictiveKind, DepictiveKind) -> DepictiveKind {
            (Adjectival, Participial) => Mixed,
            (Participial, Adjectival) => Mixed,
            (Mixed, Adjectival) => Mixed,
            (Mixed, Participial) => Mixed,
            (Adjectival, Mixed) => Mixed,
            (Participial, Mixed) => Mixed,
            (Mixed, Mixed) => Mixed,
        }

        // CGEL pp. 467–468: a genitive NP determines the following nominal.
        table genitive_marker(number, BareGenitiveHost, HostEnding) -> Selection {
            (Singular, No, Default) => Yes, (Singular, Yes, Default) => Yes,
            (Plural, Yes, PluralS) => Yes,
        }
        construction GenitiveNounPhrase: NounPhrase {
            export Definiteness = Definite;
            require possessor.BarePrepositionUse = No;
            export WholeObjectSourcePresent = whole_object_source_presence(
                head.WholeObjectSourcePresent, possessor.WholeObjectSourcePresent);
            export BarePrepositionUse = No;
            form [possessor: NounPhrase, marker: lexical(Clitic), " ", head: Nominal];
            require possessor.CaseUse = Common;
            require marker.Function = Genitive;
            require genitive_marker(possessor.number, possessor.BareGenitiveHost,
                marker.HostEnding) = Yes;
            use NounPhraseHeadAgreement;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
            require nondefinite_determiner_requirement(head.DeterminerRequirement) = Yes;
        }

        construction PossessiveNounPhrase: NounPhrase {
            export Definiteness = Definite;
            export BarePrepositionUse = No;
            form [possessor: lexical(Pronoun), " ", head: Nominal];
            require possessor.case = Genitive;
            require possessor.NominalLicense = AnyNominal;
            use NounPhraseHeadAgreement;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
            require nondefinite_determiner_requirement(head.DeterminerRequirement) = Yes;
        }

        schema KeywordComplementPreposition {
            form [head: lexical(Preposition), " ", complement: KeywordPhrase];
        }
        policy OrdinaryKeywordPreposition {
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            require head.KeywordComplement = Yes;
        }
        // CK2: this category has no free Modifier or Adjunct route. Only a
        // declared nominal frame consumes it; Of remains unlicensed as a
        // general keyword-complemented PrepositionPhrase.
        policy SelectedKeywordPreposition {
            require licence_noun_complement(head.PrepositionFunctionLicence) = Yes;
        }
        instance KeywordComplementPreposition<Result, Properties>: [
            (PrepositionPhrase, OrdinaryKeywordPreposition),
            (SelectedKeywordPreposition, SelectedKeywordPreposition),
        ] {
            use Properties;
        }
        construction QuotedComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            form [head: lexical(Preposition), " ", complement: GrantedAbility];
            require head.QuotedComplement = Yes;
            require complement.GrantedAbilityKind = Quoted;
            require licence_modifier(head.PrepositionFunctionLicence) = Yes;
            export PrepositionFunctionLicence = Modifier;
        }
        instance SharedQuantitativeDeterminerComplement<Result, Head: head, Complement: complement>: [
            (FinitePredicate, FiniteSelectedHead, QuantitativeDeterminer),
        ] {}
        instance SharedQuantitativeDeterminerComplement<Result, Head: head, Complement: complement, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, QuantitativeDeterminer, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }

        instance SharedInfinitiveComplement<Result, Head: head, Complement: complement>: [
            (FinitePredicate, FiniteSelectedHead, InfinitiveComplement),
        ] {}
        instance SharedInfinitiveComplement<Result, Head: head, Complement: complement, Properties>: [
            (SecondaryVerbPhrase, SecondarySelectedHead, InfinitiveComplement, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }

        construction AdjectivalCorrelativeComplement: PredicativeComplement {
            head phrase;
            form [phrase: CorrelativeAdjectivePhrase];
            export PredicativeKind = Adjectival;
            require phrase.AdjectiveAttributiveOnlyUse = No;
        }

        construction CorrelativePostpositiveNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: CorrelativeAdjectivePhrase];
            export NominalDependentPresent = Yes;
            require modifier.AdjectiveAttributiveOnlyUse = No;
        }

        construction SelectedPrepositionHead: SelectedPrepositionHead {
            form [head: lexical(Preposition)];
            require noun_phrase_complement(head.PrepositionComplement) = Yes;
            export HeadCoordination = No;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        construction SharedPrepositionComplement: PrepositionPhrase {
            form [head: SelectedPrepositionHead, " ", complement: AccusativePhrase];
            require head.HeadCoordination = Yes;
            export SelectedPrepositionUse = selected_preposition_complement(head.SelectedPrepositionUse,
                complement.SelectedPrepositionUse);
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
        }

    }
}
