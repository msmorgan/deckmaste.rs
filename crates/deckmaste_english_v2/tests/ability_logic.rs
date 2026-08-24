use std::collections::BTreeSet;
use std::num::NonZeroU32;
use std::path::Path;
use std::process::Command;

use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::LexicalProvenanceKind;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::parser::TextSpan;
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
    assert!(matches!(finite.clause, FiniteClause::PlainFiniteClause(_)));
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

    fn visit_sentence(&mut self, value: &Sentence) {
        self.0.push(CostVisit::Node("Sentence"));
        deckmaste_english_v2::visit::walk_sentence(self, value);
    }

    fn visit_declarative(&mut self, value: &Declarative) {
        self.0.push(CostVisit::Node("Declarative"));
        deckmaste_english_v2::visit::walk_declarative(self, value);
    }

    fn visit_imperative(&mut self, value: &Imperative) {
        self.0.push(CostVisit::Node("Imperative"));
        deckmaste_english_v2::visit::walk_imperative(self, value);
    }

    fn visit_verb_phrase(&mut self, value: &VerbPhrase) {
        self.0.push(CostVisit::Node("VerbPhrase"));
        deckmaste_english_v2::visit::walk_verb_phrase(self, value);
    }

    fn visit_destroy(&mut self, value: &Destroy) {
        self.0.push(CostVisit::Node("Destroy"));
        deckmaste_english_v2::visit::walk_destroy(self, value);
    }

    fn visit_gain_life(&mut self, value: &GainLife) {
        self.0.push(CostVisit::Node("GainLife"));
        deckmaste_english_v2::visit::walk_gain_life(self, value);
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
            "FiniteClause",
            "AbilityBody",
            "Sentences",
            "Clause",
            "FiniteClause",
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
    assert_eq!(variants("Clause"), ["Finite", "Coordination"]);
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
        trigger: TriggerPrefix::Finite(Finite {
            marker: TriggerMarker::Whenever,
            clause: FiniteClause::PlainFiniteClause(PlainFiniteClause {
                subject: Subject::SubjectPronoun(PersonalSubject {
                    word: SubjectPronoun::You,
                }),
                predicate: Predicate::Atomic(VerbPhrase::Connive(Connive)),
            }),
        }),
        intervening_if: None,
        body: AbilityBody::Sentences(
            Sentences::new(vec![Sentence::Imperative(Imperative {
                predicate: Predicate::Atomic(VerbPhrase::Connive(Connive)),
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
    let decision = analysis
        .decision()
        .expect("a selected trigger has a selection decision");
    assert_eq!(
        decision.survivors().len(),
        1,
        "closed trigger surface has exactly one surviving candidate: {text}"
    );
    assert!(matches!(
        decision.resolution(),
        SelectionResolution::Unique | SelectionResolution::Specificity
    ));
    assert!(decision.exception_uses().is_empty());
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
        assert!(matches!(finite.clause, FiniteClause::PlainFiniteClause(_)));
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
                    clause:
                        FiniteClause::PlainFiniteClause(PlainFiniteClause {
                            subject:
                                Subject::SubjectPronoun(PersonalSubject {
                                    word: SubjectPronoun::You,
                                }),
                            predicate: Predicate::Atomic(VerbPhrase::Connive(Connive)),
                        }),
                },
            ))),
        ..
    }) = intervening
    else {
        panic!("the immediate if clause has its dedicated finite-condition attachment")
    };
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
#[allow(
    clippy::too_many_lines,
    reason = "the boundary ownership and visitor oracle is deliberately literal"
)]
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
            "Clause",
            "FiniteClause",
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
            (42, 43, "form:finite_condition/finite_condition/2"),
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
            "FiniteClause",
            "ConditionClause",
            "FiniteCondition",
            "FiniteConditionValue",
            "FiniteClause",
            "AbilityBody",
            "Sentences",
            "Clause",
            "FiniteClause",
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
            "Clause",
            "FiniteClause",
        ]
    );

    for invalid in [
        "at the beginning of each player's draw step, you gain X life.",
        "At The beginning of each player's draw step, you gain X life.",
        "At the beginning of each player's draw step,you gain X life.",
        "At the beginning of each player's draw step, You gain X life.",
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
            CostVisit::Node("VerbPhrase"),
            CostVisit::Node("Destroy"),
            CostVisit::Node("AbilityBody"),
            CostVisit::Node("Sentences"),
            CostVisit::Node("Sentence"),
            CostVisit::Node("Declarative"),
            CostVisit::Node("VerbPhrase"),
            CostVisit::Node("GainLife"),
            CostVisit::Node("Sentence"),
            CostVisit::Node("Imperative"),
            CostVisit::Node("VerbPhrase"),
            CostVisit::Node("Destroy"),
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

fn gain_life_predicate(magnitude: u32) -> VerbPhrase {
    VerbPhrase::GainLife(GainLife {
        amount: Amount::Number(NumberAmount {
            number: ScalarNumber { magnitude },
        }),
    })
}

fn you_subject() -> Subject {
    Subject::SubjectPronoun(PersonalSubject {
        word: SubjectPronoun::You,
    })
}

fn plain_finite(subject: Subject, predicate: Predicate) -> FiniteClause {
    FiniteClause::PlainFiniteClause(PlainFiniteClause { subject, predicate })
}

fn predicate_identity(predicate: &VerbPhrase) -> String {
    match predicate {
        VerbPhrase::Connive(_) => "connive".to_owned(),
        VerbPhrase::GainLife(GainLife {
            amount:
                Amount::Number(NumberAmount {
                    number: ScalarNumber { magnitude },
                }),
        }) => format!("gain:{magnitude}"),
        other => panic!("unexpected coordination predicate payload: {other:?}"),
    }
}

fn unqualified_reference(noun_phrase: &NounPhrase) -> &UnqualifiedReference {
    let NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
        reference:
            NumericStage::UnqualifiedNumericStage(UnqualifiedNumericStage {
                reference:
                    ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                        reference:
                            ControllerStage::UnqualifiedControllerStage(UnqualifiedControllerStage {
                                reference,
                            }),
                    }),
            }),
    }) = noun_phrase
    else {
        panic!("coordination subject uses the exact unqualified staging")
    };
    reference
}

fn subject_identity(subject: &Subject) -> &'static str {
    match subject {
        Subject::SubjectPronoun(PersonalSubject {
            word: SubjectPronoun::You,
        }) => "you",
        Subject::SubjectNominal(NominalSubject { value }) => match unqualified_reference(value) {
            UnqualifiedReference::SelfReference(reference) => {
                assert_eq!(reference.spelling(), SelfReferenceSpelling::Abbreviated);
                "self:abbreviated"
            }
            UnqualifiedReference::IndefiniteReference(IndefiniteReference {
                nominal:
                    SingularNominal::BareSingularNominal(BareSingularNominal {
                        head:
                            SingularHead::CommonSingularHead(CommonSingularHead {
                                noun: CommonNoun::Player,
                            }),
                    }),
            }) => "player",
            other => panic!("unexpected nominal coordination subject payload: {other:?}"),
        },
        other @ Subject::SubjectPronoun(_) => {
            panic!("unexpected coordination subject payload: {other:?}")
        }
    }
}

fn finite_clause_identity(clause: &FiniteClause) -> String {
    let FiniteClause::PlainFiniteClause(PlainFiniteClause { subject, predicate }) = clause else {
        panic!("clause coordination stores complete plain finite clauses")
    };
    let Predicate::Atomic(predicate) = predicate else {
        panic!("the exact clause witnesses contain atomic predicates")
    };
    format!(
        "{}/{}",
        subject_identity(subject),
        predicate_identity(predicate)
    )
}

fn assert_one_logic_candidate(parser: &Parser, context: &ParseContext<'_>, text: &str) -> Ability {
    let analysis = parser.analyze(text, context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("logic witness must select: {text}: {analysis:?}"));
    assert_eq!(
        analysis
            .decision()
            .expect("a selected logic witness has a decision")
            .candidates()
            .len(),
        1,
        "the staged grammar materializes one semantic candidate: {text}",
    );
    assert_eq!(selected.render(context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("selected logic witness has byte ownership");
    assert!(ownership.failures().is_empty(), "{text}: {ownership:?}");
    assert!(ownership.summary().covered(), "{text}: {ownership:?}");
    selected.clone()
}

#[test]
fn auxiliaries_are_lexical_clause_structure_with_derived_bare_predicates() {
    let parser = parser();
    let context = context("Context Card", false);

    for (surface, auxiliary) in [
        ("may", Auxiliary::May),
        ("can", Auxiliary::Can),
        ("can't", Auxiliary::Cant),
        ("must", Auxiliary::Must),
    ] {
        let text = format!("You {surface} gain 2 life.");
        let selected = assert_one_logic_candidate(&parser, &context, &text);
        assert_eq!(
            selected,
            Ability::Plain(Plain {
                body: AbilityBody::Sentences(
                    Sentences::new(vec![Sentence::Declarative(Declarative {
                        clause: Clause::Finite(FiniteClause::AuxiliaryFiniteClause(
                            AuxiliaryFiniteClause {
                                subject: you_subject(),
                                auxiliary,
                                predicate: Predicate::Atomic(gain_life_predicate(2)),
                            },
                        )),
                    })])
                    .expect("the independently built ability body is nonempty"),
                ),
            }),
            "the generated AST stores only the lexical auxiliary and atomic predicate",
        );
    }

    for invalid in [
        "You may gains 2 life.",
        "A player can gains 2 life.",
        "You can't gains 2 life.",
        "A player must gains 2 life.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "an auxiliary requires the derived bare predicate form: {invalid}",
        );
    }
    assert!(
        parser.parse("A player gains 2 life.", &context).is_ok(),
        "ordinary finite clauses retain subject-derived third-person agreement",
    );
    assert!(
        parser.parse("A player gain 2 life.", &context).is_err(),
        "ordinary finite clauses do not inherit auxiliary bare agreement",
    );
}

#[test]
fn cant_apostrophe_has_one_lexical_owner_and_no_permission_leaf() {
    let parser = parser();
    let context = context("Context Card", false);
    let text = "You can't gain X life.";
    let analysis = parser.analyze(text, &context);
    assert!(analysis.selected().is_some(), "can't witness must select");
    let claims = analysis
        .ownership()
        .expect("can't witness has ownership")
        .parsed_claims();
    let claim = claims
        .iter()
        .find(|claim| claim.stable_owner_id() == "vocab:Auxiliary/Cant")
        .expect("the lexical auxiliary owns its realized bytes");
    assert_eq!((claim.span().start, claim.span().end), (3, 9));
    assert_eq!(&text[claim.span().start..claim.span().end], " can't");

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
    assert_eq!(variants("Auxiliary"), ["May", "Can", "Cant", "Must"]);
    assert_eq!(
        variants("VerbPhrase"),
        ["Destroy", "Connive", "DealDamage", "GainLife"],
        "linguistic auxiliaries add no predicate leaf",
    );
    for forbidden in [
        "Permission",
        "MayPredicate",
        "CanPredicate",
        "CantPredicate",
        "MustPredicate",
    ] {
        assert!(
            !file.items.iter().any(|item| {
                matches!(item, syn::Item::Enum(item) if item.ident == forbidden)
                    || matches!(item, syn::Item::Struct(item) if item.ident == forbidden)
            }),
            "the surface grammar does not expose engine/predicate type {forbidden}",
        );
    }
}

#[derive(Clone, Copy)]
enum CoordinationKind {
    And,
    Or,
    AndOr,
}

fn assert_predicate_coordination(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    kind: CoordinationKind,
    expected_members: &[&str],
) {
    let selected = assert_one_logic_candidate(parser, context, text);
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(sentences),
    }) = selected
    else {
        panic!("predicate coordination is a plain sentence body: {text}")
    };
    let [
        Sentence::Declarative(Declarative {
            clause:
                Clause::Finite(FiniteClause::PlainFiniteClause(PlainFiniteClause {
                    predicate: Predicate::Coordination(coordination),
                    ..
                })),
        }),
    ] = sentences.sentences()
    else {
        panic!("predicate coordination has the staged declarative AST: {text}")
    };
    let members = match (kind, coordination) {
        (CoordinationKind::And, PredicateCoordination::AndPredicateCoordination(value)) => {
            value.members()
        }
        (CoordinationKind::Or, PredicateCoordination::OrPredicateCoordination(value)) => {
            value.members()
        }
        (CoordinationKind::AndOr, PredicateCoordination::AndOrPredicateCoordination(value)) => {
            value.members()
        }
        _ => panic!("coordinator meaning is stored independently of punctuation: {text}"),
    };
    assert_eq!(
        members.iter().map(predicate_identity).collect::<Vec<_>>(),
        expected_members,
        "the AST preserves every predicate payload in source order: {text}",
    );
}

#[test]
fn predicate_coordination_is_nary_with_exact_pair_serial_and_final_surfaces() {
    let parser = parser();
    let context = context("Context Card", false);
    for (kind, coordinator) in [
        (CoordinationKind::And, "and"),
        (CoordinationKind::Or, "or"),
        (CoordinationKind::AndOr, "and/or"),
    ] {
        for (text, members) in [
            (
                format!("You gain 1 life {coordinator} connive."),
                &["gain:1", "connive"][..],
            ),
            (
                format!("You gain 1 life, connive, {coordinator} gain 2 life."),
                &["gain:1", "connive", "gain:2"][..],
            ),
            (
                format!("You gain 1 life, connive, gain 2 life, {coordinator} gain 3 life."),
                &["gain:1", "connive", "gain:2", "gain:3"][..],
            ),
        ] {
            assert_predicate_coordination(&parser, &context, &text, kind, members);
        }
    }
}

fn assert_clause_coordination(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    kind: CoordinationKind,
    expected_members: &[&str],
) {
    let selected = assert_one_logic_candidate(parser, context, text);
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(sentences),
    }) = selected
    else {
        panic!("clause coordination is a plain sentence body: {text}")
    };
    let [
        Sentence::Declarative(Declarative {
            clause: Clause::Coordination(coordination),
        }),
    ] = sentences.sentences()
    else {
        panic!("complete clauses coordinate above finite clauses: {text}")
    };
    let members = match (kind, coordination) {
        (CoordinationKind::And, ClauseCoordination::AndClauseCoordination(value)) => {
            value.members()
        }
        (CoordinationKind::Or, ClauseCoordination::OrClauseCoordination(value)) => value.members(),
        (CoordinationKind::AndOr, ClauseCoordination::AndOrClauseCoordination(value)) => {
            value.members()
        }
        _ => panic!("clause coordinator meaning is stored independently: {text}"),
    };
    assert_eq!(
        members
            .iter()
            .map(finite_clause_identity)
            .collect::<Vec<_>>(),
        expected_members,
        "the AST preserves every complete finite-clause payload in source order: {text}",
    );
}

#[test]
fn complete_finite_clause_coordination_is_nary_and_preserves_member_agreement() {
    let parser = parser();
    let context = context("Aang, A Lot to Learn", true);
    for (kind, coordinator) in [
        (CoordinationKind::And, "and"),
        (CoordinationKind::Or, "or"),
        (CoordinationKind::AndOr, "and/or"),
    ] {
        for (text, members) in [
            (
                format!("Aang gains 1 life {coordinator} you connive."),
                &["self:abbreviated/gain:1", "you/connive"][..],
            ),
            (
                format!("Aang gains 1 life, you connive, {coordinator} a player gains 2 life."),
                &["self:abbreviated/gain:1", "you/connive", "player/gain:2"][..],
            ),
            (
                format!(
                    "Aang gains 1 life, you connive, a player gains 2 life, {coordinator} Aang gains 3 life."
                ),
                &[
                    "self:abbreviated/gain:1",
                    "you/connive",
                    "player/gain:2",
                    "self:abbreviated/gain:3",
                ][..],
            ),
        ] {
            assert_clause_coordination(&parser, &context, &text, kind, members);
        }
    }
    for invalid in [
        "Aang gain 1 life and you connive.",
        "Aang gains 1 life and you connives.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "each coordinated finite clause derives agreement from its own subject: {invalid}",
        );
    }
}

#[test]
fn malformed_coordination_and_minimum_arity_are_rejected() {
    let parser = parser();
    let context = context("Context Card", false);
    for invalid in [
        "You gain 1 life, and connive.",
        "You gain 1 life, connive and gain 2 life.",
        "You gain 1 life and and connive.",
        "You gain 1 life and connive or gain 2 life.",
        "You gain 1 life and / or connive.",
        "You gain 1 life,and connive.",
        "You gain 1 life, you connive and a player gains 2 life.",
        "You gain 1 life and you connive or a player gains 2 life.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "malformed punctuation or mixed/duplicated coordinator must reject: {invalid}",
        );
    }

    assert!(AndPredicateCoordination::new(vec![gain_life_predicate(1)]).is_none());
    assert!(
        AndClauseCoordination::new(vec![plain_finite(
            you_subject(),
            Predicate::Atomic(gain_life_predicate(1)),
        )])
        .is_none()
    );

    let selected = assert_one_logic_candidate(&parser, &context, "You gain 1 life.");
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(sentences),
    }) = selected
    else {
        panic!("one atomic clause remains a plain ability")
    };
    assert!(matches!(
        sentences.sentences(),
        [Sentence::Declarative(Declarative {
            clause: Clause::Finite(FiniteClause::PlainFiniteClause(PlainFiniteClause {
                predicate: Predicate::Atomic(_),
                ..
            })),
        })]
    ));
}

#[derive(Debug, PartialEq, Eq)]
enum LogicVisit {
    Predicate,
    PredicateCoordination,
    AndPredicateCoordination,
    GainLife(u32),
    Connive,
    Clause,
    ClauseCoordination,
    AndClauseCoordination,
    FiniteClause,
    AuxiliaryFiniteClause,
    Subject,
    SubjectPronoun(SubjectPronoun),
    CommonNoun(CommonNoun),
    SelfReference(SelfReferenceSpelling),
    Auxiliary(Auxiliary),
}

#[derive(Default)]
struct LogicVisitor(Vec<LogicVisit>);

impl Visitor for LogicVisitor {
    fn visit_predicate(&mut self, value: &Predicate) {
        self.0.push(LogicVisit::Predicate);
        deckmaste_english_v2::visit::walk_predicate(self, value);
    }

    fn visit_predicate_coordination(&mut self, value: &PredicateCoordination) {
        self.0.push(LogicVisit::PredicateCoordination);
        deckmaste_english_v2::visit::walk_predicate_coordination(self, value);
    }

    fn visit_and_predicate_coordination(&mut self, value: &AndPredicateCoordination) {
        self.0.push(LogicVisit::AndPredicateCoordination);
        deckmaste_english_v2::visit::walk_and_predicate_coordination(self, value);
    }

    fn visit_verb_phrase(&mut self, value: &VerbPhrase) {
        match value {
            VerbPhrase::GainLife(GainLife {
                amount:
                    Amount::Number(NumberAmount {
                        number: ScalarNumber { magnitude },
                    }),
            }) => self.0.push(LogicVisit::GainLife(*magnitude)),
            VerbPhrase::Connive(_) => self.0.push(LogicVisit::Connive),
            other => panic!("unexpected logic visitor predicate payload: {other:?}"),
        }
    }

    fn visit_clause(&mut self, value: &Clause) {
        self.0.push(LogicVisit::Clause);
        deckmaste_english_v2::visit::walk_clause(self, value);
    }

    fn visit_clause_coordination(&mut self, value: &ClauseCoordination) {
        self.0.push(LogicVisit::ClauseCoordination);
        deckmaste_english_v2::visit::walk_clause_coordination(self, value);
    }

    fn visit_and_clause_coordination(&mut self, value: &AndClauseCoordination) {
        self.0.push(LogicVisit::AndClauseCoordination);
        deckmaste_english_v2::visit::walk_and_clause_coordination(self, value);
    }

    fn visit_finite_clause(&mut self, value: &FiniteClause) {
        self.0.push(LogicVisit::FiniteClause);
        deckmaste_english_v2::visit::walk_finite_clause(self, value);
    }

    fn visit_auxiliary_finite_clause(&mut self, value: &AuxiliaryFiniteClause) {
        self.0.push(LogicVisit::AuxiliaryFiniteClause);
        deckmaste_english_v2::visit::walk_auxiliary_finite_clause(self, value);
    }

    fn visit_subject(&mut self, value: &Subject) {
        self.0.push(LogicVisit::Subject);
        deckmaste_english_v2::visit::walk_subject(self, value);
    }

    fn visit_subject_pronoun(&mut self, value: SubjectPronoun) {
        self.0.push(LogicVisit::SubjectPronoun(value));
    }

    fn visit_common_noun(&mut self, value: CommonNoun) {
        self.0.push(LogicVisit::CommonNoun(value));
    }

    fn visit_self_reference_spelling(&mut self, value: SelfReferenceSpelling) {
        self.0.push(LogicVisit::SelfReference(value));
    }

    fn visit_auxiliary(&mut self, value: Auxiliary) {
        self.0.push(LogicVisit::Auxiliary(value));
    }
}

#[test]
fn logic_visitors_follow_semantic_member_order() {
    let parser = parser();
    let context = context("Context Card", false);
    let predicate_ability =
        assert_one_logic_candidate(&parser, &context, "You gain 1 life and connive.");
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(predicate_sentences),
    }) = predicate_ability
    else {
        unreachable!()
    };
    let [
        Sentence::Declarative(Declarative {
            clause:
                Clause::Finite(FiniteClause::PlainFiniteClause(PlainFiniteClause { predicate, .. })),
        }),
    ] = predicate_sentences.sentences()
    else {
        unreachable!()
    };
    let mut visitor = LogicVisitor::default();
    visitor.visit_predicate(predicate);
    assert_eq!(
        visitor.0,
        [
            LogicVisit::Predicate,
            LogicVisit::PredicateCoordination,
            LogicVisit::AndPredicateCoordination,
            LogicVisit::GainLife(1),
            LogicVisit::Connive,
        ],
    );

    let clause_ability = assert_one_logic_candidate(
        &parser,
        &context,
        "You gain 1 life and a player gains 2 life.",
    );
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(clause_sentences),
    }) = clause_ability
    else {
        unreachable!()
    };
    let [Sentence::Declarative(Declarative { clause })] = clause_sentences.sentences() else {
        unreachable!()
    };
    let mut visitor = LogicVisitor::default();
    visitor.visit_clause(clause);
    assert_eq!(
        visitor.0,
        [
            LogicVisit::Clause,
            LogicVisit::ClauseCoordination,
            LogicVisit::AndClauseCoordination,
            LogicVisit::FiniteClause,
            LogicVisit::Subject,
            LogicVisit::SubjectPronoun(SubjectPronoun::You),
            LogicVisit::Predicate,
            LogicVisit::GainLife(1),
            LogicVisit::FiniteClause,
            LogicVisit::Subject,
            LogicVisit::CommonNoun(CommonNoun::Player),
            LogicVisit::Predicate,
            LogicVisit::GainLife(2),
        ],
    );

    let auxiliary = assert_one_logic_candidate(&parser, &context, "You can't connive.");
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(auxiliary_sentences),
    }) = auxiliary
    else {
        unreachable!()
    };
    let [
        Sentence::Declarative(Declarative {
            clause: Clause::Finite(FiniteClause::AuxiliaryFiniteClause(auxiliary)),
        }),
    ] = auxiliary_sentences.sentences()
    else {
        unreachable!()
    };
    let mut visitor = LogicVisitor::default();
    visitor.visit_auxiliary_finite_clause(auxiliary);
    assert_eq!(
        visitor.0,
        [
            LogicVisit::AuxiliaryFiniteClause,
            LogicVisit::Subject,
            LogicVisit::SubjectPronoun(SubjectPronoun::You),
            LogicVisit::Auxiliary(Auxiliary::Cant),
            LogicVisit::Predicate,
            LogicVisit::Connive,
        ],
    );
}

fn assert_coordination_separator_claims(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    expected: &[(usize, usize, &str)],
) {
    let analysis = parser.analyze(text, context);
    assert!(
        analysis.selected().is_some(),
        "separator witness selects: {text}"
    );
    let claims = analysis
        .ownership()
        .expect("separator witness has ownership")
        .parsed_claims()
        .iter()
        .filter(|claim| claim.stable_owner_id().contains("/members/separator/"))
        .map(|claim| {
            assert_eq!(
                claim.kind(),
                LexicalProvenanceKind::FormLiteral,
                "structural separators retain literal provenance: {text}",
            );
            (
                claim.span().start,
                claim.span().end,
                claim.stable_owner_id(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(claims, expected, "exact separator owners and spans: {text}");
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the literal oracle independently pins every coordinator and positional edge"
)]
fn coordination_separators_have_exact_positional_spans_owners_and_provenance() {
    let parser = parser();
    let ordinary = context("Context Card", false);
    for (text, claims) in [
        (
            "You gain 1 life and connive.",
            &[(
                15,
                20,
                "structural:AndPredicateCoordination/members/separator/pair/0",
            )][..],
        ),
        (
            "You gain 1 life, connive, and gain 2 life.",
            &[
                (
                    15,
                    17,
                    "structural:AndPredicateCoordination/members/separator/first/0",
                ),
                (
                    24,
                    30,
                    "structural:AndPredicateCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "You gain 1 life, connive, gain 2 life, and gain 3 life.",
            &[
                (
                    15,
                    17,
                    "structural:AndPredicateCoordination/members/separator/first/0",
                ),
                (
                    24,
                    26,
                    "structural:AndPredicateCoordination/members/separator/middle/0",
                ),
                (
                    37,
                    43,
                    "structural:AndPredicateCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "You gain 1 life or connive.",
            &[(
                15,
                19,
                "structural:OrPredicateCoordination/members/separator/pair/0",
            )][..],
        ),
        (
            "You gain 1 life, connive, or gain 2 life.",
            &[
                (
                    15,
                    17,
                    "structural:OrPredicateCoordination/members/separator/first/0",
                ),
                (
                    24,
                    29,
                    "structural:OrPredicateCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "You gain 1 life, connive, gain 2 life, or gain 3 life.",
            &[
                (
                    15,
                    17,
                    "structural:OrPredicateCoordination/members/separator/first/0",
                ),
                (
                    24,
                    26,
                    "structural:OrPredicateCoordination/members/separator/middle/0",
                ),
                (
                    37,
                    42,
                    "structural:OrPredicateCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "You gain 1 life and/or connive.",
            &[(
                15,
                23,
                "structural:AndOrPredicateCoordination/members/separator/pair/0",
            )][..],
        ),
        (
            "You gain 1 life, connive, and/or gain 2 life.",
            &[
                (
                    15,
                    17,
                    "structural:AndOrPredicateCoordination/members/separator/first/0",
                ),
                (
                    24,
                    33,
                    "structural:AndOrPredicateCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "You gain 1 life, connive, gain 2 life, and/or gain 3 life.",
            &[
                (
                    15,
                    17,
                    "structural:AndOrPredicateCoordination/members/separator/first/0",
                ),
                (
                    24,
                    26,
                    "structural:AndOrPredicateCoordination/members/separator/middle/0",
                ),
                (
                    37,
                    46,
                    "structural:AndOrPredicateCoordination/members/separator/last/0",
                ),
            ][..],
        ),
    ] {
        assert_coordination_separator_claims(&parser, &ordinary, text, claims);
    }

    let legendary = context("Aang, A Lot to Learn", true);
    for (text, claims) in [
        (
            "Aang gains 1 life and you connive.",
            &[(
                17,
                22,
                "structural:AndClauseCoordination/members/separator/pair/0",
            )][..],
        ),
        (
            "Aang gains 1 life, you connive, and a player gains 2 life.",
            &[
                (
                    17,
                    19,
                    "structural:AndClauseCoordination/members/separator/first/0",
                ),
                (
                    30,
                    36,
                    "structural:AndClauseCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "Aang gains 1 life, you connive, a player gains 2 life, and Aang gains 3 life.",
            &[
                (
                    17,
                    19,
                    "structural:AndClauseCoordination/members/separator/first/0",
                ),
                (
                    30,
                    32,
                    "structural:AndClauseCoordination/members/separator/middle/0",
                ),
                (
                    53,
                    59,
                    "structural:AndClauseCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "Aang gains 1 life or you connive.",
            &[(
                17,
                21,
                "structural:OrClauseCoordination/members/separator/pair/0",
            )][..],
        ),
        (
            "Aang gains 1 life, you connive, or a player gains 2 life.",
            &[
                (
                    17,
                    19,
                    "structural:OrClauseCoordination/members/separator/first/0",
                ),
                (
                    30,
                    35,
                    "structural:OrClauseCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "Aang gains 1 life, you connive, a player gains 2 life, or Aang gains 3 life.",
            &[
                (
                    17,
                    19,
                    "structural:OrClauseCoordination/members/separator/first/0",
                ),
                (
                    30,
                    32,
                    "structural:OrClauseCoordination/members/separator/middle/0",
                ),
                (
                    53,
                    58,
                    "structural:OrClauseCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "Aang gains 1 life and/or you connive.",
            &[(
                17,
                25,
                "structural:AndOrClauseCoordination/members/separator/pair/0",
            )][..],
        ),
        (
            "Aang gains 1 life, you connive, and/or a player gains 2 life.",
            &[
                (
                    17,
                    19,
                    "structural:AndOrClauseCoordination/members/separator/first/0",
                ),
                (
                    30,
                    39,
                    "structural:AndOrClauseCoordination/members/separator/last/0",
                ),
            ][..],
        ),
        (
            "Aang gains 1 life, you connive, a player gains 2 life, and/or Aang gains 3 life.",
            &[
                (
                    17,
                    19,
                    "structural:AndOrClauseCoordination/members/separator/first/0",
                ),
                (
                    30,
                    32,
                    "structural:AndOrClauseCoordination/members/separator/middle/0",
                ),
                (
                    53,
                    62,
                    "structural:AndOrClauseCoordination/members/separator/last/0",
                ),
            ][..],
        ),
    ] {
        assert_coordination_separator_claims(&parser, &legendary, text, claims);
    }
}

#[test]
fn coordination_case_transitions_and_self_reference_reuse_existing_envelopes() {
    let parser = parser();
    let ordinary = context("Context Card", false);
    for text in [
        "You gain 1 life and connive.",
        "Whenever a player connives, you gain 1 life and connive.",
        "{T}: You gain 1 life and connive.",
        "You gain 1 life and connive. Destroy target creature.",
    ] {
        assert_one_logic_candidate(&parser, &ordinary, text);
    }
    for invalid in [
        "you gain 1 life and connive.",
        "Whenever a player connives, You gain 1 life and connive.",
        "{T}: you gain 1 life and connive.",
        "You gain 1 life and connive. destroy target creature.",
    ] {
        assert!(
            parser.parse(invalid, &ordinary).is_err(),
            "structural position derives capitalization: {invalid}",
        );
    }

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
        for text in [
            format!("{surface} gains 1 life and connives."),
            format!("{surface} gains 1 life and you connive."),
        ] {
            let selected = assert_one_logic_candidate(&parser, &context, &text);
            let mut visitor = SelfReferenceVisitor::default();
            visitor.visit_ability(&selected);
            assert_eq!(visitor.spellings, [spelling], "{text}");
        }
    }
    let context = context("Grizzly Bears", false);
    for text in [
        "Grizzly gains 1 life and connives.",
        "Grizzly gains 1 life and you connive.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "coordination cannot manufacture an ordinary-name abbreviation: {text}",
        );
    }
}

fn connive_clause() -> FiniteClause {
    plain_finite(
        you_subject(),
        Predicate::Atomic(VerbPhrase::Connive(Connive {})),
    )
}

fn player_subject() -> Subject {
    Subject::SubjectNominal(NominalSubject {
        value: NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: NumericStage::UnqualifiedNumericStage(UnqualifiedNumericStage {
                reference: ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                    reference: ControllerStage::UnqualifiedControllerStage(
                        UnqualifiedControllerStage {
                            reference: UnqualifiedReference::IndefiniteReference(
                                IndefiniteReference {
                                    nominal: SingularNominal::BareSingularNominal(
                                        BareSingularNominal {
                                            head: SingularHead::CommonSingularHead(
                                                CommonSingularHead {
                                                    noun: CommonNoun::Player,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                }),
            }),
        }),
    })
}

fn player_connive_clause() -> FiniteClause {
    plain_finite(
        player_subject(),
        Predicate::Atomic(VerbPhrase::Connive(Connive {})),
    )
}

fn tap_cost() -> ActivationCostComponent {
    ActivationCostComponent::SymbolRun(
        SymbolRun::new(vec![CostSymbol::FixedCostSymbol(FixedSymbol {
            symbol: FixedCostSymbol::Tap,
        })])
        .expect("one fixed symbol forms a nonempty symbol run"),
    )
}

fn gain_clause(amount: u32) -> Clause {
    Clause::Finite(plain_finite(
        you_subject(),
        Predicate::Atomic(gain_life_predicate(amount)),
    ))
}

#[test]
fn attachment_products_have_an_intermediate_linguistic_stage_for_imperatives() {
    let parser = parser();
    let context = context("Context Card", false);
    let expected = Sentence::Attached(Attached {
        attachment: ClauseAttachment::PreposedIfPredicate(PreposedIfPredicate {
            condition: connive_clause(),
            body: Predicate::Atomic(VerbPhrase::Connive(Connive {})),
        }),
    });
    let selected = assert_one_logic_candidate(&parser, &context, "If you connive, connive.");
    assert_eq!(
        selected,
        Ability::Plain(Plain {
            body: AbilityBody::Sentences(Sentences::new(vec![expected]).expect("one sentence")),
        }),
        "the attachment lives between Sentence and its predicate/finite-clause payload",
    );
}

#[test]
fn conditional_attachments_have_distinct_position_shapes_and_exact_asts() {
    let parser = parser();
    let context = context("Context Card", false);
    let condition = connive_clause();
    let body = gain_clause(2);

    for (text, expected) in [
        (
            "If you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: ClauseAttachment::PreposedIf(PreposedIf {
                    condition: condition.clone(),
                    body: body.clone(),
                }),
            }),
        ),
        (
            "You gain 2 life if you connive.",
            Sentence::Attached(Attached {
                attachment: ClauseAttachment::PostposedIf(PostposedIf {
                    body: body.clone(),
                    condition: condition.clone(),
                }),
            }),
        ),
        (
            "You gain 2 life unless you connive.",
            Sentence::Attached(Attached {
                attachment: ClauseAttachment::PostposedUnless(PostposedUnless {
                    body: body.clone(),
                    condition: condition.clone(),
                }),
            }),
        ),
        (
            "As long as you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: ClauseAttachment::PreposedAsLongAs(PreposedAsLongAs {
                    condition: condition.clone(),
                    body: body.clone(),
                }),
            }),
        ),
        (
            "While you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: ClauseAttachment::PreposedWhile(PreposedWhile {
                    condition: condition.clone(),
                    body: body.clone(),
                }),
            }),
        ),
        (
            "During you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: ClauseAttachment::PreposedDuring(PreposedDuring {
                    condition: condition.clone(),
                    body: body.clone(),
                }),
            }),
        ),
        (
            "Until you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: ClauseAttachment::PreposedUntil(PreposedUntil { condition, body }),
            }),
        ),
    ] {
        let selected = assert_one_logic_candidate(&parser, &context, text);
        assert_eq!(
            selected,
            Ability::Plain(Plain {
                body: AbilityBody::Sentences(
                    Sentences::new(vec![expected]).expect("one conditional sentence is nonempty"),
                ),
            }),
            "the surface has one independently specified position-specific AST: {text}",
        );
    }

    for invalid in [
        "If you connive you gain 2 life.",
        "You gain 2 life, if you connive.",
        "You gain 2 life unless, you connive.",
        "if you connive, you gain 2 life.",
        "If you connive,  you gain 2 life.",
        "You gain 2 life if you connive,.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "conditional attachment punctuation, spacing, and sentence case are structural: {invalid}",
        );
    }
}

#[test]
fn ordered_then_and_reflexive_subordinates_are_linguistic_and_disjoint() {
    let parser = parser();
    let context = context("Context Card", false);
    let ordered_text = "You gain 1 life, then you connive, then a player gains 2 life.";
    let ordered = assert_one_logic_candidate(&parser, &context, ordered_text);
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(sentences),
    }) = ordered
    else {
        panic!("ordered clauses remain inside an ordinary linguistic body")
    };
    let [
        Sentence::Attached(Attached {
            attachment: ClauseAttachment::ThenSequence(sequence),
        }),
    ] = sentences.sentences()
    else {
        panic!("then stores an ordered finite-clause sequence rather than nested sentences")
    };
    assert_eq!(
        sequence
            .members()
            .iter()
            .map(|member| match member {
                Clause::Finite(member) => finite_clause_identity(member),
                Clause::Coordination(_) => panic!("then members retain their finite clause shape"),
            })
            .collect::<Vec<_>>(),
        ["you/gain:1", "you/connive", "player/gain:2"],
    );

    for (text, kind) in [
        (
            "You gain 1 life. If you do, you connive.",
            ReflexiveSubordinateKind::IfYouDo,
        ),
        (
            "You gain 1 life. When you do, you connive.",
            ReflexiveSubordinateKind::WhenYouDo,
        ),
    ] {
        let selected = assert_one_logic_candidate(&parser, &context, text);
        let Ability::Plain(Plain {
            body: AbilityBody::Sentences(sentences),
        }) = selected
        else {
            unreachable!()
        };
        let [
            Sentence::Declarative(_),
            Sentence::Attached(Attached {
                attachment: ClauseAttachment::ReflexiveSubordinate(subordinate),
            }),
        ] = sentences.sentences()
        else {
            panic!("the reflexive subordinate is its own sentence shape: {text}")
        };
        assert_eq!(
            subordinate.kind, kind,
            "the lexical subordinate kind is stored: {text}"
        );
        assert_eq!(subordinate.body, Clause::Finite(connive_clause()));
    }

    for invalid in [
        "You gain 1 life then you connive.",
        "You gain 1 life, then you connive, a player gains 2 life.",
        "You gain 1 life. If you do you connive.",
        "You gain 1 life. if you do, you connive.",
        "You gain 1 life. When you do,  you connive.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "ordered and reflexive boundaries cannot borrow punctuation: {invalid}",
        );
    }
}

#[test]
fn ordinary_trailing_if_is_not_trigger_intervening_if_and_keeps_its_own_bytes() {
    let parser = parser();
    let context = context("Context Card", false);
    let ordinary = "Whenever a player connives, you gain 2 life if you connive.";
    let intervening = "Whenever a player connives, if you connive, you gain 2 life.";

    let ordinary_ability = assert_one_logic_candidate(&parser, &context, ordinary);
    let Ability::Triggered(Triggered {
        intervening_if: None,
        body: AbilityBody::Sentences(ordinary_body),
        ..
    }) = ordinary_ability
    else {
        panic!("ordinary trailing if belongs to the triggered body")
    };
    assert!(matches!(
        ordinary_body.sentences(),
        [Sentence::Attached(Attached {
            attachment: ClauseAttachment::PostposedIf(_),
        })]
    ));

    let intervening_analysis = parser.analyze(intervening, &context);
    let intervening_decision = intervening_analysis
        .decision()
        .expect("intervening-if surface reaches the selection boundary");
    assert_eq!(intervening_decision.candidates().len(), 2);
    assert_eq!(intervening_decision.survivors(), [0]);
    assert_eq!(intervening_decision.selected(), Some(0));
    assert_eq!(
        intervening_decision.resolution(),
        SelectionResolution::Specificity
    );
    assert!(intervening_decision.exception_uses().is_empty());
    let intervening_ability = intervening_analysis
        .selected()
        .expect("intervening-if has one selected candidate")
        .clone();
    assert_eq!(
        intervening_ability.render(&context, parser.environment()),
        intervening
    );
    let Ability::Triggered(Triggered {
        intervening_if: Some(ConditionClause::FiniteCondition(_)),
        body: AbilityBody::Sentences(intervening_body),
        ..
    }) = intervening_ability
    else {
        panic!("intervening-if remains attached to the trigger envelope")
    };
    assert!(matches!(
        intervening_body.sentences(),
        [Sentence::Declarative(_)]
    ));

    for (text, expected_commas) in [
        (ordinary, &[(26, 27, "form:triggered/triggered/1")][..]),
        (
            intervening,
            &[
                (26, 27, "form:triggered/triggered/1"),
                (42, 43, "form:finite_condition/finite_condition/2"),
            ][..],
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let comma_claims = analysis
            .ownership()
            .expect("selected attachment has ownership")
            .parsed_claims()
            .iter()
            .filter(|claim| &text[claim.span().start..claim.span().end] == ",")
            .collect::<Vec<_>>();
        assert_eq!(
            comma_claims
                .iter()
                .map(|claim| (
                    claim.span().start,
                    claim.span().end,
                    claim.stable_owner_id()
                ))
                .collect::<Vec<_>>(),
            expected_commas,
            "comma ownership is unique to its attachment path: {text}",
        );
        assert!(
            comma_claims
                .iter()
                .all(|claim| claim.kind() == LexicalProvenanceKind::FormLiteral),
            "attachment commas are literal construction owners: {text}",
        );
    }
}

#[derive(Default)]
struct AttachmentVisitor(Vec<&'static str>);

impl Visitor for AttachmentVisitor {
    fn visit_sentence(&mut self, value: &Sentence) {
        self.0.push("Sentence");
        deckmaste_english_v2::visit::walk_sentence(self, value);
    }

    fn visit_attached(&mut self, value: &Attached) {
        self.0.push("Attached");
        deckmaste_english_v2::visit::walk_attached(self, value);
    }

    fn visit_clause_attachment(&mut self, value: &ClauseAttachment) {
        self.0.push("ClauseAttachment");
        deckmaste_english_v2::visit::walk_clause_attachment(self, value);
    }

    fn visit_preposed_if_predicate(&mut self, value: &PreposedIfPredicate) {
        self.0.push("PreposedIfPredicate");
        deckmaste_english_v2::visit::walk_preposed_if_predicate(self, value);
    }

    fn visit_postposed_if_predicate(&mut self, value: &PostposedIfPredicate) {
        self.0.push("PostposedIfPredicate");
        deckmaste_english_v2::visit::walk_postposed_if_predicate(self, value);
    }

    fn visit_then_sequence(&mut self, value: &ThenSequence) {
        self.0.push("ThenSequence");
        deckmaste_english_v2::visit::walk_then_sequence(self, value);
    }

    fn visit_reflexive_subordinate(&mut self, value: &ReflexiveSubordinate) {
        self.0.push("ReflexiveSubordinate");
        deckmaste_english_v2::visit::walk_reflexive_subordinate(self, value);
    }

    fn visit_reflexive_subordinate_kind(&mut self, _value: ReflexiveSubordinateKind) {
        self.0.push("ReflexiveSubordinateKind");
    }

    fn visit_clause(&mut self, value: &Clause) {
        self.0.push("Clause");
        deckmaste_english_v2::visit::walk_clause(self, value);
    }

    fn visit_finite_clause(&mut self, value: &FiniteClause) {
        self.0.push("FiniteClause");
        deckmaste_english_v2::visit::walk_finite_clause(self, value);
    }

    fn visit_predicate(&mut self, value: &Predicate) {
        self.0.push("Predicate");
        deckmaste_english_v2::visit::walk_predicate(self, value);
    }
}

#[derive(Default)]
struct AttachmentEnvelopeVisitor(Vec<&'static str>);

impl Visitor for AttachmentEnvelopeVisitor {
    fn visit_ability(&mut self, value: &Ability) {
        self.0.push("Ability");
        deckmaste_english_v2::visit::walk_ability(self, value);
    }

    fn visit_plain(&mut self, value: &Plain) {
        self.0.push("Plain");
        deckmaste_english_v2::visit::walk_plain(self, value);
    }

    fn visit_triggered(&mut self, value: &Triggered) {
        self.0.push("Triggered");
        deckmaste_english_v2::visit::walk_triggered(self, value);
    }

    fn visit_activated(&mut self, value: &Activated) {
        self.0.push("Activated");
        deckmaste_english_v2::visit::walk_activated(self, value);
    }

    fn visit_trigger_prefix(&mut self, value: &TriggerPrefix) {
        self.0.push("TriggerPrefix");
        deckmaste_english_v2::visit::walk_trigger_prefix(self, value);
    }

    fn visit_finite(&mut self, value: &Finite) {
        self.0.push("Finite");
        deckmaste_english_v2::visit::walk_finite(self, value);
    }

    fn visit_trigger_marker(&mut self, value: TriggerMarker) {
        match value {
            TriggerMarker::Whenever => self.0.push("TriggerMarker:Whenever"),
            TriggerMarker::When => self.0.push("TriggerMarker:When"),
        }
    }

    fn visit_activation_cost_component(&mut self, value: &ActivationCostComponent) {
        self.0.push("ActivationCostComponent");
        deckmaste_english_v2::visit::walk_activation_cost_component(self, value);
    }

    fn visit_symbol_run(&mut self, value: &SymbolRun) {
        self.0.push("SymbolRun");
        deckmaste_english_v2::visit::walk_symbol_run(self, value);
    }

    fn visit_cost_symbol(&mut self, value: &CostSymbol) {
        self.0.push("CostSymbol");
        deckmaste_english_v2::visit::walk_cost_symbol(self, value);
    }

    fn visit_fixed_symbol(&mut self, value: &FixedSymbol) {
        self.0.push("FixedSymbol");
        deckmaste_english_v2::visit::walk_fixed_symbol(self, value);
    }

    fn visit_fixed_cost_symbol(&mut self, value: FixedCostSymbol) {
        match value {
            FixedCostSymbol::Tap => self.0.push("FixedCostSymbol:Tap"),
            other => panic!("unexpected activation witness symbol: {other:?}"),
        }
    }

    fn visit_ability_body(&mut self, value: &AbilityBody) {
        self.0.push("AbilityBody");
        deckmaste_english_v2::visit::walk_ability_body(self, value);
    }

    fn visit_sentences(&mut self, value: &Sentences) {
        self.0.push("Sentences");
        deckmaste_english_v2::visit::walk_sentences(self, value);
    }

    fn visit_sentence(&mut self, value: &Sentence) {
        self.0.push("Sentence");
        deckmaste_english_v2::visit::walk_sentence(self, value);
    }

    fn visit_attached(&mut self, value: &Attached) {
        self.0.push("Attached");
        deckmaste_english_v2::visit::walk_attached(self, value);
    }

    fn visit_clause_attachment(&mut self, value: &ClauseAttachment) {
        self.0.push("ClauseAttachment");
        deckmaste_english_v2::visit::walk_clause_attachment(self, value);
    }

    fn visit_preposed_if_predicate(&mut self, value: &PreposedIfPredicate) {
        self.0.push("PreposedIfPredicate");
        deckmaste_english_v2::visit::walk_preposed_if_predicate(self, value);
    }

    fn visit_postposed_if_predicate(&mut self, value: &PostposedIfPredicate) {
        self.0.push("PostposedIfPredicate");
        deckmaste_english_v2::visit::walk_postposed_if_predicate(self, value);
    }

    fn visit_finite_clause(&mut self, value: &FiniteClause) {
        self.0.push("FiniteClause");
        deckmaste_english_v2::visit::walk_finite_clause(self, value);
    }

    fn visit_subject(&mut self, value: &Subject) {
        match value {
            Subject::SubjectPronoun(PersonalSubject {
                word: SubjectPronoun::You,
            }) => self.0.push("Subject:You"),
            Subject::SubjectNominal(NominalSubject { value }) => {
                match unqualified_reference(value) {
                    UnqualifiedReference::IndefiniteReference(IndefiniteReference {
                        nominal:
                            SingularNominal::BareSingularNominal(BareSingularNominal {
                                head:
                                    SingularHead::CommonSingularHead(CommonSingularHead {
                                        noun: CommonNoun::Player,
                                    }),
                            }),
                    }) => self.0.push("Subject:Player"),
                    other => panic!("unexpected trigger subject payload: {other:?}"),
                }
            }
            other @ Subject::SubjectPronoun(_) => {
                panic!("unexpected attachment subject payload: {other:?}")
            }
        }
    }

    fn visit_predicate(&mut self, value: &Predicate) {
        match value {
            Predicate::Atomic(VerbPhrase::Connive(_)) => self.0.push("Predicate:Connive"),
            Predicate::Atomic(VerbPhrase::GainLife(GainLife {
                amount:
                    Amount::Number(NumberAmount {
                        number: ScalarNumber { magnitude: 2 },
                    }),
            })) => self.0.push("Predicate:Gain2"),
            other => panic!("unexpected attachment predicate payload: {other:?}"),
        }
    }
}

fn literal_claims(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    literals: &[&str],
) -> Vec<(usize, usize, String)> {
    parser
        .analyze(text, context)
        .ownership()
        .expect("selected attachment has ownership")
        .parsed_claims()
        .iter()
        .filter(|claim| {
            literals.contains(&&text[claim.span().start..claim.span().end])
                && claim.kind() == LexicalProvenanceKind::FormLiteral
        })
        .map(|claim| {
            (
                claim.span().start,
                claim.span().end,
                claim.stable_owner_id().to_owned(),
            )
        })
        .collect()
}

fn assert_attachment_parse_rejected(parser: &Parser, context: &ParseContext<'_>, text: &str) {
    assert!(
        parser.parse(text, context).is_err(),
        "attachment punctuation, spacing, capitalization, and order stay local: {text}",
    );
}

fn assert_attachment_has_no_selection(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    expected_span: TextSpan,
) {
    let analysis = parser.analyze(text, context);
    assert!(
        analysis.selected().is_none(),
        "the nonrecursive intermediate stage cannot select a composed/reordered attachment: {text}",
    );
    let error = parser
        .parse(text, context)
        .expect_err("the composed/reordered attachment remains a parse failure");
    let ParseError::Failure { span, .. } = error else {
        panic!("the attachment negative must be an ordinary parse failure: {text}")
    };
    assert_eq!(
        span, expected_span,
        "the rejection boundary is exact: {text}"
    );
}

#[test]
fn conditional_attachment_root_scope_matrix_is_exact() {
    let parser = parser();
    let context = context("Context Card", false);
    let condition = connive_clause();
    let gain = Predicate::Atomic(gain_life_predicate(2));
    let root_text = "If you connive, gain 2 life.";
    let root = assert_one_logic_candidate(&parser, &context, root_text);

    assert_eq!(
        root,
        Ability::Plain(Plain {
            body: AbilityBody::Sentences(
                Sentences::new(vec![Sentence::Attached(Attached {
                    attachment: ClauseAttachment::PreposedIfPredicate(PreposedIfPredicate {
                        condition: condition.clone(),
                        body: gain.clone(),
                    }),
                })])
                .expect("one root sentence"),
            ),
        }),
    );

    assert_eq!(
        literal_claims(&parser, &context, root_text, &[",", ": "]),
        vec![(
            14,
            15,
            "form:preposed_if_predicate/preposed_if_predicate/2".to_owned(),
        )],
        "the root attachment comma has an exact owner",
    );

    let mut visitor = AttachmentEnvelopeVisitor::default();
    visitor.visit_ability(&root);
    assert_eq!(
        visitor.0,
        [
            "Ability",
            "Plain",
            "AbilityBody",
            "Sentences",
            "Sentence",
            "Attached",
            "ClauseAttachment",
            "PreposedIfPredicate",
            "FiniteClause",
            "Subject:You",
            "Predicate:Connive",
            "Predicate:Gain2",
        ],
        "the condition is visited before the attached imperative predicate",
    );

    assert_attachment_parse_rejected(&parser, &context, "If you connive gain 2 life.");
    assert_attachment_parse_rejected(&parser, &context, "If you connive,  gain 2 life.");
    assert_attachment_parse_rejected(&parser, &context, "if you connive, gain 2 life.");
    assert_attachment_parse_rejected(&parser, &context, "Gain 2 life if you connive,.");
}

#[test]
fn conditional_attachment_trigger_scope_matrix_is_exact() {
    let parser = parser();
    let context = context("Context Card", false);
    let condition = connive_clause();
    let gain = Predicate::Atomic(gain_life_predicate(2));
    let trigger_text = "Whenever a player connives, gain 2 life if you connive.";
    let trigger = assert_one_logic_candidate(&parser, &context, trigger_text);

    assert_eq!(
        trigger,
        Ability::Triggered(Triggered {
            trigger: TriggerPrefix::Finite(Finite {
                marker: TriggerMarker::Whenever,
                clause: player_connive_clause(),
            }),
            intervening_if: None,
            body: AbilityBody::Sentences(
                Sentences::new(vec![Sentence::Attached(Attached {
                    attachment: ClauseAttachment::PostposedIfPredicate(PostposedIfPredicate {
                        body: gain,
                        condition,
                    }),
                })])
                .expect("one trigger-body sentence"),
            ),
        }),
        "the complete trigger envelope retains its prefix, absent intervening condition, and body",
    );
    assert_eq!(
        literal_claims(&parser, &context, trigger_text, &[",", ": "]),
        vec![(26, 27, "form:triggered/triggered/1".to_owned())],
        "the trigger-body attachment comma has an exact owner",
    );

    let mut trigger_visitor = AttachmentEnvelopeVisitor::default();
    trigger_visitor.visit_ability(&trigger);
    assert_eq!(
        trigger_visitor.0,
        [
            "Ability",
            "Triggered",
            "TriggerPrefix",
            "Finite",
            "TriggerMarker:Whenever",
            "FiniteClause",
            "Subject:Player",
            "Predicate:Connive",
            "AbilityBody",
            "Sentences",
            "Sentence",
            "Attached",
            "ClauseAttachment",
            "PostposedIfPredicate",
            "Predicate:Gain2",
            "FiniteClause",
            "Subject:You",
            "Predicate:Connive",
        ],
        "the trigger envelope delegates prefix before its attached body payload",
    );

    assert_attachment_parse_rejected(
        &parser,
        &context,
        "Whenever a player connives, Gain 2 life if you connive.",
    );
    assert_attachment_parse_rejected(
        &parser,
        &context,
        "Whenever a player connives, gain 2 life if you connive,.",
    );
}

#[test]
fn conditional_attachment_activation_scope_matrix_is_exact() {
    let parser = parser();
    let context = context("Context Card", false);
    let condition = connive_clause();
    let gain = Predicate::Atomic(gain_life_predicate(2));
    let activation_text = "{T}: If you connive, gain 2 life.";
    let activation = assert_one_logic_candidate(&parser, &context, activation_text);
    let Ability::Activated(activated) = &activation else {
        panic!("the post-colon surface has the activated envelope")
    };

    assert_eq!(
        activated.costs(),
        &[tap_cost()],
        "the complete activation envelope retains its typed tap cost",
    );
    assert_eq!(
        activated.body,
        AbilityBody::Sentences(
            Sentences::new(vec![Sentence::Attached(Attached {
                attachment: ClauseAttachment::PreposedIfPredicate(PreposedIfPredicate {
                    condition,
                    body: gain,
                }),
            })])
            .expect("one post-colon sentence"),
        ),
        "the complete activation envelope retains its attachment body",
    );
    assert_eq!(
        literal_claims(&parser, &context, activation_text, &[",", ": "]),
        vec![
            (3, 5, "form:activated/activated/1".to_owned()),
            (
                19,
                20,
                "form:preposed_if_predicate/preposed_if_predicate/2".to_owned(),
            ),
        ],
        "the post-colon attachment punctuation has exact owners",
    );

    let mut activation_visitor = AttachmentEnvelopeVisitor::default();
    activation_visitor.visit_ability(&activation);
    assert_eq!(
        activation_visitor.0,
        [
            "Ability",
            "Activated",
            "ActivationCostComponent",
            "SymbolRun",
            "CostSymbol",
            "FixedSymbol",
            "FixedCostSymbol:Tap",
            "AbilityBody",
            "Sentences",
            "Sentence",
            "Attached",
            "ClauseAttachment",
            "PreposedIfPredicate",
            "FiniteClause",
            "Subject:You",
            "Predicate:Connive",
            "Predicate:Gain2",
        ],
        "the activation envelope delegates cost before its post-colon attached body",
    );

    assert_attachment_parse_rejected(&parser, &context, "{T}: If you connive gain 2 life.");
    assert_attachment_parse_rejected(&parser, &context, "{T}:  If you connive, gain 2 life.");
    assert_attachment_parse_rejected(&parser, &context, "{T}: if you connive, gain 2 life.");
}

#[test]
fn conditional_attachment_rejects_composed_and_reordered_surfaces() {
    let parser = parser();
    let context = context("Context Card", false);
    assert_attachment_has_no_selection(
        &parser,
        &context,
        "If you connive, gain 2 life if you connive.",
        TextSpan { start: 28, end: 30 },
    );
    assert_attachment_has_no_selection(
        &parser,
        &context,
        "Gain 2 life if you connive unless you connive.",
        TextSpan { start: 27, end: 33 },
    );
    assert_attachment_has_no_selection(
        &parser,
        &context,
        "Gain 2 life, if you connive.",
        TextSpan { start: 13, end: 15 },
    );
    assert_attachment_has_no_selection(
        &parser,
        &context,
        "{T}: If you connive, gain 2 life if you connive.",
        TextSpan { start: 33, end: 35 },
    );
}

#[test]
fn conditional_and_reflexive_attachments_preserve_exact_self_reference_licensing() {
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
        for text in [
            format!("If {surface} connives, {surface} gains 2 life."),
            format!("{surface} gains 1 life. If you do, {surface} gains 2 life."),
        ] {
            let selected = assert_one_logic_candidate(&parser, &context, &text);
            let mut visitor = SelfReferenceVisitor::default();
            visitor.visit_ability(&selected);
            assert_eq!(visitor.spellings, [spelling, spelling], "{text}");
        }
    }

    let ordinary = context("Grizzly Bears", false);
    for text in [
        "If Grizzly connives, Grizzly gains 2 life.",
        "Grizzly gains 1 life. If you do, Grizzly gains 2 life.",
    ] {
        assert!(
            parser.parse(text, &ordinary).is_err(),
            "neither attachment licenses a nonlegendary abbreviation: {text}",
        );
    }
}

#[test]
fn predicate_attachments_are_staged_without_recursive_clause_bracketings() {
    let parser = parser();
    let context = context("Context Card", false);
    let condition = connive_clause();
    let gain = Predicate::Atomic(gain_life_predicate(2));

    for (text, expected) in [
        (
            "Gain 2 life if you connive.",
            ClauseAttachment::PostposedIfPredicate(PostposedIfPredicate {
                body: gain.clone(),
                condition: condition.clone(),
            }),
        ),
        (
            "Gain 2 life unless you connive.",
            ClauseAttachment::PostposedUnlessPredicate(PostposedUnlessPredicate {
                body: gain.clone(),
                condition: condition.clone(),
            }),
        ),
        (
            "As long as you connive, gain 2 life.",
            ClauseAttachment::PreposedAsLongAsPredicate(PreposedAsLongAsPredicate {
                condition: condition.clone(),
                body: gain.clone(),
            }),
        ),
        (
            "While you connive, gain 2 life.",
            ClauseAttachment::PreposedWhilePredicate(PreposedWhilePredicate {
                condition: condition.clone(),
                body: gain.clone(),
            }),
        ),
        (
            "During you connive, gain 2 life.",
            ClauseAttachment::PreposedDuringPredicate(PreposedDuringPredicate {
                condition: condition.clone(),
                body: gain.clone(),
            }),
        ),
        (
            "Until you connive, gain 2 life.",
            ClauseAttachment::PreposedUntilPredicate(PreposedUntilPredicate {
                condition: condition.clone(),
                body: gain.clone(),
            }),
        ),
    ] {
        let selected = assert_one_logic_candidate(&parser, &context, text);
        let Ability::Plain(Plain {
            body: AbilityBody::Sentences(sentences),
        }) = selected
        else {
            unreachable!()
        };
        assert_eq!(
            sentences.sentences(),
            [Sentence::Attached(Attached {
                attachment: expected
            })],
            "each predicate attachment inhabits the single intermediate stage: {text}",
        );
    }

    let ordered = assert_one_logic_candidate(&parser, &context, "Gain 1 life, then connive.");
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(ordered_sentences),
    }) = ordered
    else {
        unreachable!()
    };
    let [
        Sentence::Attached(Attached {
            attachment: ClauseAttachment::ThenPredicateSequence(sequence),
        }),
    ] = ordered_sentences.sentences()
    else {
        panic!("predicate ordering stores its members in the attachment stage")
    };
    assert_eq!(
        sequence.members(),
        [
            Predicate::Atomic(gain_life_predicate(1)),
            Predicate::Atomic(VerbPhrase::Connive(Connive {})),
        ],
    );

    let reflexive = assert_one_logic_candidate(
        &parser,
        &context,
        "You gain 1 life. When you do, gain 2 life.",
    );
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(reflexive_sentences),
    }) = reflexive
    else {
        unreachable!()
    };
    let [
        _,
        Sentence::Attached(Attached {
            attachment: ClauseAttachment::ReflexivePredicateSubordinate(subordinate),
        }),
    ] = reflexive_sentences.sentences()
    else {
        panic!("predicate reflexive subordinate stores its body in the attachment stage")
    };
    assert_eq!(subordinate.kind, ReflexiveSubordinateKind::WhenYouDo);
    assert_eq!(subordinate.body, gain);
}

#[test]
fn attachment_visitors_follow_clause_order_and_envelopes_preserve_case_and_names() {
    let parser = parser();
    let ordinary = context("Context Card", false);
    for text in [
        "If you connive, you gain 2 life.",
        "Whenever a player connives, you gain 2 life if you connive.",
        "{T}: If you connive, you gain 2 life.",
    ] {
        assert_one_logic_candidate(&parser, &ordinary, text);
    }
    for invalid in [
        "{T}: if you connive, you gain 2 life.",
        "If you connive, you gain 2 life. if you connive, you gain 2 life.",
    ] {
        assert!(parser.parse(invalid, &ordinary).is_err(), "{invalid}");
    }

    let ordered = assert_one_logic_candidate(
        &parser,
        &ordinary,
        "You gain 1 life, then you connive, then a player gains 2 life.",
    );
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(sentences),
    }) = ordered
    else {
        unreachable!()
    };
    let [
        Sentence::Attached(Attached {
            attachment: ClauseAttachment::ThenSequence(sequence),
        }),
    ] = sentences.sentences()
    else {
        unreachable!()
    };
    let mut visitor = AttachmentVisitor::default();
    visitor.visit_then_sequence(sequence);
    assert_eq!(
        visitor.0,
        [
            "ThenSequence",
            "Clause",
            "FiniteClause",
            "Predicate",
            "Clause",
            "FiniteClause",
            "Predicate",
            "Clause",
            "FiniteClause",
            "Predicate",
        ],
    );

    let reflexive = assert_one_logic_candidate(
        &parser,
        &ordinary,
        "You gain 1 life. When you do, you connive.",
    );
    let Ability::Plain(Plain {
        body: AbilityBody::Sentences(sentences),
    }) = reflexive
    else {
        unreachable!()
    };
    let [
        _,
        Sentence::Attached(Attached {
            attachment: ClauseAttachment::ReflexiveSubordinate(subordinate),
        }),
    ] = sentences.sentences()
    else {
        unreachable!()
    };
    let mut visitor = AttachmentVisitor::default();
    visitor.visit_reflexive_subordinate(subordinate);
    assert_eq!(
        visitor.0,
        [
            "ReflexiveSubordinate",
            "ReflexiveSubordinateKind",
            "Clause",
            "FiniteClause",
            "Predicate",
        ],
    );

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
        let selected = assert_one_logic_candidate(
            &parser,
            &context,
            &format!("If {surface} connives, {surface} gains 2 life."),
        );
        let mut visitor = SelfReferenceVisitor::default();
        visitor.visit_ability(&selected);
        assert_eq!(visitor.spellings, [spelling, spelling], "{name}");
    }
    let ordinary_name = context("Grizzly Bears", false);
    assert!(
        parser
            .parse("If Grizzly connives, Grizzly gains 2 life.", &ordinary_name)
            .is_err(),
        "conditional grammar cannot license an ordinary-name abbreviation",
    );
}

#[test]
fn generated_logic_report_has_only_semantic_members_and_positional_tables() {
    let source = include_str!("../src/constructions.rs");
    let invocation = deckmaste_construction_core::invocation_from_source(source)
        .expect("production construction invocation is authentic");
    let expansion = deckmaste_construction_core::generate(invocation.tokens)
        .expect("production construction invocation expands");
    let report = expansion.escape_hatches();
    assert!(report.stored_separator_fields().is_empty());
    assert!(report.stored_form_tags().is_empty());
    for role in [
        "AndPredicateCoordination.members",
        "OrPredicateCoordination.members",
        "AndOrPredicateCoordination.members",
        "AndClauseCoordination.members",
        "OrClauseCoordination.members",
        "AndOrClauseCoordination.members",
        "ThenSequence.members",
        "ThenPredicateSequence.members",
    ] {
        assert!(
            report
                .positional_separator_tables()
                .iter()
                .any(|actual| actual == role),
            "generated report contains positional separator authority for {role}",
        );
    }
    assert_eq!(
        report.sequence_feature_roles(),
        [
            "ThenPredicateSequence.members.agreement",
            "AndPredicateCoordination.members.agreement",
            "OrPredicateCoordination.members.agreement",
            "AndOrPredicateCoordination.members.agreement",
        ],
        "only predicate coordination relays homogeneous agreement; finite clauses retain independent subjects",
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
