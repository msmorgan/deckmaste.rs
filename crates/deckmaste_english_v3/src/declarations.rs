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
        feature CaseUse { Common, Nominative, Accusative }
        feature Targeting { No, Yes }
        feature CoordinationKind { Additive, Alternative, Adversative }
        feature PrepositionComplement { NounPhrase, None }
        feature AdverbialUse { Yes, No }
        feature FrequencyUnit { Yes }
        feature FiniteClauseComplement { Yes }
        feature LocativeUse { No, Yes }
        feature AdjectiveStructure { Simple, Complemented }
        feature RelativeSubordinator { Yes }
        feature ParticipialUse { Ordinary, BarePassive, Mixed }
        feature OvertHead { No, Yes }
        feature ScalarVariable { Yes }
        feature MeasureOperator { Yes }
        feature MeasurePreposition { Yes }
        feature MeasurePosition { Before, After }
        feature MeasureKind { Scalar, Pair }
        feature ComparisonMarker { Equality, Ordering }
        feature KeywordParameterClass { Nullary, Amount, Cost }
        feature LabelKind { AbilityWord }
        feature SymbolUse { Cost }
        feature ManaSymbolUse { Yes }
        feature TypeLineRole { Supertype, CardType, Subtype }
        feature IdentityUse { Name }
        feature AttributiveForm { GerundParticiple }
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
        feature NominalLicense { AnyNominal }
        feature KeywordComplement { Yes }

        category Document();
        category Ability();
        category AbilityContinuation();
        category Paragraph();
        category ParagraphItem();
        category ParagraphContinuation();
        category Sentence();
        category Clause(finiteness);
        category FiniteClause();
        category FinitePredicate(number, person);
        category SecondaryVerbPhrase(form, ParticipialUse, OvertHead);
        category BarePredicate(OvertHead);
        category ParticipialPredicate(OvertHead);
        category PastParticiplePredicate(OvertHead);
        category PredicativeComplement(PredicativeKind);
        category Ellipsis(form);
        category BareComplement();
        category ParticipialComplement();
        category PerfectComplement();
        category Nominal(number, countability, Targeting);
        category NounPremodifier();
        category NounPremodifierSeries();
        category NounPhrase(number, person, CaseUse);
        category NominativePhrase(number, person, CaseUse);
        category AccusativePhrase(number, person, CaseUse);
        category AdjectivePhrase(AdjectiveStructure);
        category Name();
        category NamePredicate();
        category PrepositionPhrase(LocativeUse, AdverbialUse);
        category LocativeComplement();
        category FrequencyPhrase();
        category SubjectRelativeClause(number);
        category ObjectRelativeClause();
        category FiniteObjectGap(number, person);
        category BareObjectGap();
        category Cardinal(number);
        category Amount();
        category MeasurePhrase(MeasureKind);
        category UnsignedScalar();
        category ScalarComponent();
        category SlashPair();
        category EqualityComplement();
        category OrderingComplement();
        category KeywordPhrase();
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
        category ClauseSeries(finiteness);
        category FinitePredicateSeries(number, person);
        category SecondaryPredicateSeries(form, ParticipialUse, OvertHead);
        category NounPhraseSeries(number, person, CaseUse, CoordinationKind);
        category AdverbPhrase(VPFinalAdjunct, ClauseInitialAdjunct);
        category InfinitiveComplement(OvertHead);
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
        category NominalSeries(number, countability, Targeting);
        category AdjectivePhraseSeries(AdjectiveStructure);
        category PrepositionPhraseSeries(LocativeUse, AdverbialUse);
        category AdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct);
        category InfinitiveComplementSeries(OvertHead);
        category FrequencyPhraseSeries();
        category FiniteObjectGapSeries(number, person);
        category BareObjectGapSeries();
        category CoordinatedFiniteClause();
        category FiniteClauseSeries();
        category PredicativeComplementSeries(PredicativeKind);
        category CorrelativeClauseSeries(finiteness, CorrelativeCoordinator, CoordinationKind);
        category CorrelativeFinitePredicateSeries(number, person, CorrelativeCoordinator,
            CoordinationKind);
        category CorrelativeSecondaryVerbPhraseSeries(form, ParticipialUse, OvertHead,
            CorrelativeCoordinator, CoordinationKind);
        category CorrelativeNounPhraseSeries(number, person, CaseUse, CorrelativeCoordinator,
            CoordinationKind);
        category CorrelativePrepositionPhraseSeries(LocativeUse, AdverbialUse,
            CorrelativeCoordinator, CoordinationKind);
        category CorrelativeAdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct,
            CorrelativeCoordinator, CoordinationKind);
        category CorrelativeManaPhraseSeries(CorrelativeCoordinator, CoordinationKind);
        category CorrelativeCardinalSeries(number, CorrelativeCoordinator, CoordinationKind);
        category CorrelativeAmountSeries(CorrelativeCoordinator, CoordinationKind);
        category CorrelativeMeasurePhraseSeries(MeasureKind, CorrelativeCoordinator,
            CoordinationKind);
        category CorrelativeKeywordPhraseSeries(CorrelativeCoordinator, CoordinationKind);
        category CorrelativeQuotedTextSeries(CorrelativeCoordinator, CoordinationKind);
        category CorrelativeInfinitiveComplementSeries(OvertHead, CorrelativeCoordinator,
            CoordinationKind);
        category CorrelativeFiniteSelectedHeadSeries(number, person, FrameUse,
            CorrelativeCoordinator, CoordinationKind);
        category CorrelativeSecondarySelectedHeadSeries(form, FrameUse, CorrelativeCoordinator,
            CoordinationKind);
        category CorrelativeAdjectivePhrase();
        category CorrelativeAdjectiveSeries(CorrelativeCoordinator);
        category CorrelativeFiniteObjectGapSeries(number, person, CorrelativeCoordinator);
        category CorrelativeBareObjectGapSeries(CorrelativeCoordinator);
        category SelectedPrepositionHead(LocativeUse, AdverbialUse, HeadCoordination);
        category SelectedPrepositionHeadSeries(LocativeUse, AdverbialUse);
        category CorrelativeFiniteClauseSeries(CorrelativeCoordinator);

        frame Intransitive = "(kind: \"Predicate\", items: [])";
        frame Transitive = "(kind: \"Predicate\", items: [Argument((relation: Object, category:\
        \"NounPhrase\"))])";
        frame ManaComplement = "(kind: \"Predicate\", items: [Argument((relation: Complement,\
        category: \"ManaPhrase\"))])";
        frame ObjectName = "(kind: \"Predicate\", items: [Argument((relation: Object, category:\
        \"NounPhrase\")), Argument((relation: Complement, category: \"Name\"))])";
        frame BareNominal = "(kind: \"Nominal\", items: [])";
        frame NominalSymbols = "(kind: \"Nominal\", items: [Marked(vocabulary: \"Preposition\",\
        member: \"Of\", slot: (relation: Complement, category: \"CostSymbols\"))])";
        frame Predicative = "(kind: \"Predicate\", items: [Argument((relation: Complement,\
        category: \"PredicativeComplement\"))])";
        frame Locative = "(kind: \"Predicate\", items: [Argument((relation: Complement, category:\
        \"LocativeComplement\"))])";
        frame BareAuxiliary = "(kind: \"Auxiliary\", items: [Argument((relation: Complement,\
        category: \"BarePredicate\"))])";
        frame ParticipialAuxiliary = "(kind: \"Auxiliary\", items: [Argument((relation:\
        Complement, category: \"ParticipialPredicate\"))])";
        frame PerfectAuxiliary = "(kind: \"Auxiliary\", items: [Argument((relation: Complement,\
        category: \"PastParticiplePredicate\"))])";
        frame Measure = "(kind: \"Predicate\", items: [Argument((relation: Complement, category:\
        \"MeasurePhrase\"))])";
        frame SlashMeasure = "(kind: \"Predicate\", items: [Argument((relation: Complement,\
        category: \"PowerToughnessAdjustment\"))])";
        frame Equality = "(kind: \"Predicate\", items: [Marked(vocabulary: \"Preposition\",\
        member: \"To\", slot: (relation: Complement, category: \"MeasurePhrase\"))])";
        frame Ordering = "(kind: \"Predicate\", items: [Marked(vocabulary: \"Preposition\",\
        member: \"Than\", slot: (relation: Complement, category: \"MeasurePhrase\"))])";
        frame ObjectEquality = "(kind: \"Predicate\", items: [Argument((relation: Object,\
        category: \"NounPhrase\")), Argument((relation: Complement, category:\
        \"ScalarEquality\"))])";
        frame KeywordObject = "(kind: \"Predicate\", items: [Argument((relation: Object, category:\
        \"KeywordPhrase\"))])";
        frame QuotedObject = "(kind: \"Predicate\", items: [Argument((relation: Object, category:\
        \"QuotedText\"))])";
        frame AmountComplement = "(kind: \"Predicate\", items: [Argument((relation: Complement,\
        category: \"Amount\"))])";
        frame CardinalComplement = "(kind: \"Predicate\", items: [Argument((relation: Complement,\
        category: \"Cardinal\"))])";
        frame InfinitiveSelection = "(kind: \"Predicate\", items: [Argument((relation: Complement,\
        category: \"InfinitiveComplement\"))])";

        // Plain form is shared by finite imperatives and nonfinite infinitivals.
        table secondary_form(form) -> form {
            (Plain) => Plain,
            (GerundParticiple) => GerundParticiple,
            (PastParticiple) => PastParticiple,
        }

        table additive_person(person, person) -> person {
            (First, First) => First,
            (First, Second) => First,
            (First, Third) => First,
            (Second, First) => First,
            (Second, Second) => Second,
            (Second, Third) => Second,
            (Third, First) => First,
            (Third, Second) => Second,
            (Third, Third) => Third,
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

        table overt_predicates(OvertHead, OvertHead) -> OvertHead {
            (No, No) => No,
            (No, Yes) => No,
            (Yes, No) => No,
            (Yes, Yes) => Yes,
        }

        table determined_number(DeterminerUse, number, countability) -> number {
            (SingularCount, Singular, Count) => Singular,
            (Unrestricted, Singular, Count) => Singular,
            (Unrestricted, Plural, Count) => Plural,
            (Unrestricted, Singular, Mass) => Singular,
            (PluralOrMass, Plural, Count) => Plural,
            (PluralOrMass, Singular, Mass) => Singular,
            (PluralCount, Plural, Count) => Plural,
            (Mass, Singular, Mass) => Singular,
            (Singular, Singular, Count) => Singular,
            (Singular, Singular, Mass) => Singular,
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

        policy SecondaryConjunctProperties {
            agree left.form = right.form;
            export form = left.form;
            export ParticipialUse = coordinated_participial_use(left.ParticipialUse,
                right.ParticipialUse);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        policy SecondaryListProperties {
            agree left.form = rest.form;
            export form = left.form;
            export ParticipialUse = coordinated_participial_use(left.ParticipialUse,
                rest.ParticipialUse);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        policy NounCoordinationAgreement {
            export number = coordinate_number(coordinator.CoordinationKind, left.number,
                right.number);
            export person = coordinate_person(coordinator.CoordinationKind, left.person,
                right.person);
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
        }

        policy NounListAgreement {
            export number = coordinate_number(rest.CoordinationKind, left.number, rest.number);
            export person = coordinate_person(rest.CoordinationKind, left.person, rest.person);
            export CaseUse = common_case(left.CaseUse, rest.CaseUse);
        }

        policy CoordinatedMeasureKind {
            agree left.MeasureKind = right.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        policy SerialMeasureKind {
            agree left.MeasureKind = rest.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        policy CoordinatedFiniteness {
            agree left.finiteness = right.finiteness;
            export finiteness = left.finiteness;
        }

        policy SerialFiniteness {
            agree left.finiteness = rest.finiteness;
            export finiteness = left.finiteness;
        }

        policy FiniteHeadAgreement {
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
        }

        policy FiniteConcord {
            agree left.number = right.number;
            agree left.person = right.person;
            export number = left.number;
            export person = left.person;
        }

        policy FiniteListConcord {
            agree left.number = rest.number;
            agree left.person = rest.person;
            export number = left.number;
            export person = left.person;
        }

        policy SecondaryHeadForm {
            export form = secondary_form(head.form);
            export OvertHead = Yes;
        }

        policy SecondaryConcord {
            agree left.form = right.form;
            export form = left.form;
        }

        policy SecondaryListConcord {
            agree left.form = rest.form;
            export form = left.form;
        }

        policy NominalHeadProperties {
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
        }

        policy SelectedFrameConcord {
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
        }

        policy SelectedFrameListConcord {
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
        }

        policy PrepositionPermissions {
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, right.AdverbialUse);
            export LocativeUse = coordinated_locative_use(left.LocativeUse, right.LocativeUse);
        }

        policy PrepositionListPermissions {
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, rest.AdverbialUse);
            export LocativeUse = coordinated_locative_use(left.LocativeUse, rest.LocativeUse);
        }

        policy AdverbPermissions {
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct,
                right.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct,
                right.ClauseInitialAdjunct);
        }

        policy AdverbListPermissions {
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct,
                rest.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct,
                rest.ClauseInitialAdjunct);
        }

        policy PredicateHeadAgreement {
            export number = head.number;
            export person = head.person;
        }

        policy NoFeatures {
        }

        policy FinalCoordinatorKind {
            export CoordinationKind = coordinator.CoordinationKind;
        }

        policy SerialCoordinatorKind {
            export CoordinationKind = rest.CoordinationKind;
        }

        policy NominativeCase {
            export CaseUse = nominative_case(head.CaseUse);
        }

        policy AccusativeCase {
            export CaseUse = accusative_case(head.CaseUse);
        }

        policy SecondaryProjection {
            export OvertHead = head.OvertHead;
            export ParticipialUse = head.ParticipialUse;
            export form = secondary_form(head.form);
        }

        policy SecondaryAdjunctProjection {
            export OvertHead = head.OvertHead;
            export ParticipialUse = head.ParticipialUse;
            export form = head.form;
        }

        policy SelectedSecondaryForm {
            export form = secondary_form(head.form);
        }

        policy CardinalAgreement {
            export number = cardinal_coordinate_number(coordinator.CoordinationKind, left.number,
                right.number);
        }

        policy CardinalListEnd {
            export CoordinationKind = coordinator.CoordinationKind;
            export number = cardinal_coordinate_number(coordinator.CoordinationKind, left.number,
                right.number);
        }

        policy CardinalListTail {
            export CoordinationKind = rest.CoordinationKind;
            export number = cardinal_coordinate_number(rest.CoordinationKind, left.number,
                rest.number);
        }

        policy CardinalListAgreement {
            export number = cardinal_coordinate_number(rest.CoordinationKind, left.number,
                rest.number);
        }

        policy SharedHeadStatus {
            export HeadCoordination = Yes;
        }

        policy NominalConcord {
            agree left.Targeting = right.Targeting;
            agree left.countability = right.countability;
            agree left.number = right.number;
            export Targeting = left.Targeting;
            export countability = left.countability;
            export number = left.number;
        }

        policy NominalListConcord {
            agree left.Targeting = rest.Targeting;
            agree left.countability = rest.countability;
            agree left.number = rest.number;
            export Targeting = left.Targeting;
            export countability = left.countability;
            export number = left.number;
        }

        policy AdjectiveStructureMerge {
            export AdjectiveStructure = coordinated_adjective_structure(left.AdjectiveStructure,
                right.AdjectiveStructure);
        }

        policy AdjectiveListStructure {
            export AdjectiveStructure = coordinated_adjective_structure(left.AdjectiveStructure,
                rest.AdjectiveStructure);
        }

        policy OvertConjunctHeads {
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        policy OvertListHeads {
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        policy UnlikePredicatives {
            export PredicativeKind = unlike_predicative_kind(left.PredicativeKind,
                right.PredicativeKind);
        }

        policy PredicativeListEnd {
            export PredicativeKind = combined_predicative_kind(left.PredicativeKind,
                right.PredicativeKind);
        }

        policy PredicativeListTail {
            export PredicativeKind = combined_predicative_kind(left.PredicativeKind,
                rest.PredicativeKind);
        }

        policy PredicativeListKind {
            export PredicativeKind = unlike_predicative_kind(left.PredicativeKind,
                rest.PredicativeKind);
        }

        policy OvertListEnd {
            export CoordinationKind = coordinator.CoordinationKind;
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        policy OvertCorrelativeListTail {
            export CoordinationKind = rest.CoordinationKind;
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        policy SharedFrameListConcord {
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        policy SharedFrameConcord {
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        policy OrdinarySecondaryHead {
            export form = secondary_form(head.form);
            export OvertHead = Yes;
            export ParticipialUse = Ordinary;
        }

        policy OrdinarySelectedPredicate {
            export form = head.form;
            export OvertHead = Yes;
            export ParticipialUse = Ordinary;
        }

        policy NounPhraseHeadAgreement {
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
        }

        policy PrepositionHeadPermissions {
            export AdverbialUse = head.AdverbialUse;
            export LocativeUse = head.LocativeUse;
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

        schema IntransitivePredicate {
            form [head: lexical(Verb)];
            require head.frame = Intransitive;
        }

        schema TransitivePredicate {
            form [head: lexical(Verb), " ", object: node];
            require head.frame = Transitive;
        }

        schema ManaComplementPredicate {
            form [head: lexical(Verb), " ", complement: node];
            require head.frame = ManaComplement;
        }

        schema LocativePredicate {
            form [head: lexical(Verb), " ", complement: node];
            require head.frame = Locative;
        }

        schema PredicativePredicate {
            form [head: lexical(Verb), " ", complement: node];
            require head.frame = Predicative;
        }

        schema BareAuxiliaryPredicate {
            form [head: lexical(Verb), complement: node];
            require head.frame = BareAuxiliary;
        }

        schema ParticipialAuxiliaryPredicate {
            form [head: lexical(Verb), complement: node];
            require head.frame = ParticipialAuxiliary;
        }

        schema PerfectAuxiliaryPredicate {
            form [head: lexical(Verb), complement: node];
            require head.frame = PerfectAuxiliary;
        }

        schema FrequencyPredicate {
            form [head: node, " ", modifier: node];
        }

        schema PrepositionPredicate {
            form [head: node, " ", modifier: node];
            require modifier.AdverbialUse = Yes;
        }

        schema OvertComplement {
            form [" ", predicate: node];
        }

        schema SlashMeasurePredicate {
            form [head: lexical(Verb), " ", measure: node];
            require head.frame = SlashMeasure;
        }

        schema MeasurePredicate {
            form [head: lexical(Verb), " ", measure: node];
            require head.frame = Measure;
            require measure.MeasureKind = Scalar;
        }

        schema ObjectEqualityPredicate {
            form [head: lexical(Verb), " ", object: node, " ", complement: node];
            require head.frame = ObjectEquality;
        }

        schema KeywordObjectPredicate {
            form [head: lexical(Verb), " ", object: node];
            require head.frame = KeywordObject;
        }

        schema QuotedObjectPredicate {
            form [head: lexical(Verb), " ", object: node];
            require head.frame = QuotedObject;
        }

        schema AmountPredicate {
            form [head: lexical(Verb), " ", amount: node];
            require head.frame = AmountComplement;
        }

        schema AdverbPredicate {
            form [head: node, " ", modifier: node];
            require modifier.VPFinalAdjunct = Yes;
        }

        schema InfinitivePredicate {
            form [head: lexical(Verb), " ", complement: node];
            require head.frame = InfinitiveSelection;
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
            form [cost: Cost, ": ", body: Paragraph];
        }

        construction AbilityWordHead: Ability {
            boundary Initial;
            form [head: lexical(Keyword), " — ", body: Ability];
            require head.LabelKind = AbilityWord;
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
            require action.OvertHead = Yes;
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
            export finiteness = Finite;
        }

        construction Imperative: Clause {
            form [predicate: BarePredicate];
            require predicate.OvertHead = Yes;
            export finiteness = Finite;
        }

        construction FiniteClause: FiniteClause {
            form [subject: NominativePhrase, " ", predicate: FinitePredicate];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
        }

        construction InitialPreposition: Clause {
            form [dependent: PrepositionPhrase, ", ", clause: Clause];
            require dependent.AdverbialUse = Yes;
            export finiteness = clause.finiteness;
        }

        construction ClausalPreposition: Clause {
            form [clause: Clause, " ", dependent: PrepositionPhrase];
            require dependent.AdverbialUse = Yes;
            export finiteness = clause.finiteness;
        }

        construction ClauseCoordination: Clause {
            form [left: Clause, ", ", coordinator: lexical(Coordinator), " ", right: Clause];
            form [left: Clause, " ", coordinator: lexical(Coordinator), " ", right: Clause];
            use CoordinatedFiniteness;
            require coordinator.NoncorrelativeCoordination = Yes;
        }
        instance CoordinationSeriesEnd<Result, Member, Agreement = NoFeatures,
            Properties = NoFeatures>: [
            (ClauseSeries, Clause, CoordinatedFiniteness),
            (FinitePredicateSeries, FinitePredicate, FiniteConcord),
            (SecondaryPredicateSeries, SecondaryVerbPhrase, SecondaryConjunctProperties),
            (NounPhraseSeries, NounPhrase, NounCoordinationAgreement, FinalCoordinatorKind),
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
            (NominalSeries, Nominal, NominalConcord),
            (AdjectivePhraseSeries, AdjectivePhrase, AdjectiveStructureMerge),
            (PrepositionPhraseSeries, PrepositionPhrase, PrepositionPermissions),
            (AdverbPhraseSeries, AdverbPhrase, AdverbPermissions),
            (InfinitiveComplementSeries, InfinitiveComplement, OvertConjunctHeads),
            (FrequencyPhraseSeries, FrequencyPhrase),
            (FiniteObjectGapSeries, FiniteObjectGap, FiniteConcord),
            (BareObjectGapSeries, BareObjectGap),
            (FiniteClauseSeries, FiniteClause),
            (PredicativeComplementSeries, PredicativeComplement, PredicativeListEnd),
            (SelectedPrepositionHeadSeries, SelectedPrepositionHead, PrepositionPermissions),
        ] {
            bind left, right = Member;
            use Agreement;
            use Properties;
        }
        instance CoordinationSeriesContinuation<Result, Member, Tail, Agreement = NoFeatures,
            Properties = NoFeatures>: [
            (ClauseSeries, Clause, Self, SerialFiniteness),
            (FinitePredicateSeries, FinitePredicate, Self, FiniteListConcord),
            (SecondaryPredicateSeries, SecondaryVerbPhrase, Self, SecondaryListProperties),
            (NounPhraseSeries, NounPhrase, Self, NounListAgreement, SerialCoordinatorKind),
            (NounPremodifierSeries, NounPremodifier, Self),
            (ManaPhraseSeries, ManaPhrase, Self),
            (CardinalSeries, Cardinal, Self, CardinalListTail),
            (AmountSeries, Amount, Self),
            (MeasurePhraseSeries, MeasurePhrase, Self, SerialMeasureKind),
            (KeywordPhraseSeries, KeywordPhrase, Self),
            (QuotedTextSeries, QuotedText, Self),
            (FiniteSelectedHeadSeries, FiniteSelectedHead, Self, FiniteListConcord,
                SelectedFrameListConcord),
            (SecondarySelectedHeadSeries, SecondarySelectedHead, Self, SecondaryListConcord,
                SelectedFrameListConcord),
            (NominalSeries, Nominal, Self, NominalListConcord),
            (AdjectivePhraseSeries, AdjectivePhrase, Self, AdjectiveListStructure),
            (PrepositionPhraseSeries, PrepositionPhrase, Self, PrepositionListPermissions),
            (AdverbPhraseSeries, AdverbPhrase, Self, AdverbListPermissions),
            (InfinitiveComplementSeries, InfinitiveComplement, Self, OvertListHeads),
            (FrequencyPhraseSeries, FrequencyPhrase, Self),
            (FiniteObjectGapSeries, FiniteObjectGap, Self, FiniteListConcord),
            (BareObjectGapSeries, BareObjectGap, Self),
            (FiniteClauseSeries, FiniteClause, Self),
            (PredicativeComplementSeries, PredicativeComplement, Self, PredicativeListTail),
            (SelectedPrepositionHeadSeries, SelectedPrepositionHead, Self,
                PrepositionListPermissions),
        ] {
            bind left = Member;
            bind rest = Tail;
            use Agreement;
            use Properties;
        }
        instance SerialCoordination<Result, Member, Tail, Agreement = NoFeatures,
            Properties = NoFeatures>: [
            (Clause, Self, ClauseSeries, SerialFiniteness),
            (FinitePredicate, Self, FinitePredicateSeries, FiniteListConcord),
            (SecondaryVerbPhrase, Self, SecondaryPredicateSeries, SecondaryListProperties),
            (NounPhrase, Self, NounPhraseSeries, NounListAgreement),
            (NounPremodifier, Self, NounPremodifierSeries),
            (ManaPhrase, Self, ManaPhraseSeries),
            (Cardinal, Self, CardinalSeries, CardinalListAgreement),
            (Amount, Self, AmountSeries),
            (MeasurePhrase, Self, MeasurePhraseSeries, SerialMeasureKind),
            (KeywordPhrase, Self, KeywordPhraseSeries),
            (QuotedText, Self, QuotedTextSeries),
            (FiniteSelectedHead, Self, FiniteSelectedHeadSeries, FiniteListConcord,
                SharedFrameListConcord),
            (SecondarySelectedHead, Self, SecondarySelectedHeadSeries, SecondaryListConcord,
                SharedFrameListConcord),
            (Nominal, Self, NominalSeries, NominalListConcord),
            (AdjectivePhrase, Self, AdjectivePhraseSeries, AdjectiveListStructure),
            (PrepositionPhrase, Self, PrepositionPhraseSeries, PrepositionListPermissions),
            (AdverbPhrase, Self, AdverbPhraseSeries, AdverbListPermissions),
            (InfinitiveComplement, Self, InfinitiveComplementSeries, OvertListHeads),
            (FrequencyPhrase, Self, FrequencyPhraseSeries),
            (FiniteObjectGap, Self, FiniteObjectGapSeries, FiniteListConcord),
            (BareObjectGap, Self, BareObjectGapSeries),
            (CoordinatedFiniteClause, FiniteClause, FiniteClauseSeries),
            (PredicativeComplement, Self, PredicativeComplementSeries, PredicativeListKind),
            (SelectedPrepositionHead, Self, SelectedPrepositionHeadSeries,
                PrepositionListPermissions, SharedHeadStatus),
        ] {
            bind left = Member;
            bind rest = Tail;
            use Agreement;
            use Properties;
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
        instance Coordination<Result, Member, Agreement = NoFeatures, Properties = NoFeatures>: [
            (NounPremodifier, Self),
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
            (PrepositionPhrase, Self, PrepositionPermissions),
            (AdverbPhrase, Self, AdverbPermissions),
            (InfinitiveComplement, Self, OvertConjunctHeads),
            (FrequencyPhrase, Self),
            (BareObjectGap, Self),
            (CoordinatedFiniteClause, FiniteClause),
            (PredicativeComplement, Self, UnlikePredicatives),
            (SelectedPrepositionHead, Self, PrepositionPermissions, SharedHeadStatus),
        ] {
            bind left, right = Member;
            use Agreement;
            use Properties;
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

        construction ParticipialPremodifier: Nominal {
            form [modifier: lexical(Verb), " ", head: Nominal];
            use NominalHeadProperties;
            require modifier.form = GerundParticiple;
            require modifier.AttributiveForm = GerundParticiple;
            require modifier.frame = Intransitive;
            require head.Targeting = No;
        }

        construction CatalogName: Name {
            form [head: lexical(Catalog)];
            require head.IdentityUse = Name;
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
        }

        construction PostmodifiedNominal: Nominal {
            form [head: Nominal, " ", modifier: PrepositionPhrase];
            use NominalHeadProperties;
        }

        construction DeterminedNounPhrase: NounPhrase {
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Ordinary;
            export number = determined_number(determiner.DeterminerUse, head.number,
                head.countability);
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
            use NounPhraseHeadAgreement;
        }

        construction NominativePronoun: NounPhrase {
            form [head: lexical(Pronoun)];
            use PredicateHeadAgreement;
            require head.case = Nominative;
            export CaseUse = Nominative;
        }
        instance CasePhrase<Result, Head, Properties>: [
            (NominativePhrase, NounPhrase, NominativeCase),
            (AccusativePhrase, NounPhrase, AccusativeCase),
        ] {
            bind head = Head;
            use Properties;
        }

        construction AccusativePronoun: NounPhrase {
            form [head: lexical(Pronoun)];
            use PredicateHeadAgreement;
            require head.case = Accusative;
            export CaseUse = Accusative;
        }

        construction PrepositionPhrase: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: AccusativePhrase];
            require head.PrepositionComplement = NounPhrase;
            use PrepositionHeadPermissions;
        }

        construction IntransitivePreposition: PrepositionPhrase {
            form [head: lexical(Preposition)];
            require head.PrepositionComplement = None;
            use PrepositionHeadPermissions;
        }

        construction ClauseComplementPreposition: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: FiniteClause];
            require head.FiniteClauseComplement = Yes;
            export LocativeUse = No;
            export AdverbialUse = Yes;
        }

        construction LocativeComplement: LocativeComplement {
            form [phrase: PrepositionPhrase];
            require phrase.LocativeUse = Yes;
        }

        construction AdjectivalComplement: PredicativeComplement {
            form [phrase: AdjectivePhrase];
            export PredicativeKind = Adjectival;
        }

        construction NominalComplement: PredicativeComplement {
            form [phrase: AccusativePhrase];
            export PredicativeKind = Nominal;
        }
        instance IntransitivePredicate<Result, Properties>: [
            (FinitePredicate, FiniteHeadAgreement),
            (SecondaryVerbPhrase, OrdinarySecondaryHead),
        ] {
            use Properties;
        }
        instance TransitivePredicate<Result, Object, Properties>: [
            (FinitePredicate, AccusativePhrase, FiniteHeadAgreement),
            (SecondaryVerbPhrase, AccusativePhrase, OrdinarySecondaryHead),
        ] {
            bind object = Object;
            use Properties;
        }
        instance ManaComplementPredicate<Result, Complement, Properties>: [
            (FinitePredicate, ManaPhrase, FiniteHeadAgreement),
            (SecondaryVerbPhrase, ManaPhrase, OrdinarySecondaryHead),
        ] {
            bind complement = Complement;
            use Properties;
        }
        instance LocativePredicate<Result, Complement, Properties>: [
            (FinitePredicate, LocativeComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, LocativeComplement, OrdinarySecondaryHead),
        ] {
            bind complement = Complement;
            use Properties;
        }
        instance PredicativePredicate<Result, Complement, Properties>: [
            (FinitePredicate, PredicativeComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, PredicativeComplement, OrdinarySecondaryHead),
        ] {
            bind complement = Complement;
            use Properties;
        }
        instance BareAuxiliaryPredicate<Result, Complement, Properties>: [
            (FinitePredicate, BareComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, BareComplement, OrdinarySecondaryHead),
        ] {
            bind complement = Complement;
            use Properties;
        }
        instance ParticipialAuxiliaryPredicate<Result, Complement, Properties>: [
            (FinitePredicate, ParticipialComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, ParticipialComplement, OrdinarySecondaryHead),
        ] {
            bind complement = Complement;
            use Properties;
        }
        instance PerfectAuxiliaryPredicate<Result, Complement, Properties>: [
            (FinitePredicate, PerfectComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, PerfectComplement, OrdinarySecondaryHead),
        ] {
            bind complement = Complement;
            use Properties;
        }

        construction CountedFrequency: FrequencyPhrase {
            form [quantity: Cardinal, " ", head: lexical(Noun)];
            require head.FrequencyUnit = Yes;
            require head.countability = Count;
            agree quantity.number = head.number;
        }
        instance FrequencyPredicate<Result, Head, Modifier, Properties>: [
            (FinitePredicate, Self, FrequencyPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, Self, FrequencyPhrase, SecondaryProjection),
        ] {
            bind head = Head;
            bind modifier = Modifier;
            use Properties;
        }

        instance PrepositionPredicate<Result, Head, Modifier, Properties>: [
            (FinitePredicate, Self, PrepositionPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, Self, PrepositionPhrase, SecondaryProjection),
        ] {
            bind head = Head;
            bind modifier = Modifier;
            use Properties;
        }

        construction BarePredicate: BarePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = Plain;
            export OvertHead = head.OvertHead;
        }

        construction ProgressiveComplement: ParticipialPredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = GerundParticiple;
            export OvertHead = head.OvertHead;
        }

        construction PassiveComplement: ParticipialPredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            require head.ParticipialUse = BarePassive;
            export OvertHead = head.OvertHead;
        }

        construction PerfectComplement: PastParticiplePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            require head.ParticipialUse = Ordinary;
            export OvertHead = head.OvertHead;
        }

        construction PassivePredicate: SecondaryVerbPhrase {
            form [head: lexical(Verb)];
            require head.form = PastParticiple;
            require head.frame = Transitive;
            export form = PastParticiple;
            export ParticipialUse = BarePassive;
            export OvertHead = Yes;
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
        instance OvertComplement<Result, Predicate>: [
            (BareComplement, BarePredicate),
            (ParticipialComplement, ParticipialPredicate),
            (PerfectComplement, PastParticiplePredicate),
        ] {
            bind predicate = Predicate;
        }

        construction BareEllipsis: BareComplement {
            form [omission: Ellipsis];
            require omission.form = Plain;
        }

        construction ProgressiveEllipsis: ParticipialComplement {
            form [omission: Ellipsis];
            require omission.form = GerundParticiple;
        }

        construction PassiveEllipsis: ParticipialComplement {
            form [omission: Ellipsis];
            require omission.form = PastParticiple;
        }

        construction PerfectEllipsis: PerfectComplement {
            form [omission: Ellipsis];
            require omission.form = PastParticiple;
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

        construction CountedNounPhrase: NounPhrase {
            form [quantity: Cardinal, " ", head: Nominal];
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

        construction SlashModifiedNominal: Nominal {
            form [modifier: SlashPair, " ", head: Nominal];
            use NominalHeadProperties;
            require head.Targeting = No;
            require head.countability = Count;
        }
        instance SlashMeasurePredicate<Result, Measure, Properties>: [
            (FinitePredicate, SlashPair, FiniteHeadAgreement),
            (SecondaryVerbPhrase, SlashPair, OrdinarySecondaryHead),
        ] {
            bind measure = Measure;
            use Properties;
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

        construction MeasuredPreposition: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: MeasurePhrase];
            require head.MeasurePreposition = Yes;
            require complement.MeasureKind = Scalar;
            export LocativeUse = No;
            export AdverbialUse = Yes;
        }

        construction MeasuredNounPhrase: NounPhrase {
            form [quantity: MeasurePhrase, " ", head: lexical(Noun)];
            require head.MeasurePosition = Before;
            require quantity.MeasureKind = Scalar;
            require head.number = Singular;
            require head.countability = Mass;
            export number = Singular;
            use ThirdPersonCommonCase;
        }

        construction MeasuredAttribute: NounPhrase {
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
        instance MeasurePredicate<Result, Measure, Properties>: [
            (FinitePredicate, MeasurePhrase, FiniteHeadAgreement),
            (SecondaryVerbPhrase, MeasurePhrase, OrdinarySecondaryHead),
        ] {
            bind measure = Measure;
            use Properties;
        }
        instance ObjectEqualityPredicate<Result, Object, Complement, Properties>: [
            (FinitePredicate, AccusativePhrase, EqualityComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, AccusativePhrase, EqualityComplement, OrdinarySecondaryHead),
        ] {
            bind object = Object;
            bind complement = Complement;
            use Properties;
        }

        instance KeywordObjectPredicate<Result, Object, Properties>: [
            (FinitePredicate, KeywordPhrase, FiniteHeadAgreement),
            (SecondaryVerbPhrase, KeywordPhrase, OrdinarySecondaryHead),
        ] {
            bind object = Object;
            use Properties;
        }

        instance QuotedObjectPredicate<Result, Object, Properties>: [
            (FinitePredicate, QuotedText, FiniteHeadAgreement),
            (SecondaryVerbPhrase, QuotedText, OrdinarySecondaryHead),
        ] {
            bind object = Object;
            use Properties;
        }

        construction SecondaryCardinal: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", count: Cardinal];
            use SecondaryHeadForm;
            require head.frame = CardinalComplement;
            export ParticipialUse = Ordinary;
        }
        construction CardinalAmount: Amount {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }

        construction ScalarAmount: Amount {
            form [value: UnsignedScalar];
        }
        instance AmountPredicate<Result, Amount, Properties>: [
            (FinitePredicate, Amount, FiniteHeadAgreement),
            (SecondaryVerbPhrase, Amount, OrdinarySecondaryHead),
        ] {
            bind amount = Amount;
            use Properties;
        }

        construction Adverb: AdverbPhrase {
            form [head: lexical(Adverb)];
            export VPFinalAdjunct = head.VPFinalAdjunct;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
        }
        instance AdverbPredicate<Result, Head, Modifier, Properties>: [
            (FinitePredicate, Self, AdverbPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, Self, AdverbPhrase, SecondaryAdjunctProjection),
        ] {
            bind head = Head;
            bind modifier = Modifier;
            use Properties;
        }

        construction InitialAdverb: Clause {
            form [modifier: AdverbPhrase, " ", clause: Clause];
            form [modifier: AdverbPhrase, ", ", clause: Clause];
            require modifier.ClauseInitialAdjunct = Yes;
            export finiteness = clause.finiteness;
        }

        construction ToInfinitive: InfinitiveComplement {
            form [marker: lexical(Subordinator), " ", predicate: BarePredicate];
            require marker.InfinitivalMarker = Yes;
            require predicate.OvertHead = Yes;
            export OvertHead = predicate.OvertHead;
        }
        instance InfinitivePredicate<Result, Complement, Properties>: [
            (FinitePredicate, InfinitiveComplement, FiniteHeadAgreement),
            (SecondaryVerbPhrase, InfinitiveComplement, OrdinarySecondaryHead),
        ] {
            bind complement = Complement;
            use Properties;
        }

        // Shared head primitives: exact selected signatures; no complement or coordination yet.

        instance SelectedVerbHead<Result, Properties>: [
            (FiniteSelectedHead, FiniteHeadAgreement),
            (SecondarySelectedHead, SelectedSecondaryForm),
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

        instance SharedObjectComplement<Result, Head, Object, Properties>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase,
                OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind object = Object;
            use Properties;
        }
        instance SharedPredicativeComplement<Result, Head, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, PredicativeComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, PredicativeComplement,
                OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind complement = Complement;
            use Properties;
        }
        instance SharedLocativeComplement<Result, Head, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, LocativeComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, LocativeComplement,
                OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind complement = Complement;
            use Properties;
        }
        instance SharedManaComplement<Result, Head, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, ManaPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, ManaPhrase, OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind complement = Complement;
            use Properties;
        }
        instance SharedAmountComplement<Result, Head, Amount, Properties>: [
            (FinitePredicate, FiniteSelectedHead, Amount, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, Amount, OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind amount = Amount;
            use Properties;
        }
        instance SharedMeasureComplement<Result, Head, Measure, Properties>: [
            (FinitePredicate, FiniteSelectedHead, MeasurePhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, MeasurePhrase, OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind measure = Measure;
            use Properties;
        }
        instance SharedSlashMeasureComplement<Result, Head, Measure, Properties>: [
            (FinitePredicate, FiniteSelectedHead, SlashPair, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, SlashPair, OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind measure = Measure;
            use Properties;
        }
        instance SharedKeywordComplement<Result, Head, Object, Properties>: [
            (FinitePredicate, FiniteSelectedHead, KeywordPhrase, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, KeywordPhrase, OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind object = Object;
            use Properties;
        }
        instance SharedQuotedComplement<Result, Head, Object, Properties>: [
            (FinitePredicate, FiniteSelectedHead, QuotedText, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, QuotedText, OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind object = Object;
            use Properties;
        }
        instance SharedAuxiliaryBareComplement<Result, Head, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, BareComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, BareComplement, OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind complement = Complement;
            use Properties;
        }
        instance SharedAuxiliaryParticipleComplement<Result, Head, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, ParticipialComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, ParticipialComplement,
                OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind complement = Complement;
            use Properties;
        }
        instance SharedAuxiliaryPerfectComplement<Result, Head, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, PerfectComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, PerfectComplement,
                OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind complement = Complement;
            use Properties;
        }
        instance SharedObjectNameComplement<Result, Head, Object, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, Name, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase, Name,
                OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind object = Object;
            bind complement = Complement;
            use Properties;
        }
        instance SharedObjectEqualityComplement<Result, Head, Object, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, AccusativePhrase, EqualityComplement,
                PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, AccusativePhrase, EqualityComplement,
                OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind object = Object;
            bind complement = Complement;
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
            form [head: lexical(Preposition), " ", complement: CoordinatedFiniteClause];
            require head.FiniteClauseComplement = Yes;
            export LocativeUse = No;
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

        // Correlative prefixes: Both binary only; main Clause Either only.
        // Nominal/AdjP deferred until pre-head placement is represented in summaries.
        instance EitherCoordination<Result, Member, Agreement = NoFeatures,
            Properties = NoFeatures>: [
            (Clause, Self, CoordinatedFiniteness),
            (FinitePredicate, Self, FiniteConcord),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
            (NounPhrase, Self, NounCoordinationAgreement),
            (PrepositionPhrase, Self, PrepositionPermissions),
            (AdverbPhrase, Self, AdverbPermissions),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self),
            (InfinitiveComplement, Self, OvertConjunctHeads),
            (FiniteSelectedHead, Self, FiniteConcord, SharedFrameConcord),
            (SecondarySelectedHead, Self, SecondaryConcord, SharedFrameConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase),
            (FiniteObjectGap, Self, FiniteConcord),
            (BareObjectGap, Self),
            (CoordinatedFiniteClause, FiniteClause),
        ] {
            bind left, right = Member;
            use Agreement;
            use Properties;
        }

        instance CorrelativeSeriesEnd<Result, Member, Agreement = NoFeatures,
            Properties = NoFeatures, Status = NoFeatures>: [
            (CorrelativeClauseSeries, Clause, CoordinatedFiniteness, FinalCoordinatorKind),
            (CorrelativeFinitePredicateSeries, FinitePredicate, FiniteConcord,
                FinalCoordinatorKind),
            (CorrelativeSecondaryVerbPhraseSeries, SecondaryVerbPhrase, SecondaryConjunctProperties,
                FinalCoordinatorKind),
            (CorrelativeNounPhraseSeries, NounPhrase, NounCoordinationAgreement,
                FinalCoordinatorKind),
            (CorrelativePrepositionPhraseSeries, PrepositionPhrase, PrepositionPermissions,
                FinalCoordinatorKind),
            (CorrelativeAdverbPhraseSeries, AdverbPhrase, AdverbPermissions, FinalCoordinatorKind),
            (CorrelativeManaPhraseSeries, ManaPhrase, FinalCoordinatorKind),
            (CorrelativeCardinalSeries, Cardinal, CardinalListEnd),
            (CorrelativeAmountSeries, Amount, FinalCoordinatorKind),
            (CorrelativeMeasurePhraseSeries, MeasurePhrase, CoordinatedMeasureKind,
                FinalCoordinatorKind),
            (CorrelativeKeywordPhraseSeries, KeywordPhrase, FinalCoordinatorKind),
            (CorrelativeQuotedTextSeries, QuotedText, FinalCoordinatorKind),
            (CorrelativeInfinitiveComplementSeries, InfinitiveComplement, OvertListEnd),
            (CorrelativeFiniteSelectedHeadSeries, FiniteSelectedHead, FiniteConcord,
                SelectedFrameConcord, FinalCoordinatorKind),
            (CorrelativeSecondarySelectedHeadSeries, SecondarySelectedHead, SecondaryConcord,
                SelectedFrameConcord, FinalCoordinatorKind),
            (CorrelativeAdjectiveSeries, AdjectivePhrase),
            (CorrelativeFiniteObjectGapSeries, FiniteObjectGap, FiniteConcord),
            (CorrelativeBareObjectGapSeries, BareObjectGap),
            (CorrelativeFiniteClauseSeries, FiniteClause),
        ] {
            bind left, right = Member;
            use Agreement;
            use Properties;
            use Status;
        }
        instance CorrelativeSeriesContinuation<Result, Member, Tail, Agreement = NoFeatures,
            Properties = NoFeatures, Status = NoFeatures>: [
            (CorrelativeClauseSeries, Clause, Self, SerialFiniteness, SerialCoordinatorKind),
            (CorrelativeFinitePredicateSeries, FinitePredicate, Self, FiniteListConcord,
                SerialCoordinatorKind),
            (CorrelativeSecondaryVerbPhraseSeries, SecondaryVerbPhrase, Self,
                SecondaryListProperties, SerialCoordinatorKind),
            (CorrelativeNounPhraseSeries, NounPhrase, Self, NounListAgreement,
                SerialCoordinatorKind),
            (CorrelativePrepositionPhraseSeries, PrepositionPhrase, Self,
                PrepositionListPermissions, SerialCoordinatorKind),
            (CorrelativeAdverbPhraseSeries, AdverbPhrase, Self, AdverbListPermissions,
                SerialCoordinatorKind),
            (CorrelativeManaPhraseSeries, ManaPhrase, Self, SerialCoordinatorKind),
            (CorrelativeCardinalSeries, Cardinal, Self, CardinalListTail),
            (CorrelativeAmountSeries, Amount, Self, SerialCoordinatorKind),
            (CorrelativeMeasurePhraseSeries, MeasurePhrase, Self, SerialMeasureKind,
                SerialCoordinatorKind),
            (CorrelativeKeywordPhraseSeries, KeywordPhrase, Self, SerialCoordinatorKind),
            (CorrelativeQuotedTextSeries, QuotedText, Self, SerialCoordinatorKind),
            (CorrelativeInfinitiveComplementSeries, InfinitiveComplement, Self,
                OvertCorrelativeListTail),
            (CorrelativeFiniteSelectedHeadSeries, FiniteSelectedHead, Self, FiniteListConcord,
                SelectedFrameListConcord, SerialCoordinatorKind),
            (CorrelativeSecondarySelectedHeadSeries, SecondarySelectedHead, Self,
                SecondaryListConcord, SelectedFrameListConcord, SerialCoordinatorKind),
            (CorrelativeAdjectiveSeries, AdjectivePhrase, Self),
            (CorrelativeFiniteObjectGapSeries, FiniteObjectGap, Self, FiniteListConcord),
            (CorrelativeBareObjectGapSeries, BareObjectGap, Self),
            (CorrelativeFiniteClauseSeries, FiniteClause, Self),
        ] {
            bind left = Member;
            bind rest = Tail;
            use Agreement;
            use Properties;
            use Status;
        }
        instance EitherSerialCoordination<Result, Member, Tail, Agreement = NoFeatures,
            Properties = NoFeatures>: [
            (Clause, Self, CorrelativeClauseSeries, SerialFiniteness),
            (FinitePredicate, Self, CorrelativeFinitePredicateSeries, FiniteListConcord),
            (SecondaryVerbPhrase, Self, CorrelativeSecondaryVerbPhraseSeries,
                SecondaryListProperties),
            (NounPhrase, Self, CorrelativeNounPhraseSeries, NounListAgreement),
            (PrepositionPhrase, Self, CorrelativePrepositionPhraseSeries,
                PrepositionListPermissions),
            (AdverbPhrase, Self, CorrelativeAdverbPhraseSeries, AdverbListPermissions),
            (ManaPhrase, Self, CorrelativeManaPhraseSeries),
            (Cardinal, Self, CorrelativeCardinalSeries, CardinalListAgreement),
            (Amount, Self, CorrelativeAmountSeries),
            (MeasurePhrase, Self, CorrelativeMeasurePhraseSeries, SerialMeasureKind),
            (KeywordPhrase, Self, CorrelativeKeywordPhraseSeries),
            (QuotedText, Self, CorrelativeQuotedTextSeries),
            (InfinitiveComplement, Self, CorrelativeInfinitiveComplementSeries, OvertListHeads),
            (FiniteSelectedHead, Self, CorrelativeFiniteSelectedHeadSeries, FiniteListConcord,
                SharedFrameListConcord),
            (SecondarySelectedHead, Self, CorrelativeSecondarySelectedHeadSeries,
                SecondaryListConcord, SharedFrameListConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase, CorrelativeAdjectiveSeries),
            (FiniteObjectGap, Self, CorrelativeFiniteObjectGapSeries, FiniteListConcord),
            (BareObjectGap, Self, CorrelativeBareObjectGapSeries),
            (CoordinatedFiniteClause, FiniteClause, CorrelativeFiniteClauseSeries),
        ] {
            bind left = Member;
            bind rest = Tail;
            use Agreement;
            use Properties;
        }
        instance BothCoordination<Result, Member, Agreement = NoFeatures,
            Properties = NoFeatures>: [
            (FinitePredicate, Self, FiniteConcord),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
            (NounPhrase, Self, NounCoordinationAgreement),
            (PrepositionPhrase, Self, PrepositionPermissions),
            (AdverbPhrase, Self, AdverbPermissions),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self),
            (InfinitiveComplement, Self, OvertConjunctHeads),
            (FiniteSelectedHead, Self, FiniteConcord, SharedFrameConcord),
            (SecondarySelectedHead, Self, SecondaryConcord, SharedFrameConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase),
            (FiniteObjectGap, Self, FiniteConcord),
            (BareObjectGap, Self),
        ] {
            bind left, right = Member;
            use Agreement;
            use Properties;
        }

        instance NeitherCoordination<Result, Member, Agreement = NoFeatures,
            Properties = NoFeatures>: [
            (FinitePredicate, Self, FiniteConcord),
            (SecondaryVerbPhrase, Self, SecondaryConjunctProperties),
            (NounPhrase, Self, NounCoordinationAgreement),
            (PrepositionPhrase, Self, PrepositionPermissions),
            (AdverbPhrase, Self, AdverbPermissions),
            (ManaPhrase, Self),
            (Cardinal, Self, CardinalAgreement),
            (Amount, Self),
            (MeasurePhrase, Self, CoordinatedMeasureKind),
            (KeywordPhrase, Self),
            (QuotedText, Self),
            (InfinitiveComplement, Self, OvertConjunctHeads),
            (FiniteSelectedHead, Self, FiniteConcord, SharedFrameConcord),
            (SecondarySelectedHead, Self, SecondaryConcord, SharedFrameConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase),
            (FiniteObjectGap, Self, FiniteConcord),
            (BareObjectGap, Self),
        ] {
            bind left, right = Member;
            use Agreement;
            use Properties;
        }

        instance NeitherSerialCoordination<Result, Member, Tail, Agreement = NoFeatures,
            Properties = NoFeatures>: [
            (FinitePredicate, Self, CorrelativeFinitePredicateSeries, FiniteListConcord),
            (SecondaryVerbPhrase, Self, CorrelativeSecondaryVerbPhraseSeries,
                SecondaryListProperties),
            (NounPhrase, Self, CorrelativeNounPhraseSeries, NounListAgreement),
            (PrepositionPhrase, Self, CorrelativePrepositionPhraseSeries,
                PrepositionListPermissions),
            (AdverbPhrase, Self, CorrelativeAdverbPhraseSeries, AdverbListPermissions),
            (ManaPhrase, Self, CorrelativeManaPhraseSeries),
            (Cardinal, Self, CorrelativeCardinalSeries, CardinalListAgreement),
            (Amount, Self, CorrelativeAmountSeries),
            (MeasurePhrase, Self, CorrelativeMeasurePhraseSeries, SerialMeasureKind),
            (KeywordPhrase, Self, CorrelativeKeywordPhraseSeries),
            (QuotedText, Self, CorrelativeQuotedTextSeries),
            (InfinitiveComplement, Self, CorrelativeInfinitiveComplementSeries, OvertListHeads),
            (FiniteSelectedHead, Self, CorrelativeFiniteSelectedHeadSeries, FiniteListConcord,
                SharedFrameListConcord),
            (SecondarySelectedHead, Self, CorrelativeSecondarySelectedHeadSeries,
                SecondaryListConcord, SharedFrameListConcord),
            (CorrelativeAdjectivePhrase, AdjectivePhrase, CorrelativeAdjectiveSeries),
            (FiniteObjectGap, Self, CorrelativeFiniteObjectGapSeries, FiniteListConcord),
            (BareObjectGap, Self, CorrelativeBareObjectGapSeries),
        ] {
            bind left = Member;
            bind rest = Tail;
            use Agreement;
            use Properties;
        }

        construction PossessiveNounPhrase: NounPhrase {
            form [possessor: lexical(Pronoun), " ", head: Nominal];
            require possessor.case = Genitive;
            require possessor.NominalLicense = AnyNominal;
            use NounPhraseHeadAgreement;
        }

        construction KeywordComplementPreposition: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: KeywordPhrase];
            require head.KeywordComplement = Yes;
            use PrepositionHeadPermissions;
        }
        instance SharedCardinalComplement<Result, Head, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, Cardinal, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, Cardinal, OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind complement = Complement;
            use Properties;
        }

        instance SharedInfinitiveComplement<Result, Head, Complement, Properties>: [
            (FinitePredicate, FiniteSelectedHead, InfinitiveComplement, PredicateHeadAgreement),
            (SecondaryVerbPhrase, SecondarySelectedHead, InfinitiveComplement,
                OrdinarySelectedPredicate),
        ] {
            bind head = Head;
            bind complement = Complement;
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
            form [head: SelectedPrepositionHead, " ", complement: AccusativePhrase];
            require head.HeadCoordination = Yes;
            use PrepositionHeadPermissions;
        }

    }
}
