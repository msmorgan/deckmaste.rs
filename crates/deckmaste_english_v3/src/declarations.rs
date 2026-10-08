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
        feature DeterminerRequirement { No, Yes } default No;
        feature NumberTransparency { No, Yes } default No;
        feature NominalAgreementNumber { Head, Plural } default Head;
        feature DeterminerAgreementNumber { Nominal, Singular } default Nominal;
        feature QuantificationalDeterminer { No, Yes } default No;
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
        feature CoordinationKind { Additive, Alternative, Adversative }
        feature PrepositionComplement { NounPhrase, PredicativeNounPhrase, PrepositionPhrase, Intransitive } set;
        table noun_phrase_complement(PrepositionComplement) -> Selection contains NounPhrase => Yes;
        table preposition_phrase_complement(PrepositionComplement) -> Selection contains PrepositionPhrase => Yes;
        table predicative_noun_phrase_complement(PrepositionComplement) -> Selection contains PredicativeNounPhrase => Yes;
        table intransitive_preposition_complement(PrepositionComplement) -> Selection contains Intransitive => Yes;
        feature AdjectiveComplementClass { Manner }
        feature AdverbComplementClass { Duration }
        feature DurationUse { No, Yes } default No;
        feature FocusingModifierUse { Restrictive }
        feature FocusedUse { No, Yes } default No;
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
        }
        feature FusedHeadUse { No, Yes } default No;
        feature QuantitativeComplement { Cardinal, CardinalPrepositionPhrase }
        feature ComparativeQuantityUse { No, Yes } default No;
        feature ComparativeQuantityStructure { Coordinated, Headed }
        feature ScalarComparativeUse { No, Yes } default No;
        feature MeasureModifierUse { No, Yes } default No;
        feature PrepositionFunctionLicence { Adjunct, Modifier, NounComplement, VerbComplement,
            PredicativeComplement, Compound, ComparativeComplement, PreposedAdjunct, Supplement } set;
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
        feature SourceModifierPresent { No, Yes } default No;
        table source_modifier_presence(SourceModifierPresent, SourceModifierPresent) -> SourceModifierPresent {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        table nominal_source_modifier(SourceModifierPresent, SourceMarker) -> SourceModifierPresent {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
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
        feature FrequencyUnit { Yes }
        feature FiniteClauseComplement { Yes, Nonsequencing }
        table finite_clause_complement(FiniteClauseComplement) -> Selection {
            (Yes) => Yes, (Nonsequencing) => Yes,
        }
        feature IrrealisClauseComplement { No, Yes } default No;
        feature VerblessClauseComplement { No, Yes } default No;
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
        feature AdjectiveStructure { Simple, Complemented }
        feature RelativeSubordinator { Yes }
        feature ParticipialUse { Ordinary, BarePassive, Mixed }
        feature PastParticipialPremodifier { No, Yes } default No;
        feature AuxiliaryComplementRealization { Overt, Elided }
        feature AuxiliaryEllipsisLicence { No, Yes } default No;
        table licensed_auxiliary_complement(AuxiliaryEllipsisLicence,
            AuxiliaryComplementRealization) -> Selection {
            (No, Overt) => Yes, (Yes, Overt) => Yes, (Yes, Elided) => Yes,
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
        feature ScalarSign { Absent, Present } default Absent;
        feature MeasureOperator { Yes }
        feature ScalarDenotation { No, Yes } default No;
        feature MeasurePosition { None, Before } default None;
        feature PropertyValuePosition { None, Single, First, Second, Pair } default None;
        feature CardinalMeasurementUse { No, Yes } default No;
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
        feature ExpandedComparisonMarker { Equality }
        feature KeywordParameterClass { Nullary, Amount, Cost, Quality, Subject, AmountCost,
            QualityCost, Ability, Condition, CostPowerToughness, Instruction }
        feature NominalBareClass { None, Interval, Boundary }
        feature NominalAdjunctClass { None, Temporal, Manner } default None;
        feature SlashPremodifierUse { No, Yes } default Yes;
        feature NominalAdjunctDeterminer { None, Demonstrative, Temporal }
        feature BareSingularUse { No, Yes } default No;
        feature RelativeUse { No, Yes } default No;
        feature ExistentialUse { No, Yes } default No;
        feature KeywordMarker { None, For, From, Onto, To, Into, At, With, Of, On, Under }
        feature KeywordPayloadOrder { AfterHead, BeforeHead, BoundSuffix }
        feature BoundKeyword { No, Yes }
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
        feature NominalComplementMarker { None, Of } default None;
        feature BarePrepositionUse { No, Yes } default No;
        feature BareNominalComplement { No, Yes } default No;
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
        }
        feature NounPremodifier { Yes }
        feature VPFinalAdjunct { Yes, No }
        feature ClauseInitialAdjunct { Yes, No } default No;
        feature InfinitivalMarker { Yes }
        feature FrameUse { Object, Predicative, Locative, Mana, Amount, Measure, SlashMeasure,
            GrantedAbility, AuxiliaryBare, AuxiliaryParticiple, AuxiliaryPerfect, ObjectName,
            Cardinal, Infinitive }
        feature HeadCoordination { No, Yes }
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

        category Document();
        category Ability(LabelKind);
        category AbilityContinuation();
        category Paragraph();
        category ParagraphItem();
        category ParagraphContinuation();
        category Sentence();
        category Clause(UnmarkedConjunctLicence);
        category FiniteClause();
        category IrrealisClause();
        category IrrealisPredicate(number, person);
        category ExistentialExtension();
        category FinitePredicate(number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence);
        category ScalarLocationPredicate(number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence);
        category SecondaryVerbPhrase(form, ParticipialUse, InternalisedComplementPresent, PurposeAdjunctLicence, MatrixComplementLicence);
        category FiniteDepictiveHost(number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence);
        category SecondaryDepictiveHost(form, ParticipialUse, InternalisedComplementPresent, PurposeAdjunctLicence, MatrixComplementLicence);
        category BarePredicate(PurposeAdjunctLicence, MatrixComplementLicence);
        category ParticipialPredicate(PurposeAdjunctLicence, MatrixComplementLicence);
        category PastParticiplePredicate(PurposeAdjunctLicence, MatrixComplementLicence);
        category PredicativeComplement(PredicativeKind, PredicativeAdjunctUse);
        category DepictivePhrase(DepictiveKind);
        category DepictivePhraseSeries(DepictiveKind);
        category Ellipsis(form);
        category BareComplement(AuxiliaryComplementRealization, PurposeAdjunctLicence, MatrixComplementLicence);
        category ParticipialComplement(PurposeAdjunctLicence, MatrixComplementLicence);
        category PerfectComplement(PurposeAdjunctLicence, MatrixComplementLicence);
        category Nominal(number, NominalAgreementNumber, countability, Targeting, NominalAdjunctClass, SlashPremodifierUse,
            NumberTransparency, ObliqueNumber, BareGenitiveHost, DeterminerRequirement, MeasurePosition, PropertyValuePosition, CardinalMeasurementUse,
            BarePrepositionUse, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent);
        category VerbalPremodifier();
        category VerbalPremodifierSeries();
        category NounPremodifier();
        category NounPremodifierSeries();
        category PartitiveModifier();
        category NounPhrase(number, person, CaseUse, Targeting, BareGenitiveHost, SubjectStructure,
            BarePrepositionUse, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent);
        category NominativePhrase(number, person, CaseUse, SubjectStructure, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent);
        category AccusativePhrase(number, person, CaseUse, SelectedPrepositionUse, PredicativeAdjunctUse, SourceModifierPresent);
        category AdjectivePhrase(AdjectiveStructure, AdjectiveOrigin, TargetingPremodifierUse);
        category VerblessClause();
        category Name(number);
        category NamePredicate();
        category PrepositionPhrase(LocativeUse, PrepositionFunctionLicence, ComplementFunctionRestriction, ObliqueNumber,
            InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse,
            ClauseInitialAdjunct, FocusedUse, SourceComplementUse, SourceModifierPresent, SourceMarker, MatrixComplementRequirement, NominalExceptionComplementUse);
        category SourceComplement();
        category LocativeComplement();
        category MannerComplement();
        category FrequencyPhrase();
        category NominalAdjunctPhrase();
        category BareTemporalNominal(NominalBareClass);
        category BoundaryComplement();
        category SubjectRelativeClause(number);
        category ObjectRelativeClause();
        category FiniteObjectGap(number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence);
        category BareObjectGap(PurposeAdjunctLicence, MatrixComplementLicence);
        category Cardinal(number);
        category QuantitativeDeterminer(DeterminerUse);
        category CardinalPrepositionPhrase(number);
        category QuantitativePrepositionPhrase(number);
        category ComparativePrepositionPhrase(number);
        category QuantityConjunct(number);
        category ComparativeQuantity(number, ComparativeQuantityStructure);
        category ScalarLocationComplement();
        category HollowInfinitivalComplement();
        category PurposeInfinitival(FocusedUse);
        category ScalarValue();
        category PropertyValue(MeasureKind);
        category Amount();
        category MeasurePhrase(MeasureKind);
        category ScalarMeasurePhrase();
        category ScalarExtentComplement();
        category ScalarExtentMeasure();
        category UnsignedScalar();
        category ScalarComponent(ScalarSign);
        category SlashPair();
        category ComparativeGovernorHead();
        category ComparativeComplement();
        category MatrixComparativeComplement(ComparisonMarker, ComparativeComplementForm);
        category MatrixComparativePhrase(ComparativeComplementForm);
        category ComparativeAdjectivePhrase();
        category KeywordPhrase();
        category KeywordQuality(KeywordQualityNumber);
        category KeywordQualityPreposition(KeywordMarker, KeywordQualityNumber);
        category KeywordQualityPrepositionSeries(KeywordMarker, KeywordQualityNumber);
        category SelectedComplementTail(HeadCoordination);
        category SelectedComplementTailSeries(HeadCoordination);
        category KeywordContinuation();
        category Cost();
        category CostComponent();
        category CostContinuation();
        category CostSymbols();
        category CostSymbol();
        category ManaPhrase();
        category ManaSymbol();
        category Parenthetical();
        category QuotedText(QuotationStructure);
        category GrantedAbility(GrantedAbilityKind);
        category GrantedAbilitySeries(GrantedAbilityKind);
        category Mode();
        category Modes();
        category ModeContinuation();
        category TypeLine();
        category SupertypePrefix();
        category CardTypes();
        category CardTypeContinuation();
        category Subtypes();
        category SubtypeContinuation();
        category ClauseSeries(UnmarkedConjunctLicence);
        category FinitePredicateSeries(number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence);
        category SecondaryPredicateSeries(form, ParticipialUse, InternalisedComplementPresent, PurposeAdjunctLicence, MatrixComplementLicence);
        category NounPhraseSeries(number, person, CaseUse, Targeting, CoordinationKind, BareGenitiveHost, SubjectStructure, BarePrepositionUse, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent);
        category AdverbPhrase(VPFinalAdjunct, ClauseInitialAdjunct, DurationUse, UnmarkedConjunctLicence, FocusedUse);
        category InfinitiveComplement();
        category FiniteSelectedHead(number, person, FrameUse, HeadCoordination, CliticHost, AuxiliaryEllipsisLicence, PurposeAdjunctLicence, MatrixComplementLicence);
        category SecondarySelectedHead(form, FrameUse, HeadCoordination, AuxiliaryEllipsisLicence, PurposeAdjunctLicence, MatrixComplementLicence);
        category FiniteSelectedHeadSeries(number, person, FrameUse, CliticHost, AuxiliaryEllipsisLicence, PurposeAdjunctLicence, MatrixComplementLicence);
        category SecondarySelectedHeadSeries(form, FrameUse, AuxiliaryEllipsisLicence, PurposeAdjunctLicence, MatrixComplementLicence);
        category ManaPhraseSeries();
        category CardinalSeries(number, CoordinationKind);
        category AmountSeries();
        category MeasurePhraseSeries(MeasureKind);
        category KeywordPhraseSeries();
        category QuotedTextSeries();
        category NominalSeries(
            number, NominalAgreementNumber, countability, Targeting, NominalAdjunctClass, SlashPremodifierUse,
            NumberTransparency, ObliqueNumber, CoordinationKind, BareGenitiveHost, DeterminerRequirement, MeasurePosition, PropertyValuePosition, CardinalMeasurementUse,
            BarePrepositionUse, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation
        , SourceModifierPresent);
        category AdjectivePhraseSeries(AdjectiveStructure, AdjectiveOrigin, TargetingPremodifierUse);
        category PrepositionPhraseSeries(LocativeUse, PrepositionFunctionLicence, ComplementFunctionRestriction, ObliqueNumber, CoordinationKind,
            InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, ClauseInitialAdjunct, FocusedUse, SourceComplementUse, SourceModifierPresent, SourceMarker, MatrixComplementRequirement, NominalExceptionComplementUse);
        category AdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct, DurationUse, UnmarkedConjunctLicence, FocusedUse);
        category InfinitiveComplementSeries();
        category FrequencyPhraseSeries();
        category FiniteObjectGapSeries(number, person, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence);
        category BareObjectGapSeries(PurposeAdjunctLicence, MatrixComplementLicence);
        category CoordinatedFiniteClause(GeneralCoordination);
        category FiniteClauseSeries(GeneralCoordination);
        category PredicativeComplementSeries(PredicativeKind, PredicativeAdjunctUse);
        category CorrelativeClauseSeries(CorrelativeCoordinator, UnmarkedConjunctLicence);
        category CorrelativeFinitePredicateSeries(number, person, CorrelativeCoordinator, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence);
        category CorrelativeSecondaryVerbPhraseSeries(form, ParticipialUse, CorrelativeCoordinator,
            InternalisedComplementPresent, PurposeAdjunctLicence, MatrixComplementLicence);
        category CorrelativeNounPhraseSeries(number, person, CaseUse, Targeting,
            CorrelativeCoordinator, CoordinationKind, BareGenitiveHost, SubjectStructure, BarePrepositionUse, SelectedPrepositionUse, PredicativeAdjunctUse, ScalarDenotation, SourceModifierPresent);
        category CorrelativePrepositionPhraseSeries(LocativeUse, PrepositionFunctionLicence, ComplementFunctionRestriction, ObliqueNumber, CoordinationKind,
            CorrelativeCoordinator, InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, ClauseInitialAdjunct, FocusedUse, SourceComplementUse, SourceModifierPresent, SourceMarker, MatrixComplementRequirement, NominalExceptionComplementUse);
        category CorrelativeAdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct,
            DurationUse, CorrelativeCoordinator, FocusedUse);
        category CorrelativeManaPhraseSeries(CorrelativeCoordinator);
        category CorrelativeCardinalSeries(number, CorrelativeCoordinator, CoordinationKind);
        category CorrelativeAmountSeries(CorrelativeCoordinator);
        category CorrelativeMeasurePhraseSeries(MeasureKind, CorrelativeCoordinator);
        category CorrelativeKeywordPhraseSeries(CorrelativeCoordinator);
        category CorrelativeQuotedTextSeries(CorrelativeCoordinator);
        category CorrelativeInfinitiveComplementSeries(CorrelativeCoordinator);
        category CorrelativeFiniteSelectedHeadSeries(number, person, FrameUse,
            CorrelativeCoordinator, CliticHost, AuxiliaryEllipsisLicence, PurposeAdjunctLicence, MatrixComplementLicence);
        category CorrelativeSecondarySelectedHeadSeries(form, FrameUse, CorrelativeCoordinator, AuxiliaryEllipsisLicence, PurposeAdjunctLicence, MatrixComplementLicence);
        category CorrelativeAdjectivePhrase();
        category CorrelativeAdjectiveSeries(CorrelativeCoordinator);
        category CorrelativeFiniteObjectGapSeries(number, person, CorrelativeCoordinator, CliticHost, PurposeAdjunctLicence, MatrixComplementLicence);
        category CorrelativeBareObjectGapSeries(CorrelativeCoordinator, PurposeAdjunctLicence, MatrixComplementLicence);
        category SelectedPrepositionHead(LocativeUse, PrepositionFunctionLicence, HeadCoordination,
            InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, SourceComplementUse, SourceModifierPresent, SourceMarker);
        category SelectedPrepositionHeadSeries(LocativeUse, PrepositionFunctionLicence, InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, PredicativeAdjunctUse, SourceComplementUse, SourceModifierPresent, SourceMarker);
        category CorrelativeFiniteClauseSeries(CorrelativeCoordinator, GeneralCoordination);

        frame_category NounPhrase = AccusativePhrase;
        frame_category SourceFreeObject = AccusativePhrase where SourceModifierPresent = No;
        frame_category NominalPredicativePreposition = PrepositionPhrase where ComplementFunctionRestriction = PreposedOrPredicative;
        frame_category PowerToughnessAdjustment = SlashPair;
        frame_category MeasurePhrase = ScalarMeasurePhrase;

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
        construction ScalarMeasurePhrase: ScalarMeasurePhrase {
            form [head: MeasurePhrase];
            require head.MeasureKind = Scalar;
        }

        frame Intransitive = Predicate();
        frame Transitive = Predicate(Object(NounPhrase));
        frame ManaComplement = Predicate(Complement(ManaPhrase));
        frame ObjectName = Predicate(Object(NounPhrase), Complement(Name));
        frame BareNominal = Nominal();
        frame NominalSymbols = Nominal(Marked(Preposition, Of, Complement(CostSymbols)));
        frame NominalInfinitive = Nominal(Complement(InfinitiveComplement));
        frame NominalPreposition = Nominal(Marked(Preposition, Of, Complement(NounPhrase)));
        frame Predicative = Predicate(Complement(PredicativeComplement));
        frame NominalPredicativePreposition = Predicate(Complement(NominalPredicativePreposition));
        frame Existential = Existential(Complement(NounPhrase), Optional(Complement(LocativeComplement)));
        frame AdjectivalPrepositionComplement = Adjectival(
            Marked(Preposition, By, Complement(NounPhrase)));
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
        frame CardinalComplement = Predicate(Complement(Cardinal));
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

        table determiner_license(DeterminerUse, number, countability) -> Selection {
            (SingularCount, Singular, Count) => Yes,
            (Unrestricted, Singular, Count) => Yes,
            (Unrestricted, Plural, Count) => Yes,
            (Unrestricted, Singular, Mass) => Yes,
            (PluralOrMass, Plural, Count) => Yes,
            (PluralOrMass, Singular, Mass) => Yes,
            (PluralCount, Plural, Count) => Yes,
            (Mass, Singular, Mass) => Yes,
            (Singular, Singular, Count) => Yes,
            (Singular, Singular, Mass) => Yes,
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
            (CardinalComplement) => Cardinal,
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
            (No, Overt) => Yes, (No, Elided) => Yes, (Yes, Overt) => Yes,
        }

        policy FiniteHeadAgreement {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
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
            export form = secondary_form(head.form);
        }

        policy NominalHeadFeatures {
            export NominalAgreementNumber = head.NominalAgreementNumber;
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            export ObliqueNumber = head.ObliqueNumber;
        }

        policy NominalHeadProperties {
            export PropertyValuePosition = None;
        }

        policy PostmodifiedNominalProperties {
            export NominalAgreementNumber = head.NominalAgreementNumber;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = nominal_source_modifier(head.SourceModifierPresent, modifier.SourceMarker);
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
            export PropertyValuePosition = None;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
        }

        policy PredicateHostAgreement {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export CliticHost = head.CliticHost;
            export number = head.number;
            export person = head.person;
        }

        policy PredicateHeadAgreement {
            export number = head.number;
            export person = head.person;
        }

        policy FiniteAuxiliaryAgreement {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        policy SharedAuxiliaryAgreement {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export CliticHost = head.CliticHost;
            export number = head.number;
            export person = head.person;
        }

        policy SecondaryAuxiliaryProperties {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export form = secondary_form(head.form);
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy SelectedAuxiliaryProperties {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export form = head.form;
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy PurposeHostProjection {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
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
            export InternalisedComplementPresent = head.InternalisedComplementPresent;
            export ParticipialUse = head.ParticipialUse;
            export form = secondary_form(head.form);
        }

        policy SecondaryPrepositionProjection {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export InternalisedComplementPresent = internalised_complement_presence(
                head.InternalisedComplementPresent, modifier.InternalisedComplementMarker);
            export ParticipialUse = head.ParticipialUse;
            export form = secondary_form(head.form);
        }

        policy SecondaryAdjunctProjection {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export InternalisedComplementPresent = head.InternalisedComplementPresent;
            export ParticipialUse = head.ParticipialUse;
            export form = head.form;
        }

        policy OrdinarySecondaryHead {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export form = secondary_form(head.form);
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy OrdinarySelectedPredicate {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export form = head.form;
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
            export NominalAgreementNumber = Head;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = No;
            export MeasurePosition = head.MeasurePosition;
            export PropertyValuePosition = head.PropertyValuePosition;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export number = head.number;
            export countability = head.countability;
            export Targeting = No;
            export NominalAdjunctClass = head.NominalAdjunctClass;
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
            export NominalComplementMarker = head.NominalComplementMarker;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export LocativeUse = head.LocativeUse;
        }

        policy AdjectiveStructureMerge<Right, Source> {
            export AdjectiveOrigin = coordinated_adjective_origin(left.AdjectiveOrigin, Right.AdjectiveOrigin);
            export AdjectiveStructure = coordinated_adjective_structure(left.AdjectiveStructure,
                Right.AdjectiveStructure);
            export TargetingPremodifierUse = coordinated_targeting_premodifier(
                left.TargetingPremodifierUse, Right.TargetingPremodifierUse);
        }

        policy AdverbPermissions<Right, Source> {
            export FocusedUse = No;
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
            agree left.MeasureKind = Right.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        policy CoordinatorKindSummary<Right, Source> {
            export CoordinationKind = Source.CoordinationKind;
        }

        policy FiniteConcord<Right, Source> {
            export PurposeAdjunctLicence = coordinated_purpose_licence(
                left.PurposeAdjunctLicence, Right.PurposeAdjunctLicence);
            export MatrixComplementLicence = matrix_host_concord(
                left.MatrixComplementLicence, Right.MatrixComplementLicence);
            require Right.CliticHost = Free;
            export CliticHost = left.CliticHost;
            agree left.number = Right.number;
            agree left.person = Right.person;
            export number = left.number;
            export person = left.person;
        }

        policy NoConcord<Right, Source> {

        }

        table coordinated_determiner_requirement(DeterminerRequirement, DeterminerRequirement)
            -> DeterminerRequirement {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
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
        }
        policy NominalConcord<Right, Source> {
            export NominalAgreementNumber = coordinated_nominal_agreement(
                Source.CoordinationKind, Right.NominalAgreementNumber);
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
            export ScalarDenotation = coordinated_scalar_denotation(left.ScalarDenotation, Right.ScalarDenotation);
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export SourceModifierPresent = source_modifier_presence(left.SourceModifierPresent,
                Right.SourceModifierPresent);
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            export CardinalMeasurementUse = coordinated_cardinal_measurement(
                left.CardinalMeasurementUse, Right.CardinalMeasurementUse);
            export MeasurePosition = coordinated_measure_position(
                left.MeasurePosition, Right.MeasurePosition);
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
            export NominalAdjunctClass = nominal_adjunct_concord(left.NominalAdjunctClass,
                Right.NominalAdjunctClass);
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
        policy NounCoordinationAgreement<Right, Source> {
            export PredicativeAdjunctUse = predicative_adjunct_concord(
                left.PredicativeAdjunctUse, Right.PredicativeAdjunctUse);
            export ScalarDenotation = coordinated_scalar_denotation(left.ScalarDenotation, Right.ScalarDenotation);
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export SourceModifierPresent = source_modifier_presence(left.SourceModifierPresent,
                Right.SourceModifierPresent);
            agree left.BarePrepositionUse = Right.BarePrepositionUse;
            export BarePrepositionUse = left.BarePrepositionUse;
            export SubjectStructure = Phrase;
            export BareGenitiveHost = No;
            export Targeting = coordinated_targeting(left.Targeting, Right.Targeting);
            export number = coordinate_number(Source.CoordinationKind, left.number,
                Right.number);
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
            export FocusedUse = No;
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct,
                Right.ClauseInitialAdjunct);
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export SourceModifierPresent = source_modifier_presence(left.SourceModifierPresent,
                Right.SourceModifierPresent);
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
            agree left.form = Right.form;
            export form = left.form;
        }

        policy SecondaryConjunctProperties<Right, Source> {
            export PurposeAdjunctLicence = coordinated_purpose_licence(
                left.PurposeAdjunctLicence, Right.PurposeAdjunctLicence);
            export MatrixComplementLicence = matrix_host_concord(
                left.MatrixComplementLicence, Right.MatrixComplementLicence);
            agree left.form = Right.form;
            export form = left.form;
            export InternalisedComplementPresent = complement_presence(
                left.InternalisedComplementPresent, Right.InternalisedComplementPresent);
            export ParticipialUse = coordinated_participial_use(left.ParticipialUse,
                Right.ParticipialUse);
        }

        policy SelectedFrameConcord<Right, Source> {
            export AuxiliaryEllipsisLicence = auxiliary_ellipsis_concord(
                left.AuxiliaryEllipsisLicence, Right.AuxiliaryEllipsisLicence);
            agree left.FrameUse = Right.FrameUse;
            export FrameUse = left.FrameUse;
        }

        policy SharedFrameConcord<Right, Source> {
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
            (Plain, Overt) => Yes, (Plain, Elided) => Yes,
            (Present, Overt) => Yes, (Present, Elided) => Yes,
            (Preterite, Overt) => Yes, (Preterite, Elided) => Yes,
            (PastParticiple, Overt) => Yes, (PastParticiple, Elided) => Yes,
            (GerundParticiple, Overt) => Yes,
        }
        policy AuxiliarySelectedPredicate {
            export PurposeAdjunctLicence = complement.PurposeAdjunctLicence;
            export MatrixComplementLicence = complement.MatrixComplementLicence;
            export form = head.form;
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
            require auxiliary_realization(head.form, complement.AuxiliaryComplementRealization)
                = Yes;
        }

        schema BareAuxiliaryPredicate {
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

        schema NominalAdjunctPredicate {
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
            form [head: node, " ", modifier: node];
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
        ] { use Properties; }

        construction MatrixComplementPredicate: SecondaryVerbPhrase {
            form [head: SecondaryVerbPhrase, " ", modifier: PrepositionPhrase];
            require modifier.MatrixComplementRequirement = Internalised;
            require head.ParticipialUse = BarePassive;
            require head.InternalisedComplementPresent = No;
            export form = head.form;
            export ParticipialUse = head.ParticipialUse;
            export InternalisedComplementPresent = Yes;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
        }

        construction InternalisedComplementPredicate: SecondaryVerbPhrase {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            form [head: SecondaryVerbPhrase, " ", modifier: PrepositionPhrase];
            require head.ParticipialUse = BarePassive;
            require head.InternalisedComplementPresent = No;
            require modifier.InternalisedComplementMarker = Yes;
            export form = head.form;
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

        schema SharedCardinalComplement {
            form [head: node, " ", complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Cardinal;
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
        }

        construction AbilityContinuation: AbilityContinuation {
            form ["\n", ability: Ability];
        }

        construction OrdinaryAbility: Ability {
            export LabelKind = None;
            form [body: Paragraph];
        }

        construction Paragraph: Paragraph {
            form [first: ParagraphItem, rest: repeat(ParagraphContinuation, "")];
        }

        construction ParagraphContinuation: ParagraphContinuation {
            form [" ", item: ParagraphItem];
        }

        construction SentenceItem: ParagraphItem {
            form [sentence: Sentence];
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
            form [cost: Cost, ": ", body: Paragraph];
        }

        construction AbilityWordHead: Ability {
            boundary Initial;
            form [head: lexical(Keyword), " — ", body: Ability];
            require head.LabelKind = AbilityWord;
            require body.LabelKind = None;
            export LabelKind = AbilityWord;
        }

        construction FlavorWordHead: Ability {
            cost 100;
            boundary Initial;
            form [head: lexical(FlavorWord), " ", body: Ability];
            require body.LabelKind = None;
            export LabelKind = FlavorWord;
        }

        construction Cost: Cost {
            form [first: CostComponent, rest: repeat(CostContinuation, "")];
        }

        construction CostContinuation: CostContinuation {
            form [", ", component: CostComponent];
        }

        construction ActionCost: CostComponent {
            boundary Initial;
            form [action: BarePredicate];
        }

        construction SymbolCost: CostComponent {
            form [symbols: CostSymbols];
        }

        // Oracle document notation, under the signed-numeral-cost project analysis.
        construction SignedScalarCost: CostComponent {
            form [value: ScalarComponent];
            require value.ScalarSign = Present;
        }

        construction ZeroScalarCost: CostComponent {
            form ["0"];
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
            boundary Initial;
            form [first: KeywordPhrase, rest: repeat(KeywordContinuation, "")];
        }

        construction KeywordContinuation: KeywordContinuation {
            form [", ", keyword: KeywordPhrase];
            form ["; ", keyword: KeywordPhrase];
        }

        construction BareKeyword: KeywordPhrase {
            form [head: lexical(Keyword)];
            require head.KeywordParameterClass = Nullary;
        }

        construction AmountKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", amount: MeasurePhrase];
            require head.KeywordParameterClass = Amount;
        }

        construction CostKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", cost: CostSymbols];
            require head.KeywordParameterClass = Cost;
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

        construction SubjectKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", subject: Nominal];
            require head.KeywordParameterClass = Subject;
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
            export KeywordParameterClass = Ability;
        }
        construction ConditionKeywordPayload: ClausalKeywordPayload {
            form [body: Sentence];
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
            form ["• ", body: Paragraph];
        }

        construction ModeContinuation: ModeContinuation {
            form ["\n", mode: Mode];
        }

        construction Modes: Modes {
            form [first: Mode, rest: repeat(ModeContinuation, "")];
        }

        construction ModalItem: ParagraphItem {
            boundary Initial;
            form [header: Clause, " —\n", modes: Modes];
        }

        separator sentence_terminal(terminal_punctuation) {
            (None) => ".", (QuotedFullStop) => "",
        }
        construction Sentence: Sentence {
            boundary Initial;
            form [clause: Clause, sentence_terminal(clause.terminal_punctuation)];
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
        }
        instance Supplementation<Result, Anchor: anchor, Properties>: [
            (Sentence, Clause, NoFeatures),
            (Ability, KeywordPhrase, UnlabelledSupplementation),
        ] {
            use Properties;
        }

        construction Declarative: Clause {
            form [clause: FiniteClause];
            export UnmarkedConjunctLicence = Yes;
        }

        construction Imperative: Clause {
            form [predicate: BarePredicate];
            export UnmarkedConjunctLicence = Yes;
        }

        construction FiniteClause: FiniteClause {
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: FinitePredicate];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
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
        construction ExistentialClause: FiniteClause {
            form [subject: lexical(Pronoun), lexical_clitic_separator(head.Clitic),
                head: lexical(Verb), " ", displaced_subject: AccusativePhrase,
                extension: optional(ExistentialExtension)];
            require subject.ExistentialUse = Yes;
            require head.frame = Existential;
            require head.finiteness = Finite;
            require tensed_form(head.form) = Yes;
            agree displaced_subject.number = head.number;
            agree displaced_subject.person = head.person;
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

        construction InitialPreposition: Clause {
            form [dependent: PrepositionPhrase, ", ", clause: Clause];
            form [dependent: PrepositionPhrase, " ", clause: Clause]
                require dependent.ClauseInitialAdjunct = Yes;
            require licence_preposed_adjunct(dependent.PrepositionFunctionLicence) = Yes;
            require preposed_complement_function(dependent.ComplementFunctionRestriction) = Yes;
            require dependent.MatrixComplementRequirement = None;
            require dependent.SelectedPrepositionUse = No;
            export UnmarkedConjunctLicence = Yes;
        }

        construction ClausalPreposition: Clause {
            form [clause: Clause, " ", dependent: PrepositionPhrase];
            require licence_adjunct(dependent.PrepositionFunctionLicence) = Yes;
            require adjunct_complement_function(dependent.ComplementFunctionRestriction,
                dependent.PredicativeAdjunctUse) = Yes;
            require dependent.MatrixComplementRequirement = None;
            require dependent.SelectedPrepositionUse = No;
            export UnmarkedConjunctLicence = clause.UnmarkedConjunctLicence;
        }

        construction MedialSupplementation: Clause {
            form [anchor: Clause, ", ", supplement: PrepositionPhrase, ", ",
                coordinator: lexical(Coordinator), " ", continuation: Clause];
            require licence_supplement(supplement.PrepositionFunctionLicence) = Yes;
            require coordinator.NoncorrelativeCoordination = Yes;
            require anchor.terminal_punctuation = None;
            export UnmarkedConjunctLicence = anchor.UnmarkedConjunctLicence;
        }

        construction ClauseCoordination: Clause {
            form [left: Clause, ", ", coordinator: lexical(Coordinator), " ", right: Clause];
            form [left: Clause, " ", coordinator: lexical(Coordinator), " ", right: Clause]
                require coordinator.BareCoordination = Yes;
            require coordinator.NoncorrelativeCoordination = Yes;
            export UnmarkedConjunctLicence = left.UnmarkedConjunctLicence;
        }
        instance CoordinationSeriesEnd<Result, Member, Agreement = NoConcord,
            Properties = NoConcord, Distribution = GeneralCoordinatorDistribution>: [
            (ClauseSeries, Clause, NoConcord, LeadingConjunct, NoConcord),
            (KeywordQualityPrepositionSeries, KeywordQualityPreposition, QualityPrepositionConcord),
            (SelectedComplementTailSeries, SelectedComplementTail, SelectedTailConcord,
                PrimitiveRightTail),
            (FinitePredicateSeries, FinitePredicate, FiniteConcord, NoConcord, NoConcord),
            (SecondaryPredicateSeries, SecondaryVerbPhrase, SecondaryConjunctProperties, NoConcord, NoConcord),
            (NounPhraseSeries, NounPhrase, NounCoordinationAgreement, CoordinatorKindSummary),
            (VerbalPremodifierSeries, VerbalPremodifier),
            (NounPremodifierSeries, NounPremodifier),
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
            (NominalSeries, Nominal, NominalConcord, CoordinatorKindSummary),
            (AdjectivePhraseSeries, AdjectivePhrase, AdjectiveStructureMerge),
            (PrepositionPhraseSeries, PrepositionPhrase, ObliquePrepositionConcord, CoordinatorKindSummary),
            (AdverbPhraseSeries, AdverbPhrase, AdverbPermissions, LeadingConjunct),
            (InfinitiveComplementSeries, InfinitiveComplement, NoConcord),
            (FrequencyPhraseSeries, FrequencyPhrase),
            (FiniteObjectGapSeries, FiniteObjectGap, FiniteConcord, NoConcord, NoConcord),
            (BareObjectGapSeries, BareObjectGap, PurposeConcord, NoConcord, NoConcord),
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
            (ClauseSeries, Clause, Self, UnmarkedConjunct),
            (KeywordQualityPrepositionSeries, KeywordQualityPreposition, Self,
                QualityPrepositionConcord),
            (SelectedComplementTailSeries, SelectedComplementTail, Self, SelectedTailConcord),
            (FinitePredicateSeries, FinitePredicate, Self, FiniteConcord),
            (SecondaryPredicateSeries, SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
            (NounPhraseSeries, NounPhrase, Self, NounCoordinationAgreement, CoordinatorKindSummary),
            (VerbalPremodifierSeries, VerbalPremodifier, Self),
            (NounPremodifierSeries, NounPremodifier, Self),
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
            (NominalSeries, Nominal, Self, NominalConcord, CoordinatorKindSummary),
            (AdjectivePhraseSeries, AdjectivePhrase, Self, AdjectiveStructureMerge),
            (PrepositionPhraseSeries, PrepositionPhrase, Self, ObliquePrepositionConcord, CoordinatorKindSummary),
            (AdverbPhraseSeries, AdverbPhrase, Self, AdverbPermissions, LeadingConjunct),
            (InfinitiveComplementSeries, InfinitiveComplement, Self, NoConcord),
            (FrequencyPhraseSeries, FrequencyPhrase, Self),
            (FiniteObjectGapSeries, FiniteObjectGap, Self, FiniteConcord),
            (BareObjectGapSeries, BareObjectGap, Self, PurposeConcord),
            (FiniteClauseSeries, FiniteClause, Self, NoConcord, ClauseCoordinationProperties),
            (DepictivePhraseSeries, DepictivePhrase, Self, DepictiveListEnd),
            (PredicativeComplementSeries, PredicativeComplement, Self, PredicativeListEnd),
            (SelectedPrepositionHeadSeries, SelectedPrepositionHead, Self, PrepositionPermissions),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
        }
        instance SerialCoordination<Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord>: [
            (Clause, Self, ClauseSeries, UnmarkedConjunct),
            (KeywordQualityPreposition, Self, KeywordQualityPrepositionSeries,
                QualityPrepositionConcord),
            (SelectedComplementTail, Self, SelectedComplementTailSeries, SelectedTailConcord),
            (FinitePredicate, Self, FinitePredicateSeries, FiniteConcord),
            (SecondaryVerbPhrase, Self, SecondaryPredicateSeries, SecondaryConjunctProperties),
            (NounPhrase, Self, NounPhraseSeries, NounCoordinationAgreement),
            (VerbalPremodifier, Self, VerbalPremodifierSeries),
            (NounPremodifier, Self, NounPremodifierSeries),
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
            (Nominal, Self, NominalSeries, NominalConcord),
            (AdjectivePhrase, Self, AdjectivePhraseSeries, AdjectiveStructureMerge),
            (PrepositionPhrase, Self, PrepositionPhraseSeries, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPhraseSeries, AdverbPermissions, LeadingConjunct),
            (InfinitiveComplement, Self, InfinitiveComplementSeries, NoConcord),
            (FrequencyPhrase, Self, FrequencyPhraseSeries),
            (FiniteObjectGap, Self, FiniteObjectGapSeries, FiniteConcord),
            (BareObjectGap, Self, BareObjectGapSeries, PurposeConcord),
            (CoordinatedFiniteClause, FiniteClause, FiniteClauseSeries, NoConcord, ClauseCoordinationProperties),
            (DepictivePhrase, Self, DepictivePhraseSeries, UnlikeDepictives),
            (PredicativeComplement, Self, PredicativeComplementSeries, UnlikePredicatives),
            (SelectedPrepositionHead, Self, SelectedPrepositionHeadSeries, PrepositionPermissions,
                SharedHeadStatus),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
        }

        construction Noun: Nominal {
            export BareGenitiveHost = head.BareGenitiveHost;
            form [head: lexical(Noun)];
            require head.framing = Unframed;
            use UnmodifiedNominalProperties;
        }

        construction BareFramedNoun: Nominal {
            export BareGenitiveHost = head.BareGenitiveHost;
            form [head: lexical(Noun)];
            require head.frame = BareNominal;
            use UnmodifiedNominalProperties;
        }

        // CGEL Ch. 14 §8.2, pp. 1259–1260: selected subjectless
        // to-infinitivals are internal Complements of nouns.
        construction InfinitiveComplementNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: lexical(Noun), " ", complement: InfinitiveComplement];
            require head.frame = NominalInfinitive;
            use UnmodifiedNominalProperties;
        }

        construction SymbolComplementNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ",
                complement: CostSymbols];
            require head.frame = NominalSymbols;
            require marker.NominalComplementMarker = Of;
            require licence_noun_complement(marker.PrepositionFunctionLicence) = Yes;
            use UnmodifiedNominalProperties;
        }

        construction PrepositionComplementNominal: Nominal {
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require head.frame = NominalPreposition;
            agree head.NominalComplementMarker = marker.NominalComplementMarker;
            require licence_noun_complement(marker.PrepositionFunctionLicence) = Yes;
            use UnmodifiedNominalProperties;
            export BareGenitiveHost = No;
        }

        construction Adjective: AdjectivePhrase {
            form [head: lexical(Adjective)];
            require head.framing = Unframed;
            export AdjectiveStructure = Simple;
            export AdjectiveOrigin = head.AdjectiveOrigin;
            export TargetingPremodifierUse = head.TargetingPremodifierUse;
        }

        construction IntransitiveAdjective: AdjectivePhrase {
            form [head: lexical(Adjective)];
            require head.frame = Intransitive;
            export AdjectiveStructure = Simple;
            export AdjectiveOrigin = head.AdjectiveOrigin;
            export TargetingPremodifierUse = head.TargetingPremodifierUse;
        }

        construction ComplementedAdjective: AdjectivePhrase {
            form [head: lexical(Adjective), " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require head.frame = AdjectivalPrepositionComplement;
            require marker.InternalisedComplementMarker = Yes;
            export AdjectiveStructure = Complemented;
            export AdjectiveOrigin = head.AdjectiveOrigin;
            export TargetingPremodifierUse = No;
        }

        construction NounPremodifier: NounPremodifier {
            form [head: lexical(Noun)];
            require head.NounPremodifier = Yes;
            require head.form = Singular;
            require head.framing = Unframed;
        }
        instance Coordination<Result, Member, Agreement = NoConcord, Properties = NoConcord, Distribution = GeneralCoordinatorDistribution>: [
            (VerbalPremodifier, Self),
            (NounPremodifier, Self),
            (KeywordQualityPreposition, Self, QualityPrepositionConcord),
            (SelectedComplementTail, Self, SelectedTailConcord, PrimitiveRightTail),
            (NounPhrase, Self, NounCoordinationAgreement),
            (FinitePredicate, Self, FiniteConcord, NoConcord, NoConcord),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties, NoConcord, NoConcord),
            (FiniteObjectGap, Self, FiniteConcord, NoConcord, NoConcord),
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
            (ComparativeGovernorHead, Self),
            (PrepositionPhrase, Self, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPermissions, LeadingConjunct),
            (InfinitiveComplement, Self, NoConcord),
            (FrequencyPhrase, Self),
            (BareObjectGap, Self, PurposeConcord, NoConcord, NoConcord),
            (CoordinatedFiniteClause, FiniteClause, NoConcord, ClauseCoordinationProperties, NoConcord),
            (DepictivePhrase, Self, UnlikeDepictives),
            (PredicativeComplement, Self, UnlikePredicatives),
            (SelectedPrepositionHead, Self, PrepositionPermissions, SharedHeadStatus),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
            use Distribution(right, coordinator);
        }

        construction NounPremodifiedNominal: Nominal {
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: NounPremodifier, " ", head: Nominal];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = head.SourceModifierPresent;
            require head.Targeting = No;
        }

        construction PremodifiedNominal: Nominal {
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: AdjectivePhrase, " ", head: Nominal];
            use NominalHeadFeatures;
            export PropertyValuePosition = premodified_property_value_position(
                modifier.AdjectiveOrigin, head.PropertyValuePosition);
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            require modifier.AdjectiveStructure = Simple;
            require targeting_premodifier(modifier.TargetingPremodifierUse, head.Targeting) = Yes;
        }

        // CGEL Ch. 13 §5.1, p. 1139 describes same with an as + NP
        // comparative Complement. Its discontinuous nominal representation
        // follows the fixed-cost-phrases project ruling of 2026-10-06.
        construction DiscontinuousComparativeNominal: Nominal {
            form [governor: lexical(Adjective), " ", head: Nominal, " ",
                marker: lexical(Preposition), " ", complement: AccusativePhrase];
            require governor.frame = DiscontinuousEquality;
            require marker.ExpandedComparisonMarker = Equality;
            require licence_comparative_complement(marker.PrepositionFunctionLicence) = Yes;
            require head.Targeting = No;
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export BareGenitiveHost = head.BareGenitiveHost;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
        }

        // CGEL Ch. 5 §7.6, p. 386: numerical cardinals can modify under
        // an outer Determiner; without it the cardinal is the Determiner.
        construction CardinalPremodifiedNominal: Nominal {
            export NominalAgreementNumber = head.NominalAgreementNumber;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            export MeasurePosition = head.MeasurePosition;
            export PropertyValuePosition = None;
            export ScalarDenotation = head.ScalarDenotation;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            form [quantity: Cardinal, " ", head: Nominal];
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            require head.DeterminerRequirement = No;
            require head.countability = Count;
            agree quantity.number = head.number;
            export DeterminerRequirement = Yes;
            export ObliqueNumber = head.ObliqueNumber;
            export BareGenitiveHost = head.BareGenitiveHost;
        }

        // Amounts remain singular measurements; the lexical mass noun does
        // not acquire a bare count-plural use. CGEL Ch. 5 §3.4, p. 354
        // distinguishes the singular conceptualization of a measured quantity.
        construction CardinalMeasuredNominal: Nominal {
            export NominalAgreementNumber = Head;
            export ScalarDenotation = Yes;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            form [quantity: Cardinal, " ", head: Nominal];
            require head.MeasurePosition = Before;
            require head.CardinalMeasurementUse = Yes;
            require head.number = Singular;
            require head.countability = Mass;
            require head.DeterminerRequirement = No;
            export number = Singular;
            export countability = Count;
            export DeterminerRequirement = Yes;
            export MeasurePosition = None;
            export PropertyValuePosition = None;
            export CardinalMeasurementUse = No;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = None;
            export SlashPremodifierUse = No;
            export NumberTransparency = No;
            export ObliqueNumber = head.ObliqueNumber;
            export BareGenitiveHost = No;
        }

        construction CardinalMeasuredNounPhrase: NounPhrase {
            export ScalarDenotation = Yes;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
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
        }

        construction PostpositiveNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: AdjectivePhrase];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            require modifier.AdjectiveStructure = Complemented;
        }

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
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: VerbalPremodifier, " ", head: Nominal];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
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
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: Name];
            export number = head.number;
            use ThirdPersonCommonCase;
        }

        construction PassiveNamePredicate: NamePredicate {
            form [head: lexical(Verb), " ", complement: Name];
            require head.form = PastParticiple;
            require head.frame = ObjectName;
        }

        construction NamedNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: NamePredicate];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
        }

        construction TargetedNominal: Nominal {
            export NominalAgreementNumber = head.NominalAgreementNumber;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
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
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            export ObliqueNumber = head.ObliqueNumber;
        }

        construction PostmodifiedNominal: Nominal {
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
        construction PartitiveNounPhrase: NounPhrase {
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            form [quantity: Cardinal, " ", complement: PrepositionPhrase,
                modifiers: repeat(PartitiveModifier, "")];
            require partitive_oblique(complement.ObliqueNumber) = Yes;
            require licence_noun_complement(complement.PrepositionFunctionLicence) = Yes;
            require complement.ComplementFunctionRestriction = None;
            export number = quantity.number;
            export Targeting = No;
            use ThirdPersonCommonCase;
        }

        // CGEL Ch. 5 §§9.1–9.2, pp. 411–413: explicitly partitive determiner-head.
        // Targeted obliques are the each-of ticket's Oracle-English decision.
        construction DeterminerHeadPartitiveNounPhrase: NounPhrase {
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
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            use ThirdPersonCommonCase;
        }

        // CGEL Ch. 5 §9.2(c), pp. 414–415: a singular demonstrative
        // can fuse Determiner and Head without an overt nominal.
        construction FusedDeterminerHeadNounPhrase: NounPhrase {
            form [head: lexical(Determinative)];
            require head.FusedHeadUse = Yes;
            require head.DeterminerUse = Singular;
            export number = Singular;
            export Targeting = No;
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            use ThirdPersonCommonCase;
        }

        construction DeterminedNounPhrase: NounPhrase {
            export ScalarDenotation = quantificational_scalar_denotation(
                determiner.QuantificationalDeterminer, head.NumberTransparency, head.ScalarDenotation);
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            export BareGenitiveHost = head.BareGenitiveHost;
            export Targeting = head.Targeting;
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Ordinary;
            require determiner_license(determiner.DeterminerUse, head.number,
                head.countability) = Yes;
            export number = quantificational_number(determiner.QuantificationalDeterminer,
                head.NumberTransparency, head.ObliqueNumber, head.number, head.NominalAgreementNumber,
                determiner.DeterminerAgreementNumber);
            use ThirdPersonCommonCase;
        }

        construction IndefiniteNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
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
        }

        table bare_noun_phrase_license(number, countability) -> Selection {
            (Plural, Count) => Yes, (Singular, Mass) => Yes,
        }
        construction BareNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            form [head: Nominal];
            require bare_noun_phrase_license(head.number, head.countability) = Yes;
            require head.DeterminerRequirement = No;
            use NounPhraseHeadAgreement;
        }

        // CGEL Ch. 7 §3.1, pp. 620–623: the bare NP is internal to the PP.
        construction BarePrepositionNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            form [head: Nominal];
            require head.BarePrepositionUse = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.DeterminerRequirement = No;
            export BarePrepositionUse = Yes;
            use NounPhraseHeadAgreement;
        }

        construction TargetNounPhrase: NounPhrase {
            export ScalarDenotation = head.ScalarDenotation;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
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
        }

        construction NominativePronoun: NounPhrase {
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
            export BarePrepositionUse = No;
            export SubjectStructure = Pronoun;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: lexical(Pronoun)];
            use PredicateHeadAgreement;
            require head.case = Nominative;
            require head.RelativeUse = No;
            require head.ExistentialUse = No;
            export CaseUse = Nominative;
        }
        instance CasePhrase<Result, Head: head, Properties>: [
            (NominativePhrase, NounPhrase, NominativeCase),
            (AccusativePhrase, NounPhrase, AccusativeCase),
        ] {
            require head.BarePrepositionUse = No;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            use Properties;
        }

        construction AccusativePronoun: NounPhrase {
            export ScalarDenotation = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
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
        }

        construction BareNominalPreposition: PrepositionPhrase {
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            export SourceComplementUse = head.SourceComplementUse;
            export SourceMarker = head.SourceMarker;
            export FocusedUse = No;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: NounPhrase];
            require complement.BarePrepositionUse = Yes;
            require head.BareNominalComplement = Yes;
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
        }

        // CGEL Ch. 7 §5.1, pp. 636–637: a predicative NP Complement
        // differs from an Object; adjunct use excludes adjective Complements.
        // Enter selects this PP by the project ruling of 2026-10-06.
        // Final Adjunct position follows the orchestrator ruling of 2026-10-07.
        // The nominal's declared use licenses that position; selected and
        // preposed uses survive independently (CGEL Ch. 4 §5.3, p. 263).
        construction PredicativeComplementPreposition: PrepositionPhrase {
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

        // CGEL Ch. 6 §7.3.1, p. 587, [42ii,iv]: restrictive focusing
        // Adverbs can modify PPs and AdvPs. Final Adjuncts are this scope.
        schema FocusedPhrase {
            form [modifier: lexical(Adverb), " ", head: node];
            require modifier.FocusingModifierUse = Restrictive;
            require head.FocusedUse = No;
        }
        policy FocusedPrepositionPermissions {
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
            export LocativeUse = No;
            export NominalComplementMarker = None;
            export InternalisedComplementMarker = No;
            export ObliqueNumber = None;
        }
        policy FocusedAdverbPermissions {
            export FocusedUse = Yes;
            require head.VPFinalAdjunct = Yes;
            export VPFinalAdjunct = head.VPFinalAdjunct;
            export ClauseInitialAdjunct = No;
            export DurationUse = head.DurationUse;
            export UnmarkedConjunctLicence = head.UnmarkedConjunctLicence;
        }
        policy FocusedPurposePermissions {
            export FocusedUse = Yes;
        }
        // Project analysis: focusing extends to an attested purpose-Clause
        // constituent; the PP/AdvP citation above does not establish this case.
        instance FocusedPhrase<Result, Head: head, Properties>: [
            (PurposeInfinitival, Self, FocusedPurposePermissions),
            (PrepositionPhrase, Self, FocusedPrepositionPermissions),
            (AdverbPhrase, Self, FocusedAdverbPermissions),
        ] {
            use Properties;
        }

        construction IntransitivePreposition: PrepositionPhrase {
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
        }

        construction VerblessClauseComplementPreposition: PrepositionPhrase {
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
            form [head: lexical(Preposition), " ", complement: VerblessClause];
            require head.VerblessClauseComplement = Yes;
            require licence_adjunct(head.PrepositionFunctionLicence) = Yes;
            export SourceComplementUse = No;
            export SourceMarker = No;
            export FocusedUse = No;
            export ClauseInitialAdjunct = No;
            export SelectedPrepositionUse = No;
            export PredicativeAdjunctUse = No;
            export SourceModifierPresent = No;
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
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: FiniteClause];
            require finite_clause_complement(head.FiniteClauseComplement) = Yes;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // CGEL Ch. 13 §5.5, p. 1152: irrealis content-clause Complement.
        construction IrrealisClauseComplementPreposition: PrepositionPhrase {
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
            export MatrixComplementRequirement = None;
            export NominalExceptionComplementUse = No;
            export ComplementFunctionRestriction = None;
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
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
            export InternalisedComplementMarker = No;
            export LocativeUse = No;
            export PrepositionFunctionLicence = Adjunct;
        }

        // CGEL Ch. 14 §8.3, p. 1262: gerund-participials freely complement
        // prepositions. Complement-specific function restriction is project policy,
        // declared by the lexicon, independently of Complement availability.
        construction GerundComplementPreposition: PrepositionPhrase {
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
            export SourceMarker = No;
        }
        construction MatrixComplementPreposition: PrepositionPhrase {
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
            export SourceMarker = No;
        }

        // CGEL Ch. 7 §2.4, p. 617 permits PP Complements; Ch. 8 §4.3,
        // p. 687 describes locative PP Complements to a source preposition.
        // The supported source-head eligibility is a declared project decision.
        construction PrepositionComplementPreposition: PrepositionPhrase {
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
        }

        // CGEL pp. 262, 1265: optional depictives are adjuncts.
        construction AdjectivalDepictive: DepictivePhrase {
            form [phrase: AdjectivePhrase];
            export DepictiveKind = Adjectival;
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
            use Properties;
        }
        instance ParticipialAuxiliaryPredicate<Result, Complement: complement, Properties>: [
            (FinitePredicate, ParticipialComplement, FiniteAuxiliaryAgreement),
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
        ] { use Properties; }

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
        schema SelectedGapPrepositionPredicate {
            form [head: lexical(Verb), " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require selected_object_marker(head.frame, marker.KeywordMarker) = Yes;
            require licence_verb_complement(marker.PrepositionFunctionLicence) = Yes;
        }
        policy PlainGapHead {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            require head.form = Plain;
        }
        policy BarePassiveHead {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            require head.form = PastParticiple;
            export form = PastParticiple;
            export ParticipialUse = BarePassive;
            export InternalisedComplementPresent = No;
        }
        instance SelectedGapPrepositionPredicate<Result, Properties>: [
            (FiniteObjectGap, FiniteHeadAgreement),
            (BareObjectGap, PlainGapHead),
            (SecondaryVerbPhrase, BarePassiveHead),
        ] {
            use Properties;
        }
        schema SelectedGapLocativePredicate {
            form [head: lexical(Verb), " ", complement: LocativeComplement];
            require head.frame = ObjectLocative;
        }
        instance SelectedGapLocativePredicate<Result, Properties>: [
            (FiniteObjectGap, FiniteHeadAgreement),
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
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
        }

        // CGEL pp. 698 and 671: licensed temporal and manner NPs as adjuncts.
        table nominal_adjunct_license(NominalAdjunctDeterminer, NominalAdjunctClass)
            -> Selection {
            (Demonstrative, Temporal) => Yes,
            (Demonstrative, Manner) => Yes,
            (Temporal, Temporal) => Yes,
        }
        table nominal_adjunct_concord(NominalAdjunctClass, NominalAdjunctClass)
            -> NominalAdjunctClass {
            (None, None) => None, (None, Temporal) => None, (None, Manner) => None,
            (Temporal, None) => None, (Temporal, Temporal) => Temporal,
            (Temporal, Manner) => None, (Manner, None) => None,
            (Manner, Temporal) => None, (Manner, Manner) => Manner,
        }
        construction NominalAdjunctPhrase: NominalAdjunctPhrase {
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require head.Targeting = No;
            require head.number = Singular;
            require nominal_adjunct_license(determiner.NominalAdjunctDeterminer,
                head.NominalAdjunctClass) = Yes;
        }
        instance NominalAdjunctPredicate<Result, Head: head, Properties>: [
            (FinitePredicate, Self, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, SecondaryProjection),
            (FiniteObjectGap, Self, PredicateHostAgreement),
            (BareObjectGap, Self, PurposeHostProjection),
        ] {
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
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            form [modifier: lexical(Adjective), " ", head: Nominal];
            require modifier.BareSingularUse = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.Targeting = No;
            use NounPhraseHeadAgreement;
        }

        construction BareParticipialStatusNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            form [modifier: lexical(Verb), " ", head: Nominal];
            require verbal_premodifier(modifier.form, modifier.frame,
                modifier.PastParticipialPremodifier) = Yes;
            require modifier.BareSingularUse = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.Targeting = No;
            use NounPhraseHeadAgreement;
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
            use Properties;
        }

        instance PrepositionPredicate<Result, Head: head, Modifier: modifier, Properties>: [
            (FinitePredicate, Self, PrepositionPhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, PrepositionPhrase, SecondaryPrepositionProjection),
            (FiniteObjectGap, Self, PrepositionPhrase, PredicateHostAgreement),
            (BareObjectGap, Self, PrepositionPhrase, PurposeHostProjection),
        ] {
            use Properties;
        }

        instance PurposePredicate<Result, Head: head, Properties>: [
            (FinitePredicate, Self, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, SecondaryAdjunctProjection),
            (FiniteObjectGap, Self, PredicateHostAgreement),
            (BareObjectGap, Self, PurposeHostProjection),
        ] {
            use Properties;
        }

        construction BarePredicate: BarePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = Plain;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
        }

        construction ProgressiveComplement: ParticipialPredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = GerundParticiple;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
        }

        // CGEL p. 1327: unlike AdjP and progressive complements coordinated under be.
        // The shared phrase describes their constituent structure; here it is a complement.
        construction MixedCopularProgressiveComplement: ParticipialPredicate {
            form [phrase: DepictivePhrase];
            require phrase.DepictiveKind = Mixed;
            export PurposeAdjunctLicence = No;
            export MatrixComplementLicence = None;
        }

        construction PassiveComplement: ParticipialPredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            require head.ParticipialUse = BarePassive;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
        }

        construction PerfectComplement: PastParticiplePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            require head.ParticipialUse = Ordinary;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
        }

        construction PassivePredicate: SecondaryVerbPhrase {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            form [head: lexical(Verb)];
            require head.form = PastParticiple;
            require head.frame = Transitive;
            export form = PastParticiple;
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
            export BareGenitiveHost = No;
            form [head: Nominal, " ", relative: SubjectRelativeClause];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            require nominal_relative_agreement(head.NominalAgreementNumber, head.number, relative.number) = Yes;
        }

        construction FiniteObjectGap: FiniteObjectGap {
            require head.PredicateFrameUse = Yes;
            form [head: lexical(Verb)];
            use FiniteHeadAgreement;
            require head.frame = Transitive;
        }

        construction BareObjectGap: BareObjectGap {
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            form [head: lexical(Verb)];
            require head.form = Plain;
            require head.frame = Transitive;
        }

        construction AuxiliaryObjectGap: FiniteObjectGap {
            form [head: lexical(Verb), " ", complement: BareObjectGap];
            use FiniteHeadAgreement;
            require head.frame = BareAuxiliary;
        }

        construction ObjectRelativeClause: ObjectRelativeClause {
            form [marker: lexical(Subordinator), " ", subject: NominativePhrase,
                clitic_separator(predicate.CliticHost), predicate: FiniteObjectGap];
            require marker.RelativeSubordinator = Yes;
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        construction ZeroObjectRelativeClause: ObjectRelativeClause {
            form [subject: NominativePhrase, clitic_separator(predicate.CliticHost),
                predicate: FiniteObjectGap];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        construction ObjectRelativeNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", relative: ObjectRelativeClause];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
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
        }

        construction BareEllipsis: BareComplement {
            form [omission: Ellipsis];
            require omission.form = Plain;
            export AuxiliaryComplementRealization = Elided;
            export PurposeAdjunctLicence = No;
            export MatrixComplementLicence = None;
        }




        construction Cardinal: Cardinal {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
            export number = head.number;
        }

        construction LargeCount: Cardinal {
            form [head: lexical(Numeral)];
            require head.numeral_kind = GroupedArabic;
            require head.numeral_size = Large;
            export number = head.number;
        }

        construction NumericQuantityConjunct: QuantityConjunct {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Arabic;
            require head.numeral_size = Small;
            export number = head.number;
        }

        construction CardinalQuantityConjunct: QuantityConjunct {
            cost 0;
            form [value: Cardinal];
            export number = value.number;
        }

        policy VariableCountProperties {
            export number = Plural;
        }
        policy ScalarVariableProperties {
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
            export ScalarDenotation = No;
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

        schema QuantitativePrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: node];
            export number = complement.number;
        }

        policy CardinalPrepositionComplement {
            require head.QuantitativeComplement = Cardinal;
        }

        policy QuantitativePrepositionComplement {
            require head.QuantitativeComplement = CardinalPrepositionPhrase;
        }

        policy ComparativePrepositionComplement {
            require head.ComparisonMarker = Ordering;
        }

        instance QuantitativePrepositionPhrase<Result, Complement: complement, Properties>: [
            (CardinalPrepositionPhrase, Cardinal, CardinalPrepositionComplement),
            (QuantitativePrepositionPhrase, CardinalPrepositionPhrase,
                QuantitativePrepositionComplement),
            (ComparativePrepositionPhrase, Cardinal, ComparativePrepositionComplement),
        ] {
            use Properties;
        }

        // CGEL Ch. 5 §7.6, p. 386: a numeral and comparative are Conjuncts.
        // §3.4, p. 353 n. 13: this coordination selects a plural head.
        construction NumeralComparativeCoordination: ComparativeQuantity {
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
            form [head: lexical(Determinative), " ", complement: ComparativePrepositionPhrase];
            require head.ComparativeQuantityUse = Yes;
            export ComparativeQuantityStructure = Headed;
            export number = complement.number;
        }

        construction ComparativeQuantityDeterminer: QuantitativeDeterminer {
            cost 0;
            form [value: ComparativeQuantity];
            export DeterminerUse = cardinal_determiner_use(value.number);
        }

        construction CardinalDeterminer: QuantitativeDeterminer {
            cost 0;
            form [value: Cardinal];
            export DeterminerUse = cardinal_determiner_use(value.number);
        }

        construction PrepositionDeterminer: QuantitativeDeterminer {
            cost 0;
            form [value: QuantitativePrepositionPhrase];
            export DeterminerUse = cardinal_determiner_use(value.number);
        }

        table cardinal_determiner_use(number) -> DeterminerUse {
            (Singular) => SingularCount, (Plural) => PluralCount,
        }

        // CGEL Ch. 5 §11(c), p. 431: degree modification within a DP.
        construction DegreeDeterminativePhrase: QuantitativeDeterminer {
            form [modifier: lexical(Determinative), " ", head: lexical(Determinative)];
            require modifier.DegreeUse = Modifier;
            require head.DegreeUse = Head;
            export DeterminerUse = head.DeterminerUse;
        }

        construction QuantifiedNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            form [quantity: QuantitativeDeterminer, " ", head: Nominal];
            require determiner_license(quantity.DeterminerUse, head.number, head.countability) = Yes;
            require head.DeterminerRequirement = No;
            use NounPhraseHeadAgreement;
        }

        construction OrdinalPremodifier: AdjectivePhrase {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Ordinal;
            export AdjectiveStructure = Simple;
            export AdjectiveOrigin = Ordinary;
            export TargetingPremodifierUse = Yes;
        }

        construction GroupedScalarNumeral: MeasurePhrase {
            form [head: lexical(Numeral)];
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
            export ScalarSign = Present;
            onset Consonant;
            form ["+", value: UnsignedScalar];
        }

        construction NegativeScalar: ScalarComponent {
            export ScalarSign = Present;
            onset Consonant;
            form ["-", value: UnsignedScalar];
            form ["−", value: UnsignedScalar];
        }

        construction SlashPair: SlashPair {
            form [left: ScalarComponent, "/", right: ScalarComponent];
        }

        schema SlashMeasure {
            form [pair: SlashPair];
            export MeasureKind = Pair;
        }
        instance SlashMeasure<Result>: [(MeasurePhrase), (PropertyValue)] {}

        table slash_premodifier_concord(SlashPremodifierUse, SlashPremodifierUse)
            -> SlashPremodifierUse {
            (Yes, Yes) => Yes, (Yes, No) => No, (No, Yes) => No, (No, No) => No,
        }

        construction SlashModifiedNominal: Nominal {
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: SlashPair, " ", head: Nominal];
            require head.SlashPremodifierUse = Yes;
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            require head.Targeting = No;
            require head.countability = Count;
        }

        construction UngroupedScalarNumeral: MeasurePhrase {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Arabic;
            require head.numeral_size = Small;
            export MeasureKind = Scalar;
        }

        construction ArithmeticMeasure: MeasurePhrase {
            form [left: MeasurePhrase, " ", operator: lexical(Preposition), " ",
                right: MeasurePhrase];
            require operator.MeasureOperator = Yes;
            require left.MeasureKind = Scalar;
            require right.MeasureKind = Scalar;
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
            export ScalarDenotation = Yes;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [quantity: MeasurePhrase, " ", head: lexical(Noun)];
            require head.MeasurePosition = Before;
            require quantity.MeasureKind = Scalar;
            require head.number = Singular;
            require head.countability = Mass;
            export number = Singular;
            use ThirdPersonCommonCase;
        }

        policy ScalarValueMeasure {
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

        construction ScalarCardinalValue: ScalarValue {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }

        construction ScalarPropertyCoordination: ScalarValue {
            cost 0;
            form [value: ComparativeQuantity];
            require value.ComparativeQuantityStructure = Coordinated;
        }

        // CGEL Ch. 5 §11(d), p. 432 n. 48 classifies greater as an
        // Adjective. Its permission in this value position is a project ruling.
        construction AdjectivalScalarCoordination: ScalarValue {
            form [left: QuantityConjunct, " ", marker: lexical(Coordinator), " ",
                right: lexical(Adjective)];
            require marker.CoordinationKind = Alternative;
            require marker.NoncorrelativeCoordination = Yes;
            require right.ScalarComparativeUse = Yes;
        }

        // Project rulings: scalar-property-values (2026-10-07) and
        // coordinated-property-values (2026-10-08). A property Nominal selects
        // its following value Complement through declared position and kind.
        construction MeasuredAttribute: NounPhrase {
            export ScalarDenotation = head.ScalarDenotation;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
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
        }
        policy NonfiniteComparisonHost {
            require matched_nonfinite_comparison(head.form, comparison.ComparativeComplementForm) = Yes;
            export PurposeAdjunctLicence = head.PurposeAdjunctLicence;
            export MatrixComplementLicence = head.MatrixComplementLicence;
            export InternalisedComplementPresent = head.InternalisedComplementPresent;
            export ParticipialUse = head.ParticipialUse;
            export form = head.form;
        }
        instance MatchedComparativeConstituent<Result, Head: head, Properties>: [
            (NounPhrase, Self, NominalComparisonHost),
            (SecondaryVerbPhrase, Self, NonfiniteComparisonHost),
        ] { use Properties; }

        construction ComparativeGovernorHead: ComparativeGovernorHead {
            form [head: lexical(Adjective), " ", marker: lexical(Preposition)];
            require comparative_marker(head.frame, marker.ComparisonMarker) = Yes;
            require licence_comparative_complement(marker.PrepositionFunctionLicence) = Yes;
        }

        construction ScalarComparativeComplement: ComparativeComplement {
            form [value: ScalarMeasurePhrase];
        }

        construction NominalComparativeComplement: ComparativeComplement {
            form [value: AccusativePhrase];
        }

        construction ComparativeAdjectivePhrase: ComparativeAdjectivePhrase {
            form [governor: ComparativeGovernorHead, " ", complement: ComparativeComplement];
        }

        construction ComparativeAdjective: AdjectivePhrase {
            form [phrase: ComparativeAdjectivePhrase];
            export AdjectiveStructure = Complemented;
            export AdjectiveOrigin = Ordinary;
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
            use Properties;
        }
        instance ComplementedDepictivePredicate<Result, Head: head, Properties>: [
            (FinitePredicate, FiniteDepictiveHost, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondaryDepictiveHost, SecondaryAdjunctProjection),
        ] {
            use Properties;
        }
        instance AdverbPredicate<Result, Head: head, Modifier: modifier, Properties>: [
            (FinitePredicate, Self, AdverbPhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, Self, AdverbPhrase, SecondaryAdjunctProjection),
        ] {
            use Properties;
        }

        construction InitialAdverb: Clause {
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
            form [marker: lexical(Subordinator), " ", predicate: node];
            require marker.InfinitivalMarker = Yes;
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
            use Properties;
        }
        instance SharedPredicativeComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, PredicativeComplement, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, PredicativeComplement,
                OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedLocativeComplement<Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, LocativeComplement, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, LocativeComplement,
                OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedManaComplement<Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, ManaPhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, ManaPhrase, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAmountComplement<Result, Head: head, Amount: amount, Properties>: [
            (FinitePredicate, FiniteSelectedHead, Amount, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, Amount, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedMeasureComplement<Result, Head: head, Measure: measure, Properties>: [
            (FinitePredicate, FiniteSelectedHead, MeasurePhrase, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, MeasurePhrase, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedSlashMeasureComplement<Result, Head: head, Measure: measure, Properties>: [
            (FinitePredicate, FiniteSelectedHead, SlashPair, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, SlashPair, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedGrantedAbilityComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, GrantedAbility, PredicateHostAgreement),
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
        instance SharedObjectNameComplement<
            Result, Head: head, Object: object, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, Name, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase, Name,
                OrdinarySelectedPredicate),
        ] {
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
            Properties = NoConcord>: [
            (Clause, Self, UnmarkedConjunctHead),
            (FinitePredicate, Self, FiniteConcord),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
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
            (CorrelativeAdjectivePhrase, AdjectivePhrase),
            (FiniteObjectGap, Self, FiniteConcord),
            (BareObjectGap, Self, PurposeConcord),
            (CoordinatedFiniteClause, FiniteClause, NoConcord, ClauseCoordinationProperties),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
        }

        instance CorrelativeSeriesEnd<Result, Member, Agreement = NoConcord, Properties = NoConcord,
            Status = NoConcord>: [
            (CorrelativeClauseSeries, Clause, NoConcord, LeadingConjunct),
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
            (CorrelativeAdjectiveSeries, AdjectivePhrase),
            (CorrelativeFiniteObjectGapSeries, FiniteObjectGap, FiniteConcord),
            (CorrelativeBareObjectGapSeries, BareObjectGap, PurposeConcord),
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
            (CorrelativeClauseSeries, Clause, Self, UnmarkedConjunct),
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
            (CorrelativeAdjectiveSeries, AdjectivePhrase, Self),
            (CorrelativeFiniteObjectGapSeries, FiniteObjectGap, Self, FiniteConcord),
            (CorrelativeBareObjectGapSeries, BareObjectGap, Self, PurposeConcord),
            (CorrelativeFiniteClauseSeries, FiniteClause, Self, NoConcord, ClauseCoordinationProperties),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
            use Status(rest, rest);
        }
        instance EitherSerialCoordination<Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord>: [
            (Clause, Self, CorrelativeClauseSeries, MarkedConjunctSeries),
            (FinitePredicate, Self, CorrelativeFinitePredicateSeries, FiniteConcord),
            (SecondaryVerbPhrase, Self, CorrelativeSecondaryVerbPhraseSeries,
                SecondaryConjunctProperties),
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
            (CorrelativeAdjectivePhrase, AdjectivePhrase, CorrelativeAdjectiveSeries),
            (FiniteObjectGap, Self, CorrelativeFiniteObjectGapSeries, FiniteConcord),
            (BareObjectGap, Self, CorrelativeBareObjectGapSeries, PurposeConcord),
            (CoordinatedFiniteClause, FiniteClause, CorrelativeFiniteClauseSeries, NoConcord, ClauseCoordinationProperties),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
        }
        instance BothCoordination<Result, Member, Agreement = NoConcord, Properties = NoConcord>: [
            (FinitePredicate, Self, FiniteConcord),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
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
            (CorrelativeAdjectivePhrase, AdjectivePhrase),
            (FiniteObjectGap, Self, FiniteConcord),
            (BareObjectGap, Self, PurposeConcord),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
        }

        instance NeitherCoordination<Result, Member, Agreement = NoConcord,
            Properties = NoConcord>: [
            (FinitePredicate, Self, FiniteConcord),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
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
            (CorrelativeAdjectivePhrase, AdjectivePhrase),
            (FiniteObjectGap, Self, FiniteConcord),
            (BareObjectGap, Self, PurposeConcord),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
        }

        instance NeitherSerialCoordination<Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord>: [
            (FinitePredicate, Self, CorrelativeFinitePredicateSeries, FiniteConcord),
            (SecondaryVerbPhrase, Self, CorrelativeSecondaryVerbPhraseSeries,
                SecondaryConjunctProperties),
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
            (CorrelativeAdjectivePhrase, AdjectivePhrase, CorrelativeAdjectiveSeries),
            (FiniteObjectGap, Self, CorrelativeFiniteObjectGapSeries, FiniteConcord),
            (BareObjectGap, Self, CorrelativeBareObjectGapSeries, PurposeConcord),
        ] {
            use Agreement(rest, rest);
            use Properties(rest, rest);
        }

        // CGEL pp. 467–468: a genitive NP determines the following nominal.
        table genitive_marker(number, BareGenitiveHost, HostEnding) -> Selection {
            (Singular, No, Default) => Yes, (Singular, Yes, Default) => Yes,
            (Plural, Yes, PluralS) => Yes,
        }
        construction GenitiveNounPhrase: NounPhrase {
            require possessor.BarePrepositionUse = No;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            form [possessor: NounPhrase, marker: lexical(Clitic), " ", head: Nominal];
            require possessor.CaseUse = Common;
            require marker.Function = Genitive;
            require genitive_marker(possessor.number, possessor.BareGenitiveHost,
                marker.HostEnding) = Yes;
            use NounPhraseHeadAgreement;
        }

        construction PossessiveNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
            export BarePrepositionUse = No;
            form [possessor: lexical(Pronoun), " ", head: Nominal];
            require possessor.case = Genitive;
            require possessor.NominalLicense = AnyNominal;
            use NounPhraseHeadAgreement;
        }

        construction KeywordComplementPreposition: PrepositionPhrase {
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
        instance SharedCardinalComplement<Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, Cardinal, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, Cardinal, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }

        instance SharedInfinitiveComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, InfinitiveComplement, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, InfinitiveComplement,
                OrdinarySelectedPredicate),
        ] {
            use Properties;
        }

        construction AdjectivalCorrelativeComplement: PredicativeComplement {
            export PredicativeAdjunctUse = No;
            form [phrase: CorrelativeAdjectivePhrase];
            export PredicativeKind = Adjectival;
        }

        construction CorrelativePostpositiveNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: CorrelativeAdjectivePhrase];
            use NominalHeadFeatures;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export PredicativeAdjunctUse = head.PredicativeAdjunctUse;
            export SourceModifierPresent = head.SourceModifierPresent;
        }

        construction SelectedPrepositionHead: SelectedPrepositionHead {
            form [head: lexical(Preposition)];
            require noun_phrase_complement(head.PrepositionComplement) = Yes;
            export HeadCoordination = No;
            use PrepositionHeadPermissions;
        }

        construction SharedPrepositionComplement: PrepositionPhrase {
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
        }

    }
}
