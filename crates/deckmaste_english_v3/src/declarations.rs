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
        feature NumberTransparency { No, Yes } default No;
        feature QuantificationalDeterminer { No, Yes } default No;
        feature ObliqueMarker { No, Yes } default No;
        feature ObliqueNumber { None, Singular, Plural } default None;
        feature CaseUse { Common, Nominative, Accusative }
        feature Targeting { No, Yes }
        feature CoordinationKind { Additive, Alternative, Adversative }
        feature PrepositionComplement { NounPhrase, None }
        feature QuantitativeComplement { Cardinal, CardinalPrepositionPhrase }
        feature AdverbialUse { Yes, No }
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
        feature AuxiliaryComplementRealization { Overt, Elided }
        feature ScalarVariable { Yes }
        feature MeasureOperator { Yes }
        feature MeasurePosition { Before, After }
        feature MeasureKind { Scalar, Pair }
        feature ComparisonMarker { Equality, Ordering }
        feature KeywordParameterClass { Nullary, Amount, Cost, Quality, Subject, AmountCost,
            QualityCost, Ability, Condition, CostPowerToughness }
        feature NominalBareClass { None, Interval, Boundary }
        feature NominalAdjunctClass { None, Temporal, Manner } default None;
        feature SlashPremodifierUse { No, Yes } default Yes;
        feature NominalAdjunctDeterminer { None, Demonstrative, Temporal }
        feature BareSingularUse { No, Yes }
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
        feature LabelKind { None, AbilityWord, FlavorWord }
        feature SymbolUse { Cost }
        feature ManaSymbolUse { Yes }
        feature TypeLineRole { Supertype, CardType, Subtype }
        feature IdentityUse { Name }
        feature NominalComplementMarker { Of }
        feature NounPremodifier { Yes }
        feature VPFinalAdjunct { Yes, No }
        feature ClauseInitialAdjunct { Yes, No }
        feature InfinitivalMarker { Yes }
        feature FrameUse { Object, Predicative, Locative, Mana, Amount, Measure, SlashMeasure,
            Keyword, Quoted, AuxiliaryBare, AuxiliaryParticiple, AuxiliaryPerfect, ObjectName,
            ObjectEquality, Cardinal, Infinitive }
        feature HeadCoordination { No, Yes }
        feature NoncorrelativeCoordination { Yes, No }
        feature CorrelativeKind { Both, Either, Neither }
        feature CorrelativeCoordinator { And, Or, Nor }
        feature PredicativeKind { Adjectival, Nominal, Mixed }
        feature DepictiveKind { Adjectival, Participial, Mixed }
        feature NominalLicense { AnyNominal }
        feature KeywordComplement { Yes }

        category Document();
        category Ability(LabelKind);
        category AbilityContinuation();
        category Paragraph();
        category ParagraphItem();
        category ParagraphContinuation();
        category Sentence();
        category Clause();
        category FiniteClause();
        category FinitePredicate(number, person);
        category SecondaryVerbPhrase(form, ParticipialUse, InternalisedComplementPresent);
        category FiniteDepictiveHost(number, person);
        category SecondaryDepictiveHost(form, ParticipialUse, InternalisedComplementPresent);
        category BarePredicate();
        category ParticipialPredicate();
        category PastParticiplePredicate();
        category PredicativeComplement(PredicativeKind);
        category DepictivePhrase(DepictiveKind);
        category DepictivePhraseSeries(DepictiveKind);
        category Ellipsis(form);
        category BareComplement(AuxiliaryComplementRealization);
        category ParticipialComplement(AuxiliaryComplementRealization);
        category PerfectComplement(AuxiliaryComplementRealization);
        category Nominal(number, countability, Targeting, NominalAdjunctClass, SlashPremodifierUse,
            NumberTransparency, ObliqueNumber);
        category VerbalPremodifier();
        category VerbalPremodifierSeries();
        category NounPremodifier();
        category NounPremodifierSeries();
        category NounPhrase(number, person, CaseUse, Targeting);
        category NominativePhrase(number, person, CaseUse);
        category AccusativePhrase(number, person, CaseUse);
        category AdjectivePhrase(AdjectiveStructure);
        category Name();
        category NamePredicate();
        category PrepositionPhrase(LocativeUse, AdverbialUse, ObliqueNumber,
            InternalisedComplementMarker);
        category LocativeComplement();
        category FrequencyPhrase();
        category NominalAdjunctPhrase();
        category BareTemporalNominal(NominalBareClass);
        category BoundaryComplement();
        category SubjectRelativeClause(number);
        category ObjectRelativeClause();
        category FiniteObjectGap(number, person);
        category BareObjectGap();
        category Cardinal(number);
        category QuantitativeDeterminer(number);
        category CardinalPrepositionPhrase(number);
        category QuantitativePrepositionPhrase(number);
        category Amount();
        category MeasurePhrase(MeasureKind);
        category ScalarMeasurePhrase();
        category ScalarExtentComplement();
        category ScalarExtentMeasure();
        category UnsignedScalar();
        category ScalarComponent();
        category SlashPair();
        category EqualityComplement();
        category OrderingComplement();
        category KeywordPhrase();
        category KeywordQuality(KeywordQualityNumber);
        category KeywordQualityPreposition(KeywordMarker, KeywordQualityNumber);
        category KeywordQualityPrepositionSeries(KeywordMarker, KeywordQualityNumber);
        category SelectedComplementTail(KeywordMarker, HeadCoordination);
        category SelectedComplementTailSeries(KeywordMarker, HeadCoordination);
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
        category QuotedText();
        category Mode();
        category Modes();
        category ModeContinuation();
        category TypeLine();
        category SupertypePrefix();
        category CardTypes();
        category CardTypeContinuation();
        category Subtypes();
        category SubtypeContinuation();
        category ClauseSeries();
        category FinitePredicateSeries(number, person);
        category SecondaryPredicateSeries(form, ParticipialUse, InternalisedComplementPresent);
        category NounPhraseSeries(number, person, CaseUse, Targeting, CoordinationKind);
        category AdverbPhrase(VPFinalAdjunct, ClauseInitialAdjunct);
        category InfinitiveComplement();
        category FiniteSelectedHead(number, person, FrameUse, HeadCoordination);
        category SecondarySelectedHead(form, FrameUse, HeadCoordination);
        category FiniteSelectedHeadSeries(number, person, FrameUse);
        category SecondarySelectedHeadSeries(form, FrameUse);
        category ManaPhraseSeries();
        category CardinalSeries(number, CoordinationKind);
        category AmountSeries();
        category MeasurePhraseSeries(MeasureKind);
        category KeywordPhraseSeries();
        category QuotedTextSeries();
        category NominalSeries(
            number, countability, Targeting, NominalAdjunctClass, SlashPremodifierUse,
            NumberTransparency, ObliqueNumber, CoordinationKind
        );
        category AdjectivePhraseSeries(AdjectiveStructure);
        category PrepositionPhraseSeries(LocativeUse, AdverbialUse, ObliqueNumber, CoordinationKind,
            InternalisedComplementMarker);
        category AdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct);
        category InfinitiveComplementSeries();
        category FrequencyPhraseSeries();
        category FiniteObjectGapSeries(number, person);
        category BareObjectGapSeries();
        category CoordinatedFiniteClause();
        category FiniteClauseSeries();
        category PredicativeComplementSeries(PredicativeKind);
        category CorrelativeClauseSeries(CorrelativeCoordinator);
        category CorrelativeFinitePredicateSeries(number, person, CorrelativeCoordinator);
        category CorrelativeSecondaryVerbPhraseSeries(form, ParticipialUse, CorrelativeCoordinator,
            InternalisedComplementPresent);
        category CorrelativeNounPhraseSeries(number, person, CaseUse, Targeting,
            CorrelativeCoordinator, CoordinationKind);
        category CorrelativePrepositionPhraseSeries(LocativeUse, AdverbialUse, ObliqueNumber, CoordinationKind,
            CorrelativeCoordinator, InternalisedComplementMarker);
        category CorrelativeAdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct,
            CorrelativeCoordinator);
        category CorrelativeManaPhraseSeries(CorrelativeCoordinator);
        category CorrelativeCardinalSeries(number, CorrelativeCoordinator, CoordinationKind);
        category CorrelativeAmountSeries(CorrelativeCoordinator);
        category CorrelativeMeasurePhraseSeries(MeasureKind, CorrelativeCoordinator);
        category CorrelativeKeywordPhraseSeries(CorrelativeCoordinator);
        category CorrelativeQuotedTextSeries(CorrelativeCoordinator);
        category CorrelativeInfinitiveComplementSeries(CorrelativeCoordinator);
        category CorrelativeFiniteSelectedHeadSeries(number, person, FrameUse,
            CorrelativeCoordinator);
        category CorrelativeSecondarySelectedHeadSeries(form, FrameUse, CorrelativeCoordinator);
        category CorrelativeAdjectivePhrase();
        category CorrelativeAdjectiveSeries(CorrelativeCoordinator);
        category CorrelativeFiniteObjectGapSeries(number, person, CorrelativeCoordinator);
        category CorrelativeBareObjectGapSeries(CorrelativeCoordinator);
        category SelectedPrepositionHead(LocativeUse, AdverbialUse, HeadCoordination,
            InternalisedComplementMarker);
        category SelectedPrepositionHeadSeries(LocativeUse, AdverbialUse, InternalisedComplementMarker);
        category CorrelativeFiniteClauseSeries(CorrelativeCoordinator);

        frame_category NounPhrase = AccusativePhrase;
        frame_category ScalarEquality = EqualityComplement;
        frame_category PowerToughnessAdjustment = SlashPair;
        frame_category MeasurePhrase = ScalarMeasurePhrase;

        schema SelectedPredicate {
            form [head: lexical(Verb), complements: selected_frame(head, Predicate)];
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
        frame Predicative = Predicate(Complement(PredicativeComplement));
        frame Locative = Predicate(Complement(LocativeComplement));
        frame BareAuxiliary = Auxiliary(Complement(BarePredicate));
        frame ParticipialAuxiliary = Auxiliary(Complement(ParticipialPredicate));
        frame PerfectAuxiliary = Auxiliary(Complement(PastParticiplePredicate));
        frame Measure = Predicate(Complement(MeasurePhrase));
        frame SlashMeasure = Predicate(Complement(PowerToughnessAdjustment));
        frame Equality = Predicate(Marked(Preposition, To, Complement(MeasurePhrase)));
        frame Ordering = Predicate(Marked(Preposition, Than, Complement(MeasurePhrase)));
        frame ObjectEquality = Predicate(Object(NounPhrase), Complement(ScalarEquality));
        frame KeywordObject = Predicate(Object(KeywordPhrase));
        frame QuotedObject = Predicate(Object(QuotedText));
        frame AmountComplement = Predicate(Complement(Amount));
        frame CardinalComplement = Predicate(Complement(Cardinal));
        frame InfinitiveSelection = Predicate(Complement(InfinitiveComplement));

        frame ObjectToObject = Predicate(Object(NounPhrase), Preposition(To), Object(NounPhrase));
        frame ObjectIntoObject = Predicate(
            Object(NounPhrase), Preposition(Into), Object(NounPhrase)
        );
        frame ObjectOnObject = Predicate(Object(NounPhrase), Preposition(On), Object(NounPhrase));
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
            (KeywordObject) => Keyword,
            (QuotedObject) => Quoted,
            (BareAuxiliary) => AuxiliaryBare,
            (ParticipialAuxiliary) => AuxiliaryParticiple,
            (PerfectAuxiliary) => AuxiliaryPerfect,
            (ObjectName) => ObjectName,
            (ObjectEquality) => ObjectEquality,
            (CardinalComplement) => Cardinal,
            (InfinitiveSelection) => Infinitive,
        }

        policy FiniteHeadAgreement {
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        policy SecondaryHeadForm {
            export form = secondary_form(head.form);
        }

        policy NominalHeadProperties {
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            export ObliqueNumber = head.ObliqueNumber;
        }

        policy NominalHeadCore {
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
        }

        policy PredicateHeadAgreement {
            export number = head.number;
            export person = head.person;
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
            export Targeting = head.Targeting;
            export number = head.number;
            export person = Third;
            export CaseUse = Common;
        }

        policy ThirdPersonCommonCase {
            export person = Third;
            export CaseUse = Common;
        }

        policy UnmodifiedNominalProperties {
            export number = head.number;
            export countability = head.countability;
            export Targeting = No;
            export NominalAdjunctClass = head.NominalAdjunctClass;
            export SlashPremodifierUse = head.SlashPremodifierUse;
            export NumberTransparency = head.NumberTransparency;
            export ObliqueNumber = None;
        }

        policy PrepositionHeadPermissions {
            export InternalisedComplementMarker = head.InternalisedComplementMarker;
            export AdverbialUse = head.AdverbialUse;
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
            agree left.number = Right.number;
            agree left.person = Right.person;
            export number = left.number;
            export person = left.person;
        }

        policy NoConcord<Right, Source> {

        }

        policy NominalConcord<Right, Source> {
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
            export InternalisedComplementMarker = complement_marker_concord(
                left.InternalisedComplementMarker, Right.InternalisedComplementMarker);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, Right.AdverbialUse);
            export LocativeUse = coordinated_locative_use(left.LocativeUse, Right.LocativeUse);
            export ObliqueNumber = coordinated_oblique(Source.CoordinationKind,
                left.ObliqueNumber, Right.ObliqueNumber);
        }

        policy PrepositionPermissions<Right, Source> {
            export InternalisedComplementMarker = complement_marker_concord(
                left.InternalisedComplementMarker, Right.InternalisedComplementMarker);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, Right.AdverbialUse);
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
            agree left.FrameUse = Right.FrameUse;
            export FrameUse = left.FrameUse;
        }

        policy SharedFrameConcord<Right, Source> {
            agree left.FrameUse = Right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        policy SharedHeadStatus<Right, Source> {
            export HeadCoordination = Yes;
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
            form [left: node, " ", coordinator: lexical(Coordinator), " ", right: node];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        schema CasePhrase {
            form [head: node];
            use PredicateHeadAgreement;
        }

        // CGEL pp. 1522–1523: gerund-participial auxiliary stranding is outside Oracle English.
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
            form [head: lexical(Verb), complement: node];
            require head.frame = BareAuxiliary;
            require auxiliary_realization(head.form, complement.AuxiliaryComplementRealization)
                = Yes;
        }

        schema ParticipialAuxiliaryPredicate {
            form [head: lexical(Verb), complement: node];
            require head.frame = ParticipialAuxiliary;
            require auxiliary_realization(head.form, complement.AuxiliaryComplementRealization)
                = Yes;
        }

        schema PerfectAuxiliaryPredicate {
            form [head: lexical(Verb), complement: node];
            require head.frame = PerfectAuxiliary;
            require auxiliary_realization(head.form, complement.AuxiliaryComplementRealization)
                = Yes;
        }

        schema NominalAdjunctPredicate {
            form [head: node, " ", modifier: NominalAdjunctPhrase];
        }

        schema FrequencyPredicate {
            form [head: node, " ", modifier: node];
        }

        schema PrepositionPredicate {
            form [head: node, " ", modifier: node];
            require modifier.AdverbialUse = Yes;
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

        schema OvertComplement {
            form [" ", predicate: node];
            export AuxiliaryComplementRealization = Overt;
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

        schema SharedKeywordComplement {
            form [head: node, " ", object: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Keyword;
        }

        schema SharedQuotedComplement {
            form [head: node, " ", object: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Quoted;
        }

        schema SharedAuxiliaryBareComplement {
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

        schema SharedObjectEqualityComplement {
            form [head: node, " ", object: node, " ", complement: node];
            require head.HeadCoordination = Yes;
            require head.FrameUse = ObjectEquality;
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
            form ["\"", text: Document, "\""];
            form ["“", text: Document, "”"];
        }

        construction QuotedClause: QuotedText {
            boundary Interior;
            form ["\"", clause: Clause, "\""];
            form ["“", clause: Clause, "”"];
        }

        construction QuotedKeyword: QuotedText {
            boundary Interior;
            form ["\"", keyword: KeywordPhrase, "\""];
            form ["\"", keyword: KeywordPhrase, ".\""];
            form ["“", keyword: KeywordPhrase, "”"];
            form ["“", keyword: KeywordPhrase, ".”"];
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
            agree left.KeywordMarker = Right.KeywordMarker;
            export KeywordMarker = left.KeywordMarker;
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
            form [clause: Clause, "."];
        }

        construction Declarative: Clause {
            form [clause: FiniteClause];
        }

        construction Imperative: Clause {
            form [predicate: BarePredicate];
        }

        construction FiniteClause: FiniteClause {
            form [subject: NominativePhrase, " ", predicate: FinitePredicate];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
        }

        construction InitialPreposition: Clause {
            form [dependent: PrepositionPhrase, ", ", clause: Clause];
            require dependent.AdverbialUse = Yes;
        }

        construction ClausalPreposition: Clause {
            form [clause: Clause, " ", dependent: PrepositionPhrase];
            require dependent.AdverbialUse = Yes;
        }

        construction ClauseCoordination: Clause {
            form [left: Clause, ", ", coordinator: lexical(Coordinator), " ", right: Clause];
            form [left: Clause, " ", coordinator: lexical(Coordinator), " ", right: Clause];
            require coordinator.NoncorrelativeCoordination = Yes;
        }
        instance CoordinationSeriesEnd<Result, Member, Agreement = NoConcord,
            Properties = NoConcord>: [
            (ClauseSeries, Clause, NoConcord),
            (KeywordQualityPrepositionSeries, KeywordQualityPreposition, QualityPrepositionConcord),
            (SelectedComplementTailSeries, SelectedComplementTail, SelectedTailConcord,
                PrimitiveRightTail),
            (FinitePredicateSeries, FinitePredicate, FiniteConcord),
            (SecondaryPredicateSeries, SecondaryVerbPhrase, SecondaryConjunctProperties),
            (NounPhraseSeries, NounPhrase, NounCoordinationAgreement, CoordinatorKindSummary),
            (VerbalPremodifierSeries, VerbalPremodifier),
            (NounPremodifierSeries, NounPremodifier),
            (ManaPhraseSeries, ManaPhrase),
            (CardinalSeries, Cardinal, CardinalListEnd),
            (AmountSeries, Amount),
            (MeasurePhraseSeries, MeasurePhrase, CoordinatedMeasureKind),
            (KeywordPhraseSeries, KeywordPhrase),
            (QuotedTextSeries, QuotedText),
            (FiniteSelectedHeadSeries, FiniteSelectedHead, FiniteConcord, SelectedFrameConcord),
            (SecondarySelectedHeadSeries, SecondarySelectedHead, SecondaryConcord,
                SelectedFrameConcord),
            (NominalSeries, Nominal, NominalConcord, CoordinatorKindSummary),
            (AdjectivePhraseSeries, AdjectivePhrase, AdjectiveStructureMerge),
            (PrepositionPhraseSeries, PrepositionPhrase, ObliquePrepositionConcord, CoordinatorKindSummary),
            (AdverbPhraseSeries, AdverbPhrase, AdverbPermissions),
            (InfinitiveComplementSeries, InfinitiveComplement, NoConcord),
            (FrequencyPhraseSeries, FrequencyPhrase),
            (FiniteObjectGapSeries, FiniteObjectGap, FiniteConcord),
            (BareObjectGapSeries, BareObjectGap),
            (FiniteClauseSeries, FiniteClause),
            (DepictivePhraseSeries, DepictivePhrase, DepictiveListEnd),
            (PredicativeComplementSeries, PredicativeComplement, PredicativeListEnd),
            (SelectedPrepositionHeadSeries, SelectedPrepositionHead, PrepositionPermissions),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
        }
        instance CoordinationSeriesContinuation<
            Result, Member: left, Tail: rest, Agreement = NoConcord,
            Properties = NoConcord>: [
            (ClauseSeries, Clause, Self, NoConcord),
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
            (FiniteSelectedHeadSeries, FiniteSelectedHead, Self, FiniteConcord,
                SelectedFrameConcord),
            (SecondarySelectedHeadSeries, SecondarySelectedHead, Self, SecondaryConcord,
                SelectedFrameConcord),
            (NominalSeries, Nominal, Self, NominalConcord, CoordinatorKindSummary),
            (AdjectivePhraseSeries, AdjectivePhrase, Self, AdjectiveStructureMerge),
            (PrepositionPhraseSeries, PrepositionPhrase, Self, ObliquePrepositionConcord, CoordinatorKindSummary),
            (AdverbPhraseSeries, AdverbPhrase, Self, AdverbPermissions),
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
            (Clause, Self, ClauseSeries, NoConcord),
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
            (QuotedText, Self, QuotedTextSeries),
            (FiniteSelectedHead, Self, FiniteSelectedHeadSeries, FiniteConcord, SharedFrameConcord),
            (SecondarySelectedHead, Self, SecondarySelectedHeadSeries, SecondaryConcord,
                SharedFrameConcord),
            (Nominal, Self, NominalSeries, NominalConcord),
            (AdjectivePhrase, Self, AdjectivePhraseSeries, AdjectiveStructureMerge),
            (PrepositionPhrase, Self, PrepositionPhraseSeries, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPhraseSeries, AdverbPermissions),
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
            form [head: lexical(Noun)];
            require head.framing = Unframed;
            use UnmodifiedNominalProperties;
        }

        construction BareFramedNoun: Nominal {
            form [head: lexical(Noun)];
            require head.frame = BareNominal;
            use UnmodifiedNominalProperties;
        }

        construction SymbolComplementNominal: Nominal {
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ",
                complement: CostSymbols];
            require head.frame = NominalSymbols;
            require marker.NominalComplementMarker = Of;
            use UnmodifiedNominalProperties;
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
        instance Coordination<Result, Member, Agreement = NoConcord, Properties = NoConcord>: [
            (VerbalPremodifier, Self),
            (NounPremodifier, Self),
            (KeywordQualityPreposition, Self, QualityPrepositionConcord),
            (SelectedComplementTail, Self, SelectedTailConcord, PrimitiveRightTail),
            (NounPhrase, Self, NounCoordinationAgreement),
            (FinitePredicate, Self, FiniteConcord),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
            (FiniteObjectGap, Self, FiniteConcord),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self),
            (FiniteSelectedHead, Self, FiniteConcord, SharedFrameConcord),
            (SecondarySelectedHead, Self, SecondaryConcord, SharedFrameConcord),
            (Nominal, Self, NominalConcord),
            (AdjectivePhrase, Self, AdjectiveStructureMerge),
            (PrepositionPhrase, Self, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPermissions),
            (InfinitiveComplement, Self, NoConcord),
            (FrequencyPhrase, Self),
            (BareObjectGap, Self),
            (CoordinatedFiniteClause, FiniteClause),
            (DepictivePhrase, Self, UnlikeDepictives),
            (PredicativeComplement, Self, UnlikePredicatives),
            (SelectedPrepositionHead, Self, PrepositionPermissions, SharedHeadStatus),
        ] {
            bind left, right = Member;
            use Agreement(right, coordinator);
            use Properties(right, coordinator);
        }

        construction NounPremodifiedNominal: Nominal {
            form [modifier: NounPremodifier, " ", head: Nominal];
            use NominalHeadProperties;
            require head.Targeting = No;
        }

        construction PremodifiedNominal: Nominal {
            form [modifier: AdjectivePhrase, " ", head: Nominal];
            use NominalHeadProperties;
            require modifier.AdjectiveStructure = Simple;
            require head.Targeting = No;
        }

        construction PostpositiveNominal: Nominal {
            form [head: Nominal, " ", modifier: AdjectivePhrase];
            use NominalHeadProperties;
            require modifier.AdjectiveStructure = Complemented;
        }

        construction VerbalPremodifier: VerbalPremodifier {
            form [head: lexical(Verb)];
            require head.form = GerundParticiple;
            require head.frame = Intransitive;
        }

        construction ParticipialPremodifier: Nominal {
            form [modifier: VerbalPremodifier, " ", head: Nominal];
            use NominalHeadProperties;
            require head.Targeting = No;
        }

        construction CatalogName: Name {
            form [head: lexical(Catalog)];
            require head.IdentityUse = Name;
        }

        // CGEL p. 516: a referential proper name has NP status. Oracle names
        // refer to one card even when the written title contains plural nouns.
        construction ProperNameNounPhrase: NounPhrase {
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
            form [head: Nominal, " ", modifier: NamePredicate];
            use NominalHeadProperties;
        }

        construction TargetedNominal: Nominal {
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
            form [head: Nominal, " ", modifier: PrepositionPhrase];
            use NominalHeadCore;
            export ObliqueNumber = nominal_oblique(head.ObliqueNumber, modifier.ObliqueNumber);
        }

        construction DeterminedNounPhrase: NounPhrase {
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
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Indefinite;
            require determiner.number = Singular;
            require head.number = Singular;
            require head.countability = Count;
            agree determiner.article_onset = head.onset;
            use NounPhraseHeadAgreement;
        }

        construction BarePlural: NounPhrase {
            form [head: Nominal];
            require head.number = Plural;
            require head.countability = Count;
            use NounPhraseHeadAgreement;
        }

        construction BareMass: NounPhrase {
            form [head: Nominal];
            require head.number = Singular;
            require head.countability = Mass;
            use NounPhraseHeadAgreement;
        }

        construction TargetNounPhrase: NounPhrase {
            form [marker: lexical(Determinative), " ", head: Nominal];
            require marker.Targeting = Yes;
            require head.Targeting = No;
            require head.number = Singular;
            export number = head.number;
            use ThirdPersonCommonCase;
            export Targeting = Yes;
        }

        construction NominativePronoun: NounPhrase {
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
            use Properties;
        }

        construction AccusativePronoun: NounPhrase {
            export Targeting = No;
            form [head: lexical(Pronoun)];
            use PredicateHeadAgreement;
            require head.case = Accusative;
            export CaseUse = Accusative;
        }

        construction PrepositionPhrase: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: AccusativePhrase];
            require head.PrepositionComplement = NounPhrase;
            export ObliqueNumber = oblique_number(head.ObliqueMarker, complement.number);
            use PrepositionHeadPermissions;
        }

        construction IntransitivePreposition: PrepositionPhrase {
            export ObliqueNumber = None;
            form [head: lexical(Preposition)];
            require head.PrepositionComplement = None;
            use PrepositionHeadPermissions;
        }

        construction ClauseComplementPreposition: PrepositionPhrase {
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: FiniteClause];
            require head.FiniteClauseComplement = Yes;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export AdverbialUse = Yes;
        }

        // CGEL Ch. 8 §2.2: gerund-participials occur under means By independently
        // of passive voice; the same complementation shape also occurs under To.
        construction GerundComplementPreposition: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: SecondaryVerbPhrase];
            require head.GerundClauseComplement = Yes;
            require complement.form = GerundParticiple;
            export ObliqueNumber = None;
            use PrepositionHeadPermissions;
        }

        construction LocativeComplement: LocativeComplement {
            form [phrase: PrepositionPhrase];
            require phrase.LocativeUse = Yes;
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
            (ObjectOnObject, On) => Yes, (ObjectOntoObject, Onto) => Yes,
            (ObjectForObject, For) => Yes,
        }
        // CGEL Ch15 §4.3 pp1341–1343: parallel NP+PP tails under one selected verb.
        construction ObjectPrepositionTail: SelectedComplementTail {
            form [object: AccusativePhrase, " ", marker: lexical(Preposition), " ",
                complement: AccusativePhrase];
            require marker.PrepositionComplement = NounPhrase;
            export KeywordMarker = marker.KeywordMarker;
            export HeadCoordination = No;
        }
        schema SelectedComplementClustersPredicate {
            form [head: lexical(Verb), " ", tail: SelectedComplementTail];
            require tail.HeadCoordination = Yes;
            require selected_object_marker(head.frame, tail.KeywordMarker) = Yes;
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
            form [head: Nominal, " ", modifier: SecondaryVerbPhrase];
            require participial_postmodifier(modifier.form, modifier.ParticipialUse) = Yes;
            use NominalHeadProperties;
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
            (FinitePredicate, Self, PredicateHeadAgreement),
            (SecondaryVerbPhrase, Self, SecondaryProjection),
            (FiniteObjectGap, Self, PredicateHeadAgreement),
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
            export NominalBareClass = Boundary;
        }
        construction BareTemporalPreposition: PrepositionPhrase {
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: BareTemporalNominal];
            agree head.NominalBareClass = complement.NominalBareClass;
            use PrepositionHeadPermissions;
        }

        // The declared modifier licenses this Oracle-register bare singular NP.
        construction BareStatusNounPhrase: NounPhrase {
            form [modifier: lexical(Adjective), " ", head: Nominal];
            require modifier.BareSingularUse = Yes;
            require head.number = Singular;
            require head.countability = Count;
            require head.Targeting = No;
            use NounPhraseHeadAgreement;
        }

        construction PronounSubjectRelative: SubjectRelativeClause {
            form [marker: lexical(Pronoun), " ", predicate: FinitePredicate];
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
            (FinitePredicate, Self, FrequencyPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, Self, FrequencyPhrase, SecondaryProjection),
        ] {
            use Properties;
        }

        instance PrepositionPredicate<Result, Head: head, Modifier: modifier, Properties>: [
            (FinitePredicate, Self, PrepositionPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, Self, PrepositionPhrase, SecondaryProjection),
            (FiniteObjectGap, Self, PrepositionPhrase, PredicateHeadAgreement),
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
            form [marker: lexical(Subordinator), " ", predicate: FinitePredicate];
            require marker.RelativeSubordinator = Yes;
            require predicate.person = Third;
            export number = predicate.number;
        }

        construction SubjectRelativeNominal: Nominal {
            form [head: Nominal, " ", relative: SubjectRelativeClause];
            use NominalHeadProperties;
            agree head.number = relative.number;
        }

        construction FiniteObjectGap: FiniteObjectGap {
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
                predicate: FiniteObjectGap];
            require marker.RelativeSubordinator = Yes;
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
        }

        construction ZeroObjectRelativeClause: ObjectRelativeClause {
            form [subject: NominativePhrase, " ", predicate: FiniteObjectGap];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
        }

        construction ObjectRelativeNominal: Nominal {
            form [head: Nominal, " ", relative: ObjectRelativeClause];
            use NominalHeadProperties;
        }

        construction OmittedPlain: Ellipsis {
            form [];
            export form = Plain;
        }

        construction OmittedGerundParticiple: Ellipsis {
            form [];
            export form = GerundParticiple;
        }

        construction OmittedPastParticiple: Ellipsis {
            form [];
            export form = PastParticiple;
        }
        instance OvertComplement<Result, Predicate: predicate>: [
            (BareComplement, BarePredicate),
            (ParticipialComplement, ParticipialPredicate),
            (PerfectComplement, PastParticiplePredicate),
        ] {
        }

        construction BareEllipsis: BareComplement {
            form [omission: Ellipsis];
            require omission.form = Plain;
            export AuxiliaryComplementRealization = Elided;
        }

        construction ProgressiveEllipsis: ParticipialComplement {
            form [omission: Ellipsis];
            require omission.form = GerundParticiple;
            export AuxiliaryComplementRealization = Elided;
        }

        construction PassiveEllipsis: ParticipialComplement {
            form [omission: Ellipsis];
            require omission.form = PastParticiple;
            export AuxiliaryComplementRealization = Elided;
        }

        construction PerfectEllipsis: PerfectComplement {
            form [omission: Ellipsis];
            require omission.form = PastParticiple;
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

        instance QuantitativePrepositionPhrase<Result, Complement: complement, Properties>: [
            (CardinalPrepositionPhrase, Cardinal, CardinalPrepositionComplement),
            (QuantitativePrepositionPhrase, CardinalPrepositionPhrase,
                QuantitativePrepositionComplement),
        ] {
            use Properties;
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
            form [quantity: QuantitativeDeterminer, " ", head: Nominal];
            require head.countability = Count;
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
            form [modifier: SlashPair, " ", head: Nominal];
            require head.SlashPremodifierUse = Yes;
            use NominalHeadProperties;
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
            export Targeting = No;
            form [head: lexical(Noun), " ", quantity: MeasurePhrase];
            require head.MeasurePosition = After;
            require quantity.MeasureKind = Scalar;
            require head.number = Singular;
            export number = Singular;
            use ThirdPersonCommonCase;
        }

        construction EqualityComplement: EqualityComplement {
            form [head: lexical(Adjective), " ", marker: lexical(Preposition), " ",
                measure: MeasurePhrase];
            require head.frame = Equality;
            require marker.ComparisonMarker = Equality;
            require measure.MeasureKind = Scalar;
        }

        construction EqualityAdjective: AdjectivePhrase {
            form [complement: EqualityComplement];
            export AdjectiveStructure = Complemented;
        }

        construction OrderingComplement: OrderingComplement {
            form [head: lexical(Adjective), " ", marker: lexical(Preposition), " ",
                measure: MeasurePhrase];
            require head.frame = Ordering;
            require marker.ComparisonMarker = Ordering;
            require measure.MeasureKind = Scalar;
        }

        construction OrderingAdjective: AdjectivePhrase {
            form [complement: OrderingComplement];
            export AdjectiveStructure = Complemented;
        }

        construction CardinalAmount: Amount {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }

        construction ScalarAmount: Amount {
            form [value: UnsignedScalar];
        }

        construction Adverb: AdverbPhrase {
            form [head: lexical(Adverb)];
            export VPFinalAdjunct = head.VPFinalAdjunct;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
        }
        instance DepictivePredicate<Result, Properties>: [
            (FinitePredicate, FiniteHeadAgreement),
            (SecondaryVerbPhrase, OrdinarySecondaryHead),
        ] {
            use Properties;
        }
        instance ComplementedDepictivePredicate<Result, Head: head, Properties>: [
            (FinitePredicate, FiniteDepictiveHost, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondaryDepictiveHost, SecondaryAdjunctProjection),
        ] {
            use Properties;
        }
        instance AdverbPredicate<Result, Head: head, Modifier: modifier, Properties>: [
            (FinitePredicate, Self, AdverbPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, Self, AdverbPhrase, SecondaryAdjunctProjection),
        ] {
            use Properties;
        }

        construction InitialAdverb: Clause {
            form [modifier: AdverbPhrase, " ", clause: Clause];
            form [modifier: AdverbPhrase, ", ", clause: Clause];
            require modifier.ClauseInitialAdjunct = Yes;
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
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase,
                OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedPredicativeComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, PredicativeComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, PredicativeComplement,
                OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedLocativeComplement<Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, LocativeComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, LocativeComplement,
                OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedManaComplement<Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, ManaPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, ManaPhrase, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAmountComplement<Result, Head: head, Amount: amount, Properties>: [
            (FinitePredicate, FiniteSelectedHead, Amount, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, Amount, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedMeasureComplement<Result, Head: head, Measure: measure, Properties>: [
            (FinitePredicate, FiniteSelectedHead, MeasurePhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, MeasurePhrase, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedSlashMeasureComplement<Result, Head: head, Measure: measure, Properties>: [
            (FinitePredicate, FiniteSelectedHead, SlashPair, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, SlashPair, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedKeywordComplement<Result, Head: head, Object: object, Properties>: [
            (FinitePredicate, FiniteSelectedHead, KeywordPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, KeywordPhrase, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedQuotedComplement<Result, Head: head, Object: object, Properties>: [
            (FinitePredicate, FiniteSelectedHead, QuotedText, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, QuotedText, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAuxiliaryBareComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, BareComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, BareComplement,
                AuxiliarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAuxiliaryParticipleComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, ParticipialComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, ParticipialComplement,
                AuxiliarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedAuxiliaryPerfectComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, PerfectComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, PerfectComplement,
                AuxiliarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedObjectNameComplement<
            Result, Head: head, Object: object, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, Name, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase, Name,
                OrdinarySelectedPredicate),
        ] {
            use Properties;
        }
        instance SharedObjectEqualityComplement<
            Result, Head: head, Object: object, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, EqualityComplement,
                PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase, EqualityComplement,
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

        table coordinated_adverbial_use(AdverbialUse, AdverbialUse) -> AdverbialUse {
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

        table coordinated_adjective_structure(AdjectiveStructure,
            AdjectiveStructure) -> AdjectiveStructure {
            (Simple, Simple) => Simple,
            (Simple, Complemented) => Complemented,
            (Complemented, Simple) => Complemented,
            (Complemented, Complemented) => Complemented,
        }

        construction CoordinatedClauseComplementPreposition: PrepositionPhrase {
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: CoordinatedFiniteClause];
            require head.FiniteClauseComplement = Yes;
            export LocativeUse = No;
            export InternalisedComplementMarker = No;
            export AdverbialUse = Yes;
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
            (Clause, Self, NoConcord),
            (FinitePredicate, Self, FiniteConcord),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
            (NounPhrase, Self, NounCoordinationAgreement),
            (PrepositionPhrase, Self, ObliquePrepositionConcord),
            (AdverbPhrase, Self, AdverbPermissions),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self),
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
            (CorrelativeClauseSeries, Clause, NoConcord),
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
            (CorrelativeClauseSeries, Clause, Self, NoConcord),
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
            (Clause, Self, CorrelativeClauseSeries, NoConcord),
            (FinitePredicate, Self, CorrelativeFinitePredicateSeries, FiniteConcord),
            (SecondaryVerbPhrase, Self, CorrelativeSecondaryVerbPhraseSeries,
                SecondaryConjunctProperties),
            (NounPhrase, Self, CorrelativeNounPhraseSeries, NounCoordinationAgreement),
            (PrepositionPhrase, Self, CorrelativePrepositionPhraseSeries, ObliquePrepositionConcord),
            (AdverbPhrase, Self, CorrelativeAdverbPhraseSeries, AdverbPermissions),
            (ManaPhrase, Self, CorrelativeManaPhraseSeries),
            (Cardinal, Self, CorrelativeCardinalSeries, CardinalAgreement),
            (Amount, Self, CorrelativeAmountSeries),
            (MeasurePhrase, Self, CorrelativeMeasurePhraseSeries, CoordinatedMeasureKind),
            (KeywordPhrase, Self, CorrelativeKeywordPhraseSeries),
            (QuotedText, Self, CorrelativeQuotedTextSeries),
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
            (AdverbPhrase, Self, AdverbPermissions),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self),
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
            (AdverbPhrase, Self, AdverbPermissions),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self),
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
            (AdverbPhrase, Self, CorrelativeAdverbPhraseSeries, AdverbPermissions),
            (ManaPhrase, Self, CorrelativeManaPhraseSeries),
            (Cardinal, Self, CorrelativeCardinalSeries, CardinalAgreement),
            (Amount, Self, CorrelativeAmountSeries),
            (MeasurePhrase, Self, CorrelativeMeasurePhraseSeries, CoordinatedMeasureKind),
            (KeywordPhrase, Self, CorrelativeKeywordPhraseSeries),
            (QuotedText, Self, CorrelativeQuotedTextSeries),
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
        construction SingularGenitiveNounPhrase: NounPhrase {
            form [possessor: NounPhrase, marker: lexical(Clitic), " ", head: Nominal];
            require possessor.CaseUse = Common;
            require possessor.number = Singular;
            require marker.Function = Genitive;
            require marker.HostEnding = Default;
            use NounPhraseHeadAgreement;
        }

        construction PossessiveNounPhrase: NounPhrase {
            form [possessor: lexical(Pronoun), " ", head: Nominal];
            require possessor.case = Genitive;
            require possessor.NominalLicense = AnyNominal;
            use NounPhraseHeadAgreement;
        }

        construction KeywordComplementPreposition: PrepositionPhrase {
            export ObliqueNumber = None;
            form [head: lexical(Preposition), " ", complement: KeywordPhrase];
            require head.KeywordComplement = Yes;
            use PrepositionHeadPermissions;
        }
        instance SharedCardinalComplement<Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, Cardinal, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, Cardinal, OrdinarySelectedPredicate),
        ] {
            use Properties;
        }

        instance SharedInfinitiveComplement<
            Result, Head: head, Complement: complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, InfinitiveComplement, PredicateHeadAgreement),
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
            form [head: Nominal, " ", modifier: CorrelativeAdjectivePhrase];
            use NominalHeadProperties;
        }

        construction SelectedPrepositionHead: SelectedPrepositionHead {
            form [head: lexical(Preposition)];
            require head.PrepositionComplement = NounPhrase;
            export HeadCoordination = No;
            use PrepositionHeadPermissions;
        }

        construction SharedPrepositionComplement: PrepositionPhrase {
            export ObliqueNumber = None;
            form [head: SelectedPrepositionHead, " ", complement: AccusativePhrase];
            require head.HeadCoordination = Yes;
            use PrepositionHeadPermissions;
        }

    }
}
