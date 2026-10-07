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
        feature DeterminerRequirement { No, Yes } default No;
        feature NumberTransparency { No, Yes } default No;
        feature QuantificationalDeterminer { No, Yes } default No;
        feature ObliqueMarker { No, Yes } default No;
        feature ObliqueNumber { None, Singular, Plural } default None;
        feature CaseUse { Common, Nominative, Accusative }
        feature Targeting { No, Yes }
        feature CoordinationKind { Additive, Alternative, Adversative }
        feature PrepositionComplement { NounPhrase, PredicativeNounPhrase, None }
        feature AdjectiveComplementClass { Manner }
        feature AdverbComplementClass { Duration }
        feature DurationUse { No, Yes } default No;
        feature QuantitativeComplement { Cardinal, CardinalPrepositionPhrase }
        feature ComparativeQuantityUse { No, Yes } default No;
        feature PrepositionFunctionLicence { Adjunct, Modifier, NounComplement, VerbComplement,
            PredicativeComplement, Compound, ComparativeComplement, PreposedAdjunct } set;
        feature CompoundComplementMarker { Of }
        feature PrepositionPhraseComplement { Yes }
        table selected_compound_complement(CompoundComplementMarker, NominalComplementMarker)
            -> Selection { (Of, Of) => Yes, }
        table licence_adjunct(PrepositionFunctionLicence) -> Selection contains Adjunct => Yes;
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
        feature FiniteClauseComplement { Yes }
        feature GerundClauseComplement { Yes }
        feature InternalisedComplementMarker { No, Yes } default No;
        feature InternalisedComplementPresent { No, Yes } default No;
        feature ExtentMarker { Yes }
        feature LocativeUse { No, Yes }
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
        feature MeasureOperator { Yes }
        feature MeasurePosition { None, Before, After } default None;
        feature CardinalMeasurementUse { No, Yes } default No;
        feature MeasureKind { Scalar, Pair }
        feature ComparisonMarker { Equality, Ordering }
        feature ExpandedComparisonMarker { Equality }
        feature KeywordParameterClass { Nullary, Amount, Cost, Quality, Subject, AmountCost,
            QualityCost, Ability, Condition, CostPowerToughness }
        feature NominalBareClass { None, Interval, Boundary }
        feature NominalAdjunctClass { None, Temporal, Manner } default None;
        feature SlashPremodifierUse { No, Yes } default Yes;
        feature NominalAdjunctDeterminer { None, Demonstrative, Temporal }
        feature BareSingularUse { No, Yes } default No;
        feature RelativeUse { Yes }
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
        category FinitePredicate(number, person, CliticHost);
        category SecondaryVerbPhrase(form, ParticipialUse, InternalisedComplementPresent);
        category FiniteDepictiveHost(number, person, CliticHost);
        category SecondaryDepictiveHost(form, ParticipialUse, InternalisedComplementPresent);
        category BarePredicate();
        category ParticipialPredicate();
        category PastParticiplePredicate();
        category PredicativeComplement(PredicativeKind);
        category DepictivePhrase(DepictiveKind);
        category DepictivePhraseSeries(DepictiveKind);
        category Ellipsis(form);
        category BareComplement(AuxiliaryComplementRealization);
        category ParticipialComplement();
        category PerfectComplement();
        category Nominal(number, countability, Targeting, NominalAdjunctClass, SlashPremodifierUse,
            NumberTransparency, ObliqueNumber, BareGenitiveHost, DeterminerRequirement, MeasurePosition, CardinalMeasurementUse,
            BarePrepositionUse, NominalComplementMarker, SelectedPrepositionUse);
        category VerbalPremodifier();
        category VerbalPremodifierSeries();
        category NounPremodifier();
        category NounPremodifierSeries();
        category PartitiveModifier();
        category NounPhrase(number, person, CaseUse, Targeting, BareGenitiveHost, SubjectStructure,
            BarePrepositionUse, SelectedPrepositionUse);
        category NominativePhrase(number, person, CaseUse, SubjectStructure, SelectedPrepositionUse);
        category AccusativePhrase(number, person, CaseUse, SelectedPrepositionUse);
        category AdjectivePhrase(AdjectiveStructure);
        category Name();
        category NamePredicate();
        category PrepositionPhrase(LocativeUse, PrepositionFunctionLicence, ObliqueNumber,
            InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse,
            ClauseInitialAdjunct);
        category LocativeComplement();
        category MannerComplement();
        category FrequencyPhrase();
        category NominalAdjunctPhrase();
        category BareTemporalNominal(NominalBareClass);
        category BoundaryComplement();
        category SubjectRelativeClause(number);
        category ObjectRelativeClause();
        category FiniteObjectGap(number, person, CliticHost);
        category BareObjectGap();
        category Cardinal(number);
        category QuantitativeDeterminer(number);
        category CardinalPrepositionPhrase(number);
        category QuantitativePrepositionPhrase(number);
        category ComparativePrepositionPhrase(number);
        category QuantityConjunct(number);
        category ComparativeQuantity(number);
        category Amount();
        category MeasurePhrase(MeasureKind);
        category ScalarMeasurePhrase();
        category ScalarExtentComplement();
        category ScalarExtentMeasure();
        category UnsignedScalar();
        category ScalarComponent();
        category SlashPair();
        category ComparativeGovernorHead();
        category ComparativeComplement();
        category ComparativeAdjectivePhrase();
        category KeywordPhrase();
        category KeywordQuality(KeywordQualityNumber);
        category KeywordQualityPreposition(KeywordMarker, KeywordQualityNumber);
        category KeywordQualityPrepositionSeries(KeywordMarker, KeywordQualityNumber);
        category SelectedComplementTail(HeadCoordination);
        category SelectedComplementTailSeries(HeadCoordination);
        category KeywordSeparator(KeywordSeparator);
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
        category FinitePredicateSeries(number, person, CliticHost);
        category SecondaryPredicateSeries(form, ParticipialUse, InternalisedComplementPresent);
        category NounPhraseSeries(number, person, CaseUse, Targeting, CoordinationKind, BareGenitiveHost, SubjectStructure, BarePrepositionUse, SelectedPrepositionUse);
        category AdverbPhrase(VPFinalAdjunct, ClauseInitialAdjunct, DurationUse, UnmarkedConjunctLicence);
        category InfinitiveComplement();
        category FiniteSelectedHead(number, person, FrameUse, HeadCoordination, CliticHost, AuxiliaryEllipsisLicence);
        category SecondarySelectedHead(form, FrameUse, HeadCoordination, AuxiliaryEllipsisLicence);
        category FiniteSelectedHeadSeries(number, person, FrameUse, CliticHost, AuxiliaryEllipsisLicence);
        category SecondarySelectedHeadSeries(form, FrameUse, AuxiliaryEllipsisLicence);
        category ManaPhraseSeries();
        category CardinalSeries(number, CoordinationKind);
        category AmountSeries();
        category MeasurePhraseSeries(MeasureKind);
        category KeywordPhraseSeries();
        category QuotedTextSeries();
        category NominalSeries(
            number, countability, Targeting, NominalAdjunctClass, SlashPremodifierUse,
            NumberTransparency, ObliqueNumber, CoordinationKind, BareGenitiveHost, DeterminerRequirement, MeasurePosition, CardinalMeasurementUse,
            BarePrepositionUse, NominalComplementMarker, SelectedPrepositionUse
        );
        category AdjectivePhraseSeries(AdjectiveStructure);
        category PrepositionPhraseSeries(LocativeUse, PrepositionFunctionLicence, ObliqueNumber, CoordinationKind,
            InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, ClauseInitialAdjunct);
        category AdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct, DurationUse, UnmarkedConjunctLicence);
        category InfinitiveComplementSeries();
        category FrequencyPhraseSeries();
        category FiniteObjectGapSeries(number, person, CliticHost);
        category BareObjectGapSeries();
        category CoordinatedFiniteClause();
        category FiniteClauseSeries();
        category PredicativeComplementSeries(PredicativeKind);
        category CorrelativeClauseSeries(CorrelativeCoordinator, UnmarkedConjunctLicence);
        category CorrelativeFinitePredicateSeries(number, person, CorrelativeCoordinator, CliticHost);
        category CorrelativeSecondaryVerbPhraseSeries(form, ParticipialUse, CorrelativeCoordinator,
            InternalisedComplementPresent);
        category CorrelativeNounPhraseSeries(number, person, CaseUse, Targeting,
            CorrelativeCoordinator, CoordinationKind, BareGenitiveHost, SubjectStructure, BarePrepositionUse, SelectedPrepositionUse);
        category CorrelativePrepositionPhraseSeries(LocativeUse, PrepositionFunctionLicence, ObliqueNumber, CoordinationKind,
            CorrelativeCoordinator, InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse, ClauseInitialAdjunct);
        category CorrelativeAdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct,
            DurationUse, CorrelativeCoordinator);
        category CorrelativeManaPhraseSeries(CorrelativeCoordinator);
        category CorrelativeCardinalSeries(number, CorrelativeCoordinator, CoordinationKind);
        category CorrelativeAmountSeries(CorrelativeCoordinator);
        category CorrelativeMeasurePhraseSeries(MeasureKind, CorrelativeCoordinator);
        category CorrelativeKeywordPhraseSeries(CorrelativeCoordinator);
        category CorrelativeQuotedTextSeries(CorrelativeCoordinator);
        category CorrelativeInfinitiveComplementSeries(CorrelativeCoordinator);
        category CorrelativeFiniteSelectedHeadSeries(number, person, FrameUse,
            CorrelativeCoordinator, CliticHost, AuxiliaryEllipsisLicence);
        category CorrelativeSecondarySelectedHeadSeries(form, FrameUse, CorrelativeCoordinator, AuxiliaryEllipsisLicence);
        category CorrelativeAdjectivePhrase();
        category CorrelativeAdjectiveSeries(CorrelativeCoordinator);
        category CorrelativeFiniteObjectGapSeries(number, person, CorrelativeCoordinator, CliticHost);
        category CorrelativeBareObjectGapSeries(CorrelativeCoordinator);
        category SelectedPrepositionHead(LocativeUse, PrepositionFunctionLicence, HeadCoordination,
            InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse);
        category SelectedPrepositionHeadSeries(LocativeUse, PrepositionFunctionLicence, InternalisedComplementMarker, NominalComplementMarker, SelectedPrepositionUse);
        category CorrelativeFiniteClauseSeries(CorrelativeCoordinator);

        frame_category NounPhrase = AccusativePhrase;
        frame_category PowerToughnessAdjustment = SlashPair;
        frame_category MeasurePhrase = ScalarMeasurePhrase;

        schema SelectedPredicate {
            form [head: lexical(Verb), complements: selected_frame(head, Predicate)];
            require head.PredicateFrameUse = Yes;
        }
        instance SelectedPredicate<Result, Properties>: [
            (FinitePredicate, FiniteHeadAgreement),
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
        frame Locative = Predicate(Complement(LocativeComplement));
        frame ObjectLocative = Predicate(Object(NounPhrase), Complement(LocativeComplement));
        frame BareAuxiliary = Auxiliary(Complement(BarePredicate));
        frame ParticipialAuxiliary = Auxiliary(Complement(ParticipialPredicate));
        frame PerfectAuxiliary = Auxiliary(Complement(PastParticiplePredicate));
        frame Measure = Predicate(Complement(MeasurePhrase));
        frame SlashMeasure = Predicate(Complement(PowerToughnessAdjustment));
        frame Equality = Predicate(Marked(Preposition, To, Complement(ComparativeComplement)));
        frame Ordering = Predicate(Marked(Preposition, Than, Complement(ComparativeComplement)));
        frame GrantedAbilityComplement = Predicate(Complement(GrantedAbility));
        frame AmountComplement = Predicate(Complement(Amount));
        frame CardinalComplement = Predicate(Complement(Cardinal));
        frame InfinitiveSelection = Predicate(Complement(InfinitiveComplement));

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
            ObliqueNumber, number) -> number {
            (No, No, None, Singular) => Singular,
            (No, No, None, Plural) => Plural,
            (No, No, Singular, Singular) => Singular,
            (No, No, Singular, Plural) => Plural,
            (No, No, Plural, Singular) => Singular,
            (No, No, Plural, Plural) => Plural,
            (No, Yes, None, Singular) => Singular,
            (No, Yes, None, Plural) => Plural,
            (No, Yes, Singular, Singular) => Singular,
            (No, Yes, Singular, Plural) => Plural,
            (No, Yes, Plural, Singular) => Singular,
            (No, Yes, Plural, Plural) => Plural,
            (Yes, No, None, Singular) => Singular,
            (Yes, No, None, Plural) => Plural,
            (Yes, No, Singular, Singular) => Singular,
            (Yes, No, Singular, Plural) => Plural,
            (Yes, No, Plural, Singular) => Singular,
            (Yes, No, Plural, Plural) => Plural,
            (Yes, Yes, None, Singular) => Singular,
            (Yes, Yes, None, Plural) => Plural,
            (Yes, Yes, Plural, Singular) => Plural,
            (Yes, Yes, Plural, Plural) => Plural,
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
        table joined_clitic(CliticHost) -> Selection {
            (Subject) => Yes, (PronounSubject) => Yes,
        }
        table clitic_complement(Clitic, AuxiliaryComplementRealization) -> Selection {
            (No, Overt) => Yes, (No, Elided) => Yes, (Yes, Overt) => Yes,
        }

        policy FiniteHeadAgreement {
            export CliticHost = clitic_host(head.Clitic, head.form, head.number, head.person);
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        policy SecondaryHeadForm {
            export form = secondary_form(head.form);
        }

        policy NominalHeadProperties {
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            export ObliqueNumber = head.ObliqueNumber;
        }

        policy NominalHeadCore {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
            export CardinalMeasurementUse = head.CardinalMeasurementUse;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
        }

        policy PredicateHostAgreement {
            export CliticHost = head.CliticHost;
            export number = head.number;
            export person = head.person;
        }

        policy PredicateHeadAgreement {
            export number = head.number;
            export person = head.person;
        }

        policy NoFeatures {
        }

        policy NominativeCase {
            export SubjectStructure = head.SubjectStructure;
            export CaseUse = nominative_case(head.CaseUse);
        }

        policy AccusativeCase {
            export CaseUse = accusative_case(head.CaseUse);
        }

        policy SecondaryProjection {
            export InternalisedComplementPresent = head.InternalisedComplementPresent;
            export ParticipialUse = head.ParticipialUse;
            export form = secondary_form(head.form);
        }

        policy SecondaryAdjunctProjection {
            export InternalisedComplementPresent = head.InternalisedComplementPresent;
            export ParticipialUse = head.ParticipialUse;
            export form = head.form;
        }

        policy OrdinarySecondaryHead {
            export form = secondary_form(head.form);
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy OrdinarySelectedPredicate {
            export form = head.form;
            export ParticipialUse = Ordinary;
            export InternalisedComplementPresent = No;
        }

        policy NounPhraseHeadAgreement {
            export SubjectStructure = Phrase;
            export BareGenitiveHost = head.BareGenitiveHost;
            export Targeting = head.Targeting;
            export number = head.number;
            export person = Third;
            export CaseUse = Common;
        }

        policy ThirdPersonCommonCase {
            export SubjectStructure = Phrase;
            export person = Third;
            export CaseUse = Common;
        }

        policy UnmodifiedNominalProperties {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = head.BarePrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export DeterminerRequirement = No;
            export MeasurePosition = head.MeasurePosition;
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
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export NominalComplementMarker = head.NominalComplementMarker;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export LocativeUse = head.LocativeUse;
        }

        policy AdjectiveStructureMerge<Right, Source> {
            export AdjectiveStructure = coordinated_adjective_structure(left.AdjectiveStructure,
                Right.AdjectiveStructure);
        }

        policy AdverbPermissions<Right, Source> {
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
            (Before, Before) => Before, (After, After) => After,
            (None, None) => None, (None, Before) => None, (None, After) => None,
            (Before, None) => None, (After, None) => None,
            (Before, After) => None, (After, Before) => None,
        }
        table coordinated_cardinal_measurement(CardinalMeasurementUse, CardinalMeasurementUse)
            -> CardinalMeasurementUse {
            (Yes, Yes) => Yes, (No, No) => No, (No, Yes) => No, (Yes, No) => No,
        }
        policy NominalConcord<Right, Source> {
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            export CardinalMeasurementUse = coordinated_cardinal_measurement(
                left.CardinalMeasurementUse, Right.CardinalMeasurementUse);
            export MeasurePosition = coordinated_measure_position(
                left.MeasurePosition, Right.MeasurePosition);
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

        table coordinated_targeting(Targeting, Targeting) -> Targeting {
            (No, No) => No, (No, Yes) => Yes, (Yes, No) => Yes, (Yes, Yes) => Yes,
        }
        policy NounCoordinationAgreement<Right, Source> {
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
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
            export PredicativeKind = unlike_predicative_kind(left.PredicativeKind,
                Right.PredicativeKind);
        }

        policy ObliquePrepositionConcord<Right, Source> {
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct,
                Right.ClauseInitialAdjunct);
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export NominalComplementMarker = None;
            export InternalisedComplementMarker = complement_marker_concord(
                left.InternalisedComplementMarker, Right.InternalisedComplementMarker);
            export PrepositionFunctionLicence = coordinated_preposition_licence(left.PrepositionFunctionLicence, Right.PrepositionFunctionLicence);
            export LocativeUse = coordinated_locative_use(left.LocativeUse, Right.LocativeUse);
            export ObliqueNumber = coordinated_oblique(Source.CoordinationKind,
                left.ObliqueNumber, Right.ObliqueNumber);
        }

        policy PrepositionPermissions<Right, Source> {
            export SelectedPrepositionUse = selected_preposition_concord(left.SelectedPrepositionUse,
                Right.SelectedPrepositionUse);
            export NominalComplementMarker = None;
            export InternalisedComplementMarker = complement_marker_concord(
                left.InternalisedComplementMarker, Right.InternalisedComplementMarker);
            export PrepositionFunctionLicence = coordinated_preposition_licence(left.PrepositionFunctionLicence, Right.PrepositionFunctionLicence);
            export LocativeUse = coordinated_locative_use(left.LocativeUse, Right.LocativeUse);
        }

        policy SecondaryConcord<Right, Source> {
            agree left.form = Right.form;
            export form = left.form;
        }

        policy SecondaryConjunctProperties<Right, Source> {
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

        schema Coordination {
            form [left: node, " ", coordinator: lexical(Coordinator), " ", right: node]
                require coordinator.BareCoordination = Yes;
            form [left: node, ", ", coordinator: lexical(Coordinator), " ", right: node]
                require coordinator.CommaCoordination = Yes;
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

        schema FrequencyPredicate {
            form [head: node, " ", modifier: node];
        }

        schema PrepositionPredicate {
            form [head: node, " ", modifier: node];
            require licence_adjunct(modifier.PrepositionFunctionLicence) = Yes;
            require modifier.SelectedPrepositionUse = No;
        }

        // CGEL Ch. 16 §10.1.1: this function belongs to the embedded Bare Passive.
        construction InternalisedComplementPredicate: SecondaryVerbPhrase {
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
            form [head: lexical(Catalog), rest: repeat(SubtypeContinuation, "")];
            require head.TypeLineRole = Subtype;
        }

        construction SubtypeContinuation: SubtypeContinuation {
            form [" ", head: lexical(Catalog)];
            require head.TypeLineRole = Subtype;
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
            form ["“", text: Document, "”"];
        }

        construction QuotedClause: QuotedText {
            export QuotationStructure = Simple;
            boundary Interior;
            form ["\"", clause: Clause, "\""];
            form ["“", clause: Clause, "”"];
        }

        construction QuotedKeyword: QuotedText {
            export QuotationStructure = Simple;
            boundary Interior;
            form ["\"", keyword: KeywordPhrase, "\""];
            form ["\"", keyword: KeywordPhrase, ".\""];
            form ["“", keyword: KeywordPhrase, "”"];
            form ["“", keyword: KeywordPhrase, ".”"];
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

        construction CostSymbols: CostSymbols {
            form [first: CostSymbol, rest: repeat(CostSymbol, "")];
        }

        construction ManaPhrase: ManaPhrase {
            form [first: ManaSymbol, rest: repeat(ManaSymbol, "")];
        }

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
        table parameter_separator(KeywordParameterSeparator, KeywordSeparator) -> Selection {
            (Space, Space) => Yes, (Dash, Dash) => Yes, (SpacedDash, SpacedDash) => Yes,
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
        construction SpaceKeywordSeparator: KeywordSeparator {
            form [" "];
            export KeywordSeparator = Space;
        }
        construction DashKeywordSeparator: KeywordSeparator {
            form ["—"];
            export KeywordSeparator = Dash;
        }
        construction SpacedDashKeywordSeparator: KeywordSeparator {
            form [" — "];
            export KeywordSeparator = SpacedDash;
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
                separator: KeywordSeparator, cost: CostSymbols];
            require head.KeywordParameterClass = AmountCost;
            require head.KeywordSeparator = Space;
            require amount.MeasureKind = Scalar;
            require parameter_separator(head.KeywordParameterSeparator,
                separator.KeywordSeparator) = Yes;
        }
        construction QualityCostKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", quality: KeywordQualityPreposition,
                separator: KeywordSeparator, cost: CostSymbols];
            require head.KeywordParameterClass = QualityCost;
            require head.KeywordSeparator = Space;
            agree head.KeywordMarker = quality.KeywordMarker;
            require compatible_quality(head.KeywordQualityNumber,
                quality.KeywordQualityNumber) = Yes;
            require parameter_separator(head.KeywordParameterSeparator,
                separator.KeywordSeparator) = Yes;
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
            form [head: lexical(Keyword), separator: KeywordSeparator,
                body: ClausalKeywordPayload];
            agree head.KeywordParameterClass = body.KeywordParameterClass;
            agree head.KeywordSeparator = separator.KeywordSeparator;
        }
        construction CostPowerToughnessKeyword: KeywordPhrase {
            form [head: lexical(Keyword), " ", cost: CostSymbols,
                separator: KeywordSeparator, size: SlashPair];
            require head.KeywordParameterClass = CostPowerToughness;
            require head.KeywordSeparator = Space;
            require parameter_separator(head.KeywordParameterSeparator,
                separator.KeywordSeparator) = Yes;
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

        construction Sentence: Sentence {
            boundary Initial;
            form [clause: Clause, "."]
                require clause.terminal_punctuation = None;
            form [clause: Clause]
                require clause.terminal_punctuation = QuotedFullStop;
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
            form [subject: NominativePhrase, " ", predicate: FinitePredicate]
                require predicate.CliticHost = Free;
            form [subject: NominativePhrase, predicate: FinitePredicate]
                require joined_clitic(predicate.CliticHost) = Yes;
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        construction InitialPreposition: Clause {
            form [dependent: PrepositionPhrase, ", ", clause: Clause];
            form [dependent: PrepositionPhrase, " ", clause: Clause]
                require dependent.ClauseInitialAdjunct = Yes;
            require licence_preposed_adjunct(dependent.PrepositionFunctionLicence) = Yes;
            require dependent.SelectedPrepositionUse = No;
            export UnmarkedConjunctLicence = Yes;
        }

        construction ClausalPreposition: Clause {
            form [clause: Clause, " ", dependent: PrepositionPhrase];
            require licence_adjunct(dependent.PrepositionFunctionLicence) = Yes;
            require dependent.SelectedPrepositionUse = No;
            export UnmarkedConjunctLicence = clause.UnmarkedConjunctLicence;
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
            (BareObjectGapSeries, BareObjectGap, NoConcord, NoConcord, NoConcord),
            (FiniteClauseSeries, FiniteClause, NoConcord, NoConcord, NoConcord),
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
            (BareObjectGapSeries, BareObjectGap, Self),
            (FiniteClauseSeries, FiniteClause, Self),
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
            (BareObjectGap, Self, BareObjectGapSeries),
            (CoordinatedFiniteClause, FiniteClause, FiniteClauseSeries),
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
        }

        construction IntransitiveAdjective: AdjectivePhrase {
            form [head: lexical(Adjective)];
            require head.frame = Intransitive;
            export AdjectiveStructure = Simple;
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
            (BareObjectGap, Self, NoConcord, NoConcord, NoConcord),
            (CoordinatedFiniteClause, FiniteClause, NoConcord, NoConcord, NoConcord),
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
            use NominalHeadProperties;
            export SelectedPrepositionUse = No;
            require head.Targeting = No;
        }

        construction PremodifiedNominal: Nominal {
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: AdjectivePhrase, " ", head: Nominal];
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            require modifier.AdjectiveStructure = Simple;
            require head.Targeting = No;
        }

        // CGEL Ch. 5 §7.6, p. 386: numerical cardinals can modify under
        // an outer Determiner; without it the cardinal is the Determiner.
        construction CardinalPremodifiedNominal: Nominal {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            export MeasurePosition = head.MeasurePosition;
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
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
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
            export CardinalMeasurementUse = No;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = None;
            export SlashPremodifierUse = No;
            export NumberTransparency = No;
            export ObliqueNumber = head.ObliqueNumber;
            export BareGenitiveHost = No;
        }

        construction CardinalMeasuredNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
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
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
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
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            require head.Targeting = No;
        }

        construction CatalogName: Name {
            form [head: lexical(Catalog)];
            require head.IdentityUse = Name;
        }

        // CGEL p. 516: a referential proper name has NP status. Oracle names
        // refer to one card even when the written title contains plural nouns.
        construction ProperNameNounPhrase: NounPhrase {
            export SelectedPrepositionUse = No;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: Name];
            export number = Singular;
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
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
        }

        construction TargetedNominal: Nominal {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = No;
            export NominalComplementMarker = None;
            export DeterminerRequirement = head.DeterminerRequirement;
            export MeasurePosition = head.MeasurePosition;
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
            require modifier.SelectedPrepositionUse = No;
            require nominal_postmodifier_licence(head.NominalComplementMarker,
                modifier.NominalComplementMarker) = Yes;
            use NominalHeadCore;
            export ObliqueNumber = nominal_oblique(head.ObliqueNumber, modifier.ObliqueNumber);
        }

        table partitive_oblique(ObliqueNumber) -> Selection {
            (Singular) => Yes, (Plural) => Yes,
        }
        construction PartitiveModifier: PartitiveModifier {
            form [" ", phrase: PrepositionPhrase];
            require licence_modifier(phrase.PrepositionFunctionLicence) = Yes;
            require phrase.SelectedPrepositionUse = No;
        }
        construction PartitiveNounPhrase: NounPhrase {
            export SelectedPrepositionUse = No;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            form [quantity: Cardinal, " ", complement: PrepositionPhrase,
                modifiers: repeat(PartitiveModifier, "")];
            require partitive_oblique(complement.ObliqueNumber) = Yes;
            require licence_noun_complement(complement.PrepositionFunctionLicence) = Yes;
            export number = quantity.number;
            export Targeting = No;
            use ThirdPersonCommonCase;
        }

        construction DeterminedNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = No;
            export BareGenitiveHost = head.BareGenitiveHost;
            export Targeting = head.Targeting;
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Ordinary;
            require determiner_license(determiner.DeterminerUse, head.number,
                head.countability) = Yes;
            export number = quantificational_number(determiner.QuantificationalDeterminer,
                head.NumberTransparency, head.ObliqueNumber, head.number);
            use ThirdPersonCommonCase;
        }

        construction IndefiniteNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = No;
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Indefinite;
            require determiner.number = Singular;
            require head.number = Singular;
            require head.countability = Count;
            agree determiner.article_onset = head.onset;
            use NounPhraseHeadAgreement;
        }

        construction BarePlural: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = No;
            form [head: Nominal];
            require head.number = Plural;
            require head.countability = Count;
            require head.DeterminerRequirement = No;
            use NounPhraseHeadAgreement;
        }

        construction BareMass: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = No;
            form [head: Nominal];
            require head.number = Singular;
            require head.countability = Mass;
            require head.DeterminerRequirement = No;
            use NounPhraseHeadAgreement;
        }

        // CGEL Ch. 7 §3.1, pp. 620–623: the bare NP is internal to the PP.
        construction BarePrepositionNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            form [head: Nominal];
            require head.BarePrepositionUse = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.DeterminerRequirement = No;
            export BarePrepositionUse = Yes;
            use NounPhraseHeadAgreement;
        }

        construction TargetNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = No;
            export BareGenitiveHost = head.BareGenitiveHost;
            form [marker: lexical(Determinative), " ", head: Nominal];
            require marker.Targeting = Yes;
            require head.Targeting = No;
            require head.number = Singular;
            require head.DeterminerRequirement = No;
            export number = head.number;
            use ThirdPersonCommonCase;
            export Targeting = Yes;
        }

        construction NominativePronoun: NounPhrase {
            export SelectedPrepositionUse = No;
            export BarePrepositionUse = No;
            export SubjectStructure = Pronoun;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: lexical(Pronoun)];
            use PredicateHeadAgreement;
            require head.case = Nominative;
            export CaseUse = Nominative;
        }
        instance CasePhrase<Result, Head: head, Properties>: [
            (NominativePhrase, NounPhrase, NominativeCase),
            (AccusativePhrase, NounPhrase, AccusativeCase),
        ] {
            require head.BarePrepositionUse = No;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            use Properties;
        }

        construction AccusativePronoun: NounPhrase {
            export SelectedPrepositionUse = No;
            export BarePrepositionUse = No;
            export SubjectStructure = Phrase;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: lexical(Pronoun)];
            use PredicateHeadAgreement;
            require head.case = Accusative;
            export CaseUse = Accusative;
        }

        construction PrepositionPhrase: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: AccusativePhrase];
            require head.PrepositionComplement = NounPhrase;
            export ObliqueNumber = oblique_number(head.ObliqueMarker, complement.number);
            export NominalComplementMarker = head.NominalComplementMarker;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export LocativeUse = head.LocativeUse;
            export SelectedPrepositionUse = selected_preposition_complement(head.SelectedPrepositionUse,
                complement.SelectedPrepositionUse);
        }

        construction BareNominalPreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: NounPhrase];
            require complement.BarePrepositionUse = Yes;
            require head.BareNominalComplement = Yes;
            require head.PrepositionComplement = NounPhrase;
            export ObliqueNumber = oblique_number(head.ObliqueMarker, complement.number);
            export NominalComplementMarker = head.NominalComplementMarker;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
            export LocativeUse = head.LocativeUse;
            export SelectedPrepositionUse = selected_preposition_complement(head.SelectedPrepositionUse,
                complement.SelectedPrepositionUse);
        }

        // CGEL Ch. 7 §5.1, pp. 636–637: a predicative NP Complement
        // differs from an Object; adjunct use excludes adjective Complements.
        // The supported predicative-NP licence is preposed; finite-clause
        // Complement permissions remain independent.
        construction PredicativeComplementPreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
            export NominalComplementMarker = None;
            form [head: lexical(Preposition), " ", complement: PredicativeComplement];
            require head.PrepositionComplement = PredicativeNounPhrase;
            require complement.PredicativeKind = Nominal;
            require licence_preposed_adjunct(head.PrepositionFunctionLicence) = Yes;
            export ObliqueNumber = None;
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export PrepositionFunctionLicence = PreposedAdjunct;
            export LocativeUse = head.LocativeUse;
        }

        construction IntransitivePreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export ObliqueNumber = None;
            form [head: lexical(Preposition)];
            require head.PrepositionComplement = None;
            use PrepositionHeadPermissions;
        }

        // CGEL Ch. 7 §3.2, p. 626: preposition + adjective idioms.
        construction AdjectiveComplementPreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
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

        construction ClauseComplementPreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: FiniteClause];
            require head.FiniteClauseComplement = Yes;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export PrepositionFunctionLicence = head.PrepositionFunctionLicence;
        }

        // CGEL Ch. 7 §5.1(d), p. 640: restricted AdvP Complements
        // include duration use. The lexical licence and its application to
        // the Oracle equative duration are declared project analyses.
        construction AdverbComplementPreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: AdverbPhrase];
            require head.AdverbComplementClass = Duration;
            require complement.DurationUse = Yes;
            export SelectedPrepositionUse = No;
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
            export InternalisedComplementMarker = No;
            export LocativeUse = No;
            export PrepositionFunctionLicence = Adjunct;
        }

        // CGEL Ch. 8 §2.2: gerund-participials occur under means By independently
        // of passive voice; the same complementation shape also occurs under To.
        construction GerundComplementPreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: SecondaryVerbPhrase];
            require head.GerundClauseComplement = Yes;
            require complement.form = GerundParticiple;
            export ObliqueNumber = None;
            use PrepositionHeadPermissions;
        }

        construction PredicativePreposition: PredicativeComplement {
            form [phrase: PrepositionPhrase];
            require licence_predicative_complement(phrase.PrepositionFunctionLicence) = Yes;
            require phrase.LocativeUse = No;
            export PredicativeKind = Prepositional;
        }
        // CGEL Ch. 7 §2.4, pp. 616–617: the selected PP is a constituent;
        // omission removes its head together with its internal Complement.
        construction CompoundPrepositionPhrase: PrepositionPhrase {
            export ClauseInitialAdjunct = No;
            form [head: lexical(Preposition), " ", complement: PrepositionPhrase];
            require selected_compound_complement(head.CompoundComplementMarker,
                complement.NominalComplementMarker) = Yes;
            require licence_compound(head.PrepositionFunctionLicence) = Yes;
            export ObliqueNumber = None;
            use PrepositionHeadPermissions;
        }

        construction PrepositionComplementPreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            form [head: lexical(Preposition), " ", complement: PrepositionPhrase];
            require head.PrepositionPhraseComplement = Yes;
            export ObliqueNumber = None;
            use PrepositionHeadPermissions;
        }


        construction LocativeComplement: LocativeComplement {
            form [phrase: PrepositionPhrase];
            require phrase.LocativeUse = Yes;
            require phrase.SelectedPrepositionUse = No;
            require licence_predicative_complement(phrase.PrepositionFunctionLicence) = Yes;
        }

        construction MannerComplement: MannerComplement {
            form [phrase: AccusativePhrase];
            require phrase.SelectedPrepositionUse = Yes;
        }

        construction AdjectivalComplement: PredicativeComplement {
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
            form [phrase: AccusativePhrase];
            export PredicativeKind = Nominal;
        }
        instance BareAuxiliaryPredicate<Result, Complement: complement, Properties>: [
            (FinitePredicate, BareComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, BareComplement, OrdinarySecondaryHead),
        ] {
            use Properties;
        }
        instance ParticipialAuxiliaryPredicate<Result, Complement: complement, Properties>: [
            (FinitePredicate, ParticipialComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, ParticipialComplement, OrdinarySecondaryHead),
        ] {
            use Properties;
        }
        instance PerfectAuxiliaryPredicate<Result, Complement: complement, Properties>: [
            (FinitePredicate, PerfectComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, PerfectComplement, OrdinarySecondaryHead),
        ] {
            use Properties;
        }

        table selected_object_marker(frame, KeywordMarker) -> Selection {
            (ObjectToObject, To) => Yes, (ObjectIntoObject, Into) => Yes,
            (ObjectOntoObject, Onto) => Yes,
            (ObjectForObject, For) => Yes,
        }
        // CGEL Ch15 §4.3 pp1341–1343: parallel NP+PP tails under one selected verb.
        construction ObjectPrepositionTail: SelectedComplementTail {
            form [object: AccusativePhrase, " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require marker.PrepositionComplement = NounPhrase;
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
        schema SelectedObjectPrepositionPredicate {
            form [head: lexical(Verb), " ", object: AccusativePhrase, " ",
                marker: lexical(Preposition), " ", complement: AccusativePhrase];
            require selected_object_marker(head.frame, marker.KeywordMarker) = Yes;
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
            require head.form = Plain;
        }
        policy BarePassiveHead {
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
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
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
            (BareObjectGap, Self, NoFeatures),
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
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: BareTemporalNominal];
            agree head.NominalBareClass = complement.NominalBareClass;
            use PrepositionHeadPermissions;
        }

        // The declared modifier licenses this Oracle-register bare singular NP.
        construction BareStatusNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
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
            form [marker: lexical(Pronoun), " ", predicate: FinitePredicate]
                require predicate.CliticHost = Free;
            form [marker: lexical(Pronoun), predicate: FinitePredicate]
                require joined_clitic(predicate.CliticHost) = Yes;
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
            (SecondaryVerbPhrase, Self, PrepositionPhrase, SecondaryProjection),
            (FiniteObjectGap, Self, PrepositionPhrase, PredicateHostAgreement),
            (BareObjectGap, Self, PrepositionPhrase, NoFeatures),
        ] {
            use Properties;
        }

        construction BarePredicate: BarePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = Plain;
        }

        construction ProgressiveComplement: ParticipialPredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = GerundParticiple;
        }

        // CGEL p. 1327: unlike AdjP and progressive complements coordinated under be.
        // The shared phrase describes their constituent structure; here it is a complement.
        construction MixedCopularProgressiveComplement: ParticipialPredicate {
            form [phrase: DepictivePhrase];
            require phrase.DepictiveKind = Mixed;
        }

        construction PassiveComplement: ParticipialPredicate {
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
            export ParticipialUse = BarePassive;
            export InternalisedComplementPresent = No;
        }

        construction SubjectRelativeClause: SubjectRelativeClause {
            form [marker: lexical(Subordinator), " ", predicate: FinitePredicate]
                require predicate.CliticHost = Free;
            form [marker: lexical(Subordinator), predicate: FinitePredicate]
                require joined_clitic(predicate.CliticHost) = Yes;
            require marker.RelativeSubordinator = Yes;
            require relative_clitic(predicate.CliticHost) = Yes;
            require predicate.person = Third;
            export number = predicate.number;
        }

        construction SubjectRelativeNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", relative: SubjectRelativeClause];
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            agree head.number = relative.number;
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

        construction AuxiliaryObjectGap: FiniteObjectGap {
            form [head: lexical(Verb), " ", complement: BareObjectGap];
            use FiniteHeadAgreement;
            require head.frame = BareAuxiliary;
        }

        construction ObjectRelativeClause: ObjectRelativeClause {
            form [marker: lexical(Subordinator), " ", subject: NominativePhrase, " ",
                predicate: FiniteObjectGap] require predicate.CliticHost = Free;
            form [marker: lexical(Subordinator), " ", subject: NominativePhrase,
                predicate: FiniteObjectGap] require joined_clitic(predicate.CliticHost) = Yes;
            require marker.RelativeSubordinator = Yes;
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        construction ZeroObjectRelativeClause: ObjectRelativeClause {
            form [subject: NominativePhrase, " ", predicate: FiniteObjectGap]
                require predicate.CliticHost = Free;
            form [subject: NominativePhrase, predicate: FiniteObjectGap]
                require joined_clitic(predicate.CliticHost) = Yes;
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
            require clitic_subject(subject.SubjectStructure, predicate.CliticHost) = Yes;
        }

        construction ObjectRelativeNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", relative: ObjectRelativeClause];
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
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
        }

        construction BareEllipsis: BareComplement {
            form [omission: Ellipsis];
            require omission.form = Plain;
            export AuxiliaryComplementRealization = Elided;
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

        construction VariableCount: Cardinal {
            form [head: lexical(Numeral)];
            require head.ScalarVariable = Yes;
            export number = Plural;
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
            export number = Plural;
        }

        // CGEL Ch. 5 §11(d), p. 432: the quantifier in the PP Complement
        // controls head selection, including singular more than one.
        construction ComparativeDeterminativePhrase: ComparativeQuantity {
            form [head: lexical(Determinative), " ", complement: ComparativePrepositionPhrase];
            require head.ComparativeQuantityUse = Yes;
            export number = complement.number;
        }

        construction ComparativeQuantityDeterminer: QuantitativeDeterminer {
            cost 0;
            form [value: ComparativeQuantity];
            export number = value.number;
        }

        construction CardinalDeterminer: QuantitativeDeterminer {
            cost 0;
            form [value: Cardinal];
            export number = value.number;
        }

        construction PrepositionDeterminer: QuantitativeDeterminer {
            cost 0;
            form [value: QuantitativePrepositionPhrase];
            export number = value.number;
        }

        construction CountedNounPhrase: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = No;
            form [quantity: QuantitativeDeterminer, " ", head: Nominal];
            require head.countability = Count;
            require head.DeterminerRequirement = No;
            agree quantity.number = head.number;
            use NounPhraseHeadAgreement;
        }

        construction OrdinalPremodifier: AdjectivePhrase {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Ordinal;
            export AdjectiveStructure = Simple;
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

        construction VariableScalar: UnsignedScalar {
            form [head: lexical(Numeral)];
            require head.ScalarVariable = Yes;
        }

        construction UnsignedScalar: ScalarComponent {
            form [value: UnsignedScalar];
        }

        construction PositiveScalar: ScalarComponent {
            onset Consonant;
            form ["+", value: UnsignedScalar];
        }

        construction NegativeScalar: ScalarComponent {
            onset Consonant;
            form ["-", value: UnsignedScalar];
        }

        construction SlashPair: SlashPair {
            form [left: ScalarComponent, "/", right: ScalarComponent];
        }

        construction SlashMeasure: MeasurePhrase {
            form [pair: SlashPair];
            export MeasureKind = Pair;
        }

        table slash_premodifier_concord(SlashPremodifierUse, SlashPremodifierUse)
            -> SlashPremodifierUse {
            (Yes, Yes) => Yes, (Yes, No) => No, (No, Yes) => No, (No, No) => No,
        }

        construction SlashModifiedNominal: Nominal {
            export BareGenitiveHost = head.BareGenitiveHost;
            form [modifier: SlashPair, " ", head: Nominal];
            require head.SlashPremodifierUse = Yes;
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            require head.Targeting = No;
            require head.countability = Count;
        }

        construction UngroupedScalarNumeral: MeasurePhrase {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Arabic;
            require head.numeral_size = Small;
            export MeasureKind = Scalar;
        }

        construction ScalarVariable: MeasurePhrase {
            form [head: lexical(Numeral)];
            require head.ScalarVariable = Yes;
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
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
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

        construction MeasuredAttribute: NounPhrase {
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
            export BarePrepositionUse = No;
            export BareGenitiveHost = No;
            export Targeting = No;
            form [head: lexical(Noun), " ", quantity: MeasurePhrase];
            require head.MeasurePosition = After;
            require quantity.MeasureKind = Scalar;
            require head.number = Singular;
            export number = Singular;
            use ThirdPersonCommonCase;
        }

        // CGEL Ch. 13 §1.3, pp. 1103–1104 distinguishes the bare comparative
        // Complement from the expanded Complement that includes its marker.
        table comparative_marker(frame, ComparisonMarker) -> Selection {
            (Equality, Equality) => Yes, (Ordering, Ordering) => Yes,
        }

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

        construction ToInfinitive: InfinitiveComplement {
            form [marker: lexical(Subordinator), " ", predicate: BarePredicate];
            require marker.InfinitivalMarker = Yes;
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
            (FinitePredicate, FiniteSelectedHead, BareComplement, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, BareComplement,
                AuxiliarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAuxiliaryParticipleComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, ParticipialComplement, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, ParticipialComplement,
                OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAuxiliaryPerfectComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, PerfectComplement, PredicateHostAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, PerfectComplement,
                OrdinarySelectedPredicate),
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
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
            export NominalComplementMarker = None;
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: CoordinatedFiniteClause];
            require head.FiniteClauseComplement = Yes;
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
            (BareObjectGap, Self),
            (CoordinatedFiniteClause, FiniteClause),
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
            (CorrelativeBareObjectGapSeries, BareObjectGap),
            (CorrelativeFiniteClauseSeries, FiniteClause),
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
            (CorrelativeBareObjectGapSeries, BareObjectGap, Self),
            (CorrelativeFiniteClauseSeries, FiniteClause, Self),
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
            (BareObjectGap, Self, CorrelativeBareObjectGapSeries),
            (CoordinatedFiniteClause, FiniteClause, CorrelativeFiniteClauseSeries),
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
            (BareObjectGap, Self),
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
            (BareObjectGap, Self),
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
            (BareObjectGap, Self, CorrelativeBareObjectGapSeries),
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
            export BarePrepositionUse = No;
            form [possessor: lexical(Pronoun), " ", head: Nominal];
            require possessor.case = Genitive;
            require possessor.NominalLicense = AnyNominal;
            use NounPhraseHeadAgreement;
        }

        construction KeywordComplementPreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: KeywordPhrase];
            require head.KeywordComplement = Yes;
            use PrepositionHeadPermissions;
        }
        construction QuotedComplementPreposition: PrepositionPhrase {
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
            export SelectedPrepositionUse = No;
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
            form [phrase: CorrelativeAdjectivePhrase];
            export PredicativeKind = Adjectival;
        }

        construction CorrelativePostpositiveNominal: Nominal {
            export BareGenitiveHost = No;
            form [head: Nominal, " ", modifier: CorrelativeAdjectivePhrase];
            use NominalHeadProperties;
            export SelectedPrepositionUse = head.SelectedPrepositionUse;
        }

        construction SelectedPrepositionHead: SelectedPrepositionHead {
            form [head: lexical(Preposition)];
            require head.PrepositionComplement = NounPhrase;
            export HeadCoordination = No;
            use PrepositionHeadPermissions;
        }

        construction SharedPrepositionComplement: PrepositionPhrase {
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
        }

    }
}
