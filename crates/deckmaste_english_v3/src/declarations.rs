//! Connected Oracle English declarations under whole-grammar activation.
//!
//! Lexical distribution and complete frame signatures govern composition.
//! The activation ticket records the remaining families and corpus obligations.

use deckmaste_construction_v3::constructions;

constructions! {
    pub mod grammar {
        feature DeterminerUse { SingularCount, Unrestricted, PluralOrMass, PluralCount, Mass, Singular }
        feature DeterminerKind { Ordinary, Indefinite }
        feature CaseUse { Common, Nominative, Accusative }
        feature Targeting { No, Yes }
        feature CoordinationKind { Additive, Alternative, Adversative }
        feature PrepositionComplement { NounPhrase }
        feature AdverbialSubordinator { Yes }
        feature RelativeSubordinator { Yes }
        feature Voice { Active, Passive, Mixed }
        feature OvertHead { No, Yes }
        feature ScalarVariable { Yes }
        feature MeasureOperator { Yes }
        feature MeasurePreposition { Yes }
        feature MeasurePosition { Before, After }
        feature ComparisonMarker { Equality, Ordering }

        category Document();
        category Ability();
        category AbilityContinuation();
        category Paragraph();
        category SentenceContinuation();
        category Sentence();
        category Clause();
        category FiniteClause();
        category SubordinateClause();
        category FinitePredicate(number, person, Voice);
        category NonfinitePredicate(form, Voice, OvertHead);
        category BarePredicate(Voice, OvertHead);
        category ParticipialPredicate(Voice, OvertHead);
        category PastParticiplePredicate(Voice, OvertHead);
        category PredicativeComplement();
        category Ellipsis(form, Voice);
        category BareComplement(Voice);
        category ParticipialComplement(Voice);
        category PerfectComplement(Voice);
        category Nominal(number, countability, Targeting);
        category NounPhrase(number, person, CaseUse);
        category NominativePhrase(number, person, CaseUse);
        category AccusativePhrase(number, person, CaseUse);
        category AdjectivePhrase();
        category PrepositionPhrase();
        category SubjectRelativeClause(number);
        category ObjectRelativeClause();
        category FiniteObjectGap(number, person);
        category BareObjectGap();
        category Cardinal(number);
        category MeasurePhrase();
        category EqualityComplement();
        category OrderingComplement();

        frame Intransitive = "(kind: \"Predicate\", items: [])";
        frame Transitive = "(kind: \"Predicate\", items: [Argument((relation: Object, category: \"NounPhrase\"))])";
        frame Predicative = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"PredicativeComplement\"))])";
        frame BareAuxiliary = "(kind: \"Auxiliary\", items: [Argument((relation: Complement, category: \"BarePredicate\"))])";
        frame ParticipialAuxiliary = "(kind: \"Auxiliary\", items: [Argument((relation: Complement, category: \"ParticipialPredicate\"))])";
        frame PerfectAuxiliary = "(kind: \"Auxiliary\", items: [Argument((relation: Complement, category: \"PastParticiplePredicate\"))])";

        frame Measure = "(kind: \"Predicate\", items: [Argument((relation: Complement, category: \"MeasurePhrase\"))])";
        frame Equality = "(kind: \"Predicate\", items: [Marked(vocabulary: \"Preposition\", member: \"To\", slot: (relation: Complement, category: \"MeasurePhrase\"))])";
        frame Ordering = "(kind: \"Predicate\", items: [Marked(vocabulary: \"Preposition\", member: \"Than\", slot: (relation: Complement, category: \"MeasurePhrase\"))])";
        frame ObjectEquality = "(kind: \"Predicate\", items: [Argument((relation: Object, category: \"NounPhrase\")), Argument((relation: Complement, category: \"ScalarEquality\"))])";

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
            form [first: Sentence, rest: repeat(SentenceContinuation, "")];
        }

        construction SentenceContinuation: SentenceContinuation {
            form [" ", sentence: Sentence];
        }

        construction Sentence: Sentence {
            form [clause: Clause, "."];
        }

        construction Declarative: Clause {
            form [clause: FiniteClause];
        }

        construction Imperative: Clause {
            form [predicate: BarePredicate];
            require predicate.OvertHead = Yes;
        }

        construction FiniteClause: FiniteClause {
            form [subject: NominativePhrase, " ", predicate: FinitePredicate];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
        }

        construction InitialSubordinate: Clause {
            form [dependent: SubordinateClause, ", ", clause: Clause];
        }

        construction FinalSubordinate: Clause {
            form [clause: Clause, " ", dependent: SubordinateClause];
        }

        construction SubordinateClause: SubordinateClause {
            form [marker: lexical(Subordinator), " ", clause: FiniteClause];
            require marker.AdverbialSubordinator = Yes;
        }

        construction InitialPreposition: Clause {
            form [dependent: PrepositionPhrase, ", ", clause: Clause];
        }

        construction ClausalPreposition: Clause {
            form [clause: Clause, " ", dependent: PrepositionPhrase];
        }

        construction ClauseCoordination: Clause {
            form [left: Clause, ", ", coordinator: lexical(Coordinator), " ", right: Clause];
        }

        construction Noun: Nominal {
            form [head: lexical(Noun)];
            export number = head.number;
            export countability = head.countability;
            export Targeting = No;
        }

        construction Adjective: AdjectivePhrase {
            form [head: lexical(Adjective)];
            require head.framing = Unframed;
        }

        construction IntransitiveAdjective: AdjectivePhrase {
            form [head: lexical(Adjective)];
            require head.frame = Intransitive;
        }

        construction PremodifiedNominal: Nominal {
            form [modifier: AdjectivePhrase, " ", head: Nominal];
            require head.Targeting = No;
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

        construction PostmodifiedNounPhrase: NounPhrase {
            form [head: NounPhrase, " ", modifier: PrepositionPhrase];
            export number = head.number;
            export person = head.person;
            export CaseUse = head.CaseUse;
        }

        construction AdditiveNounPhrase: NounPhrase {
            form [left: NounPhrase, " ", coordinator: lexical(Coordinator), " ", right: NounPhrase];
            require coordinator.CoordinationKind = Additive;
            export number = Plural;
            export person = additive_person(left.person, right.person);
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
        }

        construction AlternativeNounPhrase: NounPhrase {
            form [left: NounPhrase, " ", coordinator: lexical(Coordinator), " ", right: NounPhrase];
            require coordinator.CoordinationKind = Alternative;
            export number = right.number;
            export person = right.person;
            export CaseUse = common_case(left.CaseUse, right.CaseUse);
        }

        construction PrepositionPhrase: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: AccusativePhrase];
            require head.PrepositionComplement = NounPhrase;
        }

        construction AdjectivalComplement: PredicativeComplement {
            form [phrase: AdjectivePhrase];
        }

        construction NominalComplement: PredicativeComplement {
            form [phrase: AccusativePhrase];
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

        construction FinitePreposition: FinitePredicate {
            form [head: FinitePredicate, " ", modifier: PrepositionPhrase];
            export number = head.number;
            export person = head.person;
            export Voice = head.Voice;
        }

        construction NonfiniteIntransitive: NonfinitePredicate {
            form [head: lexical(Verb)];
            require head.finiteness = Nonfinite;
            require head.frame = Intransitive;
            export form = head.form;
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction NonfiniteTransitive: NonfinitePredicate {
            form [head: lexical(Verb), " ", object: AccusativePhrase];
            require head.finiteness = Nonfinite;
            require head.frame = Transitive;
            export form = head.form;
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction NonfinitePredicative: NonfinitePredicate {
            form [head: lexical(Verb), " ", complement: PredicativeComplement];
            require head.finiteness = Nonfinite;
            require head.frame = Predicative;
            export form = head.form;
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction NonfiniteBareAuxiliary: NonfinitePredicate {
            form [head: lexical(Verb), complement: BareComplement];
            require head.finiteness = Nonfinite;
            require head.frame = BareAuxiliary;
            export form = head.form;
            export Voice = complement.Voice;
            export OvertHead = Yes;
        }

        construction NonfiniteParticipialAuxiliary: NonfinitePredicate {
            form [head: lexical(Verb), complement: ParticipialComplement];
            require head.finiteness = Nonfinite;
            require head.frame = ParticipialAuxiliary;
            export form = head.form;
            export Voice = complement.Voice;
            export OvertHead = Yes;
        }

        construction NonfinitePerfectAuxiliary: NonfinitePredicate {
            form [head: lexical(Verb), complement: PerfectComplement];
            require head.finiteness = Nonfinite;
            require head.frame = PerfectAuxiliary;
            export form = head.form;
            export Voice = complement.Voice;
            export OvertHead = Yes;
        }

        construction NonfinitePreposition: NonfinitePredicate {
            form [head: NonfinitePredicate, " ", modifier: PrepositionPhrase];
            export form = head.form;
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
        }

        construction NonfinitePredicateCoordination: NonfinitePredicate {
            form [left: NonfinitePredicate, " ", coordinator: lexical(Coordinator), " ", right: NonfinitePredicate];
            agree left.form = right.form;
            export form = left.form;
            export Voice = coordinated_voice(left.Voice, right.Voice);
            export OvertHead = overt_predicates(left.OvertHead, right.OvertHead);
        }

        construction BarePredicate: BarePredicate {
            form [head: NonfinitePredicate];
            require head.form = Plain;
            export Voice = head.Voice;
            export OvertHead = head.OvertHead;
        }

        construction ProgressiveComplement: ParticipialPredicate {
            form [head: NonfinitePredicate];
            require head.form = GerundParticiple;
            export Voice = head.Voice;
            export OvertHead = head.OvertHead;
        }

        construction PassiveComplement: ParticipialPredicate {
            form [head: NonfinitePredicate];
            require head.form = PastParticiple;
            require head.Voice = Passive;
            export Voice = Passive;
            export OvertHead = head.OvertHead;
        }

        construction PerfectComplement: PastParticiplePredicate {
            form [head: NonfinitePredicate];
            require head.form = PastParticiple;
            export Voice = head.Voice;
            export OvertHead = head.OvertHead;
        }

        construction PassivePredicate: NonfinitePredicate {
            form [head: lexical(Verb)];
            require head.form = PastParticiple;
            require head.finiteness = Nonfinite;
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
            require head.finiteness = Nonfinite;
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
        }

        construction GroupedScalarNumeral: MeasurePhrase {
            form [head: lexical(Numeral)];
            require head.numeral_kind = GroupedArabic;
        }

        construction UngroupedScalarNumeral: MeasurePhrase {
            form [head: lexical(Numeral)];
            require head.numeral_kind = Arabic;
            require head.numeral_size = Small;
        }

        construction ScalarVariable: MeasurePhrase {
            form [head: lexical(Numeral)];
            require head.ScalarVariable = Yes;
        }

        construction ArithmeticMeasure: MeasurePhrase {
            form [left: MeasurePhrase, " ", operator: lexical(Preposition), " ", right: MeasurePhrase];
            require operator.MeasureOperator = Yes;
        }

        construction MeasuredPreposition: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: MeasurePhrase];
            require head.MeasurePreposition = Yes;
        }

        construction MeasuredNounPhrase: NounPhrase {
            form [quantity: MeasurePhrase, " ", head: lexical(Noun)];
            require head.MeasurePosition = Before;
            require head.number = Singular;
            require head.countability = Mass;
            export number = Singular;
            export person = Third;
            export CaseUse = Common;
        }

        construction MeasuredAttribute: NounPhrase {
            form [head: lexical(Noun), " ", quantity: MeasurePhrase];
            require head.MeasurePosition = After;
            require head.number = Singular;
            export number = Singular;
            export person = Third;
            export CaseUse = Common;
        }

        construction EqualityComplement: EqualityComplement {
            form [head: lexical(Adjective), " ", marker: lexical(Preposition), " ", measure: MeasurePhrase];
            require head.frame = Equality;
            require marker.ComparisonMarker = Equality;
        }

        construction EqualityAdjective: AdjectivePhrase {
            form [complement: EqualityComplement];
        }

        construction OrderingComplement: OrderingComplement {
            form [head: lexical(Adjective), " ", marker: lexical(Preposition), " ", measure: MeasurePhrase];
            require head.frame = Ordering;
            require marker.ComparisonMarker = Ordering;
        }

        construction OrderingAdjective: AdjectivePhrase {
            form [complement: OrderingComplement];
        }

        construction FiniteMeasure: FinitePredicate {
            form [head: lexical(Verb), " ", measure: MeasurePhrase];
            require head.finiteness = Finite;
            require head.frame = Measure;
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

        construction NonfiniteMeasure: NonfinitePredicate {
            form [head: lexical(Verb), " ", measure: MeasurePhrase];
            require head.finiteness = Nonfinite;
            require head.frame = Measure;
            export form = head.form;
            export Voice = Active;
            export OvertHead = Yes;
        }

        construction NonfiniteObjectEquality: NonfinitePredicate {
            form [head: lexical(Verb), " ", object: AccusativePhrase, " ", complement: EqualityComplement];
            require head.finiteness = Nonfinite;
            require head.frame = ObjectEquality;
            export form = head.form;
            export Voice = Active;
            export OvertHead = Yes;
        }
    }
}
