use std::collections::BTreeSet;
use std::num::NonZeroU32;
use std::path::Path;
use std::process::Command;

use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::LexicalProvenanceKind;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::parser::TextSpan;
use deckmaste_english_v2::render::Render as _;
use deckmaste_english_v2::visit::Visitor;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::Onset;

fn environment() -> ParserEnvironment {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    ParserEnvironment::try_from_parts(
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
    .expect("builtin-v2 declarations and catalog provider freeze")
}

fn parser() -> Parser {
    Parser::new(environment()).expect("required declarations are present")
}

fn plain_sentences(ability: &Ability) -> &Sentences {
    let Ability::Plain(Plain { body }) = ability else {
        panic!("expected a plain ability: {ability:?}")
    };
    let AbilityBody::Sentences(sentences) = body.as_ref() else {
        panic!("expected a sentence body: {ability:?}")
    };
    sentences
}

fn one_declarative_clause(ability: &Ability) -> &Clause {
    let [Sentence::Declarative(declarative)] = plain_sentences(ability).sentences() else {
        panic!("expected one declarative sentence: {ability:?}")
    };
    declarative.clause.as_ref()
}

fn context(card_name: &str, is_legendary: bool) -> ParseContext<'_> {
    ParseContext::new(card_name, is_legendary, Onset::Consonant)
        .expect("test card name is a valid parse context")
}

fn connive() -> VerbPhrase {
    let environment = environment();
    let head = DeclarationIntransitiveVerb::new(
        &environment,
        DeclarationId::new(DeclarationKind::KeywordAction, "Connive"),
    )
    .expect("the builtin grammar declares intransitive Connive");
    VerbPhrase::IntransitivePredicate(IntransitivePredicate {
        head: IntransitiveVerb::Declaration(head),
    })
}

fn declared_action_name(predicate: &VerbPhrase) -> Option<&str> {
    match predicate {
        VerbPhrase::IntransitivePredicate(IntransitivePredicate {
            head: IntransitiveVerb::Declaration(head),
        }) => Some(head.id().name()),
        VerbPhrase::TransitivePredicate(TransitivePredicate {
            head: TransitiveVerb::Declaration(head),
            ..
        }) => Some(head.id().name()),
        VerbPhrase::NumerativePredicate(NumerativePredicate {
            head: NumerativeVerb::Declaration(head),
            ..
        }) => Some(head.id().name()),
        _ => None,
    }
}

#[test]
fn linguistic_bodies_enclose_plain_and_triggered_sentence_sequences() {
    let parser = parser();
    let plain_context = context("Context Card", false);
    let plain_text = "Destroy target creature.";
    let plain = parser
        .parse(plain_text, &plain_context)
        .expect("the ordinary sentence sequence parses as a plain ability");

    let Ability::Plain(Plain { body }) = plain else {
        panic!("plain sentence sequence has the linguistic ability-body shape")
    };
    let AbilityBody::Sentences(plain_sentences) = *body else {
        panic!("plain ability has a sentence sequence")
    };
    assert_eq!(plain_sentences.sentences().len(), 1);

    let triggered_text = "Whenever a player connives, you gain X life.";
    let triggered = parser
        .parse(triggered_text, &plain_context)
        .expect("the finite triggered sentence parses");
    let Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(finite),
        intervening_if,
        body,
    }) = triggered
    else {
        panic!("triggered ability stores its finite trigger and linguistic body")
    };
    assert!(intervening_if.as_ref().is_none());
    let AbilityBody::Sentences(consequences) = *body else {
        panic!("triggered ability has a sentence sequence")
    };
    assert_eq!(finite.marker, TriggerMarker::Whenever);
    assert!(matches!(
        finite.clause.as_ref(),
        Clause::Finite(finite) if matches!(finite.as_ref(), FiniteClause::PlainFiniteClause(_))
    ));
    assert_eq!(consequences.sentences().len(), 1);
}

#[derive(Default)]
struct SubjectStructureVisitor(Vec<&'static str>);

impl Visitor for SubjectStructureVisitor {
    fn visit_indefinite_coordination_reference(&mut self, value: &IndefiniteCoordinationReference) {
        self.0.push("IndefiniteCoordinationReference");
        deckmaste_english_v2::visit::walk_indefinite_coordination_reference(self, value);
    }

    fn visit_full_or_noun_phrase_coordination(&mut self, value: &FullOrNounPhraseCoordination) {
        self.0.push("FullOrNounPhraseCoordination");
        deckmaste_english_v2::visit::walk_full_or_noun_phrase_coordination(self, value);
    }

    fn visit_determiner_scoped_nominal_coordination(
        &mut self,
        value: &DeterminerScopedNominalCoordination,
    ) {
        self.0.push("DeterminerScopedNominalCoordination");
        deckmaste_english_v2::visit::walk_determiner_scoped_nominal_coordination(self, value);
    }

    fn visit_determiner_scoped_or_nominal_pair(&mut self, value: &DeterminerScopedOrNominalPair) {
        self.0.push("DeterminerScopedOrNominalPair");
        deckmaste_english_v2::visit::walk_determiner_scoped_or_nominal_pair(self, value);
    }

    fn visit_this_determiner_phrase(&mut self, value: &ThisDeterminerPhrase) {
        self.0.push("ThisDeterminerPhrase");
        deckmaste_english_v2::visit::walk_this_determiner_phrase(self, value);
    }

    fn visit_another_determiner_phrase(&mut self, value: &AnotherDeterminerPhrase) {
        self.0.push("AnotherDeterminerPhrase");
        deckmaste_english_v2::visit::walk_another_determiner_phrase(self, value);
    }

    fn visit_another_coordination_reference(&mut self, value: &AnotherCoordinationReference) {
        self.0.push("AnotherCoordinationReference");
        deckmaste_english_v2::visit::walk_another_coordination_reference(self, value);
    }

    fn visit_other_than_qualified_reference(&mut self, value: &OtherThanQualifiedReference) {
        self.0.push("OtherThanQualifiedReference");
        deckmaste_english_v2::visit::walk_other_than_qualified_reference(self, value);
    }

    fn visit_demonstrative_possessive_reference(
        &mut self,
        value: &DemonstrativePossessiveReference,
    ) {
        self.0.push("DemonstrativePossessiveReference");
        deckmaste_english_v2::visit::walk_demonstrative_possessive_reference(self, value);
    }
}

#[test]
fn finite_subject_coordination_and_exclusion_are_linguistic_structure() {
    let parser = parser();
    let context = context("Context Card", false);
    let witnesses = [
        (
            "Whenever a spell or ability deals 1 damage to you, you gain 1 life.",
            "UnqualifiedReferenceIndefiniteCoordinationReference",
            &[
                "IndefiniteCoordinationReference",
                "DeterminerScopedNominalCoordination",
                "DeterminerScopedOrNominalPair",
            ][..],
            &[
                "form:indefinite_coordination_reference/a/0",
                "form:determiner_scoped_or_nominal_pair/determiner_scoped_or_nominal_pair/1",
            ][..],
        ),
        (
            "Whenever this creature or another creature you control deals 1 damage to you, you gain 1 life.",
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            &[
                "FullOrNounPhraseCoordination",
                "ThisDeterminerPhrase",
                "AnotherDeterminerPhrase",
            ][..],
            &[
                "structural:FullOrNounPhraseCoordination/members/separator/pair/0",
                "form:this_determiner_phrase/this_determiner_phrase/0",
                "form:another_determiner_phrase/another_determiner_phrase/0",
            ][..],
        ),
        (
            "Whenever an artifact or enchantment deals 1 damage to you, you gain 1 life.",
            "UnqualifiedReferenceIndefiniteCoordinationReference",
            &[
                "IndefiniteCoordinationReference",
                "DeterminerScopedNominalCoordination",
                "DeterminerScopedOrNominalPair",
            ][..],
            &[
                "form:indefinite_coordination_reference/an/0",
                "form:determiner_scoped_or_nominal_pair/determiner_scoped_or_nominal_pair/1",
            ][..],
        ),
        (
            "Whenever this creature or another creature deals 1 damage to you, you gain 1 life.",
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            &[
                "FullOrNounPhraseCoordination",
                "ThisDeterminerPhrase",
                "AnotherDeterminerPhrase",
            ][..],
            &["structural:FullOrNounPhraseCoordination/members/separator/pair/0"][..],
        ),
        (
            "Whenever this creature or another Warrior deals 1 damage to you, you gain 1 life.",
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            &[
                "FullOrNounPhraseCoordination",
                "ThisDeterminerPhrase",
                "AnotherDeterminerPhrase",
            ][..],
            &["structural:FullOrNounPhraseCoordination/members/separator/pair/0"][..],
        ),
        (
            "Whenever another Villain and/or artifact deals 1 damage to you, you gain 1 life.",
            "UnqualifiedReferenceAnotherCoordinationReference",
            &["AnotherCoordinationReference"][..],
            &[
                "form:another_coordination_reference/another_coordination_reference/0",
                "structural:SingularAndOrNominalCoordination/members/separator/pair/0",
            ][..],
        ),
        (
            "Whenever a player other than this creature's owner deals 1 damage to you, you gain 1 life.",
            "ControllerStageOtherThanQualifiedReference",
            &[
                "OtherThanQualifiedReference",
                "DemonstrativePossessiveReference",
            ][..],
            &[
                "form:other_than_qualified_reference/other_than_qualified_reference/1",
                "form:other_than_qualified_reference/other_than_qualified_reference/2",
                "form:demonstrative_possessive_reference/demonstrative_possessive_reference/1/affix",
            ][..],
        ),
    ];

    for (text, subject_path, visitor_events, subject_claims) in witnesses {
        let analysis = parser.analyze(text, &context);
        assert_eq!(
            analysis.outcome(),
            deckmaste_english_v2::parser::ParseAnalysisOutcome::Selected,
            "controlled supported-predicate witness must select: {text}: {:?}",
            parser.parse(text, &context),
        );
        let decision = analysis
            .decision()
            .expect("controlled supported-predicate witness has a selection decision");
        assert_eq!(decision.resolution(), SelectionResolution::Unique, "{text}");
        assert!(
            decision.candidates()[0]
                .construction_path()
                .iter()
                .any(|construction| construction == subject_path),
            "controlled witness has its exact subject construction: {text}",
        );
        assert!(decision.exception_uses().is_empty(), "{text}");
        let ownership = analysis
            .ownership()
            .expect("controlled supported-predicate witness owns its bytes");
        assert!(ownership.failures().is_empty(), "{text}: {ownership:?}");
        assert!(ownership.summary().covered(), "{text}: {ownership:?}");
        assert_eq!(ownership.rendered_text(), text, "{text}");
        let claim_ids = ownership
            .parsed_claims()
            .iter()
            .map(deckmaste_english_v2::parser::LexicalClaim::stable_owner_id)
            .collect::<BTreeSet<_>>();
        assert!(
            subject_claims.iter().all(|owner| claim_ids.contains(owner)),
            "controlled witness has exact subject claims: {text}",
        );
        let mut visitor = SubjectStructureVisitor::default();
        visitor.visit_ability(
            analysis
                .selected()
                .expect("controlled witness selected AST"),
        );
        assert_eq!(visitor.0, visitor_events, "{text}");
    }

    let mutation =
        "Whenever this creature nor another creature deals 1 damage to you, you gain 1 life.";
    let ParseError::Failure { span, .. } = parser
        .parse(mutation, &context)
        .expect_err("an unsupported coordinator remains outside the grammar")
    else {
        panic!("unsupported coordinator has an ordinary parse failure")
    };
    assert_eq!(&mutation[span.start..span.end], "nor");
}

#[test]
fn indefinite_coordination_articles_follow_the_realized_head_onset() {
    let parser = parser();
    let context = context("Context Card", false);

    for text in [
        "Whenever a spell or ability deals 1 damage to you, you gain 1 life.",
        "Whenever an artifact or enchantment deals 1 damage to you, you gain 1 life.",
    ] {
        let analysis = parser.analyze(text, &context);
        assert_eq!(
            analysis.outcome(),
            deckmaste_english_v2::parser::ParseAnalysisOutcome::Selected,
            "derived indefinite article must accept its matching head onset: {text}: {:?}",
            parser.parse(text, &context),
        );
        let decision = analysis
            .decision()
            .expect("matching indefinite coordination has a selection decision");
        assert_eq!(decision.resolution(), SelectionResolution::Unique, "{text}");
        let ownership = analysis
            .ownership()
            .expect("matching indefinite coordination owns its bytes");
        assert!(ownership.failures().is_empty(), "{text}: {ownership:?}");
        assert!(ownership.summary().covered(), "{text}: {ownership:?}");
        assert_eq!(ownership.rendered_text(), text, "{text}");
    }

    for text in [
        "Whenever an spell or ability deals 1 damage to you, you gain 1 life.",
        "Whenever a artifact or enchantment deals 1 damage to you, you gain 1 life.",
    ] {
        assert_eq!(
            parser.analyze(text, &context).outcome(),
            deckmaste_english_v2::parser::ParseAnalysisOutcome::ParseFailure,
            "indefinite article must reject the reciprocal head onset: {text}",
        );
    }
}

#[test]
fn existential_there_preserves_its_pivot_before_the_plan09_predicate_boundary() {
    let parser = parser();
    let context = context("Mask of Intolerance", false);
    let controlled = "At the beginning of each player's upkeep, if there are four or more basic land types among lands that player controls, you gain 1 life.";

    let parsed = parser
        .parse(controlled, &context)
        .expect("the preserved existential condition reaches the supported consequence");
    assert_eq!(parsed.render(&context, parser.environment()), controlled);
    let Ability::Triggered(Triggered { intervening_if, .. }) = &parsed else {
        panic!("the intervening condition owns an existential clause, pivot, and among-domain")
    };
    let Some(ConditionClause::ExistentialCondition(ExistentialCondition::ExistentialCondition(
        condition,
    ))) = intervening_if.as_ref().as_ref()
    else {
        panic!("the intervening condition owns an existential clause")
    };
    let ExistentialClause::PluralExistentialClause(existential) = &condition.clause else {
        panic!("the intervening condition owns a plural existential clause")
    };
    assert!(matches!(
        existential.pivot(),
        NounPhrase::QualifiedNounPhrase(_)
    ));

    let analysis = parser.analyze(controlled, &context);
    let decision = analysis
        .decision()
        .expect("the controlled existential witness has a selection decision");
    assert_eq!(decision.resolution(), SelectionResolution::Unique);
    assert!(decision.exception_uses().is_empty());
    let ownership = analysis
        .ownership()
        .expect("the controlled existential witness owns its bytes");
    assert!(ownership.failures().is_empty(), "{ownership:?}");
    assert!(ownership.summary().covered(), "{ownership:?}");
    assert_eq!(ownership.rendered_text(), controlled);

    let construction_path = decision.candidates()[0].construction_path();
    for required in [
        "ExistentialConditionExistentialCondition",
        "ExistentialClausePluralExistentialClause",
        "UnqualifiedReferenceCountComparisonReference",
        "PluralNominalCompoundModifiedPluralNominal",
        "AmongPhraseAmongPhrase",
        "ControllerOwnerQualificationDemonstrativeControls",
        "DeterminerPhraseThatDeterminerPhrase",
    ] {
        assert!(
            construction_path.iter().any(|actual| actual == required),
            "missing {required}: {construction_path:?}",
        );
    }
    let claims = ownership
        .parsed_claims()
        .iter()
        .map(|claim| (claim.span(), claim.stable_owner_id()))
        .collect::<Vec<_>>();
    for expected in [
        (
            TextSpan { start: 41, end: 44 },
            "form:existential_condition/existential_condition/0",
        ),
        (
            TextSpan { start: 44, end: 50 },
            "form:plural_existential_clause/plural_existential_clause/0",
        ),
        (TextSpan { start: 50, end: 54 }, "lexeme:VerbLexeme/Be/bare"),
        (
            TextSpan { start: 84, end: 90 },
            "form:among_phrase/among_phrase/0",
        ),
        (
            TextSpan { start: 78, end: 84 },
            "form:compound_modified_plural_nominal/compound_modified_plural_nominal/2",
        ),
        (
            TextSpan {
                start: 96,
                end: 101,
            },
            "form:that_determiner_phrase/that_determiner_phrase/0",
        ),
        (
            TextSpan {
                start: 108,
                end: 117,
            },
            "lexeme:VerbLexeme/Control/third_person_singular",
        ),
        (
            TextSpan {
                start: 117,
                end: 118,
            },
            "form:existential_condition/existential_condition/2",
        ),
    ] {
        assert!(
            claims.iter().any(|actual| actual == &expected),
            "{expected:?}: {claims:?}"
        );
    }

    let original = "At the beginning of each player's upkeep, if there are four or more basic land types among lands that player controls, this artifact deals 3 damage to that player.";
    let original_analysis = parser.analyze(original, &context);
    assert_eq!(
        original_analysis.outcome(),
        deckmaste_english_v2::parser::ParseAnalysisOutcome::Selected,
        "Mask's already-supported consequence makes the repaired authentic row Plan 08",
    );
    assert_eq!(
        original_analysis.decision().unwrap().resolution(),
        SelectionResolution::Unique,
    );
    let original_ownership = original_analysis
        .ownership()
        .expect("the authentic Mask row owns every byte");
    assert!(
        original_ownership.failures().is_empty(),
        "{original_ownership:?}"
    );
    assert!(
        original_ownership.summary().covered(),
        "{original_ownership:?}"
    );
    assert_eq!(original_ownership.rendered_text(), original);
}

#[derive(Default)]
struct ExistentialStructureVisitor(Vec<&'static str>);

impl Visitor for ExistentialStructureVisitor {
    fn visit_condition_clause(&mut self, value: &ConditionClause) {
        self.0.push("ConditionClause");
        deckmaste_english_v2::visit::walk_condition_clause(self, value);
    }

    fn visit_existential_condition(&mut self, value: &ExistentialCondition) {
        self.0.push("ExistentialCondition");
        deckmaste_english_v2::visit::walk_existential_condition(self, value);
    }

    fn visit_existential_condition_value(&mut self, value: &ExistentialConditionValue) {
        self.0.push("ExistentialConditionValue");
        deckmaste_english_v2::visit::walk_existential_condition_value(self, value);
    }

    fn visit_existential_clause(&mut self, value: &ExistentialClause) {
        self.0.push("ExistentialClause");
        deckmaste_english_v2::visit::walk_existential_clause(self, value);
    }

    fn visit_plural_existential_clause(&mut self, value: &PluralExistentialClause) {
        self.0.push("PluralExistentialClause");
        deckmaste_english_v2::visit::walk_plural_existential_clause(self, value);
    }

    fn visit_compound_modified_plural_nominal(&mut self, value: &CompoundModifiedPluralNominal) {
        self.0.push("CompoundModifiedPluralNominal");
        deckmaste_english_v2::visit::walk_compound_modified_plural_nominal(self, value);
    }

    fn visit_compound_nominal_modifier(&mut self, value: &CompoundNominalModifier) {
        self.0.push("CompoundNominalModifier");
        deckmaste_english_v2::visit::walk_compound_nominal_modifier(self, value);
    }

    fn visit_compound_modifier_member(&mut self, value: &CompoundModifierMember) {
        self.0.push("CompoundModifierMember");
        deckmaste_english_v2::visit::walk_compound_modifier_member(self, value);
    }

    fn visit_among_phrase(&mut self, value: &AmongPhrase) {
        self.0.push("AmongPhrase");
        deckmaste_english_v2::visit::walk_among_phrase(self, value);
    }

    fn visit_among_phrase_value(&mut self, value: &AmongPhraseValue) {
        self.0.push("AmongPhraseValue");
        deckmaste_english_v2::visit::walk_among_phrase_value(self, value);
    }

    fn visit_demonstrative_controls(&mut self, value: &DemonstrativeControls) {
        self.0.push("DemonstrativeControls");
        deckmaste_english_v2::visit::walk_demonstrative_controls(self, value);
    }

    fn visit_that_determiner_phrase(&mut self, value: &ThatDeterminerPhrase) {
        self.0.push("ThatDeterminerPhrase");
        deckmaste_english_v2::visit::walk_that_determiner_phrase(self, value);
    }
}

#[test]
fn existential_there_derives_be_agreement_and_visits_the_complete_structure() {
    let parser = parser();
    let context = context("Context Card", false);
    let cases = [
        (
            "At the beginning of upkeep, if there is an artifact, you gain 1 life.",
            "lexeme:VerbLexeme/Be/third_person_singular",
        ),
        (
            "At the beginning of upkeep, if there are artifacts, you gain 1 life.",
            "lexeme:VerbLexeme/Be/bare",
        ),
    ];

    for (text, be_claim) in cases {
        let analysis = parser.analyze(text, &context);
        assert_eq!(
            analysis.outcome(),
            deckmaste_english_v2::parser::ParseAnalysisOutcome::Selected,
            "{text}"
        );
        assert_eq!(
            analysis.decision().unwrap().resolution(),
            SelectionResolution::Unique,
            "{text}"
        );
        let ownership = analysis
            .ownership()
            .expect("existential agreement witness ownership");
        assert!(ownership.failures().is_empty(), "{text}: {ownership:?}");
        assert!(ownership.summary().covered(), "{text}: {ownership:?}");
        assert_eq!(ownership.rendered_text(), text);
        assert!(
            ownership
                .parsed_claims()
                .iter()
                .any(|claim| claim.stable_owner_id() == be_claim)
        );
    }

    let full = "At the beginning of each player's upkeep, if there are four or more basic land types among lands that player controls, you gain 1 life.";
    let parsed = parser
        .parse(full, &context)
        .expect("full existential visitor witness parses");
    let mut visitor = ExistentialStructureVisitor::default();
    visitor.visit_ability(&parsed);
    assert_eq!(
        visitor.0,
        [
            "ConditionClause",
            "ExistentialCondition",
            "ExistentialConditionValue",
            "ExistentialClause",
            "PluralExistentialClause",
            "CompoundModifiedPluralNominal",
            "CompoundNominalModifier",
            "CompoundModifierMember",
            "CompoundNominalModifier",
            "CompoundModifierMember",
            "AmongPhrase",
            "AmongPhraseValue",
            "DemonstrativeControls",
            "ThatDeterminerPhrase",
        ],
    );
}

#[test]
fn existential_there_rejects_malformed_agreement_capitalization_spacing_and_pivots() {
    let parser = parser();
    let context = context("Context Card", false);
    for text in [
        "At the beginning of upkeep, if there are an artifact, you gain 1 life.",
        "At the beginning of upkeep, if there is artifacts, you gain 1 life.",
        "At the beginning of upkeep, if There are artifacts, you gain 1 life.",
        "At the beginning of upkeep, if there  are artifacts, you gain 1 life.",
        "At the beginning of upkeep, if thereare artifacts, you gain 1 life.",
        "At the beginning of upkeep, if there are, you gain 1 life.",
        "At the beginning of upkeep, if there are artifacts among, you gain 1 life.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "malformed existential must not produce an authentic parse: {text}",
        );
    }
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

    fn visit_transitive_predicate(&mut self, value: &TransitivePredicate) {
        self.0.push(CostVisit::Node("TransitivePredicate"));
        deckmaste_english_v2::visit::walk_transitive_predicate(self, value);
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
                "form:target_determiner_phrase/target_determiner_phrase/0"
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
#[allow(
    clippy::too_many_lines,
    reason = "the literal trigger/activation ownership oracle authenticates every byte boundary"
)]
fn finite_trigger_and_activation_boundaries_keep_structural_bytes_separate_from_word_prefixes() {
    let parser = parser();
    let context = context("Context Card", false);

    let trigger_text = "Whenever a player connives, you gain X life.";
    let trigger_analysis = parser.analyze(trigger_text, &context);
    let trigger = trigger_analysis
        .selected()
        .expect("the finite trigger witness selects")
        .clone();
    let Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(finite),
        intervening_if,
        body,
    }) = trigger
    else {
        panic!("a finite trigger stores the generated finite-clause branch")
    };
    assert_eq!(finite.marker, TriggerMarker::Whenever);
    assert!(matches!(
        finite.clause.as_ref(),
        Clause::Finite(clause) if matches!(clause.as_ref(), FiniteClause::PlainFiniteClause(_))
    ));
    assert!(intervening_if.as_ref().is_none());
    assert!(matches!(body.as_ref(), AbilityBody::Sentences(_)));
    let trigger_ownership = trigger_analysis
        .ownership()
        .expect("the finite trigger witness has lexical ownership");
    assert_eq!(
        trigger_ownership
            .parsed_claims()
            .iter()
            .filter_map(|claim| {
                let span = claim.span();
                [(26, 27), (27, 31)]
                    .contains(&(span.start, span.end))
                    .then(|| {
                        (
                            span.start,
                            span.end,
                            &trigger_text[span.start..span.end],
                            claim.kind(),
                            claim.stable_owner_id(),
                        )
                    })
            })
            .collect::<Vec<_>>(),
        [
            (
                26,
                27,
                ",",
                LexicalProvenanceKind::FormLiteral,
                "form:triggered/triggered/1",
            ),
            (
                27,
                31,
                " you",
                LexicalProvenanceKind::Vocab,
                "vocab:SubjectPronoun/You",
            ),
        ],
        "the trigger comma and its following word prefix have distinct exact owners",
    );

    let intervening_text = "Whenever a player connives, if you connive, you gain X life.";
    let intervening_analysis = parser.analyze(intervening_text, &context);
    let intervening = intervening_analysis
        .selected()
        .expect("the finite intervening-if witness selects")
        .clone();
    let Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(finite),
        intervening_if,
        body,
    }) = intervening
    else {
        panic!("a finite intervening condition stays in the generated triggered envelope")
    };
    assert_eq!(finite.marker, TriggerMarker::Whenever);
    let Some(ConditionClause::FiniteCondition(finite_condition)) = intervening_if.as_ref().as_ref()
    else {
        panic!("a finite intervening condition remains present")
    };
    let FiniteCondition::FiniteCondition(condition) = finite_condition.as_ref();
    assert!(matches!(
        condition.clause.as_ref(),
        Clause::Finite(clause) if matches!(clause.as_ref(), FiniteClause::PlainFiniteClause(_))
    ));
    assert!(matches!(body.as_ref(), AbilityBody::Sentences(_)));
    let intervening_ownership = intervening_analysis
        .ownership()
        .expect("the finite intervening-if witness has lexical ownership");
    assert_eq!(
        intervening_ownership
            .parsed_claims()
            .iter()
            .filter_map(|claim| {
                let span = claim.span();
                [(26, 27), (27, 30), (42, 43), (43, 47)]
                    .contains(&(span.start, span.end))
                    .then(|| {
                        (
                            span.start,
                            span.end,
                            &intervening_text[span.start..span.end],
                            claim.kind(),
                            claim.stable_owner_id(),
                        )
                    })
            })
            .collect::<Vec<_>>(),
        [
            (
                26,
                27,
                ",",
                LexicalProvenanceKind::FormLiteral,
                "form:triggered/triggered/1",
            ),
            (
                27,
                30,
                " if",
                LexicalProvenanceKind::FormLiteral,
                "form:finite_condition/finite_condition/0",
            ),
            (
                42,
                43,
                ",",
                LexicalProvenanceKind::FormLiteral,
                "form:finite_condition/finite_condition/2",
            ),
            (
                43,
                47,
                " you",
                LexicalProvenanceKind::Vocab,
                "vocab:SubjectPronoun/You",
            ),
        ],
        "trigger and condition commas remain separate from their following word-prefix claims",
    );

    let activation_text = "{2}{W/U}{T}, [−2], Destroy target creature: You gain X life.";
    let activation_analysis = parser.analyze(activation_text, &context);
    let activation_ownership = activation_analysis
        .ownership()
        .expect("the mixed activation witness has lexical ownership");
    assert_eq!(
        activation_ownership
            .parsed_claims()
            .iter()
            .filter_map(|claim| {
                let span = claim.span();
                [(11, 13), (19, 21), (44, 46)]
                    .contains(&(span.start, span.end))
                    .then(|| {
                        (
                            span.start,
                            span.end,
                            &activation_text[span.start..span.end],
                            claim.kind(),
                            claim.stable_owner_id(),
                        )
                    })
            })
            .collect::<Vec<_>>(),
        [
            (
                11,
                13,
                ", ",
                LexicalProvenanceKind::FormLiteral,
                "structural:Activated/costs/separator/first/0",
            ),
            (
                19,
                21,
                ", ",
                LexicalProvenanceKind::FormLiteral,
                "structural:Activated/costs/separator/last/0",
            ),
            (
                44,
                46,
                ": ",
                LexicalProvenanceKind::FormLiteral,
                "form:activated/activated/1",
            ),
        ],
        "activation separators retain their complete structural surfaces",
    );
    assert!(
        parser
            .parse(
                "{2}{W/U}{T}, [−2], Destroy target creature: you gain X life.",
                &context,
            )
            .is_err(),
        "the activated-envelope colon-space remains a sentence-initial structural boundary",
    );
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
    assert_eq!(
        variants("Clause"),
        ["Finite", "Coordination", "Copular", "Passive"]
    );
    assert_eq!(variants("TriggerMarker"), ["When", "Whenever"]);
    assert_eq!(variants("TriggerPrefix"), ["Finite", "Temporal"]);
    assert_eq!(variants("AtPhrase"), ["AtPhrase"]);
    assert_eq!(
        variants("ConditionClause"),
        ["FiniteCondition", "ExistentialCondition"]
    );
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
            clause: Box::new(Clause::Finite(Box::new(FiniteClause::PlainFiniteClause(
                PlainFiniteClause::new(
                    Subject::SubjectPronoun(PersonalSubject {
                        word: SubjectPronoun::You,
                    }),
                    Box::new(Predicate::Atomic(Box::new(connive()))),
                )
                .expect("you and connive satisfy finite-clause agreement"),
            )))),
        }),
        intervening_if: Box::new(None),
        body: Box::new(AbilityBody::Sentences(
            Sentences::new(Box::new(vec![Sentence::Imperative(
                Imperative::new(Box::new(Predicate::Atomic(Box::new(connive()))))
                    .expect("connive satisfies bare imperative agreement"),
            )]))
            .expect("linguistic body remains nonempty"),
        )),
    });
    let Ability::Triggered(triggered) = value else {
        unreachable!("literal constructs the triggered variant")
    };
    let _: Box<Option<ConditionClause>> = triggered.intervening_if;

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
            intervening_if,
            ..
        }) = parsed
        else {
            panic!("finite trigger uses the finite generated prefix: {text}")
        };
        assert!(intervening_if.as_ref().is_none());
        assert_eq!(finite.marker, marker);
        assert!(matches!(
            finite.clause.as_ref(),
            Clause::Finite(clause) if matches!(clause.as_ref(), FiniteClause::PlainFiniteClause(_))
        ));
    }

    let temporal_text = "At the beginning of each player's draw step, you gain X life.";
    let temporal = assert_selected_trigger(&parser, &context, temporal_text);
    let Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Temporal(Temporal { phrase }),
        intervening_if,
        ..
    }) = temporal
    else {
        panic!("At takes the dedicated temporal phrase")
    };
    assert!(intervening_if.as_ref().is_none());
    let AtPhrase::AtPhrase(phrase) = phrase;
    assert_eq!(phrase.boundary(), AtBoundary::Beginning);
    assert_eq!(phrase.specifier(), Some(&TurnSpecifier::EachPlayer));
    assert_eq!(phrase.part(), TurnPart::DrawStep);
    assert_eq!(phrase.postmodifier(), None);

    let intervening_text = "Whenever a player connives, if you connive, you gain X life.";
    let intervening = assert_selected_trigger(&parser, &context, intervening_text);
    let Ability::Triggered(Triggered {
        trigger: TriggerPrefix::Finite(_),
        intervening_if,
        ..
    }) = intervening
    else {
        panic!("the immediate if clause has its dedicated finite-condition attachment")
    };
    let Some(ConditionClause::FiniteCondition(finite_condition)) = intervening_if.as_ref().as_ref()
    else {
        panic!("the immediate if clause is present")
    };
    let FiniteCondition::FiniteCondition(condition) = finite_condition.as_ref();
    let Clause::Finite(finite) = condition.clause.as_ref() else {
        panic!("the immediate if clause is finite")
    };
    let FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
        panic!("the immediate if clause is plain finite")
    };
    assert!(matches!(
        clause.subject(),
        Subject::SubjectPronoun(PersonalSubject {
            word: SubjectPronoun::You,
        })
    ));
    assert!(matches!(
        clause.predicate(),
        Predicate::Atomic(predicate) if declared_action_name(predicate) == Some("Connive")
    ));
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
            "Clause",
            "FiniteClause",
            "ConditionClause",
            "FiniteCondition",
            "FiniteConditionValue",
            "Clause",
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
    assert_eq!(variants("AbilityBody"), ["Sentences", "PlainModal"]);
    assert_eq!(variants("ModalMode"), ["ModalMode"]);
    assert_eq!(
        variants("ModalChooser"),
        ["You", "Opponent"],
        "the chooser is sealed semantic state rather than header spelling",
    );
    assert_eq!(
        variants("ModalChoiceBounds"),
        [
            "ExactlyOne",
            "ExactlyTwo",
            "OneToTwo",
            "OneOrMore",
            "ZeroToOne",
        ],
        "modal cardinality is stored as semantic bounds rather than a form tag",
    );
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
        ActivationCostComponent::Clause(cost_clause),
    ] = activated.costs()
    else {
        panic!("mixed cost builds the positional SymbolRun/Loyalty/Clause AST")
    };
    let Predicate::Atomic(cost_predicate) = cost_clause.predicate() else {
        panic!("the clause cost stores its predicate in the shared algebra")
    };
    assert_eq!(declared_action_name(cost_predicate), Some("Destroy"));
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
    let AbilityBody::Sentences(sentences) = activated.body.as_ref() else {
        panic!("the activation witness has an ordinary sentence body")
    };
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
                "form:target_determiner_phrase/target_determiner_phrase/0"
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
                "form:target_determiner_phrase/target_determiner_phrase/0"
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
            CostVisit::Node("TransitivePredicate"),
            CostVisit::Node("AbilityBody"),
            CostVisit::Node("Sentences"),
            CostVisit::Node("Sentence"),
            CostVisit::Node("Declarative"),
            CostVisit::Node("VerbPhrase"),
            CostVisit::Node("GainLife"),
            CostVisit::Node("Sentence"),
            CostVisit::Node("Imperative"),
            CostVisit::Node("VerbPhrase"),
            CostVisit::Node("TransitivePredicate"),
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
        "Remove a counter: You gain X life.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "malformed or later-plan activation surface must reject: {text}",
        );
    }

    let typed_pay = "Pay 2 life: You gain X life.";
    let analysis = parser.analyze(typed_pay, &context);
    assert!(
        analysis.selected().is_some(),
        "typed life payment is now an ordinary activation cost clause"
    );
    let ownership = analysis
        .ownership()
        .expect("typed life-payment activation has ownership");
    assert!(ownership.summary().covered());
    assert!(ownership.failures().is_empty());
    assert_eq!(ownership.rendered_text(), typed_pay);
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
    FiniteClause::PlainFiniteClause(
        PlainFiniteClause::new(subject, Box::new(predicate))
            .expect("the helper supplies matching subject-predicate agreement"),
    )
}

fn predicate_identity(predicate: &VerbPhrase) -> String {
    match predicate {
        predicate if declared_action_name(predicate) == Some("Connive") => "connive".to_owned(),
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
    let FiniteClause::PlainFiniteClause(clause) = clause else {
        panic!("clause coordination stores complete plain finite clauses")
    };
    let subject = clause.subject();
    let predicate = clause.predicate();
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
                body: Box::new(AbilityBody::Sentences(
                    Sentences::new(Box::new(vec![Sentence::Declarative(Declarative {
                        clause: Box::new(Clause::Finite(Box::new(
                            FiniteClause::AuxiliaryFiniteClause(
                                AuxiliaryFiniteClause::new(
                                    you_subject(),
                                    auxiliary,
                                    Box::new(Predicate::Atomic(Box::new(gain_life_predicate(2)))),
                                )
                                .expect("auxiliary clauses require a bare predicate"),
                            )
                        ))),
                    })]))
                    .expect("the independently built ability body is nonempty"),
                )),
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
    assert_eq!(
        variants("Auxiliary"),
        ["May", "Can", "Cant", "Must", "Didnt", "Would"]
    );
    assert_eq!(
        variants("VerbPhrase"),
        [
            "IntransitivePredicate",
            "TransitivePredicate",
            "NumerativePredicate",
            "DealDamage",
            "DealUnspecifiedDamage",
            "GainLife",
            "GainUnspecifiedLife",
            "DealDamageEqualTo",
            "GainLifeEqualTo",
            "LoseLife",
            "LoseLifeEqualTo",
            "PayLife",
            "PayMana",
            "AddMana",
            "DrawCards",
            "DrawCardsEqualTo",
            "RollDice",
            "PutCounters",
            "RemoveCounters",
            "PutInto",
            "PutOnto",
            "PutOn",
            "PutTo",
            "ReturnTo",
            "EnterPostState",
            "EnterLocation",
            "EnterControl",
            "LeaveLocation",
            "LookAt",
            "SearchFor",
            "HaveCardsInHand",
            "HaveLife",
            "HaveNoMaximumHandSize",
            "HaveObjectControl",
        ],
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
    let Clause::Finite(finite) = one_declarative_clause(&selected) else {
        panic!("predicate coordination has the staged declarative AST: {text}")
    };
    let FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
        panic!("predicate coordination has a plain finite clause: {text}")
    };
    let Predicate::Coordination(coordination) = clause.predicate() else {
        panic!("predicate coordination has the staged declarative AST: {text}")
    };
    let members = match (kind, coordination.as_ref()) {
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
    let Clause::Coordination(coordination) = one_declarative_clause(&selected) else {
        panic!("complete clauses coordinate above finite clauses: {text}")
    };
    let members = match (kind, coordination.as_ref()) {
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

    assert!(AndPredicateCoordination::new(Box::new(vec![gain_life_predicate(1)])).is_none());
    assert!(
        AndClauseCoordination::new(Box::new(vec![plain_finite(
            you_subject(),
            Predicate::Atomic(Box::new(gain_life_predicate(1))),
        )]))
        .is_none()
    );

    let selected = assert_one_logic_candidate(&parser, &context, "You gain 1 life.");
    let Clause::Finite(finite) = one_declarative_clause(&selected) else {
        panic!("one atomic clause retains the plain finite-clause shape")
    };
    let FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
        panic!("one atomic clause remains plain finite")
    };
    assert!(matches!(clause.predicate(), Predicate::Atomic(_)));
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
            predicate if declared_action_name(predicate) == Some("Connive") => {
                self.0.push(LogicVisit::Connive);
            }
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
    let Clause::Finite(finite) = one_declarative_clause(&predicate_ability) else {
        unreachable!()
    };
    let FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
        unreachable!()
    };
    let mut visitor = LogicVisitor::default();
    visitor.visit_predicate(clause.predicate());
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
    let mut visitor = LogicVisitor::default();
    visitor.visit_clause(one_declarative_clause(&clause_ability));
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
    let Clause::Finite(finite) = one_declarative_clause(&auxiliary) else {
        unreachable!()
    };
    let FiniteClause::AuxiliaryFiniteClause(auxiliary) = finite.as_ref() else {
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
    plain_finite(you_subject(), Predicate::Atomic(Box::new(connive())))
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
    plain_finite(player_subject(), Predicate::Atomic(Box::new(connive())))
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
    Clause::Finite(Box::new(plain_finite(
        you_subject(),
        Predicate::Atomic(Box::new(gain_life_predicate(amount))),
    )))
}

#[test]
fn attachment_products_have_an_intermediate_linguistic_stage_for_imperatives() {
    let parser = parser();
    let context = context("Context Card", false);
    let expected = Sentence::Attached(Attached {
        attachment: Box::new(ClauseAttachment::PreposedIfPredicate(Box::new(
            PreposedIfPredicate::new(
                connive_clause(),
                Box::new(Predicate::Atomic(Box::new(connive()))),
            )
            .expect("the attached imperative predicate is bare"),
        ))),
    });
    let selected = assert_one_logic_candidate(&parser, &context, "If you connive, connive.");
    assert_eq!(
        selected,
        Ability::Plain(Plain {
            body: Box::new(AbilityBody::Sentences(
                Sentences::new(Box::new(vec![expected])).expect("one sentence"),
            )),
        }),
        "the attachment lives between Sentence and its predicate/finite-clause payload",
    );
}

#[test]
#[allow(
    clippy::large_stack_arrays,
    reason = "the fixed attachment table keeps each complete expected AST beside its surface"
)]
fn conditional_attachments_have_distinct_position_shapes_and_exact_asts() {
    let parser = parser();
    let context = context("Context Card", false);
    let condition = connive_clause();
    let body = gain_clause(2);

    for (text, expected) in [
        (
            "If you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: Box::new(ClauseAttachment::PreposedIf(Box::new(PreposedIf {
                    condition: condition.clone(),
                    body: Box::new(body.clone()),
                }))),
            }),
        ),
        (
            "You gain 2 life if you connive.",
            Sentence::Attached(Attached {
                attachment: Box::new(ClauseAttachment::PostposedIf(Box::new(PostposedIf {
                    body: Box::new(body.clone()),
                    condition: condition.clone(),
                }))),
            }),
        ),
        (
            "You gain 2 life unless you connive.",
            Sentence::Attached(Attached {
                attachment: Box::new(ClauseAttachment::PostposedUnless(Box::new(
                    PostposedUnless {
                        body: Box::new(body.clone()),
                        condition: condition.clone(),
                    },
                ))),
            }),
        ),
        (
            "As long as you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: Box::new(ClauseAttachment::PreposedAsLongAs(Box::new(
                    PreposedAsLongAs {
                        condition: condition.clone(),
                        body: Box::new(body.clone()),
                    },
                ))),
            }),
        ),
        (
            "While you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: Box::new(ClauseAttachment::PreposedWhile(Box::new(PreposedWhile {
                    condition: condition.clone(),
                    body: Box::new(body.clone()),
                }))),
            }),
        ),
        (
            "During you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: Box::new(ClauseAttachment::PreposedDuring(Box::new(PreposedDuring {
                    condition: condition.clone(),
                    body: Box::new(body.clone()),
                }))),
            }),
        ),
        (
            "Until you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: Box::new(ClauseAttachment::PreposedUntil(Box::new(PreposedUntil {
                    condition,
                    body: Box::new(body),
                }))),
            }),
        ),
    ] {
        let selected = assert_one_logic_candidate(&parser, &context, text);
        assert_eq!(
            selected,
            Ability::Plain(Plain {
                body: Box::new(AbilityBody::Sentences(
                    Sentences::new(Box::new(vec![expected]))
                        .expect("one conditional sentence is nonempty"),
                )),
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
    let [Sentence::Attached(Attached { attachment })] = plain_sentences(&ordered).sentences()
    else {
        panic!("then stores an ordered finite-clause sequence rather than nested sentences")
    };
    let ClauseAttachment::ThenSequence(sequence) = attachment.as_ref() else {
        panic!("then retains its dedicated attachment")
    };
    assert_eq!(
        sequence
            .members()
            .iter()
            .map(|member| match member {
                Clause::Finite(member) => finite_clause_identity(member),
                Clause::Coordination(_) | Clause::Copular(_) | Clause::Passive(_) => {
                    panic!("then members retain their finite clause shape")
                }
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
        let [
            Sentence::Declarative(_),
            Sentence::Attached(Attached { attachment }),
        ] = plain_sentences(&selected).sentences()
        else {
            panic!("the reflexive subordinate is its own sentence shape: {text}")
        };
        let ClauseAttachment::ReflexiveSubordinate(subordinate) = attachment.as_ref() else {
            panic!("the reflexive subordinate retains its attachment: {text}")
        };
        assert_eq!(
            subordinate.kind, kind,
            "the lexical subordinate kind is stored: {text}"
        );
        assert_eq!(
            subordinate.body.as_ref(),
            &Clause::Finite(Box::new(connive_clause()))
        );
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
        intervening_if,
        body,
        ..
    }) = ordinary_ability
    else {
        panic!("ordinary trailing if belongs to the triggered body")
    };
    assert!(intervening_if.as_ref().is_none());
    let AbilityBody::Sentences(ordinary_body) = body.as_ref() else {
        panic!("ordinary trailing if has a sentence body")
    };
    assert!(matches!(
        ordinary_body.sentences(),
        [Sentence::Attached(Attached { attachment })]
            if matches!(attachment.as_ref(), ClauseAttachment::PostposedIf(_))
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
        intervening_if,
        body,
        ..
    }) = intervening_ability
    else {
        panic!("intervening-if remains attached to the trigger envelope")
    };
    assert!(matches!(
        intervening_if.as_ref().as_ref(),
        Some(ConditionClause::FiniteCondition(_))
    ));
    let AbilityBody::Sentences(intervening_body) = body.as_ref() else {
        panic!("intervening-if has a sentence body")
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
            Predicate::Atomic(predicate) if declared_action_name(predicate) == Some("Connive") => {
                self.0.push("Predicate:Connive");
            }
            Predicate::Atomic(predicate)
                if matches!(
                    predicate.as_ref(),
                    VerbPhrase::GainLife(GainLife {
                        amount: Amount::Number(NumberAmount {
                            number: ScalarNumber { magnitude: 2 },
                        }),
                    })
                ) =>
            {
                self.0.push("Predicate:Gain2");
            }
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
    let gain = Predicate::Atomic(Box::new(gain_life_predicate(2)));
    let root_text = "If you connive, gain 2 life.";
    let root = assert_one_logic_candidate(&parser, &context, root_text);

    assert_eq!(
        root,
        Ability::Plain(Plain {
            body: Box::new(AbilityBody::Sentences(
                Sentences::new(Box::new(vec![Sentence::Attached(Attached {
                    attachment: Box::new(ClauseAttachment::PreposedIfPredicate(Box::new(
                        PreposedIfPredicate::new(condition.clone(), Box::new(gain.clone()))
                            .expect("the attached gain predicate is bare"),
                    ))),
                })]))
                .expect("one root sentence"),
            )),
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
    let gain = Predicate::Atomic(Box::new(gain_life_predicate(2)));
    let trigger_text = "Whenever a player connives, gain 2 life if you connive.";
    let trigger = assert_one_logic_candidate(&parser, &context, trigger_text);

    assert_eq!(
        trigger,
        Ability::Triggered(Triggered {
            trigger: TriggerPrefix::Finite(Finite {
                marker: TriggerMarker::Whenever,
                clause: Box::new(Clause::Finite(Box::new(player_connive_clause()))),
            }),
            intervening_if: Box::new(None),
            body: Box::new(AbilityBody::Sentences(
                Sentences::new(Box::new(vec![Sentence::Attached(Attached {
                    attachment: Box::new(ClauseAttachment::PostposedIfPredicate(Box::new(
                        PostposedIfPredicate::new(Box::new(gain), condition)
                            .expect("the attached gain predicate is bare"),
                    ))),
                })]))
                .expect("one trigger-body sentence"),
            )),
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
    let gain = Predicate::Atomic(Box::new(gain_life_predicate(2)));
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
        activated.body.as_ref(),
        &AbilityBody::Sentences(
            Sentences::new(Box::new(vec![Sentence::Attached(Attached {
                attachment: Box::new(ClauseAttachment::PreposedIfPredicate(Box::new(
                    PreposedIfPredicate::new(condition, Box::new(gain))
                        .expect("the attached gain predicate is bare"),
                ))),
            })]))
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
    let gain = Predicate::Atomic(Box::new(gain_life_predicate(2)));

    for (text, expected) in [
        (
            "Gain 2 life if you connive.",
            ClauseAttachment::PostposedIfPredicate(Box::new(
                PostposedIfPredicate::new(Box::new(gain.clone()), condition.clone())
                    .expect("the attached gain predicate is bare"),
            )),
        ),
        (
            "Gain 2 life unless you connive.",
            ClauseAttachment::PostposedUnlessPredicate(Box::new(
                PostposedUnlessPredicate::new(Box::new(gain.clone()), condition.clone())
                    .expect("the attached gain predicate is bare"),
            )),
        ),
        (
            "As long as you connive, gain 2 life.",
            ClauseAttachment::PreposedAsLongAsPredicate(Box::new(
                PreposedAsLongAsPredicate::new(condition.clone(), Box::new(gain.clone()))
                    .expect("the attached gain predicate is bare"),
            )),
        ),
        (
            "While you connive, gain 2 life.",
            ClauseAttachment::PreposedWhilePredicate(Box::new(
                PreposedWhilePredicate::new(condition.clone(), Box::new(gain.clone()))
                    .expect("the attached gain predicate is bare"),
            )),
        ),
        (
            "During you connive, gain 2 life.",
            ClauseAttachment::PreposedDuringPredicate(Box::new(
                PreposedDuringPredicate::new(condition.clone(), Box::new(gain.clone()))
                    .expect("the attached gain predicate is bare"),
            )),
        ),
        (
            "Until you connive, gain 2 life.",
            ClauseAttachment::PreposedUntilPredicate(Box::new(
                PreposedUntilPredicate::new(condition.clone(), Box::new(gain.clone()))
                    .expect("the attached gain predicate is bare"),
            )),
        ),
    ] {
        let selected = assert_one_logic_candidate(&parser, &context, text);
        assert_eq!(
            plain_sentences(&selected).sentences(),
            [Sentence::Attached(Attached {
                attachment: Box::new(expected)
            })],
            "each predicate attachment inhabits the single intermediate stage: {text}",
        );
    }

    let ordered = assert_one_logic_candidate(&parser, &context, "Gain 1 life, then connive.");
    let [Sentence::Attached(Attached { attachment })] = plain_sentences(&ordered).sentences()
    else {
        panic!("predicate ordering stores its members in the attachment stage")
    };
    let ClauseAttachment::ThenPredicateSequence(sequence) = attachment.as_ref() else {
        panic!("predicate ordering retains its attachment")
    };
    assert_eq!(
        sequence.members(),
        [
            Predicate::Atomic(Box::new(gain_life_predicate(1))),
            Predicate::Atomic(Box::new(connive())),
        ],
    );

    let reflexive = assert_one_logic_candidate(
        &parser,
        &context,
        "You gain 1 life. When you do, gain 2 life.",
    );
    let [_, Sentence::Attached(Attached { attachment })] = plain_sentences(&reflexive).sentences()
    else {
        panic!("predicate reflexive subordinate stores its body in the attachment stage")
    };
    let ClauseAttachment::ReflexivePredicateSubordinate(subordinate) = attachment.as_ref() else {
        panic!("predicate reflexive subordinate retains its attachment")
    };
    assert_eq!(subordinate.kind, ReflexiveSubordinateKind::WhenYouDo);
    assert_eq!(subordinate.body(), &gain);
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
    let [Sentence::Attached(Attached { attachment })] = plain_sentences(&ordered).sentences()
    else {
        unreachable!()
    };
    let ClauseAttachment::ThenSequence(sequence) = attachment.as_ref() else {
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
    let [_, Sentence::Attached(Attached { attachment })] = plain_sentences(&reflexive).sentences()
    else {
        unreachable!()
    };
    let ClauseAttachment::ReflexiveSubordinate(subordinate) = attachment.as_ref() else {
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
    assert_eq!(
        report.stored_spelling_codecs(),
        ["SelfReferenceSpelling"],
        "modal headers store chooser and bounds semantics, never a selected spelling arm",
    );
    for role in ["ModalModeValue.sentences", "PlainModal.modes"] {
        assert!(
            report.sequence_roles().iter().any(|actual| actual == role),
            "generated report contains structural sequence authority for {role}",
        );
        assert!(
            report
                .uniform_separators()
                .iter()
                .any(|actual| actual == role),
            "generated report contains exact uniform separator authority for {role}",
        );
    }
    assert!(
        report
            .terminators()
            .iter()
            .any(|actual| actual == "ModalModeValue.sentences"),
        "each modal sentence owns its generated period terminator",
    );
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

fn modal_text(header: &str) -> String {
    format!("{header} —\n• You gain 1 life.\n• You gain 2 life.")
}

fn wrapped_modal_text(envelope: &str, header: &str) -> String {
    match envelope {
        "root" => modal_text(header),
        "trigger" => format!(
            "Whenever a player connives, if you connive, {}",
            modal_text(header)
        ),
        "activation" => format!("{{T}}: {}", modal_text(header)),
        _ => panic!("unknown modal envelope {envelope}"),
    }
}

fn selected_plain_modal<'a>(ability: &'a Ability, envelope: &str) -> &'a PlainModal {
    let body = match (envelope, ability) {
        ("root", Ability::Plain(Plain { body }))
        | ("activation", Ability::Activated(Activated { body, .. })) => body,
        (
            "trigger",
            Ability::Triggered(Triggered {
                intervening_if,
                body,
                ..
            }),
        ) if matches!(
            intervening_if.as_ref().as_ref(),
            Some(ConditionClause::FiniteCondition(_))
        ) =>
        {
            body
        }
        _ => panic!("modal body did not retain its {envelope} envelope: {ability:?}"),
    };
    let AbilityBody::PlainModal(modal) = body.as_ref() else {
        panic!("the envelope contains the unchanged plain-modal body")
    };
    modal
}

#[test]
fn every_plain_modal_header_selects_independently_in_every_ability_envelope() {
    let parser = parser();
    let context = context("Context Card", false);
    for (root_header, wrapped_header, chooser, bounds) in [
        (
            "Choose one",
            "choose one",
            ModalChooser::You,
            ModalChoiceBounds::ExactlyOne,
        ),
        (
            "Choose two",
            "choose two",
            ModalChooser::You,
            ModalChoiceBounds::ExactlyTwo,
        ),
        (
            "Choose one or both",
            "choose one or both",
            ModalChooser::You,
            ModalChoiceBounds::OneToTwo,
        ),
        (
            "Choose one or more",
            "choose one or more",
            ModalChooser::You,
            ModalChoiceBounds::OneOrMore,
        ),
        (
            "Choose up to one",
            "choose up to one",
            ModalChooser::You,
            ModalChoiceBounds::ZeroToOne,
        ),
        (
            "An opponent chooses one",
            "an opponent chooses one",
            ModalChooser::Opponent,
            ModalChoiceBounds::ExactlyOne,
        ),
    ] {
        for (envelope, header) in [
            ("root", root_header),
            ("trigger", wrapped_header),
            ("activation", root_header),
        ] {
            let text = wrapped_modal_text(envelope, header);
            let selected = assert_one_logic_candidate(&parser, &context, &text);
            let modal = selected_plain_modal(&selected, envelope);
            assert_eq!(modal.chooser(), chooser, "{text}");
            assert_eq!(modal.bounds(), bounds, "{text}");
            assert_eq!(modal.modes().len(), 2, "{text}");
            assert_eq!(
                selected.render(&context, parser.environment()),
                text,
                "the complete wrapped modal renders identically",
            );
        }
    }
}

fn modal_header_evidence() -> [(&'static str, &'static str, ModalChooser, ModalChoiceBounds); 6] {
    [
        (
            "Choose one",
            "choose one",
            ModalChooser::You,
            ModalChoiceBounds::ExactlyOne,
        ),
        (
            "Choose two",
            "choose two",
            ModalChooser::You,
            ModalChoiceBounds::ExactlyTwo,
        ),
        (
            "Choose one or both",
            "choose one or both",
            ModalChooser::You,
            ModalChoiceBounds::OneToTwo,
        ),
        (
            "Choose one or more",
            "choose one or more",
            ModalChooser::You,
            ModalChoiceBounds::OneOrMore,
        ),
        (
            "Choose up to one",
            "choose up to one",
            ModalChooser::You,
            ModalChoiceBounds::ZeroToOne,
        ),
        (
            "An opponent chooses one",
            "an opponent chooses one",
            ModalChooser::Opponent,
            ModalChoiceBounds::ExactlyOne,
        ),
    ]
}

fn expected_modal_body(chooser: ModalChooser, bounds: ModalChoiceBounds) -> AbilityBody {
    AbilityBody::PlainModal(
        PlainModal::new(
            chooser,
            bounds,
            Box::new(vec![
                modal_mode(vec![modal_sentence(gain_life_predicate(1))]),
                modal_mode(vec![modal_sentence(gain_life_predicate(2))]),
            ]),
        )
        .expect("the reviewed modal header and two nonempty modes construct"),
    )
}

fn expected_modal_envelope(
    envelope: &str,
    chooser: ModalChooser,
    bounds: ModalChoiceBounds,
) -> Ability {
    let body = expected_modal_body(chooser, bounds);
    expected_modal_envelope_with_body(envelope, body)
}

fn expected_modal_envelope_with_body(envelope: &str, body: AbilityBody) -> Ability {
    match envelope {
        "root" => Ability::Plain(Plain {
            body: Box::new(body),
        }),
        "trigger" => Ability::Triggered(Triggered {
            trigger: TriggerPrefix::Finite(Finite {
                marker: TriggerMarker::Whenever,
                clause: Box::new(Clause::Finite(Box::new(player_connive_clause()))),
            }),
            intervening_if: Box::new(Some(ConditionClause::FiniteCondition(Box::new(
                FiniteCondition::FiniteCondition(FiniteConditionValue {
                    clause: Box::new(Clause::Finite(Box::new(connive_clause()))),
                }),
            )))),
            body: Box::new(body),
        }),
        "activation" => Ability::Activated(
            Activated::new(Box::new(vec![tap_cost()]), Box::new(body))
                .expect("the exact tap cost and modal body construct"),
        ),
        _ => panic!("unknown modal envelope {envelope}"),
    }
}

#[derive(Default)]
struct CompleteModalEnvelopeVisitor(Vec<String>);

impl CompleteModalEnvelopeVisitor {
    fn push(&mut self, value: impl Into<String>) {
        self.0.push(value.into());
    }
}

impl Visitor for CompleteModalEnvelopeVisitor {
    fn visit_ability(&mut self, value: &Ability) {
        self.push("Ability");
        deckmaste_english_v2::visit::walk_ability(self, value);
    }

    fn visit_plain(&mut self, value: &Plain) {
        self.push("Plain");
        deckmaste_english_v2::visit::walk_plain(self, value);
    }

    fn visit_triggered(&mut self, value: &Triggered) {
        self.push("Triggered");
        deckmaste_english_v2::visit::walk_triggered(self, value);
    }

    fn visit_activated(&mut self, value: &Activated) {
        self.push("Activated");
        deckmaste_english_v2::visit::walk_activated(self, value);
    }

    fn visit_trigger_prefix(&mut self, value: &TriggerPrefix) {
        self.push("TriggerPrefix");
        deckmaste_english_v2::visit::walk_trigger_prefix(self, value);
    }

    fn visit_finite(&mut self, value: &Finite) {
        self.push("Finite");
        deckmaste_english_v2::visit::walk_finite(self, value);
    }

    fn visit_trigger_marker(&mut self, value: TriggerMarker) {
        self.push(format!("TriggerMarker:{value:?}"));
    }

    fn visit_condition_clause(&mut self, value: &ConditionClause) {
        self.push("ConditionClause");
        deckmaste_english_v2::visit::walk_condition_clause(self, value);
    }

    fn visit_finite_condition(&mut self, value: &FiniteCondition) {
        self.push("FiniteCondition");
        deckmaste_english_v2::visit::walk_finite_condition(self, value);
    }

    fn visit_finite_condition_value(&mut self, value: &FiniteConditionValue) {
        self.push("FiniteConditionValue");
        deckmaste_english_v2::visit::walk_finite_condition_value(self, value);
    }

    fn visit_activation_cost_component(&mut self, value: &ActivationCostComponent) {
        self.push("ActivationCostComponent");
        deckmaste_english_v2::visit::walk_activation_cost_component(self, value);
    }

    fn visit_symbol_run(&mut self, value: &SymbolRun) {
        self.push("SymbolRun");
        deckmaste_english_v2::visit::walk_symbol_run(self, value);
    }

    fn visit_cost_symbol(&mut self, value: &CostSymbol) {
        self.push("CostSymbol");
        deckmaste_english_v2::visit::walk_cost_symbol(self, value);
    }

    fn visit_fixed_symbol(&mut self, value: &FixedSymbol) {
        self.push("FixedSymbol");
        deckmaste_english_v2::visit::walk_fixed_symbol(self, value);
    }

    fn visit_fixed_cost_symbol(&mut self, value: FixedCostSymbol) {
        self.push(format!("FixedCostSymbol:{value:?}"));
    }

    fn visit_ability_body(&mut self, value: &AbilityBody) {
        self.push("AbilityBody");
        deckmaste_english_v2::visit::walk_ability_body(self, value);
    }

    fn visit_plain_modal(&mut self, value: &PlainModal) {
        self.push("PlainModal");
        deckmaste_english_v2::visit::walk_plain_modal(self, value);
    }

    fn visit_modal_chooser(&mut self, value: ModalChooser) {
        self.push(format!("ModalChooser:{value:?}"));
    }

    fn visit_modal_choice_bounds(&mut self, value: ModalChoiceBounds) {
        self.push(format!("ModalChoiceBounds:{value:?}"));
    }

    fn visit_modal_mode(&mut self, value: &ModalMode) {
        self.push("ModalMode");
        deckmaste_english_v2::visit::walk_modal_mode(self, value);
    }

    fn visit_modal_mode_value(&mut self, value: &ModalModeValue) {
        self.push("ModalModeValue");
        deckmaste_english_v2::visit::walk_modal_mode_value(self, value);
    }

    fn visit_sentence(&mut self, value: &Sentence) {
        self.push("Sentence");
        deckmaste_english_v2::visit::walk_sentence(self, value);
    }

    fn visit_declarative(&mut self, value: &Declarative) {
        self.push("Declarative");
        deckmaste_english_v2::visit::walk_declarative(self, value);
    }

    fn visit_clause(&mut self, value: &Clause) {
        self.push("Clause");
        deckmaste_english_v2::visit::walk_clause(self, value);
    }

    fn visit_finite_clause(&mut self, value: &FiniteClause) {
        self.push("FiniteClause");
        deckmaste_english_v2::visit::walk_finite_clause(self, value);
    }

    fn visit_subject(&mut self, value: &Subject) {
        self.push(format!("Subject:{}", subject_identity(value)));
    }

    fn visit_predicate(&mut self, value: &Predicate) {
        let Predicate::Atomic(value) = value else {
            panic!("modal witness predicate is atomic: {value:?}")
        };
        self.push(format!("Predicate:{}", predicate_identity(value)));
    }
}

fn expected_complete_modal_visit(
    envelope: &str,
    chooser: ModalChooser,
    bounds: ModalChoiceBounds,
) -> Vec<String> {
    let mut expected = vec!["Ability".to_owned()];
    match envelope {
        "root" => expected.push("Plain".to_owned()),
        "trigger" => expected.extend(
            [
                "Triggered",
                "TriggerPrefix",
                "Finite",
                "TriggerMarker:Whenever",
                "Clause",
                "FiniteClause",
                "Subject:player",
                "Predicate:connive",
                "ConditionClause",
                "FiniteCondition",
                "FiniteConditionValue",
                "Clause",
                "FiniteClause",
                "Subject:you",
                "Predicate:connive",
            ]
            .map(str::to_owned),
        ),
        "activation" => expected.extend(
            [
                "Activated",
                "ActivationCostComponent",
                "SymbolRun",
                "CostSymbol",
                "FixedSymbol",
                "FixedCostSymbol:Tap",
            ]
            .map(str::to_owned),
        ),
        _ => panic!("unknown modal envelope {envelope}"),
    }
    expected.extend([
        "AbilityBody".to_owned(),
        "PlainModal".to_owned(),
        format!("ModalChooser:{chooser:?}"),
        format!("ModalChoiceBounds:{bounds:?}"),
        "ModalMode".to_owned(),
        "ModalModeValue".to_owned(),
        "Sentence".to_owned(),
        "Declarative".to_owned(),
        "Clause".to_owned(),
        "FiniteClause".to_owned(),
        "Subject:you".to_owned(),
        "Predicate:gain:1".to_owned(),
        "ModalMode".to_owned(),
        "ModalModeValue".to_owned(),
        "Sentence".to_owned(),
        "Declarative".to_owned(),
        "Clause".to_owned(),
        "FiniteClause".to_owned(),
        "Subject:you".to_owned(),
        "Predicate:gain:2".to_owned(),
    ]);
    expected
}

type ModalClaim = (usize, usize, &'static str);

const ROOT_EXACTLY_ONE_CLAIMS: &[ModalClaim] = &[
    (0, 6, "vocab:ModalChooser/You"),
    (6, 10, "vocab:ModalChoiceBounds/ExactlyOne"),
    (10, 15, "form:plain_modal/exactly_one/2"),
    (15, 19, "form:modal_mode/modal_mode/0"),
    (19, 22, "vocab:SubjectPronoun/You"),
    (22, 27, "lexeme:VerbLexeme/Gain/bare"),
    (27, 29, "codec:ScalarNumber"),
    (29, 34, "form:gain_life/gain_life/2"),
    (34, 35, "structural:ModalModeValue/sentences/terminator/0"),
    (35, 36, "structural:PlainModal/modes/separator/uniform/0"),
    (36, 40, "form:modal_mode/modal_mode/0"),
    (40, 43, "vocab:SubjectPronoun/You"),
    (43, 48, "lexeme:VerbLexeme/Gain/bare"),
    (48, 50, "codec:ScalarNumber"),
    (50, 55, "form:gain_life/gain_life/2"),
    (55, 56, "structural:ModalModeValue/sentences/terminator/0"),
];

const ROOT_EXACTLY_TWO_CLAIMS: &[ModalClaim] = &[
    (0, 6, "vocab:ModalChooser/You"),
    (6, 10, "vocab:ModalChoiceBounds/ExactlyTwo"),
    (10, 15, "form:plain_modal/exactly_two/2"),
    (15, 19, "form:modal_mode/modal_mode/0"),
    (19, 22, "vocab:SubjectPronoun/You"),
    (22, 27, "lexeme:VerbLexeme/Gain/bare"),
    (27, 29, "codec:ScalarNumber"),
    (29, 34, "form:gain_life/gain_life/2"),
    (34, 35, "structural:ModalModeValue/sentences/terminator/0"),
    (35, 36, "structural:PlainModal/modes/separator/uniform/0"),
    (36, 40, "form:modal_mode/modal_mode/0"),
    (40, 43, "vocab:SubjectPronoun/You"),
    (43, 48, "lexeme:VerbLexeme/Gain/bare"),
    (48, 50, "codec:ScalarNumber"),
    (50, 55, "form:gain_life/gain_life/2"),
    (55, 56, "structural:ModalModeValue/sentences/terminator/0"),
];

const ROOT_ONE_TO_TWO_CLAIMS: &[ModalClaim] = &[
    (0, 6, "vocab:ModalChooser/You"),
    (6, 18, "vocab:ModalChoiceBounds/OneToTwo"),
    (18, 23, "form:plain_modal/one_to_two/2"),
    (23, 27, "form:modal_mode/modal_mode/0"),
    (27, 30, "vocab:SubjectPronoun/You"),
    (30, 35, "lexeme:VerbLexeme/Gain/bare"),
    (35, 37, "codec:ScalarNumber"),
    (37, 42, "form:gain_life/gain_life/2"),
    (42, 43, "structural:ModalModeValue/sentences/terminator/0"),
    (43, 44, "structural:PlainModal/modes/separator/uniform/0"),
    (44, 48, "form:modal_mode/modal_mode/0"),
    (48, 51, "vocab:SubjectPronoun/You"),
    (51, 56, "lexeme:VerbLexeme/Gain/bare"),
    (56, 58, "codec:ScalarNumber"),
    (58, 63, "form:gain_life/gain_life/2"),
    (63, 64, "structural:ModalModeValue/sentences/terminator/0"),
];

const ROOT_ONE_OR_MORE_CLAIMS: &[ModalClaim] = &[
    (0, 6, "vocab:ModalChooser/You"),
    (6, 18, "vocab:ModalChoiceBounds/OneOrMore"),
    (18, 23, "form:plain_modal/one_or_more/2"),
    (23, 27, "form:modal_mode/modal_mode/0"),
    (27, 30, "vocab:SubjectPronoun/You"),
    (30, 35, "lexeme:VerbLexeme/Gain/bare"),
    (35, 37, "codec:ScalarNumber"),
    (37, 42, "form:gain_life/gain_life/2"),
    (42, 43, "structural:ModalModeValue/sentences/terminator/0"),
    (43, 44, "structural:PlainModal/modes/separator/uniform/0"),
    (44, 48, "form:modal_mode/modal_mode/0"),
    (48, 51, "vocab:SubjectPronoun/You"),
    (51, 56, "lexeme:VerbLexeme/Gain/bare"),
    (56, 58, "codec:ScalarNumber"),
    (58, 63, "form:gain_life/gain_life/2"),
    (63, 64, "structural:ModalModeValue/sentences/terminator/0"),
];

const ROOT_ZERO_TO_ONE_CLAIMS: &[ModalClaim] = &[
    (0, 6, "vocab:ModalChooser/You"),
    (6, 16, "vocab:ModalChoiceBounds/ZeroToOne"),
    (16, 21, "form:plain_modal/zero_to_one/2"),
    (21, 25, "form:modal_mode/modal_mode/0"),
    (25, 28, "vocab:SubjectPronoun/You"),
    (28, 33, "lexeme:VerbLexeme/Gain/bare"),
    (33, 35, "codec:ScalarNumber"),
    (35, 40, "form:gain_life/gain_life/2"),
    (40, 41, "structural:ModalModeValue/sentences/terminator/0"),
    (41, 42, "structural:PlainModal/modes/separator/uniform/0"),
    (42, 46, "form:modal_mode/modal_mode/0"),
    (46, 49, "vocab:SubjectPronoun/You"),
    (49, 54, "lexeme:VerbLexeme/Gain/bare"),
    (54, 56, "codec:ScalarNumber"),
    (56, 61, "form:gain_life/gain_life/2"),
    (61, 62, "structural:ModalModeValue/sentences/terminator/0"),
];

const ROOT_OPPONENT_EXACTLY_ONE_CLAIMS: &[ModalClaim] = &[
    (0, 19, "vocab:ModalChooser/Opponent"),
    (19, 23, "vocab:ModalChoiceBounds/ExactlyOne"),
    (23, 28, "form:plain_modal/opponent_exactly_one/2"),
    (28, 32, "form:modal_mode/modal_mode/0"),
    (32, 35, "vocab:SubjectPronoun/You"),
    (35, 40, "lexeme:VerbLexeme/Gain/bare"),
    (40, 42, "codec:ScalarNumber"),
    (42, 47, "form:gain_life/gain_life/2"),
    (47, 48, "structural:ModalModeValue/sentences/terminator/0"),
    (48, 49, "structural:PlainModal/modes/separator/uniform/0"),
    (49, 53, "form:modal_mode/modal_mode/0"),
    (53, 56, "vocab:SubjectPronoun/You"),
    (56, 61, "lexeme:VerbLexeme/Gain/bare"),
    (61, 63, "codec:ScalarNumber"),
    (63, 68, "form:gain_life/gain_life/2"),
    (68, 69, "structural:ModalModeValue/sentences/terminator/0"),
];

const TRIGGER_EXACTLY_ONE_CLAIMS: &[ModalClaim] = &[
    (0, 8, "vocab:TriggerMarker/Whenever"),
    (8, 10, "form:indefinite_reference/a/0"),
    (10, 17, "lexeme:CommonNoun/Player/singular"),
    (
        17,
        26,
        "lexeme:keyword_action/Connive/third_person_singular",
    ),
    (26, 27, "form:triggered/triggered/1"),
    (27, 30, "form:finite_condition/finite_condition/0"),
    (30, 34, "vocab:SubjectPronoun/You"),
    (34, 42, "lexeme:keyword_action/Connive/bare"),
    (42, 43, "form:finite_condition/finite_condition/2"),
    (43, 50, "vocab:ModalChooser/You"),
    (50, 54, "vocab:ModalChoiceBounds/ExactlyOne"),
    (54, 59, "form:plain_modal/exactly_one/2"),
    (59, 63, "form:modal_mode/modal_mode/0"),
    (63, 66, "vocab:SubjectPronoun/You"),
    (66, 71, "lexeme:VerbLexeme/Gain/bare"),
    (71, 73, "codec:ScalarNumber"),
    (73, 78, "form:gain_life/gain_life/2"),
    (78, 79, "structural:ModalModeValue/sentences/terminator/0"),
    (79, 80, "structural:PlainModal/modes/separator/uniform/0"),
    (80, 84, "form:modal_mode/modal_mode/0"),
    (84, 87, "vocab:SubjectPronoun/You"),
    (87, 92, "lexeme:VerbLexeme/Gain/bare"),
    (92, 94, "codec:ScalarNumber"),
    (94, 99, "form:gain_life/gain_life/2"),
    (99, 100, "structural:ModalModeValue/sentences/terminator/0"),
];

const TRIGGER_EXACTLY_TWO_CLAIMS: &[ModalClaim] = &[
    (0, 8, "vocab:TriggerMarker/Whenever"),
    (8, 10, "form:indefinite_reference/a/0"),
    (10, 17, "lexeme:CommonNoun/Player/singular"),
    (
        17,
        26,
        "lexeme:keyword_action/Connive/third_person_singular",
    ),
    (26, 27, "form:triggered/triggered/1"),
    (27, 30, "form:finite_condition/finite_condition/0"),
    (30, 34, "vocab:SubjectPronoun/You"),
    (34, 42, "lexeme:keyword_action/Connive/bare"),
    (42, 43, "form:finite_condition/finite_condition/2"),
    (43, 50, "vocab:ModalChooser/You"),
    (50, 54, "vocab:ModalChoiceBounds/ExactlyTwo"),
    (54, 59, "form:plain_modal/exactly_two/2"),
    (59, 63, "form:modal_mode/modal_mode/0"),
    (63, 66, "vocab:SubjectPronoun/You"),
    (66, 71, "lexeme:VerbLexeme/Gain/bare"),
    (71, 73, "codec:ScalarNumber"),
    (73, 78, "form:gain_life/gain_life/2"),
    (78, 79, "structural:ModalModeValue/sentences/terminator/0"),
    (79, 80, "structural:PlainModal/modes/separator/uniform/0"),
    (80, 84, "form:modal_mode/modal_mode/0"),
    (84, 87, "vocab:SubjectPronoun/You"),
    (87, 92, "lexeme:VerbLexeme/Gain/bare"),
    (92, 94, "codec:ScalarNumber"),
    (94, 99, "form:gain_life/gain_life/2"),
    (99, 100, "structural:ModalModeValue/sentences/terminator/0"),
];

const TRIGGER_ONE_TO_TWO_CLAIMS: &[ModalClaim] = &[
    (0, 8, "vocab:TriggerMarker/Whenever"),
    (8, 10, "form:indefinite_reference/a/0"),
    (10, 17, "lexeme:CommonNoun/Player/singular"),
    (
        17,
        26,
        "lexeme:keyword_action/Connive/third_person_singular",
    ),
    (26, 27, "form:triggered/triggered/1"),
    (27, 30, "form:finite_condition/finite_condition/0"),
    (30, 34, "vocab:SubjectPronoun/You"),
    (34, 42, "lexeme:keyword_action/Connive/bare"),
    (42, 43, "form:finite_condition/finite_condition/2"),
    (43, 50, "vocab:ModalChooser/You"),
    (50, 62, "vocab:ModalChoiceBounds/OneToTwo"),
    (62, 67, "form:plain_modal/one_to_two/2"),
    (67, 71, "form:modal_mode/modal_mode/0"),
    (71, 74, "vocab:SubjectPronoun/You"),
    (74, 79, "lexeme:VerbLexeme/Gain/bare"),
    (79, 81, "codec:ScalarNumber"),
    (81, 86, "form:gain_life/gain_life/2"),
    (86, 87, "structural:ModalModeValue/sentences/terminator/0"),
    (87, 88, "structural:PlainModal/modes/separator/uniform/0"),
    (88, 92, "form:modal_mode/modal_mode/0"),
    (92, 95, "vocab:SubjectPronoun/You"),
    (95, 100, "lexeme:VerbLexeme/Gain/bare"),
    (100, 102, "codec:ScalarNumber"),
    (102, 107, "form:gain_life/gain_life/2"),
    (107, 108, "structural:ModalModeValue/sentences/terminator/0"),
];

const TRIGGER_ONE_OR_MORE_CLAIMS: &[ModalClaim] = &[
    (0, 8, "vocab:TriggerMarker/Whenever"),
    (8, 10, "form:indefinite_reference/a/0"),
    (10, 17, "lexeme:CommonNoun/Player/singular"),
    (
        17,
        26,
        "lexeme:keyword_action/Connive/third_person_singular",
    ),
    (26, 27, "form:triggered/triggered/1"),
    (27, 30, "form:finite_condition/finite_condition/0"),
    (30, 34, "vocab:SubjectPronoun/You"),
    (34, 42, "lexeme:keyword_action/Connive/bare"),
    (42, 43, "form:finite_condition/finite_condition/2"),
    (43, 50, "vocab:ModalChooser/You"),
    (50, 62, "vocab:ModalChoiceBounds/OneOrMore"),
    (62, 67, "form:plain_modal/one_or_more/2"),
    (67, 71, "form:modal_mode/modal_mode/0"),
    (71, 74, "vocab:SubjectPronoun/You"),
    (74, 79, "lexeme:VerbLexeme/Gain/bare"),
    (79, 81, "codec:ScalarNumber"),
    (81, 86, "form:gain_life/gain_life/2"),
    (86, 87, "structural:ModalModeValue/sentences/terminator/0"),
    (87, 88, "structural:PlainModal/modes/separator/uniform/0"),
    (88, 92, "form:modal_mode/modal_mode/0"),
    (92, 95, "vocab:SubjectPronoun/You"),
    (95, 100, "lexeme:VerbLexeme/Gain/bare"),
    (100, 102, "codec:ScalarNumber"),
    (102, 107, "form:gain_life/gain_life/2"),
    (107, 108, "structural:ModalModeValue/sentences/terminator/0"),
];

const TRIGGER_ZERO_TO_ONE_CLAIMS: &[ModalClaim] = &[
    (0, 8, "vocab:TriggerMarker/Whenever"),
    (8, 10, "form:indefinite_reference/a/0"),
    (10, 17, "lexeme:CommonNoun/Player/singular"),
    (
        17,
        26,
        "lexeme:keyword_action/Connive/third_person_singular",
    ),
    (26, 27, "form:triggered/triggered/1"),
    (27, 30, "form:finite_condition/finite_condition/0"),
    (30, 34, "vocab:SubjectPronoun/You"),
    (34, 42, "lexeme:keyword_action/Connive/bare"),
    (42, 43, "form:finite_condition/finite_condition/2"),
    (43, 50, "vocab:ModalChooser/You"),
    (50, 60, "vocab:ModalChoiceBounds/ZeroToOne"),
    (60, 65, "form:plain_modal/zero_to_one/2"),
    (65, 69, "form:modal_mode/modal_mode/0"),
    (69, 72, "vocab:SubjectPronoun/You"),
    (72, 77, "lexeme:VerbLexeme/Gain/bare"),
    (77, 79, "codec:ScalarNumber"),
    (79, 84, "form:gain_life/gain_life/2"),
    (84, 85, "structural:ModalModeValue/sentences/terminator/0"),
    (85, 86, "structural:PlainModal/modes/separator/uniform/0"),
    (86, 90, "form:modal_mode/modal_mode/0"),
    (90, 93, "vocab:SubjectPronoun/You"),
    (93, 98, "lexeme:VerbLexeme/Gain/bare"),
    (98, 100, "codec:ScalarNumber"),
    (100, 105, "form:gain_life/gain_life/2"),
    (105, 106, "structural:ModalModeValue/sentences/terminator/0"),
];

const TRIGGER_OPPONENT_EXACTLY_ONE_CLAIMS: &[ModalClaim] = &[
    (0, 8, "vocab:TriggerMarker/Whenever"),
    (8, 10, "form:indefinite_reference/a/0"),
    (10, 17, "lexeme:CommonNoun/Player/singular"),
    (
        17,
        26,
        "lexeme:keyword_action/Connive/third_person_singular",
    ),
    (26, 27, "form:triggered/triggered/1"),
    (27, 30, "form:finite_condition/finite_condition/0"),
    (30, 34, "vocab:SubjectPronoun/You"),
    (34, 42, "lexeme:keyword_action/Connive/bare"),
    (42, 43, "form:finite_condition/finite_condition/2"),
    (43, 63, "vocab:ModalChooser/Opponent"),
    (63, 67, "vocab:ModalChoiceBounds/ExactlyOne"),
    (67, 72, "form:plain_modal/opponent_exactly_one/2"),
    (72, 76, "form:modal_mode/modal_mode/0"),
    (76, 79, "vocab:SubjectPronoun/You"),
    (79, 84, "lexeme:VerbLexeme/Gain/bare"),
    (84, 86, "codec:ScalarNumber"),
    (86, 91, "form:gain_life/gain_life/2"),
    (91, 92, "structural:ModalModeValue/sentences/terminator/0"),
    (92, 93, "structural:PlainModal/modes/separator/uniform/0"),
    (93, 97, "form:modal_mode/modal_mode/0"),
    (97, 100, "vocab:SubjectPronoun/You"),
    (100, 105, "lexeme:VerbLexeme/Gain/bare"),
    (105, 107, "codec:ScalarNumber"),
    (107, 112, "form:gain_life/gain_life/2"),
    (112, 113, "structural:ModalModeValue/sentences/terminator/0"),
];

const ACTIVATION_EXACTLY_ONE_CLAIMS: &[ModalClaim] = &[
    (0, 1, "form:symbol_run/symbol_run/0/prefix"),
    (1, 2, "vocab:FixedCostSymbol/Tap"),
    (2, 3, "form:symbol_run/symbol_run/0/suffix"),
    (3, 5, "form:activated/activated/1"),
    (5, 11, "vocab:ModalChooser/You"),
    (11, 15, "vocab:ModalChoiceBounds/ExactlyOne"),
    (15, 20, "form:plain_modal/exactly_one/2"),
    (20, 24, "form:modal_mode/modal_mode/0"),
    (24, 27, "vocab:SubjectPronoun/You"),
    (27, 32, "lexeme:VerbLexeme/Gain/bare"),
    (32, 34, "codec:ScalarNumber"),
    (34, 39, "form:gain_life/gain_life/2"),
    (39, 40, "structural:ModalModeValue/sentences/terminator/0"),
    (40, 41, "structural:PlainModal/modes/separator/uniform/0"),
    (41, 45, "form:modal_mode/modal_mode/0"),
    (45, 48, "vocab:SubjectPronoun/You"),
    (48, 53, "lexeme:VerbLexeme/Gain/bare"),
    (53, 55, "codec:ScalarNumber"),
    (55, 60, "form:gain_life/gain_life/2"),
    (60, 61, "structural:ModalModeValue/sentences/terminator/0"),
];

const ACTIVATION_EXACTLY_TWO_CLAIMS: &[ModalClaim] = &[
    (0, 1, "form:symbol_run/symbol_run/0/prefix"),
    (1, 2, "vocab:FixedCostSymbol/Tap"),
    (2, 3, "form:symbol_run/symbol_run/0/suffix"),
    (3, 5, "form:activated/activated/1"),
    (5, 11, "vocab:ModalChooser/You"),
    (11, 15, "vocab:ModalChoiceBounds/ExactlyTwo"),
    (15, 20, "form:plain_modal/exactly_two/2"),
    (20, 24, "form:modal_mode/modal_mode/0"),
    (24, 27, "vocab:SubjectPronoun/You"),
    (27, 32, "lexeme:VerbLexeme/Gain/bare"),
    (32, 34, "codec:ScalarNumber"),
    (34, 39, "form:gain_life/gain_life/2"),
    (39, 40, "structural:ModalModeValue/sentences/terminator/0"),
    (40, 41, "structural:PlainModal/modes/separator/uniform/0"),
    (41, 45, "form:modal_mode/modal_mode/0"),
    (45, 48, "vocab:SubjectPronoun/You"),
    (48, 53, "lexeme:VerbLexeme/Gain/bare"),
    (53, 55, "codec:ScalarNumber"),
    (55, 60, "form:gain_life/gain_life/2"),
    (60, 61, "structural:ModalModeValue/sentences/terminator/0"),
];

const ACTIVATION_ONE_TO_TWO_CLAIMS: &[ModalClaim] = &[
    (0, 1, "form:symbol_run/symbol_run/0/prefix"),
    (1, 2, "vocab:FixedCostSymbol/Tap"),
    (2, 3, "form:symbol_run/symbol_run/0/suffix"),
    (3, 5, "form:activated/activated/1"),
    (5, 11, "vocab:ModalChooser/You"),
    (11, 23, "vocab:ModalChoiceBounds/OneToTwo"),
    (23, 28, "form:plain_modal/one_to_two/2"),
    (28, 32, "form:modal_mode/modal_mode/0"),
    (32, 35, "vocab:SubjectPronoun/You"),
    (35, 40, "lexeme:VerbLexeme/Gain/bare"),
    (40, 42, "codec:ScalarNumber"),
    (42, 47, "form:gain_life/gain_life/2"),
    (47, 48, "structural:ModalModeValue/sentences/terminator/0"),
    (48, 49, "structural:PlainModal/modes/separator/uniform/0"),
    (49, 53, "form:modal_mode/modal_mode/0"),
    (53, 56, "vocab:SubjectPronoun/You"),
    (56, 61, "lexeme:VerbLexeme/Gain/bare"),
    (61, 63, "codec:ScalarNumber"),
    (63, 68, "form:gain_life/gain_life/2"),
    (68, 69, "structural:ModalModeValue/sentences/terminator/0"),
];

const ACTIVATION_ONE_OR_MORE_CLAIMS: &[ModalClaim] = &[
    (0, 1, "form:symbol_run/symbol_run/0/prefix"),
    (1, 2, "vocab:FixedCostSymbol/Tap"),
    (2, 3, "form:symbol_run/symbol_run/0/suffix"),
    (3, 5, "form:activated/activated/1"),
    (5, 11, "vocab:ModalChooser/You"),
    (11, 23, "vocab:ModalChoiceBounds/OneOrMore"),
    (23, 28, "form:plain_modal/one_or_more/2"),
    (28, 32, "form:modal_mode/modal_mode/0"),
    (32, 35, "vocab:SubjectPronoun/You"),
    (35, 40, "lexeme:VerbLexeme/Gain/bare"),
    (40, 42, "codec:ScalarNumber"),
    (42, 47, "form:gain_life/gain_life/2"),
    (47, 48, "structural:ModalModeValue/sentences/terminator/0"),
    (48, 49, "structural:PlainModal/modes/separator/uniform/0"),
    (49, 53, "form:modal_mode/modal_mode/0"),
    (53, 56, "vocab:SubjectPronoun/You"),
    (56, 61, "lexeme:VerbLexeme/Gain/bare"),
    (61, 63, "codec:ScalarNumber"),
    (63, 68, "form:gain_life/gain_life/2"),
    (68, 69, "structural:ModalModeValue/sentences/terminator/0"),
];

const ACTIVATION_ZERO_TO_ONE_CLAIMS: &[ModalClaim] = &[
    (0, 1, "form:symbol_run/symbol_run/0/prefix"),
    (1, 2, "vocab:FixedCostSymbol/Tap"),
    (2, 3, "form:symbol_run/symbol_run/0/suffix"),
    (3, 5, "form:activated/activated/1"),
    (5, 11, "vocab:ModalChooser/You"),
    (11, 21, "vocab:ModalChoiceBounds/ZeroToOne"),
    (21, 26, "form:plain_modal/zero_to_one/2"),
    (26, 30, "form:modal_mode/modal_mode/0"),
    (30, 33, "vocab:SubjectPronoun/You"),
    (33, 38, "lexeme:VerbLexeme/Gain/bare"),
    (38, 40, "codec:ScalarNumber"),
    (40, 45, "form:gain_life/gain_life/2"),
    (45, 46, "structural:ModalModeValue/sentences/terminator/0"),
    (46, 47, "structural:PlainModal/modes/separator/uniform/0"),
    (47, 51, "form:modal_mode/modal_mode/0"),
    (51, 54, "vocab:SubjectPronoun/You"),
    (54, 59, "lexeme:VerbLexeme/Gain/bare"),
    (59, 61, "codec:ScalarNumber"),
    (61, 66, "form:gain_life/gain_life/2"),
    (66, 67, "structural:ModalModeValue/sentences/terminator/0"),
];

const ACTIVATION_OPPONENT_EXACTLY_ONE_CLAIMS: &[ModalClaim] = &[
    (0, 1, "form:symbol_run/symbol_run/0/prefix"),
    (1, 2, "vocab:FixedCostSymbol/Tap"),
    (2, 3, "form:symbol_run/symbol_run/0/suffix"),
    (3, 5, "form:activated/activated/1"),
    (5, 24, "vocab:ModalChooser/Opponent"),
    (24, 28, "vocab:ModalChoiceBounds/ExactlyOne"),
    (28, 33, "form:plain_modal/opponent_exactly_one/2"),
    (33, 37, "form:modal_mode/modal_mode/0"),
    (37, 40, "vocab:SubjectPronoun/You"),
    (40, 45, "lexeme:VerbLexeme/Gain/bare"),
    (45, 47, "codec:ScalarNumber"),
    (47, 52, "form:gain_life/gain_life/2"),
    (52, 53, "structural:ModalModeValue/sentences/terminator/0"),
    (53, 54, "structural:PlainModal/modes/separator/uniform/0"),
    (54, 58, "form:modal_mode/modal_mode/0"),
    (58, 61, "vocab:SubjectPronoun/You"),
    (61, 66, "lexeme:VerbLexeme/Gain/bare"),
    (66, 68, "codec:ScalarNumber"),
    (68, 73, "form:gain_life/gain_life/2"),
    (73, 74, "structural:ModalModeValue/sentences/terminator/0"),
];

fn assert_complete_modal_envelope(
    parser: &Parser,
    context: &ParseContext<'_>,
    envelope: &str,
    header: &str,
    chooser: ModalChooser,
    bounds: ModalChoiceBounds,
    claims: &[ModalClaim],
) {
    let text = wrapped_modal_text(envelope, header);
    let analysis = parser.analyze(&text, context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("complete modal envelope must select: {analysis:?}"));
    assert_eq!(
        selected,
        &expected_modal_envelope(envelope, chooser, bounds),
        "the whole envelope AST is exact: {text}",
    );
    assert_eq!(
        selected.render(context, parser.environment()),
        text,
        "the whole envelope renders identically",
    );

    let mut visitor = CompleteModalEnvelopeVisitor::default();
    visitor.visit_ability(selected);
    assert_eq!(
        visitor.0,
        expected_complete_modal_visit(envelope, chooser, bounds),
        "the full visitor preorder retains wrapper and payload order: {text}",
    );

    let ownership = analysis
        .ownership()
        .expect("the selected whole envelope owns every byte");
    assert!(ownership.failures().is_empty(), "{text}: {ownership:?}");
    assert_eq!(
        ownership
            .parsed_claims()
            .iter()
            .map(|claim| {
                (
                    claim.span().start,
                    claim.span().end,
                    claim.stable_owner_id(),
                )
            })
            .collect::<Vec<_>>(),
        claims,
        "the whole envelope has literal exact claim spans and owners: {text}",
    );
}

#[test]
fn every_root_modal_header_has_a_complete_ast_visit_and_literal_claim_oracle() {
    let parser = parser();
    let context = context("Context Card", false);
    let claims = [
        ROOT_EXACTLY_ONE_CLAIMS,
        ROOT_EXACTLY_TWO_CLAIMS,
        ROOT_ONE_TO_TWO_CLAIMS,
        ROOT_ONE_OR_MORE_CLAIMS,
        ROOT_ZERO_TO_ONE_CLAIMS,
        ROOT_OPPONENT_EXACTLY_ONE_CLAIMS,
    ];
    for ((root_header, _, chooser, bounds), claims) in
        modal_header_evidence().into_iter().zip(claims)
    {
        assert_complete_modal_envelope(
            &parser,
            &context,
            "root",
            root_header,
            chooser,
            bounds,
            claims,
        );
    }
}

#[test]
fn every_trigger_modal_header_has_a_complete_ast_visit_and_literal_claim_oracle() {
    let parser = parser();
    let context = context("Context Card", false);
    let claims = [
        TRIGGER_EXACTLY_ONE_CLAIMS,
        TRIGGER_EXACTLY_TWO_CLAIMS,
        TRIGGER_ONE_TO_TWO_CLAIMS,
        TRIGGER_ONE_OR_MORE_CLAIMS,
        TRIGGER_ZERO_TO_ONE_CLAIMS,
        TRIGGER_OPPONENT_EXACTLY_ONE_CLAIMS,
    ];
    for ((_, wrapped_header, chooser, bounds), claims) in
        modal_header_evidence().into_iter().zip(claims)
    {
        assert_complete_modal_envelope(
            &parser,
            &context,
            "trigger",
            wrapped_header,
            chooser,
            bounds,
            claims,
        );
    }
}

#[test]
fn every_activation_modal_header_has_a_complete_ast_visit_and_literal_claim_oracle() {
    let parser = parser();
    let context = context("Context Card", false);
    let claims = [
        ACTIVATION_EXACTLY_ONE_CLAIMS,
        ACTIVATION_EXACTLY_TWO_CLAIMS,
        ACTIVATION_ONE_TO_TWO_CLAIMS,
        ACTIVATION_ONE_OR_MORE_CLAIMS,
        ACTIVATION_ZERO_TO_ONE_CLAIMS,
        ACTIVATION_OPPONENT_EXACTLY_ONE_CLAIMS,
    ];
    for ((root_header, _, chooser, bounds), claims) in
        modal_header_evidence().into_iter().zip(claims)
    {
        assert_complete_modal_envelope(
            &parser,
            &context,
            "activation",
            root_header,
            chooser,
            bounds,
            claims,
        );
    }
}

fn modal_sentence(predicate: VerbPhrase) -> Sentence {
    Sentence::Declarative(Declarative {
        clause: Box::new(Clause::Finite(Box::new(plain_finite(
            you_subject(),
            Predicate::Atomic(Box::new(predicate)),
        )))),
    })
}

fn modal_mode(sentences: Vec<Sentence>) -> ModalMode {
    ModalMode::ModalMode(
        ModalModeValue::new(Box::new(sentences))
            .expect("a modal mode has a nonempty sentence sequence"),
    )
}

#[derive(Default)]
struct ModalVisitor(Vec<String>);

impl Visitor for ModalVisitor {
    fn visit_ability(&mut self, value: &Ability) {
        self.0.push("Ability".to_owned());
        deckmaste_english_v2::visit::walk_ability(self, value);
    }

    fn visit_plain(&mut self, value: &Plain) {
        self.0.push("Plain".to_owned());
        deckmaste_english_v2::visit::walk_plain(self, value);
    }

    fn visit_ability_body(&mut self, value: &AbilityBody) {
        self.0.push("AbilityBody".to_owned());
        deckmaste_english_v2::visit::walk_ability_body(self, value);
    }

    fn visit_plain_modal(&mut self, value: &PlainModal) {
        self.0.push("PlainModal".to_owned());
        deckmaste_english_v2::visit::walk_plain_modal(self, value);
    }

    fn visit_modal_chooser(&mut self, value: ModalChooser) {
        self.0.push(format!("ModalChooser:{value:?}"));
    }

    fn visit_modal_choice_bounds(&mut self, value: ModalChoiceBounds) {
        self.0.push(format!("ModalChoiceBounds:{value:?}"));
    }

    fn visit_modal_mode(&mut self, value: &ModalMode) {
        self.0.push("ModalMode".to_owned());
        deckmaste_english_v2::visit::walk_modal_mode(self, value);
    }

    fn visit_modal_mode_value(&mut self, value: &ModalModeValue) {
        self.0.push("ModalModeValue".to_owned());
        deckmaste_english_v2::visit::walk_modal_mode_value(self, value);
    }

    fn visit_sentence(&mut self, value: &Sentence) {
        self.0.push("Sentence".to_owned());
        deckmaste_english_v2::visit::walk_sentence(self, value);
    }

    fn visit_gain_life(&mut self, value: &GainLife) {
        let Amount::Number(NumberAmount {
            number: ScalarNumber { magnitude },
        }) = value.amount
        else {
            panic!("modal visitor expects a literal life amount")
        };
        self.0.push(format!("GainLife:{magnitude}"));
    }

    fn visit_intransitive_predicate(&mut self, value: &IntransitivePredicate) {
        assert!(matches!(
            &value.head,
            IntransitiveVerb::Declaration(head) if head.id().name() == "Connive"
        ));
        self.0.push("Connive".to_owned());
    }
}

#[test]
fn plain_modal_exact_ast_render_visitor_and_claims_are_hand_derived() {
    let parser = parser();
    let context = context("Context Card", false);
    let text = "Choose one —\n• You gain 1 life. You connive.\n• You gain 2 life.";
    let selected = assert_one_logic_candidate(&parser, &context, text);
    let expected = Ability::Plain(Plain {
        body: Box::new(AbilityBody::PlainModal(
            PlainModal::new(
                ModalChooser::You,
                ModalChoiceBounds::ExactlyOne,
                Box::new(vec![
                    modal_mode(vec![
                        modal_sentence(gain_life_predicate(1)),
                        modal_sentence(connive()),
                    ]),
                    modal_mode(vec![modal_sentence(gain_life_predicate(2))]),
                ]),
            )
            .expect("two nonempty modes and a licensed header construct"),
        )),
    });
    assert_eq!(selected, expected);
    assert_eq!(selected.render(&context, parser.environment()), text);

    let mut visitor = ModalVisitor::default();
    visitor.visit_ability(&selected);
    assert_eq!(
        visitor.0,
        [
            "Ability",
            "Plain",
            "AbilityBody",
            "PlainModal",
            "ModalChooser:You",
            "ModalChoiceBounds:ExactlyOne",
            "ModalMode",
            "ModalModeValue",
            "Sentence",
            "GainLife:1",
            "Sentence",
            "Connive",
            "ModalMode",
            "ModalModeValue",
            "Sentence",
            "GainLife:2",
        ],
        "visitor order is header semantics, then every mode and sentence in source order",
    );

    let analysis = parser.analyze(text, &context);
    let ownership = analysis.ownership().expect("the selected modal owns bytes");
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
            (0, 6, "vocab:ModalChooser/You"),
            (6, 10, "vocab:ModalChoiceBounds/ExactlyOne"),
            (10, 15, "form:plain_modal/exactly_one/2"),
            (15, 19, "form:modal_mode/modal_mode/0"),
            (19, 22, "vocab:SubjectPronoun/You"),
            (22, 27, "lexeme:VerbLexeme/Gain/bare"),
            (27, 29, "codec:ScalarNumber"),
            (29, 34, "form:gain_life/gain_life/2"),
            (34, 35, "structural:ModalModeValue/sentences/terminator/0"),
            (
                35,
                36,
                "structural:ModalModeValue/sentences/separator/uniform/0"
            ),
            (36, 39, "vocab:SubjectPronoun/You"),
            (39, 47, "lexeme:keyword_action/Connive/bare"),
            (47, 48, "structural:ModalModeValue/sentences/terminator/0"),
            (48, 49, "structural:PlainModal/modes/separator/uniform/0"),
            (49, 53, "form:modal_mode/modal_mode/0"),
            (53, 56, "vocab:SubjectPronoun/You"),
            (56, 61, "lexeme:VerbLexeme/Gain/bare"),
            (61, 63, "codec:ScalarNumber"),
            (63, 68, "form:gain_life/gain_life/2"),
            (68, 69, "structural:ModalModeValue/sentences/terminator/0"),
        ],
    );
}

#[test]
fn modal_header_guard_selection_is_mutation_authenticated() {
    let parser = parser();
    let context = context("Context Card", false);
    let modes = || {
        vec![
            modal_mode(vec![modal_sentence(gain_life_predicate(1))]),
            modal_mode(vec![modal_sentence(gain_life_predicate(2))]),
        ]
    };
    for (chooser, bounds, header) in [
        (
            ModalChooser::You,
            ModalChoiceBounds::ExactlyOne,
            "Choose one",
        ),
        (
            ModalChooser::You,
            ModalChoiceBounds::ExactlyTwo,
            "Choose two",
        ),
        (
            ModalChooser::You,
            ModalChoiceBounds::OneToTwo,
            "Choose one or both",
        ),
        (
            ModalChooser::You,
            ModalChoiceBounds::OneOrMore,
            "Choose one or more",
        ),
        (
            ModalChooser::You,
            ModalChoiceBounds::ZeroToOne,
            "Choose up to one",
        ),
        (
            ModalChooser::Opponent,
            ModalChoiceBounds::ExactlyOne,
            "An opponent chooses one",
        ),
    ] {
        let modal = PlainModal::new(chooser, bounds, Box::new(modes()))
            .expect("every admitted semantic header combination constructs");
        let ability = Ability::Plain(Plain {
            body: Box::new(AbilityBody::PlainModal(modal)),
        });
        assert_eq!(
            ability.render(&context, parser.environment()),
            modal_text(header),
            "mutating semantic chooser or bounds selects its one guarded surface",
        );
    }
    for bounds in [
        ModalChoiceBounds::ExactlyTwo,
        ModalChoiceBounds::OneToTwo,
        ModalChoiceBounds::OneOrMore,
        ModalChoiceBounds::ZeroToOne,
    ] {
        assert!(
            PlainModal::new(ModalChooser::Opponent, bounds, Box::new(modes())).is_none(),
            "the opponent chooser is sealed to exactly one mode",
        );
    }
    assert!(
        ModalModeValue::new(Box::default()).is_none(),
        "a mode cannot lose its last sentence",
    );
    assert!(
        PlainModal::new(
            ModalChooser::You,
            ModalChoiceBounds::ExactlyOne,
            Box::new(vec![modal_mode(vec![modal_sentence(gain_life_predicate(
                1
            ))])]),
        )
        .is_none(),
        "a modal group cannot lose its second mode",
    );
}

fn assert_ordinary_modal_failure(parser: &Parser, context: &ParseContext<'_>, text: &str) {
    let analysis = parser.analyze(text, context);
    assert!(
        analysis.selected().is_none(),
        "modal negative selected: {text}"
    );
    assert!(
        matches!(parser.parse(text, context), Err(ParseError::Failure { .. })),
        "modal negative must be an ordinary parse failure: {text}",
    );
}

fn wrap_modal_surface(envelope: &str, modal: &str) -> String {
    match envelope {
        "root" => modal.to_owned(),
        "trigger" => format!("Whenever a player connives, if you connive, {modal}"),
        "activation" => format!("{{T}}: {modal}"),
        _ => panic!("unknown modal envelope {envelope}"),
    }
}

fn reciprocal_modal_boundary_negatives(envelope: &str) -> Vec<String> {
    let header = if envelope == "trigger" { "choose one" } else { "Choose one" };
    let wrong_you_header = if envelope == "trigger" { "Choose one" } else { "choose one" };
    let wrong_opponent_header = if envelope == "trigger" {
        "An opponent chooses one"
    } else {
        "an opponent chooses one"
    };
    [
        format!("{wrong_you_header} —\n• You gain 1 life.\n• You gain 2 life."),
        format!("{wrong_opponent_header} —\n• You gain 1 life.\n• You gain 2 life."),
        format!("{header}  —\n• You gain 1 life.\n• You gain 2 life."),
        format!("{header} —\n•  You gain 1 life.\n• You gain 2 life."),
        format!("{header} —\n• You gain 1 life.  You connive.\n• You gain 2 life."),
        format!("{header} —\n• You gain 1 life.You connive.\n• You gain 2 life."),
        format!("{header} —\n• You gain 1 life. you connive.\n• You gain 2 life."),
        format!("{header} —\n• You gain 1 life.\n\n• You gain 2 life."),
        format!("{header} —\n• You gain 1 life.• You gain 2 life."),
        format!("{header} —\n• You gain 1 life. • You gain 2 life."),
        format!("{header} —\n• You gain 1 life.  • You gain 2 life."),
        format!("{header} —\n• You gain 1 life.\n• you gain 2 life."),
    ]
    .into_iter()
    .map(|modal| wrap_modal_surface(envelope, &modal))
    .collect()
}

#[test]
fn reciprocal_modal_boundary_mutations_reject_in_every_envelope() {
    let parser = parser();
    let context = context("Context Card", false);
    for envelope in ["root", "trigger", "activation"] {
        for text in reciprocal_modal_boundary_negatives(envelope) {
            assert_ordinary_modal_failure(&parser, &context, &text);
        }
    }
}

#[test]
fn reciprocal_modal_boundary_failure_spans_are_literal() {
    let parser = parser();
    let context = context("Context Card", false);
    let mut actual = Vec::new();
    for envelope in ["root", "trigger", "activation"] {
        for (case, text) in reciprocal_modal_boundary_negatives(envelope)
            .into_iter()
            .enumerate()
        {
            let Err(ParseError::Failure { span, .. }) = parser.parse(&text, &context) else {
                panic!("boundary mutation is an ordinary parse failure: {text}")
            };
            actual.push((envelope, case, span));
        }
    }
    assert_eq!(
        actual,
        [
            ("root", 0, TextSpan { start: 0, end: 6 }),
            ("root", 1, TextSpan { start: 0, end: 2 }),
            ("root", 2, TextSpan { start: 12, end: 15 }),
            ("root", 3, TextSpan { start: 20, end: 23 }),
            ("root", 4, TextSpan { start: 37, end: 40 }),
            ("root", 5, TextSpan { start: 35, end: 38 }),
            ("root", 6, TextSpan { start: 36, end: 39 }),
            ("root", 7, TextSpan { start: 37, end: 40 }),
            ("root", 8, TextSpan { start: 35, end: 38 }),
            ("root", 9, TextSpan { start: 36, end: 39 }),
            ("root", 10, TextSpan { start: 37, end: 40 }),
            ("root", 11, TextSpan { start: 40, end: 43 }),
            ("trigger", 0, TextSpan { start: 44, end: 50 }),
            ("trigger", 1, TextSpan { start: 44, end: 46 }),
            ("trigger", 2, TextSpan { start: 56, end: 59 }),
            ("trigger", 3, TextSpan { start: 64, end: 67 }),
            ("trigger", 4, TextSpan { start: 81, end: 84 }),
            ("trigger", 5, TextSpan { start: 79, end: 82 }),
            ("trigger", 6, TextSpan { start: 80, end: 83 }),
            ("trigger", 7, TextSpan { start: 81, end: 84 }),
            ("trigger", 8, TextSpan { start: 79, end: 82 }),
            ("trigger", 9, TextSpan { start: 80, end: 83 }),
            ("trigger", 10, TextSpan { start: 81, end: 84 }),
            ("trigger", 11, TextSpan { start: 84, end: 87 }),
            ("activation", 0, TextSpan { start: 5, end: 11 }),
            ("activation", 1, TextSpan { start: 5, end: 7 }),
            ("activation", 2, TextSpan { start: 17, end: 20 }),
            ("activation", 3, TextSpan { start: 25, end: 28 }),
            ("activation", 4, TextSpan { start: 42, end: 45 }),
            ("activation", 5, TextSpan { start: 40, end: 43 }),
            ("activation", 6, TextSpan { start: 41, end: 44 }),
            ("activation", 7, TextSpan { start: 42, end: 45 }),
            ("activation", 8, TextSpan { start: 40, end: 43 }),
            ("activation", 9, TextSpan { start: 41, end: 44 }),
            ("activation", 10, TextSpan { start: 42, end: 45 }),
            ("activation", 11, TextSpan { start: 45, end: 48 }),
        ],
        "each reciprocal boundary mutation has a stable exact failure span",
    );
}

#[test]
fn plain_modal_rejects_every_deferred_or_malformed_surface_as_an_ordinary_failure() {
    let parser = parser();
    let context = context("Context Card", false);
    for text in [
        "Choose one —",
        "Choose one —\n",
        "Choose one —\n• You gain 1 life.",
        "Choose one -\n• You gain 1 life.\n• You gain 2 life.",
        "Choose one --\n• You gain 1 life.\n• You gain 2 life.",
        "Choose one –\n• You gain 1 life.\n• You gain 2 life.",
        "Choose one—\n• You gain 1 life.\n• You gain 2 life.",
        "Choose one — • You gain 1 life.\n• You gain 2 life.",
        "Choose one —\n•You gain 1 life.\n• You gain 2 life.",
        "Choose one —\n* You gain 1 life.\n• You gain 2 life.",
        "Choose one —\n• You gain 1 life\n• You gain 2 life.",
        "Choose one —\n• You gain 1 life. • You gain 2 life.",
        "Choose one —\n• You gain 1 life.\n• you gain 2 life.",
        "Choose one —\n• First — You gain 1 life.\n• Second — You gain 2 life.",
        "Choose up to five {P} worth of modes.\n{P} — You gain 1 life.\n{P} — You gain 2 life.",
        "Choose one. If this spell was kicked, choose any number instead.\n• You gain 1 life.\n• You gain 2 life.",
        "Choose one or more —\n• You gain 1 life.\n• You gain 2 life.\nYou may choose the same mode more than once.",
        "Escalate {1}\nChoose one —\n• You gain 1 life.\n• You gain 2 life.",
        "Landfall — Choose one —\n• You gain 1 life.\n• You gain 2 life.",
        "Choose one —\n• You gain 1 life. (You really do.)\n• You gain 2 life.",
        "I — Choose one —\n• You gain 1 life.\n• You gain 2 life.",
    ] {
        assert_ordinary_modal_failure(&parser, &context, text);
    }
}

#[test]
fn modal_self_reference_licensing_is_identical_in_every_envelope() {
    let parser = parser();
    let legendary = context("Aang, A Lot to Learn", true);
    let ordinary = context("Grizzly Bears", false);
    for (envelope, legendary_header, ordinary_header) in [
        ("root", "Choose one", "Choose one"),
        ("trigger", "choose one", "choose one"),
        ("activation", "Choose one", "Choose one"),
    ] {
        let legendary_text = wrapped_modal_text(envelope, legendary_header)
            .replace("You gain 1 life", "Aang gains 1 life")
            .replace("You gain 2 life", "Aang gains 2 life");
        let selected = assert_one_logic_candidate(&parser, &legendary, &legendary_text);
        let mut visitor = SelfReferenceVisitor::default();
        visitor.visit_ability(&selected);
        assert_eq!(
            visitor.spellings,
            [
                SelfReferenceSpelling::Abbreviated,
                SelfReferenceSpelling::Abbreviated,
            ],
            "exact Legendary metadata licenses both modal references: {legendary_text}",
        );

        let ordinary_text = wrapped_modal_text(envelope, ordinary_header)
            .replace("You gain 1 life", "Grizzly gains 1 life")
            .replace("You gain 2 life", "Grizzly gains 2 life");
        assert_ordinary_modal_failure(&parser, &ordinary, &ordinary_text);
    }
}

fn full_self_reference_subject(context: &ParseContext<'_>) -> Subject {
    let reference = SourceSelfReference::new(SelfReferenceSpelling::Full, context)
        .expect("every nonempty context licenses its exact full spelling");
    Subject::SubjectNominal(NominalSubject {
        value: NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: NumericStage::UnqualifiedNumericStage(UnqualifiedNumericStage {
                reference: ZoneStage::UnqualifiedZoneStage(UnqualifiedZoneStage {
                    reference: ControllerStage::UnqualifiedControllerStage(
                        UnqualifiedControllerStage {
                            reference: UnqualifiedReference::SelfReference(reference),
                        },
                    ),
                }),
            }),
        }),
    })
}

fn full_self_reference_modal_body(context: &ParseContext<'_>) -> AbilityBody {
    let sentence = |magnitude| {
        Sentence::Declarative(Declarative {
            clause: Box::new(Clause::Finite(Box::new(plain_finite(
                full_self_reference_subject(context),
                Predicate::Atomic(Box::new(gain_life_predicate(magnitude))),
            )))),
        })
    };
    AbilityBody::PlainModal(
        PlainModal::new(
            ModalChooser::You,
            ModalChoiceBounds::ExactlyOne,
            Box::new(vec![
                modal_mode(vec![sentence(1)]),
                modal_mode(vec![sentence(2)]),
            ]),
        )
        .expect("two full-self-reference modes construct"),
    )
}

#[test]
fn nonlegendary_full_self_reference_selects_complete_modal_envelopes() {
    let parser = parser();
    let ordinary = context("Grizzly Bears", false);
    for (envelope, header) in [
        ("root", "Choose one"),
        ("trigger", "choose one"),
        ("activation", "Choose one"),
    ] {
        let text = wrapped_modal_text(envelope, header)
            .replace("You gain 1 life", "Grizzly Bears gains 1 life")
            .replace("You gain 2 life", "Grizzly Bears gains 2 life");
        let selected = assert_one_logic_candidate(&parser, &ordinary, &text);
        assert_eq!(
            selected,
            expected_modal_envelope_with_body(envelope, full_self_reference_modal_body(&ordinary),),
            "the complete {envelope} envelope retains full-only self-reference ASTs",
        );
        assert_eq!(
            selected.render(&ordinary, parser.environment()),
            text,
            "the complete full-name envelope renders identically",
        );
        let mut visitor = SelfReferenceVisitor::default();
        visitor.visit_ability(&selected);
        assert_eq!(
            visitor.spellings,
            [SelfReferenceSpelling::Full, SelfReferenceSpelling::Full],
            "the nonlegendary modal body observes only the full spelling: {text}",
        );
    }
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
