use std::fmt;
use std::ops::Deref;

use deckmaste_english::CatalogKind;
use deckmaste_english::Catalogs;
use deckmaste_english::Numeral;
use deckmaste_english::parse_with_catalogs;
use deckmaste_english::parse_with_identity;
use deckmaste_english::syntax::*;
use deckmaste_english::word::*;
use serde::Serialize;
use serde::ser;

#[test]
fn feature_vocabulary() {
    let person: deckmaste_english::word::Person = deckmaste_english::features::Person::Second;
    let _: deckmaste_english::features::Person = person;

    let number: deckmaste_english::word::Number = deckmaste_english::features::Number::Plural;
    let _: deckmaste_english::features::Number = number;

    let verb_slot: deckmaste_english::word::VerbSlot =
        deckmaste_english::features::VerbSlot::Present { person, number };
    let _: deckmaste_english::features::VerbSlot = verb_slot;

    let cardinality: deckmaste_english::syntax::NounCardinality =
        deckmaste_english::features::NounCardinality::PluralCount;
    let _: deckmaste_english::features::NounCardinality = cardinality;

    let onset: deckmaste_english::word::InitialSound = deckmaste_english::features::Onset::Vowel;
    let _: deckmaste_english::features::Onset = onset;

    let pronoun: deckmaste_english::word::Pronoun = deckmaste_english::features::PronounClass::They;
    let _: deckmaste_english::features::PronounClass = pronoun;

    let pronoun_case: deckmaste_english::word::PronounCase =
        deckmaste_english::features::PronounCase::Object;
    let _: deckmaste_english::features::PronounCase = pronoun_case;

    let gap: deckmaste_english::syntax::RelativeGap = deckmaste_english::features::GapState::Object;
    let _: deckmaste_english::features::GapState = gap;

    let canonical: deckmaste_english::features::Conjunction =
        deckmaste_english::syntax::PredicateConjunction::And;
    let nominal: deckmaste_english::syntax::NounPhraseConjunction = canonical;
    assert_eq!(nominal, deckmaste_english::features::Conjunction::And);
}

#[test]
fn existential_forms_expose_validated_features_and_keep_legacy_unit_names() {
    use deckmaste_english::features::Contraction;
    use deckmaste_english::features::Number;
    use deckmaste_english::features::Person;
    use deckmaste_english::features::VerbSlot;

    for (source, slot, contraction, variant) in [
        (
            "There is a creature.",
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Singular,
            },
            Contraction::Full,
            "Is",
        ),
        (
            "There's a creature.",
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Singular,
            },
            Contraction::Contracted,
            "ContractedIs",
        ),
        (
            "There are creatures.",
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Plural,
            },
            Contraction::Full,
            "Are",
        ),
        (
            "There was a creature.",
            VerbSlot::Past {
                person: Person::Third,
                number: Number::Singular,
            },
            Contraction::Full,
            "Was",
        ),
        (
            "There were creatures.",
            VerbSlot::Past {
                person: Person::Third,
                number: Number::Plural,
            },
            Contraction::Full,
            "Were",
        ),
    ] {
        let report = parse_with_catalogs(source, &Catalogs::default());
        let AbilityKind::Paragraph(paragraph) = &report.ast().abilities[0].kind else {
            panic!("expected a paragraph for {source:?}");
        };
        let SentenceBody::Independent(IndependentClause::Existential(existential)) =
            &paragraph.sentences[0].body
        else {
            panic!("expected an existential clause for {source:?}");
        };
        assert_eq!(existential.form.verb_slot(), slot, "{source}");
        assert_eq!(existential.form.contraction(), contraction, "{source}");
        assert_eq!(
            serialize_unit_variant(existential.form),
            variant,
            "{source}"
        );
    }

    assert_eq!(
        ExistentialForm::new(
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Plural,
            },
            Contraction::Contracted,
        ),
        None,
    );
}

#[test]
fn existential_legacy_value_paths_compile_and_match_as_constants() {
    const FORMS: [ExistentialForm; 5] = [
        ExistentialForm::Is,
        ExistentialForm::ContractedIs,
        ExistentialForm::Are,
        ExistentialForm::Was,
        ExistentialForm::Were,
    ];

    let names = FORMS.map(|form| match form {
        ExistentialForm::Is => "Is",
        ExistentialForm::ContractedIs => "ContractedIs",
        ExistentialForm::Are => "Are",
        ExistentialForm::Was => "Was",
        ExistentialForm::Were => "Were",
        _ => unreachable!("validated existential forms have exactly five values"),
    });

    assert_eq!(names, ["Is", "ContractedIs", "Are", "Was", "Were"]);
    assert_eq!(FORMS.map(serialize_unit_variant), names);
}

fn serialize_unit_variant(value: impl Serialize) -> &'static str {
    value
        .serialize(UnitVariantSerializer)
        .expect("the value must serialize as a unit variant")
}

#[derive(Debug)]
struct UnitVariantSerializationError;

impl fmt::Display for UnitVariantSerializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("expected a unit-variant serialization")
    }
}

impl std::error::Error for UnitVariantSerializationError {}

impl ser::Error for UnitVariantSerializationError {
    fn custom<T: fmt::Display>(_message: T) -> Self {
        Self
    }
}

struct UnitVariantSerializer;

macro_rules! unsupported_unit_variant_serialization {
    ($($name:ident($($argument:ident: $type:ty),*)),+ $(,)?) => {
        $(
            fn $name(self, $($argument: $type),*) -> Result<Self::Ok, Self::Error> {
                Err(UnitVariantSerializationError)
            }
        )+
    };
}

impl ser::Serializer for UnitVariantSerializer {
    type Ok = &'static str;
    type Error = UnitVariantSerializationError;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(variant)
    }

    unsupported_unit_variant_serialization!(
        serialize_bool(_value: bool),
        serialize_i8(_value: i8),
        serialize_i16(_value: i16),
        serialize_i32(_value: i32),
        serialize_i64(_value: i64),
        serialize_i128(_value: i128),
        serialize_u8(_value: u8),
        serialize_u16(_value: u16),
        serialize_u32(_value: u32),
        serialize_u64(_value: u64),
        serialize_u128(_value: u128),
        serialize_f32(_value: f32),
        serialize_f64(_value: f64),
        serialize_char(_value: char),
        serialize_str(_value: &str),
        serialize_bytes(_value: &[u8]),
        serialize_none(),
        serialize_unit(),
        serialize_unit_struct(_name: &'static str),
    );

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn serialize_seq(self, _length: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn serialize_tuple(self, _length: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn serialize_map(self, _length: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(UnitVariantSerializationError)
    }

    fn collect_str<T: ?Sized + fmt::Display>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(UnitVariantSerializationError)
    }
}

#[test]
fn public_parser_returns_a_source_independent_grammar_tree() {
    let source = String::from("Draw a card.");
    let report = parse_with_catalogs(&source, &Catalogs::default());

    assert!(report.diagnostics().is_empty());
    assert!(matches!(
        report.ast().abilities[0].kind,
        AbilityKind::Paragraph(ref paragraph)
            if matches!(
                paragraph.sentences[0].body,
                SentenceBody::Independent(IndependentClause::Imperative(
                    Predicate::Transitive(_)
                ))
            )
    ));

    let ast = report.into_ast();
    drop(source);

    assert_eq!(ast.render("Test Card", false).unwrap(), "Draw a card.");
}

#[test]
fn selected_constituents_expose_the_possessive_determiner() {
    let source = concat!(
        "Whenever another creature enters, this creature deals 1 damage ",
        "to that creature's controller."
    );
    let catalogs = Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"]);
    let report = parse_with_identity(source, &catalogs, "Rampaging Ferocidon", false);
    let constituents = report
        .provenance()
        .selections()
        .iter()
        .flat_map(deckmaste_english::ParseSelection::constituent_spans)
        .filter_map(|span| span.text(source))
        .collect::<Vec<_>>();

    assert!(constituents.contains(&"that creature's"));
    assert!(constituents.contains(&"creature's"));
    assert!(constituents.contains(&"controller"));
}

#[test]
fn selected_constituents_include_a_chart_lowered_quoted_ability() {
    let source = concat!(
        "This creature gains trample and ",
        "\"Whenever this creature attacks, draw a card.\""
    );
    let catalogs = Catalogs::default()
        .with_catalog(CatalogKind::CardType, ["Creature"])
        .with_catalog(CatalogKind::KeywordAbility, ["Trample"]);
    let report = parse_with_identity(source, &catalogs, "Test Card", false);
    let constituents = report
        .provenance()
        .selections()
        .iter()
        .flat_map(deckmaste_english::ParseSelection::constituent_spans)
        .filter_map(|span| span.text(source))
        .collect::<Vec<_>>();

    assert!(constituents.contains(&"this creature attacks"));
    assert!(constituents.contains(&"draw a card."));
}

#[test]
fn adversarial_shared_determiner_sentences_round_trip_without_recovery() {
    let catalogs = Catalogs::default()
        .with_catalog(
            CatalogKind::CardType,
            [
                "Artifact",
                "Battle",
                "Creature",
                "Enchantment",
                "Land",
                "Planeswalker",
            ],
        )
        .with_catalog(CatalogKind::CreatureType, ["Elf", "Orc"]);
    for source in [
        concat!(
            "Choose target creature. That creature gets +1/+1 and deals 3 damage to ",
            "target player or planeswalker and 1 damage to each creature that player ",
            "or that planeswalker controls."
        ),
        "Exile target artifact, creature, or planeswalker and target land or battle.",
        "Destroy target attacking or blocking creature.",
        "Put a +1/+1 counter on an Elf, Orc, or enchantment creature you control.",
    ] {
        let report = parse_with_catalogs(source, &catalogs);
        assert!(
            report.ast().recoveries().is_empty(),
            "{source}: {report:#?}"
        );
        assert_eq!(
            report.into_ast().render("Test Card", false).unwrap(),
            source,
            "{source}"
        );
    }
}

#[test]
fn rejected_keyword_list_does_not_leave_constituent_provenance() {
    let source = "Suspend 3, definitely not a keyword.";
    let catalogs = Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Suspend"]);
    let report = parse_with_catalogs(source, &catalogs);

    assert!(
        report
            .provenance()
            .selections()
            .iter()
            .all(|selection| selection.span().text(source) != Some("3")),
        "a rejected keyword-list probe leaked its counted-argument selection"
    );
}

/// Parses `source` as the named face and returns its rendered round-trip and
/// owned AST. The parse and render share the identity, so a recognized
/// self-reference re-emits the face's own name.
fn parse_face(source: &str, catalogs: &Catalogs, name: &str, legendary: bool) -> (String, TestAst) {
    let ast = parse_with_identity(source, catalogs, name, legendary).into_ast();
    let rendered = ast
        .render(name, legendary)
        .expect("named face should render");
    (rendered, TestAst(ast))
}

#[derive(Debug)]
struct TestAst(OracleText);

impl Deref for TestAst {
    type Target = OracleText;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for TestAst {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#?}", self.0)
    }
}

#[derive(Default)]
struct SyntaxInventory<'syntax> {
    noun_phrases: Vec<&'syntax NounPhrase>,
    nominals: Vec<&'syntax NominalPhrase>,
    nouns: Vec<&'syntax NounInstance>,
    adjectives: Vec<&'syntax Adjective>,
    predicate_heads: Vec<&'syntax PredicateHead>,
    prepositions: Vec<Preposition>,
    quantities: Vec<Quantity>,
    power_toughness: Vec<PowerToughness>,
    pronouns: Vec<Pronoun>,
    this_cards: Vec<ThisCardForm>,
    coordinated_adjectives: usize,
}

impl<'syntax> SyntaxInventory<'syntax> {
    fn from_ast(ast: &'syntax OracleText) -> Self {
        let mut inventory = Self::default();
        for ability in &ast.abilities {
            inventory.ability(ability);
        }
        inventory
    }

    fn has_noun(&self, predicate: impl Fn(&NounInstance) -> bool) -> bool {
        self.nouns.iter().copied().any(predicate)
    }

    fn has_noun_lexeme(&self, predicate: impl Fn(&Noun) -> bool) -> bool {
        self.nouns.iter().any(|instance| {
            let noun = match instance {
                NounInstance::Singular(noun)
                | NounInstance::Plural(noun)
                | NounInstance::Mass(noun) => noun,
            };
            predicate(noun)
        })
    }

    fn has_adjective(&self, predicate: impl Fn(&Adjective) -> bool) -> bool {
        self.adjectives.iter().copied().any(predicate)
    }

    fn has_verb(&self, predicate: impl Fn(&Verb) -> bool) -> bool {
        self.predicate_heads
            .iter()
            .any(|head| predicate(&head.verb.verb))
    }

    fn ability(&mut self, ability: &'syntax Ability) {
        match &ability.kind {
            AbilityKind::Activated(activated) => {
                self.cost(&activated.cost);
                self.paragraph(&activated.effect);
            }
            AbilityKind::ClassLevel(level) => self.cost(&level.cost),
            AbilityKind::Chapter(chapter) => self.paragraph(&chapter.body),
            AbilityKind::RollRow(row) => self.paragraph(&row.body),
            AbilityKind::LevelBand(band) => {
                self.power_toughness.push(band.stats);
                for ability in &band.abilities {
                    self.ability(ability);
                }
            }
            AbilityKind::StationThreshold(threshold) => self.ability(&threshold.ability),
            AbilityKind::Triggered(triggered) => {
                self.trigger_event(&triggered.conditions.first.event);
                for coordination in &triggered.conditions.rest {
                    self.trigger_event(&coordination.condition.event);
                }
                if let Some(condition) = &triggered.intervening_condition {
                    self.dependent_clause(condition);
                }
                self.paragraph(&triggered.effect);
            }
            AbilityKind::Loyalty(loyalty) => self.paragraph(&loyalty.effect),
            AbilityKind::Modal(modal) => {
                match &modal.frame {
                    ModalFrame::Activated(cost) => self.cost(cost),
                    ModalFrame::Triggered(trigger) => self.trigger_header(trigger),
                    ModalFrame::Unframed
                    | ModalFrame::Loyalty(_)
                    | ModalFrame::Chapter(_)
                    | ModalFrame::Keyword(_) => {}
                }
                self.paragraph(&modal.header);
                for mode in &modal.modes {
                    if let Some(heading) = &mode.heading {
                        self.cost(&heading.cost);
                    }
                    self.paragraph(&mode.body);
                }
            }
            AbilityKind::Keyword(list) => {
                for keyword in &list.abilities {
                    self.keyword_argument(&keyword.argument);
                }
                if let Some(trailing) = &list.trailing {
                    self.paragraph(trailing);
                }
            }
            AbilityKind::Paragraph(paragraph) => self.paragraph(paragraph),
        }
    }

    fn keyword_argument(&mut self, argument: &'syntax KeywordArgument) {
        match argument {
            KeywordArgument::Counted(quantity) => self.quantities.push(*quantity),
            KeywordArgument::Costed(KeywordCost::Sentence { ability, .. }) => {
                self.ability(ability);
            }
            KeywordArgument::Costed(KeywordCost::Components { cost, .. }) => self.cost(cost),
            KeywordArgument::RestrictedCost {
                preposition,
                restriction,
                cost,
            } => {
                if let Some(preposition) = preposition {
                    self.prepositions.push(*preposition);
                }
                self.noun_phrase(restriction);
                match cost {
                    KeywordCost::Sentence { ability, .. } => self.ability(ability),
                    KeywordCost::Components { cost, .. } => self.cost(cost),
                    KeywordCost::Symbols(_) => {}
                }
            }
            KeywordArgument::Qualified(phrase) => self.phrase(phrase),
            KeywordArgument::Predicated(predicated) => {
                for quality in &predicated.qualities {
                    if let Some(preposition) = quality.preposition {
                        self.prepositions.push(preposition);
                    }
                    self.phrase(&quality.quality);
                }
            }
            KeywordArgument::Statted { stats, .. } => self.power_toughness.push(*stats),
            KeywordArgument::Absent
            | KeywordArgument::Costed(KeywordCost::Symbols(_))
            | KeywordArgument::CountedCost { .. }
            | KeywordArgument::Named { .. }
            | KeywordArgument::Recovered { .. } => {}
        }
    }

    fn cost(&mut self, cost: &'syntax Cost) {
        for component in &cost.components {
            self.cost_component(component);
        }
    }

    fn cost_component(&mut self, component: &'syntax CostComponent) {
        match component {
            CostComponent::Clause(clause) => self.independent_clause(clause),
            CostComponent::Noun(noun) => self.noun_phrase(noun),
            CostComponent::Alternative(left, right) => {
                self.cost_component(left);
                self.cost_component(right);
            }
            CostComponent::Symbols(_) | CostComponent::Recovered(_) => {}
        }
    }

    fn paragraph(&mut self, paragraph: &'syntax Paragraph) {
        for sentence in &paragraph.sentences {
            match &sentence.body {
                SentenceBody::Independent(clause) => self.independent_clause(clause),
                SentenceBody::Choice(choice) => {
                    if let Some(trigger) = &choice.trigger_prefix {
                        self.trigger_header(trigger);
                    }
                    self.predicate(&choice.imperative);
                }
                SentenceBody::PowerToughness(stats) => self.power_toughness.push(*stats),
                SentenceBody::Triggered(triggered) => {
                    self.trigger_header(&triggered.trigger);
                    self.independent_clause(&triggered.effect);
                }
                SentenceBody::Recovered(_) => {}
            }
        }
    }

    fn trigger_header(&mut self, trigger: &'syntax TriggerHeader) {
        self.trigger_event(&trigger.event);
        if let Some(condition) = &trigger.intervening_condition {
            self.dependent_clause(condition);
        }
    }

    fn trigger_event(&mut self, event: &'syntax TriggerEvent) {
        match event {
            TriggerEvent::Clause(clause) => self.independent_clause(clause),
            TriggerEvent::Temporal(noun) => self.noun_phrase(noun),
        }
    }

    fn independent_clause(&mut self, clause: &'syntax IndependentClause) {
        match clause {
            IndependentClause::Transitive(subject, predicate) => {
                self.subject(subject);
                self.transitive_predicate(predicate);
            }
            IndependentClause::Intransitive(subject, predicate) => {
                self.subject(subject);
                self.predicate_head(&predicate.head);
                self.predicate_elements(&predicate.elements);
            }
            IndependentClause::Copular(subject, predicate) => {
                self.subject(subject);
                self.copular_complement(&predicate.complement);
                self.predicate_adjuncts(&predicate.adjuncts);
            }
            IndependentClause::Passive(subject, predicate) => {
                self.subject(subject);
                self.predicate_head(&predicate.head);
                if let Some(object) = &predicate.retained_object {
                    self.predicate_object(object);
                }
                self.predicate_elements(&predicate.elements);
            }
            IndependentClause::Predicated(subject, expression) => {
                if let Some(subject) = subject {
                    self.subject(subject);
                }
                self.predicate_expression(expression);
            }
            IndependentClause::Imperative(predicate) => self.predicate(predicate),
            IndependentClause::Deontic(subject, _, predicate) => {
                self.subject(subject);
                if let Some(predicate) = predicate {
                    self.predicate(predicate);
                }
            }
            IndependentClause::Existential(existential) => {
                self.noun_phrase(&existential.pivot);
                self.predicate_adjuncts(&existential.adjuncts);
            }
            IndependentClause::Proform(subject, _) => self.subject(subject),
            IndependentClause::Complex(complex) => {
                self.independent_clause(&complex.matrix);
                for attachment in &complex.attachments {
                    self.clause_attachment(&attachment.payload);
                }
            }
            IndependentClause::Coordinated(coordinated) => {
                self.independent_clause(&coordinated.first);
                for coordination in &coordinated.rest {
                    let CoordinatedClauseMember::Independent(clause) = &coordination.member;
                    self.independent_clause(clause);
                }
            }
        }
    }

    fn predicate_expression(&mut self, expression: &'syntax PredicateExpression) {
        match expression {
            PredicateExpression::Simple(predicate) => self.predicate(predicate),
            PredicateExpression::Coordinated(coordination) => {
                for expression in coordination.conjuncts() {
                    self.predicate_expression(expression);
                }
            }
        }
    }

    fn clause_attachment(&mut self, attachment: &'syntax ClauseAttachmentKind) {
        match attachment {
            ClauseAttachmentKind::Dependent(clause) => self.dependent_clause(clause),
            ClauseAttachmentKind::Adjunct(adjunct) => self.predicate_adjunct(adjunct),
            ClauseAttachmentKind::Exception(rider) => {
                self.independent_clause(&rider.first);
                for conjunct in &rider.rest {
                    self.independent_clause(&conjunct.clause);
                }
            }
            ClauseAttachmentKind::Restriction(run) => {
                self.predicate_adjuncts(&run.first);
                for member in &run.rest {
                    self.predicate_adjuncts(&member.adjuncts);
                }
            }
            ClauseAttachmentKind::Appositive(clause) => self.independent_clause(clause),
        }
    }

    fn dependent_clause(&mut self, clause: &'syntax DependentClause) {
        match clause {
            DependentClause::Subordinate(_, SubordinateBody::Finite(clause)) => {
                self.independent_clause(clause);
            }
            DependentClause::Subordinate(_, SubordinateBody::Infinitive(clause))
            | DependentClause::Infinitive(clause) => self.predicate(&clause.predicate),
            DependentClause::Subordinate(_, SubordinateBody::Gerund(clause))
            | DependentClause::Gerund(clause) => self.gerund_clause(clause),
            DependentClause::Subordinate(
                _,
                SubordinateBody::Elliptical(EllipticalClause::Adjective(adjective)),
            ) => self.adjective_phrase(adjective),
            DependentClause::Relative(relative) => self.relative_clause(relative),
        }
    }

    fn gerund_clause(&mut self, clause: &'syntax GerundClause) {
        self.predicate(&clause.predicate);
        for attachment in &clause.attachments {
            self.dependent_clause(&attachment.payload);
        }
    }

    fn subject(&mut self, subject: &'syntax Subject) {
        self.noun_phrase(&subject.0);
    }

    fn predicate(&mut self, predicate: &'syntax Predicate) {
        match predicate {
            Predicate::Transitive(predicate) => self.transitive_predicate(predicate),
            Predicate::Intransitive(predicate) => {
                self.predicate_head(&predicate.head);
                self.predicate_elements(&predicate.elements);
            }
            Predicate::Copular(predicate) => {
                self.copular_complement(&predicate.complement);
                self.predicate_adjuncts(&predicate.adjuncts);
            }
            Predicate::Passive(predicate) => {
                self.predicate_head(&predicate.head);
                if let Some(object) = &predicate.retained_object {
                    self.predicate_object(object);
                }
                self.predicate_elements(&predicate.elements);
            }
            Predicate::Deontic(predicate) => {
                if let Some(inner) = &predicate.inner {
                    self.predicate(inner);
                }
            }
            Predicate::Attached(predicate) => {
                self.predicate(&predicate.predicate);
                for attachment in &predicate.attachments {
                    self.clause_attachment(&attachment.payload);
                }
            }
            Predicate::Proform(_) => {}
        }
    }

    fn transitive_predicate(&mut self, predicate: &'syntax TransitivePredicate) {
        self.predicate_head(&predicate.head);
        self.predicate_elements(&predicate.pre_object_elements);
        self.predicate_object(&predicate.object);
        self.predicate_elements(&predicate.elements);
    }

    fn predicate_head(&mut self, head: &'syntax PredicateHead) {
        self.predicate_heads.push(head);
    }

    fn predicate_object(&mut self, object: &'syntax PredicateObject) {
        match object {
            PredicateObject::NounPhrase(noun) => self.noun_phrase(noun),
            PredicateObject::Ability(ability) => {
                if let Some(argument) = &ability.argument {
                    self.predicate_object(argument);
                }
            }
            PredicateObject::Quantity(quantity) => self.quantities.push(*quantity),
            PredicateObject::PowerToughness(stats) => self.power_toughness.push(*stats),
            PredicateObject::EmbeddedAbility(ability) => self.ability(ability),
            PredicateObject::QuotedAbility(quoted) => self.ability(&quoted.ability),
            PredicateObject::Coordinated(coordinated) => {
                self.predicate_object(&coordinated.first);
                for coordination in &coordinated.rest {
                    self.predicate_object(&coordination.object);
                }
            }
            PredicateObject::OracleSymbol(_) | PredicateObject::SymbolSequence(_) => {}
        }
    }

    fn predicate_elements(&mut self, elements: &'syntax [PredicateElement]) {
        for element in elements {
            match element {
                PredicateElement::Complement(PredicateComplement::IndirectObject(noun)) => {
                    self.noun_phrase(noun);
                }
                PredicateElement::Complement(PredicateComplement::Adjective(adjective)) => {
                    self.adjective_phrase(adjective);
                }
                PredicateElement::Complement(PredicateComplement::CoordinatedAdjective(
                    coordinated,
                )) => self.coordinated_adjective_phrase(coordinated),
                PredicateElement::Complement(PredicateComplement::Prepositional(preposition)) => {
                    self.prepositional_phrase(preposition);
                }
                PredicateElement::Complement(PredicateComplement::Infinitive(infinitive)) => {
                    self.predicate(&infinitive.predicate);
                }
                PredicateElement::Adjunct(adjunct) => self.predicate_adjunct(adjunct),
                PredicateElement::Particle(_) | PredicateElement::CoinResult(_) => {}
            }
        }
    }

    fn predicate_adjuncts(&mut self, adjuncts: &'syntax [PredicateAdjunct]) {
        for adjunct in adjuncts {
            self.predicate_adjunct(adjunct);
        }
    }

    fn predicate_adjunct(&mut self, adjunct: &'syntax PredicateAdjunct) {
        match adjunct {
            PredicateAdjunct::Temporal(noun) | PredicateAdjunct::Manner(noun) => {
                self.noun_phrase(noun);
            }
            PredicateAdjunct::Prepositional(preposition)
            | PredicateAdjunct::Exception(preposition) => {
                self.prepositional_phrase(preposition);
            }
            PredicateAdjunct::Dependent(dependent) => self.dependent_clause(dependent),
            PredicateAdjunct::Adverb(_) | PredicateAdjunct::Frequency(_) => {}
        }
    }

    fn copular_complement(&mut self, complement: &'syntax CopularComplement) {
        match complement {
            CopularComplement::NounPhrase(noun) => self.noun_phrase(noun),
            CopularComplement::Adjective(adjective) => self.adjective_phrase(adjective),
            CopularComplement::CoordinatedAdjective(coordinated) => {
                self.coordinated_adjective_phrase(coordinated);
            }
            CopularComplement::Prepositional(preposition) => {
                self.prepositional_phrase(preposition);
            }
            CopularComplement::PowerToughness(stats) => self.power_toughness.push(*stats),
            CopularComplement::CatalogAtom(_) => {}
        }
    }

    fn relative_clause(&mut self, relative: &'syntax RelativeClause) {
        match &relative.body {
            RelativeBody::SubjectGap(predicate) => self.predicate(predicate),
            RelativeBody::ObjectGap { subject, predicate } => {
                self.subject(subject);
                self.predicate_head(&predicate.head);
                self.predicate_elements(&predicate.elements);
            }
        }
    }

    fn noun_phrase(&mut self, phrase: &'syntax NounPhrase) {
        self.noun_phrases.push(phrase);
        match phrase {
            NounPhrase::Nominal(nominal) => self.nominal_phrase(nominal),
            NounPhrase::Pronoun { pronoun, .. } => self.pronouns.push(*pronoun),
            NounPhrase::Possessive(possessor) => self.possessor(possessor),
            NounPhrase::Quantity(quantity) => self.quantities.push(*quantity),
            NounPhrase::ThisCard(form) => self.this_cards.push(*form),
            NounPhrase::Partitive(partitive) => {
                if let PartitiveHead::Quantity(quantity) = partitive.head {
                    self.quantities.push(quantity);
                }
                self.noun_phrase(&partitive.whole);
            }
            NounPhrase::CoordinatedNominal(coordinated) => {
                self.determiner(&coordinated.determiner);
                self.nominal_phrase(&coordinated.first);
                for coordination in &coordinated.rest {
                    self.nominal_phrase(&coordination.phrase);
                }
                for complement in &coordinated.complements {
                    self.nominal_complement(complement);
                }
            }
            NounPhrase::Coordinated(coordinated) => {
                self.noun_phrase(&coordinated.first);
                for coordination in &coordinated.rest {
                    self.noun_phrase(&coordination.phrase);
                }
            }
            NounPhrase::SetException(exception) => {
                self.noun_phrase(&exception.included);
                self.noun_phrase(&exception.excluded);
            }
            NounPhrase::Arithmetic(ArithmeticValue::Minus { left, right }) => {
                self.noun_phrase(left);
                self.noun_phrase(right);
            }
            NounPhrase::Arithmetic(ArithmeticValue::Half { value, .. }) => {
                self.noun_phrase(value);
            }
            NounPhrase::Demonstrative(_) => {}
        }
    }

    fn possessor(&mut self, possessor: &'syntax Possessor) {
        match possessor {
            Possessor::Pronoun(pronoun) => self.pronouns.push(*pronoun),
            Possessor::NounPhrase(noun) => self.noun_phrase(noun),
        }
    }

    fn determiner(&mut self, determiner: &'syntax Determiner) {
        match determiner {
            Determiner::Target(Some(quantity)) | Determiner::Quantity(quantity) => {
                self.quantities.push(*quantity);
            }
            Determiner::Possessive(possessor) => self.possessor(possessor),
            Determiner::The
            | Determiner::Each
            | Determiner::Another
            | Determiner::Indefinite
            | Determiner::Demonstrative(_)
            | Determiner::Target(None)
            | Determiner::All
            | Determiner::Any
            | Determiner::No => {}
        }
    }

    fn nominal_phrase(&mut self, nominal: &'syntax NominalPhrase) {
        self.nominals.push(nominal);
        if let Some(determiner) = &nominal.determiner {
            self.determiner(determiner);
        }
        self.nouns.push(&nominal.head);
        for modifier in &nominal.modifiers {
            self.nominal_modifier(modifier);
        }
        for complement in &nominal.complements {
            self.nominal_complement(complement);
        }
    }

    fn nominal_complement(&mut self, complement: &'syntax NominalComplement) {
        match complement {
            NominalComplement::Adjective(adjective) => self.adjective_phrase(adjective),
            NominalComplement::CoordinatedAdjective(coordinated) => {
                self.adjective_phrase(&coordinated.first);
                for member in &coordinated.rest {
                    self.adjective_phrase(&member.phrase);
                }
            }
            NominalComplement::Prepositional(preposition) => {
                self.prepositional_phrase(preposition);
            }
            NominalComplement::Infinitive(infinitive) => {
                self.predicate(&infinitive.predicate);
            }
            NominalComplement::Relative(relative) => self.relative_clause(relative),
            NominalComplement::ReducedRecipientPassive(predicate) => {
                self.transitive_predicate(predicate);
            }
            NominalComplement::Quantity(quantity) => self.quantities.push(*quantity),
            NominalComplement::PowerToughness(stats) => self.power_toughness.push(*stats),
            NominalComplement::EventClause(clause) => self.independent_clause(clause),
            NominalComplement::KeywordArgument(argument) => self.keyword_argument(argument),
            NominalComplement::Devotion(_) => {}
        }
    }

    fn nominal_modifier(&mut self, modifier: &'syntax NominalModifier) {
        match modifier {
            NominalModifier::Adjective { phrase, .. } => self.adjective_phrase(phrase),
            NominalModifier::Noun { noun, .. } => self.nouns.push(noun),
            NominalModifier::Quantity(quantity) => self.quantities.push(*quantity),
            NominalModifier::PowerToughness(stats) => self.power_toughness.push(*stats),
            NominalModifier::CombatStepName { participants } => self.nouns.push(participants),
            NominalModifier::Coordinated(coordinated) => {
                self.nominal_modifier(&coordinated.first);
                for coordination in &coordinated.rest {
                    self.nominal_modifier(&coordination.modifier);
                }
            }
        }
    }

    fn adjective_phrase(&mut self, phrase: &'syntax AdjectivePhrase) {
        self.adjectives.push(&phrase.head);
        for complement in &phrase.complements {
            match complement {
                AdjectiveComplement::Comparison(comparison)
                | AdjectiveComplement::PostnominalComparison(comparison) => {
                    self.phrase(&comparison.standard);
                }
                AdjectiveComplement::Prepositional(preposition) => {
                    self.prepositional_phrase(preposition);
                }
                AdjectiveComplement::Infinitive(infinitive) => {
                    self.predicate(&infinitive.predicate);
                }
            }
        }
    }

    fn coordinated_adjective_phrase(&mut self, phrase: &'syntax CoordinatedAdjectivePhrase) {
        self.coordinated_adjectives += 1;
        self.adjective_phrase(&phrase.first);
        for coordination in &phrase.rest {
            self.adjective_phrase(&coordination.phrase);
        }
    }

    fn prepositional_phrase(&mut self, phrase: &'syntax PrepositionalPhrase) {
        self.prepositions.push(phrase.head().preposition);
        self.phrase(&phrase.head().object);
    }

    fn phrase(&mut self, phrase: &'syntax Phrase) {
        match phrase {
            Phrase::Clause(clause) => match clause.as_ref() {
                Clause::Independent(clause) => self.independent_clause(clause),
                Clause::Dependent(clause) => self.dependent_clause(clause),
            },
            Phrase::NounPhrase(noun) => self.noun_phrase(noun),
            Phrase::AdjectivePhrase(adjective) => self.adjective_phrase(adjective),
            Phrase::PrepositionalPhrase(preposition) => {
                self.prepositional_phrase(preposition);
            }
            Phrase::Quantity(quantity) => self.quantities.push(*quantity),
            Phrase::Cost(cost) => self.cost(cost),
            Phrase::ThisCard(form) => self.this_cards.push(*form),
            Phrase::PowerToughness(stats) => self.power_toughness.push(*stats),
            Phrase::EmbeddedAbility(ability) => self.ability(ability),
            Phrase::QuotedAbility(quoted) => self.ability(&quoted.ability),
            Phrase::Adverb(_)
            | Phrase::CatalogAtom(_)
            | Phrase::ColorWord(_)
            | Phrase::OracleSymbol(_)
            | Phrase::SymbolSequence(_)
            | Phrase::NumberLiteral(_)
            | Phrase::SignedScalar(_)
            | Phrase::Recovered(_) => {}
        }
    }
}

fn assert_no_recovery(ast: &OracleText) {
    assert!(ast.recoveries().is_empty(), "AST:\n{ast:#?}");
}

fn only_ability(ast: &OracleText) -> &Ability {
    let [ability] = ast.abilities.as_slice() else {
        panic!("expected exactly one ability: {ast:#?}");
    };
    ability
}

fn only_paragraph(ast: &OracleText) -> &Paragraph {
    let AbilityKind::Paragraph(paragraph) = &only_ability(ast).kind else {
        panic!("expected a paragraph ability: {ast:#?}");
    };
    paragraph
}

fn only_activated(ast: &OracleText) -> &ActivatedAbility {
    let AbilityKind::Activated(activated) = &only_ability(ast).kind else {
        panic!("expected an activated ability: {ast:#?}");
    };
    activated
}

fn only_independent_clause(ast: &OracleText) -> &IndependentClause {
    let [sentence] = only_paragraph(ast).sentences.as_slice() else {
        panic!("expected exactly one sentence: {ast:#?}");
    };
    let SentenceBody::Independent(clause) = &sentence.body else {
        panic!("expected an independent-clause sentence: {ast:#?}");
    };
    clause
}

fn exception_rider(clause: &IndependentClause) -> Option<&ExceptionRider> {
    match clause {
        IndependentClause::Complex(complex) => complex.attachments.iter().find_map(|attachment| {
            let ClauseAttachmentKind::Exception(rider) = &attachment.payload else {
                return None;
            };
            Some(rider)
        }),
        IndependentClause::Predicated(_, expression) => exception_rider_in_expression(expression),
        IndependentClause::Imperative(predicate) => exception_rider_in_predicate(predicate),
        IndependentClause::Deontic(_, _, predicate) => {
            predicate.as_ref().and_then(exception_rider_in_predicate)
        }
        IndependentClause::Coordinated(coordination) => exception_rider(&coordination.first)
            .or_else(|| {
                coordination.rest.iter().find_map(|member| {
                    let CoordinatedClauseMember::Independent(clause) = &member.member;
                    exception_rider(clause)
                })
            }),
        IndependentClause::Transitive(..)
        | IndependentClause::Intransitive(..)
        | IndependentClause::Copular(..)
        | IndependentClause::Passive(..)
        | IndependentClause::Existential(..)
        | IndependentClause::Proform(..) => None,
    }
}

fn exception_rider_in_expression(expression: &PredicateExpression) -> Option<&ExceptionRider> {
    match expression {
        PredicateExpression::Simple(predicate) => exception_rider_in_predicate(predicate),
        PredicateExpression::Coordinated(coordination) => coordination
            .conjuncts()
            .iter()
            .find_map(exception_rider_in_expression),
    }
}

fn exception_rider_in_predicate(predicate: &Predicate) -> Option<&ExceptionRider> {
    match predicate {
        Predicate::Attached(attached) => attached.attachments.iter().find_map(|attachment| {
            let ClauseAttachmentKind::Exception(rider) = &attachment.payload else {
                return None;
            };
            Some(rider)
        }),
        Predicate::Deontic(deontic) => deontic
            .inner
            .as_deref()
            .and_then(exception_rider_in_predicate),
        Predicate::Transitive(_)
        | Predicate::Intransitive(_)
        | Predicate::Copular(_)
        | Predicate::Passive(_)
        | Predicate::Proform(_) => None,
    }
}

fn appositive(clause: &IndependentClause) -> Option<&IndependentClause> {
    match clause {
        IndependentClause::Complex(complex) => complex.attachments.iter().find_map(|attachment| {
            let ClauseAttachmentKind::Appositive(appositive) = &attachment.payload else {
                return None;
            };
            Some(appositive.as_ref())
        }),
        IndependentClause::Predicated(_, expression) => appositive_in_expression(expression),
        IndependentClause::Imperative(predicate) => appositive_in_predicate(predicate),
        IndependentClause::Deontic(_, _, predicate) => {
            predicate.as_ref().and_then(appositive_in_predicate)
        }
        IndependentClause::Coordinated(coordination) => {
            appositive(&coordination.first).or_else(|| {
                coordination.rest.iter().find_map(|member| {
                    let CoordinatedClauseMember::Independent(clause) = &member.member;
                    appositive(clause)
                })
            })
        }
        IndependentClause::Transitive(..)
        | IndependentClause::Intransitive(..)
        | IndependentClause::Copular(..)
        | IndependentClause::Passive(..)
        | IndependentClause::Existential(..)
        | IndependentClause::Proform(..) => None,
    }
}

fn appositive_in_expression(expression: &PredicateExpression) -> Option<&IndependentClause> {
    match expression {
        PredicateExpression::Simple(predicate) => appositive_in_predicate(predicate),
        PredicateExpression::Coordinated(coordination) => coordination
            .conjuncts()
            .iter()
            .find_map(appositive_in_expression),
    }
}

fn appositive_in_predicate(predicate: &Predicate) -> Option<&IndependentClause> {
    match predicate {
        Predicate::Attached(attached) => attached.attachments.iter().find_map(|attachment| {
            let ClauseAttachmentKind::Appositive(appositive) = &attachment.payload else {
                return None;
            };
            Some(appositive.as_ref())
        }),
        Predicate::Deontic(deontic) => deontic.inner.as_deref().and_then(appositive_in_predicate),
        Predicate::Transitive(_)
        | Predicate::Intransitive(_)
        | Predicate::Copular(_)
        | Predicate::Passive(_)
        | Predicate::Proform(_) => None,
    }
}

fn predicate_head(predicate: &Predicate) -> Option<&PredicateHead> {
    match predicate {
        Predicate::Transitive(predicate) => Some(&predicate.head),
        Predicate::Intransitive(predicate) => Some(&predicate.head),
        Predicate::Passive(predicate) => Some(&predicate.head),
        Predicate::Attached(attached) => predicate_head(&attached.predicate),
        Predicate::Deontic(deontic) => deontic.inner.as_deref().and_then(predicate_head),
        Predicate::Copular(_) | Predicate::Proform(_) => None,
    }
}

fn matrix_predicate_head(clause: &IndependentClause) -> Option<&PredicateHead> {
    match clause {
        IndependentClause::Transitive(_, predicate) => Some(&predicate.head),
        IndependentClause::Intransitive(_, predicate) => Some(&predicate.head),
        IndependentClause::Passive(_, predicate) => Some(&predicate.head),
        IndependentClause::Predicated(_, PredicateExpression::Simple(predicate))
        | IndependentClause::Imperative(predicate) => predicate_head(predicate),
        IndependentClause::Deontic(_, _, predicate) => predicate.as_ref().and_then(predicate_head),
        IndependentClause::Complex(complex) => matrix_predicate_head(&complex.matrix),
        IndependentClause::Copular(..)
        | IndependentClause::Predicated(_, PredicateExpression::Coordinated(_))
        | IndependentClause::Existential(_)
        | IndependentClause::Proform(..)
        | IndependentClause::Coordinated(_) => None,
    }
}

fn clause_subject(clause: &IndependentClause) -> Option<&NounPhrase> {
    match clause {
        IndependentClause::Transitive(subject, _)
        | IndependentClause::Intransitive(subject, _)
        | IndependentClause::Copular(subject, _)
        | IndependentClause::Passive(subject, _)
        | IndependentClause::Deontic(subject, _, _)
        | IndependentClause::Proform(subject, _)
        | IndependentClause::Predicated(Some(subject), _) => Some(&subject.0),
        IndependentClause::Complex(complex) => clause_subject(&complex.matrix),
        IndependentClause::Predicated(None, _)
        | IndependentClause::Imperative(_)
        | IndependentClause::Existential(_)
        | IndependentClause::Coordinated(_) => None,
    }
}

fn predicate_object(predicate: &Predicate) -> Option<&PredicateObject> {
    match predicate {
        Predicate::Transitive(predicate) => Some(&predicate.object),
        Predicate::Attached(attached) => predicate_object(&attached.predicate),
        Predicate::Deontic(deontic) => deontic.inner.as_deref().and_then(predicate_object),
        Predicate::Intransitive(_)
        | Predicate::Copular(_)
        | Predicate::Passive(_)
        | Predicate::Proform(_) => None,
    }
}

fn direct_object(clause: &IndependentClause) -> Option<&PredicateObject> {
    match clause {
        IndependentClause::Transitive(_, predicate) => Some(&predicate.object),
        IndependentClause::Predicated(_, PredicateExpression::Simple(predicate))
        | IndependentClause::Imperative(predicate) => predicate_object(predicate),
        IndependentClause::Deontic(_, _, predicate) => {
            predicate.as_ref().and_then(predicate_object)
        }
        IndependentClause::Complex(complex) => direct_object(&complex.matrix),
        IndependentClause::Intransitive(..)
        | IndependentClause::Copular(..)
        | IndependentClause::Passive(..)
        | IndependentClause::Predicated(_, PredicateExpression::Coordinated(_))
        | IndependentClause::Existential(_)
        | IndependentClause::Proform(..)
        | IndependentClause::Coordinated(_) => None,
    }
}

fn matrix_distributive_each(ast: &OracleText) -> bool {
    matrix_predicate_head(only_independent_clause(ast)).is_some_and(|head| head.distributive_each)
}

fn first_independent_in_paragraph(paragraph: &Paragraph) -> Option<&IndependentClause> {
    paragraph.sentences.iter().find_map(|sentence| {
        let SentenceBody::Independent(clause) = &sentence.body else {
            return None;
        };
        Some(clause)
    })
}

fn first_effect_clause(ability: &Ability) -> Option<&IndependentClause> {
    match &ability.kind {
        AbilityKind::Activated(activated) => first_independent_in_paragraph(&activated.effect),
        AbilityKind::Chapter(chapter) => first_independent_in_paragraph(&chapter.body),
        AbilityKind::RollRow(row) => first_independent_in_paragraph(&row.body),
        AbilityKind::Triggered(triggered) => first_independent_in_paragraph(&triggered.effect),
        AbilityKind::Loyalty(loyalty) => first_independent_in_paragraph(&loyalty.effect),
        AbilityKind::Paragraph(paragraph) => first_independent_in_paragraph(paragraph),
        AbilityKind::ClassLevel(_)
        | AbilityKind::LevelBand(_)
        | AbilityKind::StationThreshold(_)
        | AbilityKind::Modal(_)
        | AbilityKind::Keyword(_) => None,
    }
}

fn last_effect_clause(ast: &OracleText) -> &IndependentClause {
    ast.abilities
        .iter()
        .rev()
        .find_map(first_effect_clause)
        .unwrap_or_else(|| panic!("expected an effect clause: {ast:#?}"))
}

fn is_any_number_of(phrase: &NounPhrase) -> bool {
    let NounPhrase::Nominal(NominalPhrase {
        determiner: Some(Determiner::Any),
        modifiers,
        head: NounInstance::Singular(Noun::Word(Vocab::Number)),
        complements,
        ..
    }) = phrase
    else {
        return false;
    };
    modifiers.is_empty()
        && matches!(
            complements.first(),
            Some(NominalComplement::Prepositional(
                PrepositionalPhrase::Simple(deckmaste_english::syntax::SimplePrepositionalPhrase {
                    preposition: Preposition::Of,
                    ..
                })
            ))
        )
}

fn has_finite_subordinate(clause: &IndependentClause, expected: Subordinator) -> bool {
    match clause {
        IndependentClause::Complex(complex) => {
            complex.attachments.iter().any(|attachment| {
                matches!(
                    &attachment.payload,
                    ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                        subordinator,
                        SubordinateBody::Finite(_)
                    )) if *subordinator == expected
                )
            }) || has_finite_subordinate(&complex.matrix, expected)
        }
        IndependentClause::Predicated(_, PredicateExpression::Simple(predicate))
        | IndependentClause::Imperative(predicate) => {
            predicate_has_finite_subordinate(predicate, expected)
        }
        IndependentClause::Deontic(_, _, predicate) => predicate
            .as_ref()
            .is_some_and(|predicate| predicate_has_finite_subordinate(predicate, expected)),
        IndependentClause::Coordinated(coordination) => {
            has_finite_subordinate(&coordination.first, expected)
                || coordination.rest.iter().any(|member| {
                    let CoordinatedClauseMember::Independent(clause) = &member.member;
                    has_finite_subordinate(clause, expected)
                })
        }
        IndependentClause::Transitive(..)
        | IndependentClause::Intransitive(..)
        | IndependentClause::Copular(..)
        | IndependentClause::Passive(..)
        | IndependentClause::Predicated(_, PredicateExpression::Coordinated(_))
        | IndependentClause::Existential(_)
        | IndependentClause::Proform(..) => false,
    }
}

fn predicate_has_finite_subordinate(predicate: &Predicate, expected: Subordinator) -> bool {
    match predicate {
        Predicate::Attached(attached) => {
            attached.attachments.iter().any(|attachment| {
                matches!(
                    &attachment.payload,
                    ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                        subordinator,
                        SubordinateBody::Finite(_)
                    )) if *subordinator == expected
                )
            }) || predicate_has_finite_subordinate(&attached.predicate, expected)
        }
        Predicate::Deontic(deontic) => deontic
            .inner
            .as_deref()
            .is_some_and(|predicate| predicate_has_finite_subordinate(predicate, expected)),
        Predicate::Transitive(_)
        | Predicate::Intransitive(_)
        | Predicate::Copular(_)
        | Predicate::Passive(_)
        | Predicate::Proform(_) => false,
    }
}

fn matrix_has_prepositional_adjunct(clause: &IndependentClause, expected: Preposition) -> bool {
    let elements = match clause {
        IndependentClause::Transitive(_, predicate)
        | IndependentClause::Predicated(
            _,
            PredicateExpression::Simple(Predicate::Transitive(predicate)),
        ) => &predicate.elements,
        IndependentClause::Intransitive(_, predicate)
        | IndependentClause::Predicated(
            _,
            PredicateExpression::Simple(Predicate::Intransitive(predicate)),
        ) => &predicate.elements,
        IndependentClause::Passive(_, predicate) => &predicate.elements,
        IndependentClause::Complex(complex) => {
            return matrix_has_prepositional_adjunct(&complex.matrix, expected);
        }
        _ => return false,
    };
    elements.iter().any(|element| {
        matches!(
            element,
            PredicateElement::Adjunct(PredicateAdjunct::Prepositional(PrepositionalPhrase::Simple(deckmaste_english::syntax::SimplePrepositionalPhrase {
                preposition,
                ..
            }))) if *preposition == expected
        )
    })
}

// --- Self-reference recognition (positives) --------------------------------

#[test]
fn single_word_full_name_is_a_full_self_reference() {
    let (rendered, ast) = parse_face(
        "Progenitus attacks.",
        &Catalogs::default(),
        "Progenitus",
        true,
    );
    assert_eq!(rendered, "Progenitus attacks.");
    assert!(
        matches!(
            only_independent_clause(&ast),
            IndependentClause::Intransitive(
                Subject(NounPhrase::ThisCard(ThisCardForm::FullName)),
                _
            )
        ),
        "AST:\n{ast:#?}"
    );
}

#[test]
fn a_nickname_is_an_abbreviated_self_reference() {
    let (rendered, ast) = parse_face(
        "Ashcoat attacks.",
        &Catalogs::default(),
        "Ashcoat of the Shadow Swarm",
        true,
    );
    assert_eq!(rendered, "Ashcoat attacks.");
    assert!(
        matches!(
            only_independent_clause(&ast),
            IndependentClause::Intransitive(
                Subject(NounPhrase::ThisCard(ThisCardForm::AbbreviatedName)),
                _
            )
        ),
        "AST:\n{ast:#?}"
    );
}

#[test]
fn a_two_word_nickname_is_recognized_as_one_self_reference() {
    let (rendered, ast) = parse_face(
        "Sidar Jabari attacks.",
        &Catalogs::default(),
        "Sidar Jabari of Zhalfir",
        true,
    );
    assert_eq!(rendered, "Sidar Jabari attacks.");
    assert!(
        matches!(
            only_independent_clause(&ast),
            IndependentClause::Intransitive(
                Subject(NounPhrase::ThisCard(ThisCardForm::AbbreviatedName)),
                _
            )
        ),
        "AST:\n{ast:#?}"
    );
}

#[test]
fn a_roman_numeral_nickname_is_recognized() {
    // `King Darien XLVIII` shortens to `King Darien` by dropping the numeral.
    let (rendered, ast) = parse_face(
        "King Darien attacks.",
        &Catalogs::default(),
        "King Darien XLVIII",
        true,
    );
    assert_eq!(rendered, "King Darien attacks.");
    assert!(
        matches!(
            only_independent_clause(&ast),
            IndependentClause::Intransitive(
                Subject(NounPhrase::ThisCard(ThisCardForm::AbbreviatedName)),
                _
            )
        ),
        "AST:\n{ast:#?}"
    );
}

// --- Self-reference must lose to the surrounding grammar (traps) ------------

#[test]
fn sliver_stays_a_creature_type_not_a_self_reference() {
    // `Sliver Queen` shortens to `Sliver`, but in a token's type line `Sliver`
    // is the catalog creature type, which is equally structural and wins the tie.
    let catalogs = Catalogs::default()
        .with_catalog(CatalogKind::CreatureType, ["Sliver"])
        .with_catalog(CatalogKind::CardType, ["Creature"]);
    let (rendered, ast) = parse_face(
        "Create a 1/1 colorless Sliver creature token.",
        &catalogs,
        "Sliver Queen",
        true,
    );
    assert_eq!(rendered, "Create a 1/1 colorless Sliver creature token.");
    let IndependentClause::Imperative(Predicate::Transitive(predicate)) =
        only_independent_clause(&ast)
    else {
        panic!("expected a transitive imperative: {ast:#?}");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(token)) = &predicate.object else {
        panic!("expected a nominal token object: {ast:#?}");
    };
    assert!(
        token.modifiers.iter().any(|modifier| matches!(
            modifier,
            NominalModifier::Noun {
                noun: NounInstance::Singular(Noun::Catalog(atom)),
                ..
            } if atom.canonical() == "Sliver"
        )),
        "Sliver did not remain a creature-type modifier:\n{ast:#?}"
    );
}

#[test]
fn a_nickname_that_is_a_common_noun_keeps_its_capital() {
    // `carnage` is a lowercase vocabulary noun; the nickname `Carnage` must win
    // over that sentence-initial reading so its capital survives the round-trip.
    let (rendered, ast) = parse_face(
        "Carnage attacks.",
        &Catalogs::default(),
        "Carnage, Blood Artist",
        true,
    );
    assert_eq!(rendered, "Carnage attacks.");
    assert!(
        matches!(
            only_independent_clause(&ast),
            IndependentClause::Intransitive(
                Subject(NounPhrase::ThisCard(ThisCardForm::AbbreviatedName)),
                _
            )
        ),
        "AST:\n{ast:#?}"
    );
}

#[test]
fn a_nickname_that_is_a_keyword_ability_keeps_its_capital() {
    // A keyword-ability noun renders lowercase (`prowl`); the nickname `Prowl`
    // must win so the printed name keeps its capital.
    let catalogs = Catalogs::default().with_catalog(CatalogKind::KeywordAbility, ["Prowl"]);
    let (rendered, ast) = parse_face("Prowl attacks.", &catalogs, "Prowl, Stoic Strategist", true);
    assert_eq!(rendered, "Prowl attacks.");
    assert!(
        matches!(
            only_independent_clause(&ast),
            IndependentClause::Intransitive(
                Subject(NounPhrase::ThisCard(ThisCardForm::AbbreviatedName)),
                _
            )
        ),
        "AST:\n{ast:#?}"
    );
}

#[test]
fn a_the_headed_nickname_keeps_its_capital_the() {
    // `The Beast` also parses as the lowercase `the` determiner plus a `Beast`
    // creature type; the nickname must win so the capital `The` survives.
    let catalogs = Catalogs::default().with_catalog(CatalogKind::CreatureType, ["Beast"]);
    let (rendered, ast) = parse_face(
        "Whenever a creature dies, untap The Beast.",
        &catalogs,
        "The Beast, Deathless Prince",
        true,
    );
    assert_eq!(rendered, "Whenever a creature dies, untap The Beast.");
    assert!(
        matches!(
            &only_ability(&ast).kind,
            AbilityKind::Triggered(TriggeredAbility { effect, .. })
                if matches!(
                    effect.sentences.as_slice(),
                    [Sentence {
                        body: SentenceBody::Independent(IndependentClause::Imperative(
                            Predicate::Transitive(TransitivePredicate {
                                kind: Transitive {
                                    object: PredicateObject::NounPhrase(NounPhrase::ThisCard(
                                        ThisCardForm::AbbreviatedName
                                    )),
                                    ..
                                },
                                ..
                            })
                        )),
                        ..
                    }]
                )
        ),
        "AST:\n{ast:#?}"
    );
}

// --- Copy-effect exception riders [CR#707.9] -------------------------------

/// A legendary identity (nickname `Nissa`, full name `Nissa Revane`) plus the
/// catalog terms the exception conjuncts below name.
fn copy_catalogs() -> Catalogs {
    Catalogs::new(
        ["Flying"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    )
    .with_catalog(CatalogKind::CreatureType, ["Illusion"])
    .with_catalog(CatalogKind::CardType, ["Creature", "Permanent"])
    .with_catalog(CatalogKind::Supertype, ["Legendary", "Snow"])
}

const NISSA: &str = "Nissa Revane";

#[test]
fn single_conjunct_exception_rider_round_trips() {
    // An enter-as-copy host with one power/toughness copular exception conjunct.
    let source = "You may have this creature enter as a copy of any creature on \
                  the battlefield, except it's 7/7.";
    let (rendered, ast) = parse_face(source, &copy_catalogs(), NISSA, true);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let rider = exception_rider(only_independent_clause(&ast))
        .unwrap_or_else(|| panic!("expected an exception rider: {ast}"));
    assert!(
        matches!(
            rider.first.as_ref(),
            IndependentClause::Copular(
                _,
                CopularPredicate {
                    complement: CopularComplement::PowerToughness(_),
                    ..
                }
            )
        ) && rider.rest.is_empty(),
        "AST:\n{ast}"
    );
}

#[test]
fn quoted_final_exception_conjunct_stays_inside_oxford_rider() {
    // Sakashima-shaped: all three members belong to one Oxford exception list,
    // including the final quoted-ability clause.
    let source = "You may have Nissa Revane enter as a copy of any creature on \
                  the battlefield, except its name is Nissa Revane, it's legendary \
                  in addition to its other types, and it has \"{2}{U}{U}: Return \
                  Nissa Revane to its owner's hand at the beginning of the next \
                  end step.\"";
    let (rendered, ast) = parse_face(source, &copy_catalogs(), NISSA, true);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let clause = only_independent_clause(&ast);
    let rider =
        exception_rider(clause).unwrap_or_else(|| panic!("expected an exception rider: {ast}"));
    assert!(
        !matches!(clause, IndependentClause::Coordinated(_))
            && matches!(
                rider.first.as_ref(),
                IndependentClause::Copular(
                    _,
                    CopularPredicate {
                        complement: CopularComplement::NounPhrase(NounPhrase::ThisCard(
                            ThisCardForm::FullName
                        )),
                        ..
                    }
                )
            )
            && matches!(
                rider.rest.as_slice(),
                [
                    ExceptionConjunct {
                        conjunction: None,
                        clause: IndependentClause::Copular(..),
                    },
                    ExceptionConjunct {
                        conjunction: Some(PredicateConjunction::And),
                        clause: IndependentClause::Transitive(
                            _,
                            TransitivePredicate {
                                kind: Transitive {
                                    object: PredicateObject::QuotedAbility(_),
                                    ..
                                },
                                ..
                            }
                        ),
                    },
                ]
            ),
        "AST:\n{ast}"
    );
}

#[test]
fn two_member_post_exception_coordination_stays_outside_the_rider() {
    // With no preceding comma member, `, and <clause>` is ordinary outer
    // clause coordination rather than an Oxford close for the rider.
    let source = "You may have Nissa Revane enter as a copy of any creature on \
                  the battlefield, except its name is Nissa Revane, and it has \
                  \"{2}{U}{U}: Return Nissa Revane to its owner's hand.\"";
    let (rendered, ast) = parse_face(source, &copy_catalogs(), NISSA, true);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let clause = only_independent_clause(&ast);
    let rider =
        exception_rider(clause).unwrap_or_else(|| panic!("expected an exception rider: {ast}"));
    assert!(
        rider.rest.is_empty()
            && matches!(
                clause,
                IndependentClause::Coordinated(CoordinatedIndependentClause {
                    rest,
                    ..
                }) if matches!(
                    rest.as_slice(),
                    [ClauseCoordination {
                        member: CoordinatedClauseMember::Independent(clause),
                        ..
                    }] if matches!(
                        clause.as_ref(),
                        IndependentClause::Transitive(
                            _,
                            TransitivePredicate {
                                kind: Transitive {
                                    object: PredicateObject::QuotedAbility(_),
                                    ..
                                },
                                ..
                            }
                        )
                    )
                )
            ),
        "AST:\n{ast}"
    );
}

#[test]
fn quoted_ability_exception_on_a_token_copy_round_trips() {
    // A token-copy host with a single quoted-ability exception conjunct.
    let source = "Create a token that's a copy of target permanent, except the \
                  token has \"When this token enters, if it's a creature, it \
                  fights up to one target creature you don't control.\"";
    let (rendered, ast) = parse_face(source, &copy_catalogs(), NISSA, true);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let rider = exception_rider(only_independent_clause(&ast))
        .unwrap_or_else(|| panic!("expected an exception rider: {ast}"));
    assert!(
        matches!(
            rider.first.as_ref(),
            IndependentClause::Transitive(
                _,
                TransitivePredicate {
                    kind: Transitive {
                        object: PredicateObject::QuotedAbility(_),
                        ..
                    },
                    ..
                }
            )
        ) && rider.rest.is_empty(),
        "AST:\n{ast}"
    );
}

#[test]
fn name_exception_on_a_becomes_copy_carries_a_self_reference() {
    // A becomes-copy host whose exception names the copy as the face itself; the
    // name literal is the existing self-reference structure, not a new payload.
    let source = "This creature becomes a copy of target creature, except its \
                  name is Nissa Revane and it isn't legendary.";
    let (rendered, ast) = parse_face(source, &copy_catalogs(), NISSA, true);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let rider = exception_rider(only_independent_clause(&ast))
        .unwrap_or_else(|| panic!("expected an exception rider: {ast}"));
    assert!(
        matches!(
            rider.first.as_ref(),
            IndependentClause::Copular(
                _,
                CopularPredicate {
                    complement: CopularComplement::NounPhrase(NounPhrase::ThisCard(
                        ThisCardForm::FullName
                    )),
                    ..
                }
            )
        ) && rider.rest.len() == 1,
        "AST:\n{ast}"
    );
}

// --- Family C: predicative-adjective coordination ---------------------------

/// Catalogs naming the card types and supertypes the Family C witnesses use;
/// colors are built-in adjectives and need no catalog entry.
fn predicative_catalogs() -> Catalogs {
    Catalogs::default()
        .with_catalog(
            CatalogKind::CardType,
            ["Creature", "Permanent", "Planeswalker"],
        )
        .with_catalog(CatalogKind::Supertype, ["Legendary", "Snow"])
}

#[test]
fn contracted_relative_copular_coordinates_color_adjectives() {
    // Fry / Aether Gust shape: `that's <color> or <color>` on a coordinated
    // relative antecedent.
    let source = "Fry deals 5 damage to target creature or planeswalker that's white or blue.";
    let (rendered, ast) = parse_face(source, &predicative_catalogs(), "Fry", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert_eq!(
        SyntaxInventory::from_ast(&ast).coordinated_adjectives,
        1,
        "AST:\n{ast}"
    );
}

#[test]
fn matrix_copular_coordinates_supertype_adjectives() {
    // Moritte shape: `it's legendary and snow …`. Supertypes scan as both
    // adjective and noun, so this also exercises the all-adjective gate keeping
    // the adjective reading.
    let source = "It's legendary and snow in addition to its other types.";
    let (rendered, ast) = parse_face(source, &predicative_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert_eq!(
        SyntaxInventory::from_ast(&ast).coordinated_adjectives,
        1,
        "AST:\n{ast}"
    );
}

#[test]
fn noncontracted_relative_copular_coordinates_with_and_or() {
    // Glistening Deluge shape: `that are <color> and/or <color>` on the
    // intransitive-`be` verb-phrase path, admitting the `and/or` connective.
    let source = "Creatures that are green and/or white get -2/-2 until end of turn.";
    let (rendered, ast) = parse_face(source, &predicative_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert_eq!(
        SyntaxInventory::from_ast(&ast).coordinated_adjectives,
        1,
        "AST:\n{ast}"
    );
}

#[test]
fn coordinated_noun_object_after_a_verb_is_not_a_predicative_adjective() {
    // The gate: a bare coordinated *noun* pair after a verb keeps its ordinary
    // coordinated-noun-object parse and never reduces as a coordinated adjective
    // complement.
    let source = "Exile target creature or planeswalker.";
    let (rendered, ast) = parse_face(source, &predicative_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert_eq!(
        SyntaxInventory::from_ast(&ast).coordinated_adjectives,
        0,
        "AST:\n{ast}"
    );
}

// --- Family A: base power and toughness -------------------------------------

fn characteristic_catalogs() -> Catalogs {
    Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"])
}

#[test]
fn base_power_and_toughness_stat_sets_a_characteristic_pair() {
    // Candlekeep Inspiration shape: `have base power and toughness X/X, where X
    // is …`. The `power and toughness` pair rides the existing noun-phrase
    // coordination under the shared `base` modifier; the `X/X` value is a
    // power/toughness complement on the final characteristic.
    let source = "Creatures you control have base power and toughness X/X, where X is the \
                  number of cards in your graveyard.";
    let (rendered, ast) = parse_face(source, &characteristic_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let Some(PredicateObject::NounPhrase(NounPhrase::Coordinated(coordination))) =
        direct_object(only_independent_clause(&ast))
    else {
        panic!("expected a coordinated characteristic object: {ast}");
    };
    assert!(
        matches!(
            coordination.first.as_ref(),
            NounPhrase::Nominal(NominalPhrase {
                modifiers,
                head: NounInstance::Singular(Noun::Word(Vocab::Power)),
                complements,
                ..
            }) if complements.is_empty()
                && matches!(
                    modifiers.as_slice(),
                    [NominalModifier::Noun {
                        polarity: Polarity::Positive,
                        noun: NounInstance::Singular(Noun::Word(vocab)),
                    }] if vocab.spelling() == "base"
                )
        ) && matches!(
            coordination.rest.as_slice(),
            [NounPhraseCoordination {
                conjunction: Some(NounPhraseConjunction::And),
                comma: deckmaste_english::features::Comma::Absent,
                phrase: NounPhrase::Nominal(NominalPhrase {
                    head: NounInstance::Mass(Noun::Word(Vocab::Toughness)),
                    complements,
                    ..
                }),
            }] if matches!(
                complements.as_slice(),
                [NominalComplement::PowerToughness(PowerToughness {
                    power: SignedScalar {
                        sign: ScalarSign::None,
                        value: ScalarValue::X,
                    },
                    toughness: SignedScalar {
                        sign: ScalarSign::None,
                        value: ScalarValue::X,
                    },
                })]
            )
        ),
        "AST:\n{ast}"
    );
}

#[test]
fn base_power_or_toughness_quantity_bound_rides_the_existing_quantity_complement() {
    // Angelic Aberration stretch shape: the `or` pair with a `1 or less`
    // quantity bound reuses the existing quantity complement — no new machinery.
    let source = "Creatures you control have base power or toughness 1 or less.";
    let (rendered, ast) = parse_face(source, &characteristic_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let Some(PredicateObject::NounPhrase(NounPhrase::Coordinated(coordination))) =
        direct_object(only_independent_clause(&ast))
    else {
        panic!("expected a coordinated characteristic object: {ast}");
    };
    assert!(
        matches!(
            coordination.rest.as_slice(),
            [NounPhraseCoordination {
                conjunction: Some(NounPhraseConjunction::Or),
                phrase: NounPhrase::Nominal(NominalPhrase {
                    head: NounInstance::Mass(Noun::Word(Vocab::Toughness)),
                    complements,
                    ..
                }),
                ..
            }] if matches!(
                complements.as_slice(),
                [NominalComplement::Quantity(Quantity::OrComparison(
                    QuantityValue::Literal(NumberLiteral {
                        value: 1,
                        numeral: Numeral::Arabic(false),
                    }),
                    ComparativeWord::Less,
                ))]
            )
        ),
        "AST:\n{ast}"
    );
}

// --- Activation-cost round --------------------------------------------------

fn cost_catalogs() -> Catalogs {
    Catalogs::default().with_catalog(CatalogKind::CardType, ["Artifact", "Creature"])
}

#[test]
fn labeled_activation_cost_peels_a_flavor_header() {
    // Boast/Forecast/ability-word shape: an arbitrary label before a spaced
    // em dash heads the activation cost; the label is licensed flavor-header
    // opacity and the remainder parses as an ordinary cost.
    let source = "Boast — {1}{R}: Draw a card.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        only_activated(&ast).cost.flavor_header.is_some(),
        "AST:\n{ast}"
    );
}

#[test]
fn alternative_cost_components_join_with_or() {
    // A top-level `or` between cost payments becomes an Alternative component
    // rather than recovering verbatim.
    let source = "{T} or {W}: Add {G}.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        matches!(
            only_activated(&ast).cost.components.as_slice(),
            [CostComponent::Alternative(..)]
        ),
        "AST:\n{ast}"
    );
}

#[test]
fn an_or_inside_a_cost_clause_does_not_split_into_alternatives() {
    // The gate on the alternative split: it is tried only after the whole
    // component fails to parse, so a coordinated noun object inside a cost
    // clause keeps its ordinary parse.
    let source = "{T}, Sacrifice an artifact or creature: Draw a card.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        only_activated(&ast)
            .cost
            .components
            .iter()
            .all(|component| !matches!(component, CostComponent::Alternative(..))),
        "AST:\n{ast}"
    );
}

#[test]
fn discard_at_random_parses_as_a_cost_clause() {
    // `at random` rides the prepositional path with `random` as a mass noun
    // (the idiom is historically `at` + noun).
    let source = "{T}, Discard a card at random: Draw a card.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
}

#[test]
fn cost_noun_phrases_coordinate_with_and_or() {
    // Keskit/Mechtitan shape: `and/or` joins the two nominal heads under the
    // shared quantity (exactly two total); `then` still never does.
    let source = "Sacrifice two artifacts and/or creatures: Draw a card.";
    let (rendered, ast) = parse_face(source, &cost_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        matches!(
            only_activated(&ast).cost.components.as_slice(),
            [CostComponent::Clause(clause)] if matches!(
                clause.as_ref(),
                IndependentClause::Imperative(Predicate::Transitive(TransitivePredicate {
                    kind: Transitive {
                        object: PredicateObject::NounPhrase(
                            NounPhrase::CoordinatedNominal(coordinated)
                        ),
                        ..
                    },
                    ..
                })) if matches!(
                    coordinated.determiner,
                    Determiner::Quantity(Quantity::Exact(NumberLiteral { value: 2, .. }))
                ) && matches!(
                    coordinated.rest.as_slice(),
                    [NominalPhraseCoordination {
                        conjunction: Some(NounPhraseConjunction::AndOr),
                        ..
                    }]
                )
            )
        ),
        "AST:\n{ast}"
    );
}

// --- R30: the library-iteration cluster -------------------------------------

/// Card-type and creature-type atoms the library-iteration witnesses name.
fn library_catalogs() -> Catalogs {
    Catalogs::default()
        .with_catalog(CatalogKind::CardType, ["Creature", "Land"])
        .with_catalog(CatalogKind::CreatureType, ["Creature"])
}

#[test]
fn reveal_until_binds_an_iterated_action_with_a_clausal_until_complement() {
    // Ajani, Valiant Protector shape: the reveal loop bounded by a finite
    // `until you reveal a <kind> card` event clause. `until` scans as a
    // subordinator so the existing `<clause> <subordinator> <clause>` rule
    // attaches the complement.
    let source = "Reveal cards from the top of your library until you reveal a creature card.";
    let (rendered, ast) = parse_face(
        source,
        &library_catalogs(),
        "Ajani, Valiant Protector",
        true,
    );
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        has_finite_subordinate(only_independent_clause(&ast), Subordinator::Until),
        "expected a finite `until` subordinate clause\nAST:\n{ast}"
    );
}

#[test]
fn repeat_header_rides_the_clausal_until() {
    // Dance with Calamity / Eureka shape: `Repeat this process until <clause>`
    // reuses the same clausal-`until` complement — no repeat-specific rule.
    let source = "Repeat this process until the tie is broken.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Timesifter", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        has_finite_subordinate(only_independent_clause(&ast), Subordinator::Until),
        "expected a finite `until` subordinate clause\nAST:\n{ast}"
    );
}

#[test]
fn until_end_of_turn_stays_a_durational_preposition() {
    // The gate: adding the `until` subordinator must not hijack the durational
    // adjunct `until end of turn`, which stays a prepositional phrase — the two
    // readings are disjoint by what follows (a clause vs. a noun phrase).
    let source = "Target creature gets +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &library_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        matrix_has_prepositional_adjunct(only_independent_clause(&ast), Preposition::Until),
        "expected the durational preposition\nAST:\n{ast}"
    );
    assert!(
        !has_finite_subordinate(only_independent_clause(&ast), Subordinator::Until),
        "durational `until end of turn` must not parse as a subordinate clause\nAST:\n{ast}"
    );
}

#[test]
fn the_other_is_an_anaphoric_fused_head_nominal() {
    // Sea Gate Oracle / Sleight of Hand shape: one verb distributes over two
    // coordinated object+destination pairs, the second object being the
    // anaphoric fused head `the other` (the sibling of `the rest`).
    let source = "Put one of them into your hand and the other on the bottom of your library.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Sea Gate Oracle", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .has_noun(|noun| matches!(noun, NounInstance::Singular(Noun::Word(Vocab::Other)))),
        "expected `other` to head the fused-head nominal\nAST:\n{ast}"
    );
}

#[test]
fn the_others_plural_fused_head_nominal_round_trips() {
    // The plural anaphor `the others` declines regularly off the same head.
    let source = "Return the others to the battlefield.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .has_noun(|noun| matches!(noun, NounInstance::Plural(Noun::Word(Vocab::Other)))),
        "expected `others` to head the fused-head nominal\nAST:\n{ast}"
    );
}

#[test]
fn attributive_other_stays_an_adjective_not_a_noun_modifier() {
    // The gate: `other` is scanned as a count noun only to license the fused
    // head, and its noun reading is dispreferenced. In modifier position the
    // attributive-adjective reading — equal in every structural cost — must
    // still win, never a `NominalModifier::Noun`.
    let source = "All other creatures get +1/+1.";
    let (rendered, ast) = parse_face(source, &library_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let inventory = SyntaxInventory::from_ast(&ast);
    assert!(
        inventory.has_adjective(|adjective| matches!(adjective, Adjective::Word(Vocab::Other))),
        "expected `other` as an adjective modifier\nAST:\n{ast}"
    );
    assert!(
        !inventory
            .has_noun(|noun| matches!(noun, NounInstance::Singular(Noun::Word(Vocab::Other)))),
        "`other` must not reduce as a noun modifier\nAST:\n{ast}"
    );
}

#[test]
fn top_of_library_peek_and_play_are_supported() {
    // Xanathar / Case of the Locked Hothouse shape (the in-scope conjuncts):
    // `you may look at the top card of their library any time` and `you may
    // play the top card of their library` already parse — a regression guard,
    // no new machinery.
    let peek = "You may look at the top card of their library any time.";
    let (rendered, ast) = parse_face(peek, &Catalogs::default(), "Xanathar, Guild Kingpin", true);
    assert_eq!(rendered, peek);
    assert_no_recovery(&ast);

    let play = "You may play the top card of their library.";
    let (rendered, ast) = parse_face(play, &Catalogs::default(), "Xanathar, Guild Kingpin", true);
    assert_eq!(rendered, play);
    assert_no_recovery(&ast);
}

// --- R31: the choice / mode-header cluster ----------------------------------

/// The card-type atom the choice-cluster witnesses name.
fn choice_catalogs() -> Catalogs {
    Catalogs::default().with_catalog(CatalogKind::CardType, ["Creature"])
}

#[test]
fn trailing_dash_body_licenses_an_or_coordinated_appositive() {
    // The Master, Gallifrey's End shape: a complete clause whose tail names a
    // choice, then a spaced ` — ` and an `or`-coordinated pair of full clauses
    // spelling the options. Both sides parse as clauses; the dash body attaches
    // as a `ComplexClause` appositive. Licensed on shape, never on any word.
    let source = "That player faces a villainous choice — They lose 4 life, or you create a token.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        matches!(
            appositive(only_independent_clause(&ast)),
            Some(IndependentClause::Coordinated(_))
        ),
        "expected a trailing dash-body appositive\nAST:\n{ast}"
    );
}

#[test]
fn dash_appositive_attaches_under_a_coordinated_predicate_matrix() {
    // This Is How It Ends shape: the matrix is itself a `then`-coordinated
    // predicate, and the appositive trails the whole clause.
    let source = "Target creature's owner shuffles it into their library, then faces a villainous \
                  choice — They lose 5 life, or they draw a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "This Is How It Ends", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        matches!(
            appositive(only_independent_clause(&ast)),
            Some(IndependentClause::Coordinated(_))
        ),
        "AST:\n{ast}"
    );
}

#[test]
fn flavor_header_dash_is_not_read_as_an_appositive() {
    // Over-fire gate: a flavor header (`<label> — <sentence>`, the label not a
    // clause and the body a single sentence) stays a flavor header. The
    // appositive requires a clause matrix and an `or`-coordinated body, so a
    // non-clause label never fires it.
    let source = "Zorbo Rampage! — Draw a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    let paragraph = only_paragraph(&ast);
    assert!(
        paragraph.flavor_header.is_some(),
        "expected a flavor header\nAST:\n{ast}"
    );
    assert!(
        matches!(
            paragraph.sentences.as_slice(),
            [Sentence {
                body: SentenceBody::Independent(clause),
                ..
            }] if appositive(clause).is_none()
        ),
        "a flavor header must not become an appositive\nAST:\n{ast}"
    );
}

#[test]
fn single_clause_dash_body_does_not_license_an_appositive() {
    // Over-fire gate: the dash body must be `or`-coordinated. A clause matrix
    // followed by a single clause after the dash fails that test, so no
    // appositive fires — the sentence stays a verbatim recovered span.
    let source = "That player faces a villainous choice — They lose 4 life.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        matches!(
            only_paragraph(&ast).sentences.as_slice(),
            [Sentence {
                body: SentenceBody::Recovered(_),
                ..
            }]
        ),
        "a single-clause dash body must not become an appositive\nAST:\n{ast}"
    );
}

#[test]
fn bare_power_toughness_sentence_is_a_verbless_body() {
    // Vincent's Limit Break tiered-mode body: `3/2.` is a whole sentence whose
    // content is the base power and toughness the mode sets. The period marks a
    // sentence; the value reuses the statistic the object and copular positions
    // already model, and the renderer re-derives the period.
    let source = "3/2.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        matches!(
            only_paragraph(&ast).sentences.as_slice(),
            [Sentence {
                body: SentenceBody::PowerToughness(_),
                ..
            }]
        ),
        "expected a verbless power/toughness body\nAST:\n{ast}"
    );
}

#[test]
fn bare_power_toughness_without_a_period_stays_verbatim() {
    // Over-fire gate: a bare `N/N` with no terminal period is a level-band stat
    // line, not a sentence. It must stay a recovered span (rendered verbatim),
    // never a P/T body that would gain a spurious period.
    let source = "2/3";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        matches!(
            only_paragraph(&ast).sentences.as_slice(),
            [Sentence {
                body: SentenceBody::Recovered(_),
                ..
            }]
        ),
        "a periodless `N/N` must not become a P/T sentence body\nAST:\n{ast}"
    );
}

#[test]
fn those_characteristics_anaphor_parses_as_a_plural_nominal() {
    // Genku, Future Shaper / Outlaws' Merriment modal header: `... token with
    // those characteristics`. The blocker was a plural-form lemma; the singular
    // `characteristic` lemma lets the regular plural satisfy the `those`
    // demonstrative's required plural nominal.
    let source = "Create a creature token with those characteristics.";
    let (rendered, ast) = parse_face(source, &choice_catalogs(), "Genku, Future Shaper", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_noun(|noun| matches!(
            noun,
            NounInstance::Plural(Noun::Word(vocab))
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "characteristic"
        )),
        "expected the singular `characteristic` lemma pluralized under `those`\nAST:\n{ast}"
    );
}

#[test]
fn descended_intervening_condition_parses_as_a_verb_clause() {
    // Molten Collapse modal header: `If you descended this turn, you may choose
    // both instead.` The bare `both` object and trailing `instead` already
    // parse; the sole blocker was the verb `descend`, now a regular-vocabulary
    // verb whose ordinary past tense heads the `if` condition.
    let source = "If you descended this turn, you may choose both instead.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Molten Collapse", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
}

// --- R32: the flavor-word sentence-header cluster ---------------------------

/// The flavor words the header-cluster witnesses name, plus the one card-type
/// atom their bodies need. `Exterminate!` is a real member ending in `!`: the
/// terminal-punctuation gate must still route it to the old paragraph path.
fn flavor_catalogs() -> Catalogs {
    Catalogs::default()
        .with_catalog(
            CatalogKind::FlavorWord,
            [
                "Chaos",
                "Polymorphine",
                "Make Them Pay",
                "Sanctified Rules of Combat",
                "Aerial Blast",
                "Exterminate!",
            ],
        )
        .with_catalog(CatalogKind::CardType, ["Creature"])
}

#[test]
fn flavor_word_header_before_a_paragraph_peels_as_licensed_opacity() {
    // Callidus Assassin shape: a bare flavor-word label set off by a spaced em
    // dash blocked the whole ability. Peeling the catalog-member label leaves
    // the paragraph body to parse by the ordinary machinery, and the label is
    // reproduced verbatim as an ability-level flavor header (licensed opacity,
    // never recovery).
    let source = "Polymorphine — Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Callidus Assassin", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let header = only_ability(&ast)
        .flavor_header
        .as_ref()
        .unwrap_or_else(|| panic!("expected an ability flavor header: {ast}"));
    assert!(
        header.text() == "Polymorphine" && header.source_tokens() == 1,
        "expected an ability-level flavor-word header\nAST:\n{ast}"
    );
    assert!(matches!(only_ability(&ast).kind, AbilityKind::Paragraph(_)));
}

#[test]
fn flavor_word_header_before_a_trigger_lets_the_trigger_recover() {
    // Vincent, Vengeful Atoner shape: the label stands ahead of a trigger frame
    // no paragraph-level header could reach. Peeling it at the ability level
    // lets `Whenever …` recover as a triggered ability.
    let source = "Chaos — Whenever this creature attacks, draw a card.";
    let (rendered, ast) = parse_face(
        source,
        &flavor_catalogs(),
        "Vincent, Vengeful Atoner",
        false,
    );
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        matches!(
            only_ability(&ast),
            Ability {
                flavor_header: Some(header),
                kind: AbilityKind::Triggered(_),
                ..
            } if header.text() == "Chaos"
        ),
        "expected the trigger behind the label to recover\nAST:\n{ast}"
    );
}

#[test]
fn multi_word_flavor_word_header_peels_the_whole_label() {
    // `Make Them Pay` is a three-token flavor word: the peel matches the whole
    // catalog member, never a shorter prefix, and reproduces it verbatim.
    let source = "Make Them Pay — Draw a card.";
    let (rendered, ast) = parse_face(
        source,
        &flavor_catalogs(),
        "The Master, Gallifrey's End",
        false,
    );
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let header = only_ability(&ast)
        .flavor_header
        .as_ref()
        .unwrap_or_else(|| panic!("expected an ability flavor header: {ast}"));
    assert!(
        header.text() == "Make Them Pay" && header.source_tokens() == 3,
        "expected the whole three-token label as one flavor header\nAST:\n{ast}"
    );
}

#[test]
fn flavor_word_header_uncovers_a_villainous_choice_appositive() {
    // Midnight Crusader Shuttle / The Master cascade: behind the un-peeled label
    // sat a villainous-choice appositive the prior round's machinery already
    // handles. Once the label peels, the inner `— <or-coordinated>` attaches as
    // a `ComplexClause` appositive with no new grammar.
    let source =
        "Chaos — That player faces a villainous choice — They lose 4 life, or you create a token.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Sycorax Commander", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        only_ability(&ast)
            .flavor_header
            .as_ref()
            .is_some_and(|header| header.text() == "Chaos")
            && matches!(
                &only_ability(&ast).kind,
                AbilityKind::Paragraph(Paragraph { sentences, .. }) if matches!(
                    sentences.as_slice(),
                    [Sentence {
                        body: SentenceBody::Independent(clause),
                        ..
                    }] if appositive(clause).is_some()
                )
            ),
        "expected the inner villainous-choice appositive to parse\nAST:\n{ast}"
    );
}

#[test]
fn flavor_word_header_inside_a_saga_chapter_body_peels() {
    // Summon: Primal Garuda shape: a saga chapter body opens with a flavor-word
    // label (`I — Aerial Blast — <effect>`). The chapter header `I` peels
    // structurally at the ability level; the paragraph-level peel then takes the
    // catalog-member label off the chapter body so the effect recovers. This is
    // the paragraph-start half of the licensing, one level below the trigger
    // case, reached only after the chapter frame splits the header.
    let source = "I — Aerial Blast — This creature deals 4 damage to target creature.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Summon: Primal Garuda", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        matches!(
            &only_ability(&ast).kind,
            AbilityKind::Chapter(ChapterAbility { body, .. })
                if body
                    .flavor_header
                    .as_ref()
                    .is_some_and(|header| header.text() == "Aerial Blast")
        ),
        "expected the chapter body's flavor-word header to peel\nAST:\n{ast}"
    );
}

#[test]
fn saga_chapter_roman_header_is_not_a_flavor_word() {
    // Over-fire gate (a): a saga chapter's `I — <body>` carries a spaced em
    // dash, but `I` is not a flavor-word member, so the peel declines and the
    // chapter frame keeps it. Nothing peels as a flavor header.
    let source = "I — Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Test Saga", false);
    assert_eq!(rendered, source);
    assert!(
        only_ability(&ast).flavor_header.is_none()
            && matches!(
                &only_ability(&ast).kind,
                AbilityKind::Chapter(ChapterAbility { body, .. })
                    if body.flavor_header.is_none()
            ),
        "a roman chapter header must not peel as a flavor word\nAST:\n{ast}"
    );
}

#[test]
fn a_capitalized_non_member_label_stays_recovered() {
    // Over-fire gate (b): a capitalized label that is not a catalog member is
    // never peeled — it falls through to a verbatim recovered span exactly as
    // before, so the peel is strictly catalog-licensed.
    let source = "Foobar — Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        only_ability(&ast).flavor_header.is_none() && only_paragraph(&ast).flavor_header.is_none(),
        "a non-member label must not peel\nAST:\n{ast}"
    );
    assert!(!ast.recoveries().is_empty(), "AST:\n{ast}");
}

#[test]
fn a_terminal_punctuation_flavor_member_keeps_the_paragraph_path() {
    // Over-fire gate (c): `Exterminate!` is a real flavor-word member, but it
    // ends in inert terminal punctuation, so the ability-level peel declines and
    // the existing paragraph-level flavor-header peel keeps it. The header lands
    // on the paragraph, not the ability.
    let source = "Exterminate! — Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        only_ability(&ast).flavor_header.is_none() && only_paragraph(&ast).flavor_header.is_some(),
        "an exclamation-terminated member must keep the paragraph flavor-header path\nAST:\n{ast}"
    );
}

#[test]
fn a_cost_flavor_header_with_a_non_member_label_is_unaffected() {
    // Over-fire gate (d): the activation-cost flavor-header peel is untouched. A
    // bare non-member label ahead of a cost still lands on the cost, never the
    // ability, so activation-cost handling does not move.
    let source = "Sneaky — {T}: Draw a card.";
    let (rendered, ast) = parse_face(source, &flavor_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    let ability = only_ability(&ast);
    assert!(
        matches!(
            &ability.kind,
            AbilityKind::Activated(ActivatedAbility { cost, .. })
                if cost.flavor_header.is_some()
        ),
        "a non-member cost label must stay a cost flavor header\nAST:\n{ast}"
    );
    assert!(
        ability.ability_word.is_none() && ability.flavor_header.is_none(),
        "the ability-level flavor slot must stay empty for a cost header\nAST:\n{ast}"
    );
}

// --- R33: the lexical-sweep round --------------------------------------------

/// The card-type atoms the lexical-sweep witnesses name.
fn r33_catalogs() -> Catalogs {
    Catalogs::default().with_catalog(
        CatalogKind::CardType,
        [
            "Creature",
            "Land",
            "Artifact",
            "Enchantment",
            "Planeswalker",
        ],
    )
}

#[test]
fn one_is_a_dispreferenced_fused_head_noun() {
    // Touch of Darkness shape: sentence-initial `One` coordinated with `more`
    // has no following head noun, so `one` fills the fused head itself — the
    // sibling of `other`'s fused head, dispreferenced the same way
    // (`is_fused_head_noun`) so it never outranks the ordinary numeral reading.
    let source = "One or more target creatures become black until end of turn.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Touch of Darkness", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .has_noun(|noun| matches!(noun, NounInstance::Singular(Noun::Word(Vocab::One)))),
        "expected `One` to head the fused-head nominal\nAST:\n{ast}"
    );
}

#[test]
fn number_literal_one_still_wins_over_the_noun_reading() {
    // The gate: `one` is scanned as a count noun only to license the fused
    // head above. In an ordinary quantity position the numeral reading — equal
    // in every other structural cost — must still win the tiebreak, never a
    // `Noun::Word(One)`.
    let source = "Return up to one target creature card from your graveyard to the battlefield.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Badlands Revival", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let inventory = SyntaxInventory::from_ast(&ast);
    assert!(
        inventory.nominals.iter().any(|nominal| matches!(
            nominal.determiner,
            Some(Determiner::Target(Some(Quantity::UpTo(
                QuantityValue::Literal(NumberLiteral { value: 1, .. })
            ))))
        )),
        "expected `one` as a number literal\nAST:\n{ast}"
    );
    assert!(
        !inventory.has_noun_lexeme(|noun| matches!(noun, Noun::Word(Vocab::One))),
        "`one` must not reduce as a fused-head noun in quantity position\nAST:\n{ast}"
    );
}

#[test]
fn itself_reflexive_pronoun_round_trips() {
    // Asmoranomardicadaistinaculdacar shape: a creature-subject reflexive
    // object, the sibling of `each_other` at the same object-case pronoun
    // slot — no new grammar, just a new pronoun identity.
    let source = "Target creature deals 6 damage to itself.";
    let (rendered, ast) = parse_face(
        source,
        &r33_catalogs(),
        "Asmoranomardicadaistinaculdacar",
        false,
    );
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .pronouns
            .contains(&Pronoun::Itself),
        "expected the reflexive pronoun `itself`\nAST:\n{ast}"
    );
}

#[test]
fn himself_reflexive_pronoun_round_trips() {
    // Sarkhan the Mad shape: the same object-case reflexive identity, standing
    // in for a legendary planeswalker subject.
    let source = "Sarkhan deals damage to himself equal to that card's mana value.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Sarkhan the Mad", true);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .pronouns
            .contains(&Pronoun::Himself),
        "expected the reflexive pronoun `himself`\nAST:\n{ast}"
    );
}

#[test]
fn yours_absolute_pronoun_round_trips() {
    // Geyser Drake shape: the absolute possessive `yours`, a third object-case
    // identity at the same existing pronoun slot.
    let source = "During turns other than yours, spells you cast cost {1} less to cast.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Geyser Drake", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .pronouns
            .contains(&Pronoun::YoursAbsolute),
        "expected the absolute possessive pronoun `yours`\nAST:\n{ast}"
    );
}

#[test]
fn fewest_is_a_fused_head_superlative_noun() {
    // Balance shape: `the fewest` has no following head noun, so the
    // superlative itself heads the nominal (plain noun sense, no dispreference
    // needed — `fewest` has no competing reading).
    let source = "Each player chooses a number of lands they control equal to the number of \
        lands controlled by the player who controls the fewest, then sacrifices the rest.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Balance", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .has_noun_lexeme(|noun| matches!(noun, Noun::Word(Vocab::Fewest))),
        "expected `fewest` to head the fused-head nominal\nAST:\n{ast}"
    );
}

#[test]
fn most_is_an_attributive_superlative_adjective() {
    // No Witnesses shape: `the most creatures` modifies a following plural
    // head noun, exercising the adjective sense of the same `most` entry.
    let source = "Each player who controls the most creatures investigates.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "No Witnesses", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .has_adjective(|adjective| matches!(adjective, Adjective::Word(Vocab::Most))),
        "expected `most` as the superlative modifier\nAST:\n{ast}"
    );
}

#[test]
fn nearest_is_an_attributive_superlative_adjective() {
    // Mystic Barrier shape: `the nearest opponent` uses the same attributive
    // superlative pattern as `most`.
    let source = "Each player may attack only the nearest opponent in the last chosen direction \
        and planeswalkers controlled by that opponent.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Mystic Barrier", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let inventory = SyntaxInventory::from_ast(&ast);
    assert!(
        inventory.has_adjective(|adjective| matches!(adjective, Adjective::Word(Vocab::Nearest)))
            && !inventory.has_noun(|noun| matches!(
                noun,
                NounInstance::Singular(Noun::Word(Vocab::Nearest))
            )),
        "expected `nearest` as the superlative modifier\nAST:\n{ast}"
    );
}

#[test]
fn nearest_retains_its_fused_head_noun_reading() {
    let source = "Choose the nearest.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let inventory = SyntaxInventory::from_ast(&ast);
    assert!(
        inventory
            .has_noun(|noun| matches!(noun, NounInstance::Singular(Noun::Word(Vocab::Nearest))))
            && !inventory
                .has_adjective(|adjective| matches!(adjective, Adjective::Word(Vocab::Nearest))),
        "expected `nearest` to remain a fused nominal head\nAST:\n{ast}"
    );
}

#[test]
fn bid_bidding_and_bidder_forms_round_trip() {
    // Illicit Auction shape: the irregular verb `bid` (bid/bidding/bid), its
    // gerund-as-noun `the bidding` (the existing nominalization rule, no
    // separate vocabulary entry), and the derived agent noun `bidder` all
    // combine in one fully structured auction sequence.
    let source = "Each player may bid life for control of target creature. \
        You start the bidding with a bid of 0. \
        In turn order, each player may top the high bid. \
        The bidding ends if the high bid stands. \
        The high bidder loses life equal to the high bid and gains control of the creature.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Illicit Auction", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .has_noun_lexeme(|noun| matches!(noun, Noun::Agentive(Verb::Word(Vocab::Bid)))),
        "expected `bidder` to retain its source verb\nAST:\n{ast}"
    );
}

#[test]
fn explicit_rules_nouns_win_over_productive_agent_readings() {
    let source = "Each player draws a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let inventory = SyntaxInventory::from_ast(&ast);
    assert!(
        inventory.has_noun_lexeme(|noun| matches!(noun, Noun::Word(Vocab::Player))),
        "expected the explicit rules noun `player`\nAST:\n{ast}"
    );
    assert!(
        !inventory.has_noun_lexeme(|noun| matches!(noun, Noun::Agentive(Verb::Word(Vocab::Play)))),
        "the derived reading must not outrank the explicit noun\nAST:\n{ast}"
    );
}

#[test]
fn crew_ordinary_verb_coexists_with_the_crew_keyword_action() {
    // Canyon Vaulter shape: `crews` as an ordinary third-person verb in
    // running prose, distinct from (and non-conflicting with) the catalog-
    // driven `Crew N` keyword-action cost line.
    let source =
        "Whenever this creature crews an artifact, that artifact gains flying until end of turn.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Canyon Vaulter", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_verb(|verb| matches!(
            verb,
            Verb::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "crew"
        )),
        "expected `crew` as an ordinary verb\nAST:\n{ast}"
    );
}

#[test]
fn escape_verb_round_trips() {
    // Chainweb Aracnir shape: `escapes` as an ordinary present-tense verb,
    // distinct from the unrelated `Escape—<cost>` keyword-cost header line
    // (a separate, unspaced-em-dash frame this round does not touch).
    let source = "This creature escapes with three +1/+1 counters on it.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Chainweb Aracnir", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_verb(|verb| matches!(
            verb,
            Verb::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "escape"
        )),
        "expected `escape` as an ordinary verb\nAST:\n{ast}"
    );
}

#[test]
fn everything_mass_noun_round_trips() {
    // Hexdrinker shape (`Protection from everything`) is a bare verbless
    // level-body line, licensed only inside the class-level frame this round
    // does not touch; the same `everything`-as-bare-object-of-`from` shape
    // exercised as an ordinary clause instead.
    let source = "Enchanted creature has protection from everything.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_noun(|noun| matches!(
            noun,
            NounInstance::Mass(Noun::Word(vocab))
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "everything"
        )),
        "expected `everything` as a mass noun\nAST:\n{ast}"
    );
}

#[test]
fn void_voter_and_fame_nouns_round_trip() {
    // Dauthi Voidwalker (`void` as a noun-modifier in the productive `<word>
    // counter` pattern), Elrond of the White Council (`voter` as a bare
    // subject noun), and Seize the Spotlight (`fame` as a bare object noun).
    let void = "If a card would be put into an opponent's graveyard from anywhere, instead \
        exile it with a void counter on it.";
    let (rendered, ast) = parse_face(void, &Catalogs::default(), "Dauthi Voidwalker", false);
    assert_eq!(rendered, void);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_noun_lexeme(|noun| matches!(
            noun,
            Noun::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "void"
        )),
        "expected `void` as a noun modifier\nAST:\n{ast}"
    );

    let voter = "For each fellowship vote, the voter chooses a creature they control.";
    let (rendered, ast) = parse_face(voter, &r33_catalogs(), "Elrond of the White Council", true);
    assert_eq!(rendered, voter);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .has_noun_lexeme(|noun| matches!(noun, Noun::Agentive(Verb::Word(Vocab::Vote)))),
        "expected `voter` to retain its source verb\nAST:\n{ast}"
    );

    let fame = "Each opponent chooses fame or fortune.";
    let (rendered, ast) = parse_face(fame, &Catalogs::default(), "Seize the Spotlight", false);
    assert_eq!(rendered, fame);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_noun_lexeme(|noun| matches!(
            noun,
            Noun::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "fame"
        )),
        "expected `fame` as a bare noun\nAST:\n{ast}"
    );
}

#[test]
fn either_is_a_fused_head_pronoun_noun() {
    // Worms of the Earth shape: `does either` has no following head noun, so
    // `either` fills the fused head itself.
    let source = "If a player does either, destroy this enchantment.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Worms of the Earth", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_noun_lexeme(|noun| matches!(
            noun,
            Noun::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "either"
        )),
        "expected `either` as a fused-head noun\nAST:\n{ast}"
    );
}

#[test]
fn much_and_many_are_fused_head_nouns_with_a_partitive_complement() {
    // Mana Reflection (`as much of that mana`) and Vorinclex, Monstrous Raider
    // (`that many of each of those kinds of counters`): both fill a partitive
    // `of`-complement head the dedicated `Quantity::ThatMuch`/`ThatMany`
    // determiner shapes don't reach — that shape only covers a simple
    // determined noun as the partitive whole, not a nested partitive.
    let much = "If you tap a permanent for mana, it produces twice as much of that mana instead.";
    let (rendered, ast) = parse_face(much, &Catalogs::default(), "Mana Reflection", false);
    assert_eq!(rendered, much);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_noun_lexeme(|noun| matches!(
            noun,
            Noun::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "much"
        )),
        "expected `much` as a fused-head noun\nAST:\n{ast}"
    );

    let many = "If you would put one or more counters on a permanent or player, put twice that \
        many of each of those kinds of counters on that permanent or player instead.";
    let (rendered, ast) = parse_face(
        many,
        &Catalogs::default(),
        "Vorinclex, Monstrous Raider",
        true,
    );
    assert_eq!(rendered, many);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_noun_lexeme(|noun| matches!(
            noun,
            Noun::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "many"
        )),
        "expected `many` as a fused-head noun\nAST:\n{ast}"
    );
}

#[test]
fn print_and_expansion_nouns_round_trip() {
    // Apocalypse Chime shape: `printed` (the regular verb `print`'s past
    // participle) and `expansion` (a regular noun) complete the surrounding
    // reduced-relative and prepositional-phrase shapes. `originally` stays
    // out of vocabulary this round (its adverb sense regressed elsewhere — see
    // the residue notes), so it fills the reduced relative's subject slot as
    // licensed opacity rather than as an adverb; this is a real, measured,
    // already-landed improvement over the prior full-clause failure.
    let source = "Destroy all nontoken permanents with a name originally printed in the \
        Homelands expansion. They can't be regenerated.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Apocalypse Chime", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let inventory = SyntaxInventory::from_ast(&ast);
    assert!(
        inventory.has_verb(|verb| matches!(
            verb,
            Verb::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "print"
        )),
        "expected `print` as the reduced-relative verb\nAST:\n{ast}"
    );
    assert!(
        inventory.has_noun_lexeme(|noun| matches!(
            noun,
            Noun::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "expansion"
        )),
        "expected `expansion` as a bare noun\nAST:\n{ast}"
    );
}

#[test]
fn level_and_rad_nouns_round_trip() {
    // The Class-card `level counter` idiom and Mariposa Military Base's `rad
    // counter`: both are the same productive `<word> counter(s)` noun-
    // modifier pattern as `void` above.
    let level = "{1}: Put a level counter on this.";
    let (rendered, ast) = parse_face(level, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, level);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_noun_lexeme(|noun| matches!(
            noun,
            Noun::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "level"
        )),
        "expected `level` as a noun modifier\nAST:\n{ast}"
    );

    let rad = "You may have this land enter tapped. If you do, you get two rad counters.";
    let (rendered, ast) = parse_face(rad, &r33_catalogs(), "Mariposa Military Base", false);
    assert_eq!(rendered, rad);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_noun_lexeme(|noun| matches!(
            noun,
            Noun::Word(vocab)
                if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "rad"
        )),
        "expected `rad` as a noun modifier\nAST:\n{ast}"
    );
}

#[test]
fn between_preposition_licenses_a_coordinated_number_range() {
    // By Invitation Only shape: `between 0 and 13` is the primary corpus
    // motivation for the new `Preposition::Between` variant — a plain
    // addition to the closed preposition set feeding the existing generic
    // `PrepositionalPhrase` production, not a new chart rule.
    let source = "Choose a number between 0 and 13.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "By Invitation Only", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .prepositions
            .contains(&Preposition::Between),
        "expected the new `between` preposition\nAST:\n{ast}"
    );
}

#[test]
fn a_leveler_face_round_trips_through_its_level_bands() {
    // Beastbreaker of Bala Ged's reminder-stripped body: two level bands, one
    // holding no contained ability and one holding a single keyword. The
    // whole-sentence bracket read's absence assertion — no `Recovered`
    // anywhere on the face — proves both the header and the stat line are
    // carried structurally rather than recovering.
    let source = "Level up {2}{G}\nLEVEL 1-3\n4/4\nLEVEL 4+\n6/6\nTrample";
    let catalogs = Catalogs::new(
        ["Level up", "Trample"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    );
    let (rendered, ast) = parse_face(source, &catalogs, "Beastbreaker of Bala Ged", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
}

#[test]
fn a_station_face_round_trips_through_its_threshold_rows() {
    // Hearthhull, the Worldseed's reminder-stripped body: an activated
    // threshold row, a three-keyword threshold row, and a trailing [CR#721.4]
    // always-on sibling ability. The whole-sentence bracket read's absence
    // assertion proves no threshold row lowers as a `RollRow` or recovers.
    let source = "Station\n2+ | {1}, {T}, Sacrifice a land: Draw two cards. You may play an \
        additional land this turn.\n8+ | Flying, vigilance, haste\nWhenever you sacrifice a \
        land, each opponent loses 2 life.";
    let catalogs = Catalogs::new(
        ["Station", "Flying", "Vigilance", "Haste"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    );
    let (rendered, ast) = parse_face(source, &catalogs, "Hearthhull, the Worldseed", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        ast.abilities
            .iter()
            .all(|ability| !matches!(ability.kind, AbilityKind::RollRow(_))),
        "AST:\n{ast}"
    );
}

#[test]
fn between_preposition_licenses_the_difference_between_shape() {
    // Jaws of Defeat shape (the `difference between X and Y` idiom): a second,
    // independent corpus motivation for `Preposition::Between`, guarding the
    // new enum variant's exhaustive wiring (scanner, renderer, and the
    // `PrepositionalPhrase` production) against a narrower reading that only
    // happened to fit the number-range case above.
    let source = "That player loses life equal to the difference between its power and its \
        toughness.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .prepositions
            .contains(&Preposition::Between),
        "expected the new `between` preposition\nAST:\n{ast}"
    );
}

// --- `kwverbs`: ability-derived keyword-action verbs (`mutate`) -----------
//
// `mutate` is a keyword *ability* [CR#702], not a keyword action [CR#701],
// so Scryfall's `keyword-actions` catalog never lists it. Its CR-defined
// verb usage (`this creature mutates`) is added as a hand-curated
// `Verb::KeywordAction` supplement (see the durability comment at the merge
// site in `crates/deckmaste_english/src/catalog.rs`, `Catalogs::rebuild`).
//
// `exploit` was attempted alongside `mutate` in this round's Gate B and
// dropped: it created a competing verb reading for the keyword-ability atom
// `exploit` filling a bare object slot on a coordinated-subject face (Henry
// Wu, InGen Geneticist — see
// `keyword_ability_object_stays_an_atom_and_verb_stays_out` below, and ticket
// `english-ability-derived-verb-batch`).

#[test]
fn mutate_trigger_event_parses() {
    // Archipelagore shape: `Whenever this creature mutates,` plus the
    // `has mutated` count-noun collateral, in one sentence.
    let source = "Whenever this creature mutates, tap up to X target creatures, where X is \
        the number of times this creature has mutated.";
    let catalogs = Catalogs::new(
        ["Mutate"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    );
    let (rendered, ast) = parse_face(source, &catalogs, "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast).has_verb(|verb| matches!(verb, Verb::KeywordAction(_))),
        "AST:\n{ast}"
    );
}

#[test]
fn exploit_trigger_event_parses_with_its_object() {
    // Diver Skaab shape: `When this creature exploits a creature,` — with
    // `exploit` reverted (Gate B failed the over-fire check below), this
    // stays an absence assertion: the object must NOT strand as a separate
    // clause the way a wrong tree would, but `exploits` is not yet a known
    // verb, so the whole trigger recovers as one span.
    let source = "When this creature exploits a creature, target creature's owner puts it \
        on their choice of the top or bottom of their library.";
    let catalogs = Catalogs::new(
        ["Exploit"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    );
    let (rendered, ast) = parse_face(source, &catalogs, "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        !SyntaxInventory::from_ast(&ast).has_verb(|verb| matches!(verb, Verb::KeywordAction(_))),
        "`exploit` is reverted (Gate B failed); it must not render as a verb:\nAST:\n{ast}"
    );
    assert!(
        !ast.recoveries().is_empty(),
        "the whole `exploits` trigger should still recover as one span:\nAST:\n{ast}"
    );
}

#[test]
fn keyword_ability_line_is_not_a_verb_clause() {
    // Both keyword lines still classify as keyword abilities, not clauses.
    // `mutate` is now a known verb; `exploit` is not (reverted).
    let source = "Mutate {5}{U}\nExploit";
    let catalogs = Catalogs::new(
        ["Mutate", "Exploit"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    );
    let (rendered, ast) = parse_face(source, &catalogs, "Test Card", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        !SyntaxInventory::from_ast(&ast).has_verb(|verb| matches!(verb, Verb::KeywordAction(_))),
        "AST:\n{ast}"
    );
}

#[test]
fn keyword_ability_object_stays_an_atom_and_verb_stays_out() {
    // Henry Wu, InGen Geneticist shape (the round's Gate B failure): `Henry
    // Wu and other Human creatures you control have exploit.` — on this
    // coordinated-subject face, a hand-curated `exploit` verb entry flips
    // `exploit` from the keyword-ability atom filling `have`'s object into a
    // second, wrongly coordinated intransitive predicate. `exploit` is NOT
    // in the hand-curated table (see `ABILITY_DERIVED_KEYWORD_ACTION_VERBS`
    // in `catalog.rs`), so this must stay the atom reading. This test is the
    // regression guard against silently re-adding `exploit` without first
    // fixing that over-fire.
    let source = "Henry Wu and other Human creatures you control have exploit.\nWhenever a \
        creature you control exploits a non-Human creature, draw a card.";
    let catalogs = Catalogs::new(
        ["Exploit"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    )
    .with_catalog(CatalogKind::CreatureType, ["Human"]);
    let (rendered, ast) = parse_face(source, &catalogs, "Test Card", false);
    assert_eq!(rendered, source);
    assert!(
        !SyntaxInventory::from_ast(&ast).has_verb(|verb| matches!(verb, Verb::KeywordAction(_))),
        "`exploit` must not render as a verb anywhere while it is reverted:\nAST:\n{ast}"
    );
}

#[test]
fn the_championed_trigger_is_still_missing_and_guarded() {
    // The `championed` *verb* stays OUT (only one corpus witness; see the
    // round's plan), so Mistbind Clique's trigger sentence must still recover
    // and the `copular_complement_head_is_opaque` guard (grammar/ability.rs)
    // stays load-bearing, keeping that trigger from camouflaging as a
    // copular/opaque noun reading.
    //
    // Its keyword line `Champion a Faerie` no longer recovers: round `enchant`
    // gave the keyword-argument slot the bare noun phrase [CR#702.72a] writes
    // for champion, so exactly one span is left.
    let source = "Champion a Faerie\nWhen a Faerie is championed with this creature, tap all \
        lands target player controls.";
    let catalogs = Catalogs::new(
        ["Champion"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    )
    .with_catalog(CatalogKind::CreatureType, ["Faerie"]);
    let (rendered, ast) = parse_face(source, &catalogs, "Test Card", false);
    assert_eq!(rendered, source);
    assert_eq!(
        ast.recoveries().len(),
        1,
        "expected only the `championed` trigger to still recover:\nAST:\n{ast}"
    );
}

#[test]
fn mutate_and_exploit_round_trip() {
    // Render-back equality for `mutate`'s two in-scope verb forms (landed);
    // `exploit`'s two forms are included as reverted-absence checks (Gate B
    // failed, see above) so a future re-add is exercised here too.
    let catalogs = Catalogs::new(
        ["Mutate", "Exploit"],
        std::iter::empty::<&str>(),
        std::iter::empty::<&str>(),
    );
    for source in [
        "Whenever this creature mutates, scry 2.",
        "Whenever this creature mutates, tap up to X target creatures, where X is the \
            number of times this creature has mutated.",
        "When this creature exploits a creature, draw a card.",
        "The exploited creature's toughness becomes 0 until end of turn.",
    ] {
        let (rendered, ast) = parse_face(source, &catalogs, "Test Card", false);
        assert_eq!(rendered, source, "AST:\n{ast}");
    }
}

// --- qfloat: finite verbal quantifier float (Stage 2) -----------------------

#[test]
fn qfloat_core_cardinal_subject_each_gets() {
    let source = "Two target creatures each get +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        matrix_distributive_each(&ast),
        "expected the floated `each` flag on the predicate head\nAST:\n{ast}"
    );
}

#[test]
fn qfloat_core_pronoun_and_target_each_draw() {
    let source = "You and target opponent each draw a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(matrix_distributive_each(&ast), "AST:\n{ast}");
}

#[test]
fn qfloat_core_they_each_deal_damage() {
    let source = "They each deal damage equal to their power to target creature.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(matrix_distributive_each(&ast), "AST:\n{ast}");
}

#[test]
fn qfloat_core_up_to_n_each_gain_haste() {
    let source = "Up to two target creatures each gain haste until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(matrix_distributive_each(&ast), "AST:\n{ast}");
}

#[test]
fn qfloat_core_one_or_two_each_gets_lowers_as_coordinated_np() {
    // Stage 1 (`Two`/`One` cardinal case-insensitivity) is dropped this round,
    // so the subject is NOT `Determiner::Target(Some(Quantity::Or(1, 2)))` — it
    // lowers as the pre-existing coordinated fused-head tree, the same
    // analysis the grammar test-locks for `One or more target creatures`
    // (`one_is_a_dispreferenced_fused_head_noun`). Assert the float and the
    // predicate, not `Quantity::Or(1, 2)`.
    let source = "One or two target creatures each get +2/+1 until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(matrix_distributive_each(&ast), "AST:\n{ast}");
    assert!(
        matches!(
            clause_subject(only_independent_clause(&ast)),
            Some(NounPhrase::Coordinated(CoordinatedNounPhrase { first, .. }))
                if matches!(
                    first.as_ref(),
                    NounPhrase::Nominal(NominalPhrase {
                        head: NounInstance::Singular(Noun::Word(Vocab::One)),
                        ..
                    })
                )
        ),
        "expected the pre-existing coordinated fused-head `One` reading\nAST:\n{ast}"
    );
    assert!(
        !matches!(
            clause_subject(only_independent_clause(&ast)),
            Some(NounPhrase::Nominal(NominalPhrase {
                determiner: Some(Determiner::Target(Some(Quantity::Or(_, _)))),
                ..
            }))
        ),
        "Stage 1 is dropped this round; the subject must not become a single \
         `Quantity::Or(1, 2)` nominal\nAST:\n{ast}"
    );
}

#[test]
fn qfloat_core_relative_that_each_have() {
    let source = "Target creature cards that each have a different mana value get +1/+1.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .predicate_heads
            .iter()
            .any(|head| head.distributive_each
                && matches!(head.verb.verb, Verb::Word(Vocab::Have))),
        "expected the relative-clause distributive float to attach\nAST:\n{ast}"
    );
}

#[test]
fn qfloat_core_you_each_put() {
    let source = "You each put the card you revealed into your hand.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(matrix_distributive_each(&ast), "AST:\n{ast}");
}

#[test]
fn qfloat_core_those_players_each_discard() {
    let source = "Those players each discard two cards at random.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(matrix_distributive_each(&ast), "AST:\n{ast}");
}

// --- qfloat: named anti-misparse gates --------------------------------------

#[test]
fn qfloat_anti_misparse_each_sacrifice_stays_verbal() {
    let source = "You and that player each sacrifice a creature.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        matches!(
            matrix_predicate_head(only_independent_clause(&ast)),
            Some(PredicateHead {
                verb: VerbInstance {
                    verb: Verb::Word(Vocab::Sacrifice),
                    ..
                },
                ..
            })
        ),
        "`sacrifice` must be a transitive finite verb, never a nominal reading \
         inside the discarded `each` slot\nAST:\n{ast}"
    );
}

#[test]
fn qfloat_anti_misparse_each_discard_stays_verbal() {
    let source = "Those players each discard a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        matches!(
            matrix_predicate_head(only_independent_clause(&ast)),
            Some(PredicateHead {
                verb: VerbInstance {
                    verb: Verb::Word(Vocab::Discard),
                    ..
                },
                ..
            })
        ),
        "`discard` must be a transitive finite verb\nAST:\n{ast}"
    );
}

#[test]
fn qfloat_anti_misparse_singular_subject_does_not_admit_the_float() {
    // The dot-1 host gate in `accepts_predicate_prefix` requires a plural or
    // second-person subject; `Target creature` is third-person singular, so
    // `each` cannot be scanned there and the whole sentence stays unparsed by
    // the target grammar (a full `Recovered` clause), never a wrong tree.
    for source in [
        "Target creature each gets +1/+1.",
        "Target creature each discard a card.",
    ] {
        let (_, ast) = parse_face(source, &library_catalogs(), "Test Card", false);
        assert!(
            !ast.recoveries().is_empty(),
            "`{source}` must not complete under the new productions\nAST:\n{ast}"
        );
    }
}

#[test]
fn qfloat_anti_misparse_singular_antecedent_relative_no_float() {
    // A singular antecedent's relative `each` (`gets`, third-singular) fails
    // the plural-agreement reduce gate in `RelativeSubjectDistributiveEach`,
    // so the whole sentence stays unparsed rather than silently completing on
    // a mismatched antecedent.
    let source = "Target creature that each gets +1/+1 dies.";
    let (_, ast) = parse_face(source, &library_catalogs(), "Test Card", false);
    assert!(
        !ast.recoveries().is_empty(),
        "a singular antecedent must not acquire the relative float\nAST:\n{ast}"
    );
}

#[test]
fn qfloat_anti_misparse_goblin_game_pronominal_each_unchanged() {
    let source =
        "If two or more players are tied for fewest, each loses half their life, rounded up.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert!(
        SyntaxInventory::from_ast(&ast)
            .predicate_heads
            .iter()
            .all(|head| !head.distributive_each),
        "the pronominal `each` subject must not reach the new float tag or \
         `PredicateHead::distributive_each`\nAST:\n{ast}"
    );
}

// --- anof: notional plural concord for `any number of` (Stage A) -----------

#[test]
fn anof_concord_any_number_of_target_players_draw() {
    let source = "Any number of target players draw a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        clause_subject(only_independent_clause(&ast)).is_some_and(is_any_number_of),
        "expected the ordinary nominal head/complement shape\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_any_number_of_target_creatures_get() {
    let source = "Any number of target creatures get +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        clause_subject(only_independent_clause(&ast)).is_some_and(is_any_number_of),
        "expected the ordinary nominal head/complement shape\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_any_number_of_target_creatures_are_red() {
    let source = "Any number of target creatures are red until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        clause_subject(only_independent_clause(&ast)).is_some_and(is_any_number_of),
        "expected the ordinary nominal head/complement shape\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_any_number_of_target_players_each_discard() {
    let source = "Any number of target players each discard a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        clause_subject(only_independent_clause(&ast)).is_some_and(is_any_number_of),
        "expected the ordinary nominal head/complement shape\nAST:\n{ast}"
    );
    assert!(
        matrix_distributive_each(&ast),
        "floated forms must also carry `PredicateHead::distributive_each`\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_any_number_of_target_players_each_mill() {
    let source = "Any number of target players each mill two cards.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        clause_subject(only_independent_clause(&ast)).is_some_and(is_any_number_of),
        "expected the ordinary nominal head/complement shape\nAST:\n{ast}"
    );
    assert!(
        matrix_distributive_each(&ast),
        "floated forms must also carry `PredicateHead::distributive_each`\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_singular_predicate_retains_singular_agreement() {
    // ORCHESTRATOR CORRECTION A2: a singular finite verb must keep the
    // formal-singular reading. Zero corpus faces exercise this; it is a
    // synthetic guard on the `precedence: 1` registration cost.
    let source = "Any number of target creatures gets +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        clause_subject(only_independent_clause(&ast)).is_some_and(is_any_number_of),
        "expected the ordinary nominal head/complement shape\nAST:\n{ast}"
    );
    assert!(
        !matrix_distributive_each(&ast),
        "a singular predicate must not float `each` or select the notional \
         reading\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_the_number_of_singular_predicate_unchanged() {
    let source = "The number of target creatures gets +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
}

#[test]
fn anof_concord_the_number_of_plural_predicate_does_not_recover() {
    // `the` is not the closed-class `any` determiner the new production
    // scans, so this must remain the pre-existing unresolved shape rather
    // than gain the notional analysis.
    let source = "The number of target creatures get +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert!(
        !ast.recoveries().is_empty(),
        "`the number of ... get ...` must not gain the new `any`-keyed \
         production\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_each_of_partitive_singular_agreement_unchanged() {
    let source = "Each of those creatures gets +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
}

#[test]
fn anof_concord_one_of_partitive_singular_agreement_unchanged() {
    let source = "One of those creatures gets +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
}

#[test]
fn anof_concord_singular_final_noun_phrase_cannot_use_notional_production() {
    // The final noun phrase is singular (`target creature`), so the new
    // production's reduce-time plural check must decline; only the ordinary
    // (unresolved-singular-predicate) parse is possible here.
    let source = "Any number of target creature gets +2/+2 until end of turn.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert!(
        !matrix_distributive_each(&ast),
        "a singular final noun phrase must not license the notional \
         production\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_curse_of_surveillance_tree_is_sound() {
    // Curse of Surveillance: the effect-matrix predicate is `draw`, its
    // subject is exactly the `any number of target players other than that
    // player` nominal, and `attached to that player` stays inside the
    // object-side nominal material. The pre-stage giant-subject/`attach`
    // matrix tree must be absent.
    let source = "Enchant player\nAt the beginning of enchanted player's upkeep, any number \
        of target players other than that player each draw cards equal to the number of \
        Curses attached to that player.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Curse of Surveillance", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    let clause = last_effect_clause(&ast);
    assert!(
        clause_subject(clause).is_some_and(is_any_number_of),
        "expected the ordinary nominal head/complement shape as the subject\nAST:\n{ast}"
    );
    assert!(
        matrix_predicate_head(clause).is_some_and(|head| head.distributive_each),
        "expected the floated `each` on the `draw` predicate\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_launch_the_fleet_embedded_recovery_exposed() {
    // Launch the Fleet: the outer float resolves and the walker reports the
    // newly visible embedded-rules recovery inside the quote. Do not hide
    // that recovery to force the census prediction.
    let source = "Strive — This spell costs {1} more to cast for each target beyond the \
        first.\nUntil end of turn, any number of target creatures each gain \"Whenever \
        this creature attacks, create a 1/1 white Soldier creature token that's tapped \
        and attacking.\"";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Launch the Fleet", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    // The outer `any number of ... each gain` float resolves cleanly; the
    // quoted granted ability is expected to surface as an embedded-rules
    // recovery, and that recovery must not be hidden to force the census
    // prediction.
    assert!(
        matrix_predicate_head(last_effect_clause(&ast)).is_some_and(|head| head.distributive_each),
        "expected the outer float to resolve\nAST:\n{ast}"
    );
    assert!(
        !ast.recoveries().is_empty(),
        "the embedded-rules recovery inside the quote must remain visible\nAST:\n{ast}"
    );
}

#[test]
fn anof_concord_opaque_descendant_inside_of_complement_still_recovered() {
    // Proves the new lowering does not hide census material from the
    // recovery walker (`syntax/mod.rs:580-630` already traverses nominal
    // prepositional complements): an opaque one-word leaf nested inside the
    // `of` complement's own modifier still surfaces as `Opaque`, not silently
    // swallowed by the new production's lowering.
    let source = "Any number of target creatures with flying gobbledygook get +2/+2.";
    let (_rendered, ast) = parse_face(source, &Catalogs::default(), "Test Card", false);
    assert!(
        !ast.lexical_opacity().is_empty(),
        "an opaque descendant inside the `of` complement must still surface, \
         not be hidden by the new lowering\nAST:\n{ast}"
    );
}

// --- anof: sentence-initial capitalized cardinals (Stage B) ----------------

#[test]
fn anof_cardinal_two_target_players_exchange_life_totals() {
    // Soul Conduit shape, stripped of its activation cost.
    let source = "Two target players exchange life totals.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        ast.lexical_opacity().is_empty(),
        "sentence-initial `Two` must not become an opaque noun leaf\nAST:\n{ast}"
    );
    assert!(
        SyntaxInventory::from_ast(&ast)
            .nominals
            .iter()
            .any(|nominal| matches!(
                nominal,
                NominalPhrase {
                    determiner: Some(Determiner::Target(Some(Quantity::Exact(NumberLiteral {
                        value: 2,
                        numeral: Numeral::Cardinal,
                    })))),
                    head: NounInstance::Plural(Noun::Word(Vocab::Player)),
                    ..
                }
            )),
        "expected the quantified-target structure, not an opaque modifier\nAST:\n{ast}"
    );
}

#[test]
fn anof_cardinal_lowercase_object_position_unchanged() {
    let source = "Destroy two target creatures.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Test Card", false);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(ast.lexical_opacity().is_empty(), "AST:\n{ast}");
}

#[test]
fn anof_cardinal_numeral_codec_contract_untouched() {
    // The codec contract is untouched: only the grammar folds case.
    assert!(Numeral::Cardinal.parse("Two").is_err());
    assert!(Numeral::Cardinal.parse("two").is_ok());
}

#[test]
fn anof_cardinal_ordinal_and_lowercase_roman_keep_strict_behavior() {
    assert!(Numeral::Ordinal.parse("Second").is_err());
    assert!(Numeral::Ordinal.parse("second").is_ok());
    assert!(Numeral::Roman.parse("x").is_err());
    assert!(Numeral::Roman.parse("X").is_ok());
}

#[test]
fn anof_cardinal_faceless_one_retains_noun_reading() {
    // The full name is pre-collapsed to a single self-reference token before
    // the new cardinal case-fold retry ever sees a `One` token, so this
    // reading is untouched by Stage B.
    let source = "Whenever Faceless One attacks, draw a card.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Faceless One", true);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .this_cards
            .contains(&ThisCardForm::FullName),
        "AST:\n{ast}"
    );
}

#[test]
fn anof_cardinal_doubtless_ones_retains_noun_reading() {
    let source = "Doubtless One's power and toughness are each equal to the number of \
        Clerics on the battlefield.\nWhenever this creature deals damage, you gain that \
        much life.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "Doubtless One", true);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .this_cards
            .contains(&ThisCardForm::FullName),
        "AST:\n{ast}"
    );
}

#[test]
fn anof_cardinal_the_one_ring_retains_noun_reading() {
    let source = "Whenever The One Ring becomes the target of a spell or ability, \
        you lose 3 life.";
    let (rendered, ast) = parse_face(source, &Catalogs::default(), "The One Ring", true);
    assert_eq!(rendered, source, "AST:\n{ast}");
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .this_cards
            .contains(&ThisCardForm::FullName),
        "AST:\n{ast}"
    );
}

#[test]
fn anof_cardinal_one_or_more_fused_head_gate_stays_green() {
    // Named gate from `english-quantifier-float-residue.md` §1: must stay
    // green, unmodified by Stage B's case-folding retry.
    let source = "One or more target creatures become black until end of turn.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Touch of Darkness", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    assert!(
        SyntaxInventory::from_ast(&ast)
            .has_noun(|noun| matches!(noun, NounInstance::Singular(Noun::Word(Vocab::One)))),
        "expected `One` to still head the fused-head nominal\nAST:\n{ast}"
    );
}

#[test]
fn anof_cardinal_number_literal_one_still_wins_gate_stays_green() {
    let source = "Return up to one target creature card from your graveyard to the battlefield.";
    let (rendered, ast) = parse_face(source, &r33_catalogs(), "Badlands Revival", false);
    assert_eq!(rendered, source);
    assert_no_recovery(&ast);
    let inventory = SyntaxInventory::from_ast(&ast);
    assert!(
        inventory.nominals.iter().any(|nominal| matches!(
            nominal.determiner,
            Some(Determiner::Target(Some(Quantity::UpTo(
                QuantityValue::Literal(NumberLiteral { value: 1, .. })
            ))))
        )),
        "expected `one` as a number literal\nAST:\n{ast}"
    );
    assert!(
        !inventory.has_noun_lexeme(|noun| matches!(noun, Noun::Word(Vocab::One))),
        "`one` must not reduce as a fused-head noun in quantity position\nAST:\n{ast}"
    );
}
