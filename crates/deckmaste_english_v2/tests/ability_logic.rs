use std::collections::BTreeSet;
use std::num::NonZeroU32;
use std::path::Path;
use std::process::Command;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::CoreVerbIdentity;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::environment::VerbInventoryRef;
use deckmaste_english_v2::parser::LexicalProvenanceKind;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::parser::TextSpan;
use deckmaste_english_v2::render::Render as _;
use deckmaste_english_v2::visit::Visitor;

fn environment() -> ParserEnvironment {
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
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
    let AbilityBody::Sentences(sentences) = body else {
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
        VerbInventoryRef::Declaration(DeclarationId::new(
            DeclarationKind::KeywordAction,
            "Connive",
        )),
    )
    .expect("the builtin grammar declares intransitive Connive");
    VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
        frame: Box::new(LexicalVerbPhrase::IntransitiveLexicalVerbPhrase(
            IntransitiveLexicalVerbPhrase::IntransitivePredicate(IntransitivePredicate { head }),
        )),
    })
}

fn declared_action_name(predicate: &VerbPhrase) -> Option<&str> {
    match predicate {
        VerbPhrase::BaseVerbPhrase(BaseVerbPhrase { frame }) => match frame.as_ref() {
            LexicalVerbPhrase::IntransitiveLexicalVerbPhrase(
                IntransitiveLexicalVerbPhrase::IntransitivePredicate(IntransitivePredicate {
                    head,
                }),
            ) => match head.reference() {
                VerbInventoryRef::Declaration(id) => Some(id.name()),
                VerbInventoryRef::Core(_) => None,
            },
            LexicalVerbPhrase::TransitiveLexicalVerbPhrase(transitive_lexical_verb_phrase) => {
                match transitive_lexical_verb_phrase.as_ref() {
                    TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate {
                        head,
                        ..
                    }) => match head.reference() {
                        VerbInventoryRef::Declaration(id) => Some(id.name()),
                        VerbInventoryRef::Core(_) => None,
                    },
                }
            }
            LexicalVerbPhrase::MeasureComplementLexicalVerbPhrase(
                MeasureComplementLexicalVerbPhrase::MeasureComplementPredicate(
                    MeasureComplementPredicate { head, .. },
                ),
            ) => match head.reference() {
                VerbInventoryRef::Declaration(id) => Some(id.name()),
                VerbInventoryRef::Core(_) => None,
            },
            _ => None,
        },
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
    let AbilityBody::Sentences(plain_sentences) = body else {
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
    let AbilityBody::Sentences(consequences) = body else {
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
    fn visit_determined_nominal(&mut self, value: &DeterminedNominal) {
        self.0.push("DeterminedNominal");
        deckmaste_english_v2::visit::walk_determined_nominal(self, value);
    }

    fn visit_determinative(&mut self, value: &Determinative) {
        self.0.push("Determinative");
        deckmaste_english_v2::visit::walk_determinative(self, value);
    }

    fn visit_nominal(&mut self, value: &Nominal) {
        self.0.push("Nominal");
        deckmaste_english_v2::visit::walk_nominal(self, value);
    }

    fn visit_nominal_coordination(&mut self, value: &NominalCoordination) {
        self.0.push("NominalCoordination");
        deckmaste_english_v2::visit::walk_nominal_coordination(self, value);
    }

    fn visit_or_nominal_coordination(&mut self, value: &OrNominalCoordination) {
        self.0.push("OrNominalCoordination");
        deckmaste_english_v2::visit::walk_or_nominal_coordination(self, value);
    }

    fn visit_and_or_nominal_coordination(&mut self, value: &AndOrNominalCoordination) {
        self.0.push("AndOrNominalCoordination");
        deckmaste_english_v2::visit::walk_and_or_nominal_coordination(self, value);
    }

    fn visit_full_or_noun_phrase_coordination(&mut self, value: &FullOrNounPhraseCoordination) {
        self.0.push("FullOrNounPhraseCoordination");
        deckmaste_english_v2::visit::walk_full_or_noun_phrase_coordination(self, value);
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

    fn visit_genitive_determiner_reference(&mut self, value: &GenitiveDeterminerReference) {
        self.0.push("GenitiveDeterminerReference");
        deckmaste_english_v2::visit::walk_genitive_determiner_reference(self, value);
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "the contract compares the complete coordination and exclusion structural family"
)]
fn finite_subject_coordination_and_exclusion_are_linguistic_structure() {
    let parser = parser();
    let context = context("Context Card", false);
    let witnesses = [
        (
            "Whenever a spell or ability deals 1 damage to you, you gain 1 life.",
            "UnqualifiedReferenceDeterminedNominal",
            &[
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "NominalCoordination",
                "OrNominalCoordination",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
            ][..],
            &[
                "determinative:DeterminativeHead/IndefiniteArticle",
                "structural:OrNominalCoordination/members/separator/pair/0",
            ][..],
        ),
        (
            "Whenever this creature or another creature you control deals 1 damage to you, you gain 1 life.",
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            &[
                "FullOrNounPhraseCoordination",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
            ][..],
            &[
                "structural:FullOrNounPhraseCoordination/members/separator/pair/0",
                "determinative:DeterminativeHead/ProximalDemonstrative",
                "determinative:DeterminativeHead/Another",
            ][..],
        ),
        (
            "Whenever an artifact or enchantment deals 1 damage to you, you gain 1 life.",
            "UnqualifiedReferenceDeterminedNominal",
            &[
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "NominalCoordination",
                "OrNominalCoordination",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
            ][..],
            &[
                "determinative:DeterminativeHead/IndefiniteArticle",
                "structural:OrNominalCoordination/members/separator/pair/0",
            ][..],
        ),
        (
            "Whenever this creature or another creature deals 1 damage to you, you gain 1 life.",
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            &[
                "FullOrNounPhraseCoordination",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
            ][..],
            &["structural:FullOrNounPhraseCoordination/members/separator/pair/0"][..],
        ),
        (
            "Whenever this creature or another Warrior deals 1 damage to you, you gain 1 life.",
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            &[
                "FullOrNounPhraseCoordination",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
            ][..],
            &["structural:FullOrNounPhraseCoordination/members/separator/pair/0"][..],
        ),
        (
            "Whenever another Villain and/or artifact deals 1 damage to you, you gain 1 life.",
            "UnqualifiedReferenceDeterminedNominal",
            &[
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "NominalCoordination",
                "AndOrNominalCoordination",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
            ][..],
            &[
                "determinative:DeterminativeHead/Another",
                "structural:AndOrNominalCoordination/members/separator/pair/0",
            ][..],
        ),
        (
            "Whenever a player other than this creature's owner deals 1 damage to you, you gain 1 life.",
            "PostmodifiedReferenceOtherThanQualifiedReference",
            &[
                "OtherThanQualifiedReference",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "GenitiveDeterminerReference",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
            ][..],
            &[
                "form:other_than_qualified_reference/other_than_qualified_reference/1",
                "form:other_than_qualified_reference/other_than_qualified_reference/2",
                "form:possessive/singular/0/affix",
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
        assert!(
            matches!(
                decision.resolution(),
                SelectionResolution::Unique | SelectionResolution::Specificity
            ),
            "{text}: {decision:?}",
        );
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
fn existential_there_preserves_its_pivot_before_the_predicate_boundary() {
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
    let Some(ConditionClause::FiniteCondition(finite_condition)) = intervening_if.as_ref().as_ref()
    else {
        panic!("the intervening condition owns an existential clause")
    };
    let FiniteCondition::FiniteCondition(condition) = finite_condition.as_ref();
    let Clause::Finite(finite) = condition.clause.as_ref() else {
        panic!("the intervening condition owns a finite clause")
    };
    let FiniteClause::ExistentialFiniteClause(existential) = finite.as_ref() else {
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
    assert!(matches!(
        decision.resolution(),
        SelectionResolution::Unique | SelectionResolution::Specificity
    ));
    assert!(decision.exception_uses().is_empty());
    let ownership = analysis
        .ownership()
        .expect("the controlled existential witness owns its bytes");
    assert!(ownership.failures().is_empty(), "{ownership:?}");
    assert!(ownership.summary().covered(), "{ownership:?}");
    assert_eq!(ownership.rendered_text(), controlled);

    let construction_path = decision.candidates()[0].construction_path();
    for required in [
        "FiniteConditionFiniteCondition",
        "FiniteClauseExistentialFiniteClause",
        "UnqualifiedReferenceDeterminedNominal",
        "NominalModifiedPluralNominal",
        "NominalModifierSupertypeModifier",
        "NominalModifierNounModifier",
        "PrepositionalPhrasePrepositionalPhrase",
        "PostmodifiedReferenceRelativeQualifiedReference",
        "PositiveObjectGapRelativeClausePositiveObjectGapRelative",
        "UnqualifiedReferenceDeterminedNominal",
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
            "form:finite_condition/finite_condition/0",
        ),
        (
            TextSpan { start: 44, end: 50 },
            "form:existential_finite_clause/existential_finite_clause/0",
        ),
        (TextSpan { start: 50, end: 54 }, "vocab:FiniteCopula/Are"),
        (TextSpan { start: 84, end: 90 }, "vocab:Preposition/Among"),
        (TextSpan { start: 67, end: 73 }, "vocab:Supertype/Basic"),
        (TextSpan { start: 73, end: 78 }, "lexeme:type/Land/singular"),
        (
            TextSpan { start: 78, end: 84 },
            "lexeme:CommonNoun/Type/plural",
        ),
        (
            TextSpan {
                start: 96,
                end: 101,
            },
            "determinative:DeterminativeHead/DistalDemonstrative",
        ),
        (
            TextSpan {
                start: 108,
                end: 117,
            },
            "core-verb:Control",
        ),
        (
            TextSpan {
                start: 117,
                end: 118,
            },
            "form:finite_condition/finite_condition/2",
        ),
    ] {
        assert!(
            claims.iter().any(|actual| actual == &expected),
            "{expected:?}: {claims:?}"
        );
    }

    let original = "At the beginning of each player's upkeep, if there are two or more basic land types among lands that player controls, this artifact deals 5 damage to that player.";
    let original_analysis = parser.analyze(original, &context);
    assert_eq!(
        original_analysis.outcome(),
        deckmaste_english_v2::parser::ParseAnalysisOutcome::Selected,
        "the already-supported consequence keeps the repaired condition row selected",
    );
    assert!(matches!(
        original_analysis.decision().unwrap().resolution(),
        SelectionResolution::Unique | SelectionResolution::Specificity
    ));
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

    fn visit_existential_finite_clause(&mut self, value: &ExistentialFiniteClause) {
        self.0.push("ExistentialFiniteClause");
        deckmaste_english_v2::visit::walk_existential_finite_clause(self, value);
    }

    fn visit_modified_plural_nominal(&mut self, value: &ModifiedPluralNominal) {
        self.0.push("ModifiedPluralNominal");
        deckmaste_english_v2::visit::walk_modified_plural_nominal(self, value);
    }

    fn visit_supertype_modifier(&mut self, value: &SupertypeModifier) {
        self.0.push("SupertypeModifier");
        deckmaste_english_v2::visit::walk_supertype_modifier(self, value);
    }

    fn visit_noun_modifier(&mut self, value: &NounModifier) {
        self.0.push("NounModifier");
        deckmaste_english_v2::visit::walk_noun_modifier(self, value);
    }

    fn visit_prepositional_phrase(&mut self, value: &PrepositionalPhrase) {
        self.0.push("PrepositionalPhrase");
        deckmaste_english_v2::visit::walk_prepositional_phrase(self, value);
    }

    fn visit_prepositional_phrase_value(&mut self, value: &PrepositionalPhraseValue) {
        self.0.push("PrepositionalPhraseValue");
        deckmaste_english_v2::visit::walk_prepositional_phrase_value(self, value);
    }

    fn visit_relative_qualified_reference(&mut self, value: &RelativeQualifiedReference) {
        self.0.push("RelativeQualifiedReference");
        deckmaste_english_v2::visit::walk_relative_qualified_reference(self, value);
    }

    fn visit_positive_object_gap_relative_clause(
        &mut self,
        value: &PositiveObjectGapRelativeClause,
    ) {
        self.0.push("PositiveObjectGapRelative");
        deckmaste_english_v2::visit::walk_positive_object_gap_relative_clause(self, value);
    }

    fn visit_determined_nominal(&mut self, value: &DeterminedNominal) {
        self.0.push("DeterminedNominal");
        deckmaste_english_v2::visit::walk_determined_nominal(self, value);
    }

    fn visit_determinative(&mut self, value: &Determinative) {
        self.0.push("Determinative");
        deckmaste_english_v2::visit::walk_determinative(self, value);
    }

    fn visit_nominal(&mut self, value: &Nominal) {
        self.0.push("Nominal");
        deckmaste_english_v2::visit::walk_nominal(self, value);
    }
}

#[test]
fn existential_there_derives_be_concord_class_and_visits_the_complete_structure() {
    let parser = parser();
    let context = context("Context Card", false);
    let cases = [
        (
            "At the beginning of upkeep, if there is an artifact, you gain 1 life.",
            "vocab:FiniteCopula/Is",
        ),
        (
            "At the beginning of upkeep, if there are artifacts, you gain 1 life.",
            "vocab:FiniteCopula/Are",
        ),
    ];

    for (text, be_claim) in cases {
        let analysis = parser.analyze(text, &context);
        assert_eq!(
            analysis.outcome(),
            deckmaste_english_v2::parser::ParseAnalysisOutcome::Selected,
            "{text}"
        );
        assert!(
            matches!(
                analysis.decision().unwrap().resolution(),
                SelectionResolution::Unique | SelectionResolution::Specificity
            ),
            "{text}"
        );
        let ownership = analysis
            .ownership()
            .expect("existential concord_class witness ownership");
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
            "PrepositionalPhrase",
            "PrepositionalPhraseValue",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "PrepositionalPhrase",
            "PrepositionalPhraseValue",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "Nominal",
            "ConditionClause",
            "ExistentialFiniteClause",
            "RelativeQualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "ModifiedPluralNominal",
            "SupertypeModifier",
            "NounModifier",
            "PrepositionalPhrase",
            "PrepositionalPhraseValue",
            "DeterminedNominal",
            "Nominal",
            "PositiveObjectGapRelative",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
        ],
    );
}

#[test]
fn existential_there_rejects_malformed_concord_class_capitalization_spacing_and_pivots() {
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

    fn visit_prepositional_phrase(&mut self, value: &PrepositionalPhrase) {
        self.0.push("PrepositionalPhrase");
        deckmaste_english_v2::visit::walk_prepositional_phrase(self, value);
    }

    fn visit_prepositional_phrase_value(&mut self, value: &PrepositionalPhraseValue) {
        self.0.push("PrepositionalPhraseValue");
        deckmaste_english_v2::visit::walk_prepositional_phrase_value(self, value);
    }

    fn visit_trigger_marker(&mut self, _value: TriggerMarker) {
        self.0.push("TriggerMarker");
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
            (8, 10, "determinative:DeterminativeHead/IndefiniteArticle"),
            (10, 17, "lexeme:CommonNoun/Player/singular"),
            (
                17,
                26,
                "lexeme:keyword_action/Connive/third_person_singular"
            ),
            (26, 27, "form:triggered/triggered/1"),
            (27, 31, "vocab:SubjectPronoun/You"),
            (31, 36, "core-verb:Gain"),
            (36, 38, "vocab:Variable/X"),
            (38, 43, "lexeme:CommonNoun/Life/singular"),
            (43, 44, "structural:Sentences/sentences/terminator/0"),
            (44, 45, "structural:Sentences/sentences/separator/uniform/0"),
            (45, 52, "lexeme:keyword_action/Destroy/bare"),
            (52, 59, "vocab:TargetingMarker/Target"),
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
    assert!(matches!(body, AbilityBody::Sentences(_)));
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
    assert!(matches!(body, AbilityBody::Sentences(_)));
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
fn finite_trigger_predicate_concord_class_is_derived_from_its_subject() {
    let parser = parser();
    let context = context("Context Card", false);
    let text = "Whenever you connive, you gain X life.";
    let parsed = parser
        .parse(text, &context)
        .expect("second-person trigger subject requires bare predicate concord_class");
    assert_eq!(parsed.render(&context, parser.environment()), text);
    assert!(
        parser
            .parse("Whenever you connives, you gain X life.", &context)
            .is_err(),
        "third-person-singular concord_class must reject a second-person trigger subject"
    );
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
        trigger: TriggerPrefix::Temporal(temporal),
        intervening_if,
        ..
    }) = temporal
    else {
        panic!("At takes the dedicated temporal phrase")
    };
    assert!(intervening_if.as_ref().is_none());
    let PrepositionalPhrase::PrepositionalPhrase(phrase) = temporal.phrase() else {
        panic!("the temporal trigger takes an ordinary prepositional complement")
    };
    assert!(matches!(phrase.preposition, Preposition::At));
    assert!(matches!(
        phrase.complement.as_ref(),
        PrepositionalComplement::Object(_)
    ));

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
fn at_phrase_composes_temporal_nouns_through_the_general_noun_phrase_grammar() {
    let parser = parser();
    let context = context("Context Card", false);

    for text in [
        "At the beginning of turn, you gain X life.",
        "At the beginning of beginning phase, you gain X life.",
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
        "At your upkeep, you gain X life.",
        "At end of turn, you gain X life.",
        "At the end of combat, you gain X life.",
        "At end of your combat, you gain X life.",
        "At the beginning of an opponent's upkeep, you gain X life.",
        "At the beginning of each of your upkeeps, you gain X life.",
        "At the beginning of upkeep on your turn, you gain X life.",
        "At the beginning of draw step on each opponent's turn, you gain X life.",
        "At the beginning of your upkeep on your turn, you gain X life.",
        "At end of combat on your turn, you gain X life.",
        "At the beginning of each of your combats on your turn, you gain X life.",
    ] {
        assert_selected_trigger(&parser, &context, text);
    }

    for invalid in [
        "At the beginning of luncheon, you gain X life.",
        "At the beginning of each of your upkeep, you gain X life.",
        "At the beginning of each of your combat on your turn, you gain X life.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "unknown words and singular `each of` wholes remain ungrammatical: {invalid}"
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
            (0, 2, "vocab:Preposition/At"),
            (2, 6, "determinative:DeterminativeHead/DefiniteArticle"),
            (6, 16, "lexeme:CommonNoun/Beginning/singular"),
            (16, 19, "vocab:Preposition/Of"),
            (19, 24, "determinative:DeterminativeHead/Each"),
            (24, 31, "lexeme:CommonNoun/Player/singular"),
            (31, 33, "form:possessive/singular/0/affix"),
            (33, 43, "lexeme:turn_part/DrawStep/singular"),
            (43, 44, "form:triggered/triggered/1"),
            (44, 48, "vocab:SubjectPronoun/You"),
            (48, 53, "core-verb:Gain"),
            (53, 55, "vocab:Variable/X"),
            (55, 60, "lexeme:CommonNoun/Life/singular"),
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
            "PrepositionalPhrase",
            "PrepositionalPhraseValue",
            "PrepositionalPhrase",
            "PrepositionalPhraseValue",
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
            (8, 10, "determinative:DeterminativeHead/IndefiniteArticle"),
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
            (47, 52, "core-verb:Gain"),
            (52, 54, "vocab:Variable/X"),
            (54, 59, "lexeme:CommonNoun/Life/singular"),
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
            "PrepositionalPhrase",
            "PrepositionalPhraseValue",
            "PrepositionalPhrase",
            "PrepositionalPhraseValue",
            "PrepositionalPhrase",
            "PrepositionalPhraseValue",
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
    let decision = analysis
        .decision()
        .expect("a selected activation has a selection decision");
    assert_eq!(decision.candidates().len(), 1, "{text}: {decision:#?}");
    assert_eq!(decision.resolution(), SelectionResolution::Unique);
    assert_eq!(decision.survivors(), [0], "{text}: {decision:#?}");
    assert_eq!(decision.selected(), Some(0), "{text}: {decision:#?}");
    assert!(decision.exception_uses().is_empty(), "{text}");
    assert_eq!(selected.render(context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("a selected activation has byte ownership");
    assert!(ownership.failures().is_empty(), "{text}: {ownership:?}");
    assert!(ownership.summary().covered(), "{text}: {ownership:?}");
    selected.clone()
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
        "", "00", "01", "+2", "−2", "H", "E", "TK", "∞", "Y", "Z", "U/W", "B/W", "B/U", "R/U",
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
    let decision = analysis
        .decision()
        .expect("selected mixed activation has a decision");
    assert_eq!(decision.candidates().len(), 1, "{decision:#?}");
    assert_eq!(decision.survivors(), [0], "{decision:#?}");
    assert_eq!(decision.selected(), Some(0), "{decision:#?}");
    assert_eq!(decision.resolution(), SelectionResolution::Unique);
    assert!(decision.exception_uses().is_empty());
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
    let AbilityBody::Sentences(sentences) = &activated.body else {
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
            (28, 35, "vocab:TargetingMarker/Target"),
            (35, 44, "lexeme:type/Creature/singular"),
            (44, 46, "form:activated/activated/1"),
            (46, 49, "vocab:SubjectPronoun/You"),
            (49, 54, "core-verb:Gain"),
            (54, 56, "vocab:Variable/X"),
            (56, 61, "lexeme:CommonNoun/Life/singular"),
            (61, 62, "structural:Sentences/sentences/terminator/0"),
            (62, 63, "structural:Sentences/sentences/separator/uniform/0"),
            (63, 70, "lexeme:keyword_action/Destroy/bare"),
            (70, 77, "vocab:TargetingMarker/Target"),
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
            CostVisit::Node("TransitivePredicate"),
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
    gain_life(Amount::Number(NumberAmount {
        number: ScalarNumber { magnitude },
    }))
}

fn gain_life(amount: Amount) -> VerbPhrase {
    let environment = environment();
    let head = DeclarationTransitiveVerb::new(
        &environment,
        VerbInventoryRef::Core(CoreVerbIdentity::Gain),
    )
    .expect("the core inventory declares Gain with a transitive frame");
    let determiner = Determiner::Headed(Determinative::MassQuantityDeterminer(
        MassQuantityDeterminer { amount },
    ));
    let nominal = Nominal::MassNominal(MassNominal {
        noun: MassNoun::MassNoun(
            MassNounValue::new(Noun::Lexeme(CommonNoun::Life))
                .expect("life is declared as a mass noun"),
        ),
    });
    let reference = UnqualifiedReference::DeterminedNominal(
        DeterminedNominal::new(determiner, nominal)
            .expect("a quantity determiner licenses the mass noun life"),
    );
    let object = Object::ObjectNominal(
        NominalObject::new(Box::new(NounPhrase::QualifiedNounPhrase(
            QualifiedNounPhrase {
                reference: Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
                    UnqualifiedPostmodifiedReference {
                        reference: Box::new(reference),
                    },
                )),
            },
        )))
        .expect("ordinary noun phrase is licensed as an object"),
    );
    VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
        frame: Box::new(LexicalVerbPhrase::TransitiveLexicalVerbPhrase(Box::new(
            TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate { head, object }),
        ))),
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
            .expect("the helper supplies matching subject-predicate concord_class"),
    )
}

fn predicate_identity(predicate: &VerbPhrase) -> String {
    if declared_action_name(predicate) == Some("Connive") {
        return "connive".to_owned();
    }
    if let Some(magnitude) = gain_life_magnitude(predicate) {
        return format!("gain:{magnitude}");
    }
    panic!("unexpected coordination predicate payload: {predicate:?}")
}

fn coordinated_predicate_identity(predicate: &CoordinatedPredicate) -> String {
    let CoordinatedPredicate::Atomic(predicate) = predicate else {
        panic!("legacy coordination witness has an atomic predicate payload: {predicate:?}")
    };
    predicate_identity(predicate)
}

fn unqualified_reference(noun_phrase: &NounPhrase) -> &UnqualifiedReference {
    let NounPhrase::QualifiedNounPhrase(qualified) = noun_phrase else {
        panic!("coordination subject uses the exact unqualified staging")
    };
    let PostmodifiedReference::UnqualifiedPostmodifiedReference(reference) =
        qualified.reference.as_ref()
    else {
        panic!("coordination subject uses the exact unmodified reference")
    };
    &reference.reference
}

fn gain_life_magnitude(predicate: &VerbPhrase) -> Option<u32> {
    let VerbPhrase::BaseVerbPhrase(BaseVerbPhrase { frame }) = predicate else {
        return None;
    };
    let LexicalVerbPhrase::TransitiveLexicalVerbPhrase(frame) = frame.as_ref() else {
        return None;
    };
    let TransitiveLexicalVerbPhrase::TransitivePredicate(TransitivePredicate { head, object }) =
        frame.as_ref();
    if head.reference() != &VerbInventoryRef::Core(CoreVerbIdentity::Gain) {
        return None;
    }
    let Object::ObjectNominal(object) = object else {
        return None;
    };
    let UnqualifiedReference::DeterminedNominal(determined) = unqualified_reference(object.value())
    else {
        return None;
    };
    let Determiner::Headed(Determinative::MassQuantityDeterminer(MassQuantityDeterminer {
        amount:
            Amount::Number(NumberAmount {
                number: ScalarNumber { magnitude },
            }),
    })) = determined.det()
    else {
        return None;
    };
    matches!(determined.nominal, Nominal::MassNominal(_)).then_some(*magnitude)
}

fn is_indefinite_player(reference: &UnqualifiedReference) -> bool {
    let UnqualifiedReference::DeterminedNominal(determined) = reference else {
        return false;
    };
    let Determiner::Headed(Determinative::SingularSimpleDeterminative(det)) = determined.det()
    else {
        return false;
    };
    det.head == DeterminativeHead::Closed(DeterminativeHeadLemma::IndefiniteArticle)
        && matches!(
            &determined.nominal,
            Nominal::BareSingularNominal(nominal)
                if matches!(nominal.head(), Head::NounSingularHead(head)
                    if head.noun() == &Noun::Lexeme(CommonNoun::Player))
        )
}

fn indefinite_player_reference() -> UnqualifiedReference {
    let det = Determinative::SingularSimpleDeterminative(SingularSimpleDeterminative {
        head: DeterminativeHead::Closed(DeterminativeHeadLemma::IndefiniteArticle),
    });
    let nominal = Nominal::BareSingularNominal(
        BareSingularNominal::new(Head::NounSingularHead(
            NounSingularHead::new(Noun::Lexeme(CommonNoun::Player))
                .expect("player is a count noun"),
        ))
        .expect("the Singular nominal accepts a Singular Head"),
    );
    UnqualifiedReference::DeterminedNominal(
        DeterminedNominal::new(Determiner::Headed(det), nominal)
            .expect("an indefinite determiner agrees with a singular player nominal"),
    )
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
            reference if is_indefinite_player(reference) => "player",
            other => panic!("unexpected nominal coordination subject payload: {other:?}"),
        },
        other @ (Subject::SubjectPronoun(_) | Subject::VariableSubject(_)) => {
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

fn coordinated_clause_identity(clause: &CoordinatedClause) -> String {
    let CoordinatedClause::Finite(clause) = clause;
    finite_clause_identity(clause)
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

/// Like [`assert_one_logic_candidate`], but for surfaces whose head is ordinary
/// sentence grammar, where a preposed clause can attach either to the head or
/// to the enclosing trigger. Selection must still resolve to exactly one
/// reading; only the materialized candidate count is allowed to exceed one.
fn assert_resolved_logic_candidate(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
) -> Ability {
    let analysis = parser.analyze(text, context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("logic witness must select: {text}: {analysis:?}"));
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

    for (surface, head_path) in [
        ("may", "AuxiliaryHeadInventoryAuxiliary"),
        ("can", "AuxiliaryHeadInventoryAuxiliary"),
        ("can't", "AuxiliaryHeadInventoryAuxiliary"),
        ("must", "AuxiliaryHeadInventoryAuxiliary"),
    ] {
        let text = format!("You {surface} gain 2 life.");
        let selected = assert_one_logic_candidate(&parser, &context, &text);
        let Clause::Finite(finite) = one_declarative_clause(&selected) else {
            panic!("the auxiliary inhabits an ordinary finite clause")
        };
        let FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
            panic!("the auxiliary is a predicate inside the shared finite clause")
        };
        assert!(matches!(clause.predicate(), Predicate::Auxiliary(_)));
        let analysis = parser.analyze(&text, &context);
        let path = analysis.decision().unwrap().candidates()[0].construction_path();
        assert!(
            path.iter().any(|actual| actual == head_path),
            "{text}: {path:?}"
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
        "ordinary finite clauses retain subject-derived third-person concord_class",
    );
    assert!(
        parser.parse("A player gain 2 life.", &context).is_err(),
        "ordinary finite clauses do not inherit auxiliary bare concord_class",
    );
}

#[derive(Clone, Copy)]
enum CoordinationKind {
    And,
    Or,
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
        _ => panic!("coordinator meaning is stored independently of punctuation: {text}"),
    };
    assert_eq!(
        members
            .iter()
            .map(coordinated_predicate_identity)
            .collect::<Vec<_>>(),
        expected_members,
        "the AST preserves every predicate payload in source order: {text}",
    );
}

#[test]
fn predicate_coordination_is_nary_with_exact_pair_serial_and_final_surfaces() {
    let parser = parser();
    let context = context("Context Card", false);
    for (kind, coordinator) in [(CoordinationKind::And, "and"), (CoordinationKind::Or, "or")] {
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
        _ => panic!("clause coordinator meaning is stored independently: {text}"),
    };
    assert_eq!(
        members
            .iter()
            .map(coordinated_clause_identity)
            .collect::<Vec<_>>(),
        expected_members,
        "the AST preserves every complete finite-clause payload in source order: {text}",
    );
}

#[test]
fn complete_finite_clause_coordination_is_nary_and_preserves_member_concord_class() {
    let parser = parser();
    let context = context("Aang, A Lot to Learn", true);
    for (kind, coordinator) in [(CoordinationKind::And, "and"), (CoordinationKind::Or, "or")] {
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
            "each coordinated finite clause derives concord_class from its own subject: {invalid}",
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

    assert!(
        AndPredicateCoordination::new(Box::new(vec![CoordinatedPredicate::Atomic(Box::new(
            gain_life_predicate(1),
        ))]))
        .is_none()
    );
    assert!(
        AndClauseCoordination::new(Box::new(vec![CoordinatedClause::Finite(Box::new(
            plain_finite(
                you_subject(),
                Predicate::Atomic(Box::new(gain_life_predicate(1))),
            ),
        ))]))
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
    Subject,
    SubjectPronoun(SubjectPronoun),
    CommonNoun(CommonNoun),
    SelfReference(SelfReferenceSpelling),
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
        if let Some(magnitude) = gain_life_magnitude(value) {
            self.0.push(LogicVisit::GainLife(magnitude));
        } else if declared_action_name(value) == Some("Connive") {
            self.0.push(LogicVisit::Connive);
        } else {
            panic!("unexpected logic visitor predicate payload: {value:?}");
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
    let FiniteClause::PlainFiniteClause(_) = finite.as_ref() else {
        unreachable!()
    };
    let mut visitor = LogicVisitor::default();
    visitor.visit_finite_clause(finite);
    assert_eq!(
        visitor.0,
        [
            LogicVisit::FiniteClause,
            LogicVisit::Subject,
            LogicVisit::SubjectPronoun(SubjectPronoun::You),
            LogicVisit::Predicate,
            LogicVisit::Connive,
        ],
    );
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

#[expect(
    clippy::unnecessary_box_returns,
    reason = "every consumer of this witness takes an owned boxed condition clause"
)]
fn connive_condition_clause() -> Box<Clause> {
    Box::new(Clause::Finite(Box::new(connive_clause())))
}

fn player_subject() -> Subject {
    Subject::SubjectNominal(NominalSubject {
        value: NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
                UnqualifiedPostmodifiedReference {
                    reference: Box::new(indefinite_player_reference()),
                },
            )),
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

#[expect(
    clippy::boxed_local,
    reason = "the helper consumes the boxed Clause shape used by the exact AST witnesses"
)]
fn if_clause_tail(condition: Box<Clause>) -> IfClauseTail {
    let Clause::Finite(condition) = *condition else {
        panic!("the clause-tail witness condition is finite")
    };
    IfClauseTail::IfClauseTail(IfClauseTailValue {
        condition: *condition,
    })
}

fn preposed_if_clause_tail(condition: Box<Clause>) -> PreposedIfClauseTail {
    PreposedIfClauseTail::PreposedIfClauseTail(PreposedIfClauseTailValue { condition })
}

#[expect(
    clippy::boxed_local,
    reason = "the helper consumes the boxed Clause shape used by the exact AST witnesses"
)]
fn unless_clause_tail(condition: Box<Clause>) -> UnlessClauseTail {
    let Clause::Finite(condition) = *condition else {
        panic!("the clause-tail witness condition is finite")
    };
    UnlessClauseTail::UnlessClauseTail(UnlessClauseTailValue {
        condition: *condition,
    })
}

fn preposed_as_long_as_clause_tail(condition: Box<Clause>) -> PreposedAsLongAsClauseTail {
    PreposedAsLongAsClauseTail::PreposedAsLongAsClauseTail(PreposedAsLongAsClauseTailValue {
        condition,
    })
}

fn postposed_if_clause_tail(condition: Box<Clause>) -> PostposedClauseTail {
    PostposedClauseTail::Simple(Box::new(SimplePostposedClauseTail::If(Box::new(
        if_clause_tail(condition),
    ))))
}

fn postposed_unless_clause_tail(condition: Box<Clause>) -> PostposedClauseTail {
    PostposedClauseTail::Simple(Box::new(SimplePostposedClauseTail::Unless(Box::new(
        unless_clause_tail(condition),
    ))))
}

fn preposed_clause_tail(tail: PreposedClauseTail, body: Clause) -> ClauseAttachment {
    ClauseAttachment::PreposedClauseTail(Box::new(PreposedClauseTailAttachment {
        tail: Box::new(tail),
        body: Box::new(body),
    }))
}

fn postposed_clause_tail(body: Clause, tail: PostposedClauseTail) -> Clause {
    let body = match body {
        Clause::Finite(body) => ClauseTailBody::Finite(body),
        Clause::Coordination(body) => ClauseTailBody::Coordination(body),
        Clause::Tail(_) => panic!("a clause tail body is deliberately nonrecursive"),
    };
    Clause::Tail(Box::new(PostposedClauseTailClause::PostposedClauseTail(
        PostposedClauseTailClauseValue {
            body: Box::new(body),
            tail: Box::new(tail),
        },
    )))
}

fn preposed_predicate_clause_tail(tail: PreposedClauseTail, body: Predicate) -> ClauseAttachment {
    ClauseAttachment::PreposedPredicateClauseTail(Box::new(
        PreposedPredicateClauseTailAttachment::new(Box::new(tail), Box::new(body))
            .expect("the attached imperative predicate is bare"),
    ))
}

fn postposed_predicate_clause_tail(body: Predicate, tail: PostposedClauseTail) -> ClauseAttachment {
    ClauseAttachment::PostposedPredicateClauseTail(Box::new(
        PostposedPredicateClauseTailAttachment::new(Box::new(body), Box::new(tail))
            .expect("the attached imperative predicate is bare"),
    ))
}

fn is_preposed_if_clause_attachment(attachment: &ClauseAttachment) -> bool {
    matches!(
        attachment,
        ClauseAttachment::PreposedClauseTail(value)
            if matches!(value.tail.as_ref(), PreposedClauseTail::If(_))
    )
}

fn is_preposed_if_predicate_attachment(attachment: &ClauseAttachment) -> bool {
    matches!(
        attachment,
        ClauseAttachment::PreposedPredicateClauseTail(value)
            if matches!(value.tail.as_ref(), PreposedClauseTail::If(_))
    )
}

fn is_postposed_if_clause(clause: &Clause) -> bool {
    matches!(
        clause,
        Clause::Tail(value)
            if matches!(value.as_ref(), PostposedClauseTailClause::PostposedClauseTail(value)
                if matches!(value.tail.as_ref(), PostposedClauseTail::Simple(tail)
                    if matches!(tail.as_ref(), SimplePostposedClauseTail::If(_))))
    )
}

#[test]
fn attachment_products_have_an_intermediate_linguistic_stage_for_imperatives() {
    let parser = parser();
    let context = context("Context Card", false);
    let expected = Sentence::Attached(Attached {
        attachment: Box::new(preposed_predicate_clause_tail(
            PreposedClauseTail::If(Box::new(
                preposed_if_clause_tail(connive_condition_clause()),
            )),
            Predicate::Atomic(Box::new(connive())),
        )),
    });
    let selected = assert_one_logic_candidate(&parser, &context, "If you connive, connive.");
    assert_eq!(
        selected,
        Ability::Plain(Plain {
            body: AbilityBody::Sentences(
                Sentences::new(Box::new(vec![expected])).expect("one sentence"),
            ),
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
    let preposed_condition = connive_condition_clause();
    let body = gain_clause(2);

    for (text, expected) in [
        (
            "If you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: Box::new(preposed_clause_tail(
                    PreposedClauseTail::If(Box::new(preposed_if_clause_tail(
                        preposed_condition.clone(),
                    ))),
                    body.clone(),
                )),
            }),
        ),
        (
            "You gain 2 life if you connive.",
            Sentence::Declarative(Declarative {
                clause: Box::new(postposed_clause_tail(
                    body.clone(),
                    postposed_if_clause_tail(Box::new(Clause::Finite(Box::new(condition.clone())))),
                )),
            }),
        ),
        (
            "You gain 2 life unless you connive.",
            Sentence::Declarative(Declarative {
                clause: Box::new(postposed_clause_tail(
                    body.clone(),
                    postposed_unless_clause_tail(Box::new(Clause::Finite(Box::new(
                        condition.clone(),
                    )))),
                )),
            }),
        ),
        (
            "As long as you connive, you gain 2 life.",
            Sentence::Attached(Attached {
                attachment: Box::new(preposed_clause_tail(
                    PreposedClauseTail::AsLongAs(Box::new(preposed_as_long_as_clause_tail(
                        preposed_condition.clone(),
                    ))),
                    body.clone(),
                )),
            }),
        ),
    ] {
        let selected = assert_one_logic_candidate(&parser, &context, text);
        assert_eq!(
            selected,
            Ability::Plain(Plain {
                body: AbilityBody::Sentences(
                    Sentences::new(Box::new(vec![expected]))
                        .expect("one conditional sentence is nonempty"),
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
fn ordered_then_and_proverb_conditions_are_linguistic_and_disjoint() {
    let parser = parser();
    let context = context("Context Card", false);
    let ordered_text = "You gain 1 life, you connive, then a player gains 2 life.";
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
                Clause::Coordination(_) | Clause::Tail(_) => {
                    panic!("then members retain their finite clause shape")
                }
            })
            .collect::<Vec<_>>(),
        ["you/gain:1", "you/connive", "player/gain:2"],
    );

    let if_text = "You gain 1 life. If you do, you connive.";
    let selected = assert_one_logic_candidate(&parser, &context, if_text);
    let [
        Sentence::Declarative(_),
        Sentence::Attached(Attached { attachment }),
    ] = plain_sentences(&selected).sentences()
    else {
        panic!("the proverb condition is its own sentence shape")
    };
    assert!(is_preposed_if_clause_attachment(attachment));

    let when_text = "When you do, you connive.";
    let triggered = assert_one_logic_candidate(&parser, &context, when_text);
    assert!(matches!(triggered, Ability::Triggered(_)));
    for text in [if_text, when_text] {
        let analysis = parser.analyze(text, &context);
        let path = analysis.decision().unwrap().candidates()[0].construction_path();
        assert!(
            path.iter()
                .any(|actual| actual == "VerbPhraseProVerbPredicate"),
            "the generic pro-verb construction owns do: {text}: {path:?}",
        );
    }

    for invalid in [
        "You gain 1 life then you connive.",
        "You gain 1 life, then you connive, then a player gains 2 life.",
        "You gain 1 life. If you do you connive.",
        "You gain 1 life. if you do, you connive.",
        "When you do,  you connive.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "ordered and proverb boundaries cannot borrow punctuation: {invalid}",
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
    let AbilityBody::Sentences(ordinary_body) = body else {
        panic!("ordinary trailing if has a sentence body")
    };
    assert!(matches!(
        ordinary_body.sentences(),
        [Sentence::Declarative(Declarative { clause })]
            if is_postposed_if_clause(clause)
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
    let AbilityBody::Sentences(intervening_body) = body else {
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

    fn visit_preposed_predicate_clause_tail_attachment(
        &mut self,
        value: &PreposedPredicateClauseTailAttachment,
    ) {
        self.0.push("PreposedPredicateClauseTailAttachment");
        deckmaste_english_v2::visit::walk_preposed_predicate_clause_tail_attachment(self, value);
    }

    fn visit_postposed_predicate_clause_tail_attachment(
        &mut self,
        value: &PostposedPredicateClauseTailAttachment,
    ) {
        self.0.push("PostposedPredicateClauseTailAttachment");
        deckmaste_english_v2::visit::walk_postposed_predicate_clause_tail_attachment(self, value);
    }

    fn visit_then_sequence(&mut self, value: &ThenSequence) {
        self.0.push("ThenSequence");
        deckmaste_english_v2::visit::walk_then_sequence(self, value);
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

    fn visit_preposed_predicate_clause_tail_attachment(
        &mut self,
        value: &PreposedPredicateClauseTailAttachment,
    ) {
        self.0.push("PreposedPredicateClauseTailAttachment");
        deckmaste_english_v2::visit::walk_preposed_predicate_clause_tail_attachment(self, value);
    }

    fn visit_postposed_predicate_clause_tail_attachment(
        &mut self,
        value: &PostposedPredicateClauseTailAttachment,
    ) {
        self.0.push("PostposedPredicateClauseTailAttachment");
        deckmaste_english_v2::visit::walk_postposed_predicate_clause_tail_attachment(self, value);
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
                    reference if is_indefinite_player(reference) => self.0.push("Subject:Player"),
                    other => panic!("unexpected trigger subject payload: {other:?}"),
                }
            }
            other @ (Subject::SubjectPronoun(_) | Subject::VariableSubject(_)) => {
                panic!("unexpected attachment subject payload: {other:?}")
            }
        }
    }

    fn visit_predicate(&mut self, value: &Predicate) {
        match value {
            Predicate::Atomic(predicate) if declared_action_name(predicate) == Some("Connive") => {
                self.0.push("Predicate:Connive");
            }
            Predicate::Atomic(predicate) if gain_life_magnitude(predicate) == Some(2) => {
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
    let condition = connive_condition_clause();
    let gain = Predicate::Atomic(Box::new(gain_life_predicate(2)));
    let root_text = "If you connive, gain 2 life.";
    let root = assert_one_logic_candidate(&parser, &context, root_text);

    assert_eq!(
        root,
        Ability::Plain(Plain {
            body: AbilityBody::Sentences(
                Sentences::new(Box::new(vec![Sentence::Attached(Attached {
                    attachment: Box::new(preposed_predicate_clause_tail(
                        PreposedClauseTail::If(Box::new(preposed_if_clause_tail(
                            condition.clone(),
                        ))),
                        gain.clone(),
                    )),
                })]))
                .expect("one root sentence"),
            ),
        }),
    );

    assert_eq!(
        literal_claims(&parser, &context, root_text, &[",", ": "]),
        vec![(
            14,
            15,
            "form:preposed_predicate_clause_tail/preposed_predicate_clause_tail/1".to_owned(),
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
            "PreposedPredicateClauseTailAttachment",
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
            body: AbilityBody::Sentences(
                Sentences::new(Box::new(vec![Sentence::Attached(Attached {
                    attachment: Box::new(postposed_predicate_clause_tail(
                        gain,
                        postposed_if_clause_tail(Box::new(Clause::Finite(Box::new(condition)))),
                    )),
                })]))
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
            "PostposedPredicateClauseTailAttachment",
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
    let preposed_condition = connive_condition_clause();
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
        &activated.body,
        &AbilityBody::Sentences(
            Sentences::new(Box::new(vec![Sentence::Attached(Attached {
                attachment: Box::new(preposed_predicate_clause_tail(
                    PreposedClauseTail::If(Box::new(preposed_if_clause_tail(preposed_condition))),
                    gain,
                )),
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
                "form:preposed_predicate_clause_tail/preposed_predicate_clause_tail/1".to_owned(),
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
            "PreposedPredicateClauseTailAttachment",
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
        TextSpan { start: 31, end: 34 },
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
        TextSpan { start: 36, end: 39 },
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
    let preposed_condition = connive_condition_clause();
    let gain = Predicate::Atomic(Box::new(gain_life_predicate(2)));

    for (text, expected) in [
        (
            "Gain 2 life if you connive.",
            postposed_predicate_clause_tail(
                gain.clone(),
                postposed_if_clause_tail(Box::new(Clause::Finite(Box::new(condition.clone())))),
            ),
        ),
        (
            "Gain 2 life unless you connive.",
            postposed_predicate_clause_tail(
                gain.clone(),
                postposed_unless_clause_tail(Box::new(Clause::Finite(Box::new(condition.clone())))),
            ),
        ),
        (
            "As long as you connive, gain 2 life.",
            preposed_predicate_clause_tail(
                PreposedClauseTail::AsLongAs(Box::new(preposed_as_long_as_clause_tail(
                    preposed_condition.clone(),
                ))),
                gain.clone(),
            ),
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
    let [Sentence::Imperative(imperative)] = plain_sentences(&ordered).sentences() else {
        panic!("predicate ordering inhabits the shared predicate stage")
    };
    let Predicate::ThenSequence(sequence) = imperative.predicate() else {
        panic!("predicate ordering retains its dedicated predicate alternative")
    };
    let ThenPredicateSequence::ThenPredicateSequence(sequence) = sequence.as_ref();
    assert_eq!(
        sequence.members(),
        [
            CoordinatedPredicate::Atomic(Box::new(gain_life_predicate(1))),
            CoordinatedPredicate::Atomic(Box::new(connive())),
        ],
    );

    let proverb = assert_one_logic_candidate(&parser, &context, "If you do, gain 2 life.");
    let [Sentence::Attached(Attached { attachment })] = plain_sentences(&proverb).sentences()
    else {
        panic!("the proverb-conditioned imperative stores its body in the attachment stage")
    };
    assert!(is_preposed_if_predicate_attachment(attachment));
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
        "You gain 1 life, you connive, then a player gains 2 life.",
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

    let proverb = assert_one_logic_candidate(&parser, &ordinary, "If you do, you connive.");
    let [Sentence::Attached(Attached { attachment })] = plain_sentences(&proverb).sentences()
    else {
        unreachable!()
    };
    let ClauseAttachment::PreposedClauseTail(preposed) = attachment.as_ref() else {
        unreachable!()
    };
    let PreposedClauseTail::If(if_tail) = preposed.tail.as_ref() else {
        unreachable!()
    };
    let PreposedIfClauseTail::PreposedIfClauseTail(if_tail) = if_tail.as_ref();
    let Clause::Finite(condition) = if_tail.condition.as_ref() else {
        unreachable!()
    };
    let mut visitor = AttachmentVisitor::default();
    visitor.visit_finite_clause(condition.as_ref());
    assert_eq!(visitor.0, ["FiniteClause", "Predicate"],);

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
    let AbilityBody::PlainModal(modal) = body else {
        panic!("the envelope contains the unchanged plain-modal body")
    };
    modal
}

/// Every attested dash head, written as the head clause renders it.
const DASH_MODAL_HEADERS: [&str; 10] = [
    "Choose one",
    "Choose two",
    "Choose one or both",
    "Choose one or more",
    "Choose up to one",
    "An opponent chooses one",
    "An opponent chooses two",
    "An opponent chooses one or both",
    "An opponent chooses one or more",
    "An opponent chooses up to one",
];

fn continuation_header(header: &str) -> String {
    let mut characters = header.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_lowercase().chain(characters).collect()
    })
}

fn mode_marker(mode: &ModalMode) -> &ModeMarker {
    let ModalMode::ModalMode(value) = mode;
    value.marker.as_ref()
}

fn bullet_modes(modal: &PlainModal) -> bool {
    modal
        .modes()
        .iter()
        .all(|mode| matches!(mode_marker(mode), ModeMarker::Bullet(_)))
}

#[test]
fn every_plain_modal_header_selects_independently_in_every_ability_envelope() {
    let parser = parser();
    let context = context("Context Card", false);
    for header in DASH_MODAL_HEADERS {
        for envelope in ["root", "trigger", "activation"] {
            let written = if envelope == "trigger" {
                continuation_header(header)
            } else {
                header.to_owned()
            };
            let text = wrapped_modal_text(envelope, &written);
            let selected = if envelope == "trigger" {
                assert_resolved_logic_candidate(&parser, &context, &text)
            } else {
                assert_one_logic_candidate(&parser, &context, &text)
            };
            let modal = selected_plain_modal(&selected, envelope);
            assert!(
                matches!(modal.head.as_ref(), ModalHead::Dash(_)),
                "an em-dash head is a clause plus the structural dash: {text}",
            );
            assert_eq!(modal.modes().len(), 2, "{text}");
            assert!(bullet_modes(modal), "{text}");
            assert_eq!(
                selected.render(&context, parser.environment()),
                text,
                "the complete wrapped modal renders identically",
            );
        }
    }
}

#[test]
fn every_modal_head_kind_selects_with_its_own_mode_markers() {
    let parser = parser();
    let context = context("Context Card", false);
    let dash = "Choose one —\n• You gain 1 life.\n• You gain 2 life.";
    let sentence = concat!(
        "Choose three. You may choose the same mode more than once.\n",
        "• You gain 1 life.\n• You gain 2 life.",
    );
    let keyword = "Spree\n+ {1} — You gain 1 life.\n+ {2}{R} — You gain 2 life.";
    let pawprint = "Spree\n{P} — You gain 1 life.\n{P}{P} — You gain 2 life.";

    for (text, bullet) in [
        (dash, true),
        (sentence, true),
        (keyword, false),
        (pawprint, false),
    ] {
        let selected = assert_one_logic_candidate(&parser, &context, text);
        let modal = selected_plain_modal(&selected, "root");
        assert_eq!(modal.modes().len(), 2, "{text}");
        assert_eq!(bullet_modes(modal), bullet, "{text}");
        assert_eq!(
            selected.render(&context, parser.environment()),
            text,
            "every head kind renders back byte for byte",
        );
    }

    let dash_head =
        selected_plain_modal(&assert_one_logic_candidate(&parser, &context, dash), "root")
            .head
            .as_ref()
            .clone();
    assert!(matches!(dash_head, ModalHead::Dash(_)));
    let sentence_head = selected_plain_modal(
        &assert_one_logic_candidate(&parser, &context, sentence),
        "root",
    )
    .head
    .as_ref()
    .clone();
    assert!(
        matches!(sentence_head, ModalHead::Sentence(_)),
        "a period-terminated allowance is a sentence head, not a dash head",
    );
    let keyword_head = selected_plain_modal(
        &assert_one_logic_candidate(&parser, &context, keyword),
        "root",
    )
    .head
    .as_ref()
    .clone();
    assert!(
        matches!(keyword_head, ModalHead::Keyword(_)),
        "the spree keyword line is itself the modal head",
    );
}

#[test]
fn the_weighted_mode_marker_is_one_construction_for_spree_and_pawprint() {
    let parser = parser();
    let context = context("Context Card", false);
    let selected = assert_one_logic_candidate(
        &parser,
        &context,
        "Spree\n+ {1} — You gain 1 life.\n{P}{P} — You gain 2 life.",
    );
    let modal = selected_plain_modal(&selected, "root");
    let markers = modal
        .modes()
        .iter()
        .map(|mode| match mode_marker(mode) {
            ModeMarker::Weighted(marker) => marker.additional.is_some(),
            ModeMarker::Bullet(_) => panic!("weighted lines are not bullets"),
            ModeMarker::FlavorWord(_) => panic!("weighted lines are not flavor-word modes"),
        })
        .collect::<Vec<_>>();
    assert_eq!(
        markers,
        [true, false],
        "the plus sign is the marker's optional additional-cost mark, not a second construction",
    );
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
        ModalModeValue::new(
            Box::new(ModeMarker::Bullet(BulletMarker {})),
            Box::new(sentences),
        )
        .expect("a bulleted modal mode has a nonempty sentence sequence"),
    )
}

#[test]
fn plain_modal_modes_render_visitor_and_claims_are_hand_derived() {
    let parser = parser();
    let context = context("Context Card", false);
    let text = "Choose one —\n• You gain 1 life. You connive.\n• You gain 2 life.";
    let selected = assert_one_logic_candidate(&parser, &context, text);
    let modal = selected_plain_modal(&selected, "root");
    assert_eq!(
        modal.modes(),
        [
            modal_mode(vec![
                modal_sentence(gain_life_predicate(1)),
                modal_sentence(connive()),
            ]),
            modal_mode(vec![modal_sentence(gain_life_predicate(2))]),
        ],
        "both bulleted modes keep their exact hand-derived sentence ASTs",
    );
    assert_eq!(selected.render(&context, parser.environment()), text);

    assert!(
        ModalModeValue::new(
            Box::new(ModeMarker::Bullet(BulletMarker {})),
            Box::default(),
        )
        .is_none(),
        "a mode cannot lose its last sentence",
    );
    assert!(
        PlainModal::new(
            modal.head.clone(),
            Box::new(vec![modal_mode(vec![modal_sentence(gain_life_predicate(
                1
            ))])]),
        )
        .is_none(),
        "a modal group cannot lose its second mode",
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
            (0, 6, "core-verb:Choose"),
            (6, 10, "codec:CardinalNumber"),
            (10, 14, "form:dash_head/dash_head/1"),
            (14, 15, "form:plain_modal/plain_modal/1"),
            (15, 19, "form:bullet_marker/bullet_marker/0"),
            (19, 22, "vocab:SubjectPronoun/You"),
            (22, 27, "core-verb:Gain"),
            (27, 29, "codec:ScalarNumber"),
            (29, 34, "lexeme:CommonNoun/Life/singular"),
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
            (49, 53, "form:bullet_marker/bullet_marker/0"),
            (53, 56, "vocab:SubjectPronoun/You"),
            (56, 61, "core-verb:Gain"),
            (61, 63, "codec:ScalarNumber"),
            (63, 68, "lexeme:CommonNoun/Life/singular"),
            (68, 69, "structural:ModalModeValue/sentences/terminator/0"),
        ],
        "the dash head and both bullet markers own disjoint structural bytes",
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
fn plain_modal_accepts_conditional_choice_count_and_rejects_malformed_surfaces() {
    let parser = parser();
    let context = context("Context Card", false);
    let conditional_choice_count = "Choose one. If this spell was kicked, choose any number instead.\n• You gain 1 life.\n• You gain 2 life.";
    // “Modal choices”: “Bullet groups also use complete-sentence headers when the choice count is conditional.”
    let selected = assert_one_logic_candidate(&parser, &context, conditional_choice_count);
    assert_eq!(
        selected.render(&context, parser.environment()),
        conditional_choice_count,
    );
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
        let selected = if envelope == "trigger" {
            assert_resolved_logic_candidate(&parser, &legendary, &legendary_text)
        } else {
            assert_one_logic_candidate(&parser, &legendary, &legendary_text)
        };
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
            reference: Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
                UnqualifiedPostmodifiedReference {
                    reference: Box::new(UnqualifiedReference::SelfReference(reference)),
                },
            )),
        }),
    })
}

fn full_self_reference_modes(context: &ParseContext<'_>) -> Vec<ModalMode> {
    let sentence = |magnitude| {
        Sentence::Declarative(Declarative {
            clause: Box::new(Clause::Finite(Box::new(plain_finite(
                full_self_reference_subject(context),
                Predicate::Atomic(Box::new(gain_life_predicate(magnitude))),
            )))),
        })
    };
    vec![modal_mode(vec![sentence(1)]), modal_mode(vec![sentence(2)])]
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
        let selected = if envelope == "trigger" {
            assert_resolved_logic_candidate(&parser, &ordinary, &text)
        } else {
            assert_one_logic_candidate(&parser, &ordinary, &text)
        };
        assert_eq!(
            selected_plain_modal(&selected, envelope).modes(),
            full_self_reference_modes(&ordinary),
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
            "deckmaste_construction_core".to_owned(),
            "deckmaste_english_v2".to_owned(),
            "ron".to_owned(),
            "serde".to_owned(),
            "thiserror".to_owned(),
        ]),
        "the complete direct normal-dependency set remains bounded and excludes deckmaste_core, deckmaste_features, deckmaste_english, and macro_ron",
    );
}
