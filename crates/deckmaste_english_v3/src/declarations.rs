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
        feature DegreeUse { Head, Modifier }
        feature PredeterminerUse { No, Multiplier, Fraction } default No;
        table predeterminer_host(PredeterminerUse, Definiteness) -> Selection {
            (Multiplier, Definite) => Yes, (Fraction, Definite) => Yes,
        }
        feature ParticipialSupplementUse { No, Yes } default No;
        table coordinated_participial_supplement(ParticipialSupplementUse, ParticipialSupplementUse)
            -> ParticipialSupplementUse {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        feature Definiteness { Unspecified, Nondefinite, Definite } default Unspecified;
        feature NounPhraseStructure { Other, ProperName, IndefiniteArticle } default Other;
        feature DeterminerRequirement { No, Yes, Definite } default No;
        feature NumberTransparency { No, Yes } default No;
        feature SubjectRelativeNumber { None, Singular, Plural } default None;
        table relative_number_concord(SubjectRelativeNumber, SubjectRelativeNumber) -> SubjectRelativeNumber {
            (None, None) => None, (None, Singular) => Singular, (Singular, None) => Singular,
            (None, Plural) => Plural, (Plural, None) => Plural,
            (Singular, Singular) => Singular, (Plural, Plural) => Plural,
        }
        table deferred_relative_number(NominalAgreementNumber, number) -> SubjectRelativeNumber {
            (Head, Singular) => None, (Head, Plural) => None,
            (Plural, Singular) => Singular, (Plural, Plural) => Plural,
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
        }
        table resolved_relative_number(SubjectRelativeNumber, NominalAgreementNumber, number, DeterminerAgreementNumber) -> Selection {
            (None, _, _, _) => Yes,
            (Singular, _, _, Singular) => Yes,
            (Singular, Head, Singular, Nominal) => Yes,
            (Plural, Head, Plural, Nominal) => Yes,
            (Plural, Plural, _, Nominal) => Yes,
        }
        feature NominalAgreementNumber { Head, Plural } default Head;
        feature DeterminerAgreementNumber { Nominal, Singular } default Nominal;
        feature QuantificationalDeterminer { No, Yes } default No;
        feature QuantificationalAdjunctUse { No, Yes } default No;
        feature AuxiliaryUse { No, Yes } default No;
        feature Polarity { Positive, Negative } default Positive;
        feature PrimaryNegation { Yes }
        feature ObliqueMarker { No, Yes } default No;
        feature ObliqueNumber { None, Singular, Plural } default None;
        feature CaseUse { Common, Nominative, Accusative }
        feature Targeting { No, Yes }
        feature TargetingPremodifierUse { No, Yes } default No;
        table targeting_premodifier(TargetingPremodifierUse, Targeting) -> Selection {
            (No, No) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        table coordinated_targeting_premodifier(TargetingPremodifierUse, TargetingPremodifierUse)
            -> TargetingPremodifierUse {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        feature ClauseCoordinationHost { Simple, Coordinated } default Simple;
        feature ClauseComplementBoundary { None, Finite, Verbless } default None;
        feature ConnectiveAdjunctUse { No, Yes } default No;
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
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        feature CoordinationKind { Additive, Alternative, Adversative }
        feature PrepositionComplement { NounPhrase, PredicativeNounPhrase, PrepositionPhrase, Intransitive } set;
        table noun_phrase_complement(PrepositionComplement) -> Selection contains NounPhrase => Yes;
        table preposition_phrase_complement(PrepositionComplement) -> Selection contains PrepositionPhrase => Yes;
        table predicative_noun_phrase_complement(PrepositionComplement) -> Selection contains PredicativeNounPhrase => Yes;
        table intransitive_preposition_complement(PrepositionComplement) -> Selection contains Intransitive => Yes;
        feature AdjectiveComplementClass { None, Manner, ColorWord } default None;
        feature AdverbComplementClass { Duration }
        feature DurationUse { No, Yes } default No;
        feature FocusingModifierUse { Restrictive }
        feature FocusedUse { No, Yes } default No;
        // CF7: a focused coordinate blocks focus on the whole coordination.
        table coordinated_focus(FocusedUse, FocusedUse) -> FocusedUse {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        // CF8: two focused Adjuncts use the common member route exclusively.
        table ordinary_adjunct_coordination(FocusedUse, FocusedUse) -> Selection {
            (No, No) => Yes, (No, Yes) => Yes, (Yes, No) => Yes,
        }
        feature NominalAdjunctHost { No, Yes } default Yes;
        feature PredicativeAdjunctUse { No, Yes } default No;
        table predicative_adjunct_concord(PredicativeAdjunctUse, PredicativeAdjunctUse)
            -> PredicativeAdjunctUse {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        feature PurposeAdjunctLicence { No, Yes } default Yes;
        table coordinated_purpose_licence(PurposeAdjunctLicence, PurposeAdjunctLicence)
            -> PurposeAdjunctLicence {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        policy PurposeConcord<Right, Source> {
            export PurposeAdjunctLicence = coordinated_purpose_licence(
                left.PurposeAdjunctLicence, Right.PurposeAdjunctLicence);
            export MatrixComplementLicence = matrix_host_concord(
                left.MatrixComplementLicence, Right.MatrixComplementLicence);
            export SourceComplementSelection = source_selection_union(
                left.SourceComplementSelection, Right.SourceComplementSelection);
        }
        feature FusedHeadUse { No, Yes } default No;
        feature ComparativePostmodifierUse { No, Yes } default No;
        feature QuantitativeComplement { None, Cardinal, CardinalPrepositionPhrase, AdjectivePhrase } default None;
        feature QuantitativeAdjectiveComplementUse { No, Yes } default No;
        feature ComparativeQuantityUse { No, Yes } default No;
        feature QuantitativeConjunctUse { No, Yes } default No;
        feature ComparativeDeterminerUse { No, Yes } default No;
        feature ComparativeQuantityStructure { Coordinated, Headed }
        feature ScalarComparativeUse { No, Yes } default No;
        feature ImplicitComparativeUse { No, Yes } default No;
        feature AdjectiveAttributiveOnlyUse { No, Yes } default No;
        table implicit_comparative_attributive_use(ImplicitComparativeUse) -> AdjectiveAttributiveOnlyUse {
            (No) => No, (Yes) => Yes,
        }
        table adjective_attributive_concord(AdjectiveAttributiveOnlyUse, AdjectiveAttributiveOnlyUse)
            -> AdjectiveAttributiveOnlyUse {
            (No, No) => No, (_, _) => Yes,
        }
        feature ComparativeModifierPresent { No, Yes } default No;
        table implicit_comparative_concord(ImplicitComparativeUse, ImplicitComparativeUse)
            -> ImplicitComparativeUse {
            (Yes, Yes) => Yes, (_, _) => No,
        }
        table comparative_modifier_present(ImplicitComparativeUse, ComparativeModifierPresent)
            -> ComparativeModifierPresent {
            (Yes, _) => Yes, (No, Yes) => Yes, (No, No) => No,
        }
        feature MeasureModifierUse { No, Yes } default No;
        feature PrepositionFunctionLicence { Adjunct, Modifier, NounComplement, VerbComplement,
            PredicativeComplement, Compound, ComparativeComplement, PreposedAdjunct,
            Supplement, MedialSupplement } set;
        feature MatrixComplementGovernor { No, Yes } default No;
        feature MatrixComplementRequirement { None, Internalised, Nominal, Content } default None;
        feature MatrixComplementLicence { None, Nominal, Content } default None;
        feature NominalExceptionComplementUse { No, Yes } default No;
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
        feature PrepositionPhraseComplementLicence { Unrestricted, Source }
        feature SourceComplementUse { No, Yes } default No;
        feature SourceMarker { No, Yes } default No;
        feature SourceComplementSelection { No, Yes } default No;
        table source_selection_union(SourceComplementSelection, SourceComplementSelection)
            -> SourceComplementSelection {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        // Orchestrator ruling RS2: a source-selected PP is a Complement.
        table source_adjunct_admission(SourceComplementSelection, SourceMarker) -> Selection {
            (No, No) => Yes, (No, Yes) => Yes, (Yes, No) => Yes,
        }
        feature SourceModifierPresent { No, Yes } default No;
        table source_modifier_presence(SourceModifierPresent, SourceModifierPresent) -> SourceModifierPresent {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        table nominal_source_modifier(SourceModifierPresent, SourceMarker) -> SourceModifierPresent {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        feature WholeObjectSourcePresent { No, Yes } default No;
        table whole_object_source_presence(WholeObjectSourcePresent, WholeObjectSourcePresent) -> WholeObjectSourcePresent {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
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
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        table licensed_preposition_phrase_complement(PrepositionPhraseComplementLicence,
            SourceComplementUse) -> Selection {
            (Unrestricted, No) => Yes, (Unrestricted, Yes) => Yes, (Source, Yes) => Yes,
        }
        table coordinated_source_complement_use(SourceComplementUse, SourceComplementUse)
            -> SourceComplementUse {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
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
        feature RelativisableAdjunctClass { None, Temporal } default None;
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
        feature ComparativeObjectGapLicence { No, Yes } default No;
        table comparative_gap_concord(ComparativeObjectGapLicence, ComparativeObjectGapLicence)
            -> ComparativeObjectGapLicence {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        feature InterveningAdjunctLicence { No, Yes } default No;
        table coordinated_intervening_adjunct(InterveningAdjunctLicence, InterveningAdjunctLicence)
            -> InterveningAdjunctLicence {
            (Yes, Yes) => Yes, (No, No) => No, (No, Yes) => No, (Yes, No) => No,
        }
        feature IrrealisClauseComplement { No, Yes } default No;
        feature VerblessClauseComplement { No, Yes, OvertSubject } default No;
        feature VerblessPredicateUse { No, Yes } default No;
        feature GerundClauseComplement { Yes }
        feature ComplementFunctionRestriction { None, Adjunct, Compound, PreposedOrPredicative } default None;
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
        feature InternalisedComplementMarker { No, Yes } default No;
        feature InternalisedComplementPresent { No, Yes } default No;
        feature ExtentMarker { Yes }
        feature LocativeUse { No, Yes }
        feature AdjectiveOrigin { Ordinary, Participial } default Ordinary;
        feature AdjectivePremodifierUse { No, Yes } default Yes;
        table coordinated_adjective_premodifier(AdjectivePremodifierUse, AdjectivePremodifierUse)
            -> AdjectivePremodifierUse {
            (Yes, Yes) => Yes, (_, _) => No,
        }
        feature AdjectiveStructure { Simple, Complemented }
        feature RelativeSubordinator { Yes }
        feature ParticipialUse { Ordinary, BarePassive, Mixed }
        feature PastParticipialPremodifier { No, Yes } default No;
        feature AuxiliaryComplementRealization { Overt, Elided, ProForm }
        feature ProFormComplementUse { No, Yes } default No;
        feature AuxiliaryProFormLicence { No, Yes } default No;
        feature AuxiliaryEllipsisLicence { No, Yes } default No;
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
        table auxiliary_pro_form_concord(AuxiliaryProFormLicence,
            AuxiliaryProFormLicence) -> AuxiliaryProFormLicence {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        table auxiliary_ellipsis_concord(AuxiliaryEllipsisLicence,
            AuxiliaryEllipsisLicence) -> AuxiliaryEllipsisLicence {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        feature Clitic { No, Yes } default No;
        feature CliticHost { Free, Subject, PronounSubject }
        feature SubjectStructure { Phrase, Pronoun }
        feature PredicateFrameUse { No, Yes } default Yes;
        feature ScalarVariable { Yes }
        feature ScalarSign { Absent, Plus, Hyphen, Minus } default Absent;
        table loyalty_sign(ScalarSign) -> Selection { (Plus) => Yes, (Minus) => Yes, }
        feature MeasureOperator { Yes }
        feature ScalarDenotation { No, Yes } default No;
        feature MeasurePosition { None, Before } default None;
        feature UnitBearing { No, Yes } default No;
        table nominal_unit_bearing(MeasurePosition) -> UnitBearing {
            (None) => No, (Before) => Yes,
        }
        table arithmetic_unit_bearing(UnitBearing, UnitBearing) -> UnitBearing {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        feature ScalarMeasurementHeadUse { No, Yes } default No;
        feature PropertyValuePosition { None, Single, First, Second, Pair } default None;
        feature CardinalMeasurementUse { No, DeterminerRequired, DeterminerProjection, Yes } default No;
        feature MeasureKind { Scalar, Pair }
        feature MatrixComparisonUse { No, Yes } default No;
        feature ComparativeComplementForm { Nominal, BareInfinitival, GerundParticipial }
        table nonfinite_comparative_form(form) -> ComparativeComplementForm {
            (Plain) => BareInfinitival, (GerundParticiple) => GerundParticipial,
        }
        table matched_nonfinite_comparison(form, ComparativeComplementForm) -> Selection {
            (Plain, BareInfinitival) => Yes, (GerundParticiple, GerundParticipial) => Yes,
        }
        feature ComparisonMarker { Equality, Ordering }
        feature ExpandedComparisonMarker { Equality, Ordering }
        feature KeywordParameterClass { Nullary, Amount, Cost, Quality, Subject, AmountCost,
            QualityCost, Ability, Condition, CostPowerToughness, Instruction }
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
        feature NominalAdjunctClass { None, Temporal, Manner } default None;
        feature SlashPremodifierUse { No, Yes } default Yes;
        feature NominalDependentPresent { No, Yes } default No;
        feature NominalAdjunctDependent { None, Demonstrative, Temporal, Definite, DeicticTemporal } default None;
        feature BareSingularUse { No, Yes } default No;
        feature RelativeUse { No, Yes } default No;
        feature ExistentialUse { No, Yes } default No;
        feature KeywordMarker { None, Among, For, From, Onto, To, Into, At, With, Of, On, Under }
        feature KeywordPayloadOrder { AfterHead, BeforeHead, BoundSuffix }
        feature BoundKeyword { No, Yes }
        feature CreatureSubtypeStem { No, Yes } default No;
        feature CreatureSubtypeStemUse { No, Yes } default No;
        feature KeywordSeparator { Space, Dash, SpacedDash }
        feature KeywordParameterSeparator { Space, Dash, SpacedDash }
        feature KeywordQualityNumber { Any, Singular, Plural }
        feature Selection { Yes }
        feature Function { Genitive }
        feature HostEnding { Default, PluralS }
        feature BareGenitiveHost { No, Yes } default No;
        feature LabelKind { None, AbilityWord, FlavorWord }
        feature SymbolUse { Cost }
        feature ManaSymbolUse { Yes }
        feature TypeLineRole { Supertype, CardType, Subtype }
        feature NameNumber { Singular, Plural } default Singular;
        table name_number(NameNumber) -> number {
            (Singular) => Singular, (Plural) => Plural,
        }
        feature IdentityUse { Name }
        feature NominalComplementMarker { None, Of, To } default None;
        feature TaxonomicComplementUse { No, Yes } default No;
        table nominal_complement_marker(NominalComplementMarker, NominalComplementMarker) -> Selection {
            (None, None) => Yes, (Of, Of) => Yes, (To, To) => Yes,
        }
        feature BarePrepositionUse { No, Location, SpatialEdge } default No;
        feature BareNominalComplement { No, Location, SpatialEdge } default No;
        table bare_preposition_use(BarePrepositionUse) -> Selection {
            (Location) => Yes, (SpatialEdge) => Yes,
        }
        table bare_nominal_complement(BareNominalComplement, BarePrepositionUse) -> Selection {
            (Location, Location) => Yes, (SpatialEdge, SpatialEdge) => Yes,
        }
        feature SelectedPrepositionUse { No, Yes } default No;
        table selected_preposition_concord(SelectedPrepositionUse, SelectedPrepositionUse)
            -> SelectedPrepositionUse {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        table selected_preposition_complement(SelectedPrepositionUse, SelectedPrepositionUse)
            -> SelectedPrepositionUse {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        table nominal_postmodifier_licence(NominalComplementMarker, NominalComplementMarker)
            -> Selection {
            (None, None) => Yes, (None, Of) => Yes, (Of, None) => Yes,
            (None, To) => Yes, (Of, To) => Yes, (To, None) => Yes, (To, Of) => Yes,
        }
        feature NounPremodifier { Yes, Attribute, Keyword, None }
        feature PremodifierCoordination { No, Yes } default No;
        feature AttributePremodifierHead { No, Yes } default No;
        feature KeywordPremodifierHead { No, Yes } default No;
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
        table coordinated_attribute_premodifier_head(AttributePremodifierHead,
            AttributePremodifierHead) -> AttributePremodifierHead {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        table coordinated_keyword_premodifier_head(KeywordPremodifierHead,
            KeywordPremodifierHead) -> KeywordPremodifierHead {
            (Yes, Yes) => Yes, (_, _) => No,
        }
        policy NounPremodifierConcord<Right, Source> {
            export PremodifierCoordination = Yes;
            export NounPremodifier = coordinated_noun_premodifier(
                left.NounPremodifier, Right.NounPremodifier);
        }
        feature VPFinalAdjunct { Yes, No }
        feature ClauseInitialAdjunct { Yes, No } default No;
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
            QuantitativeDeterminer, Infinitive }
        feature HeadCoordination { No, Yes }
        feature SelectedTailLicence { ParallelFrames, DeclaredFrames } default ParallelFrames;
        feature BareCoordination { No, Yes } default Yes;
        feature CommaCoordination { No, Yes } default No;
        feature GeneralCoordination { No, Yes } default Yes;
        feature UnmarkedConjunctLicence { No, Yes } default Yes;
        feature NoncorrelativeCoordination { Yes, No }
        feature CorrelativeKind { Both, Either, Neither }
        feature CorrelativeCoordinator { And, Or, Nor }
        feature PredicativeKind { Adjectival, Nominal, Prepositional, Mixed }
        feature DepictiveKind { Adjectival, Participial, Mixed }
        feature NominalLicense { AnyNominal }
        feature KeywordComplement { Yes }
        feature QuotedComplement { Yes }
        feature QuotationStructure { Simple, Coordinated }
        feature GrantedAbilityKind { Keyword, Quoted }
        feature AbilityTermination { Complete, Unterminated } default Complete;
        feature SentenceLineList { No, Yes }
        feature ModeListLayout { DashHeader, SentenceHeader }

        category Document();
        category Ability(LabelKind, AbilityTermination, SentenceLineList);
        category AbilityContinuation();
        category Paragraph();
        category ParagraphItem();
        category ParagraphContinuation();
        category SentenceParagraph();
        category SentenceContinuation();
        category Sentence(AbilityTermination);
        category Clause(ClauseCoordinationHost, UnmarkedConjunctLicence, SourceComplementSelection);
        category ImperativeClause(ClauseCoordinationHost, UnmarkedConjunctLicence, SourceComplementSelection);
        category FiniteClause(SourceComplementSelection);
        category FiniteGappedClause();
        category InterveningPrepositionAdjunct();
        category IrrealisClause();
        category IrrealisPredicate(number, person);
        category CentralAdverbSelectedHead(Polarity, number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, frame);
        category ExistentialExtension();
        category DummyPronoun();
        category ExistentialPredicate(Polarity, NominalAdjunctHost, ClauseComplementBoundary, number, person, CliticHost,
            PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category FinitePredicate(Polarity, NominalAdjunctHost, ClauseComplementBoundary, number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category QuantifiedAuxiliaryHead(Polarity, number, person, frame, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category ScalarLocationPredicate(Polarity, number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category SecondaryVerbPhrase(ParticipialSupplementUse, NominalAdjunctHost, ClauseComplementBoundary, form, ParticipialUse, InternalisedComplementPresent, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category FiniteDepictiveHost(Polarity, number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category SecondaryDepictiveHost(ParticipialSupplementUse, form, ParticipialUse, InternalisedComplementPresent, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category BarePredicate(PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category ParticipialPredicate(PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, CentralAdjunctPosition);
        category PastParticiplePredicate(PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category PredicativeComplement(PredicativeKind, PredicativeAdjunctUse);
        category DepictivePhrase(DepictiveKind);
        category DepictivePhraseSeries(DepictiveKind);
        category Ellipsis(form);
        category BareComplement(AuxiliaryComplementRealization, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category ParticipialComplement(PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category PerfectComplement(PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category Nominal(ComparativeModifierPresent, AttributePremodifierHead, KeywordPremodifierHead, number, NominalAgreementNumber, countability, Targeting, NominalAdjunctClass, NominalDependentPresent,
            RelativisableAdjunctClass, SlashPremodifierUse,
            NumberTransparency, ObliqueNumber, BareGenitiveHost, DeterminerRequirement, MeasurePosition, ScalarMeasurementHeadUse, PropertyValuePosition, CardinalMeasurementUse,
            BarePrepositionUse, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent, WholeObjectSourcePresent, SubjectRelativeNumber, UnmarkedConjunctLicence);
        category VerbalPremodifier();
        category VerbalPremodifierSeries();
        category NounPremodifier(NounPremodifier, PremodifierCoordination);
        category NounPremodifierSeries(NounPremodifier, PremodifierCoordination);
        category PartitiveModifier();
        category NounPhrase(NounPhraseStructure, FocusedUse, UnitBearing, number, person, CaseUse, Targeting, BareGenitiveHost, SubjectStructure,
            BarePrepositionUse, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent, WholeObjectSourcePresent, DeterminerAgreementNumber, ComparativePostmodifierUse, Definiteness);
        category NominativePhrase(number, person, CaseUse, SubjectStructure, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent, WholeObjectSourcePresent);
        category AccusativePhrase(number, person, CaseUse, SelectedPrepositionUse, PredicativeAdjunctUse, SourceModifierPresent, WholeObjectSourcePresent);
        category AdjectivePhrase(AdjectiveComplementClass, AdjectiveAttributiveOnlyUse, ImplicitComparativeUse, AdjectiveStructure, AdjectiveOrigin, AdjectivePremodifierUse, TargetingPremodifierUse, PremodifierCoordination);
        category Premodifier(NounPremodifier, AdjectiveOrigin, TargetingPremodifierUse,
            SelectedPrepositionUse, PredicativeAdjunctUse);
        category PremodifierCoordination(NounPremodifier, AdjectiveOrigin, TargetingPremodifierUse,
            SelectedPrepositionUse, PredicativeAdjunctUse);
        category VerblessClause(VerblessClauseComplement);
        category Name(number);
        category NamePredicate();
        category PrepositionPhrase(InterveningAdjunctLicence, ConnectiveAdjunctUse, NominalAdjunctHost, ClauseComplementBoundary, LocativeUse, PrepositionFunctionLicence, ComplementFunctionRestriction, ObliqueNumber,
            InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse,
            ClauseInitialAdjunct, FocusedUse, SourceComplementUse, SourceModifierPresent, WholeObjectSourcePresent, SourceMarker, MatrixComplementRequirement, NominalExceptionComplementUse);
        category SourceComplement();
        category LocativeComplement();
        category MannerComplement();
        category FrequencyPhrase();
        category NominalAdjunctPhrase(NominalAdjunctClass, NominalAdjunctDependent);
        category BareTemporalNominal(NominalBareClass);
        category BoundaryComplement();
        category SubjectRelativeClause(number);
        category ObjectRelativeClause(RelativeGapKind);
        category FiniteObjectGap(Polarity, NominalAdjunctHost, ClauseComplementBoundary, number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, ComparativeObjectGapLicence);
        category BareObjectGap(NominalAdjunctHost, ClauseComplementBoundary, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, ComparativeObjectGapLicence);
        category PastParticipleObjectGap(NominalAdjunctHost, ClauseComplementBoundary, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, ComparativeObjectGapLicence);
        category GerundParticipleObjectGap(ClauseComplementBoundary, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, ComparativeObjectGapLicence);

        category Cardinal(number);
        category CardinalModifier();
        category SecondaryNegator();
        category SecondaryNegatorHead();
        category PrimaryNegator();
        category NegatedAuxiliaryHead(PredicateFrameUse, Polarity, number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, frame);
        category QuantitativeAdjectiveComplement();
        category QuantitativeDeterminer(DeterminerUse, DeterminerAgreementNumber, Definiteness, NominalAdjunctDependent);
        category CardinalPrepositionPhrase(number);
        category QuantitativePrepositionPhrase(number);
        category ComparativePrepositionPhrase(number);
        category QuantityConjunct(number, CardinalMeasurementUse);
        category ComparativeQuantity(number, ComparativeQuantityStructure, MeasureModifierUse);
        category ScalarLocationComplement();
        category HollowInfinitivalComplement();
        category PurposeInfinitival(FocusedUse);
        category ScalarValue(PredicativeKind);
        category PropertyValue(MeasureKind);
        category Amount();
        category MeasurePhrase(MeasureKind, UnitBearing);
        category ArithmeticOperand(UnitBearing);
        category ScalarMeasurePhrase();
        category ScalarExtentComplement();
        category ScalarExtentMeasure();
        category UnsignedScalar();
        category ScalarComponent(ScalarSign);
        category SlashPair();
        category ComparativeGovernorHead(ComparisonMarker);
        category ComparativeComplement(AdjectivePremodifierUse);
        category MatrixComparativeComplement(ComparisonMarker, ComparativeComplementForm);
        category MatrixComparativePhrase(ComparativeComplementForm);
        category ComparativeAdjectivePhrase(ComparisonMarker, AdjectivePremodifierUse);
        category DiscontinuousComparativeGovernor(frame, ImplicitComparativeUse);
        category ComparativeDeterminerHead(ComparativeDeterminerUse, DeterminerUse,
            NominalAdjunctDependent);
        category KeywordPhrase();
        category KeywordQuality(KeywordQualityNumber);
        category KeywordQualityPreposition(KeywordMarker, KeywordQualityNumber);
        category KeywordQualityPrepositionSeries(KeywordMarker, KeywordQualityNumber);
        category SelectedComplementTail(HeadCoordination);
        category SelectedComplementTailSeries(HeadCoordination);
        category KeywordContinuation();
        category Cost(CostComponentForm);
        category CostComponent(CostComponentForm);
        category CostContinuation(CostComponentForm);
        category CostSymbols();
        category CostSymbol();
        category ManaPhrase();
        category ManaSymbol();
        category Parenthetical();
        category QuotedText(QuotationStructure);
        category GrantedAbility(GrantedAbilityKind);
        category GrantedAbilitySeries(GrantedAbilityKind);
        category Mode(LabelKind);
        category Modes(ModeListLayout);
        category ModeContinuation();
        category NamedModes(ModeListLayout);
        category NamedModeContinuation();
        category TypeLine();
        category SupertypePrefix();
        category CardTypes();
        category CardTypeContinuation();
        category Subtypes();
        category SubtypeContinuation();
        category ClauseSeries(ClauseCoordinationHost, UnmarkedConjunctLicence, SourceComplementSelection);
        category FinitePredicateSeries(Polarity, number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category SecondaryPredicateSeries(ParticipialSupplementUse, form, ParticipialUse, InternalisedComplementPresent, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category NounPhraseSeries(NounPhraseStructure, FocusedUse, UnitBearing, number, person, CaseUse, Targeting, CoordinationKind, BareGenitiveHost, SubjectStructure, BarePrepositionUse, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent, WholeObjectSourcePresent, DeterminerAgreementNumber, ComparativePostmodifierUse, Definiteness);
        category AdverbPhrase(NominalAdjunctHost, VPFinalAdjunct, ClauseInitialAdjunct, DurationUse, UnmarkedConjunctLicence, FocusedUse);
        category InfinitiveComplement();
        category FiniteSelectedHead(Polarity, number, person, FrameUse, HeadCoordination, CliticHost, AuxiliaryEllipsisLicence, AuxiliaryProFormLicence, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category SecondarySelectedHead(form, FrameUse, HeadCoordination, AuxiliaryEllipsisLicence, AuxiliaryProFormLicence, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category FiniteSelectedHeadSeries(Polarity, number, person, FrameUse, CliticHost, AuxiliaryEllipsisLicence, AuxiliaryProFormLicence, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category SecondarySelectedHeadSeries(form, FrameUse, AuxiliaryEllipsisLicence, AuxiliaryProFormLicence, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category ManaPhraseSeries();
        category CardinalSeries(number, CoordinationKind);
        category AmountSeries();
        category MeasurePhraseSeries(MeasureKind, UnitBearing);
        category KeywordPhraseSeries();
        category QuotedTextSeries();
        category NominalSeries(
            ComparativeModifierPresent, AttributePremodifierHead, KeywordPremodifierHead, number, NominalAgreementNumber, countability, Targeting, NominalAdjunctClass, NominalDependentPresent,
            RelativisableAdjunctClass, SlashPremodifierUse,
            NumberTransparency, ObliqueNumber, CoordinationKind, BareGenitiveHost, DeterminerRequirement, MeasurePosition, ScalarMeasurementHeadUse, PropertyValuePosition, CardinalMeasurementUse,
            BarePrepositionUse, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation
        , SourceModifierPresent, WholeObjectSourcePresent, SubjectRelativeNumber, UnmarkedConjunctLicence);
        category AdjectivePhraseSeries(AdjectiveComplementClass, AdjectiveAttributiveOnlyUse, ImplicitComparativeUse, AdjectiveStructure, AdjectiveOrigin, AdjectivePremodifierUse, TargetingPremodifierUse, PremodifierCoordination);
        category PrepositionPhraseSeries(InterveningAdjunctLicence, ConnectiveAdjunctUse, NominalAdjunctHost, ClauseComplementBoundary, LocativeUse, PrepositionFunctionLicence, ComplementFunctionRestriction, ObliqueNumber, CoordinationKind,
            InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, ClauseInitialAdjunct, FocusedUse, SourceComplementUse, SourceModifierPresent, WholeObjectSourcePresent, SourceMarker, MatrixComplementRequirement, NominalExceptionComplementUse);
        category FocusedAdjunctMember(FocusedUse, NominalAdjunctHost, ClauseComplementBoundary, ConnectiveAdjunctUse, SourceMarker, SourceComplementUse);
        category FocusedAdjunctSeries(FocusedUse, NominalAdjunctHost, ClauseComplementBoundary, ConnectiveAdjunctUse, SourceMarker, SourceComplementUse);
        category FocusedAdjunctCoordination(FocusedUse, NominalAdjunctHost, ClauseComplementBoundary, ConnectiveAdjunctUse, SourceMarker, SourceComplementUse, PrepositionFunctionLicence, ComplementFunctionRestriction, PredicativeAdjunctUse, SelectedPrepositionUse, MatrixComplementRequirement, InternalisedComplementMarker);
        category AdverbPhraseSeries(NominalAdjunctHost, VPFinalAdjunct, ClauseInitialAdjunct, DurationUse, UnmarkedConjunctLicence, FocusedUse);
        category InfinitiveComplementSeries();
        category FrequencyPhraseSeries();
        category FiniteObjectGapSeries(Polarity, number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, ComparativeObjectGapLicence);
        category BareObjectGapSeries(PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, ComparativeObjectGapLicence);
        category PastParticipleObjectGapSeries(PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, ComparativeObjectGapLicence);
        category CoordinatedFiniteClause(GeneralCoordination);
        category FiniteClauseSeries(GeneralCoordination);
        category PredicativeComplementSeries(PredicativeKind, PredicativeAdjunctUse);
        category CorrelativeClauseSeries(ClauseCoordinationHost, CorrelativeCoordinator, UnmarkedConjunctLicence, SourceComplementSelection);
        category CorrelativeFinitePredicateSeries(Polarity, number, person, CorrelativeCoordinator, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category CorrelativeSecondaryVerbPhraseSeries(ParticipialSupplementUse, form, ParticipialUse, CorrelativeCoordinator,
            InternalisedComplementPresent, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category CorrelativeNounPhraseSeries(NounPhraseStructure, FocusedUse, UnitBearing, number, person, CaseUse, Targeting,
            CorrelativeCoordinator, CoordinationKind, BareGenitiveHost, SubjectStructure, BarePrepositionUse, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent, WholeObjectSourcePresent, DeterminerAgreementNumber, ComparativePostmodifierUse, Definiteness);
        category CorrelativePrepositionPhraseSeries(InterveningAdjunctLicence, ConnectiveAdjunctUse, NominalAdjunctHost, ClauseComplementBoundary, LocativeUse, PrepositionFunctionLicence, ComplementFunctionRestriction, ObliqueNumber, CoordinationKind,
            CorrelativeCoordinator, InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, ClauseInitialAdjunct, FocusedUse, SourceComplementUse, SourceModifierPresent, WholeObjectSourcePresent, SourceMarker, MatrixComplementRequirement, NominalExceptionComplementUse);
        category CorrelativeAdverbPhraseSeries(NominalAdjunctHost, VPFinalAdjunct, ClauseInitialAdjunct,
            DurationUse, CorrelativeCoordinator, FocusedUse);
        category CorrelativeManaPhraseSeries(CorrelativeCoordinator);
        category CorrelativeCardinalSeries(number, CorrelativeCoordinator, CoordinationKind);
        category CorrelativeAmountSeries(CorrelativeCoordinator);
        category CorrelativeMeasurePhraseSeries(UnitBearing, MeasureKind, CorrelativeCoordinator);
        category CorrelativeKeywordPhraseSeries(CorrelativeCoordinator);
        category CorrelativeQuotedTextSeries(CorrelativeCoordinator);
        category CorrelativeInfinitiveComplementSeries(CorrelativeCoordinator);
        category CorrelativeFiniteSelectedHeadSeries(Polarity, number, person, FrameUse,
            CorrelativeCoordinator, CliticHost, AuxiliaryEllipsisLicence, AuxiliaryProFormLicence, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category CorrelativeSecondarySelectedHeadSeries(form, FrameUse, CorrelativeCoordinator, AuxiliaryEllipsisLicence, AuxiliaryProFormLicence, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection);
        category CorrelativeAdjectivePhrase(AdjectiveAttributiveOnlyUse);
        category CorrelativeAdjectiveSeries(AdjectiveAttributiveOnlyUse, CorrelativeCoordinator);
        category CorrelativeFiniteObjectGapSeries(Polarity, number, person, CorrelativeCoordinator, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, ComparativeObjectGapLicence);
        category CorrelativeBareObjectGapSeries(CorrelativeCoordinator, PurposeAdjunctLicence, MatrixComplementLicence, SourceComplementSelection, ComparativeObjectGapLicence);
        category SelectedPrepositionHead(LocativeUse, PrepositionFunctionLicence, HeadCoordination,
            InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, SourceComplementUse, SourceModifierPresent, WholeObjectSourcePresent, SourceMarker);
        category SelectedPrepositionHeadSeries(LocativeUse, PrepositionFunctionLicence, InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, SourceComplementUse, SourceModifierPresent, WholeObjectSourcePresent, SourceMarker);
        category CorrelativeFiniteClauseSeries(CorrelativeCoordinator, GeneralCoordination);

        frame_category NounPhrase = AccusativePhrase;
        // PW-R1 (orchestrator ruling, 2026-10-10): a selected frame
        // takes the overt subject–predicate form, not reducible if able.
        frame_category VerblessClause = VerblessClause where VerblessClauseComplement = OvertSubject;
        frame_category SourceFreeObject = AccusativePhrase where SourceModifierPresent = No;
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
        instance SelectedPredicate<Result, Properties, BoundaryProperties = NoFeatures>: [
            (FinitePredicate, FiniteHeadAgreement, NoClauseComplementBoundary),
            (IrrealisPredicate, IrrealisHeadAgreement),
            (SecondaryVerbPhrase, OrdinarySecondaryHead, NoClauseComplementBoundary),
        ] {
            use Properties;
            use BoundaryProperties;
        }
        // NB1–NB4: analytic verbal negation belongs to a positive finite
        // auxiliary head; its ordinary selected Complement is unchanged.
        construction NegatedAuxiliaryHead: NegatedAuxiliaryHead {
            form [head: lexical(Verb), " ", negator: PrimaryNegator];
            export frame = head.frame;
            export PredicateFrameUse = head.PredicateFrameUse;
            require head.AuxiliaryUse = Yes;
            require head.Polarity = Positive;
            export Polarity = Negative;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        construction NegatedSelectedPredicate: FinitePredicate {
            require head.PredicateFrameUse = Yes;
            form [head: NegatedAuxiliaryHead, complements: selected_frame(head, Predicate)];
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use PredicateHostAgreement;
        }

        construction NegatedParticipialPredicate: FinitePredicate {
            form [head: NegatedAuxiliaryHead, complement: ParticipialComplement];
            require head.frame = ParticipialAuxiliary;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            form [head: CentralAdverbSelectedHead, complements: selected_frame(head, Predicate)];
            use PredicateHostAgreement;
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

        table coordinate_number(CoordinationKind, number, number) -> number {
            (Additive, Singular, Singular) => Plural,
            (Additive, Singular, Plural) => Plural,
            (Additive, Plural, Singular) => Plural,
            (Additive, Plural, Plural) => Plural,
            (Alternative, Singular, Singular) => Singular,
            (Alternative, Singular, Plural) => Plural,
            (Alternative, Plural, Singular) => Singular,
            (Alternative, Plural, Plural) => Plural,
        }

        // The ticket's corpus restriction retires single-referent additive
        // Subject agreement; each coordinate keeps its determiner-selection Number.
        table coordinated_nominal_agreement(CoordinationKind, NominalAgreementNumber)
            -> NominalAgreementNumber {
            (Additive, Head) => Plural, (Additive, Plural) => Plural,
            (Alternative, Head) => Head, (Alternative, Plural) => Plural,
        }
        table nominal_agreement_number(NominalAgreementNumber, number) -> number {
            (Head, Singular) => Singular, (Head, Plural) => Plural,
            (Plural, Singular) => Plural, (Plural, Plural) => Plural,
        }

        table nominal_relative_agreement(NominalAgreementNumber, number, number) -> Selection {
            (Head, Singular, Singular) => Yes, (Head, Plural, Plural) => Yes,
            (Plural, Singular, Plural) => Yes, (Plural, Plural, Plural) => Yes,
            (Plural, Singular, Singular) => Yes, (Plural, Plural, Singular) => Yes,
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
        }

        table nominative_case(CaseUse) -> CaseUse {
            (Common) => Nominative,
            (Nominative) => Nominative,
        }

        table accusative_case(CaseUse) -> CaseUse {
            (Common) => Accusative,
            (Accusative) => Accusative,
        }

        // Local complement selection, not the voice of an auxiliary chain.
        table complement_marker_concord(InternalisedComplementMarker,
            InternalisedComplementMarker) -> InternalisedComplementMarker {
            (Yes, Yes) => Yes, (No, Yes) => No, (Yes, No) => No, (No, No) => No,
        }
        table complement_presence(InternalisedComplementPresent,
            InternalisedComplementPresent) -> InternalisedComplementPresent {
            (Yes, Yes) => Yes, (No, Yes) => Yes, (Yes, No) => Yes, (No, No) => No,
        }
        table internalised_complement_presence(InternalisedComplementPresent,
            InternalisedComplementMarker) -> InternalisedComplementPresent {
            (Yes, Yes) => Yes, (No, Yes) => Yes, (Yes, No) => Yes, (No, No) => No,
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
        table coordinated_transparency(NumberTransparency, NumberTransparency)
            -> NumberTransparency {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
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
            export Polarity = head.Polarity;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        policy IrrealisHeadAgreement {
            require head.form = Irrealis;
            require head.finiteness = Finite;
            require head.Clitic = No;
            export number = head.number;
            export person = head.person;
        }

        policy SecondaryHeadForm {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export form = secondary_form(head.form);
        }

        policy NominalHeadFeatures {
            export NominalDependentPresent = Yes;
            export AttributePremodifierHead = head.AttributePremodifierHead;
            export KeywordPremodifierHead = head.KeywordPremodifierHead;
            export NominalAgreementNumber = head.NominalAgreementNumber;
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
            export ScalarMeasurementHeadUse = No;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export RelativisableAdjunctClass = head.RelativisableAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            export ObliqueNumber = head.ObliqueNumber;
        }

        policy ParticipialNominalHeadFeatures {
            export NominalDependentPresent = Yes;
            export AttributePremodifierHead = head.AttributePremodifierHead;
            export KeywordPremodifierHead = head.KeywordPremodifierHead;
            export NominalAgreementNumber = head.NominalAgreementNumber;
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
            export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export RelativisableAdjunctClass = head.RelativisableAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            export ObliqueNumber = head.ObliqueNumber;
        }

        policy NominalHeadProperties {
            export PropertyValuePosition = None;
        }

        policy PostmodifiedNominalProperties {
            export NominalDependentPresent = Yes;
            export AttributePremodifierHead = head.AttributePremodifierHead;
            export KeywordPremodifierHead = head.KeywordPremodifierHead;
            export NominalAgreementNumber = head.NominalAgreementNumber;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = nominal_source_modifier(head.SourceModifierPresent, modifier.SourceMarker);
            export WholeObjectSourcePresent = whole_object_source_modifier(
                head.WholeObjectSourcePresent, modifier.WholeObjectSourcePresent, modifier.SourceMarker);
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
            export ScalarMeasurementHeadUse = No;
            export PropertyValuePosition = None;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export RelativisableAdjunctClass = head.RelativisableAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
        }

        policy PredicateHostAgreement {
            export Polarity = head.Polarity;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export CliticHost = head.CliticHost;
            export number = head.number;
            export person = head.person;
        }

        policy FiniteGapHostProjection {
            export Polarity = head.Polarity;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export CliticHost = head.CliticHost;
            export number = head.number;
            export person = head.person;
            export ComparativeObjectGapLicence = head.ComparativeObjectGapLicence;
        }
        policy BareGapHostProjection {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export ComparativeObjectGapLicence = head.ComparativeObjectGapLicence;
        }
        policy FiniteGapHeadAgreement {
            export Polarity = head.Polarity;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export ComparativeObjectGapLicence = head.ComparativeObjectGapLicence;
        }

        policy PredicateHeadAgreement {
            export number = head.number;
            export person = head.person;
        }

        policy FiniteAuxiliaryAgreement {
            export Polarity = head.Polarity;
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export SourceComplementSelection = complement.SourceComplementSelection;
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        policy SharedAuxiliaryAgreement {
            export Polarity = head.Polarity;
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export SourceComplementSelection = complement.SourceComplementSelection;
            export CliticHost = head.CliticHost;
            export number = head.number;
            export person = head.person;
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
            export form = head.form;
            export ParticipialSupplementUse = No;
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy PurposeHostProjection {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
        }

        policy NoFeatures {
        }

        policy NominativeCase {
            export ScalarDenotation = head.ScalarDenotation;
            export SubjectStructure = head.SubjectStructure;
            export CaseUse = nominative_case(head.CaseUse);
        }

        policy AccusativeCase {
            export CaseUse = accusative_case(head.CaseUse);
        }

        policy SecondaryProjection {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export InternalisedComplementPresent = head.InternalisedComplementPresent;
            export ParticipialSupplementUse = head.ParticipialSupplementUse;
            export ParticipialUse = head.ParticipialUse;
            export form = secondary_form(head.form);
        }

        table internalised_adjunct_admission(InternalisedComplementPresent, InternalisedComplementMarker) -> Selection {
            (No, No) => Yes, (No, Yes) => Yes, (Yes, No) => Yes,
        }
        policy SecondaryPrepositionProjection {
            require internalised_adjunct_admission(head.InternalisedComplementPresent,
                modifier.InternalisedComplementMarker) = Yes;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export InternalisedComplementPresent = internalised_complement_presence(
                head.InternalisedComplementPresent, modifier.InternalisedComplementMarker);
            export ParticipialSupplementUse = head.ParticipialSupplementUse;
            export ParticipialUse = head.ParticipialUse;
            export form = secondary_form(head.form);
        }

        policy SecondaryAdjunctProjection {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export InternalisedComplementPresent = head.InternalisedComplementPresent;
            export ParticipialSupplementUse = head.ParticipialSupplementUse;
            export ParticipialUse = head.ParticipialUse;
            export form = head.form;
        }

        policy OrdinarySecondaryHead {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export form = secondary_form(head.form);
            export ParticipialSupplementUse = No;
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy OrdinarySelectedPredicate {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export form = head.form;
            export ParticipialSupplementUse = No;
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy NounPhraseHeadAgreement {
            export ScalarDenotation = head.ScalarDenotation;
            export SubjectStructure = Phrase;
            export BareGenitiveHost = head.BareGenitiveHost;
            export Targeting = head.Targeting;
            export number = nominal_agreement_number(head.NominalAgreementNumber, head.number);
            export person = Third;
            export CaseUse = Common;
        }

        policy ThirdPersonCommonCase {
            export SubjectStructure = Phrase;
            export person = Third;
            export CaseUse = Common;
        }

        policy UnmodifiedNominalProperties {
            export AttributePremodifierHead = head.AttributePremodifierHead;
            export KeywordPremodifierHead = head.KeywordPremodifierHead;
            export NominalAgreementNumber = Head;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
            export ScalarMeasurementHeadUse = head.ScalarMeasurementHeadUse;
            export PropertyValuePosition = head.PropertyValuePosition;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export number = head.number;
            export countability = head.countability;
            export Targeting = No;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export RelativisableAdjunctClass = head.RelativisableAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            export ObliqueNumber = None;
        }

        policy PrepositionHeadPermissions {
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export NominalComplementMarker = head.NominalComplementMarker;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export LocativeUse = head.LocativeUse;
        }

        table coordinated_adjective_complement_class(AdjectiveComplementClass,
            AdjectiveComplementClass) -> AdjectiveComplementClass {
            (ColorWord, ColorWord) => ColorWord, (Manner, Manner) => Manner, (_, _) => None,
        }
        policy AdjectiveAttributiveConcord<Right, Source> {
            export AdjectiveAttributiveOnlyUse = adjective_attributive_concord(
                left.AdjectiveAttributiveOnlyUse, Right.AdjectiveAttributiveOnlyUse);
        }
        policy AdjectiveStructureMerge<Right, Source> {
            export AdjectiveComplementClass = coordinated_adjective_complement_class(
                left.AdjectiveComplementClass, Right.AdjectiveComplementClass);
            export AdjectiveAttributiveOnlyUse = adjective_attributive_concord(
                left.AdjectiveAttributiveOnlyUse, Right.AdjectiveAttributiveOnlyUse);
            export ImplicitComparativeUse = implicit_comparative_concord(
                left.ImplicitComparativeUse, Right.ImplicitComparativeUse);
            export AdjectivePremodifierUse = coordinated_adjective_premodifier(
                left.AdjectivePremodifierUse, Right.AdjectivePremodifierUse);
            export PremodifierCoordination = Yes;
            export AdjectiveOrigin = coordinated_adjective_origin(left.AdjectiveOrigin, Right.AdjectiveOrigin);
            export AdjectiveStructure = coordinated_adjective_structure(left.AdjectiveStructure,
                Right.AdjectiveStructure);
            export TargetingPremodifierUse = coordinated_targeting_premodifier(
                left.TargetingPremodifierUse, Right.TargetingPremodifierUse);
        }

        policy PremodifierConcord<Right, Source> {
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

        policy AdverbPermissions<Right, Source> {
            export NominalAdjunctHost = Yes;
            require ordinary_adjunct_coordination(left.FocusedUse, Right.FocusedUse) = Yes;
            export FocusedUse = coordinated_focus(left.FocusedUse, Right.FocusedUse);
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct,
                Right.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct,
                Right.ClauseInitialAdjunct);
            export DurationUse = coordinated_duration_use(left.DurationUse, Right.DurationUse);
        }

        policy CardinalAgreement<Right, Source> {
            export number = cardinal_coordinate_number(Source.CoordinationKind, left.number,
                Right.number);
        }

        policy CardinalListEnd<Right, Source> {
            export CoordinationKind = Source.CoordinationKind;
            export number = cardinal_coordinate_number(Source.CoordinationKind, left.number,
                Right.number);
        }

        policy CoordinatedMeasureKind<Right, Source> {
            export UnitBearing = arithmetic_unit_bearing(left.UnitBearing, Right.UnitBearing);
            agree left.MeasureKind = Right.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        // R15d/R15e: an asyndetic shared-head group cannot resegment an
        // unmarked member of a coordinator-ending Nominal list.
        policy UnmarkedNominalConjunct<Right, Source> {
            require left.UnmarkedConjunctLicence = Yes;
        }
        policy UnmarkedNominalSeriesConjunct<Right, Source> {
            require left.UnmarkedConjunctLicence = Yes;
            export CoordinationKind = Source.CoordinationKind;
        }

        policy CoordinatorKindSummary<Right, Source> {
            export CoordinationKind = Source.CoordinationKind;
        }

        policy FiniteConcord<Right, Source> {
            export Polarity = left.Polarity;
            export PurposeAdjunctLicence = coordinated_purpose_licence(
                left.PurposeAdjunctLicence, Right.PurposeAdjunctLicence);
            export MatrixComplementLicence = matrix_host_concord(
                left.MatrixComplementLicence, Right.MatrixComplementLicence);
            export SourceComplementSelection = source_selection_union(
                left.SourceComplementSelection, Right.SourceComplementSelection);
            require Right.CliticHost = Free;
            export CliticHost = left.CliticHost;
            agree left.number = Right.number;
            agree left.person = Right.person;
            export number = left.number;
            export person = left.person;
        }

        policy FiniteGapConcord<Right, Source> {
            export Polarity = left.Polarity;
            export PurposeAdjunctLicence = coordinated_purpose_licence(
                left.PurposeAdjunctLicence, Right.PurposeAdjunctLicence);
            export MatrixComplementLicence = matrix_host_concord(
                left.MatrixComplementLicence, Right.MatrixComplementLicence);
            export SourceComplementSelection = source_selection_union(
                left.SourceComplementSelection, Right.SourceComplementSelection);
            require Right.CliticHost = Free;
            export CliticHost = left.CliticHost;
            agree left.number = Right.number;
            agree left.person = Right.person;
            export number = left.number;
            export person = left.person;
            export ComparativeObjectGapLicence = comparative_gap_concord(
                left.ComparativeObjectGapLicence, Right.ComparativeObjectGapLicence);
        }
        policy BareGapConcord<Right, Source> {
            export PurposeAdjunctLicence = coordinated_purpose_licence(
                left.PurposeAdjunctLicence, Right.PurposeAdjunctLicence);
            export MatrixComplementLicence = matrix_host_concord(
                left.MatrixComplementLicence, Right.MatrixComplementLicence);
            export SourceComplementSelection = source_selection_union(
                left.SourceComplementSelection, Right.SourceComplementSelection);
            export ComparativeObjectGapLicence = comparative_gap_concord(
                left.ComparativeObjectGapLicence, Right.ComparativeObjectGapLicence);
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
            (Additive, None, None) => None, (Additive, None, Single) => None,
            (Additive, None, First) => None, (Additive, None, Second) => None,
            (Additive, None, Pair) => None, (Additive, Single, None) => None,
            (Additive, Single, Single) => None, (Additive, Single, First) => None,
            (Additive, Single, Second) => None, (Additive, Single, Pair) => None,
            (Additive, First, None) => None, (Additive, First, Single) => None,
            (Additive, First, First) => None, (Additive, First, Second) => Pair,
            (Additive, First, Pair) => None, (Additive, Second, None) => None,
            (Additive, Second, Single) => None, (Additive, Second, First) => None,
            (Additive, Second, Second) => None, (Additive, Second, Pair) => None,
            (Additive, Pair, None) => None, (Additive, Pair, Single) => None,
            (Additive, Pair, First) => None, (Additive, Pair, Second) => None,
            (Additive, Pair, Pair) => None, (Alternative, None, None) => None,
            (Alternative, None, Single) => None, (Alternative, None, First) => None,
            (Alternative, None, Second) => None, (Alternative, None, Pair) => None,
            (Alternative, Single, None) => None, (Alternative, Single, Single) => None,
            (Alternative, Single, First) => None, (Alternative, Single, Second) => None,
            (Alternative, Single, Pair) => None, (Alternative, First, None) => None,
            (Alternative, First, Single) => None, (Alternative, First, First) => None,
            (Alternative, First, Second) => None, (Alternative, First, Pair) => None,
            (Alternative, Second, None) => None, (Alternative, Second, Single) => None,
            (Alternative, Second, First) => None, (Alternative, Second, Second) => None,
            (Alternative, Second, Pair) => None, (Alternative, Pair, None) => None,
            (Alternative, Pair, Single) => None, (Alternative, Pair, First) => None,
            (Alternative, Pair, Second) => None, (Alternative, Pair, Pair) => None,
            (Adversative, None, None) => None, (Adversative, None, Single) => None,
            (Adversative, None, First) => None, (Adversative, None, Second) => None,
            (Adversative, None, Pair) => None, (Adversative, Single, None) => None,
            (Adversative, Single, Single) => None, (Adversative, Single, First) => None,
            (Adversative, Single, Second) => None, (Adversative, Single, Pair) => None,
            (Adversative, First, None) => None, (Adversative, First, Single) => None,
            (Adversative, First, First) => None, (Adversative, First, Second) => None,
            (Adversative, First, Pair) => None, (Adversative, Second, None) => None,
            (Adversative, Second, Single) => None, (Adversative, Second, First) => None,
            (Adversative, Second, Second) => None, (Adversative, Second, Pair) => None,
            (Adversative, Pair, None) => None, (Adversative, Pair, Single) => None,
            (Adversative, Pair, First) => None, (Adversative, Pair, Second) => None,
            (Adversative, Pair, Pair) => None,
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
        table property_value_kind(PropertyValuePosition, MeasureKind) -> Selection {
            (Single, Scalar) => Yes, (First, Scalar) => Yes, (Second, Scalar) => Yes,
            (Pair, Pair) => Yes,
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
        policy NominalConcord<Right, Source> {
            export ComparativeModifierPresent = No;
            export UnmarkedConjunctLicence = Yes;
            export SubjectRelativeNumber = relative_number_concord(left.SubjectRelativeNumber, Right.SubjectRelativeNumber);
            export AttributePremodifierHead = coordinated_attribute_premodifier_head(
                left.AttributePremodifierHead, Right.AttributePremodifierHead);
            export KeywordPremodifierHead = coordinated_keyword_premodifier_head(
                left.KeywordPremodifierHead, Right.KeywordPremodifierHead);
            export NominalAgreementNumber = coordinated_nominal_agreement(
                Source.CoordinationKind, Right.NominalAgreementNumber);
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
            export ScalarDenotation = coordinated_scalar_denotation(left.ScalarDenotation, Right.ScalarDenotation);
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export SourceModifierPresent = source_modifier_presence(left.SourceModifierPresent,
                Right.SourceModifierPresent);
            export WholeObjectSourcePresent = whole_object_source_presence(left.WholeObjectSourcePresent,
                Right.WholeObjectSourcePresent);
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            export CardinalMeasurementUse = coordinated_cardinal_measurement(
                left.CardinalMeasurementUse, Right.CardinalMeasurementUse);
            export MeasurePosition = coordinated_measure_position(
                left.MeasurePosition, Right.MeasurePosition);
            export ScalarMeasurementHeadUse = No;
            export PropertyValuePosition = coordinated_property_value_position(
                Source.CoordinationKind, left.PropertyValuePosition, Right.PropertyValuePosition);
            export DeterminerRequirement = coordinated_determiner_requirement(
                left.DeterminerRequirement, Right.DeterminerRequirement);
            export BareGenitiveHost = No;
            export NumberTransparency = coordinated_transparency(left.NumberTransparency,
                Right.NumberTransparency);
            export ObliqueNumber = coordinated_oblique(Source.CoordinationKind,
                left.ObliqueNumber, Right.ObliqueNumber);
            agree left.Targeting = Right.Targeting;
            agree left.countability = Right.countability;
            agree left.number = Right.number;
            export Targeting = left.Targeting;
            export NominalDependentPresent = nominal_dependent_presence(left.NominalDependentPresent,
                Right.NominalDependentPresent);
            export NominalAdjunctClass = nominal_adjunct_concord(left.NominalAdjunctClass,
                Right.NominalAdjunctClass);
            export RelativisableAdjunctClass = coordinated_relativisable_adjunct_class(
                left.RelativisableAdjunctClass, Right.RelativisableAdjunctClass);
            export SlashPremodifierUse = slash_premodifier_concord(left.SlashPremodifierUse,
                Right.SlashPremodifierUse);
            export countability = left.countability;
            export number = left.number;
        }

        table coordinated_scalar_denotation(ScalarDenotation, ScalarDenotation) -> ScalarDenotation {
            (Yes, Yes) => Yes, (No, No) => No, (No, Yes) => No, (Yes, No) => No,
        }

        table coordinated_targeting(Targeting, Targeting) -> Targeting {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        table coordinated_determiner_agreement(DeterminerAgreementNumber, DeterminerAgreementNumber)
            -> DeterminerAgreementNumber {
            (Singular, Singular) => Singular, (Singular, Nominal) => Nominal,
            (Nominal, Singular) => Nominal, (Nominal, Nominal) => Nominal,
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
        }
        policy NounCoordinationAgreement<Right, Source> {
            export NounPhraseStructure = Other;
            export FocusedUse = coordinated_focus(left.FocusedUse, Right.FocusedUse);
            export Definiteness = Unspecified;
            export UnitBearing = arithmetic_unit_bearing(left.UnitBearing, Right.UnitBearing);
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = coordinated_determiner_agreement(
                left.DeterminerAgreementNumber, Right.DeterminerAgreementNumber);
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
            export ScalarDenotation = coordinated_scalar_denotation(left.ScalarDenotation, Right.ScalarDenotation);
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export SourceModifierPresent = source_modifier_presence(left.SourceModifierPresent,
                Right.SourceModifierPresent);
            export WholeObjectSourcePresent = whole_object_source_presence(left.WholeObjectSourcePresent,
                Right.WholeObjectSourcePresent);
            agree left.BarePrepositionUse = Right.BarePrepositionUse;
            export BarePrepositionUse = left.BarePrepositionUse;
            export SubjectStructure = Phrase;
            export BareGenitiveHost = No;
            export Targeting = coordinated_targeting(left.Targeting, Right.Targeting);
            export number = determined_np_coordinate_number(Source.CoordinationKind,
                left.number, Right.number, left.DeterminerAgreementNumber, Right.DeterminerAgreementNumber);
            export person = coordinate_person(Source.CoordinationKind, left.person,
                Right.person);
            export CaseUse = common_case(left.CaseUse, Right.CaseUse);
        }

        policy PredicativeListEnd<Right, Source> {
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
            export PredicativeKind = combined_predicative_kind(left.PredicativeKind,
                Right.PredicativeKind);
        }

        policy DepictiveListEnd<Right, Source> {
            export DepictiveKind = combined_depictive_kind(left.DepictiveKind, Right.DepictiveKind);
        }

        policy UnlikeDepictives<Right, Source> {
            export DepictiveKind = unlike_depictive_kind(left.DepictiveKind, Right.DepictiveKind);
        }

        policy UnlikePredicatives<Right, Source> {
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
            export PredicativeKind = unlike_predicative_kind(left.PredicativeKind,
                Right.PredicativeKind);
        }

        table nominal_exception_concord(NominalExceptionComplementUse, NominalExceptionComplementUse)
            -> NominalExceptionComplementUse {
            (No, No) => No, (No, Yes) => No, (Yes, No) => No, (Yes, Yes) => Yes,
        }
        policy ObliquePrepositionConcord<Right, Source> {
            require ordinary_adjunct_coordination(left.FocusedUse, Right.FocusedUse) = Yes;
            export InterveningAdjunctLicence = coordinated_intervening_adjunct(
                left.InterveningAdjunctLicence, Right.InterveningAdjunctLicence);
            export ConnectiveAdjunctUse = connective_adjunct_concord(
                left.ConnectiveAdjunctUse, Right.ConnectiveAdjunctUse);
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = coordinated_clause_complement_boundary(
                left.ClauseComplementBoundary, Right.ClauseComplementBoundary);
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
            export NominalExceptionComplementUse = nominal_exception_concord(
                left.NominalExceptionComplementUse, Right.NominalExceptionComplementUse);
            export MatrixComplementRequirement = matrix_complement_concord(
                left.MatrixComplementRequirement, Right.MatrixComplementRequirement);
            export ComplementFunctionRestriction = coordinated_complement_function_restriction(
                left.ComplementFunctionRestriction, Right.ComplementFunctionRestriction,
                left.PrepositionFunctionLicence, Right.PrepositionFunctionLicence);
            export SourceMarker = coordinated_source_marker(left.SourceMarker, Right.SourceMarker);
            export SourceComplementUse = coordinated_source_complement_use(
                left.SourceComplementUse, Right.SourceComplementUse);
            export FocusedUse = coordinated_focus(left.FocusedUse, Right.FocusedUse);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct,
                Right.ClauseInitialAdjunct);
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export SourceModifierPresent = source_modifier_presence(left.SourceModifierPresent,
                Right.SourceModifierPresent);
            export WholeObjectSourcePresent = whole_object_source_presence(left.WholeObjectSourcePresent,
                Right.WholeObjectSourcePresent);
            export NominalComplementMarker = None;
            export InternalisedComplementMarker = complement_marker_concord(
                left.InternalisedComplementMarker, Right.InternalisedComplementMarker);
            export PrepositionFunctionLicence = coordinated_preposition_licence(left.PrepositionFunctionLicence, Right.PrepositionFunctionLicence);
            export LocativeUse = coordinated_locative_use(left.LocativeUse, Right.LocativeUse);
            export ObliqueNumber = coordinated_oblique(Source.CoordinationKind,
                left.ObliqueNumber, Right.ObliqueNumber);
        }

        policy PrepositionPermissions<Right, Source> {
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
            export SourceMarker = coordinated_source_marker(left.SourceMarker, Right.SourceMarker);
            export SourceComplementUse = coordinated_source_complement_use(
                left.SourceComplementUse, Right.SourceComplementUse);
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export SourceModifierPresent = source_modifier_presence(left.SourceModifierPresent,
                Right.SourceModifierPresent);
            export WholeObjectSourcePresent = whole_object_source_presence(left.WholeObjectSourcePresent,
                Right.WholeObjectSourcePresent);
            export NominalComplementMarker = None;
            export InternalisedComplementMarker = complement_marker_concord(
                left.InternalisedComplementMarker, Right.InternalisedComplementMarker);
            export PrepositionFunctionLicence = coordinated_preposition_licence(left.PrepositionFunctionLicence, Right.PrepositionFunctionLicence);
            export LocativeUse = coordinated_locative_use(left.LocativeUse, Right.LocativeUse);
        }

        policy SecondaryConcord<Right, Source> {
            export PurposeAdjunctLicence = coordinated_purpose_licence(
                left.PurposeAdjunctLicence, Right.PurposeAdjunctLicence);
            export MatrixComplementLicence = matrix_host_concord(
                left.MatrixComplementLicence, Right.MatrixComplementLicence);
            export SourceComplementSelection = source_selection_union(
                left.SourceComplementSelection, Right.SourceComplementSelection);
            agree left.form = Right.form;
            export form = left.form;
        }

        policy SecondaryConjunctProperties<Right, Source> {
            export PurposeAdjunctLicence = coordinated_purpose_licence(
                left.PurposeAdjunctLicence, Right.PurposeAdjunctLicence);
            export MatrixComplementLicence = matrix_host_concord(
                left.MatrixComplementLicence, Right.MatrixComplementLicence);
            export SourceComplementSelection = source_selection_union(
                left.SourceComplementSelection, Right.SourceComplementSelection);
            agree left.form = Right.form;
            export form = left.form;
            export InternalisedComplementPresent = complement_presence(
                left.InternalisedComplementPresent, Right.InternalisedComplementPresent);
            export ParticipialSupplementUse = coordinated_participial_supplement(
                left.ParticipialSupplementUse, Right.ParticipialSupplementUse);
            export ParticipialUse = coordinated_participial_use(left.ParticipialUse,
                Right.ParticipialUse);
        }

        policy SelectedFrameConcord<Right, Source> {
            export AuxiliaryProFormLicence = auxiliary_pro_form_concord(
                left.AuxiliaryProFormLicence, Right.AuxiliaryProFormLicence);
            export AuxiliaryEllipsisLicence = auxiliary_ellipsis_concord(
                left.AuxiliaryEllipsisLicence, Right.AuxiliaryEllipsisLicence);
            agree left.FrameUse = Right.FrameUse;
            export FrameUse = left.FrameUse;
        }

        policy SharedFrameConcord<Right, Source> {
            export AuxiliaryProFormLicence = auxiliary_pro_form_concord(
                left.AuxiliaryProFormLicence, Right.AuxiliaryProFormLicence);
            export AuxiliaryEllipsisLicence = auxiliary_ellipsis_concord(
                left.AuxiliaryEllipsisLicence, Right.AuxiliaryEllipsisLicence);
            agree left.FrameUse = Right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        policy SharedHeadStatus<Right, Source> {
            export HeadCoordination = Yes;
        }

        policy UnmarkedConjunctHead<Right, Source> {
            export UnmarkedConjunctLicence = Yes;
        }

        policy MarkedConjunctSeries<Right, Source> {
            require Right.UnmarkedConjunctLicence = Yes;
            export UnmarkedConjunctLicence = Yes;
        }

        policy LeadingConjunct<Right, Source> {
            export UnmarkedConjunctLicence = left.UnmarkedConjunctLicence;
        }

        policy UnmarkedConjunct<Right, Source> {
            require Right.UnmarkedConjunctLicence = Yes;
            export UnmarkedConjunctLicence = left.UnmarkedConjunctLicence;
        }

        policy NoClauseComplementBoundary {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
        }

        policy CoordinatedClauseHost<Right, Source> {
            export SourceComplementSelection = source_selection_union(
                left.SourceComplementSelection, Right.SourceComplementSelection);
            export ClauseCoordinationHost = Coordinated;
        }

        policy ClauseCoordinationProperties<Right, Source> {
            export GeneralCoordination = Source.GeneralCoordination;
        }

        policy GeneralCoordinatorDistribution<Right, Source> {
            require Source.GeneralCoordination = Yes;
        }

        schema CoordinationSeriesEnd {
            form [left: node, ", ", coordinator: lexical(Coordinator), " ", right: node];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        schema CoordinationSeriesContinuation {
            form [left: node, ", ", rest: node];
        }

        schema SerialCoordination {
            form [left: node, ", ", rest: node];
        }

        separator coordination_separator(CommaCoordination) {
            (No) => " ", (Yes) => ", ",
        }
        schema Coordination {
            form [left: node, coordination_separator(coordinator.CommaCoordination),
                coordinator: lexical(Coordinator), " ", right: node];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        schema CasePhrase {
            form [head: node];
            use PredicateHeadAgreement;
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
            export form = head.form;
            export ParticipialSupplementUse = No;
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
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
            export PurposeAdjunctLicence = predicate.PurposeAdjunctLicence;
            export MatrixComplementLicence = predicate.MatrixComplementLicence;
            export SourceComplementSelection = predicate.SourceComplementSelection;
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
        instance LicensedMatrixComplementPredicate<Result, Head: head, Properties>: [
            (FinitePredicate, Self, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, SecondaryProjection),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None; use Properties; }

        construction MatrixComplementPredicate: SecondaryVerbPhrase {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            form [head: SecondaryVerbPhrase, " ", modifier: PrepositionPhrase];
            require modifier.MatrixComplementRequirement = Internalised;
            require head.ParticipialUse = BarePassive;
            require head.InternalisedComplementPresent = No;
            export form = head.form;
            export ParticipialSupplementUse = head.ParticipialSupplementUse;
            export ParticipialUse = head.ParticipialUse;
            export InternalisedComplementPresent = Yes;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
        }

        construction InternalisedComplementPredicate: SecondaryVerbPhrase {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            form [head: SecondaryVerbPhrase, " ", modifier: PrepositionPhrase];
            require head.ParticipialUse = BarePassive;
            require head.InternalisedComplementPresent = No;
            require modifier.InternalisedComplementMarker = Yes;
            export form = head.form;
            export ParticipialSupplementUse = head.ParticipialSupplementUse;
            export ParticipialUse = head.ParticipialUse;
            export InternalisedComplementPresent = Yes;
        }

        policy NoComplementProperties {
        }

        policy OvertComplementRealization {
            export AuxiliaryComplementRealization = Overt;
        }

        schema OvertComplement {
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
            export AuxiliaryProFormLicence = head.AuxiliaryProFormLicence;
            export AuxiliaryEllipsisLicence = head.AuxiliaryEllipsisLicence;
            // An uncomplemented coordinate head requires the strong form.
            require head.Clitic = No;
            form [head: lexical(Verb)];
            export FrameUse = selected_frame_use(head.frame);
            export HeadCoordination = No;
        }

        schema SharedObjectComplement {
            form [head: node, " ", object: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Object;
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
            form [marker: lexical(Determinative), " ", left: node, " ",
                coordinator: lexical(Coordinator), " ", right: node];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
        }

        schema CorrelativeSeriesEnd {
            form [left: node, ", ", coordinator: lexical(Coordinator), " ", right: node];
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
        }

        schema CorrelativeSeriesContinuation {
            form [left: node, ", ", rest: node];
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
        }

        schema EitherSerialCoordination {
            form [marker: lexical(Determinative), " ", left: node, ", ", rest: node];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
        }

        schema BothCoordination {
            form [marker: lexical(Determinative), " ", left: node, " ",
                coordinator: lexical(Coordinator), " ", right: node];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
        }

        schema NeitherCoordination {
            form [marker: lexical(Determinative), " ", left: node, " ",
                coordinator: lexical(Coordinator), " ", right: node];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
        }

        schema NeitherSerialCoordination {
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
            export LabelKind = None;
            export SentenceLineList = No;
            form [body: Paragraph] export AbilityTermination = Complete;
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
            export QuotationStructure = Simple;
            form ["\"", text: Document, "\""];
        }

        construction QuotedClause: QuotedText {
            export QuotationStructure = Simple;
            boundary Interior;
            form ["\"", clause: Clause, "\""];
            form ["\"", clause: Ability, "\""]
                require clause.AbilityTermination = Unterminated
                require clause.terminal_punctuation = None;
        }

        construction QuotedKeyword: QuotedText {
            export QuotationStructure = Simple;
            boundary Interior;
            form ["\"", keyword: KeywordPhrase, "\""];
            form ["\"", keyword: KeywordPhrase, ".\""];
        }

        construction KeywordGrantedAbility: GrantedAbility {
            form [keyword: KeywordPhrase];
            export GrantedAbilityKind = Keyword;
        }

        construction QuotedGrantedAbility: GrantedAbility {
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
        policy GrantedAbilityConcord<Right, Source> {
            export GrantedAbilityKind = combined_ability_kind(left.GrantedAbilityKind,
                Right.GrantedAbilityKind);
        }
        policy GrantedAbilityCoordination<Right, Source> {
            export GrantedAbilityKind = combined_ability_kind(left.GrantedAbilityKind,
                Right.GrantedAbilityKind);
            require quoted_ability_combination(left.GrantedAbilityKind,
                Right.GrantedAbilityKind) = Yes;
        }
        policy CoordinatedQuotation<Right, Source> {
            export QuotationStructure = Coordinated;
        }

        construction ActivatedAbility: Ability {
            export LabelKind = None;
            export SentenceLineList = No;
            form [cost: Cost, ": ", body: Paragraph] export AbilityTermination = Complete;
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
            form [first: CostComponent, rest: repeat(CostContinuation, "")];
            export CostComponentForm = final_cost_component(first.CostComponentForm,
                last(rest).CostComponentForm);
        }

        construction CostContinuation: CostContinuation {
            form [", ", component: CostComponent];
            export CostComponentForm = component.CostComponentForm;
        }

        construction ActionCost: CostComponent {
            boundary Initial;
            form [action: BarePredicate];
            export CostComponentForm = Action;
        }

        construction SymbolCost: CostComponent {
            form [symbols: CostSymbols];
            export CostComponentForm = Symbols;
        }

        // Oracle document notation, under the signed-numeral-cost project analysis.
        construction SignedScalarCost: Cost {
            form [value: ScalarComponent];
            require loyalty_sign(value.ScalarSign) = Yes;
            export CostComponentForm = Symbols;
        }

        construction ZeroScalarCost: Cost {
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
        policy QualityPrepositionConcord<Right, Source> {
            agree left.KeywordMarker = Right.KeywordMarker;
            export KeywordMarker = left.KeywordMarker;
            export KeywordQualityNumber = coordinated_quality(left.KeywordQualityNumber,
                Right.KeywordQualityNumber);
        }
        construction AdjectivalKeywordQuality: KeywordQuality {
            form [phrase: AdjectivePhrase];
            require phrase.AdjectiveAttributiveOnlyUse = No;
            require phrase.AdjectiveStructure = Simple;
            export KeywordQualityNumber = Any;
        }
        construction NominalKeywordQuality: KeywordQuality {
            form [phrase: Nominal];
            require phrase.Targeting = No;
            export KeywordQualityNumber = quality_number(phrase.number);
        }
        construction NounPhraseKeywordQuality: KeywordQuality {
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
        construction QualityCostKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", quality: KeywordQualityPreposition,
                keyword_parameter_separator(head.KeywordParameterSeparator), cost: CostSymbols];
            require head.KeywordParameterClass = QualityCost;
            require head.KeywordSeparator = Space;
            agree head.KeywordMarker = quality.KeywordMarker;
            require compatible_quality(head.KeywordQualityNumber,
                quality.KeywordQualityNumber) = Yes;
        }
        category ClausalKeywordPayload(KeywordParameterClass);
        construction AbilityKeywordPayload: ClausalKeywordPayload {
            form [body: Ability];
            require body.LabelKind = None;
            require body.AbilityTermination = Complete;
            export KeywordParameterClass = Ability;
        }
        construction ConditionKeywordPayload: ClausalKeywordPayload {
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
            require head.KeywordParameterClass = CostPowerToughness;
            require head.KeywordSeparator = Space;
        }

        construction RemindedKeyword: KeywordPhrase {
            form [keyword: KeywordPhrase, " ", reminder: Parenthetical];
        }

        construction Mode: Mode {
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
            boundary Initial;
            form [clause: Clause, sentence_terminal(clause.terminal_punctuation)]
                export AbilityTermination = Complete;
            form [clause: Clause] require clause.terminal_punctuation = None
                export AbilityTermination = Unterminated;
        }

        // Project analysis: a variable-defining PP supplements the completed
        // clause or keyword label once. CGEL Ch. 11 §4.8, p. 971 supports
        // content-clause selection, not this variable-defining application.
        schema Supplementation {
            boundary Initial;
            form [anchor: node, ", ", supplement: PrepositionPhrase, "."];
            require licence_supplement(supplement.PrepositionFunctionLicence) = Yes;
            require anchor.terminal_punctuation = None;
        }
        policy UnlabelledSupplementation {
            export LabelKind = None;
            export SentenceLineList = No;
            export AbilityTermination = Complete;
        }
        policy CompleteSentence {
            export AbilityTermination = Complete;
        }
        instance Supplementation<Result, Anchor: anchor, Properties>: [
            (Sentence, Clause, CompleteSentence),
            (Ability, KeywordPhrase, UnlabelledSupplementation),
        ] {
            use Properties;
        }

        // HR4: an embedded participial Supplement has a Clause or action
        // Cost anchor. Unlike terminal supplementation, it has no initial
        // capitalization boundary and leaves punctuation to its container.
        schema ParticipialSupplementation {
            form [anchor: node, ", ", supplement: SecondaryVerbPhrase];
            require supplement.ParticipialUse = BarePassive;
            require supplement.form = PastParticiple;
            require supplement.ParticipialSupplementUse = Yes;
            require anchor.terminal_punctuation = None;
        }
        policy SourceSelectionAnchor {
            export SourceComplementSelection = anchor.SourceComplementSelection;
        }
        policy ImperativeSupplement {
            export SourceComplementSelection = anchor.SourceComplementSelection;
            export ClauseCoordinationHost = anchor.ClauseCoordinationHost;
            export UnmarkedConjunctLicence = anchor.UnmarkedConjunctLicence;
        }
        policy CostSupplement {
            require anchor.CostComponentForm = Action;
            export CostComponentForm = Action;
        }
        instance ParticipialSupplementation<Result, Anchor: anchor, Properties>: [
            (FiniteClause, FiniteClause, SourceSelectionAnchor),
            (Clause, ImperativeClause, ImperativeSupplement),
            (CostComponent, CostComponent, CostSupplement),
        ] {
            use Properties;
        }

        construction Declarative: Clause {
            export SourceComplementSelection = clause.SourceComplementSelection;
            export ClauseCoordinationHost = Simple;
            form [clause: FiniteClause];
            export UnmarkedConjunctLicence = Yes;
        }

        // HR4: a disjoint Clause anchor keeps the finite and imperative
        // supplementation routes from duplicating a declarative Reading.
        construction SupplementAnchorImperative: ImperativeClause {
            export SourceComplementSelection = predicate.SourceComplementSelection;
            export ClauseCoordinationHost = Simple;
            form [predicate: BarePredicate];
            export UnmarkedConjunctLicence = Yes;
        }

        construction Imperative: Clause {
            export SourceComplementSelection = predicate.SourceComplementSelection;
            export ClauseCoordinationHost = Simple;
            form [predicate: BarePredicate];
            export UnmarkedConjunctLicence = Yes;
        }

        // CGEL Ch. 8 §20.1, p. 781: without a Subject, front and
        // pre-verbal central position are not overtly distinct. The licensed
        // central analysis of Oracle imperative also is the project application.
        construction CentralAdverbImperative: Clause {
            export SourceComplementSelection = predicate.SourceComplementSelection;
            export ClauseCoordinationHost = Simple;
            form [modifier: lexical(Adverb), " ", predicate: BarePredicate];
            require central_imperative(modifier.CentralAdjunctLicence) = Yes;
            export UnmarkedConjunctLicence = Yes;
        }


        table finite_subject_number(number, number) -> Selection {
            (Singular, Singular) => Yes, (Plural, Plural) => Yes,
        }
        table finite_subject_person(person, person) -> Selection {
            (First, First) => Yes, (Second, Second) => Yes, (Third, Third) => Yes,
        }
        construction FiniteClause: FiniteClause {
            export SourceComplementSelection = predicate.SourceComplementSelection;
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: FinitePredicate]
                require finite_subject_number(subject.number, predicate.number) = Yes
                require finite_subject_person(subject.person, predicate.person) = Yes
                require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
            form [subject: DummyPronoun, clitic_separator(predicate.CliticHost),
                predicate: ExistentialPredicate];
        }

        // CGEL Ch. 5 §9.2, p. 413: implicit-partitive adjuncts quantify
        // the Subject. Clause attachment scopes over the whole Predicate.
        construction QuantifiedClause: FiniteClause {
            export SourceComplementSelection = predicate.SourceComplementSelection;
            form [subject: NominativePhrase, " ", quantifier: lexical(Determinative),
                " ", predicate: FinitePredicate];
            require quantifier.QuantificationalAdjunctUse = Yes;
            require subject.number = Plural;
            require predicate.CliticHost = Free;
            agree subject.number = predicate.number;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            form [head: QuantifiedAuxiliaryHead, complements: selected_frame(head, Predicate)];
            use PredicateHostAgreement;
        }
        construction QuantifiedScalarLocation: ScalarLocationPredicate {
            form [head: QuantifiedAuxiliaryHead, complements: selected_frame(head, ScalarLocation)];
            use PredicateHostAgreement;
        }

        construction QuantifiedBareAuxiliaryPredicate: FinitePredicate {
            require licensed_auxiliary_pro_form(head.AuxiliaryProFormLicence,
                complement.AuxiliaryComplementRealization) = Yes;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
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

        // CGEL Ch. 3 §§1.7–1.8.1, pp. 86–88: finite irrealis is non-tensed
        // and distinct from the ordinary agreeing preterite.
        construction IrrealisClause: IrrealisClause {
            form [subject: NominativePhrase, " ", predicate: IrrealisPredicate];
            agree subject.number = predicate.number;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
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
            export SourceComplementSelection = predicate.SourceComplementSelection;
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: ScalarLocationPredicate];
            require subject.ScalarDenotation = Yes;
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        construction ScalarLocationPredicate: ScalarLocationPredicate {
            form [head: lexical(Verb), complements: selected_frame(head, ScalarLocation)];
            use FiniteHeadAgreement;
        }

        // Preposed temporal NP position: project ruling R11a (2026-10-09).
        // Keep the existing PP forms and their Reading identities.
        construction InitialPreposition: Clause {
            export SourceComplementSelection = clause.SourceComplementSelection;
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
            require source_adjunct_admission(clause.SourceComplementSelection,
                dependent.SourceMarker) = Yes;
            export SourceComplementSelection = clause.SourceComplementSelection;
            require clause.ClauseCoordinationHost = Coordinated;
            export ClauseCoordinationHost = clause.ClauseCoordinationHost;
            form [clause: Clause, " ", dependent: PrepositionPhrase];
            require licence_adjunct(dependent.PrepositionFunctionLicence) = Yes;
            require adjunct_complement_function(dependent.ComplementFunctionRestriction,
                dependent.PredicativeAdjunctUse) = Yes;
            require dependent.MatrixComplementRequirement = None;
            require dependent.SelectedPrepositionUse = No;
            export UnmarkedConjunctLicence = clause.UnmarkedConjunctLicence;
        }

        construction MedialSupplementation: Clause {
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
            (ClauseSeries, Clause, LeadingConjunct, CoordinatedClauseHost, NoConcord),
            (KeywordQualityPrepositionSeries, KeywordQualityPreposition, QualityPrepositionConcord),
            (SelectedComplementTailSeries, SelectedComplementTail, SelectedTailConcord,
                PrimitiveRightTail),
            (FinitePredicateSeries, FinitePredicate, FiniteConcord, NoConcord, NoConcord),
            (SecondaryPredicateSeries, SecondaryVerbPhrase, SecondaryConjunctProperties, NoConcord, NoConcord),
            (NounPhraseSeries, NounPhrase, NounCoordinationAgreement, CoordinatorKindSummary),
            (VerbalPremodifierSeries, VerbalPremodifier),
            (NounPremodifierSeries, NounPremodifier, NounPremodifierConcord),
            (ManaPhraseSeries, ManaPhrase),
            (CardinalSeries, Cardinal, CardinalListEnd),
            (AmountSeries, Amount),
            (MeasurePhraseSeries, MeasurePhrase, CoordinatedMeasureKind),
            (KeywordPhraseSeries, KeywordPhrase),
            (QuotedTextSeries, QuotedText),
            (GrantedAbilitySeries, GrantedAbility, GrantedAbilityConcord),
            (FiniteSelectedHeadSeries, FiniteSelectedHead, FiniteConcord, SelectedFrameConcord, NoConcord),
            (SecondarySelectedHeadSeries, SecondarySelectedHead, SecondaryConcord,
                SelectedFrameConcord, NoConcord),
            (NominalSeries, Nominal, NominalConcord, UnmarkedNominalSeriesConjunct),
            (AdjectivePhraseSeries, AdjectivePhrase, AdjectiveStructureMerge),
            (PrepositionPhraseSeries, PrepositionPhrase, ObliquePrepositionConcord, CoordinatorKindSummary),
            (AdverbPhraseSeries, AdverbPhrase, AdverbPermissions, LeadingConjunct),
            (InfinitiveComplementSeries, InfinitiveComplement, NoConcord),
            (FrequencyPhraseSeries, FrequencyPhrase),
            (FiniteObjectGapSeries, FiniteObjectGap, FiniteGapConcord, NoConcord, NoConcord),
            (BareObjectGapSeries, BareObjectGap, BareGapConcord, NoConcord, NoConcord),
            (PastParticipleObjectGapSeries, PastParticipleObjectGap, BareGapConcord, NoConcord, NoConcord),
            (FiniteClauseSeries, FiniteClause, NoConcord, ClauseCoordinationProperties, NoConcord),
            (DepictivePhraseSeries, DepictivePhrase, DepictiveListEnd),
            (PredicativeComplementSeries, PredicativeComplement, PredicativeListEnd),
            (SelectedPrepositionHeadSeries, SelectedPrepositionHead, PrepositionPermissions),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
            use Distribution(right, coordinator);
        }
        instance CoordinationSeriesContinuation<
            Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord>: [
            (PremodifierCoordination, Premodifier, Self, PremodifierConcord),
            (ClauseSeries, Clause, Self, UnmarkedConjunct, CoordinatedClauseHost),
            (KeywordQualityPrepositionSeries, KeywordQualityPreposition, Self,
                QualityPrepositionConcord),
            (SelectedComplementTailSeries, SelectedComplementTail, Self, SelectedTailConcord),
            (FinitePredicateSeries, FinitePredicate, Self, FiniteConcord),
            (SecondaryPredicateSeries, SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
            (NounPhraseSeries, NounPhrase, Self, NounCoordinationAgreement, CoordinatorKindSummary),
            (VerbalPremodifierSeries, VerbalPremodifier, Self),
            (NounPremodifierSeries, NounPremodifier, Self, NounPremodifierConcord),
            (ManaPhraseSeries, ManaPhrase, Self),
            (CardinalSeries, Cardinal, Self, CardinalListEnd),
            (AmountSeries, Amount, Self),
            (MeasurePhraseSeries, MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhraseSeries, KeywordPhrase, Self),
            (QuotedTextSeries, QuotedText, Self),
            (GrantedAbilitySeries, GrantedAbility, Self, GrantedAbilityConcord),
            (FiniteSelectedHeadSeries, FiniteSelectedHead, Self, FiniteConcord,
                SelectedFrameConcord),
            (SecondarySelectedHeadSeries, SecondarySelectedHead, Self, SecondaryConcord,
                SelectedFrameConcord),
            (NominalSeries, Nominal, Self, NominalConcord, UnmarkedNominalSeriesConjunct),
            (AdjectivePhraseSeries, AdjectivePhrase, Self, AdjectiveStructureMerge),
            (PrepositionPhraseSeries, PrepositionPhrase, Self, ObliquePrepositionConcord, CoordinatorKindSummary),
            (AdverbPhraseSeries, AdverbPhrase, Self, AdverbPermissions, LeadingConjunct),
            (InfinitiveComplementSeries, InfinitiveComplement, Self, NoConcord),
            (FrequencyPhraseSeries, FrequencyPhrase, Self),
            (FiniteObjectGapSeries, FiniteObjectGap, Self, FiniteGapConcord),
            (BareObjectGapSeries, BareObjectGap, Self, BareGapConcord),
            (PastParticipleObjectGapSeries, PastParticipleObjectGap, Self, BareGapConcord),
            (FiniteClauseSeries, FiniteClause, Self, NoConcord, ClauseCoordinationProperties),
            (DepictivePhraseSeries, DepictivePhrase, Self, DepictiveListEnd),
            (PredicativeComplementSeries, PredicativeComplement, Self, PredicativeListEnd),
            (SelectedPrepositionHeadSeries, SelectedPrepositionHead, Self, PrepositionPermissions),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
        }
        instance SerialCoordination<Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord, BoundaryProperties = NoFeatures>: [
            (PremodifierCoordination, Premodifier, Premodifier, PremodifierConcord),
            (Clause, Self, ClauseSeries, UnmarkedConjunct, CoordinatedClauseHost),
            (KeywordQualityPreposition, Self, KeywordQualityPrepositionSeries,
                QualityPrepositionConcord),
            (SelectedComplementTail, Self, SelectedComplementTailSeries, SelectedTailConcord),
            (FinitePredicate, Self, FinitePredicateSeries, FiniteConcord, NoConcord, NoClauseComplementBoundary),
            (SecondaryVerbPhrase, Self, SecondaryPredicateSeries, SecondaryConjunctProperties, NoConcord, NoClauseComplementBoundary),
            (NounPhrase, Self, NounPhraseSeries, NounCoordinationAgreement),
            (VerbalPremodifier, Self, VerbalPremodifierSeries),
            (NounPremodifier, Self, NounPremodifierSeries, NounPremodifierConcord),
            (ManaPhrase, Self, ManaPhraseSeries),
            (Cardinal, Self, CardinalSeries, CardinalAgreement),
            (Amount, Self, AmountSeries),
            (MeasurePhrase, Self, MeasurePhraseSeries, CoordinatedMeasureKind),
            (KeywordPhrase, Self, KeywordPhraseSeries),
            (QuotedText, Self, QuotedTextSeries, NoConcord, CoordinatedQuotation),
            (GrantedAbility, Self, GrantedAbilitySeries, GrantedAbilityCoordination),
            (FiniteSelectedHead, Self, FiniteSelectedHeadSeries, FiniteConcord, SharedFrameConcord),
            (SecondarySelectedHead, Self, SecondarySelectedHeadSeries, SecondaryConcord,
                SharedFrameConcord),
            (Nominal, Self, NominalSeries, NominalConcord, UnmarkedNominalConjunct),
            (AdjectivePhrase, Self, AdjectivePhraseSeries, AdjectiveStructureMerge),
            (PrepositionPhrase, Self, PrepositionPhraseSeries, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPhraseSeries, AdverbPermissions, LeadingConjunct),
            (InfinitiveComplement, Self, InfinitiveComplementSeries, NoConcord),
            (FrequencyPhrase, Self, FrequencyPhraseSeries),
            (FiniteObjectGap, Self, FiniteObjectGapSeries, FiniteGapConcord, NoConcord, NoClauseComplementBoundary),
            (BareObjectGap, Self, BareObjectGapSeries, BareGapConcord, NoConcord, NoClauseComplementBoundary),
            (PastParticipleObjectGap, Self, PastParticipleObjectGapSeries, BareGapConcord, NoConcord, NoClauseComplementBoundary),
            (CoordinatedFiniteClause, FiniteClause, FiniteClauseSeries, NoConcord, ClauseCoordinationProperties),
            (DepictivePhrase, Self, DepictivePhraseSeries, UnlikeDepictives),
            (PredicativeComplement, Self, PredicativeComplementSeries, UnlikePredicatives),
            (SelectedPrepositionHead, Self, SelectedPrepositionHeadSeries, PrepositionPermissions,
                SharedHeadStatus),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
            use BoundaryProperties;
        }


        // CF8 (orchestrator, 2026-10-10): like and unlike focused Adjuncts
        // share one member Category. Each form checks its own function licence.
        construction FocusedAdjunctMember: FocusedAdjunctMember {
            export FocusedUse = Yes;
            require head.FocusedUse = Yes;
            form [head: PrepositionPhrase]
                require licence_adjunct(head.PrepositionFunctionLicence) = Yes
                require adjunct_complement_function(head.ComplementFunctionRestriction,
                    head.PredicativeAdjunctUse) = Yes
                require head.SelectedPrepositionUse = No
                require head.MatrixComplementRequirement = None
                export NominalAdjunctHost = Yes
                export ClauseComplementBoundary = head.ClauseComplementBoundary
                export ConnectiveAdjunctUse = head.ConnectiveAdjunctUse
                export SourceMarker = head.SourceMarker
                export SourceComplementUse = head.SourceComplementUse;
            form [head: AdverbPhrase]
                require head.VPFinalAdjunct = Yes
                export NominalAdjunctHost = No
                export ClauseComplementBoundary = None
                export ConnectiveAdjunctUse = No
                export SourceMarker = No
                export SourceComplementUse = No;
        }
        // A bounded frequency stays inside its coordinate. The standalone
        // predicate route is retained; no extra standalone AdvP is admitted.
        instance NominalAdjunctPredicate<Result, Head: head>: [
            (FocusedAdjunctMember, AdverbPhrase),
        ] {
            require head.FocusedUse = Yes;
            require head.VPFinalAdjunct = Yes;
            export FocusedUse = Yes;
            export NominalAdjunctHost = No;
            export ClauseComplementBoundary = None;
            export ConnectiveAdjunctUse = No;
            export SourceMarker = No;
            export SourceComplementUse = No;
        }
        policy FocusedAdjunctConcord<Right, Source> {
            export FocusedUse = Yes;
            export NominalAdjunctHost = Right.NominalAdjunctHost;
            export ClauseComplementBoundary = coordinated_clause_complement_boundary(
                left.ClauseComplementBoundary, Right.ClauseComplementBoundary);
            export ConnectiveAdjunctUse = connective_adjunct_concord(
                left.ConnectiveAdjunctUse, Right.ConnectiveAdjunctUse);
            export SourceMarker = coordinated_source_marker(left.SourceMarker, Right.SourceMarker);
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
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = Yes;
            export NominalDependentPresent = No;
            export SubjectRelativeNumber = None;
            export BareGenitiveHost = head.BareGenitiveHost;
            form [head: lexical(Noun)];
            require head.framing = Unframed;
            use UnmodifiedNominalProperties;
        }

        construction BareFramedNoun: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = Yes;
            export NominalDependentPresent = No;
            export SubjectRelativeNumber = None;
            export BareGenitiveHost = head.BareGenitiveHost;
            form [head: lexical(Noun)];
            require head.frame = BareNominal;
            use UnmodifiedNominalProperties;
        }

        // CGEL Ch. 14 §8.2, pp. 1259–1260: selected subjectless
        // to-infinitivals are internal Complements of nouns.
        construction InfinitiveComplementNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = Yes;
            export NominalDependentPresent = Yes;
            export SubjectRelativeNumber = None;
            export BareGenitiveHost = No;
            form [head: lexical(Noun), " ", complement: InfinitiveComplement];
            require head.frame = NominalInfinitive;
            use UnmodifiedNominalProperties;
        }

        construction SymbolComplementNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = Yes;
            export NominalDependentPresent = Yes;
            export SubjectRelativeNumber = None;
            export BareGenitiveHost = No;
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ",
                complement: CostSymbols];
            require head.frame = NominalSymbols;
            require marker.NominalComplementMarker = Of;
            require licence_noun_complement(marker.PrepositionFunctionLicence) = Yes;
            use UnmodifiedNominalProperties;
        }

        table nominal_preposition_frame(frame) -> Selection {
            (NominalPreposition) => Yes, (NominalToPreposition) => Yes,
        }
        // Orchestrator rulings DV1–DV4: the selected colour Complement stays
        // inside the noun's frame, without a general colour-word NP route.
        construction PrepositionComplementNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
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
            use UnmodifiedNominalProperties;
            export BareGenitiveHost = No;
        }

        construction Adjective: AdjectivePhrase {
            export AdjectiveComplementClass = head.AdjectiveComplementClass;
            export AdjectiveAttributiveOnlyUse = No;
            export ImplicitComparativeUse = No;
            export PremodifierCoordination = No;
            form [head: lexical(Adjective)];
            require head.framing = Unframed;
            export AdjectiveStructure = Simple;
            export AdjectiveOrigin = head.AdjectiveOrigin;
            export AdjectivePremodifierUse = head.AdjectivePremodifierUse;
            export TargetingPremodifierUse = head.TargetingPremodifierUse;
        }

        construction IntransitiveAdjective: AdjectivePhrase {
            export AdjectiveComplementClass = head.AdjectiveComplementClass;
            export AdjectiveAttributiveOnlyUse = implicit_comparative_attributive_use(
                head.ImplicitComparativeUse);
            export ImplicitComparativeUse = head.ImplicitComparativeUse;
            export PremodifierCoordination = No;
            form [head: lexical(Adjective)];
            require head.frame = Intransitive;
            export AdjectiveStructure = Simple;
            export AdjectiveOrigin = head.AdjectiveOrigin;
            export AdjectivePremodifierUse = head.AdjectivePremodifierUse;
            export TargetingPremodifierUse = head.TargetingPremodifierUse;
        }

        construction ComplementedAdjective: AdjectivePhrase {
            export AdjectiveComplementClass = None;
            export AdjectiveAttributiveOnlyUse = No;
            export ImplicitComparativeUse = No;
            export PremodifierCoordination = No;
            form [head: lexical(Adjective), " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require head.frame = AdjectivalPrepositionComplement;
            require marker.InternalisedComplementMarker = Yes;
            export AdjectiveStructure = Complemented;
            export AdjectiveOrigin = head.AdjectiveOrigin;
            export AdjectivePremodifierUse = head.AdjectivePremodifierUse;
            export TargetingPremodifierUse = No;
        }

        // The Adjective selects its infinitival Complement; DS3 licenses
        // the resulting complemented phrase on the existing PostpositiveNominal route.
        construction InfinitivalAdjective: AdjectivePhrase {
            export AdjectiveComplementClass = None;
            export AdjectiveAttributiveOnlyUse = No;
            export ImplicitComparativeUse = No;
            export PremodifierCoordination = No;
            form [head: lexical(Adjective), " ", complement: InfinitiveComplement];
            require head.frame = AdjectivalInfinitive;
            export AdjectiveStructure = Complemented;
            export AdjectiveOrigin = head.AdjectiveOrigin;
            export AdjectivePremodifierUse = head.AdjectivePremodifierUse;
            export TargetingPremodifierUse = No;
        }

        construction NounPremodifier: NounPremodifier {
            export PremodifierCoordination = No;
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
            (VerbalPremodifier, Self),
            (NounPremodifier, Self, NounPremodifierConcord),
            (KeywordQualityPreposition, Self, QualityPrepositionConcord),
            (SelectedComplementTail, Self, SelectedTailConcord, PrimitiveRightTail),
            (NounPhrase, Self, NounCoordinationAgreement),
            (FinitePredicate, Self, FiniteConcord, NoConcord, NoConcord, NoClauseComplementBoundary),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties, NoConcord, NoConcord, NoClauseComplementBoundary),
            (FiniteObjectGap, Self, FiniteGapConcord, NoConcord, NoConcord, NoClauseComplementBoundary),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self, NoConcord, CoordinatedQuotation),
            (GrantedAbility, Self, GrantedAbilityCoordination),
            (FiniteSelectedHead, Self, FiniteConcord, SharedFrameConcord, NoConcord),
            (SecondarySelectedHead, Self, SecondaryConcord, SharedFrameConcord, NoConcord),
            (Nominal, Self, NominalConcord),
            (AdjectivePhrase, Self, AdjectiveStructureMerge),
            (ComparativeGovernorHead, Self, NoConcord, ComparativeGovernorComparison),
            (PrepositionPhrase, Self, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPermissions, LeadingConjunct),
            (InfinitiveComplement, Self, NoConcord),
            (FrequencyPhrase, Self),
            (BareObjectGap, Self, BareGapConcord, NoConcord, NoConcord, NoClauseComplementBoundary),
            (PastParticipleObjectGap, Self, BareGapConcord, NoConcord, NoConcord, NoClauseComplementBoundary),
            (CoordinatedFiniteClause, FiniteClause, NoConcord, ClauseCoordinationProperties, NoConcord),
            (DepictivePhrase, Self, UnlikeDepictives),
            (PredicativeComplement, Self, UnlikePredicatives),
            (SelectedPrepositionHead, Self, PrepositionPermissions, SharedHeadStatus),
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
            form [head: NounPremodifier];
            require head.PremodifierCoordination = No;
            export NounPremodifier = head.NounPremodifier;
            export AdjectiveOrigin = Ordinary;
            export TargetingPremodifierUse = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
        }

        construction AdjectivePremodifierMember: Premodifier {
            form [head: AdjectivePhrase];
            require head.PremodifierCoordination = No;
            require head.AdjectiveStructure = Simple;
            require head.AdjectivePremodifierUse = Yes;
            export NounPremodifier = None;
            export AdjectiveOrigin = head.AdjectiveOrigin;
            export TargetingPremodifierUse = head.TargetingPremodifierUse;
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

        construction CoordinatedPremodifiedNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = No;
            form [modifier: PremodifierCoordination, " ", head: Nominal];
            require licensed_noun_premodifier(modifier.NounPremodifier,
                head.AttributePremodifierHead, head.KeywordPremodifierHead) = Yes;
            require targeting_premodifier(modifier.TargetingPremodifierUse, head.Targeting) = Yes;
            use NominalHeadFeatures;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = head.BareGenitiveHost;
            export PropertyValuePosition = coordinated_premodifier_property_value(
                modifier.NounPremodifier, modifier.AdjectiveOrigin, head.PropertyValuePosition);
            export SelectedPrepositionUse = selected_preposition_complement(
                modifier.SelectedPrepositionUse, head.SelectedPrepositionUse);
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                modifier.PredicativeAdjunctUse, head.PredicativeAdjunctUse);
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
        }

        construction NounPremodifiedNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: NounPremodifier, " ", head: Nominal];
            require licensed_noun_premodifier(modifier.NounPremodifier,
                head.AttributePremodifierHead, head.KeywordPremodifierHead) = Yes;
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            require head.Targeting = No;
        }

        construction PremodifiedNominal: Nominal {
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: AdjectivePhrase, " ", head: Nominal];
            use NominalHeadFeatures;
            export ComparativeModifierPresent = comparative_modifier_present(
                modifier.ImplicitComparativeUse, head.ComparativeModifierPresent);
            export PropertyValuePosition = premodified_property_value_position(
                modifier.AdjectiveOrigin, head.PropertyValuePosition);
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            require modifier.AdjectiveStructure = Simple;
            require modifier.AdjectivePremodifierUse = Yes;
            require targeting_premodifier(modifier.TargetingPremodifierUse, head.Targeting) = Yes;
        }

        // CGEL pp. 393, 395 gives comparative Determiner function and head
        // selection; CQ1–CQ4 (2026-10-10) supplies the shared representation
        // and immediate NP Complement analysis. CGEL p. 1139 describes same/as.
        schema DiscontinuousComparativeNominal {
            form [governor: node, " ", head: Nominal, " ",
                marker: lexical(Preposition), " ", complement: AccusativePhrase];
            require licence_comparative_complement(marker.PrepositionFunctionLicence) = Yes;
            require head.Targeting = No;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
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
            export ImplicitComparativeUse = head.ImplicitComparativeUse;
        }
        policy DiscontinuousGovernorConcord<Right, Source> {
            export frame = coordinated_discontinuous_frame(left.frame, Right.frame);
            export ImplicitComparativeUse = implicit_comparative_concord(
                left.ImplicitComparativeUse, Right.ImplicitComparativeUse);
        }
        instance Coordination: DiscontinuousComparativeGovernor {
            bind left, right = DiscontinuousComparativeGovernor;
            use DiscontinuousGovernorConcord(right, coordinator);
        }
        construction ComparativeDeterminerHead: ComparativeDeterminerHead {
            form [head: lexical(Determinative)];
            require head.ComparativeDeterminerUse = Yes;
            export ComparativeDeterminerUse = head.ComparativeDeterminerUse;
            export DeterminerUse = head.DeterminerUse;
            export NominalAdjunctDependent = head.NominalAdjunctDependent;
        }

        policy DiscontinuousComparativeModifier {
            export BareGenitiveHost = head.BareGenitiveHost;
            require discontinuous_comparative_marker(governor.frame,
                marker.ExpandedComparisonMarker) = Yes;
            export ComparativeModifierPresent = comparative_modifier_present(
                governor.ImplicitComparativeUse, head.ComparativeModifierPresent);
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
        }

        policy DiscontinuousComparativeDeterminer {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Nondefinite;
            require governor.ComparativeDeterminerUse = Yes;
            require determiner_license(governor.DeterminerUse, head.number, head.countability,
                head.DeterminerRequirement, governor.NominalAdjunctDependent) = Yes;
            require marker.ExpandedComparisonMarker = Ordering;
            require head.DeterminerRequirement = No;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber,
                head.NominalAgreementNumber, head.number) = Yes;
            export UnitBearing = nominal_unit_bearing(head.MeasurePosition);
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export BarePrepositionUse = No;
        }

        instance DiscontinuousComparativeNominal<Result, Governor: governor,
            HeadFeatures, HeadProperties, Properties>: [
            (Nominal, DiscontinuousComparativeGovernor, NominalHeadFeatures, NominalHeadProperties,
                DiscontinuousComparativeModifier),
            (NounPhrase, ComparativeDeterminerHead, NounPhraseHeadAgreement, NoFeatures,
                DiscontinuousComparativeDeterminer),
        ] { use HeadFeatures; use HeadProperties; use Properties; }

        // CGEL Ch. 5 §7.6, p. 386: numerical cardinals can modify under
        // an outer Determiner; without it the cardinal is the Determiner.
        construction CardinalPremodifiedNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export NominalDependentPresent = Yes;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export AttributePremodifierHead = head.AttributePremodifierHead;
            export KeywordPremodifierHead = head.KeywordPremodifierHead;
            export NominalAgreementNumber = head.NominalAgreementNumber;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            export MeasurePosition = head.MeasurePosition;
            export ScalarMeasurementHeadUse = No;
            export PropertyValuePosition = None;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            form [quantity: Cardinal, " ", head: Nominal];
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export RelativisableAdjunctClass = head.RelativisableAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            require head.DeterminerRequirement = No;
            require head.countability = Count;
            agree quantity.number = head.number;
            export DeterminerRequirement = Yes;
            export ObliqueNumber = head.ObliqueNumber;
            export BareGenitiveHost = head.BareGenitiveHost;
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
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export NominalDependentPresent = Yes;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export AttributePremodifierHead = head.AttributePremodifierHead;
            export KeywordPremodifierHead = head.KeywordPremodifierHead;
            export NominalAgreementNumber = Head;
            export ScalarDenotation = Yes;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
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
            export ScalarMeasurementHeadUse = No;
            export PropertyValuePosition = None;
            export CardinalMeasurementUse = measured_nominal_cardinal_use(
                head.CardinalMeasurementUse, quantity.CardinalMeasurementUse);
            export Targeting = head.Targeting;
            export NominalAdjunctClass = None;
            export RelativisableAdjunctClass = head.RelativisableAdjunctClass;
            export SlashPremodifierUse = No;
            export NumberTransparency = No;
            export ObliqueNumber = head.ObliqueNumber;
            export BareGenitiveHost = No;
        }

        construction CardinalMeasuredNounPhrase: NounPhrase {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Unspecified;
            export UnitBearing = Yes;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export ScalarDenotation = Yes;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            form [quantity: Cardinal, " ", head: Nominal];
            require head.MeasurePosition = Before;
            require head.CardinalMeasurementUse = Yes;
            require head.number = Singular;
            require head.countability = Mass;
            require head.DeterminerRequirement = No;
            export number = Singular;
            export BareGenitiveHost = No;
            export Targeting = head.Targeting;
            use ThirdPersonCommonCase;
            require singular_relative_number(head.SubjectRelativeNumber) = Yes;
        }

        schema PostpositiveNominal {
            form [head: node, " ", modifier: node];
        }
        policy PostpositiveNominalProperties {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = No;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            require modifier.AdjectiveStructure = Complemented;
            require modifier.AdjectiveAttributiveOnlyUse = No;
        }
        // Pre-integrate review ruling (2026-10-09): a declared fused-head NP
        // licence admits the comparative Postmodifier without pruning nominal scope.
        policy PostpositiveNounPhraseProperties {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = head.Definiteness;
            export UnitBearing = head.UnitBearing;
            require head.ComparativePostmodifierUse = Yes;
            export DeterminerAgreementNumber = head.DeterminerAgreementNumber;
            export number = head.number;
            export person = head.person;
            export CaseUse = head.CaseUse;
            export Targeting = head.Targeting;
            export BareGenitiveHost = head.BareGenitiveHost;
            export SubjectStructure = head.SubjectStructure;
            export BarePrepositionUse = head.BarePrepositionUse;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export ScalarDenotation = head.ScalarDenotation;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export ComparativePostmodifierUse = No;
        }
        instance PostpositiveNominal<Result, Head: head, Modifier: modifier,
            HeadFeatures, HeadProperties, Properties>: [
            (Nominal, Self, AdjectivePhrase, NominalHeadFeatures, NominalHeadProperties,
                PostpositiveNominalProperties),
            (NounPhrase, Self, ComparativeAdjectivePhrase, NoFeatures, NoFeatures,
                PostpositiveNounPhraseProperties),
        ] { use HeadFeatures; use HeadProperties; use Properties; }

        // CGEL Ch. 5 §14.2, p. 444: participial VPs can premodify a noun.
        table verbal_premodifier(form, frame, PastParticipialPremodifier) -> Selection {
            (GerundParticiple, Intransitive, No) => Yes,
            (GerundParticiple, Intransitive, Yes) => Yes,
            (PastParticiple, Transitive, Yes) => Yes,
        }
        construction VerbalPremodifier: VerbalPremodifier {
            form [head: lexical(Verb)];
            require verbal_premodifier(head.form, head.frame, head.PastParticipialPremodifier) = Yes;
        }

        construction ParticipialPremodifier: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: VerbalPremodifier, " ", head: Nominal];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
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
            export NounPhraseStructure = ProperName;
            export FocusedUse = No;
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
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: Name];
            export number = head.number;
            use ThirdPersonCommonCase;
        }

        // CGEL Ch. 15 §5.2(b), pp. 1357–1358: an indefinite NP can
        // supplement a proper-name anchor. NA1–NA4 (orchestrator ruling,
        // 2026-10-10) license this unclosed NP shape wherever a name occurs.
        construction AscriptiveNounPhraseSupplement: NounPhrase {
            form [anchor: NounPhrase, ", ", supplement: NounPhrase];
            require anchor.NounPhraseStructure = ProperName;
            require supplement.NounPhraseStructure = IndefiniteArticle;
            require supplement.Definiteness = Nondefinite;
            export NounPhraseStructure = Other;
            export FocusedUse = anchor.FocusedUse;
            export Definiteness = anchor.Definiteness;
            export UnitBearing = anchor.UnitBearing;
            export number = anchor.number;
            export person = anchor.person;
            export CaseUse = anchor.CaseUse;
            export Targeting = anchor.Targeting;
            export BareGenitiveHost = anchor.BareGenitiveHost;
            export SubjectStructure = anchor.SubjectStructure;
            export BarePrepositionUse = anchor.BarePrepositionUse;
            export SelectedPrepositionUse = anchor.SelectedPrepositionUse;
            export PredicativeAdjunctUse = anchor.PredicativeAdjunctUse;
            export ScalarDenotation = anchor.ScalarDenotation;
            export SourceModifierPresent = anchor.SourceModifierPresent;
            export WholeObjectSourcePresent = anchor.WholeObjectSourcePresent;
            export DeterminerAgreementNumber = anchor.DeterminerAgreementNumber;
            export ComparativePostmodifierUse = anchor.ComparativePostmodifierUse;
        }

        construction PassiveNamePredicate: NamePredicate {
            form [head: lexical(Verb), " ", complement: Name];
            require head.form = PastParticiple;
            require head.frame = ObjectName;
        }

        construction NamedNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: NamePredicate];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
        }

        construction TargetedNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export NominalDependentPresent = Yes;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export AttributePremodifierHead = head.AttributePremodifierHead;
            export KeywordPremodifierHead = head.KeywordPremodifierHead;
            export NominalAgreementNumber = head.NominalAgreementNumber;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
            export ScalarMeasurementHeadUse = No;
            export PropertyValuePosition = None;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export BareGenitiveHost = head.BareGenitiveHost;
            form [marker: lexical(Determinative), " ", head: Nominal];
            require marker.Targeting = Yes;
            require head.Targeting = No;
            export number = head.number;
            export countability = head.countability;
            export Targeting = Yes;
            export NominalAdjunctClass = None;
            export RelativisableAdjunctClass = head.RelativisableAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            export ObliqueNumber = head.ObliqueNumber;
        }

        construction PostmodifiedNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: PrepositionPhrase];
            require licence_modifier(modifier.PrepositionFunctionLicence) = Yes;
            require modifier.ComplementFunctionRestriction = None;
            require modifier.SelectedPrepositionUse = No;
            require nominal_postmodifier_licence(head.NominalComplementMarker,
                modifier.NominalComplementMarker) = Yes;
            use PostmodifiedNominalProperties;
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
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
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

        // CGEL Ch. 5 §§9.1–9.2, pp. 411–413: explicitly partitive determiner-head.
        // Targeted obliques are the each-of ticket's Oracle-English decision.
        construction DeterminerHeadPartitiveNounPhrase: NounPhrase {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Unspecified;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            form [head: lexical(Determinative), " ", complement: PrepositionPhrase,
                modifiers: repeat(PartitiveModifier, "")];
            require head.NominalComplementMarker = Of;
            agree head.NominalComplementMarker = complement.NominalComplementMarker;
            require complement.ObliqueNumber = Plural;
            require licence_noun_complement(complement.PrepositionFunctionLicence) = Yes;
            require complement.ComplementFunctionRestriction = None;
            require head.DeterminerUse = SingularCount;
            export number = Singular;
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
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = head.Definiteness;
            export UnitBearing = No;
            export ComparativePostmodifierUse = head.ComparativePostmodifierUse;
            export DeterminerAgreementNumber = Nominal;
            form [head: lexical(Determinative)];
            require head.FusedHeadUse = Yes;
            require head.DeterminerUse = Singular;
            export number = Singular;
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

        // CGEL Ch. 5 §12(b–c), p. 434: fractions and multipliers are
        // external Modifiers; HR1–HR2 require a declared definite host.
        construction PredeterminerNounPhrase: NounPhrase {
            export NounPhraseStructure = Other;
            form [modifier: lexical(Adverb), " ", head: NounPhrase];
            form [modifier: lexical(Noun), " ", head: NounPhrase];
            require predeterminer_host(modifier.PredeterminerUse, head.Definiteness) = Yes;
            export UnitBearing = head.UnitBearing;
            export number = head.number;
            export person = head.person;
            export CaseUse = head.CaseUse;
            export Targeting = head.Targeting;
            export BareGenitiveHost = head.BareGenitiveHost;
            export SubjectStructure = head.SubjectStructure;
            export BarePrepositionUse = head.BarePrepositionUse;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export ScalarDenotation = head.ScalarDenotation;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export DeterminerAgreementNumber = head.DeterminerAgreementNumber;
            export ComparativePostmodifierUse = head.ComparativePostmodifierUse;
            export FocusedUse = No;
            export Definiteness = head.Definiteness;
        }

        construction DeterminedNounPhrase: NounPhrase {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = determiner.Definiteness;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = determiner.DeterminerAgreementNumber;
            export ScalarDenotation = quantificational_scalar_denotation(
                determiner.QuantificationalDeterminer, head.NumberTransparency, head.ScalarDenotation);
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            export BareGenitiveHost = head.BareGenitiveHost;
            export Targeting = head.Targeting;
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
            export NounPhraseStructure = IndefiniteArticle;
            export FocusedUse = No;
            export Definiteness = Nondefinite;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Indefinite;
            require determiner.number = Singular;
            require head.number = Singular;
            require head.countability = Count;
            agree determiner.article_onset = head.onset;
            export number = determiner.number;
            export ScalarDenotation = head.ScalarDenotation;
            export Targeting = head.Targeting;
            export BareGenitiveHost = head.BareGenitiveHost;
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
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Nondefinite;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
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
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Unspecified;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            form [head: Nominal];
            require bare_preposition_use(head.BarePrepositionUse) = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.DeterminerRequirement = No;
            export BarePrepositionUse = head.BarePrepositionUse;
            use NounPhraseHeadAgreement;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
        }

        construction TargetNounPhrase: NounPhrase {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Nondefinite;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export ScalarDenotation = head.ScalarDenotation;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            export BareGenitiveHost = head.BareGenitiveHost;
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
            export NounPhraseStructure = Other;
            export FocusedUse = No;
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
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            use Properties;
        }

        construction AccusativePronoun: NounPhrase {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
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
            use PredicateHeadAgreement;
            require head.case = Accusative;
            require head.ExistentialUse = No;
            export CaseUse = Accusative;
        }

        construction PrepositionPhrase: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = head.NominalExceptionComplementUse;
            export ComplementFunctionRestriction = None;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: AccusativePhrase];
            require noun_phrase_complement(head.PrepositionComplement) = Yes;
            export ObliqueNumber = oblique_number(head.ObliqueMarker, complement.number);
            export NominalComplementMarker = head.NominalComplementMarker;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export LocativeUse = head.LocativeUse;
            export SelectedPrepositionUse = selected_preposition_complement(head.SelectedPrepositionUse,
                complement.SelectedPrepositionUse);
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = whole_object_oblique_source(
                head.ObliqueMarker, complement.WholeObjectSourcePresent);
        }

        construction BareNominalPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: NounPhrase];
            require bare_nominal_complement(head.BareNominalComplement,
                complement.BarePrepositionUse) = Yes;
            require noun_phrase_complement(head.PrepositionComplement) = Yes;
            export ObliqueNumber = oblique_number(head.ObliqueMarker, complement.number);
            export NominalComplementMarker = head.NominalComplementMarker;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export LocativeUse = head.LocativeUse;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export PredicativeAdjunctUse = complement.PredicativeAdjunctUse;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = PreposedOrPredicative;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            form [head: lexical(Preposition), " ", complement: PredicativeComplement];
            require predicative_noun_phrase_complement(head.PrepositionComplement) = Yes;
            require complement.PredicativeKind = Nominal;
            require licence_preposed_adjunct(head.PrepositionFunctionLicence) = Yes;
            require licence_predicative_complement(head.PrepositionFunctionLicence) = Yes;
            export ObliqueNumber = None;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export LocativeUse = head.LocativeUse;
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
            export InterveningAdjunctLicence = head.InterveningAdjunctLicence;
            export ConnectiveAdjunctUse = head.ConnectiveAdjunctUse;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = head.ClauseComplementBoundary;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export ComplementFunctionRestriction = head.ComplementFunctionRestriction;
            require adjunct_complement_function(head.ComplementFunctionRestriction,
                head.PredicativeAdjunctUse) = Yes;
            export MatrixComplementRequirement = head.MatrixComplementRequirement;
            export NominalExceptionComplementUse = head.NominalExceptionComplementUse;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
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
            export ObliqueNumber = None;
        }
        policy FocusedAdverbPermissions {
            export NominalAdjunctHost = Yes;
            export FocusedUse = Yes;
            require head.VPFinalAdjunct = Yes;
            export VPFinalAdjunct = head.VPFinalAdjunct;
            export ClauseInitialAdjunct = No;
            export DurationUse = head.DurationUse;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
        }
        policy FocusedNounPhrasePermissions {
            export NounPhraseStructure = Other;
            export FocusedUse = Yes;
            export UnitBearing = head.UnitBearing;
            export number = head.number;
            export person = head.person;
            export CaseUse = head.CaseUse;
            export Targeting = head.Targeting;
            export BareGenitiveHost = head.BareGenitiveHost;
            export SubjectStructure = head.SubjectStructure;
            export BarePrepositionUse = head.BarePrepositionUse;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export ScalarDenotation = head.ScalarDenotation;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export DeterminerAgreementNumber = head.DeterminerAgreementNumber;
            export ComparativePostmodifierUse = head.ComparativePostmodifierUse;
            export Definiteness = head.Definiteness;
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
            export ConnectiveAdjunctUse = head.ConnectiveAdjunctUse;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export ObliqueNumber = None;
            form [head: lexical(Preposition)];
            require intransitive_preposition_complement(head.PrepositionComplement) = Yes;
            use PrepositionHeadPermissions;
        }

        // CGEL Ch. 7 §3.2, p. 626: preposition + adjective idioms.
        construction AdjectiveComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            form [head: lexical(Preposition), " ", complement: lexical(Adjective)];
            require head.AdjectiveComplementClass = Manner;
            agree head.AdjectiveComplementClass = complement.AdjectiveComplementClass;
            require complement.framing = Unframed;
            export ObliqueNumber = None;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // CGEL Ch. 14 §10(b), p. 1267: reducible Clause Complements.
        // The lexical permission for an unrealized infinitival Complement
        // and the clause-final distribution are declared project decisions.
        construction VerblessClause: VerblessClause {
            form [predicate: lexical(Adjective)];
            require predicate.VerblessPredicateUse = Yes;
            export VerblessClauseComplement = Yes;
        }

        // CGEL Ch. 14 §10(a), p. 1267: with takes subject–predicate
        // clause Complements, including nonfinite and verbless forms.
        // The participial/adjectival restriction is project ruling PW2/PW6
        // (english-v3-play-with-absolute, 2026-10-10).
        construction SubjectAdjectiveVerblessClause: VerblessClause {
            form [subject: AccusativePhrase, " ", predicate: AdjectivePhrase];
            export VerblessClauseComplement = OvertSubject;
            require predicate.AdjectiveAttributiveOnlyUse = No;
        }

        construction SubjectParticipialClause: VerblessClause {
            form [subject: AccusativePhrase, " ", predicate: SecondaryVerbPhrase];
            require predicate.form = PastParticiple;
            require predicate.ParticipialUse = BarePassive;
            export VerblessClauseComplement = OvertSubject;
        }

        construction VerblessClauseComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = Verbless;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: VerblessClause];
            require head.VerblessClauseComplement = Yes;
            require complement.VerblessClauseComplement = Yes;
            require licence_adjunct(head.PrepositionFunctionLicence) = Yes;
            export SourceComplementUse = No;
            export SourceMarker = No;
            export FocusedUse = No;
            export ClauseInitialAdjunct = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export PrepositionFunctionLicence = Adjunct;
        }

        // CGEL Ch. 13 §5.5, pp. 1151–1152: compound prepositions take
        // content clauses, including modal preterites and irrealis.
        // Their Oracle function and irrealis licences are lexical declarations.
        construction ClauseComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = head.InterveningAdjunctLicence;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = Finite;
            export MatrixComplementRequirement = head.MatrixComplementRequirement;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = head.ComplementFunctionRestriction;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = Finite;
            export MatrixComplementRequirement = head.MatrixComplementRequirement;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = head.ComplementFunctionRestriction;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = Adjunct;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: AdverbPhrase];
            require head.AdverbComplementClass = Duration;
            require complement.DurationUse = Yes;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = head.ComplementFunctionRestriction;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: SecondaryVerbPhrase];
            require head.GerundClauseComplement = Yes;
            require complement.form = GerundParticiple;
            export ObliqueNumber = None;
            use PrepositionHeadPermissions;
        }

        construction PredicativePreposition: PredicativeComplement {
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export FocusedUse = No;
            export ClauseInitialAdjunct = No;
            form [head: lexical(Preposition), " ", complement: PrepositionPhrase];
            require selected_compound_complement(head.CompoundComplementMarker,
                complement.NominalComplementMarker) = Yes;
            require licence_compound(head.PrepositionFunctionLicence) = Yes;
            require compound_complement_function(complement.ComplementFunctionRestriction) = Yes;
            export ObliqueNumber = None;
            use PrepositionHeadPermissions;
        }

        // Project analysis: the numeric nominal exception retains its
        // cardinal Complement inside the selected PP, without licensing a
        // free duration, purpose or nominal-modifier use.
        construction QuantitativeExceptionPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: Cardinal];
            require head.NominalExceptionComplementUse = Yes;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = Yes;
            export PrepositionFunctionLicence = Compound;
            export LocativeUse = No;
            export ObliqueNumber = None;
            export InternalisedComplementMarker = No;
            export NominalComplementMarker = None;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export ClauseInitialAdjunct = No;
            export FocusedUse = No;
            export SourceComplementUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export SourceMarker = No;
        }
        construction MatrixComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: PrepositionPhrase];
            require head.MatrixComplementGovernor = Yes;
            require complement.MatrixComplementRequirement = None;
            export MatrixComplementRequirement = matrix_pp_requirement(
                complement.InternalisedComplementMarker, complement.NominalExceptionComplementUse);
            export NominalExceptionComplementUse = No;
            export PrepositionFunctionLicence = Adjunct;
            export LocativeUse = No;
            export ObliqueNumber = None;
            export InternalisedComplementMarker = No;
            export NominalComplementMarker = None;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export ClauseInitialAdjunct = No;
            export FocusedUse = No;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: PrepositionPhrase];
            require preposition_phrase_complement(head.PrepositionComplement) = Yes;
            require complement.MatrixComplementRequirement = None;
            require complement.ComplementFunctionRestriction = None;
            require licensed_preposition_phrase_complement(head.PrepositionPhraseComplementLicence,
                complement.SourceComplementUse) = Yes;
            export ObliqueNumber = None;
            use PrepositionHeadPermissions;
        }


        construction SourceComplement: SourceComplement {
            form [phrase: PrepositionPhrase];
            require phrase.SourceMarker = Yes;
            require licence_verb_complement(phrase.PrepositionFunctionLicence) = Yes;
            require phrase.ComplementFunctionRestriction = None;
        }

        construction LocativeComplement: LocativeComplement {
            form [phrase: PrepositionPhrase];
            require phrase.LocativeUse = Yes;
            require phrase.SelectedPrepositionUse = No;
            require licence_predicative_complement(phrase.PrepositionFunctionLicence) = Yes;
            require phrase.ComplementFunctionRestriction = None;
        }

        construction MannerComplement: MannerComplement {
            form [phrase: AccusativePhrase];
            require phrase.SelectedPrepositionUse = Yes;
        }

        construction AdjectivalComplement: PredicativeComplement {
            export PredicativeAdjunctUse = No;
            form [phrase: AdjectivePhrase];
            export PredicativeKind = Adjectival;
            require phrase.AdjectiveAttributiveOnlyUse = No;
        }

        // CGEL pp. 262, 1265: optional depictives are adjuncts.
        construction AdjectivalDepictive: DepictivePhrase {
            form [phrase: AdjectivePhrase];
            export DepictiveKind = Adjectival;
            require phrase.AdjectiveAttributiveOnlyUse = No;
        }

        construction ParticipialDepictive: DepictivePhrase {
            form [phrase: SecondaryVerbPhrase];
            require phrase.form = GerundParticiple;
            require phrase.ParticipialUse = Ordinary;
            export DepictiveKind = Participial;
        }

        construction NominalComplement: PredicativeComplement {
            export PredicativeAdjunctUse = phrase.PredicativeAdjunctUse;
            form [phrase: AccusativePhrase];
            export PredicativeKind = Nominal;
        }
        instance BareAuxiliaryPredicate<Result, Complement: complement, Properties>: [
            (FinitePredicate, BareComplement, FiniteAuxiliaryAgreement),
            (SecondaryVerbPhrase, BareComplement, SecondaryAuxiliaryProperties),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance ParticipialAuxiliaryPredicate<Result, Complement: complement, Properties, BoundaryProperties = NoFeatures>: [
            (FinitePredicate, ParticipialComplement, FiniteAuxiliaryAgreement, NoClauseComplementBoundary),
            (IrrealisPredicate, ParticipialComplement, IrrealisHeadAgreement),
            (SecondaryVerbPhrase, ParticipialComplement, SecondaryAuxiliaryProperties, NoClauseComplementBoundary),
        ] {
            use Properties;
            use BoundaryProperties;
        }
        instance PerfectAuxiliaryPredicate<Result, Complement: complement, Properties>: [
            (FinitePredicate, PerfectComplement, FiniteAuxiliaryAgreement),
            (SecondaryVerbPhrase, PerfectComplement, SecondaryAuxiliaryProperties),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
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
            form [object: AccusativePhrase, " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require noun_phrase_complement(marker.PrepositionComplement) = Yes;
            require licence_verb_complement(marker.PrepositionFunctionLicence) = Yes;
            segment Predicate;
            export HeadCoordination = No;
        }
        construction ObjectLocativeTail: SelectedComplementTail {
            form [object: AccusativePhrase, " ", complement: LocativeComplement];
            segment Predicate;
            export HeadCoordination = No;
        }
        // GC7: each coordinate selects its own Manner Complement.
        construction ObjectLocativeMannerTail: SelectedComplementTail {
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None; use Properties; }

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
            export ComparativeObjectGapLicence = head.ComparativeObjectGapLicence;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            require head.form = Plain;
        }
        policy BarePassiveHead {
            export ParticipialSupplementUse = head.ParticipialSupplementUse;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        construction BareNominalPrepositionPassive: SecondaryVerbPhrase {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        construction SelectedExtentPassive: SecondaryVerbPhrase {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            form [head: lexical(Verb), " ", complement: ScalarExtentComplement];
            require head.frame = ObjectExtent;
            use BarePassiveHead;
        }
        construction SelectedFramePassive: SecondaryVerbPhrase {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            form [head: lexical(Verb), complements: selected_frame(head, Passive)];
            use BarePassiveHead;
        }
        construction RetainedObjectPassive: SecondaryVerbPhrase {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
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
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: SecondaryVerbPhrase];
            require participial_postmodifier(modifier.form, modifier.ParticipialUse) = Yes;
            use ParticipialNominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
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
        table nominal_dependent_presence(NominalDependentPresent, NominalDependentPresent)
            -> NominalDependentPresent {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        table nominal_adjunct_concord(NominalAdjunctClass, NominalAdjunctClass)
            -> NominalAdjunctClass {
            (None, None) => None, (None, Temporal) => None, (None, Manner) => None,
            (Temporal, None) => None, (Temporal, Temporal) => Temporal,
            (Temporal, Manner) => None, (Manner, None) => None,
            (Manner, Temporal) => None, (Manner, Manner) => Manner,
        }
        construction NominalAdjunctPhrase: NominalAdjunctPhrase {
            export NominalAdjunctClass = head.NominalAdjunctClass;
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
        instance NominalAdjunctPredicate<Result, Head: head, Properties>: [
            (ExistentialPredicate, Self, PredicateHostAgreement),
            (FinitePredicate, Self, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, SecondaryProjection),
            (FiniteObjectGap, Self, FiniteGapHostProjection),
            (BareObjectGap, Self, BareGapHostProjection),
        ] {
            require nonconnective_trailing_adjunct_host(head.ClauseComplementBoundary) = Yes;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: BareTemporalNominal];
            agree head.NominalBareClass = complement.NominalBareClass;
            use PrepositionHeadPermissions;
        }

        // The declared modifier licenses this Oracle-register bare singular NP.
        construction BareStatusNounPhrase: NounPhrase {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Unspecified;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
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
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Unspecified;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            form [modifier: lexical(Verb), " ", head: Nominal];
            require verbal_premodifier(modifier.form, modifier.frame,
                modifier.PastParticipialPremodifier) = Yes;
            require modifier.BareSingularUse = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.Targeting = No;
            use NounPhraseHeadAgreement;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
        }

        construction PronounSubjectRelative: SubjectRelativeClause {
            form [marker: lexical(Pronoun), clitic_separator(predicate.CliticHost),
                predicate: FinitePredicate];
            require marker.RelativeUse = Yes;
            require marker.case = Nominative;
            agree marker.person = predicate.person;
            agree marker.number = predicate.number;
            export number = predicate.number;
        }

        construction QuantifiedSubjectRelative: SubjectRelativeClause {
            form [marker: lexical(Subordinator), " ", quantifier: lexical(Determinative),
                " ", predicate: FinitePredicate];
            require marker.RelativeSubordinator = Yes;
            require quantifier.QuantificationalAdjunctUse = Yes;
            require predicate.CliticHost = Free;
            require predicate.number = Plural;
            require predicate.person = Third;
            export number = predicate.number;
        }

        construction CountedFrequency: FrequencyPhrase {
            form [quantity: Cardinal, " ", head: lexical(Noun)];
            require head.FrequencyUnit = Yes;
            require head.countability = Count;
            agree quantity.number = head.number;
        }
        instance FrequencyPredicate<Result, Head: head, Modifier: modifier, Properties>: [
            (FinitePredicate, Self, FrequencyPhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, FrequencyPhrase, SecondaryProjection),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }

        instance PrepositionPredicate<Result, Head: head, Modifier: modifier, Properties>: [
            (ExistentialPredicate, Self, PrepositionPhrase, PredicateHostAgreement),
            (FinitePredicate, Self, PrepositionPhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, PrepositionPhrase, SecondaryPrepositionProjection),
            (FiniteObjectGap, Self, PrepositionPhrase, FiniteGapHostProjection),
            (BareObjectGap, Self, PrepositionPhrase, BareGapHostProjection),
        ] {
            use Properties;
        }

        instance PurposePredicate<Result, Head: head, Properties>: [
            (FinitePredicate, Self, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, SecondaryAdjunctProjection),
            (FiniteObjectGap, Self, FiniteGapHostProjection),
            (BareObjectGap, Self, BareGapHostProjection),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }

        construction BarePredicate: BarePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = Plain;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
        }

        construction ProgressiveComplement: ParticipialPredicate {
            export CentralAdjunctPosition = PostAuxiliary;
            form [head: SecondaryVerbPhrase];
            require head.form = GerundParticiple;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
        }

        // CGEL p. 1327: unlike AdjP and progressive complements coordinated under be.
        // The shared phrase describes their constituent structure; here it is a complement.
        construction MixedCopularProgressiveComplement: ParticipialPredicate {
            export CentralAdjunctPosition = PostAuxiliary;
            form [phrase: DepictivePhrase];
            require phrase.DepictiveKind = Mixed;
            export PurposeAdjunctLicence = No;
            export MatrixComplementLicence = None;
            export SourceComplementSelection = No;
        }

        construction PassiveComplement: ParticipialPredicate {
            export CentralAdjunctPosition = PostCopula;
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            require head.ParticipialUse = BarePassive;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
        }

        construction PerfectComplement: PastParticiplePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            require head.ParticipialUse = Ordinary;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
        }

        construction PassivePredicate: SecondaryVerbPhrase {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            form [head: lexical(Verb)];
            require head.form = PastParticiple;
            require head.frame = Transitive;
            export form = PastParticiple;
            export ParticipialSupplementUse = No;
            export ParticipialUse = BarePassive;
            export InternalisedComplementPresent = No;
        }

        construction SubjectRelativeClause: SubjectRelativeClause {
            form [marker: lexical(Subordinator), clitic_separator(predicate.CliticHost),
                predicate: FinitePredicate];
            require marker.RelativeSubordinator = Yes;
            require relative_clitic(predicate.CliticHost) = Yes;
            require predicate.person = Third;
            export number = predicate.number;
        }

        construction SubjectRelativeNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            require relative_number_chain(head.SubjectRelativeNumber, relative.number) = Yes;
            export SubjectRelativeNumber = deferred_relative_number(head.NominalAgreementNumber, relative.number);
            export BareGenitiveHost = No;
            form [head: Nominal, " ", relative: SubjectRelativeClause];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            require nominal_relative_agreement(head.NominalAgreementNumber, head.number, relative.number) = Yes;
        }

        construction FiniteObjectGap: FiniteObjectGap {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export ComparativeObjectGapLicence = head.ComparativeObjectGapLicence;
            require head.PredicateFrameUse = Yes;
            form [head: lexical(Verb)];
            use FiniteHeadAgreement;
            require head.frame = Transitive;
        }

        construction BareObjectGap: BareObjectGap {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export ComparativeObjectGapLicence = head.ComparativeObjectGapLicence;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            form [head: lexical(Verb)];
            require head.form = Plain;
            require head.frame = Transitive;
        }

        construction PastParticipleObjectGap: PastParticipleObjectGap {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export ComparativeObjectGapLicence = head.ComparativeObjectGapLicence;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            form [head: lexical(Verb)];
            require head.PredicateFrameUse = Yes;
            require head.form = PastParticiple;
            require head.frame = Transitive;
        }

        construction GerundParticipleObjectGap: GerundParticipleObjectGap {
            export ClauseComplementBoundary = None;
            export ComparativeObjectGapLicence = head.ComparativeObjectGapLicence;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            form [head: lexical(Verb)];
            require head.PredicateFrameUse = Yes;
            require head.form = GerundParticiple;
            require head.frame = Transitive;
        }

        construction AuxiliaryObjectGap: FiniteObjectGap {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
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
            export RelativeGapKind = Object;
            form [marker: lexical(Subordinator), " ", subject: NominativePhrase,
                clitic_separator(predicate.CliticHost), predicate: FiniteObjectGap];
            require marker.RelativeSubordinator = Yes;
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        construction ZeroObjectRelativeClause: ObjectRelativeClause {
            export RelativeGapKind = Object;
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: FiniteObjectGap];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        // R12f: the Subject and finite Object-gap Predicate form the
        // comparative-clause Complement, without a relative marker.
        construction FiniteGappedClause: FiniteGappedClause {
            require predicate.ComparativeObjectGapLicence = Yes;
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: FiniteObjectGap];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        // CGEL Ch. 12 §3.3(e), pp. 1045–1046: a bare relative can have
        // an Adjunct gap. The temporal head licence is declared by the noun.
        construction AdverbialGapRelativeClause: ObjectRelativeClause {
            form [clause: FiniteClause];
            export RelativeGapKind = TemporalAdjunct;
        }

        construction ObjectRelativeNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            require relative_head_licence(relative.RelativeGapKind,
                head.RelativisableAdjunctClass) = Yes;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = No;
            form [head: Nominal, " ", relative: ObjectRelativeClause];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
        }

        construction OmittedPlain: Ellipsis {
            form [];
            export form = Plain;
        }


        instance OvertComplement<Result, Predicate: predicate, Properties = NoComplementProperties>: [
            (BareComplement, BarePredicate, OvertComplementRealization),
            (ParticipialComplement, ParticipialPredicate),
            (PerfectComplement, PastParticiplePredicate),
        ] {
            use Properties;
            export PurposeAdjunctLicence = predicate.PurposeAdjunctLicence;
            export MatrixComplementLicence = predicate.MatrixComplementLicence;
            export SourceComplementSelection = predicate.SourceComplementSelection;
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
            form [omission: Ellipsis];
            require omission.form = Plain;
            export AuxiliaryComplementRealization = Elided;
            export PurposeAdjunctLicence = No;
            export MatrixComplementLicence = None;
            export SourceComplementSelection = No;
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
        }

        construction LargeCount: Cardinal {
            form [modifier: optional(CardinalModifier), head: lexical(Numeral)];
            require head.numeral_kind = GroupedArabic;
            require head.numeral_size = Large;
            export number = head.number;
        }

        construction NumericQuantityConjunct: QuantityConjunct {
            export CardinalMeasurementUse = DeterminerRequired;
            form [head: lexical(Numeral)];
            require head.numeral_kind = Arabic;
            require head.numeral_size = Small;
            export number = head.number;
        }

        construction CardinalQuantityConjunct: QuantityConjunct {
            export CardinalMeasurementUse = No;
            cost 0;
            form [value: Cardinal];
            export number = value.number;
        }

        policy VariableCountProperties {
            export number = Plural;
        }
        policy ScalarVariableProperties {
            export UnitBearing = No;
            export MeasureKind = Scalar;
        }
        policy VariableSubjectProperties {
            export number = Singular;
            export person = Third;
            export CaseUse = Nominative;
            export SubjectStructure = Phrase;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
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
        }
        schema QuantitativePrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: node,
                quantitative_modifier_separator(head.QuantitativeComplement)];
        }

        policy CardinalPrepositionComplement {
            export number = complement.number;
            require head.QuantitativeComplement = Cardinal;
        }

        policy QuantitativePrepositionComplement {
            export number = complement.number;
            require head.QuantitativeComplement = CardinalPrepositionPhrase;
        }

        policy ComparativePrepositionComplement {
            export number = complement.number;
            require head.ComparisonMarker = Ordering;
        }

        // CGEL Ch. 5 §11(e), p. 432: a bounding PP modifies a numeral.
        // Its adjectival Complement licence follows ticket rulings R9a–R9d.
        policy QuantitativeAdjectiveComplementLicence {
            require head.QuantitativeComplement = AdjectivePhrase;
        }

        instance QuantitativePrepositionPhrase<Result, Complement: complement, Properties>: [
            (CardinalPrepositionPhrase, Cardinal, CardinalPrepositionComplement),
            (QuantitativePrepositionPhrase, CardinalPrepositionPhrase,
                QuantitativePrepositionComplement),
            (ComparativePrepositionPhrase, Cardinal, ComparativePrepositionComplement),
            (CardinalModifier, QuantitativeAdjectiveComplement, QuantitativeAdjectiveComplementLicence),
        ] {
            use Properties;
        }

        // CGEL Ch. 5 §7.6, p. 386: a numeral and comparative are Conjuncts.
        // §3.4, p. 353 n. 13: with a count noun, it selects a plural head.
        construction NumeralComparativeCoordination: ComparativeQuantity {
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
            export MeasureModifierUse = head.MeasureModifierUse;
            form [head: lexical(Determinative), " ", complement: ComparativePrepositionPhrase];
            require head.ComparativeQuantityUse = Yes;
            export ComparativeQuantityStructure = Headed;
            export number = complement.number;
        }

        // MH2: a declared Determinative may coordinate with a cardinal count.
        construction CardinalDeterminativeCoordination: QuantitativeDeterminer {
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
            export Definiteness = Nondefinite;
            export NominalAdjunctDependent = None;
            cost 0;
            form [value: ComparativeQuantity];
            export DeterminerUse = cardinal_determiner_use(value.number);
            export DeterminerAgreementNumber = cardinal_agreement_number(value.number);
        }

        construction CardinalDeterminer: QuantitativeDeterminer {
            export Definiteness = Nondefinite;
            export NominalAdjunctDependent = None;
            cost 0;
            form [value: Cardinal];
            export DeterminerUse = cardinal_determiner_use(value.number);
            export DeterminerAgreementNumber = cardinal_agreement_number(value.number);
        }

        construction PrepositionDeterminer: QuantitativeDeterminer {
            export Definiteness = Nondefinite;
            export NominalAdjunctDependent = None;
            cost 0;
            form [value: QuantitativePrepositionPhrase];
            export DeterminerUse = cardinal_determiner_use(value.number);
            export DeterminerAgreementNumber = cardinal_agreement_number(value.number);
        }

        table cardinal_agreement_number(number) -> DeterminerAgreementNumber {
            (Singular) => Singular, (Plural) => Nominal,
        }
        table quantified_nominal_number(DeterminerAgreementNumber, NominalAgreementNumber, number) -> number {
            (Singular, Head, Singular) => Singular, (Singular, Head, Plural) => Singular,
            (Singular, Plural, Singular) => Singular, (Singular, Plural, Plural) => Singular,
            (Nominal, Head, Singular) => Singular, (Nominal, Head, Plural) => Plural,
            (Nominal, Plural, Singular) => Plural, (Nominal, Plural, Plural) => Plural,
        }
        table cardinal_determiner_use(number) -> DeterminerUse {
            (Singular) => SingularCount, (Plural) => PluralCount,
        }

        // CGEL Ch. 5 §11(c), p. 431: degree modification within a DP.
        construction DegreeDeterminativePhrase: QuantitativeDeterminer {
            export Definiteness = modifier.Definiteness;
            export NominalAdjunctDependent = None;
            export DeterminerAgreementNumber = head.DeterminerAgreementNumber;
            form [modifier: lexical(Determinative), " ", head: lexical(Determinative)];
            require modifier.DegreeUse = Modifier;
            require head.DegreeUse = Head;
            export DeterminerUse = head.DeterminerUse;
        }

        // Scalar Denotation includes measured amounts. Quantification of a
        // head with declared Measure Position supplies that amount; ordinary
        // quantified objects retain their head's Scalar Denotation.
        table quantified_scalar_denotation(ScalarDenotation, MeasurePosition) -> ScalarDenotation {
            (No, None) => No, (No, Before) => Yes,
            (Yes, None) => Yes, (Yes, Before) => Yes,
        }

        construction QuantifiedNounPhrase: NounPhrase {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = quantity.Definiteness;
            export UnitBearing = nominal_unit_bearing(head.MeasurePosition);
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = quantity.DeterminerAgreementNumber;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            form [quantity: QuantitativeDeterminer, " ", head: Nominal];
            require determiner_license(quantity.DeterminerUse, head.number, head.countability,
                head.DeterminerRequirement, quantity.NominalAdjunctDependent) = Yes;
            require head.DeterminerRequirement = No;
            export ScalarDenotation = quantified_scalar_denotation(head.ScalarDenotation, head.MeasurePosition);
            export SubjectStructure = Phrase;
            export BareGenitiveHost = head.BareGenitiveHost;
            export Targeting = head.Targeting;
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
            export PremodifierCoordination = No;
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
            export ScalarSign = Absent;
            form [value: UnsignedScalar];
        }

        construction PositiveScalar: ScalarComponent {
            export ScalarSign = Plus;
            onset Consonant;
            form ["+", value: UnsignedScalar];
        }

        construction NegativeScalar: ScalarComponent {
            onset Consonant;
            form ["-", value: UnsignedScalar] export ScalarSign = Hyphen;
            form ["−", value: UnsignedScalar] export ScalarSign = Minus;
        }

        construction SlashPair: SlashPair {
            form [left: ScalarComponent, "/", right: ScalarComponent];
        }

        policy UnitFreeMeasure {
            export UnitBearing = No;
        }
        schema SlashMeasure {
            form [pair: SlashPair];
            export MeasureKind = Pair;
        }
        instance SlashMeasure<Result, Properties = NoFeatures>: [
            (MeasurePhrase, UnitFreeMeasure), (PropertyValue),
        ] { use Properties; }

        table slash_premodifier_concord(SlashPremodifierUse, SlashPremodifierUse)
            -> SlashPremodifierUse {
            (Yes, Yes) => Yes, (Yes, No) => No, (No, Yes) => No, (No, No) => No,
        }

        construction SlashModifiedNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: SlashPair, " ", head: Nominal];
            require head.SlashPremodifierUse = Yes;
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
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
            export UnitBearing = value.UnitBearing;
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
            export UnitBearing = value.UnitBearing;
            form [value: NounPhrase];
            require value.ScalarDenotation = Yes;
        }

        // Arithmetic nominal operands follow ticket rulings R10a–R10d
        // (2026-10-09). CGEL's PP complementation does not supply this route.
        construction ArithmeticMeasure: MeasurePhrase {
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
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Unspecified;
            export UnitBearing = Yes;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export ScalarDenotation = Yes;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
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
            export PredicativeKind = Adjectival;
            form [value: ComparativeAdjectivePhrase];
        }

        construction ScalarCardinalValue: ScalarValue {
            export PredicativeKind = Nominal;
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }

        construction ScalarPropertyCoordination: ScalarValue {
            export PredicativeKind = Nominal;
            cost 0;
            form [value: ComparativeQuantity];
            require value.ComparativeQuantityStructure = Coordinated;
        }

        // CGEL Ch. 5 §11(d), p. 432 n. 48 classifies greater as an
        // Adjective. Its permission in this value position is a project ruling.
        construction AdjectivalScalarCoordination: ScalarValue {
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
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Unspecified;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export ScalarDenotation = head.ScalarDenotation;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: Nominal, " ", quantity: PropertyValue];
            require property_value_kind(head.PropertyValuePosition, quantity.MeasureKind) = Yes;
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
            form [governor: lexical(Adverb), " ", complement: MatrixComparativeComplement];
            require governor.MatrixComparisonUse = Yes;
            agree governor.ComparisonMarker = complement.ComparisonMarker;
            export ComparativeComplementForm = complement.ComparativeComplementForm;
        }
        schema MatchedComparativeConstituent {
            form [head: node, " ", comparison: MatrixComparativePhrase];
        }
        policy NominalComparisonHost {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = head.Definiteness;
            export UnitBearing = head.UnitBearing;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = head.DeterminerAgreementNumber;
            require comparison.ComparativeComplementForm = Nominal;
            export number = head.number;
            export person = head.person;
            export CaseUse = head.CaseUse;
            export Targeting = head.Targeting;
            export BareGenitiveHost = head.BareGenitiveHost;
            export SubjectStructure = head.SubjectStructure;
            export BarePrepositionUse = head.BarePrepositionUse;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export ScalarDenotation = head.ScalarDenotation;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
        }
        policy NonfiniteComparisonHost {
            require matched_nonfinite_comparison(head.form, comparison.ComparativeComplementForm) = Yes;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export SourceComplementSelection = head.SourceComplementSelection;
            export InternalisedComplementPresent = head.InternalisedComplementPresent;
            export ParticipialSupplementUse = No;
            export ParticipialUse = head.ParticipialUse;
            export form = head.form;
        }
        instance MatchedComparativeConstituent<Result, Head: head, Properties, BoundaryProperties = NoFeatures>: [
            (NounPhrase, Self, NominalComparisonHost),
            (SecondaryVerbPhrase, Self, NonfiniteComparisonHost, NoClauseComplementBoundary),
        ] { use Properties; use BoundaryProperties; }

        table coordinated_comparison_marker(ComparisonMarker, ComparisonMarker)
            -> ComparisonMarker {
            (Equality, Equality) => Equality, (Equality, Ordering) => Ordering,
            (Ordering, Equality) => Ordering, (Ordering, Ordering) => Ordering,
        }
        policy ComparativeGovernorComparison<Right, Source> {
            export ComparisonMarker = coordinated_comparison_marker(
                left.ComparisonMarker, Right.ComparisonMarker);
        }

        construction ComparativeGovernorHead: ComparativeGovernorHead {
            export ComparisonMarker = marker.ComparisonMarker;
            form [head: lexical(Adjective), " ", marker: lexical(Preposition)];
            require comparative_marker(head.frame, marker.ComparisonMarker) = Yes;
            require licence_comparative_complement(marker.PrepositionFunctionLicence) = Yes;
        }

        construction ScalarComparativeComplement: ComparativeComplement {
            export AdjectivePremodifierUse = Yes;
            form [value: ScalarMeasurePhrase];
        }

        // SC3 excludes a second, adjective-premodified quantity analysis.
        construction CardinalComparativeComplement: ComparativeComplement {
            export AdjectivePremodifierUse = No;
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }

        construction NominalComparativeComplement: ComparativeComplement {
            export AdjectivePremodifierUse = Yes;
            form [value: AccusativePhrase];
        }

        construction ComparativeAdjectivePhrase: ComparativeAdjectivePhrase {
            export ComparisonMarker = governor.ComparisonMarker;
            export AdjectivePremodifierUse = complement.AdjectivePremodifierUse;
            form [governor: ComparativeGovernorHead, " ", complement: ComparativeComplement];
        }

        construction ComparativeAdjective: AdjectivePhrase {
            export AdjectiveComplementClass = None;
            export AdjectiveAttributiveOnlyUse = No;
            export ImplicitComparativeUse = No;
            export PremodifierCoordination = No;
            form [phrase: ComparativeAdjectivePhrase];
            export AdjectiveStructure = Complemented;
            export AdjectiveOrigin = Ordinary;
            export AdjectivePremodifierUse = phrase.AdjectivePremodifierUse;
            export TargetingPremodifierUse = No;
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
            export NominalAdjunctHost = Yes;
            export FocusedUse = No;
            form [governor: lexical(Adverb), " ", head: lexical(Adverb), " ",
                marker: lexical(Preposition), " ", complement: FiniteClause];
            require governor.ComparisonMarker = Equality;
            require head.DurationUse = Yes;
            require marker.ExpandedComparisonMarker = Equality;
            export DurationUse = head.DurationUse;
            export VPFinalAdjunct = head.VPFinalAdjunct;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export UnmarkedConjunctLicence = governor.UnmarkedConjunctLicence;
        }

        construction Adverb: AdverbPhrase {
            export NominalAdjunctHost = Yes;
            require head.MatrixComparisonUse = No;
            export FocusedUse = No;
            form [head: lexical(Adverb)];
            export VPFinalAdjunct = head.VPFinalAdjunct;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export DurationUse = head.DurationUse;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
        }
        instance DepictivePredicate<Result, Properties>: [
            (FinitePredicate, FiniteHeadAgreement),
            (SecondaryVerbPhrase, OrdinarySecondaryHead),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance ComplementedDepictivePredicate<Result, Head: head, Properties>: [
            (FinitePredicate, FiniteDepictiveHost, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondaryDepictiveHost, SecondaryAdjunctProjection),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance AdverbPredicate<Result, Head: head, Modifier: modifier, Properties>: [
            (FinitePredicate, Self, AdverbPhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, AdverbPhrase, SecondaryAdjunctProjection),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }

        construction InitialAdverb: Clause {
            export SourceComplementSelection = clause.SourceComplementSelection;
            export ClauseCoordinationHost = Simple;
            form [modifier: AdverbPhrase, " ", clause: Clause];
            form [modifier: AdverbPhrase, ", ", clause: Clause];
            require modifier.ClauseInitialAdjunct = Yes;
            export UnmarkedConjunctLicence = modifier.UnmarkedConjunctLicence;
        }

        // Project analysis: a mana measure modifies the comparative determinative.
        construction MeasuredScalarLocation: ScalarLocationComplement {
            form [modifier: CostSymbols, " ", head: lexical(Determinative)];
            require head.MeasureModifierUse = Yes;
        }

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
        ] {
            export FocusedUse = No;
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

        instance SharedObjectComplement<Result, Head: head, Object: object, Properties>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase,
                OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedPredicativeComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, PredicativeComplement, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, PredicativeComplement,
                OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedLocativeComplement<Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, LocativeComplement, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, LocativeComplement,
                OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedManaComplement<Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, ManaPhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, ManaPhrase, OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedAmountComplement<Result, Head: head, Amount: amount, Properties>: [
            (FinitePredicate, FiniteSelectedHead, Amount, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, Amount, OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedMeasureComplement<Result, Head: head, Measure: measure, Properties>: [
            (FinitePredicate, FiniteSelectedHead, MeasurePhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, MeasurePhrase, OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedSlashMeasureComplement<Result, Head: head, Measure: measure, Properties>: [
            (FinitePredicate, FiniteSelectedHead, SlashPair, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, SlashPair, OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedGrantedAbilityComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, GrantedAbility, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, GrantedAbility, OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedAuxiliaryBareComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, BareComplement, SharedAuxiliaryAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, BareComplement,
                AuxiliarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedAuxiliaryParticipleComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, ParticipialComplement, SharedAuxiliaryAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, ParticipialComplement,
                SelectedAuxiliaryProperties),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedAuxiliaryPerfectComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, PerfectComplement, SharedAuxiliaryAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, PerfectComplement,
                SelectedAuxiliaryProperties),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        instance SharedObjectNameComplement<
            Result, Head: head, Object: object, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, Name, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase, Name,
                OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }
        table coordinated_locative_use(LocativeUse, LocativeUse) -> LocativeUse {
            (Yes, Yes) => Yes,
            (Yes, No) => No,
            (No, Yes) => No,
            (No, No) => No,
        }


        table coordinated_final_adjunct(VPFinalAdjunct, VPFinalAdjunct) -> VPFinalAdjunct {
            (Yes, Yes) => Yes,
            (Yes, No) => No,
            (No, Yes) => No,
            (No, No) => No,
        }

        table coordinated_initial_adjunct(ClauseInitialAdjunct,
            ClauseInitialAdjunct) -> ClauseInitialAdjunct {
            (Yes, Yes) => Yes,
            (Yes, No) => No,
            (No, Yes) => No,
            (No, No) => No,
        }

        table coordinated_duration_use(DurationUse, DurationUse) -> DurationUse {
            (Yes, Yes) => Yes, (Yes, No) => No, (No, Yes) => No, (No, No) => No,
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
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = Finite;
            export MatrixComplementRequirement = head.MatrixComplementRequirement;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
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

        // Correlative prefixes: Both binary only; main Clause Either only.
        // Nominal/AdjP deferred until pre-head placement is represented in summaries.
        instance EitherCoordination<Result, Member, Agreement = NoConcord,
            Properties = NoConcord, BoundaryProperties = NoFeatures>: [
            (Clause, Self, UnmarkedConjunctHead, CoordinatedClauseHost),
            (FinitePredicate, Self, FiniteConcord, NoConcord, NoClauseComplementBoundary),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties, NoConcord, NoClauseComplementBoundary),
            (NounPhrase, Self, NounCoordinationAgreement),
            (PrepositionPhrase, Self, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPermissions, UnmarkedConjunctHead),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self, NoConcord, CoordinatedQuotation),
            (InfinitiveComplement, Self, NoConcord),
            (FiniteSelectedHead, Self, FiniteConcord, SharedFrameConcord),
            (SecondarySelectedHead, Self, SecondaryConcord, SharedFrameConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase, AdjectiveAttributiveConcord),
            (FiniteObjectGap, Self, FiniteGapConcord, NoConcord, NoClauseComplementBoundary),
            (BareObjectGap, Self, BareGapConcord, NoConcord, NoClauseComplementBoundary),
            (CoordinatedFiniteClause, FiniteClause, NoConcord, ClauseCoordinationProperties),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
            use BoundaryProperties;
        }

        instance CorrelativeSeriesEnd<Result, Member, Agreement = NoConcord, Properties = NoConcord,
            Status = NoConcord>: [
            (CorrelativeClauseSeries, Clause, LeadingConjunct, CoordinatedClauseHost),
            (CorrelativeFinitePredicateSeries, FinitePredicate, FiniteConcord),
            (CorrelativeSecondaryVerbPhraseSeries, SecondaryVerbPhrase,
                SecondaryConjunctProperties),
            (CorrelativeNounPhraseSeries, NounPhrase, NounCoordinationAgreement,
                CoordinatorKindSummary),
            (CorrelativePrepositionPhraseSeries, PrepositionPhrase, ObliquePrepositionConcord, CoordinatorKindSummary),
            (CorrelativeAdverbPhraseSeries, AdverbPhrase, AdverbPermissions),
            (CorrelativeManaPhraseSeries, ManaPhrase),
            (CorrelativeCardinalSeries, Cardinal, CardinalListEnd),
            (CorrelativeAmountSeries, Amount),
            (CorrelativeMeasurePhraseSeries, MeasurePhrase, CoordinatedMeasureKind),
            (CorrelativeKeywordPhraseSeries, KeywordPhrase),
            (CorrelativeQuotedTextSeries, QuotedText),
            (CorrelativeInfinitiveComplementSeries, InfinitiveComplement),
            (CorrelativeFiniteSelectedHeadSeries, FiniteSelectedHead, FiniteConcord,
                SelectedFrameConcord),
            (CorrelativeSecondarySelectedHeadSeries, SecondarySelectedHead, SecondaryConcord,
                SelectedFrameConcord),
            (CorrelativeAdjectiveSeries, AdjectivePhrase, AdjectiveAttributiveConcord),
            (CorrelativeFiniteObjectGapSeries, FiniteObjectGap, FiniteGapConcord),
            (CorrelativeBareObjectGapSeries, BareObjectGap, BareGapConcord),
            (CorrelativeFiniteClauseSeries, FiniteClause, NoConcord, ClauseCoordinationProperties),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
            use Status(right, coordinator);
        }
        instance CorrelativeSeriesContinuation<
            Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord, Status = NoConcord>: [
            (CorrelativeClauseSeries, Clause, Self, UnmarkedConjunct, CoordinatedClauseHost),
            (CorrelativeFinitePredicateSeries, FinitePredicate, Self, FiniteConcord),
            (CorrelativeSecondaryVerbPhraseSeries, SecondaryVerbPhrase, Self,
                SecondaryConjunctProperties),
            (CorrelativeNounPhraseSeries, NounPhrase, Self, NounCoordinationAgreement,
                CoordinatorKindSummary),
            (CorrelativePrepositionPhraseSeries, PrepositionPhrase, Self, ObliquePrepositionConcord, CoordinatorKindSummary),
            (CorrelativeAdverbPhraseSeries, AdverbPhrase, Self, AdverbPermissions),
            (CorrelativeManaPhraseSeries, ManaPhrase, Self),
            (CorrelativeCardinalSeries, Cardinal, Self, CardinalListEnd),
            (CorrelativeAmountSeries, Amount, Self),
            (CorrelativeMeasurePhraseSeries, MeasurePhrase, Self, CoordinatedMeasureKind),
            (CorrelativeKeywordPhraseSeries, KeywordPhrase, Self),
            (CorrelativeQuotedTextSeries, QuotedText, Self),
            (CorrelativeInfinitiveComplementSeries, InfinitiveComplement, Self),
            (CorrelativeFiniteSelectedHeadSeries, FiniteSelectedHead, Self, FiniteConcord,
                SelectedFrameConcord),
            (CorrelativeSecondarySelectedHeadSeries, SecondarySelectedHead, Self, SecondaryConcord,
                SelectedFrameConcord),
            (CorrelativeAdjectiveSeries, AdjectivePhrase, Self, AdjectiveAttributiveConcord),
            (CorrelativeFiniteObjectGapSeries, FiniteObjectGap, Self, FiniteGapConcord),
            (CorrelativeBareObjectGapSeries, BareObjectGap, Self, BareGapConcord),
            (CorrelativeFiniteClauseSeries, FiniteClause, Self, NoConcord, ClauseCoordinationProperties),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
            use Status(rest, rest);
        }
        instance EitherSerialCoordination<Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord, BoundaryProperties = NoFeatures>: [
            (Clause, Self, CorrelativeClauseSeries, MarkedConjunctSeries, CoordinatedClauseHost),
            (FinitePredicate, Self, CorrelativeFinitePredicateSeries, FiniteConcord, NoConcord, NoClauseComplementBoundary),
            (SecondaryVerbPhrase, Self, CorrelativeSecondaryVerbPhraseSeries, SecondaryConjunctProperties, NoConcord, NoClauseComplementBoundary),
            (NounPhrase, Self, CorrelativeNounPhraseSeries, NounCoordinationAgreement),
            (PrepositionPhrase, Self, CorrelativePrepositionPhraseSeries, ObliquePrepositionConcord),
            (AdverbPhrase, Self, CorrelativeAdverbPhraseSeries, AdverbPermissions, UnmarkedConjunctHead),
            (ManaPhrase, Self, CorrelativeManaPhraseSeries),
            (Cardinal, Self, CorrelativeCardinalSeries, CardinalAgreement),
            (Amount, Self, CorrelativeAmountSeries),
            (MeasurePhrase, Self, CorrelativeMeasurePhraseSeries, CoordinatedMeasureKind),
            (KeywordPhrase, Self, CorrelativeKeywordPhraseSeries),
            (QuotedText, Self, CorrelativeQuotedTextSeries, NoConcord, CoordinatedQuotation),
            (InfinitiveComplement, Self, CorrelativeInfinitiveComplementSeries, NoConcord),
            (FiniteSelectedHead, Self, CorrelativeFiniteSelectedHeadSeries, FiniteConcord,
                SharedFrameConcord),
            (SecondarySelectedHead, Self, CorrelativeSecondarySelectedHeadSeries, SecondaryConcord,
                SharedFrameConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase, CorrelativeAdjectiveSeries, AdjectiveAttributiveConcord),
            (FiniteObjectGap, Self, CorrelativeFiniteObjectGapSeries, FiniteGapConcord, NoConcord, NoClauseComplementBoundary),
            (BareObjectGap, Self, CorrelativeBareObjectGapSeries, BareGapConcord, NoConcord, NoClauseComplementBoundary),
            (CoordinatedFiniteClause, FiniteClause, CorrelativeFiniteClauseSeries, NoConcord, ClauseCoordinationProperties),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
            use BoundaryProperties;
        }
        instance BothCoordination<Result, Member, Agreement = NoConcord, Properties = NoConcord, BoundaryProperties = NoFeatures>: [
            (FinitePredicate, Self, FiniteConcord, NoConcord, NoClauseComplementBoundary),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties, NoConcord, NoClauseComplementBoundary),
            (NounPhrase, Self, NounCoordinationAgreement),
            (PrepositionPhrase, Self, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPermissions, UnmarkedConjunctHead),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self, NoConcord, CoordinatedQuotation),
            (InfinitiveComplement, Self, NoConcord),
            (FiniteSelectedHead, Self, FiniteConcord, SharedFrameConcord),
            (SecondarySelectedHead, Self, SecondaryConcord, SharedFrameConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase, AdjectiveAttributiveConcord),
            (FiniteObjectGap, Self, FiniteGapConcord, NoConcord, NoClauseComplementBoundary),
            (BareObjectGap, Self, BareGapConcord, NoConcord, NoClauseComplementBoundary),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
            use BoundaryProperties;
        }

        instance NeitherCoordination<Result, Member, Agreement = NoConcord,
            Properties = NoConcord, BoundaryProperties = NoFeatures>: [
            (FinitePredicate, Self, FiniteConcord, NoConcord, NoClauseComplementBoundary),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties, NoConcord, NoClauseComplementBoundary),
            (NounPhrase, Self, NounCoordinationAgreement),
            (PrepositionPhrase, Self, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPermissions, UnmarkedConjunctHead),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self, NoConcord, CoordinatedQuotation),
            (InfinitiveComplement, Self, NoConcord),
            (FiniteSelectedHead, Self, FiniteConcord, SharedFrameConcord),
            (SecondarySelectedHead, Self, SecondaryConcord, SharedFrameConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase, AdjectiveAttributiveConcord),
            (FiniteObjectGap, Self, FiniteGapConcord, NoConcord, NoClauseComplementBoundary),
            (BareObjectGap, Self, BareGapConcord, NoConcord, NoClauseComplementBoundary),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
            use BoundaryProperties;
        }

        instance NeitherSerialCoordination<Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord, BoundaryProperties = NoFeatures>: [
            (FinitePredicate, Self, CorrelativeFinitePredicateSeries, FiniteConcord, NoConcord, NoClauseComplementBoundary),
            (SecondaryVerbPhrase, Self, CorrelativeSecondaryVerbPhraseSeries, SecondaryConjunctProperties, NoConcord, NoClauseComplementBoundary),
            (NounPhrase, Self, CorrelativeNounPhraseSeries, NounCoordinationAgreement),
            (PrepositionPhrase, Self, CorrelativePrepositionPhraseSeries, ObliquePrepositionConcord),
            (AdverbPhrase, Self, CorrelativeAdverbPhraseSeries, AdverbPermissions, UnmarkedConjunctHead),
            (ManaPhrase, Self, CorrelativeManaPhraseSeries),
            (Cardinal, Self, CorrelativeCardinalSeries, CardinalAgreement),
            (Amount, Self, CorrelativeAmountSeries),
            (MeasurePhrase, Self, CorrelativeMeasurePhraseSeries, CoordinatedMeasureKind),
            (KeywordPhrase, Self, CorrelativeKeywordPhraseSeries),
            (QuotedText, Self, CorrelativeQuotedTextSeries, NoConcord, CoordinatedQuotation),
            (InfinitiveComplement, Self, CorrelativeInfinitiveComplementSeries, NoConcord),
            (FiniteSelectedHead, Self, CorrelativeFiniteSelectedHeadSeries, FiniteConcord,
                SharedFrameConcord),
            (SecondarySelectedHead, Self, CorrelativeSecondarySelectedHeadSeries, SecondaryConcord,
                SharedFrameConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase, CorrelativeAdjectiveSeries, AdjectiveAttributiveConcord),
            (FiniteObjectGap, Self, CorrelativeFiniteObjectGapSeries, FiniteGapConcord, NoConcord, NoClauseComplementBoundary),
            (BareObjectGap, Self, CorrelativeBareObjectGapSeries, BareGapConcord, NoConcord, NoClauseComplementBoundary),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
            use BoundaryProperties;
        }

        // CGEL pp. 467–468: a genitive NP determines the following nominal.
        table genitive_marker(number, BareGenitiveHost, HostEnding) -> Selection {
            (Singular, No, Default) => Yes, (Singular, Yes, Default) => Yes,
            (Plural, Yes, PluralS) => Yes,
        }
        construction GenitiveNounPhrase: NounPhrase {
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Definite;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            require possessor.BarePrepositionUse = No;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
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
            export NounPhraseStructure = Other;
            export FocusedUse = No;
            export Definiteness = Definite;
            export UnitBearing = No;
            export ComparativePostmodifierUse = No;
            export DeterminerAgreementNumber = Nominal;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            export BarePrepositionUse = No;
            form [possessor: lexical(Pronoun), " ", head: Nominal];
            require possessor.case = Genitive;
            require possessor.NominalLicense = AnyNominal;
            use NounPhraseHeadAgreement;
            require resolved_ordinary_relative_number(head.SubjectRelativeNumber, head.NominalAgreementNumber, head.number) = Yes;
            require nondefinite_determiner_requirement(head.DeterminerRequirement) = Yes;
        }

        construction KeywordComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: KeywordPhrase];
            require head.KeywordComplement = Yes;
            use PrepositionHeadPermissions;
        }
        construction QuotedComplementPreposition: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: GrantedAbility];
            require head.QuotedComplement = Yes;
            require complement.GrantedAbilityKind = Quoted;
            require licence_modifier(head.PrepositionFunctionLicence) = Yes;
            export PrepositionFunctionLicence = Modifier;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export LocativeUse = head.LocativeUse;
        }
        instance SharedQuantitativeDeterminerComplement<Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, QuantitativeDeterminer, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, QuantitativeDeterminer, OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }

        instance SharedInfinitiveComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, InfinitiveComplement, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, InfinitiveComplement,
                OrdinarySelectedPredicate),
        ] {
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            use Properties;
        }

        construction AdjectivalCorrelativeComplement: PredicativeComplement {
            export PredicativeAdjunctUse = No;
            form [phrase: CorrelativeAdjectivePhrase];
            export PredicativeKind = Adjectival;
            require phrase.AdjectiveAttributiveOnlyUse = No;
        }

        construction CorrelativePostpositiveNominal: Nominal {
            export ComparativeModifierPresent = head.ComparativeModifierPresent;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
            export SubjectRelativeNumber = head.SubjectRelativeNumber;
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: CorrelativeAdjectivePhrase];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export WholeObjectSourcePresent = head.WholeObjectSourcePresent;
            require modifier.AdjectiveAttributiveOnlyUse = No;
        }

        construction SelectedPrepositionHead: SelectedPrepositionHead {
            form [head: lexical(Preposition)];
            require noun_phrase_complement(head.PrepositionComplement) = Yes;
            export HeadCoordination = No;
            use PrepositionHeadPermissions;
        }

        construction SharedPrepositionComplement: PrepositionPhrase {
            export InterveningAdjunctLicence = No;
            export ConnectiveAdjunctUse = No;
            export NominalAdjunctHost = Yes;
            export ClauseComplementBoundary = None;
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = No;
            export ObliqueNumber = None;
            form [head: SelectedPrepositionHead, " ", complement: AccusativePhrase];
            require head.HeadCoordination = Yes;
            export NominalComplementMarker = head.NominalComplementMarker;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export LocativeUse = head.LocativeUse;
            export SelectedPrepositionUse = selected_preposition_complement(head.SelectedPrepositionUse,
                complement.SelectedPrepositionUse);
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export WholeObjectSourcePresent = No;
        }

    }
}
