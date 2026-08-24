use std::path::Path;

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
    for rejected in [
        AtPhraseValue::new(AtBoundary::End, None, TurnPart::Turn, None),
        AtPhraseValue::new(
            AtBoundary::Beginning,
            Some(TurnSpecifier::EachOfYour),
            TurnPart::Upkeep,
            None,
        ),
        AtPhraseValue::new(
            AtBoundary::Beginning,
            None,
            TurnPart::DrawStep,
            Some(TurnOwnerPostmodifier::YourTurn),
        ),
    ] {
        assert!(
            rejected.is_none(),
            "the checked public product rejects a forbidden temporal cross-product",
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
