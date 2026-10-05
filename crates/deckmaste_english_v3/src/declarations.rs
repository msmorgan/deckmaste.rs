//! Connected Oracle English declarations under whole-grammar activation.
//!
//! Lexical distribution and complete frame signatures govern composition.
//! The activation ticket records the remaining families and corpus obligations.

use deckmaste_construction_v3::constructions;

constructions! {
    pub mod grammar {
        capitalization Positional;
        feature DeterminerUse { SingularCount, Unrestricted, PluralOrMass, PluralCount, Mass, Singular }
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
        feature Voice { Active, Passive, Mixed }
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

        category Document();
        category Ability();
        category AbilityContinuation();
        category Paragraph();
        category ParagraphItem();
        category ParagraphContinuation();
        category Sentence();
        category Clause(finiteness);
        category FiniteClause();
        category FinitePredicate(number, person, Voice);
        category SecondaryVerbPhrase(form, Voice, OvertHead);
        category BarePredicate(Voice, OvertHead);
        category ParticipialPredicate(Voice, OvertHead);
        category PastParticiplePredicate(Voice, OvertHead);
        category PredicativeComplement(PredicativeKind);
        category Ellipsis(form, Voice);
        category BareComplement(Voice);
        category ParticipialComplement(Voice);
        category PerfectComplement(Voice);
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
        category FinitePredicateSeries(number, person, Voice);
        category SecondaryPredicateSeries(form, Voice, OvertHead);
        category NounPhraseSeries(number, person, CaseUse, CoordinationKind);

        frame Intransitive = "(kind: \"Predicate\", items: [])";
        frame Transitive = "(kind: \"Predicate\", items: [Argument((relation: Object, category: \"NounPhrase\"))])";
        frame ManaComplement = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"ManaPhrase\"))])";
        frame ObjectName = "(kind: \"Predicate\", items: [Argument((relation: Object, category: \"NounPhrase\")), Argument((relation: Complement, category: \"Name\"))])";
        frame BareNominal = "(kind: \"Nominal\", items: [])";
        frame NominalSymbols = "(kind: \"Nominal\", items: [Marked(vocabulary: \"Preposition\", member: \"Of\", slot: (relation: Complement, category: \"CostSymbols\"))])";
        frame Predicative = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"PredicativeComplement\"))])";
        frame Locative = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"LocativeComplement\"))])";
        frame BareAuxiliary = "(kind: \"Auxiliary\", items: [Argument((relation: Complement, category: \"BarePredicate\"))])";
        frame ParticipialAuxiliary = "(kind: \"Auxiliary\", items: [Argument((relation: Complement, category: \"ParticipialPredicate\"))])";
        frame PerfectAuxiliary = "(kind: \"Auxiliary\", items: [Argument((relation: Complement, category: \"PastParticiplePredicate\"))])";

        frame Measure = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"MeasurePhrase\"))])";
        frame SlashMeasure = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"PowerToughnessAdjustment\"))])";
        frame Equality = "(kind: \"Predicate\", items: [Marked(vocabulary: \"Preposition\", member: \"To\", slot: (relation: Complement, category: \"MeasurePhrase\"))])";
        frame Ordering = "(kind: \"Predicate\", items: [Marked(vocabulary: \"Preposition\", member: \"Than\", slot: (relation: Complement, category: \"MeasurePhrase\"))])";
        frame ObjectEquality = "(kind: \"Predicate\", items: [Argument((relation: Object, category: \"NounPhrase\")), Argument((relation: Complement, category: \"ScalarEquality\"))])";
        frame KeywordObject = "(kind: \"Predicate\", items: [Argument((relation: Object, category: \"KeywordPhrase\"))])";
        frame QuotedObject = "(kind: \"Predicate\", items: [Argument((relation: Object, category: \"QuotedText\"))])";
        frame AmountComplement = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"Amount\"))])";
        frame CardinalComplement = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"Cardinal\"))])";

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

        table coordinated_voice(Voice, Voice) -> Voice {
            (Active, Active) => Active,
            (Active, Passive) => Mixed,
            (Active, Mixed) => Mixed,
            (Passive, Active) => Mixed,
            (Passive, Passive) => Passive,
            (Passive, Mixed) => Mixed,
            (Mixed, Active) => Mixed,
            (Mixed, Passive) => Mixed,
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

        construction EmptyDocument: Document {
            form [];
        }

        construction TypeLine: TypeLine {
            form [supertypes: repeat(SupertypePrefix, ""), types: CardTypes];
        }

        construction SubtypedLine: TypeLine {
            form [supertypes: repeat(SupertypePrefix, ""), types: CardTypes, " — ", subtypes: Subtypes];
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
            agree left.finiteness = right.finiteness;
            export finiteness = left.finiteness;
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction ClauseSeriesEnd: ClauseSeries {
            form [left: Clause, ", ", coordinator: lexical(Coordinator), " ", right: Clause];
            agree left.finiteness = right.finiteness;
            export finiteness = left.finiteness;
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction ClauseSeriesContinuation: ClauseSeries {
            form [left: Clause, ", ", rest: ClauseSeries];
            agree left.finiteness = rest.finiteness;
            export finiteness = left.finiteness;
        }

        construction SerialClause: Clause {
            form [left: Clause, ", ", rest: ClauseSeries];
            agree left.finiteness = rest.finiteness;
            export finiteness = left.finiteness;
        }

        construction FinitePredicateSeriesEnd: FinitePredicateSeries {
            form [left: FinitePredicate, ", ", coordinator: lexical(Coordinator), " ", right: FinitePredicate];
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction FinitePredicateSeriesContinuation: FinitePredicateSeries {
            form [left: FinitePredicate, ", ", rest: FinitePredicateSeries];
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
        }

        construction SerialFinitePredicate: FinitePredicate {
            form [left: FinitePredicate, ", ", rest: FinitePredicateSeries];
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
        }

        construction SecondaryPredicateSeriesEnd: SecondaryPredicateSeries {
            form [left: SecondaryVerbPhrase, ", ", coordinator: lexical(Coordinator), " ", right: SecondaryVerbPhrase];
            agree left.form = right.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction SecondaryPredicateSeriesContinuation: SecondaryPredicateSeries {
            form [left: SecondaryVerbPhrase, ", ", rest: SecondaryPredicateSeries];
            agree left.form = rest.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        construction SerialSecondaryPredicate: SecondaryVerbPhrase {
            form [left: SecondaryVerbPhrase, ", ", rest: SecondaryPredicateSeries];
            agree left.form = rest.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        construction NounPhraseSeriesEnd: NounPhraseSeries {
            form [left: NounPhrase, ", ", coordinator: lexical(Coordinator), " ", right: NounPhrase];
            export number = coordinate_number(coordinator.CoordinationKind, left.number, right.number);
            export person = coordinate_person(coordinator.CoordinationKind, left.person, right.person);
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
            export CoordinationKind = coordinator.CoordinationKind;
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction NounPhraseSeriesContinuation: NounPhraseSeries {
            form [left: NounPhrase, ", ", rest: NounPhraseSeries];
            export number = coordinate_number(rest.CoordinationKind, left.number, rest.number);
            export person = coordinate_person(rest.CoordinationKind, left.person, rest.person);
            export CaseUse = common_case(left.CaseUse, rest.CaseUse);
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialNounPhrase: NounPhrase {
            form [left: NounPhrase, ", ", rest: NounPhraseSeries];
            export number = coordinate_number(rest.CoordinationKind, left.number, rest.number);
            export person = coordinate_person(rest.CoordinationKind, left.person, rest.person);
            export CaseUse = common_case(left.CaseUse, rest.CaseUse);

        }

        construction Noun: Nominal {
            form [head: lexical(Noun)];
            require head.framing = Unframed;
            export number = head.number;
            export countability = head.countability;
            export Targeting = No;
        }

        construction BareFramedNoun: Nominal {
            form [head: lexical(Noun)];
            require head.frame = BareNominal;
            export number = head.number;
            export countability = head.countability;
            export Targeting = No;
        }

        construction SymbolComplementNominal: Nominal {
            form [head: lexical(Noun), " ", marker: lexical(Preposition), " ", complement: CostSymbols];
            require head.frame = NominalSymbols;
            require marker.NominalComplementMarker = Of;
            export number = head.number;
            export countability = head.countability;
            export Targeting = No;
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

        construction NounPremodifierCoordination: NounPremodifier {
            form [left: NounPremodifier, " ", coordinator: lexical(Coordinator), " ", right: NounPremodifier];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction NounPremodifierSeriesEnd: NounPremodifierSeries {
            form [left: NounPremodifier, ", ", coordinator: lexical(Coordinator), " ", right: NounPremodifier];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction NounPremodifierSeriesContinuation: NounPremodifierSeries {
            form [left: NounPremodifier, ", ", rest: NounPremodifierSeries];
        }

        construction SerialNounPremodifier: NounPremodifier {
            form [left: NounPremodifier, ", ", rest: NounPremodifierSeries];
        }

        construction NounPremodifiedNominal: Nominal {
            form [modifier: NounPremodifier, " ", head: Nominal];
            require head.Targeting = No;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
        }

        construction PremodifiedNominal: Nominal {
            form [modifier: AdjectivePhrase, " ", head: Nominal];
            require modifier.AdjectiveStructure = Simple;
            require head.Targeting = No;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
        }

        construction PostpositiveNominal: Nominal {
            form [head: Nominal, " ", modifier: AdjectivePhrase];
            require modifier.AdjectiveStructure = Complemented;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
        }

        construction ParticipialPremodifier: Nominal {
            form [modifier: lexical(Verb), " ", head: Nominal];
            require modifier.form = GerundParticiple;
            require modifier.AttributiveForm = GerundParticiple;
            require modifier.frame = Intransitive;
            require head.Targeting = No;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
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
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
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
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
        }

        construction DeterminedNounPhrase: NounPhrase {
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Ordinary;
            export number = determined_number(determiner.DeterminerUse, head.number, head.countability);
            export person = Third;
            export CaseUse = Common;
        }

        construction IndefiniteNounPhrase: NounPhrase {
            form [determiner: lexical(Determinative), " ", head: Nominal];
            require determiner.DeterminerKind = Indefinite;
            require determiner.number = Singular;
            require head.number = Singular;
            require head.countability = Count;
            agree determiner.article_onset = head.onset;
            export number = head.number;
            export person = Third;
            export CaseUse = Common;
        }

        construction BarePlural: NounPhrase {
            form [head: Nominal];
            require head.number = Plural;
            require head.countability = Count;
            export number = head.number;
            export person = Third;
            export CaseUse = Common;
        }

        construction BareMass: NounPhrase {
            form [head: Nominal];
            require head.number = Singular;
            require head.countability = Mass;
            export number = head.number;
            export person = Third;
            export CaseUse = Common;
        }

        construction TargetNounPhrase: NounPhrase {
            form [marker: lexical(Determinative), " ", head: Nominal];
            require marker.Targeting = Yes;
            require head.Targeting = No;
            export number = head.number;
            export person = Third;
            export CaseUse = Common;
        }

        construction NominativePronoun: NounPhrase {
            form [head: lexical(Pronoun)];
            require head.case = Nominative;
            export number = head.number;
            export person = head.person;
            export CaseUse = Nominative;
        }

        construction NominativePhrase: NominativePhrase {
            form [head: NounPhrase];
            export number = head.number;
            export person = head.person;
            export CaseUse = nominative_case(head.CaseUse);
        }

        construction AccusativePronoun: NounPhrase {
            form [head: lexical(Pronoun)];
            require head.case = Accusative;
            export number = head.number;
            export person = head.person;
            export CaseUse = Accusative;
        }

        construction AccusativePhrase: AccusativePhrase {
            form [head: NounPhrase];
            export number = head.number;
            export person = head.person;
            export CaseUse = accusative_case(head.CaseUse);
        }

        construction AdditiveNounPhrase: NounPhrase {
            form [left: NounPhrase, " ", coordinator: lexical(Coordinator), " ", right: NounPhrase];
            require coordinator.CoordinationKind = Additive;
            export number = Plural;
            export person = additive_person(left.person, right.person);
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction AlternativeNounPhrase: NounPhrase {
            form [left: NounPhrase, " ", coordinator: lexical(Coordinator), " ", right: NounPhrase];
            require coordinator.CoordinationKind = Alternative;
            export number = right.number;
            export person = right.person;
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction PrepositionPhrase: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: AccusativePhrase];
            require head.PrepositionComplement = NounPhrase;
            export LocativeUse = head.LocativeUse;
            export AdverbialUse = head.AdverbialUse;
        }

        construction IntransitivePreposition: PrepositionPhrase {
            form [head: lexical(Preposition)];
            require head.PrepositionComplement = None;
            export LocativeUse = head.LocativeUse;
            export AdverbialUse = head.AdverbialUse;
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

        construction FiniteIntransitive: FinitePredicate {
            form [head: lexical(Verb)];
            require head.finiteness = Finite;
            require head.frame = Intransitive;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteTransitive: FinitePredicate {
            form [head: lexical(Verb), " ", object: AccusativePhrase];
            require head.finiteness = Finite;
            require head.frame = Transitive;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteManaComplement: FinitePredicate {
            form [head: lexical(Verb), " ", complement: ManaPhrase];
            require head.finiteness = Finite;
            require head.frame = ManaComplement;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteLocative: FinitePredicate {
            form [head: lexical(Verb), " ", complement: LocativeComplement];
            require head.frame = Locative;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FinitePredicative: FinitePredicate {
            form [head: lexical(Verb), " ", complement: PredicativeComplement];
            require head.finiteness = Finite;
            require head.frame = Predicative;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteBareAuxiliary: FinitePredicate {
            form [head: lexical(Verb), complement: BareComplement];
            require head.finiteness = Finite;
            require head.frame = BareAuxiliary;
            export number = head.number;
            export person = head.person;
            export Voice = complement.Voice;
        }

        construction FiniteParticipialAuxiliary: FinitePredicate {
            form [head: lexical(Verb), complement: ParticipialComplement];
            require head.finiteness = Finite;
            require head.frame = ParticipialAuxiliary;
            export number = head.number;
            export person = head.person;
            export Voice = complement.Voice;
        }

        construction FinitePerfectAuxiliary: FinitePredicate {
            form [head: lexical(Verb), complement: PerfectComplement];
            require head.finiteness = Finite;
            require head.frame = PerfectAuxiliary;
            export number = head.number;
            export person = head.person;
            export Voice = complement.Voice;
        }

        construction CountedFrequency: FrequencyPhrase {
            form [quantity: Cardinal, " ", head: lexical(Noun)];
            require head.FrequencyUnit = Yes;
            require head.countability = Count;
            agree quantity.number = head.number;
        }

        construction FiniteFrequency: FinitePredicate {
            form [head: FinitePredicate, " ", modifier: FrequencyPhrase];
            export number = head.number;
            export person = head.person;
            export Voice = head.Voice;
        }

        construction SecondaryFrequency: SecondaryVerbPhrase {
            form [head: SecondaryVerbPhrase, " ", modifier: FrequencyPhrase];
            export form = secondary_form(head.form);
            export Voice = head.Voice;
            export OvertHead = head.OvertHead;
        }

        construction FinitePreposition: FinitePredicate {
            form [head: FinitePredicate, " ", modifier: PrepositionPhrase];
            require modifier.AdverbialUse = Yes;
            export number = head.number;
            export person = head.person;
            export Voice = head.Voice;
        }

        construction SecondaryIntransitive: SecondaryVerbPhrase {
            form [head: lexical(Verb)];
            require head.frame = Intransitive;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction SecondaryTransitive: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", object: AccusativePhrase];
            require head.frame = Transitive;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction SecondaryManaComplement: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", complement: ManaPhrase];
            require head.frame = ManaComplement;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction SecondaryLocative: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", complement: LocativeComplement];
            require head.frame = Locative;
            export form = secondary_form(head.form);
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondaryPredicative: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", complement: PredicativeComplement];
            require head.frame = Predicative;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction SecondaryBareAuxiliary: SecondaryVerbPhrase {
            form [head: lexical(Verb), complement: BareComplement];
            require head.frame = BareAuxiliary;
            export form = secondary_form(head.form);
            export Voice = complement.Voice;
            export OvertHead = Yes;
        }

        construction SecondaryParticipialAuxiliary: SecondaryVerbPhrase {
            form [head: lexical(Verb), complement: ParticipialComplement];
            require head.frame = ParticipialAuxiliary;
            export form = secondary_form(head.form);
            export Voice = complement.Voice;
            export OvertHead = Yes;
        }

        construction SecondaryPerfectAuxiliary: SecondaryVerbPhrase {
            form [head: lexical(Verb), complement: PerfectComplement];
            require head.frame = PerfectAuxiliary;
            export form = secondary_form(head.form);
            export Voice = complement.Voice;
            export OvertHead = Yes;
        }

        construction SecondaryPreposition: SecondaryVerbPhrase {
            form [head: SecondaryVerbPhrase, " ", modifier: PrepositionPhrase];
            require modifier.AdverbialUse = Yes;
            export form = secondary_form(head.form);
            export Voice = head.Voice;
            export OvertHead = head.OvertHead;
        }

        construction FinitePredicateCoordination: FinitePredicate {
            form [left: FinitePredicate, " ", coordinator: lexical(Coordinator), " ", right: FinitePredicate];
            agree left.number = right.number;
            agree left.person = right.person;
            export number = left.number;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction SecondaryVerbPhraseCoordination: SecondaryVerbPhrase {
            form [left: SecondaryVerbPhrase, " ", coordinator: lexical(Coordinator), " ", right: SecondaryVerbPhrase];
            agree left.form = right.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction BarePredicate: BarePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = Plain;
            export Voice = head.Voice;
            export OvertHead = head.OvertHead;
        }

        construction ProgressiveComplement: ParticipialPredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = GerundParticiple;
            export Voice = head.Voice;
            export OvertHead = head.OvertHead;
        }

        construction PassiveComplement: ParticipialPredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            require head.Voice = Passive;
            export Voice = Passive;
            export OvertHead = head.OvertHead;
        }

        construction PerfectComplement: PastParticiplePredicate {
            form [head: SecondaryVerbPhrase];
            require head.form = PastParticiple;
            export Voice = head.Voice;
            export OvertHead = head.OvertHead;
        }

        construction PassivePredicate: SecondaryVerbPhrase {
            form [head: lexical(Verb)];
            require head.form = PastParticiple;
            require head.frame = Transitive;
            export form = PastParticiple;
            export Voice = Passive;
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
            agree head.number = relative.number;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
        }

        construction FiniteObjectGap: FiniteObjectGap {
            form [head: lexical(Verb)];
            require head.finiteness = Finite;
            require head.frame = Transitive;
            export number = head.number;
            export person = head.person;
        }

        construction BareObjectGap: BareObjectGap {
            form [head: lexical(Verb)];
            require head.form = Plain;
            require head.frame = Transitive;
        }

        construction AuxiliaryObjectGap: FiniteObjectGap {
            form [head: lexical(Verb), " ", complement: BareObjectGap];
            require head.finiteness = Finite;
            require head.frame = BareAuxiliary;
            export number = head.number;
            export person = head.person;
        }

        construction SharedObjectGap: FiniteObjectGap {
            form [left: FiniteObjectGap, " ", coordinator: lexical(Coordinator), " ", right: FiniteObjectGap];
            agree left.number = right.number;
            agree left.person = right.person;
            export number = left.number;
            export person = left.person;
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction ObjectRelativeClause: ObjectRelativeClause {
            form [marker: lexical(Subordinator), " ", subject: NominativePhrase, " ", predicate: FiniteObjectGap];
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
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
        }

        construction OmittedPlainActive: Ellipsis {
            form [];
            export form = Plain;
            export Voice = Active;
        }

        construction OmittedPlainPassive: Ellipsis {
            form [];
            export form = Plain;
            export Voice = Passive;
        }

        construction OmittedGerundParticipleActive: Ellipsis {
            form [];
            export form = GerundParticiple;
            export Voice = Active;
        }

        construction OmittedGerundParticiplePassive: Ellipsis {
            form [];
            export form = GerundParticiple;
            export Voice = Passive;
        }

        construction OmittedPastParticipleActive: Ellipsis {
            form [];
            export form = PastParticiple;
            export Voice = Active;
        }

        construction OmittedPastParticiplePassive: Ellipsis {
            form [];
            export form = PastParticiple;
            export Voice = Passive;
        }

        construction OvertBareComplement: BareComplement {
            form [" ", predicate: BarePredicate];
            export Voice = predicate.Voice;
        }

        construction OvertParticipialComplement: ParticipialComplement {
            form [" ", predicate: ParticipialPredicate];
            export Voice = predicate.Voice;
        }

        construction OvertPerfectComplement: PerfectComplement {
            form [" ", predicate: PastParticiplePredicate];
            export Voice = predicate.Voice;
        }

        construction BareEllipsis: BareComplement {
            form [omission: Ellipsis];
            require omission.form = Plain;
            export Voice = omission.Voice;
        }

        construction ProgressiveEllipsis: ParticipialComplement {
            form [omission: Ellipsis];
            require omission.form = GerundParticiple;
            export Voice = omission.Voice;
        }

        construction PassiveEllipsis: ParticipialComplement {
            form [omission: Ellipsis];
            require omission.form = PastParticiple;
            require omission.Voice = Passive;
            export Voice = omission.Voice;
        }

        construction PerfectEllipsis: PerfectComplement {
            form [omission: Ellipsis];
            require omission.form = PastParticiple;
            export Voice = omission.Voice;
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
            export number = head.number;
            export person = Third;
            export CaseUse = Common;
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
            require head.Targeting = No;
            require head.countability = Count;
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
        }

        construction FiniteSlashMeasure: FinitePredicate {
            form [head: lexical(Verb), " ", measure: SlashPair];
            require head.finiteness = Finite;
            require head.frame = SlashMeasure;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction SecondarySlashMeasure: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", measure: SlashPair];
            require head.frame = SlashMeasure;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
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
            form [left: MeasurePhrase, " ", operator: lexical(Preposition), " ", right: MeasurePhrase];
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
            export person = Third;
            export CaseUse = Common;
        }

        construction MeasuredAttribute: NounPhrase {
            form [head: lexical(Noun), " ", quantity: MeasurePhrase];
            require head.MeasurePosition = After;
            require quantity.MeasureKind = Scalar;
            require head.number = Singular;
            export number = Singular;
            export person = Third;
            export CaseUse = Common;
        }

        construction EqualityComplement: EqualityComplement {
            form [head: lexical(Adjective), " ", marker: lexical(Preposition), " ", measure: MeasurePhrase];
            require head.frame = Equality;
            require marker.ComparisonMarker = Equality;
            require measure.MeasureKind = Scalar;
        }

        construction EqualityAdjective: AdjectivePhrase {
            form [complement: EqualityComplement];
            export AdjectiveStructure = Complemented;
        }

        construction OrderingComplement: OrderingComplement {
            form [head: lexical(Adjective), " ", marker: lexical(Preposition), " ", measure: MeasurePhrase];
            require head.frame = Ordering;
            require marker.ComparisonMarker = Ordering;
            require measure.MeasureKind = Scalar;
        }

        construction OrderingAdjective: AdjectivePhrase {
            form [complement: OrderingComplement];
            export AdjectiveStructure = Complemented;
        }

        construction FiniteMeasure: FinitePredicate {
            form [head: lexical(Verb), " ", measure: MeasurePhrase];
            require head.finiteness = Finite;
            require head.frame = Measure;
            require measure.MeasureKind = Scalar;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteObjectEquality: FinitePredicate {
            form [head: lexical(Verb), " ", object: AccusativePhrase, " ", complement: EqualityComplement];
            require head.finiteness = Finite;
            require head.frame = ObjectEquality;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction SecondaryMeasure: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", measure: MeasurePhrase];
            require head.frame = Measure;
            require measure.MeasureKind = Scalar;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction SecondaryObjectEquality: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", object: AccusativePhrase, " ", complement: EqualityComplement];
            require head.frame = ObjectEquality;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction FiniteKeywordObject: FinitePredicate {
            form [head: lexical(Verb), " ", object: KeywordPhrase];
            require head.finiteness = Finite;
            require head.frame = KeywordObject;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction SecondaryKeywordObject: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", object: KeywordPhrase];
            require head.frame = KeywordObject;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction FiniteQuotedObject: FinitePredicate {
            form [head: lexical(Verb), " ", object: QuotedText];
            require head.finiteness = Finite;
            require head.frame = QuotedObject;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction SecondaryQuotedObject: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", object: QuotedText];
            require head.frame = QuotedObject;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction SecondaryCardinal: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", count: Cardinal];
            require head.frame = CardinalComplement;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }
        construction CardinalAmount: Amount {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Cardinal;
        }

        construction ScalarAmount: Amount {
            form [value: UnsignedScalar];
        }

        construction FiniteAmount: FinitePredicate {
            form [head: lexical(Verb), " ", amount: Amount];
            require head.frame = AmountComplement;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction SecondaryAmount: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", amount: Amount];
            require head.frame = AmountComplement;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        feature VPFinalAdjunct { Yes, No }
        feature ClauseInitialAdjunct { Yes, No }
        feature InfinitivalMarker { Yes }
        category AdverbPhrase(VPFinalAdjunct, ClauseInitialAdjunct);
        category InfinitiveComplement(Voice, OvertHead);
        frame InfinitiveSelection = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"InfinitiveComplement\"))])";

        construction Adverb: AdverbPhrase {
            form [head: lexical(Adverb)];
            export VPFinalAdjunct = head.VPFinalAdjunct;
            export ClauseInitialAdjunct = head.ClauseInitialAdjunct;
        }

        construction FiniteAdverb: FinitePredicate {
            form [head: FinitePredicate, " ", modifier: AdverbPhrase];
            require modifier.VPFinalAdjunct = Yes;
            export number = head.number;
            export person = head.person;
            export Voice = head.Voice;
        }

        construction SecondaryAdverb: SecondaryVerbPhrase {
            form [head: SecondaryVerbPhrase, " ", modifier: AdverbPhrase];
            require modifier.VPFinalAdjunct = Yes;
            export form = head.form;
            export Voice = head.Voice;
            export OvertHead = head.OvertHead;
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
            export Voice = predicate.Voice;
            export OvertHead = predicate.OvertHead;
        }

        construction FiniteInfinitive: FinitePredicate {
            form [head: lexical(Verb), " ", complement: InfinitiveComplement];
            require head.frame = InfinitiveSelection;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction SecondaryInfinitive: SecondaryVerbPhrase {
            form [head: lexical(Verb), " ", complement: InfinitiveComplement];
            require head.frame = InfinitiveSelection;
            export form = secondary_form(head.form);
            export Voice = Active;
            export OvertHead = Yes;
        }

        // Shared head primitives: exact selected signatures; no complement or coordination yet.
        feature FrameUse { Object, Predicative, Locative, Mana, Amount, Measure, SlashMeasure, Keyword, Quoted, AuxiliaryBare, AuxiliaryParticiple, AuxiliaryPerfect, ObjectName, ObjectEquality, Cardinal, Infinitive }
        feature HeadCoordination { No, Yes }
        category FiniteSelectedHead(number, person, FrameUse, HeadCoordination);
        category SecondarySelectedHead(form, FrameUse, HeadCoordination);
        category FiniteSelectedHeadSeries(number, person, FrameUse);
        category SecondarySelectedHeadSeries(form, FrameUse);
        construction FiniteSelectedObjectHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = Transitive;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Object;
            export HeadCoordination = No;
        }

        construction FiniteSelectedPredicativeHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = Predicative;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Predicative;
            export HeadCoordination = No;
        }

        construction FiniteSelectedLocativeHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = Locative;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Locative;
            export HeadCoordination = No;
        }

        construction FiniteSelectedManaHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = ManaComplement;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Mana;
            export HeadCoordination = No;
        }

        construction FiniteSelectedAmountHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = AmountComplement;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Amount;
            export HeadCoordination = No;
        }

        construction FiniteSelectedMeasureHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = Measure;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Measure;
            export HeadCoordination = No;
        }

        construction FiniteSelectedSlashMeasureHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = SlashMeasure;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = SlashMeasure;
            export HeadCoordination = No;
        }

        construction FiniteSelectedKeywordHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = KeywordObject;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Keyword;
            export HeadCoordination = No;
        }

        construction FiniteSelectedQuotedHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = QuotedObject;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Quoted;
            export HeadCoordination = No;
        }

        construction FiniteSelectedAuxiliaryBareHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = BareAuxiliary;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = AuxiliaryBare;
            export HeadCoordination = No;
        }

        construction FiniteSelectedAuxiliaryParticipleHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = ParticipialAuxiliary;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = AuxiliaryParticiple;
            export HeadCoordination = No;
        }

        construction FiniteSelectedAuxiliaryPerfectHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = PerfectAuxiliary;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = AuxiliaryPerfect;
            export HeadCoordination = No;
        }

        construction FiniteSelectedObjectNameHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = ObjectName;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = ObjectName;
            export HeadCoordination = No;
        }

        construction FiniteSelectedObjectEqualityHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = ObjectEquality;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = ObjectEquality;
            export HeadCoordination = No;
        }

        construction SecondarySelectedObjectHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = Transitive;
            export form = secondary_form(head.form);
            export FrameUse = Object;
            export HeadCoordination = No;
        }

        construction SecondarySelectedPredicativeHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = Predicative;
            export form = secondary_form(head.form);
            export FrameUse = Predicative;
            export HeadCoordination = No;
        }

        construction SecondarySelectedLocativeHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = Locative;
            export form = secondary_form(head.form);
            export FrameUse = Locative;
            export HeadCoordination = No;
        }

        construction SecondarySelectedManaHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = ManaComplement;
            export form = secondary_form(head.form);
            export FrameUse = Mana;
            export HeadCoordination = No;
        }

        construction SecondarySelectedAmountHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = AmountComplement;
            export form = secondary_form(head.form);
            export FrameUse = Amount;
            export HeadCoordination = No;
        }

        construction SecondarySelectedMeasureHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = Measure;
            export form = secondary_form(head.form);
            export FrameUse = Measure;
            export HeadCoordination = No;
        }

        construction SecondarySelectedSlashMeasureHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = SlashMeasure;
            export form = secondary_form(head.form);
            export FrameUse = SlashMeasure;
            export HeadCoordination = No;
        }

        construction SecondarySelectedKeywordHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = KeywordObject;
            export form = secondary_form(head.form);
            export FrameUse = Keyword;
            export HeadCoordination = No;
        }

        construction SecondarySelectedQuotedHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = QuotedObject;
            export form = secondary_form(head.form);
            export FrameUse = Quoted;
            export HeadCoordination = No;
        }

        construction SecondarySelectedAuxiliaryBareHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = BareAuxiliary;
            export form = secondary_form(head.form);
            export FrameUse = AuxiliaryBare;
            export HeadCoordination = No;
        }

        construction SecondarySelectedAuxiliaryParticipleHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = ParticipialAuxiliary;
            export form = secondary_form(head.form);
            export FrameUse = AuxiliaryParticiple;
            export HeadCoordination = No;
        }

        construction SecondarySelectedAuxiliaryPerfectHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = PerfectAuxiliary;
            export form = secondary_form(head.form);
            export FrameUse = AuxiliaryPerfect;
            export HeadCoordination = No;
        }

        construction SecondarySelectedObjectNameHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = ObjectName;
            export form = secondary_form(head.form);
            export FrameUse = ObjectName;
            export HeadCoordination = No;
        }

        construction SecondarySelectedObjectEqualityHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = ObjectEquality;
            export form = secondary_form(head.form);
            export FrameUse = ObjectEquality;
            export HeadCoordination = No;
        }



        feature NoncorrelativeCoordination { Yes, No }
        feature CorrelativeKind { Both, Either, Neither }
        feature CorrelativeCoordinator { And, Or, Nor }
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

        category ManaPhraseSeries();

        construction ManaPhraseCoordination: ManaPhrase {
            form [left: ManaPhrase, " ", coordinator: lexical(Coordinator), " ", right: ManaPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction ManaPhraseSeriesEnd: ManaPhraseSeries {
            form [left: ManaPhrase, ", ", coordinator: lexical(Coordinator), " ", right: ManaPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction ManaPhraseSeriesContinuation: ManaPhraseSeries {
            form [left: ManaPhrase, ", ", rest: ManaPhraseSeries];
        }

        construction SerialManaPhrase: ManaPhrase {
            form [left: ManaPhrase, ", ", rest: ManaPhraseSeries];
        }

        category CardinalSeries(number, CoordinationKind);

        construction CardinalCoordination: Cardinal {
            form [left: Cardinal, " ", coordinator: lexical(Coordinator), " ", right: Cardinal];
            require coordinator.NoncorrelativeCoordination = Yes;
            export number = cardinal_coordinate_number(coordinator.CoordinationKind, left.number, right.number);
        }

        construction CardinalSeriesEnd: CardinalSeries {
            form [left: Cardinal, ", ", coordinator: lexical(Coordinator), " ", right: Cardinal];
            require coordinator.NoncorrelativeCoordination = Yes;
            export number = cardinal_coordinate_number(coordinator.CoordinationKind, left.number, right.number);
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CardinalSeriesContinuation: CardinalSeries {
            form [left: Cardinal, ", ", rest: CardinalSeries];
            export number = cardinal_coordinate_number(rest.CoordinationKind, left.number, rest.number);
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialCardinal: Cardinal {
            form [left: Cardinal, ", ", rest: CardinalSeries];
            export number = cardinal_coordinate_number(rest.CoordinationKind, left.number, rest.number);
        }

        category AmountSeries();

        construction AmountCoordination: Amount {
            form [left: Amount, " ", coordinator: lexical(Coordinator), " ", right: Amount];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction AmountSeriesEnd: AmountSeries {
            form [left: Amount, ", ", coordinator: lexical(Coordinator), " ", right: Amount];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction AmountSeriesContinuation: AmountSeries {
            form [left: Amount, ", ", rest: AmountSeries];
        }

        construction SerialAmount: Amount {
            form [left: Amount, ", ", rest: AmountSeries];
        }

        category MeasurePhraseSeries(MeasureKind);

        construction MeasurePhraseCoordination: MeasurePhrase {
            form [left: MeasurePhrase, " ", coordinator: lexical(Coordinator), " ", right: MeasurePhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
            agree left.MeasureKind = right.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        construction MeasurePhraseSeriesEnd: MeasurePhraseSeries {
            form [left: MeasurePhrase, ", ", coordinator: lexical(Coordinator), " ", right: MeasurePhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
            agree left.MeasureKind = right.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        construction MeasurePhraseSeriesContinuation: MeasurePhraseSeries {
            form [left: MeasurePhrase, ", ", rest: MeasurePhraseSeries];
            agree left.MeasureKind = rest.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        construction SerialMeasurePhrase: MeasurePhrase {
            form [left: MeasurePhrase, ", ", rest: MeasurePhraseSeries];
            agree left.MeasureKind = rest.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        category KeywordPhraseSeries();

        construction KeywordPhraseCoordination: KeywordPhrase {
            form [left: KeywordPhrase, " ", coordinator: lexical(Coordinator), " ", right: KeywordPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction KeywordPhraseSeriesEnd: KeywordPhraseSeries {
            form [left: KeywordPhrase, ", ", coordinator: lexical(Coordinator), " ", right: KeywordPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction KeywordPhraseSeriesContinuation: KeywordPhraseSeries {
            form [left: KeywordPhrase, ", ", rest: KeywordPhraseSeries];
        }

        construction SerialKeywordPhrase: KeywordPhrase {
            form [left: KeywordPhrase, ", ", rest: KeywordPhraseSeries];
        }

        category QuotedTextSeries();

        construction QuotedTextCoordination: QuotedText {
            form [left: QuotedText, " ", coordinator: lexical(Coordinator), " ", right: QuotedText];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction QuotedTextSeriesEnd: QuotedTextSeries {
            form [left: QuotedText, ", ", coordinator: lexical(Coordinator), " ", right: QuotedText];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction QuotedTextSeriesContinuation: QuotedTextSeries {
            form [left: QuotedText, ", ", rest: QuotedTextSeries];
        }

        construction SerialQuotedText: QuotedText {
            form [left: QuotedText, ", ", rest: QuotedTextSeries];
        }

        construction FiniteSelectedHeadCoordination: FiniteSelectedHead {
            form [left: FiniteSelectedHead, " ", coordinator: lexical(Coordinator), " ", right: FiniteSelectedHead];
            require coordinator.NoncorrelativeCoordination = Yes;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction FiniteSelectedHeadSeriesEnd: FiniteSelectedHeadSeries {
            form [left: FiniteSelectedHead, ", ", coordinator: lexical(Coordinator), " ", right: FiniteSelectedHead];
            require coordinator.NoncorrelativeCoordination = Yes;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
        }

        construction FiniteSelectedHeadSeriesContinuation: FiniteSelectedHeadSeries {
            form [left: FiniteSelectedHead, ", ", rest: FiniteSelectedHeadSeries];
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
        }

        construction SerialFiniteSelectedHead: FiniteSelectedHead {
            form [left: FiniteSelectedHead, ", ", rest: FiniteSelectedHeadSeries];
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction FiniteSharedObjectComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", object: AccusativePhrase];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Object;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedPredicativeComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", complement: PredicativeComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Predicative;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedLocativeComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", complement: LocativeComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Locative;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedManaComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", complement: ManaPhrase];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Mana;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedAmountComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", amount: Amount];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Amount;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedMeasureComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", measure: MeasurePhrase];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Measure;
            require measure.MeasureKind = Scalar;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedSlashMeasureComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", measure: SlashPair];
            require head.HeadCoordination = Yes;
            require head.FrameUse = SlashMeasure;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedKeywordComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", object: KeywordPhrase];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Keyword;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedQuotedComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", object: QuotedText];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Quoted;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedAuxiliaryBareComplement: FinitePredicate {
            form [head: FiniteSelectedHead, complement: BareComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = AuxiliaryBare;
            export number = head.number;
            export person = head.person;
            export Voice = complement.Voice;
        }

        construction FiniteSharedAuxiliaryParticipleComplement: FinitePredicate {
            form [head: FiniteSelectedHead, complement: ParticipialComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = AuxiliaryParticiple;
            export number = head.number;
            export person = head.person;
            export Voice = complement.Voice;
        }

        construction FiniteSharedAuxiliaryPerfectComplement: FinitePredicate {
            form [head: FiniteSelectedHead, complement: PerfectComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = AuxiliaryPerfect;
            export number = head.number;
            export person = head.person;
            export Voice = complement.Voice;
        }

        construction FiniteSharedObjectNameComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", object: AccusativePhrase, " ", complement: Name];
            require head.HeadCoordination = Yes;
            require head.FrameUse = ObjectName;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction FiniteSharedObjectEqualityComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", object: AccusativePhrase, " ", complement: EqualityComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = ObjectEquality;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction SecondarySelectedHeadCoordination: SecondarySelectedHead {
            form [left: SecondarySelectedHead, " ", coordinator: lexical(Coordinator), " ", right: SecondarySelectedHead];
            require coordinator.NoncorrelativeCoordination = Yes;
            agree left.form = right.form;
            export form = left.form;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction SecondarySelectedHeadSeriesEnd: SecondarySelectedHeadSeries {
            form [left: SecondarySelectedHead, ", ", coordinator: lexical(Coordinator), " ", right: SecondarySelectedHead];
            require coordinator.NoncorrelativeCoordination = Yes;
            agree left.form = right.form;
            export form = left.form;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
        }

        construction SecondarySelectedHeadSeriesContinuation: SecondarySelectedHeadSeries {
            form [left: SecondarySelectedHead, ", ", rest: SecondarySelectedHeadSeries];
            agree left.form = rest.form;
            export form = left.form;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
        }

        construction SerialSecondarySelectedHead: SecondarySelectedHead {
            form [left: SecondarySelectedHead, ", ", rest: SecondarySelectedHeadSeries];
            agree left.form = rest.form;
            export form = left.form;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction SecondarySharedObjectComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", object: AccusativePhrase];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Object;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedPredicativeComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", complement: PredicativeComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Predicative;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedLocativeComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", complement: LocativeComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Locative;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedManaComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", complement: ManaPhrase];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Mana;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedAmountComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", amount: Amount];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Amount;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedMeasureComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", measure: MeasurePhrase];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Measure;
            require measure.MeasureKind = Scalar;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedSlashMeasureComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", measure: SlashPair];
            require head.HeadCoordination = Yes;
            require head.FrameUse = SlashMeasure;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedKeywordComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", object: KeywordPhrase];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Keyword;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedQuotedComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", object: QuotedText];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Quoted;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedAuxiliaryBareComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, complement: BareComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = AuxiliaryBare;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = complement.Voice;
        }

        construction SecondarySharedAuxiliaryParticipleComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, complement: ParticipialComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = AuxiliaryParticiple;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = complement.Voice;
        }

        construction SecondarySharedAuxiliaryPerfectComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, complement: PerfectComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = AuxiliaryPerfect;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = complement.Voice;
        }

        construction SecondarySharedObjectNameComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", object: AccusativePhrase, " ", complement: Name];
            require head.HeadCoordination = Yes;
            require head.FrameUse = ObjectName;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction SecondarySharedObjectEqualityComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", object: AccusativePhrase, " ", complement: EqualityComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = ObjectEquality;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
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

        table coordinated_initial_adjunct(ClauseInitialAdjunct, ClauseInitialAdjunct) -> ClauseInitialAdjunct {
            (Yes, Yes) => Yes,
            (Yes, No) => No,
            (No, Yes) => No,
            (No, No) => No,
        }

        table coordinated_adjective_structure(AdjectiveStructure, AdjectiveStructure) -> AdjectiveStructure {
            (Simple, Simple) => Simple,
            (Simple, Complemented) => Complemented,
            (Complemented, Simple) => Complemented,
            (Complemented, Complemented) => Complemented,
        }

        category NominalSeries(number, countability, Targeting);

        construction NominalCoordination: Nominal {
            form [left: Nominal, " ", coordinator: lexical(Coordinator), " ", right: Nominal];
            require coordinator.NoncorrelativeCoordination = Yes;
            agree left.number = right.number;
            agree left.countability = right.countability;
            agree left.Targeting = right.Targeting;
            export number = left.number;
            export countability = left.countability;
            export Targeting = left.Targeting;
        }

        construction NominalSeriesEnd: NominalSeries {
            form [left: Nominal, ", ", coordinator: lexical(Coordinator), " ", right: Nominal];
            require coordinator.NoncorrelativeCoordination = Yes;
            agree left.number = right.number;
            agree left.countability = right.countability;
            agree left.Targeting = right.Targeting;
            export number = left.number;
            export countability = left.countability;
            export Targeting = left.Targeting;
        }

        construction NominalSeriesContinuation: NominalSeries {
            form [left: Nominal, ", ", rest: NominalSeries];
            agree left.number = rest.number;
            agree left.countability = rest.countability;
            agree left.Targeting = rest.Targeting;
            export number = left.number;
            export countability = left.countability;
            export Targeting = left.Targeting;
        }

        construction SerialNominal: Nominal {
            form [left: Nominal, ", ", rest: NominalSeries];
            agree left.number = rest.number;
            agree left.countability = rest.countability;
            agree left.Targeting = rest.Targeting;
            export number = left.number;
            export countability = left.countability;
            export Targeting = left.Targeting;
        }

        category AdjectivePhraseSeries(AdjectiveStructure);

        construction AdjectivePhraseCoordination: AdjectivePhrase {
            form [left: AdjectivePhrase, " ", coordinator: lexical(Coordinator), " ", right: AdjectivePhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
            export AdjectiveStructure = coordinated_adjective_structure(left.AdjectiveStructure, right.AdjectiveStructure);
        }

        construction AdjectivePhraseSeriesEnd: AdjectivePhraseSeries {
            form [left: AdjectivePhrase, ", ", coordinator: lexical(Coordinator), " ", right: AdjectivePhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
            export AdjectiveStructure = coordinated_adjective_structure(left.AdjectiveStructure, right.AdjectiveStructure);
        }

        construction AdjectivePhraseSeriesContinuation: AdjectivePhraseSeries {
            form [left: AdjectivePhrase, ", ", rest: AdjectivePhraseSeries];
            export AdjectiveStructure = coordinated_adjective_structure(left.AdjectiveStructure, rest.AdjectiveStructure);
        }

        construction SerialAdjectivePhrase: AdjectivePhrase {
            form [left: AdjectivePhrase, ", ", rest: AdjectivePhraseSeries];
            export AdjectiveStructure = coordinated_adjective_structure(left.AdjectiveStructure, rest.AdjectiveStructure);
        }

        category PrepositionPhraseSeries(LocativeUse, AdverbialUse);

        construction PrepositionPhraseCoordination: PrepositionPhrase {
            form [left: PrepositionPhrase, " ", coordinator: lexical(Coordinator), " ", right: PrepositionPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
            export LocativeUse = coordinated_locative_use(left.LocativeUse, right.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, right.AdverbialUse);
        }

        construction PrepositionPhraseSeriesEnd: PrepositionPhraseSeries {
            form [left: PrepositionPhrase, ", ", coordinator: lexical(Coordinator), " ", right: PrepositionPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
            export LocativeUse = coordinated_locative_use(left.LocativeUse, right.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, right.AdverbialUse);
        }

        construction PrepositionPhraseSeriesContinuation: PrepositionPhraseSeries {
            form [left: PrepositionPhrase, ", ", rest: PrepositionPhraseSeries];
            export LocativeUse = coordinated_locative_use(left.LocativeUse, rest.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, rest.AdverbialUse);
        }

        construction SerialPrepositionPhrase: PrepositionPhrase {
            form [left: PrepositionPhrase, ", ", rest: PrepositionPhraseSeries];
            export LocativeUse = coordinated_locative_use(left.LocativeUse, rest.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, rest.AdverbialUse);
        }

        category AdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct);

        construction AdverbPhraseCoordination: AdverbPhrase {
            form [left: AdverbPhrase, " ", coordinator: lexical(Coordinator), " ", right: AdverbPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, right.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, right.ClauseInitialAdjunct);
        }

        construction AdverbPhraseSeriesEnd: AdverbPhraseSeries {
            form [left: AdverbPhrase, ", ", coordinator: lexical(Coordinator), " ", right: AdverbPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, right.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, right.ClauseInitialAdjunct);
        }

        construction AdverbPhraseSeriesContinuation: AdverbPhraseSeries {
            form [left: AdverbPhrase, ", ", rest: AdverbPhraseSeries];
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, rest.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, rest.ClauseInitialAdjunct);
        }

        construction SerialAdverbPhrase: AdverbPhrase {
            form [left: AdverbPhrase, ", ", rest: AdverbPhraseSeries];
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, rest.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, rest.ClauseInitialAdjunct);
        }

        category InfinitiveComplementSeries(Voice, OvertHead);

        construction InfinitiveComplementCoordination: InfinitiveComplement {
            form [left: InfinitiveComplement, " ", coordinator: lexical(Coordinator), " ", right: InfinitiveComplement];
            require coordinator.NoncorrelativeCoordination = Yes;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        construction InfinitiveComplementSeriesEnd: InfinitiveComplementSeries {
            form [left: InfinitiveComplement, ", ", coordinator: lexical(Coordinator), " ", right: InfinitiveComplement];
            require coordinator.NoncorrelativeCoordination = Yes;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        construction InfinitiveComplementSeriesContinuation: InfinitiveComplementSeries {
            form [left: InfinitiveComplement, ", ", rest: InfinitiveComplementSeries];
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        construction SerialInfinitiveComplement: InfinitiveComplement {
            form [left: InfinitiveComplement, ", ", rest: InfinitiveComplementSeries];
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        category FrequencyPhraseSeries();

        construction FrequencyPhraseCoordination: FrequencyPhrase {
            form [left: FrequencyPhrase, " ", coordinator: lexical(Coordinator), " ", right: FrequencyPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction FrequencyPhraseSeriesEnd: FrequencyPhraseSeries {
            form [left: FrequencyPhrase, ", ", coordinator: lexical(Coordinator), " ", right: FrequencyPhrase];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction FrequencyPhraseSeriesContinuation: FrequencyPhraseSeries {
            form [left: FrequencyPhrase, ", ", rest: FrequencyPhraseSeries];
        }

        construction SerialFrequencyPhrase: FrequencyPhrase {
            form [left: FrequencyPhrase, ", ", rest: FrequencyPhraseSeries];
        }


        category FiniteObjectGapSeries(number, person);

        construction FiniteObjectGapSeriesEnd: FiniteObjectGapSeries {
            form [left: FiniteObjectGap, ", ", coordinator: lexical(Coordinator), " ", right: FiniteObjectGap];
            require coordinator.NoncorrelativeCoordination = Yes;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
        }

        construction FiniteObjectGapSeriesContinuation: FiniteObjectGapSeries {
            form [left: FiniteObjectGap, ", ", rest: FiniteObjectGapSeries];
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
        }

        construction SerialFiniteObjectGap: FiniteObjectGap {
            form [left: FiniteObjectGap, ", ", rest: FiniteObjectGapSeries];
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
        }

        category BareObjectGapSeries();

        construction BareObjectGapCoordination: BareObjectGap {
            form [left: BareObjectGap, " ", coordinator: lexical(Coordinator), " ", right: BareObjectGap];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction BareObjectGapSeriesEnd: BareObjectGapSeries {
            form [left: BareObjectGap, ", ", coordinator: lexical(Coordinator), " ", right: BareObjectGap];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction BareObjectGapSeriesContinuation: BareObjectGapSeries {
            form [left: BareObjectGap, ", ", rest: BareObjectGapSeries];
        }

        construction SerialBareObjectGap: BareObjectGap {
            form [left: BareObjectGap, ", ", rest: BareObjectGapSeries];
        }

        category CoordinatedFiniteClause();

        category FiniteClauseSeries();

        construction CoordinatedFiniteClauseCoordination: CoordinatedFiniteClause {
            form [left: FiniteClause, " ", coordinator: lexical(Coordinator), " ", right: FiniteClause];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction FiniteClauseSeriesEnd: FiniteClauseSeries {
            form [left: FiniteClause, ", ", coordinator: lexical(Coordinator), " ", right: FiniteClause];
            require coordinator.NoncorrelativeCoordination = Yes;
        }

        construction FiniteClauseSeriesContinuation: FiniteClauseSeries {
            form [left: FiniteClause, ", ", rest: FiniteClauseSeries];
        }

        construction SerialCoordinatedFiniteClause: CoordinatedFiniteClause {
            form [left: FiniteClause, ", ", rest: FiniteClauseSeries];
        }

        construction CoordinatedClauseComplementPreposition: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: CoordinatedFiniteClause];
            require head.FiniteClauseComplement = Yes;
            export LocativeUse = No;
            export AdverbialUse = Yes;
        }
        feature PredicativeKind { Adjectival, Nominal, Mixed }
        category PredicativeComplementSeries(PredicativeKind);
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
        construction UnlikePredicativeCoordination: PredicativeComplement {
            form [left: PredicativeComplement, " ", coordinator: lexical(Coordinator), " ", right: PredicativeComplement];
            require coordinator.NoncorrelativeCoordination = Yes;
            export PredicativeKind = unlike_predicative_kind(left.PredicativeKind, right.PredicativeKind);
        }
        construction PredicativeComplementSeriesEnd: PredicativeComplementSeries {
            form [left: PredicativeComplement, ", ", coordinator: lexical(Coordinator), " ", right: PredicativeComplement];
            require coordinator.NoncorrelativeCoordination = Yes;
            export PredicativeKind = combined_predicative_kind(left.PredicativeKind, right.PredicativeKind);
        }
        construction PredicativeComplementSeriesContinuation: PredicativeComplementSeries {
            form [left: PredicativeComplement, ", ", rest: PredicativeComplementSeries];
            export PredicativeKind = combined_predicative_kind(left.PredicativeKind, rest.PredicativeKind);
        }
        construction SerialUnlikePredicative: PredicativeComplement {
            form [left: PredicativeComplement, ", ", rest: PredicativeComplementSeries];
            export PredicativeKind = unlike_predicative_kind(left.PredicativeKind, rest.PredicativeKind);
        }

        // Correlative prefixes: Both binary only; main Clause Either only.
        // Nominal/AdjP deferred until pre-head placement is represented in summaries.
        construction EitherClauseCoordination: Clause {
            form [marker: lexical(Determinative), " ", left: Clause, " ", coordinator: lexical(Coordinator), " ", right: Clause];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            agree left.finiteness = right.finiteness;
            export finiteness = left.finiteness;
        }

        category CorrelativeClauseSeries(finiteness, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeClauseSeriesEnd: CorrelativeClauseSeries {
            form [left: Clause, ", ", coordinator: lexical(Coordinator), " ", right: Clause];
            agree left.finiteness = right.finiteness;
            export finiteness = left.finiteness;
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeClauseSeriesContinuation: CorrelativeClauseSeries {
            form [left: Clause, ", ", rest: CorrelativeClauseSeries];
            agree left.finiteness = rest.finiteness;
            export finiteness = left.finiteness;
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherClauseCoordination: Clause {
            form [marker: lexical(Determinative), " ", left: Clause, ", ", rest: CorrelativeClauseSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            agree left.finiteness = rest.finiteness;
            export finiteness = left.finiteness;
        }

        construction BothFinitePredicateCoordination: FinitePredicate {
            form [marker: lexical(Determinative), " ", left: FinitePredicate, " ", coordinator: lexical(Coordinator), " ", right: FinitePredicate];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, right.Voice);
        }

        construction EitherFinitePredicateCoordination: FinitePredicate {
            form [marker: lexical(Determinative), " ", left: FinitePredicate, " ", coordinator: lexical(Coordinator), " ", right: FinitePredicate];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, right.Voice);
        }

        construction NeitherFinitePredicateCoordination: FinitePredicate {
            form [marker: lexical(Determinative), " ", left: FinitePredicate, " ", coordinator: lexical(Coordinator), " ", right: FinitePredicate];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, right.Voice);
        }

        category CorrelativeFinitePredicateSeries(number, person, Voice, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeFinitePredicateSeriesEnd: CorrelativeFinitePredicateSeries {
            form [left: FinitePredicate, ", ", coordinator: lexical(Coordinator), " ", right: FinitePredicate];
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeFinitePredicateSeriesContinuation: CorrelativeFinitePredicateSeries {
            form [left: FinitePredicate, ", ", rest: CorrelativeFinitePredicateSeries];
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherFinitePredicateCoordination: FinitePredicate {
            form [marker: lexical(Determinative), " ", left: FinitePredicate, ", ", rest: CorrelativeFinitePredicateSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
        }

        construction SerialNeitherFinitePredicateCoordination: FinitePredicate {
            form [marker: lexical(Determinative), " ", left: FinitePredicate, ", ", rest: CorrelativeFinitePredicateSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
        }

        construction BothSecondaryVerbPhraseCoordination: SecondaryVerbPhrase {
            form [marker: lexical(Determinative), " ", left: SecondaryVerbPhrase, " ", coordinator: lexical(Coordinator), " ", right: SecondaryVerbPhrase];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            agree left.form = right.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        construction EitherSecondaryVerbPhraseCoordination: SecondaryVerbPhrase {
            form [marker: lexical(Determinative), " ", left: SecondaryVerbPhrase, " ", coordinator: lexical(Coordinator), " ", right: SecondaryVerbPhrase];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            agree left.form = right.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        construction NeitherSecondaryVerbPhraseCoordination: SecondaryVerbPhrase {
            form [marker: lexical(Determinative), " ", left: SecondaryVerbPhrase, " ", coordinator: lexical(Coordinator), " ", right: SecondaryVerbPhrase];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            agree left.form = right.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        category CorrelativeSecondaryVerbPhraseSeries(form, Voice, OvertHead, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeSecondaryVerbPhraseSeriesEnd: CorrelativeSecondaryVerbPhraseSeries {
            form [left: SecondaryVerbPhrase, ", ", coordinator: lexical(Coordinator), " ", right: SecondaryVerbPhrase];
            agree left.form = right.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeSecondaryVerbPhraseSeriesContinuation: CorrelativeSecondaryVerbPhraseSeries {
            form [left: SecondaryVerbPhrase, ", ", rest: CorrelativeSecondaryVerbPhraseSeries];
            agree left.form = rest.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherSecondaryVerbPhraseCoordination: SecondaryVerbPhrase {
            form [marker: lexical(Determinative), " ", left: SecondaryVerbPhrase, ", ", rest: CorrelativeSecondaryVerbPhraseSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            agree left.form = rest.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        construction SerialNeitherSecondaryVerbPhraseCoordination: SecondaryVerbPhrase {
            form [marker: lexical(Determinative), " ", left: SecondaryVerbPhrase, ", ", rest: CorrelativeSecondaryVerbPhraseSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            agree left.form = rest.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        construction BothNounPhraseCoordination: NounPhrase {
            form [marker: lexical(Determinative), " ", left: NounPhrase, " ", coordinator: lexical(Coordinator), " ", right: NounPhrase];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            export number = coordinate_number(coordinator.CoordinationKind, left.number, right.number);
            export person = coordinate_person(coordinator.CoordinationKind, left.person, right.person);
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
        }

        construction EitherNounPhraseCoordination: NounPhrase {
            form [marker: lexical(Determinative), " ", left: NounPhrase, " ", coordinator: lexical(Coordinator), " ", right: NounPhrase];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            export number = coordinate_number(coordinator.CoordinationKind, left.number, right.number);
            export person = coordinate_person(coordinator.CoordinationKind, left.person, right.person);
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
        }

        construction NeitherNounPhraseCoordination: NounPhrase {
            form [marker: lexical(Determinative), " ", left: NounPhrase, " ", coordinator: lexical(Coordinator), " ", right: NounPhrase];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            export number = coordinate_number(coordinator.CoordinationKind, left.number, right.number);
            export person = coordinate_person(coordinator.CoordinationKind, left.person, right.person);
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
        }

        category CorrelativeNounPhraseSeries(number, person, CaseUse, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeNounPhraseSeriesEnd: CorrelativeNounPhraseSeries {
            form [left: NounPhrase, ", ", coordinator: lexical(Coordinator), " ", right: NounPhrase];
            export number = coordinate_number(coordinator.CoordinationKind, left.number, right.number);
            export person = coordinate_person(coordinator.CoordinationKind, left.person, right.person);
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeNounPhraseSeriesContinuation: CorrelativeNounPhraseSeries {
            form [left: NounPhrase, ", ", rest: CorrelativeNounPhraseSeries];
            export number = coordinate_number(rest.CoordinationKind, left.number, rest.number);
            export person = coordinate_person(rest.CoordinationKind, left.person, rest.person);
            export CaseUse = common_case(left.CaseUse, rest.CaseUse);
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherNounPhraseCoordination: NounPhrase {
            form [marker: lexical(Determinative), " ", left: NounPhrase, ", ", rest: CorrelativeNounPhraseSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            export number = coordinate_number(rest.CoordinationKind, left.number, rest.number);
            export person = coordinate_person(rest.CoordinationKind, left.person, rest.person);
            export CaseUse = common_case(left.CaseUse, rest.CaseUse);
        }

        construction SerialNeitherNounPhraseCoordination: NounPhrase {
            form [marker: lexical(Determinative), " ", left: NounPhrase, ", ", rest: CorrelativeNounPhraseSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            export number = coordinate_number(rest.CoordinationKind, left.number, rest.number);
            export person = coordinate_person(rest.CoordinationKind, left.person, rest.person);
            export CaseUse = common_case(left.CaseUse, rest.CaseUse);
        }

        construction BothPrepositionPhraseCoordination: PrepositionPhrase {
            form [marker: lexical(Determinative), " ", left: PrepositionPhrase, " ", coordinator: lexical(Coordinator), " ", right: PrepositionPhrase];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            export LocativeUse = coordinated_locative_use(left.LocativeUse, right.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, right.AdverbialUse);
        }

        construction EitherPrepositionPhraseCoordination: PrepositionPhrase {
            form [marker: lexical(Determinative), " ", left: PrepositionPhrase, " ", coordinator: lexical(Coordinator), " ", right: PrepositionPhrase];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            export LocativeUse = coordinated_locative_use(left.LocativeUse, right.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, right.AdverbialUse);
        }

        construction NeitherPrepositionPhraseCoordination: PrepositionPhrase {
            form [marker: lexical(Determinative), " ", left: PrepositionPhrase, " ", coordinator: lexical(Coordinator), " ", right: PrepositionPhrase];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            export LocativeUse = coordinated_locative_use(left.LocativeUse, right.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, right.AdverbialUse);
        }

        category CorrelativePrepositionPhraseSeries(LocativeUse, AdverbialUse, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativePrepositionPhraseSeriesEnd: CorrelativePrepositionPhraseSeries {
            form [left: PrepositionPhrase, ", ", coordinator: lexical(Coordinator), " ", right: PrepositionPhrase];
            export LocativeUse = coordinated_locative_use(left.LocativeUse, right.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, right.AdverbialUse);
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativePrepositionPhraseSeriesContinuation: CorrelativePrepositionPhraseSeries {
            form [left: PrepositionPhrase, ", ", rest: CorrelativePrepositionPhraseSeries];
            export LocativeUse = coordinated_locative_use(left.LocativeUse, rest.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, rest.AdverbialUse);
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherPrepositionPhraseCoordination: PrepositionPhrase {
            form [marker: lexical(Determinative), " ", left: PrepositionPhrase, ", ", rest: CorrelativePrepositionPhraseSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            export LocativeUse = coordinated_locative_use(left.LocativeUse, rest.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, rest.AdverbialUse);
        }

        construction SerialNeitherPrepositionPhraseCoordination: PrepositionPhrase {
            form [marker: lexical(Determinative), " ", left: PrepositionPhrase, ", ", rest: CorrelativePrepositionPhraseSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            export LocativeUse = coordinated_locative_use(left.LocativeUse, rest.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, rest.AdverbialUse);
        }

        construction BothAdverbPhraseCoordination: AdverbPhrase {
            form [marker: lexical(Determinative), " ", left: AdverbPhrase, " ", coordinator: lexical(Coordinator), " ", right: AdverbPhrase];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, right.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, right.ClauseInitialAdjunct);
        }

        construction EitherAdverbPhraseCoordination: AdverbPhrase {
            form [marker: lexical(Determinative), " ", left: AdverbPhrase, " ", coordinator: lexical(Coordinator), " ", right: AdverbPhrase];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, right.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, right.ClauseInitialAdjunct);
        }

        construction NeitherAdverbPhraseCoordination: AdverbPhrase {
            form [marker: lexical(Determinative), " ", left: AdverbPhrase, " ", coordinator: lexical(Coordinator), " ", right: AdverbPhrase];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, right.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, right.ClauseInitialAdjunct);
        }

        category CorrelativeAdverbPhraseSeries(VPFinalAdjunct, ClauseInitialAdjunct, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeAdverbPhraseSeriesEnd: CorrelativeAdverbPhraseSeries {
            form [left: AdverbPhrase, ", ", coordinator: lexical(Coordinator), " ", right: AdverbPhrase];
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, right.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, right.ClauseInitialAdjunct);
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeAdverbPhraseSeriesContinuation: CorrelativeAdverbPhraseSeries {
            form [left: AdverbPhrase, ", ", rest: CorrelativeAdverbPhraseSeries];
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, rest.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, rest.ClauseInitialAdjunct);
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherAdverbPhraseCoordination: AdverbPhrase {
            form [marker: lexical(Determinative), " ", left: AdverbPhrase, ", ", rest: CorrelativeAdverbPhraseSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, rest.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, rest.ClauseInitialAdjunct);
        }

        construction SerialNeitherAdverbPhraseCoordination: AdverbPhrase {
            form [marker: lexical(Determinative), " ", left: AdverbPhrase, ", ", rest: CorrelativeAdverbPhraseSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            export VPFinalAdjunct = coordinated_final_adjunct(left.VPFinalAdjunct, rest.VPFinalAdjunct);
            export ClauseInitialAdjunct = coordinated_initial_adjunct(left.ClauseInitialAdjunct, rest.ClauseInitialAdjunct);
        }

        construction BothManaPhraseCoordination: ManaPhrase {
            form [marker: lexical(Determinative), " ", left: ManaPhrase, " ", coordinator: lexical(Coordinator), " ", right: ManaPhrase];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
        }

        construction EitherManaPhraseCoordination: ManaPhrase {
            form [marker: lexical(Determinative), " ", left: ManaPhrase, " ", coordinator: lexical(Coordinator), " ", right: ManaPhrase];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
        }

        construction NeitherManaPhraseCoordination: ManaPhrase {
            form [marker: lexical(Determinative), " ", left: ManaPhrase, " ", coordinator: lexical(Coordinator), " ", right: ManaPhrase];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
        }

        category CorrelativeManaPhraseSeries(CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeManaPhraseSeriesEnd: CorrelativeManaPhraseSeries {
            form [left: ManaPhrase, ", ", coordinator: lexical(Coordinator), " ", right: ManaPhrase];
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeManaPhraseSeriesContinuation: CorrelativeManaPhraseSeries {
            form [left: ManaPhrase, ", ", rest: CorrelativeManaPhraseSeries];
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherManaPhraseCoordination: ManaPhrase {
            form [marker: lexical(Determinative), " ", left: ManaPhrase, ", ", rest: CorrelativeManaPhraseSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
        }

        construction SerialNeitherManaPhraseCoordination: ManaPhrase {
            form [marker: lexical(Determinative), " ", left: ManaPhrase, ", ", rest: CorrelativeManaPhraseSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
        }

        construction BothCardinalCoordination: Cardinal {
            form [marker: lexical(Determinative), " ", left: Cardinal, " ", coordinator: lexical(Coordinator), " ", right: Cardinal];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            export number = cardinal_coordinate_number(coordinator.CoordinationKind, left.number, right.number);
        }

        construction EitherCardinalCoordination: Cardinal {
            form [marker: lexical(Determinative), " ", left: Cardinal, " ", coordinator: lexical(Coordinator), " ", right: Cardinal];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            export number = cardinal_coordinate_number(coordinator.CoordinationKind, left.number, right.number);
        }

        construction NeitherCardinalCoordination: Cardinal {
            form [marker: lexical(Determinative), " ", left: Cardinal, " ", coordinator: lexical(Coordinator), " ", right: Cardinal];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            export number = cardinal_coordinate_number(coordinator.CoordinationKind, left.number, right.number);
        }

        category CorrelativeCardinalSeries(number, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeCardinalSeriesEnd: CorrelativeCardinalSeries {
            form [left: Cardinal, ", ", coordinator: lexical(Coordinator), " ", right: Cardinal];
            export number = cardinal_coordinate_number(coordinator.CoordinationKind, left.number, right.number);
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeCardinalSeriesContinuation: CorrelativeCardinalSeries {
            form [left: Cardinal, ", ", rest: CorrelativeCardinalSeries];
            export number = cardinal_coordinate_number(rest.CoordinationKind, left.number, rest.number);
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherCardinalCoordination: Cardinal {
            form [marker: lexical(Determinative), " ", left: Cardinal, ", ", rest: CorrelativeCardinalSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            export number = cardinal_coordinate_number(rest.CoordinationKind, left.number, rest.number);
        }

        construction SerialNeitherCardinalCoordination: Cardinal {
            form [marker: lexical(Determinative), " ", left: Cardinal, ", ", rest: CorrelativeCardinalSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            export number = cardinal_coordinate_number(rest.CoordinationKind, left.number, rest.number);
        }

        construction BothAmountCoordination: Amount {
            form [marker: lexical(Determinative), " ", left: Amount, " ", coordinator: lexical(Coordinator), " ", right: Amount];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
        }

        construction EitherAmountCoordination: Amount {
            form [marker: lexical(Determinative), " ", left: Amount, " ", coordinator: lexical(Coordinator), " ", right: Amount];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
        }

        construction NeitherAmountCoordination: Amount {
            form [marker: lexical(Determinative), " ", left: Amount, " ", coordinator: lexical(Coordinator), " ", right: Amount];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
        }

        category CorrelativeAmountSeries(CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeAmountSeriesEnd: CorrelativeAmountSeries {
            form [left: Amount, ", ", coordinator: lexical(Coordinator), " ", right: Amount];
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeAmountSeriesContinuation: CorrelativeAmountSeries {
            form [left: Amount, ", ", rest: CorrelativeAmountSeries];
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherAmountCoordination: Amount {
            form [marker: lexical(Determinative), " ", left: Amount, ", ", rest: CorrelativeAmountSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
        }

        construction SerialNeitherAmountCoordination: Amount {
            form [marker: lexical(Determinative), " ", left: Amount, ", ", rest: CorrelativeAmountSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
        }

        construction BothMeasurePhraseCoordination: MeasurePhrase {
            form [marker: lexical(Determinative), " ", left: MeasurePhrase, " ", coordinator: lexical(Coordinator), " ", right: MeasurePhrase];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            agree left.MeasureKind = right.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        construction EitherMeasurePhraseCoordination: MeasurePhrase {
            form [marker: lexical(Determinative), " ", left: MeasurePhrase, " ", coordinator: lexical(Coordinator), " ", right: MeasurePhrase];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            agree left.MeasureKind = right.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        construction NeitherMeasurePhraseCoordination: MeasurePhrase {
            form [marker: lexical(Determinative), " ", left: MeasurePhrase, " ", coordinator: lexical(Coordinator), " ", right: MeasurePhrase];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            agree left.MeasureKind = right.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        category CorrelativeMeasurePhraseSeries(MeasureKind, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeMeasurePhraseSeriesEnd: CorrelativeMeasurePhraseSeries {
            form [left: MeasurePhrase, ", ", coordinator: lexical(Coordinator), " ", right: MeasurePhrase];
            agree left.MeasureKind = right.MeasureKind;
            export MeasureKind = left.MeasureKind;
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeMeasurePhraseSeriesContinuation: CorrelativeMeasurePhraseSeries {
            form [left: MeasurePhrase, ", ", rest: CorrelativeMeasurePhraseSeries];
            agree left.MeasureKind = rest.MeasureKind;
            export MeasureKind = left.MeasureKind;
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherMeasurePhraseCoordination: MeasurePhrase {
            form [marker: lexical(Determinative), " ", left: MeasurePhrase, ", ", rest: CorrelativeMeasurePhraseSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            agree left.MeasureKind = rest.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        construction SerialNeitherMeasurePhraseCoordination: MeasurePhrase {
            form [marker: lexical(Determinative), " ", left: MeasurePhrase, ", ", rest: CorrelativeMeasurePhraseSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            agree left.MeasureKind = rest.MeasureKind;
            export MeasureKind = left.MeasureKind;
        }

        construction BothKeywordPhraseCoordination: KeywordPhrase {
            form [marker: lexical(Determinative), " ", left: KeywordPhrase, " ", coordinator: lexical(Coordinator), " ", right: KeywordPhrase];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
        }

        construction EitherKeywordPhraseCoordination: KeywordPhrase {
            form [marker: lexical(Determinative), " ", left: KeywordPhrase, " ", coordinator: lexical(Coordinator), " ", right: KeywordPhrase];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
        }

        construction NeitherKeywordPhraseCoordination: KeywordPhrase {
            form [marker: lexical(Determinative), " ", left: KeywordPhrase, " ", coordinator: lexical(Coordinator), " ", right: KeywordPhrase];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
        }

        category CorrelativeKeywordPhraseSeries(CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeKeywordPhraseSeriesEnd: CorrelativeKeywordPhraseSeries {
            form [left: KeywordPhrase, ", ", coordinator: lexical(Coordinator), " ", right: KeywordPhrase];
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeKeywordPhraseSeriesContinuation: CorrelativeKeywordPhraseSeries {
            form [left: KeywordPhrase, ", ", rest: CorrelativeKeywordPhraseSeries];
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherKeywordPhraseCoordination: KeywordPhrase {
            form [marker: lexical(Determinative), " ", left: KeywordPhrase, ", ", rest: CorrelativeKeywordPhraseSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
        }

        construction SerialNeitherKeywordPhraseCoordination: KeywordPhrase {
            form [marker: lexical(Determinative), " ", left: KeywordPhrase, ", ", rest: CorrelativeKeywordPhraseSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
        }

        construction BothQuotedTextCoordination: QuotedText {
            form [marker: lexical(Determinative), " ", left: QuotedText, " ", coordinator: lexical(Coordinator), " ", right: QuotedText];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
        }

        construction EitherQuotedTextCoordination: QuotedText {
            form [marker: lexical(Determinative), " ", left: QuotedText, " ", coordinator: lexical(Coordinator), " ", right: QuotedText];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
        }

        construction NeitherQuotedTextCoordination: QuotedText {
            form [marker: lexical(Determinative), " ", left: QuotedText, " ", coordinator: lexical(Coordinator), " ", right: QuotedText];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
        }

        category CorrelativeQuotedTextSeries(CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeQuotedTextSeriesEnd: CorrelativeQuotedTextSeries {
            form [left: QuotedText, ", ", coordinator: lexical(Coordinator), " ", right: QuotedText];
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeQuotedTextSeriesContinuation: CorrelativeQuotedTextSeries {
            form [left: QuotedText, ", ", rest: CorrelativeQuotedTextSeries];
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherQuotedTextCoordination: QuotedText {
            form [marker: lexical(Determinative), " ", left: QuotedText, ", ", rest: CorrelativeQuotedTextSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
        }

        construction SerialNeitherQuotedTextCoordination: QuotedText {
            form [marker: lexical(Determinative), " ", left: QuotedText, ", ", rest: CorrelativeQuotedTextSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
        }

        construction BothInfinitiveComplementCoordination: InfinitiveComplement {
            form [marker: lexical(Determinative), " ", left: InfinitiveComplement, " ", coordinator: lexical(Coordinator), " ", right: InfinitiveComplement];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        construction EitherInfinitiveComplementCoordination: InfinitiveComplement {
            form [marker: lexical(Determinative), " ", left: InfinitiveComplement, " ", coordinator: lexical(Coordinator), " ", right: InfinitiveComplement];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        construction NeitherInfinitiveComplementCoordination: InfinitiveComplement {
            form [marker: lexical(Determinative), " ", left: InfinitiveComplement, " ", coordinator: lexical(Coordinator), " ", right: InfinitiveComplement];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        category CorrelativeInfinitiveComplementSeries(Voice, OvertHead, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeInfinitiveComplementSeriesEnd: CorrelativeInfinitiveComplementSeries {
            form [left: InfinitiveComplement, ", ", coordinator: lexical(Coordinator), " ", right: InfinitiveComplement];
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeInfinitiveComplementSeriesContinuation: CorrelativeInfinitiveComplementSeries {
            form [left: InfinitiveComplement, ", ", rest: CorrelativeInfinitiveComplementSeries];
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherInfinitiveComplementCoordination: InfinitiveComplement {
            form [marker: lexical(Determinative), " ", left: InfinitiveComplement, ", ", rest: CorrelativeInfinitiveComplementSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        construction SerialNeitherInfinitiveComplementCoordination: InfinitiveComplement {
            form [marker: lexical(Determinative), " ", left: InfinitiveComplement, ", ", rest: CorrelativeInfinitiveComplementSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            export Voice = coordinated_voice(left.Voice, rest.Voice);
            export OvertHead = overt_predicates(left.OvertHead, rest.OvertHead);
        }

        construction BothFiniteSelectedHeadCoordination: FiniteSelectedHead {
            form [marker: lexical(Determinative), " ", left: FiniteSelectedHead, " ", coordinator: lexical(Coordinator), " ", right: FiniteSelectedHead];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction EitherFiniteSelectedHeadCoordination: FiniteSelectedHead {
            form [marker: lexical(Determinative), " ", left: FiniteSelectedHead, " ", coordinator: lexical(Coordinator), " ", right: FiniteSelectedHead];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction NeitherFiniteSelectedHeadCoordination: FiniteSelectedHead {
            form [marker: lexical(Determinative), " ", left: FiniteSelectedHead, " ", coordinator: lexical(Coordinator), " ", right: FiniteSelectedHead];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        category CorrelativeFiniteSelectedHeadSeries(number, person, FrameUse, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeFiniteSelectedHeadSeriesEnd: CorrelativeFiniteSelectedHeadSeries {
            form [left: FiniteSelectedHead, ", ", coordinator: lexical(Coordinator), " ", right: FiniteSelectedHead];
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeFiniteSelectedHeadSeriesContinuation: CorrelativeFiniteSelectedHeadSeries {
            form [left: FiniteSelectedHead, ", ", rest: CorrelativeFiniteSelectedHeadSeries];
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherFiniteSelectedHeadCoordination: FiniteSelectedHead {
            form [marker: lexical(Determinative), " ", left: FiniteSelectedHead, ", ", rest: CorrelativeFiniteSelectedHeadSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction SerialNeitherFiniteSelectedHeadCoordination: FiniteSelectedHead {
            form [marker: lexical(Determinative), " ", left: FiniteSelectedHead, ", ", rest: CorrelativeFiniteSelectedHeadSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction BothSecondarySelectedHeadCoordination: SecondarySelectedHead {
            form [marker: lexical(Determinative), " ", left: SecondarySelectedHead, " ", coordinator: lexical(Coordinator), " ", right: SecondarySelectedHead];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            agree left.form = right.form;
            export form = left.form;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction EitherSecondarySelectedHeadCoordination: SecondarySelectedHead {
            form [marker: lexical(Determinative), " ", left: SecondarySelectedHead, " ", coordinator: lexical(Coordinator), " ", right: SecondarySelectedHead];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            agree left.form = right.form;
            export form = left.form;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction NeitherSecondarySelectedHeadCoordination: SecondarySelectedHead {
            form [marker: lexical(Determinative), " ", left: SecondarySelectedHead, " ", coordinator: lexical(Coordinator), " ", right: SecondarySelectedHead];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            agree left.form = right.form;
            export form = left.form;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        category CorrelativeSecondarySelectedHeadSeries(form, FrameUse, CorrelativeCoordinator, CoordinationKind);
        construction CorrelativeSecondarySelectedHeadSeriesEnd: CorrelativeSecondarySelectedHeadSeries {
            form [left: SecondarySelectedHead, ", ", coordinator: lexical(Coordinator), " ", right: SecondarySelectedHead];
            agree left.form = right.form;
            export form = left.form;
            agree left.FrameUse = right.FrameUse;
            export FrameUse = left.FrameUse;
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
            export CoordinationKind = coordinator.CoordinationKind;
        }

        construction CorrelativeSecondarySelectedHeadSeriesContinuation: CorrelativeSecondarySelectedHeadSeries {
            form [left: SecondarySelectedHead, ", ", rest: CorrelativeSecondarySelectedHeadSeries];
            agree left.form = rest.form;
            export form = left.form;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
            export CoordinationKind = rest.CoordinationKind;
        }

        construction SerialEitherSecondarySelectedHeadCoordination: SecondarySelectedHead {
            form [marker: lexical(Determinative), " ", left: SecondarySelectedHead, ", ", rest: CorrelativeSecondarySelectedHeadSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            agree left.form = rest.form;
            export form = left.form;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }

        construction SerialNeitherSecondarySelectedHeadCoordination: SecondarySelectedHead {
            form [marker: lexical(Determinative), " ", left: SecondarySelectedHead, ", ", rest: CorrelativeSecondarySelectedHeadSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            agree left.form = rest.form;
            export form = left.form;
            agree left.FrameUse = rest.FrameUse;
            export FrameUse = left.FrameUse;
            export HeadCoordination = Yes;
        }



        feature NominalLicense { AnyNominal }

        construction PossessiveNounPhrase: NounPhrase {
            form [possessor: lexical(Pronoun), " ", head: Nominal];
            require possessor.case = Genitive;
            require possessor.NominalLicense = AnyNominal;
            export number = head.number;
            export person = Third;
            export CaseUse = Common;
        }


        feature KeywordComplement { Yes }

        construction KeywordComplementPreposition: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: KeywordPhrase];
            require head.KeywordComplement = Yes;
            export LocativeUse = head.LocativeUse;
            export AdverbialUse = head.AdverbialUse;
        }


        construction FiniteSelectedCardinalHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = CardinalComplement;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Cardinal;
            export HeadCoordination = No;
        }

        construction SecondarySelectedCardinalHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = CardinalComplement;
            export form = secondary_form(head.form);
            export FrameUse = Cardinal;
            export HeadCoordination = No;
        }

        construction FiniteSelectedInfinitiveHead: FiniteSelectedHead {
            form [head: lexical(Verb)];
            require head.frame = InfinitiveSelection;
            require head.finiteness = Finite;
            export number = head.number;
            export person = head.person;
            export FrameUse = Infinitive;
            export HeadCoordination = No;
        }

        construction SecondarySelectedInfinitiveHead: SecondarySelectedHead {
            form [head: lexical(Verb)];
            require head.frame = InfinitiveSelection;
            export form = secondary_form(head.form);
            export FrameUse = Infinitive;
            export HeadCoordination = No;
        }

        construction FiniteSharedCardinalComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", complement: Cardinal];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Cardinal;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction SecondarySharedCardinalComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", complement: Cardinal];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Cardinal;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        construction FiniteSharedInfinitiveComplement: FinitePredicate {
            form [head: FiniteSelectedHead, " ", complement: InfinitiveComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Infinitive;
            export number = head.number;
            export person = head.person;
            export Voice = Active;
        }

        construction SecondarySharedInfinitiveComplement: SecondaryVerbPhrase {
            form [head: SecondarySelectedHead, " ", complement: InfinitiveComplement];
            require head.HeadCoordination = Yes;
            require head.FrameUse = Infinitive;
            export form = head.form;
            export OvertHead = Yes;
            export Voice = Active;
        }

        category CorrelativeAdjectivePhrase();
        category CorrelativeAdjectiveSeries(CorrelativeCoordinator);

        construction BothAdjectives: CorrelativeAdjectivePhrase {
            form [marker: lexical(Determinative), " ", left: AdjectivePhrase, " ", coordinator: lexical(Coordinator), " ", right: AdjectivePhrase];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
        }

        construction EitherAdjectives: CorrelativeAdjectivePhrase {
            form [marker: lexical(Determinative), " ", left: AdjectivePhrase, " ", coordinator: lexical(Coordinator), " ", right: AdjectivePhrase];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
        }

        construction NeitherAdjectives: CorrelativeAdjectivePhrase {
            form [marker: lexical(Determinative), " ", left: AdjectivePhrase, " ", coordinator: lexical(Coordinator), " ", right: AdjectivePhrase];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
        }

        construction CorrelativeAdjectiveSeriesEnd: CorrelativeAdjectiveSeries {
            form [left: AdjectivePhrase, ", ", coordinator: lexical(Coordinator), " ", right: AdjectivePhrase];
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
        }

        construction CorrelativeAdjectiveSeriesContinuation: CorrelativeAdjectiveSeries {
            form [left: AdjectivePhrase, ", ", rest: CorrelativeAdjectiveSeries];
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
        }

        construction SerialEitherAdjectives: CorrelativeAdjectivePhrase {
            form [marker: lexical(Determinative), " ", left: AdjectivePhrase, ", ", rest: CorrelativeAdjectiveSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
        }

        construction SerialNeitherAdjectives: CorrelativeAdjectivePhrase {
            form [marker: lexical(Determinative), " ", left: AdjectivePhrase, ", ", rest: CorrelativeAdjectiveSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
        }

        construction AdjectivalCorrelativeComplement: PredicativeComplement {
            form [phrase: CorrelativeAdjectivePhrase];
            export PredicativeKind = Adjectival;
        }

        construction CorrelativePostpositiveNominal: Nominal {
            form [head: Nominal, " ", modifier: CorrelativeAdjectivePhrase];
            export number = head.number;
            export countability = head.countability;
            export Targeting = head.Targeting;
        }


        construction BothFiniteObjectGapCoordination: FiniteObjectGap {
            form [marker: lexical(Determinative), " ", left: FiniteObjectGap, " ", coordinator: lexical(Coordinator), " ", right: FiniteObjectGap];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
        }

        construction EitherFiniteObjectGapCoordination: FiniteObjectGap {
            form [marker: lexical(Determinative), " ", left: FiniteObjectGap, " ", coordinator: lexical(Coordinator), " ", right: FiniteObjectGap];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
        }

        construction NeitherFiniteObjectGapCoordination: FiniteObjectGap {
            form [marker: lexical(Determinative), " ", left: FiniteObjectGap, " ", coordinator: lexical(Coordinator), " ", right: FiniteObjectGap];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
        }

        category CorrelativeFiniteObjectGapSeries(number, person, CorrelativeCoordinator);

        construction CorrelativeFiniteObjectGapSeriesEnd: CorrelativeFiniteObjectGapSeries {
            form [left: FiniteObjectGap, ", ", coordinator: lexical(Coordinator), " ", right: FiniteObjectGap];
            agree left.number = right.number;
            export number = left.number;
            agree left.person = right.person;
            export person = left.person;
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
        }

        construction CorrelativeFiniteObjectGapSeriesContinuation: CorrelativeFiniteObjectGapSeries {
            form [left: FiniteObjectGap, ", ", rest: CorrelativeFiniteObjectGapSeries];
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
        }

        construction SerialEitherFiniteObjectGapCoordination: FiniteObjectGap {
            form [marker: lexical(Determinative), " ", left: FiniteObjectGap, ", ", rest: CorrelativeFiniteObjectGapSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
        }

        construction SerialNeitherFiniteObjectGapCoordination: FiniteObjectGap {
            form [marker: lexical(Determinative), " ", left: FiniteObjectGap, ", ", rest: CorrelativeFiniteObjectGapSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
            agree left.number = rest.number;
            export number = left.number;
            agree left.person = rest.person;
            export person = left.person;
        }

        construction BothBareObjectGapCoordination: BareObjectGap {
            form [marker: lexical(Determinative), " ", left: BareObjectGap, " ", coordinator: lexical(Coordinator), " ", right: BareObjectGap];
            require marker.CorrelativeKind = Both;
            require coordinator.CorrelativeCoordinator = And;
        }

        construction EitherBareObjectGapCoordination: BareObjectGap {
            form [marker: lexical(Determinative), " ", left: BareObjectGap, " ", coordinator: lexical(Coordinator), " ", right: BareObjectGap];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
        }

        construction NeitherBareObjectGapCoordination: BareObjectGap {
            form [marker: lexical(Determinative), " ", left: BareObjectGap, " ", coordinator: lexical(Coordinator), " ", right: BareObjectGap];
            require marker.CorrelativeKind = Neither;
            require coordinator.CorrelativeCoordinator = Nor;
        }

        category CorrelativeBareObjectGapSeries(CorrelativeCoordinator);

        construction CorrelativeBareObjectGapSeriesEnd: CorrelativeBareObjectGapSeries {
            form [left: BareObjectGap, ", ", coordinator: lexical(Coordinator), " ", right: BareObjectGap];
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
        }

        construction CorrelativeBareObjectGapSeriesContinuation: CorrelativeBareObjectGapSeries {
            form [left: BareObjectGap, ", ", rest: CorrelativeBareObjectGapSeries];
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
        }

        construction SerialEitherBareObjectGapCoordination: BareObjectGap {
            form [marker: lexical(Determinative), " ", left: BareObjectGap, ", ", rest: CorrelativeBareObjectGapSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
        }

        construction SerialNeitherBareObjectGapCoordination: BareObjectGap {
            form [marker: lexical(Determinative), " ", left: BareObjectGap, ", ", rest: CorrelativeBareObjectGapSeries];
            require marker.CorrelativeKind = Neither;
            require rest.CorrelativeCoordinator = Nor;
        }


        category SelectedPrepositionHead(LocativeUse, AdverbialUse, HeadCoordination);
        category SelectedPrepositionHeadSeries(LocativeUse, AdverbialUse);

        construction SelectedPrepositionHead: SelectedPrepositionHead {
            form [head: lexical(Preposition)];
            require head.PrepositionComplement = NounPhrase;
            export LocativeUse = head.LocativeUse;
            export AdverbialUse = head.AdverbialUse;
            export HeadCoordination = No;
        }


        construction SelectedPrepositionHeadCoordination: SelectedPrepositionHead {
            form [left: SelectedPrepositionHead, " ", coordinator: lexical(Coordinator), " ", right: SelectedPrepositionHead];
            require coordinator.NoncorrelativeCoordination = Yes;
            export LocativeUse = coordinated_locative_use(left.LocativeUse, right.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, right.AdverbialUse);
            export HeadCoordination = Yes;
        }

        construction SelectedPrepositionHeadSeriesEnd: SelectedPrepositionHeadSeries {
            form [left: SelectedPrepositionHead, ", ", coordinator: lexical(Coordinator), " ", right: SelectedPrepositionHead];
            require coordinator.NoncorrelativeCoordination = Yes;
            export LocativeUse = coordinated_locative_use(left.LocativeUse, right.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, right.AdverbialUse);
        }

        construction SelectedPrepositionHeadSeriesContinuation: SelectedPrepositionHeadSeries {
            form [left: SelectedPrepositionHead, ", ", rest: SelectedPrepositionHeadSeries];
            export LocativeUse = coordinated_locative_use(left.LocativeUse, rest.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, rest.AdverbialUse);
        }

        construction SerialSelectedPrepositionHead: SelectedPrepositionHead {
            form [left: SelectedPrepositionHead, ", ", rest: SelectedPrepositionHeadSeries];
            export LocativeUse = coordinated_locative_use(left.LocativeUse, rest.LocativeUse);
            export AdverbialUse = coordinated_adverbial_use(left.AdverbialUse, rest.AdverbialUse);
            export HeadCoordination = Yes;
        }

        construction SharedPrepositionComplement: PrepositionPhrase {
            form [head: SelectedPrepositionHead, " ", complement: AccusativePhrase];
            require head.HeadCoordination = Yes;
            export LocativeUse = head.LocativeUse;
            export AdverbialUse = head.AdverbialUse;
        }


        construction EitherCoordinatedFiniteClause: CoordinatedFiniteClause {
            form [marker: lexical(Determinative), " ", left: FiniteClause, " ", coordinator: lexical(Coordinator), " ", right: FiniteClause];
            require marker.CorrelativeKind = Either;
            require coordinator.CorrelativeCoordinator = Or;
        }
        category CorrelativeFiniteClauseSeries(CorrelativeCoordinator);
        construction CorrelativeFiniteClauseSeriesEnd: CorrelativeFiniteClauseSeries {
            form [left: FiniteClause, ", ", coordinator: lexical(Coordinator), " ", right: FiniteClause];
            export CorrelativeCoordinator = coordinator.CorrelativeCoordinator;
        }
        construction CorrelativeFiniteClauseSeriesContinuation: CorrelativeFiniteClauseSeries {
            form [left: FiniteClause, ", ", rest: CorrelativeFiniteClauseSeries];
            export CorrelativeCoordinator = rest.CorrelativeCoordinator;
        }
        construction SerialEitherCoordinatedFiniteClause: CoordinatedFiniteClause {
            form [marker: lexical(Determinative), " ", left: FiniteClause, ", ", rest: CorrelativeFiniteClauseSeries];
            require marker.CorrelativeKind = Either;
            require rest.CorrelativeCoordinator = Or;
        }

    }
}
