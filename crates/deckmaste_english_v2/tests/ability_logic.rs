use std::collections::BTreeSet;
use std::num::NonZeroU32;
use std::path::Path;
use std::process::Command;

use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::render::Render as _;
use deckmaste_english_v2::visit::Visitor;
use macro_ron::v2::Onset;

fn parser() -> Parser {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    let environment = ParserEnvironment::try_from_parts(
        declarations,
        [CatalogProviderRows::new(
            CatalogProvider::CardNames,
            [CatalogProviderRow::new(
                "context-card",
                "Context Card",
                Onset::Consonant,
            )],
        )],
    )
    .expect("builtin-v2 declarations and catalog provider freeze");
    Parser::new(environment).expect("required declarations are present")
}

fn context(card_name: &str, is_legendary: bool) -> ParseContext<'_> {
    ParseContext::new(card_name, is_legendary, Onset::Consonant)
        .expect("test card name is a valid parse context")
}

#[test]
fn linguistic_bodies_enclose_plain_and_triggered_sentence_sequences() {
    let parser = parser();
    let plain_context = context("Context Card", false);
    let plain_text = "Destroy target creature.";
    let plain = parser
        .parse(plain_text, &plain_context)
        .expect("the ordinary sentence sequence parses as a plain ability");

    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(plain_sentences),
    }) = plain
    else {
        panic!("plain sentence sequence has the linguistic ability-body shape")
    };
    assert_eq!(plain_sentences.sentences().len(), 1);

    let triggered_text = "Whenever a player connives, you gain X life.";
    let triggered = parser
        .parse(triggered_text, &plain_context)
        .expect("the finite triggered sentence parses");
    let Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(finite),
        intervening_if: None,
        body: AbilityBody::Sentences(consequences),
    }) = triggered
    else {
        panic!("triggered ability stores its finite trigger and linguistic body")
    };
    assert_eq!(finite.marker, TriggerMarker::Whenever);
    assert!(matches!(finite.clause(), Clause::FiniteClause(_)));
    assert_eq!(consequences.sentences().len(), 1);
}

#[derive(Default)]
struct SelfReferenceVisitor {
    spellings: Vec<SelfReferenceSpelling>,
}

impl Visitor for SelfReferenceVisitor {
    fn visit_self_reference_spelling(&mut self, value: SelfReferenceSpelling) {
        self.spellings.push(value);
    }
}

#[derive(Default)]
struct AbilityEnvelopeVisitor(Vec<&'static str>);

impl Visitor for AbilityEnvelopeVisitor {
    fn visit_at_boundary(&mut self, _value: AtBoundary) {
        self.0.push("AtBoundary");
    }

    fn visit_ability(&mut self, value: &Ability) {
        self.0.push("Ability");
        deckmaste_english_v2::visit::walk_ability(self, value);
    }

    fn visit_triggered(&mut self, value: &Triggered) {
        self.0.push("Triggered");
        deckmaste_english_v2::visit::walk_triggered(self, value);
    }

    fn visit_trigger_prefix(&mut self, value: &TriggerPrefix) {
        self.0.push("TriggerPrefix");
        deckmaste_english_v2::visit::walk_trigger_prefix(self, value);
    }

    fn visit_finite(&mut self, value: &Finite) {
        self.0.push("Finite");
        deckmaste_english_v2::visit::walk_finite(self, value);
    }

    fn visit_temporal(&mut self, value: &Temporal) {
        self.0.push("Temporal");
        deckmaste_english_v2::visit::walk_temporal(self, value);
    }

    fn visit_at_phrase(&mut self, value: &AtPhrase) {
        self.0.push("AtPhrase");
        deckmaste_english_v2::visit::walk_at_phrase(self, value);
    }

    fn visit_at_phrase_value(&mut self, value: &AtPhraseValue) {
        self.0.push("AtPhraseValue");
        deckmaste_english_v2::visit::walk_at_phrase_value(self, value);
    }

    fn visit_trigger_marker(&mut self, _value: TriggerMarker) {
        self.0.push("TriggerMarker");
    }

    fn visit_turn_owner_postmodifier(&mut self, _value: TurnOwnerPostmodifier) {
        self.0.push("TurnOwnerPostmodifier");
    }

    fn visit_turn_part(&mut self, _value: TurnPart) {
        self.0.push("TurnPart");
    }

    fn visit_turn_specifier(&mut self, _value: TurnSpecifier) {
        self.0.push("TurnSpecifier");
    }

    fn visit_clause(&mut self, value: &Clause) {
        self.0.push("Clause");
        deckmaste_english_v2::visit::walk_clause(self, value);
    }

    fn visit_finite_clause(&mut self, value: &FiniteClause) {
        self.0.push("FiniteClause");
        deckmaste_english_v2::visit::walk_finite_clause(self, value);
    }

    fn visit_condition_clause(&mut self, value: &ConditionClause) {
        self.0.push("ConditionClause");
        deckmaste_english_v2::visit::walk_condition_clause(self, value);
    }

    fn visit_finite_condition(&mut self, value: &FiniteCondition) {
        self.0.push("FiniteCondition");
        deckmaste_english_v2::visit::walk_finite_condition(self, value);
    }

    fn visit_finite_condition_value(&mut self, value: &FiniteConditionValue) {
        self.0.push("FiniteConditionValue");
        deckmaste_english_v2::visit::walk_finite_condition_value(self, value);
    }

    fn visit_ability_body(&mut self, value: &AbilityBody) {
        self.0.push("AbilityBody");
        deckmaste_english_v2::visit::walk_ability_body(self, value);
    }

    fn visit_sentences(&mut self, value: &Sentences) {
        self.0.push("Sentences");
        deckmaste_english_v2::visit::walk_sentences(self, value);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CostVisit {
    Node(&'static str),
    Fixed(FixedCostSymbol),
    Generic(u32),
    LoyaltyMagnitude(NonZeroU32),
    MonocoloredHybrid(MonocoloredHybridColor),
}

#[derive(Default)]
struct CostVisitor(Vec<CostVisit>);

impl Visitor for CostVisitor {
    fn visit_ability(&mut self, value: &Ability) {
        self.0.push(CostVisit::Node("Ability"));
        deckmaste_english_v2::visit::walk_ability(self, value);
    }

    fn visit_activated(&mut self, value: &Activated) {
        self.0.push(CostVisit::Node("Activated"));
        deckmaste_english_v2::visit::walk_activated(self, value);
    }

    fn visit_activation_cost_component(&mut self, value: &ActivationCostComponent) {
        self.0.push(CostVisit::Node("ActivationCostComponent"));
        deckmaste_english_v2::visit::walk_activation_cost_component(self, value);
    }

    fn visit_symbol_run(&mut self, value: &SymbolRun) {
        self.0.push(CostVisit::Node("SymbolRun"));
        deckmaste_english_v2::visit::walk_symbol_run(self, value);
    }

    fn visit_cost_symbol(&mut self, value: &CostSymbol) {
        self.0.push(CostVisit::Node("CostSymbol"));
        deckmaste_english_v2::visit::walk_cost_symbol(self, value);
    }

    fn visit_generic_cost_symbol(&mut self, value: &GenericCostSymbol) {
        self.0.push(CostVisit::Node("GenericCostSymbol"));
        deckmaste_english_v2::visit::walk_generic_cost_symbol(self, value);
    }

    fn visit_fixed_symbol(&mut self, value: &FixedSymbol) {
        self.0.push(CostVisit::Node("FixedSymbol"));
        deckmaste_english_v2::visit::walk_fixed_symbol(self, value);
    }

    fn visit_fixed_cost_symbol(&mut self, value: FixedCostSymbol) {
        self.0.push(CostVisit::Fixed(value));
    }

    fn visit_monocolored_hybrid_symbol(&mut self, value: &MonocoloredHybridSymbol) {
        self.0.push(CostVisit::Node("MonocoloredHybridSymbol"));
        deckmaste_english_v2::visit::walk_monocolored_hybrid_symbol(self, value);
    }

    fn visit_monocolored_hybrid_color(&mut self, value: MonocoloredHybridColor) {
        self.0.push(CostVisit::MonocoloredHybrid(value));
    }

    fn visit_scalar_number(&mut self, value: &ScalarNumber) {
        self.0.push(CostVisit::Generic(value.magnitude));
    }

    fn visit_loyalty(&mut self, value: &Loyalty) {
        self.0.push(CostVisit::Node("Loyalty"));
        deckmaste_english_v2::visit::walk_loyalty(self, value);
    }

    fn visit_loyalty_value(&mut self, value: &LoyaltyValue) {
        self.0.push(CostVisit::Node("LoyaltyValue"));
        deckmaste_english_v2::visit::walk_loyalty_value(self, value);
    }

    fn visit_positive_loyalty(&mut self, value: &PositiveLoyalty) {
        self.0.push(CostVisit::Node("PositiveLoyalty"));
        deckmaste_english_v2::visit::walk_positive_loyalty(self, value);
    }

    fn visit_zero_loyalty(&mut self, value: &ZeroLoyalty) {
        self.0.push(CostVisit::Node("ZeroLoyalty"));
        deckmaste_english_v2::visit::walk_zero_loyalty(self, value);
    }

    fn visit_negative_loyalty(&mut self, value: &NegativeLoyalty) {
        self.0.push(CostVisit::Node("NegativeLoyalty"));
        deckmaste_english_v2::visit::walk_negative_loyalty(self, value);
    }

    fn visit_loyalty_magnitude(&mut self, value: &LoyaltyMagnitude) {
        self.0.push(CostVisit::LoyaltyMagnitude(value.magnitude));
    }

    fn visit_cost_clause(&mut self, value: &CostClause) {
        self.0.push(CostVisit::Node("CostClause"));
        deckmaste_english_v2::visit::walk_cost_clause(self, value);
    }

    fn visit_ability_body(&mut self, value: &AbilityBody) {
        self.0.push(CostVisit::Node("AbilityBody"));
        deckmaste_english_v2::visit::walk_ability_body(self, value);
    }

    fn visit_sentences(&mut self, value: &Sentences) {
        self.0.push(CostVisit::Node("Sentences"));
        deckmaste_english_v2::visit::walk_sentences(self, value);
    }
}

#[test]
fn finite_trigger_boundaries_preserve_case_ownership_and_structural_visit_order() {
    let parser = parser();
    let context = context("Context Card", false);
    let text = "Whenever a player connives, you gain X life. Destroy target creature.";
    let analysis = parser.analyze(text, &context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("well-cased trigger must parse: {analysis:?}"));
    assert_eq!(selected.render(&context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("selected triggered ability owns every byte");
    assert!(ownership.failures().is_empty(), "{ownership:?}");
    assert!(ownership.summary().covered(), "{ownership:?}");
    assert_eq!(
        ownership
            .parsed_claims()
            .iter()
            .map(|claim| (
                claim.span().start,
                claim.span().end,
                claim.stable_owner_id()
            ))
            .collect::<Vec<_>>(),
        [
            (0, 8, "vocab:TriggerMarker/Whenever"),
            (8, 10, "form:indefinite_reference/a/0"),
            (10, 17, "lexeme:CommonNoun/Player/singular"),
            (
                17,
                26,
                "lexeme:keyword_action/Connive/third_person_singular"
            ),
            (26, 27, "form:triggered/triggered/1"),
            (27, 31, "vocab:SubjectPronoun/You"),
            (31, 36, "lexeme:VerbLexeme/Gain/bare"),
            (36, 38, "vocab:Variable/X"),
            (38, 43, "form:gain_life/gain_life/2"),
            (43, 44, "structural:Sentences/sentences/terminator/0"),
            (44, 45, "structural:Sentences/sentences/separator/uniform/0"),
            (45, 52, "lexeme:keyword_action/Destroy/bare"),
            (
                52,
                59,
                "form:target_singular_selector/target_singular_selector/0"
            ),
            (59, 68, "lexeme:type/Creature/singular"),
            (68, 69, "structural:Sentences/sentences/terminator/0"),
        ]
    );

    let mut visitor = AbilityEnvelopeVisitor::default();
    visitor.visit_ability(selected);
    assert_eq!(
        visitor.0,
        [
            "Ability",
            "Triggered",
            "TriggerPrefix",
            "Finite",
            "TriggerMarker",
            "Clause",
            "FiniteClause",
            "AbilityBody",
            "Sentences",
        ]
    );

    for invalid in [
        "whenever a player connives, you gain X life. Destroy target creature.",
        "Whenever A player connives, you gain X life. Destroy target creature.",
        "Whenever a player connives,you gain X life. Destroy target creature.",
        "Whenever a player connives, You gain X life. Destroy target creature.",
        "Whenever a player connives, you gain X life. destroy target creature.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "case/boundary mutation must be rejected: {invalid}"
        );
    }
}

#[test]
fn finite_trigger_predicate_agreement_is_derived_from_its_subject() {
    let parser = parser();
    let context = context("Context Card", false);
    let text = "Whenever you connive, you gain X life.";
    let parsed = parser
        .parse(text, &context)
        .expect("second-person trigger subject requires bare predicate agreement");
    assert_eq!(parsed.render(&context, parser.environment()), text);
    assert!(
        parser
            .parse("Whenever you connives, you gain X life.", &context)
            .is_err(),
        "third-person-singular agreement must reject a second-person trigger subject"
    );
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "the exact generated trigger and temporal inventories are deliberately literal"
)]
fn generated_trigger_and_condition_inventories_exclude_surface_tags_and_event_shapes() {
    let source = include_str!("../src/constructions.rs");
    let invocation = deckmaste_construction_core::invocation_from_source(source)
        .expect("production construction invocation is authentic");
    let expansion = deckmaste_construction_core::generate(invocation.tokens)
        .expect("production construction invocation expands");
    let file = syn::parse2::<syn::File>(expansion.tokens()).expect("generated Rust parses");
    let variants = |name| {
        file.items
            .iter()
            .find_map(|item| match item {
                syn::Item::Enum(item) if item.ident == name => Some(
                    item.variants
                        .iter()
                        .map(|variant| variant.ident.to_string())
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            })
            .unwrap_or_else(|| panic!("generated public enum {name} is present"))
    };
    assert_eq!(variants("Clause"), ["FiniteClause", "Where"]);
    assert_eq!(variants("TriggerMarker"), ["When", "Whenever"]);
    assert_eq!(variants("TriggerPrefix"), ["Finite", "Temporal"]);
    assert_eq!(variants("AtPhrase"), ["AtPhrase"]);
    assert_eq!(variants("ConditionClause"), ["FiniteCondition"]);
    assert_eq!(variants("FiniteCondition"), ["FiniteCondition"]);
    assert_eq!(variants("AtBoundary"), ["Beginning", "End"]);
    assert_eq!(
        variants("TurnPart"),
        [
            "Turn",
            "BeginningPhase",
            "FirstMainPhase",
            "SecondMainPhase",
            "PrecombatMainPhase",
            "PostcombatMainPhase",
            "MainPhase",
            "Combat",
            "CombatPhase",
            "EndingPhase",
            "UntapStep",
            "Upkeep",
            "DrawStep",
            "DeclareAttackersStep",
            "DeclareBlockersStep",
            "CombatDamageStep",
            "EndStep",
            "CleanupStep",
        ]
    );
    assert_eq!(
        variants("TurnSpecifier"),
        [
            "Your",
            "Each",
            "EachPlayer",
            "EachOpponent",
            "EachOfYour",
            "The",
            "TheNext",
        ]
    );
    assert_eq!(
        variants("TurnOwnerPostmodifier"),
        ["YourTurn", "EachOpponentsTurn"]
    );
    assert!(
        !file
            .items
            .iter()
            .any(|item| matches!(item, syn::Item::Enum(item) if item.ident == "TriggerWord")),
        "the retired bootstrap trigger-word type is absent"
    );

    let value = Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(
            Finite::new(
                TriggerMarker::Whenever,
                Clause::FiniteClause(FiniteClause {
                    subject: Subject::SubjectPronoun(PersonalSubject {
                        word: SubjectPronoun::You,
                    }),
                    predicate: VerbPhrase::Connive(Connive),
                }),
            )
            .expect("finite trigger accepts a finite clause"),
        ),
        intervening_if: None,
        body: AbilityBody::Sentences(
            Sentences::new(vec![Sentence::Imperative(Imperative {
                predicate: VerbPhrase::Connive(Connive),
            })])
            .expect("linguistic body remains nonempty"),
        ),
    });
    let Ability::Triggered(triggered) = value else {
        unreachable!("literal constructs the triggered variant")
    };
    let _: Option<ConditionClause> = triggered.intervening_if;

    assert!(
        AtPhraseValue::new(
            AtBoundary::Beginning,
            Some(TurnSpecifier::EachOfYour),
            TurnPart::MainPhase,
            None,
        )
        .is_some(),
        "the checked public product admits a plural main-phase combination",
    );
    for (family, rejected) in [
        (
            "End requires combat",
            AtPhraseValue::new(AtBoundary::End, None, TurnPart::Turn, None),
        ),
        (
            "End forbids a specifier",
            AtPhraseValue::new(
                AtBoundary::End,
                Some(TurnSpecifier::Your),
                TurnPart::Combat,
                None,
            ),
        ),
        (
            "End forbids a postmodifier",
            AtPhraseValue::new(
                AtBoundary::End,
                None,
                TurnPart::Combat,
                Some(TurnOwnerPostmodifier::YourTurn),
            ),
        ),
        (
            "EachOfYour requires a pluralizable main phase",
            AtPhraseValue::new(
                AtBoundary::Beginning,
                Some(TurnSpecifier::EachOfYour),
                TurnPart::Upkeep,
                None,
            ),
        ),
        (
            "EachOfYour does not enter the combat-postmodifier family",
            AtPhraseValue::new(
                AtBoundary::Beginning,
                Some(TurnSpecifier::EachOfYour),
                TurnPart::MainPhase,
                Some(TurnOwnerPostmodifier::YourTurn),
            ),
        ),
        (
            "an absent specifier permits a postmodifier only for combat",
            AtPhraseValue::new(
                AtBoundary::Beginning,
                None,
                TurnPart::DrawStep,
                Some(TurnOwnerPostmodifier::YourTurn),
            ),
        ),
        (
            "an ordinary specifier permits a postmodifier only for combat",
            AtPhraseValue::new(
                AtBoundary::Beginning,
                Some(TurnSpecifier::Your),
                TurnPart::Upkeep,
                Some(TurnOwnerPostmodifier::YourTurn),
            ),
        ),
    ] {
        assert!(
            rejected.is_none(),
            "the checked public product rejects the forbidden `{family}` cross-product",
        );
    }
}

fn assert_selected_trigger(parser: &Parser, context: &ParseContext<'_>, text: &str) -> Ability {
    let analysis = parser.analyze(text, context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("closed trigger surface must select: {text}: {analysis:?}"));
    assert_eq!(
        analysis
            .decision()
            .expect("a selected trigger has a selection decision")
            .candidates()
            .len(),
        1,
        "closed trigger surface has one semantic candidate: {text}"
    );
    assert_eq!(selected.render(context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("a selected trigger has byte ownership");
    assert!(ownership.failures().is_empty(), "{text}: {ownership:?}");
    assert!(ownership.summary().covered(), "{text}: {ownership:?}");
    selected.clone()
}

#[test]
fn finite_temporal_and_intervening_trigger_prefixes_have_dedicated_generated_shapes() {
    let parser = parser();
    let context = context("Context Card", false);

    for (text, marker) in [
        (
            "When a player connives, you gain X life.",
            TriggerMarker::When,
        ),
        (
            "Whenever a player connives, you gain X life.",
            TriggerMarker::Whenever,
        ),
    ] {
        let parsed = assert_selected_trigger(&parser, &context, text);
        let Ability::Triggered(Triggered {
            trigger: TriggerPrefix::Finite(finite),
            intervening_if: None,
            ..
        }) = parsed
        else {
            panic!("finite trigger uses the finite generated prefix: {text}")
        };
        assert_eq!(finite.marker, marker);
        assert!(matches!(finite.clause(), Clause::FiniteClause(_)));
    }

    let temporal_text = "At the beginning of each player's draw step, you gain X life.";
    let temporal = assert_selected_trigger(&parser, &context, temporal_text);
    let Ability::Triggered(Triggered {
        trigger:
            TriggerPrefix::Temporal(Temporal {
                phrase: AtPhrase::AtPhrase(phrase),
            }),
        intervening_if: None,
        ..
    }) = temporal
    else {
        panic!("At takes the dedicated temporal phrase")
    };
    assert_eq!(phrase.boundary(), AtBoundary::Beginning);
    assert_eq!(phrase.specifier(), Some(&TurnSpecifier::EachPlayer));
    assert_eq!(phrase.part(), TurnPart::DrawStep);
    assert_eq!(phrase.postmodifier(), None);

    let intervening_text = "Whenever a player connives, if you connive, you gain X life.";
    let intervening = assert_selected_trigger(&parser, &context, intervening_text);
    let Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(_),
        intervening_if:
            Some(ConditionClause::FiniteCondition(FiniteCondition::FiniteCondition(
                FiniteConditionValue {
                    subject:
                        Subject::SubjectPronoun(PersonalSubject {
                            word: SubjectPronoun::You,
                        }),
                    predicate: VerbPhrase::Connive(Connive),
                },
            ))),
        ..
    }) = intervening
    else {
        panic!("the immediate if clause has its dedicated finite-condition attachment")
    };

    assert!(
        parser
            .parse(
                "Whenever a player connives, you gain X life if you connive.",
                &context,
            )
            .is_err(),
        "ordinary postposed if syntax cannot materialize as intervening-if"
    );
}

#[test]
fn at_phrase_accepts_every_closed_vocabulary_member_and_restricted_cross_product() {
    let parser = parser();
    let context = context("Context Card", false);

    for text in [
        "At the beginning of turn, you gain X life.",
        "At the beginning of beginning phase, you gain X life.",
        "At the beginning of first main phase, you gain X life.",
        "At the beginning of second main phase, you gain X life.",
        "At the beginning of precombat main phase, you gain X life.",
        "At the beginning of postcombat main phase, you gain X life.",
        "At the beginning of main phase, you gain X life.",
        "At the beginning of combat, you gain X life.",
        "At the beginning of combat phase, you gain X life.",
        "At the beginning of ending phase, you gain X life.",
        "At the beginning of untap step, you gain X life.",
        "At the beginning of upkeep, you gain X life.",
        "At the beginning of draw step, you gain X life.",
        "At the beginning of declare attackers step, you gain X life.",
        "At the beginning of declare blockers step, you gain X life.",
        "At the beginning of combat damage step, you gain X life.",
        "At the beginning of end step, you gain X life.",
        "At the beginning of cleanup step, you gain X life.",
        "At the beginning of your upkeep, you gain X life.",
        "At the beginning of each upkeep, you gain X life.",
        "At the beginning of each player's upkeep, you gain X life.",
        "At the beginning of each opponent's upkeep, you gain X life.",
        "At the beginning of the upkeep, you gain X life.",
        "At the beginning of the next upkeep, you gain X life.",
        "At the beginning of each of your first main phases, you gain X life.",
        "At the beginning of each of your second main phases, you gain X life.",
        "At the beginning of each of your precombat main phases, you gain X life.",
        "At the beginning of each of your postcombat main phases, you gain X life.",
        "At the beginning of each of your main phases, you gain X life.",
        "At the beginning of combat on your turn, you gain X life.",
        "At the beginning of combat on each opponent's turn, you gain X life.",
        "At the beginning of your combat on your turn, you gain X life.",
        "At the beginning of your combat on each opponent's turn, you gain X life.",
        "At the beginning of each combat on your turn, you gain X life.",
        "At the beginning of each combat on each opponent's turn, you gain X life.",
        "At the beginning of each player's combat on your turn, you gain X life.",
        "At the beginning of each player's combat on each opponent's turn, you gain X life.",
        "At the beginning of each opponent's combat on your turn, you gain X life.",
        "At the beginning of each opponent's combat on each opponent's turn, you gain X life.",
        "At the beginning of the combat on your turn, you gain X life.",
        "At the beginning of the combat on each opponent's turn, you gain X life.",
        "At the beginning of the next combat on your turn, you gain X life.",
        "At the beginning of the next combat on each opponent's turn, you gain X life.",
        "At end of combat, you gain X life.",
    ] {
        assert_selected_trigger(&parser, &context, text);
    }

    for invalid in [
        "At your upkeep, you gain X life.",
        "At end of turn, you gain X life.",
        "At the end of combat, you gain X life.",
        "At end of your combat, you gain X life.",
        "At the beginning of an opponent's upkeep, you gain X life.",
        "At the beginning of luncheon, you gain X life.",
        "At the beginning of each of your upkeep, you gain X life.",
        "At the beginning of each of your upkeeps, you gain X life.",
        "At the beginning of upkeep on your turn, you gain X life.",
        "At the beginning of draw step on each opponent's turn, you gain X life.",
        "At the beginning of your upkeep on your turn, you gain X life.",
        "At end of combat on your turn, you gain X life.",
        "At the beginning of each of your combat on your turn, you gain X life.",
        "At the beginning of each of your combats on your turn, you gain X life.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "forbidden At-phrase cross-product must not parse: {invalid}"
        );
    }
}

#[test]
fn each_of_your_main_phase_requires_plural_morphology() {
    let parser = parser();
    let context = context("Context Card", false);
    let singular = "At the beginning of each of your main phase, you gain X life.";

    assert!(
        parser.parse(singular, &context).is_err(),
        "EachOfYour must not admit the singular main-phase realization",
    );
}

#[test]
fn temporal_and_intervening_boundaries_own_exact_bytes_and_visit_structure() {
    let parser = parser();
    let context = context("Context Card", false);
    let temporal_text = "At the beginning of each player's draw step, you gain X life.";
    let analysis = parser.analyze(temporal_text, &context);
    let temporal = analysis
        .selected()
        .unwrap_or_else(|| panic!("temporal witness must parse: {analysis:?}"));
    assert_eq!(
        analysis
            .ownership()
            .expect("temporal witness has ownership")
            .parsed_claims()
            .iter()
            .map(|claim| (
                claim.span().start,
                claim.span().end,
                claim.stable_owner_id()
            ))
            .collect::<Vec<_>>(),
        [
            (0, 2, "form:temporal/temporal/0"),
            (2, 19, "vocab:AtBoundary/Beginning"),
            (19, 33, "vocab:TurnSpecifier/EachPlayer"),
            (33, 43, "vocab:TurnPart/DrawStep"),
            (43, 44, "form:triggered/triggered/1"),
            (44, 48, "vocab:SubjectPronoun/You"),
            (48, 53, "lexeme:VerbLexeme/Gain/bare"),
            (53, 55, "vocab:Variable/X"),
            (55, 60, "form:gain_life/gain_life/2"),
            (60, 61, "structural:Sentences/sentences/terminator/0"),
        ]
    );

    let mut visitor = AbilityEnvelopeVisitor::default();
    visitor.visit_ability(temporal);
    assert_eq!(
        visitor.0,
        [
            "Ability",
            "Triggered",
            "TriggerPrefix",
            "Temporal",
            "AtPhrase",
            "AtPhraseValue",
            "AtBoundary",
            "TurnSpecifier",
            "TurnPart",
            "AbilityBody",
            "Sentences",
        ]
    );

    let intervening_text = "Whenever a player connives, if you connive, you gain X life.";
    let analysis = parser.analyze(intervening_text, &context);
    let intervening = analysis
        .selected()
        .unwrap_or_else(|| panic!("intervening-if witness must parse: {analysis:?}"));
    let ownership = analysis
        .ownership()
        .expect("intervening-if witness has ownership");
    assert!(ownership.failures().is_empty(), "{ownership:?}");
    assert!(ownership.summary().covered(), "{ownership:?}");
    assert_eq!(
        ownership
            .parsed_claims()
            .iter()
            .map(|claim| (
                claim.span().start,
                claim.span().end,
                claim.stable_owner_id()
            ))
            .collect::<Vec<_>>(),
        [
            (0, 8, "vocab:TriggerMarker/Whenever"),
            (8, 10, "form:indefinite_reference/a/0"),
            (10, 17, "lexeme:CommonNoun/Player/singular"),
            (
                17,
                26,
                "lexeme:keyword_action/Connive/third_person_singular"
            ),
            (26, 27, "form:triggered/triggered/1"),
            (27, 30, "form:finite_condition/finite_condition/0"),
            (30, 34, "vocab:SubjectPronoun/You"),
            (34, 42, "lexeme:keyword_action/Connive/bare"),
            (42, 43, "form:finite_condition/finite_condition/3"),
            (43, 47, "vocab:SubjectPronoun/You"),
            (47, 52, "lexeme:VerbLexeme/Gain/bare"),
            (52, 54, "vocab:Variable/X"),
            (54, 59, "form:gain_life/gain_life/2"),
            (59, 60, "structural:Sentences/sentences/terminator/0"),
        ]
    );

    let mut visitor = AbilityEnvelopeVisitor::default();
    visitor.visit_ability(intervening);
    assert_eq!(
        visitor.0,
        [
            "Ability",
            "Triggered",
            "TriggerPrefix",
            "Finite",
            "TriggerMarker",
            "Clause",
            "FiniteClause",
            "ConditionClause",
            "FiniteCondition",
            "FiniteConditionValue",
            "AbilityBody",
            "Sentences",
        ]
    );

    let combat = assert_selected_trigger(
        &parser,
        &context,
        "At the beginning of combat on each opponent's turn, you gain X life.",
    );
    let mut visitor = AbilityEnvelopeVisitor::default();
    visitor.visit_ability(&combat);
    assert_eq!(
        visitor.0,
        [
            "Ability",
            "Triggered",
            "TriggerPrefix",
            "Temporal",
            "AtPhrase",
            "AtPhraseValue",
            "AtBoundary",
            "TurnPart",
            "TurnOwnerPostmodifier",
            "AbilityBody",
            "Sentences",
        ]
    );

    for invalid in [
        "at the beginning of each player's draw step, you gain X life.",
        "At The beginning of each player's draw step, you gain X life.",
        "At the beginning of each player's draw step,you gain X life.",
        "At the beginning of each player's draw step, You gain X life.",
        "Whenever a player connives, If you connive, you gain X life.",
        "Whenever a player connives, if you connive,you gain X life.",
        "Whenever a player connives, if you connive you gain X life.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "temporal/intervening case or boundary mutation must reject: {invalid}",
        );
    }
}

#[test]
fn self_reference_context_is_reused_in_finite_temporal_and_intervening_triggers() {
    let parser = parser();
    for (name, legendary, valid_spelling) in [
        (
            "Aang, A Lot to Learn",
            true,
            SelfReferenceSpelling::Abbreviated,
        ),
        ("Grizzly Bears", false, SelfReferenceSpelling::Full),
    ] {
        let context = context(name, legendary);
        let selected_surfaces = if legendary {
            [
                "Whenever Aang gains 2 life, you gain X life.",
                "At the beginning of your upkeep, Aang gains 2 life.",
                "Whenever a player connives, if Aang gains 2 life, you gain X life.",
            ]
        } else {
            [
                "Whenever Grizzly Bears gains 2 life, you gain X life.",
                "At the beginning of your upkeep, Grizzly Bears gains 2 life.",
                "Whenever a player connives, if Grizzly Bears gains 2 life, you gain X life.",
            ]
        };
        for text in selected_surfaces {
            let parsed = assert_selected_trigger(&parser, &context, text);
            let mut visitor = SelfReferenceVisitor::default();
            visitor.visit_ability(&parsed);
            assert_eq!(visitor.spellings, [valid_spelling], "{text}");
        }

        assert_eq!(
            SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context).is_some(),
            legendary,
            "only authoritative Legendary metadata licenses the short arm: {name}"
        );
        if !legendary {
            for invalid in [
                "Whenever Grizzly gains 2 life, you gain X life.",
                "At the beginning of your upkeep, Grizzly gains 2 life.",
                "Whenever a player connives, if Grizzly gains 2 life, you gain X life.",
            ] {
                assert!(
                    parser.parse(invalid, &context).is_err(),
                    "the ordinary-name shortcut is rejected in every trigger family: {invalid}"
                );
            }
        }
    }
}

fn assert_public_self_reference_surface(
    parser: &Parser,
    context: &ParseContext<'_>,
    surface: &str,
    spelling: SelfReferenceSpelling,
) {
    let text = format!("{surface} gains 2 life.");
    let analysis = parser.analyze(&text, context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("the context identity must scan and materialize: {analysis:?}"));
    let decision = analysis
        .decision()
        .expect("selected ability has a decision");
    assert_eq!(
        decision.candidates().len(),
        1,
        "self-reference reading is canonical"
    );
    assert_eq!(selected.render(context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("selected context identity has byte ownership");
    assert!(ownership.failures().is_empty(), "{surface}: {ownership:?}");
    assert!(ownership.summary().covered(), "{surface}: {ownership:?}");
    assert_eq!(
        ownership.rendered_text(),
        surface.to_owned() + " gains 2 life."
    );

    let mut visitor = SelfReferenceVisitor::default();
    visitor.visit_ability(selected);
    assert_eq!(visitor.spellings, [spelling], "{surface}");
}

#[test]
fn authoritative_legendary_contexts_license_only_their_distinct_self_reference_arm() {
    let parser = parser();

    for (name, abbreviated, onset) in [
        ("Aang, A Lot to Learn", "Aang", Onset::Vowel),
        ("The Balrog, Durin's Bane", "The Balrog", Onset::Consonant),
        ("King Darien XLVIII", "King Darien", Onset::Consonant),
        ("Sidar Jabari of Zhalfir", "Sidar Jabari", Onset::Consonant),
        ("Tor Wauki the Younger", "Tor Wauki", Onset::Consonant),
        ("Sliver Queen", "Sliver", Onset::Consonant),
    ] {
        let context = ParseContext::new(name, true, onset)
            .expect("authoritative nonempty legendary face context is valid");
        assert_eq!(context.abbreviated_card_name(), abbreviated, "{name}");
        let full = SourceSelfReference::new(SelfReferenceSpelling::Full, &context)
            .expect("the full generated context arm is constructible");
        let shortened = SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context)
            .expect("a distinct legendary abbreviation is constructible");
        assert_eq!(full.spelling(), SelfReferenceSpelling::Full, "{name}");
        assert_eq!(
            shortened.spelling(),
            SelfReferenceSpelling::Abbreviated,
            "{name}"
        );
        assert_public_self_reference_surface(&parser, &context, name, SelfReferenceSpelling::Full);
        assert_public_self_reference_surface(
            &parser,
            &context,
            abbreviated,
            SelfReferenceSpelling::Abbreviated,
        );

        let triggered = parser
            .parse(
                &format!("Whenever {abbreviated} gains 2 life, you gain X life."),
                &context,
            )
            .expect("a licensed abbreviation parses through the finite triggered family");
        assert_eq!(
            triggered.render(&context, parser.environment()),
            format!("Whenever {abbreviated} gains 2 life, you gain X life.")
        );

        let mut visitor = SelfReferenceVisitor::default();
        visitor.visit_ability(&triggered);
        assert_eq!(
            visitor.spellings,
            [SelfReferenceSpelling::Abbreviated],
            "{name}"
        );
    }
}

#[test]
fn ordinary_and_nonshortening_legendary_contexts_reject_an_abbreviated_constructor_arm() {
    let parser = parser();
    for (name, legendary, onset) in [
        ("Grizzly Bears", false, Onset::Consonant),
        ("+2 Mace", false, Onset::Consonant),
        ("Fear, Fire, Foes!", false, Onset::Consonant),
        ("The First Sliver", true, Onset::Consonant),
        ("Progenitus", true, Onset::Consonant),
    ] {
        let context = ParseContext::new(name, legendary, onset)
            .expect("authoritative nonempty face context is valid");
        assert_eq!(context.abbreviated_card_name(), name, "{name}");
        let full = SourceSelfReference::new(SelfReferenceSpelling::Full, &context)
            .expect("the canonical full generated arm is constructible");
        assert!(
            SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context).is_none(),
            "equal context spellings reject the noncanonical abbreviated constructor: {name}"
        );

        assert_eq!(full.spelling(), SelfReferenceSpelling::Full, "{name}");
        assert_public_self_reference_surface(&parser, &context, name, SelfReferenceSpelling::Full);
        let triggered = parser
            .parse(
                &format!("Whenever {name} gains 2 life, you gain X life."),
                &context,
            )
            .expect("the full-only self reference parses through the finite triggered family");
        assert_eq!(
            triggered.render(&context, parser.environment()),
            format!("Whenever {name} gains 2 life, you gain X life.")
        );
    }
}

fn assert_selected_activated(parser: &Parser, context: &ParseContext<'_>, text: &str) -> Ability {
    let analysis = parser.analyze(text, context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("closed activated surface must select: {text}: {analysis:?}"));
    assert_eq!(
        analysis
            .decision()
            .expect("a selected activation has a selection decision")
            .candidates()
            .len(),
        1,
        "closed activated surface has one semantic candidate: {text}",
    );
    assert_eq!(selected.render(context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("a selected activation has byte ownership");
    assert!(ownership.failures().is_empty(), "{text}: {ownership:?}");
    assert!(ownership.summary().covered(), "{text}: {ownership:?}");
    selected.clone()
}

#[test]
fn generated_activation_inventory_is_closed_typed_and_surface_free() {
    let source = include_str!("../src/constructions.rs");
    let invocation = deckmaste_construction_core::invocation_from_source(source)
        .expect("production construction invocation is authentic");
    let expansion = deckmaste_construction_core::generate(invocation.tokens)
        .expect("production construction invocation expands");
    let file = syn::parse2::<syn::File>(expansion.tokens()).expect("generated Rust parses");
    let variants = |name| {
        file.items
            .iter()
            .find_map(|item| match item {
                syn::Item::Enum(item) if item.ident == name => Some(
                    item.variants
                        .iter()
                        .map(|variant| variant.ident.to_string())
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            })
            .unwrap_or_else(|| panic!("generated public enum {name} is present"))
    };

    assert_eq!(variants("Ability"), ["Plain", "Triggered", "Activated"]);
    assert_eq!(
        variants("ActivationCostComponent"),
        ["SymbolRun", "Loyalty", "Clause"],
    );
    assert_eq!(
        variants("CostSymbol"),
        [
            "GenericCostSymbol",
            "FixedCostSymbol",
            "MonocoloredHybridSymbol",
        ],
    );
    assert_eq!(
        variants("LoyaltyValue"),
        ["PositiveLoyalty", "ZeroLoyalty", "NegativeLoyalty"],
    );
    assert_eq!(
        variants("FixedCostSymbol"),
        [
            "Variable",
            "White",
            "Blue",
            "Black",
            "Red",
            "Green",
            "Colorless",
            "Snow",
            "HybridWhiteBlue",
            "HybridWhiteBlack",
            "HybridBlueBlack",
            "HybridBlueRed",
            "HybridBlackRed",
            "HybridBlackGreen",
            "HybridRedGreen",
            "HybridRedWhite",
            "HybridGreenWhite",
            "HybridGreenBlue",
            "ColorlessHybridWhite",
            "ColorlessHybridBlue",
            "ColorlessHybridBlack",
            "ColorlessHybridRed",
            "ColorlessHybridGreen",
            "PhyrexianWhite",
            "PhyrexianBlue",
            "PhyrexianBlack",
            "PhyrexianRed",
            "PhyrexianGreen",
            "HybridPhyrexianWhiteBlue",
            "HybridPhyrexianWhiteBlack",
            "HybridPhyrexianBlueBlack",
            "HybridPhyrexianBlueRed",
            "HybridPhyrexianBlackRed",
            "HybridPhyrexianBlackGreen",
            "HybridPhyrexianRedGreen",
            "HybridPhyrexianRedWhite",
            "HybridPhyrexianGreenWhite",
            "HybridPhyrexianGreenBlue",
            "Tap",
            "Untap",
        ],
    );
    assert_eq!(
        variants("MonocoloredHybridColor"),
        ["White", "Blue", "Black", "Red", "Green"],
    );

    for forbidden in [
        "ManaCost",
        "EngineCost",
        "Payment",
        "CostOpcode",
        "RawCost",
        "CostText",
        "CostForm",
        "ActivatedForm",
    ] {
        assert!(
            !file.items.iter().any(|item| {
                matches!(item, syn::Item::Enum(item) if item.ident == forbidden)
                    || matches!(item, syn::Item::Struct(item) if item.ident == forbidden)
            }),
            "the generated public AST has no engine, raw-text, or surface-tag type `{forbidden}`",
        );
    }
}

#[test]
fn every_fixed_symbol_interior_and_generic_decimal_has_one_typed_ast() {
    let parser = parser();
    let context = context("Context Card", false);
    let fixed = [
        ("X", FixedCostSymbol::Variable),
        ("W", FixedCostSymbol::White),
        ("U", FixedCostSymbol::Blue),
        ("B", FixedCostSymbol::Black),
        ("R", FixedCostSymbol::Red),
        ("G", FixedCostSymbol::Green),
        ("C", FixedCostSymbol::Colorless),
        ("S", FixedCostSymbol::Snow),
        ("W/U", FixedCostSymbol::HybridWhiteBlue),
        ("W/B", FixedCostSymbol::HybridWhiteBlack),
        ("U/B", FixedCostSymbol::HybridBlueBlack),
        ("U/R", FixedCostSymbol::HybridBlueRed),
        ("B/R", FixedCostSymbol::HybridBlackRed),
        ("B/G", FixedCostSymbol::HybridBlackGreen),
        ("R/G", FixedCostSymbol::HybridRedGreen),
        ("R/W", FixedCostSymbol::HybridRedWhite),
        ("G/W", FixedCostSymbol::HybridGreenWhite),
        ("G/U", FixedCostSymbol::HybridGreenBlue),
        ("C/W", FixedCostSymbol::ColorlessHybridWhite),
        ("C/U", FixedCostSymbol::ColorlessHybridBlue),
        ("C/B", FixedCostSymbol::ColorlessHybridBlack),
        ("C/R", FixedCostSymbol::ColorlessHybridRed),
        ("C/G", FixedCostSymbol::ColorlessHybridGreen),
        ("W/P", FixedCostSymbol::PhyrexianWhite),
        ("U/P", FixedCostSymbol::PhyrexianBlue),
        ("B/P", FixedCostSymbol::PhyrexianBlack),
        ("R/P", FixedCostSymbol::PhyrexianRed),
        ("G/P", FixedCostSymbol::PhyrexianGreen),
        ("W/U/P", FixedCostSymbol::HybridPhyrexianWhiteBlue),
        ("W/B/P", FixedCostSymbol::HybridPhyrexianWhiteBlack),
        ("U/B/P", FixedCostSymbol::HybridPhyrexianBlueBlack),
        ("U/R/P", FixedCostSymbol::HybridPhyrexianBlueRed),
        ("B/R/P", FixedCostSymbol::HybridPhyrexianBlackRed),
        ("B/G/P", FixedCostSymbol::HybridPhyrexianBlackGreen),
        ("R/G/P", FixedCostSymbol::HybridPhyrexianRedGreen),
        ("R/W/P", FixedCostSymbol::HybridPhyrexianRedWhite),
        ("G/W/P", FixedCostSymbol::HybridPhyrexianGreenWhite),
        ("G/U/P", FixedCostSymbol::HybridPhyrexianGreenBlue),
        ("T", FixedCostSymbol::Tap),
        ("Q", FixedCostSymbol::Untap),
    ];

    for (interior, expected) in fixed {
        let text = format!("{{{interior}}}: You gain X life.");
        let Ability::Activated(activated) = assert_selected_activated(&parser, &context, &text)
        else {
            panic!("a cost-colon surface has the activated envelope: {text}")
        };
        let [ActivationCostComponent::SymbolRun(run)] = activated.costs() else {
            panic!("a single braced token is one symbol-run component: {text}")
        };
        assert_eq!(
            run.symbols(),
            &[CostSymbol::FixedCostSymbol(FixedSymbol {
                symbol: expected
            })],
            "fixed symbol identity is typed rather than slash text: {text}",
        );
    }

    for (interior, expected) in [
        ("2/W", MonocoloredHybridColor::White),
        ("2/U", MonocoloredHybridColor::Blue),
        ("2/B", MonocoloredHybridColor::Black),
        ("2/R", MonocoloredHybridColor::Red),
        ("2/G", MonocoloredHybridColor::Green),
    ] {
        let text = format!("{{{interior}}}: You gain X life.");
        let Ability::Activated(activated) = assert_selected_activated(&parser, &context, &text)
        else {
            panic!("a monocolored hybrid symbol has the activated envelope: {text}")
        };
        let [ActivationCostComponent::SymbolRun(run)] = activated.costs() else {
            panic!("one monocolored hybrid symbol is one symbol-run component: {text}")
        };
        assert_eq!(
            run.symbols(),
            &[CostSymbol::MonocoloredHybridSymbol(
                MonocoloredHybridSymbol { color: expected },
            )],
            "the monocolored-hybrid color is typed and slash text is derived: {text}",
        );
    }

    for (surface, magnitude) in [
        ("0", 0),
        ("2", 2),
        ("1,000", 1_000),
        ("4,294,967,295", u32::MAX),
    ] {
        let text = format!("{{{surface}}}: You gain X life.");
        let Ability::Activated(activated) = assert_selected_activated(&parser, &context, &text)
        else {
            panic!("a generic symbol has the activated envelope: {text}")
        };
        let [ActivationCostComponent::SymbolRun(run)] = activated.costs() else {
            panic!("one generic symbol is one symbol-run component: {text}")
        };
        assert_eq!(
            run.symbols(),
            &[CostSymbol::GenericCostSymbol(GenericCostSymbol {
                magnitude: ScalarNumber { magnitude },
            })],
            "generic magnitude remains a typed canonical unsigned value: {text}",
        );
    }
}

#[test]
fn symbol_interior_and_braced_run_rejection_set_is_closed() {
    let parser = parser();
    let context = context("Context Card", false);
    for interior in [
        "", "00", "01", "+2", "−2", "P", "H", "E", "TK", "∞", "Y", "Z", "U/W", "B/W", "B/U", "R/U",
        "R/B", "G/B", "G/R", "W/R", "W/G", "U/G", "W/2", "U/2", "B/2", "R/2", "G/2", "W/C", "U/C",
        "B/C", "R/C", "G/C", "P/W", "P/U", "P/B", "P/R", "P/G", "U/W/P", "B/W/P", "B/U/P", "R/U/P",
        "R/B/P", "G/B/P", "G/R/P", "W/R/P", "W/G/P", "U/G/P", "W/P/U", "W/U/B", "W/W", "2/P",
        "C/P", " W", "W ", "W /U", "W/ U",
    ] {
        let text = format!("{{{interior}}}: You gain X life.");
        assert!(
            parser.parse(&text, &context).is_err(),
            "unlicensed symbol interior must reject: {text}",
        );
    }

    for text in [
        "{W} {U}: You gain X life.",
        "{W},{U}: You gain X life.",
        "{W}{}{U}: You gain X life.",
        "{{W}}: You gain X life.",
        "{W: You gain X life.",
        "W}: You gain X life.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "a symbol run requires one or more exactly adjacent braced members: {text}",
        );
    }
}

#[test]
fn loyalty_values_are_bracketed_typed_and_nonzero_away_from_zero() {
    let parser = parser();
    let context = context("Context Card", false);
    let one = NonZeroU32::new(1).expect("one is nonzero");
    let two = NonZeroU32::new(2).expect("two is nonzero");

    for (text, expected) in [
        (
            "[+1]: You gain X life.",
            LoyaltyValue::PositiveLoyalty(PositiveLoyalty {
                magnitude: LoyaltyMagnitude { magnitude: one },
            }),
        ),
        (
            "[0]: You gain X life.",
            LoyaltyValue::ZeroLoyalty(ZeroLoyalty),
        ),
        (
            "[−2]: You gain X life.",
            LoyaltyValue::NegativeLoyalty(NegativeLoyalty {
                magnitude: LoyaltyMagnitude { magnitude: two },
            }),
        ),
    ] {
        let Ability::Activated(activated) = assert_selected_activated(&parser, &context, text)
        else {
            panic!("a loyalty-cost surface has the activated envelope: {text}")
        };
        let [ActivationCostComponent::Loyalty(Loyalty { value })] = activated.costs() else {
            panic!("a bracketed value is one loyalty component: {text}")
        };
        assert_eq!(
            value, &expected,
            "loyalty sign and magnitude are typed: {text}"
        );
    }

    for text in [
        "[-2]: You gain X life.",
        "[+0]: You gain X life.",
        "[−0]: You gain X life.",
        "[1]: You gain X life.",
        "[01]: You gain X life.",
        "[+01]: You gain X life.",
        "[−01]: You gain X life.",
        "[+1: You gain X life.",
        "+1]: You gain X life.",
        "[[+1]]: You gain X life.",
        "[]: You gain X life.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "malformed or semantically invalid loyalty value must reject: {text}",
        );
    }
}

#[test]
fn mixed_activation_has_exact_ast_render_build_visit_and_byte_ownership() {
    let parser = parser();
    let context = context("Context Card", false);
    let text =
        "{2}{W/U}{T}, [−2], Destroy target creature: You gain X life. Destroy target creature.";
    let analysis = parser.analyze(text, &context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("mixed activation must select: {analysis:?}"));
    assert_eq!(
        analysis
            .decision()
            .expect("selected mixed activation has a decision")
            .candidates()
            .len(),
        1,
        "the generated build yields one semantic AST",
    );
    assert_eq!(selected.render(&context, parser.environment()), text);

    let Ability::Activated(activated) = selected else {
        panic!("the cost-colon surface builds the generated activated envelope")
    };
    let [
        ActivationCostComponent::SymbolRun(symbols),
        ActivationCostComponent::Loyalty(Loyalty {
            value: LoyaltyValue::NegativeLoyalty(NegativeLoyalty { magnitude }),
        }),
        ActivationCostComponent::Clause(CostClause {
            predicate: VerbPhrase::Destroy(_),
        }),
    ] = activated.costs()
    else {
        panic!("mixed cost builds the positional SymbolRun/Loyalty/Clause AST")
    };
    assert_eq!(
        symbols.symbols(),
        &[
            CostSymbol::GenericCostSymbol(GenericCostSymbol {
                magnitude: ScalarNumber { magnitude: 2 },
            }),
            CostSymbol::FixedCostSymbol(FixedSymbol {
                symbol: FixedCostSymbol::HybridWhiteBlue,
            }),
            CostSymbol::FixedCostSymbol(FixedSymbol {
                symbol: FixedCostSymbol::Tap,
            }),
        ],
    );
    assert_eq!(magnitude.magnitude, NonZeroU32::new(2).unwrap());
    let AbilityBody::Sentences(sentences) = &activated.body;
    assert_eq!(sentences.sentences().len(), 2);
    assert!(matches!(sentences.sentences()[0], Sentence::Declarative(_)));
    assert!(matches!(sentences.sentences()[1], Sentence::Imperative(_)));

    let ownership = analysis
        .ownership()
        .expect("selected mixed activation owns every byte");
    assert!(ownership.failures().is_empty(), "{ownership:?}");
    assert!(ownership.summary().covered(), "{ownership:?}");
    assert_eq!(
        ownership
            .parsed_claims()
            .iter()
            .map(|claim| (
                claim.span().start,
                claim.span().end,
                claim.stable_owner_id(),
            ))
            .collect::<Vec<_>>(),
        [
            (0, 1, "form:symbol_run/symbol_run/0/prefix"),
            (1, 2, "codec:ScalarNumber"),
            (2, 4, "structural:SymbolRun/symbols/separator/uniform/0"),
            (4, 7, "vocab:FixedCostSymbol/HybridWhiteBlue"),
            (7, 9, "structural:SymbolRun/symbols/separator/uniform/0"),
            (9, 10, "vocab:FixedCostSymbol/Tap"),
            (10, 11, "form:symbol_run/symbol_run/0/suffix"),
            (11, 13, "structural:Activated/costs/separator/first/0"),
            (13, 14, "form:loyalty/loyalty/0/prefix"),
            (14, 17, "form:negative_loyalty/negative_loyalty/0/affix"),
            (17, 18, "codec:LoyaltyMagnitude"),
            (18, 19, "form:loyalty/loyalty/0/suffix"),
            (19, 21, "structural:Activated/costs/separator/last/0"),
            (21, 28, "lexeme:keyword_action/Destroy/bare"),
            (
                28,
                35,
                "form:target_singular_selector/target_singular_selector/0"
            ),
            (35, 44, "lexeme:type/Creature/singular"),
            (44, 46, "form:activated/activated/1"),
            (46, 49, "vocab:SubjectPronoun/You"),
            (49, 54, "lexeme:VerbLexeme/Gain/bare"),
            (54, 56, "vocab:Variable/X"),
            (56, 61, "form:gain_life/gain_life/2"),
            (61, 62, "structural:Sentences/sentences/terminator/0"),
            (62, 63, "structural:Sentences/sentences/separator/uniform/0"),
            (63, 70, "lexeme:keyword_action/Destroy/bare"),
            (
                70,
                77,
                "form:target_singular_selector/target_singular_selector/0"
            ),
            (77, 86, "lexeme:type/Creature/singular"),
            (86, 87, "structural:Sentences/sentences/terminator/0"),
        ],
    );

    let mut visitor = CostVisitor::default();
    visitor.visit_ability(selected);
    assert_eq!(
        visitor.0,
        [
            CostVisit::Node("Ability"),
            CostVisit::Node("Activated"),
            CostVisit::Node("ActivationCostComponent"),
            CostVisit::Node("SymbolRun"),
            CostVisit::Node("CostSymbol"),
            CostVisit::Node("GenericCostSymbol"),
            CostVisit::Generic(2),
            CostVisit::Node("CostSymbol"),
            CostVisit::Node("FixedSymbol"),
            CostVisit::Fixed(FixedCostSymbol::HybridWhiteBlue),
            CostVisit::Node("CostSymbol"),
            CostVisit::Node("FixedSymbol"),
            CostVisit::Fixed(FixedCostSymbol::Tap),
            CostVisit::Node("ActivationCostComponent"),
            CostVisit::Node("Loyalty"),
            CostVisit::Node("LoyaltyValue"),
            CostVisit::Node("NegativeLoyalty"),
            CostVisit::LoyaltyMagnitude(NonZeroU32::new(2).unwrap()),
            CostVisit::Node("ActivationCostComponent"),
            CostVisit::Node("CostClause"),
            CostVisit::Node("AbilityBody"),
            CostVisit::Node("Sentences"),
        ],
    );
}

#[test]
fn activation_boundaries_case_and_out_of_scope_costs_reject() {
    let parser = parser();
    let context = context("Context Card", false);
    for text in [
        ": You gain X life.",
        "{T} : You gain X life.",
        "{T}:You gain X life.",
        "{T}:  You gain X life.",
        "{T}: you gain X life.",
        "{2},{T}: You gain X life.",
        "{2},  {T}: You gain X life.",
        "{2}, destroy target creature: You gain X life.",
        "{2}, Destroy target creature,: You gain X life.",
        "{T}: You gain X life. destroy target creature.",
        "Channel — {T}: You gain X life.",
        "Sacrifice a creature: You gain X life.",
        "Discard a card: You gain X life.",
        "Exile a card: You gain X life.",
        "Remove a counter: You gain X life.",
        "Reveal a card: You gain X life.",
        "Pay 2 life: You gain X life.",
        "Tap a creature: You gain X life.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "malformed or later-plan activation surface must reject: {text}",
        );
    }
}

#[test]
fn activated_body_reuses_legendary_license_and_rejects_ordinary_shortening() {
    let parser = parser();
    for (name, legendary, surface, spelling) in [
        (
            "Aang, A Lot to Learn",
            true,
            "Aang",
            SelfReferenceSpelling::Abbreviated,
        ),
        (
            "Grizzly Bears",
            false,
            "Grizzly Bears",
            SelfReferenceSpelling::Full,
        ),
    ] {
        let context = context(name, legendary);
        let text = format!("{{T}}: {surface} gains 2 life.");
        let selected = assert_selected_activated(&parser, &context, &text);
        let mut visitor = SelfReferenceVisitor::default();
        visitor.visit_ability(&selected);
        assert_eq!(visitor.spellings, [spelling], "{text}");
        assert_eq!(
            SourceSelfReference::new(SelfReferenceSpelling::Abbreviated, &context).is_some(),
            legendary,
            "the activated envelope uses the same metadata-licensed identity constructor",
        );
    }

    let ordinary = context("Grizzly Bears", false);
    assert!(
        parser
            .parse("{T}: Grizzly gains 2 life.", &ordinary)
            .is_err(),
        "an activated body cannot manufacture an ordinary-name abbreviation",
    );
}

#[test]
fn english_v2_direct_dependency_set_stays_independent_of_core_features_and_v1() {
    let output = Command::new(env!("CARGO"))
        .args([
            "tree",
            "--manifest-path",
            concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
            "--package",
            "deckmaste_english_v2",
            "--depth",
            "1",
            "--edges",
            "normal",
            "--prefix",
            "none",
            "--format",
            "{p}",
        ])
        .output()
        .expect("Cargo can resolve the English-v2 direct dependency graph");
    assert!(
        output.status.success(),
        "cargo tree failed: {}",
        String::from_utf8_lossy(&output.stderr),
    );
    let packages = String::from_utf8(output.stdout)
        .expect("cargo tree package names are UTF-8")
        .lines()
        .filter_map(|line| line.split_whitespace().next().map(str::to_owned))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        packages,
        BTreeSet::from([
            "anyhow".to_owned(),
            "deckmaste_construction".to_owned(),
            "deckmaste_english_v2".to_owned(),
            "macro_ron".to_owned(),
            "thiserror".to_owned(),
        ]),
        "the complete direct normal-dependency set remains unchanged and excludes deckmaste_core, deckmaste_features, and deckmaste_english",
    );
}
