use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use macro_ron::v2::DeclarationIdentity;
use macro_ron::v2::Onset;
use macro_ron::v2::read_str;

fn environment() -> ParserEnvironment {
    let declarations = [
        (
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Sacrifice.ron",
            r#"KeywordAction(name:"Sacrifice",spelling:"sacrifice",grammar:Verb(bare:"sacrifice",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"connive",grammar:Verb(bare:"connive",valence:Intransitive))"#,
        ),
        (
            "/synthetic/actions/Scry.ron",
            r#"KeywordAction(name:"Scry",spelling:"scry",grammar:Verb(bare:"scry",third_person:"scries",valence:Custom(shapes:[[],[Amount]])))"#,
        ),
        (
            "/synthetic/actions/Surveil.ron",
            r#"KeywordAction(name:"Surveil",spelling:"surveil",grammar:Verb(bare:"surveil",valence:Numerative))"#,
        ),
    ]
    .into_iter()
    .map(|(path, source)| read_str(path, source).expect("synthetic keyword action is valid"));
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
    .expect("synthetic predicate environment freezes")
}

fn parser() -> Parser {
    Parser::new(environment()).expect("shared active frames have no fixed-name requirements")
}

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card", false, Onset::Consonant).expect("context is valid")
}

fn assert_selected(parser: &Parser, context: &ParseContext<'_>, text: &str) -> Ability {
    let analysis = parser.analyze(text, context);
    let selected = analysis.selected().unwrap_or_else(|| {
        panic!("complete predicate document must select {text:?}: {analysis:?}")
    });
    let decision = analysis
        .decision()
        .expect("selected document has a decision");
    assert_eq!(
        decision.resolution(),
        SelectionResolution::Unique,
        "{text:?}"
    );
    assert_eq!(decision.survivors().len(), 1, "{text:?}");
    assert!(decision.exception_uses().is_empty(), "{text:?}");
    assert_eq!(selected.render(context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("selected document has ownership");
    assert!(ownership.summary().covered(), "{text:?}: {ownership:?}");
    assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
    assert_eq!(ownership.rendered_text(), text);
    assert_eq!(ownership.summary().gap_spans(), 0, "{text:?}");
    assert_eq!(ownership.summary().overlap_spans(), 0, "{text:?}");
    assert_eq!(ownership.summary().synthetic_claims(), 0, "{text:?}");
    assert_eq!(
        ownership.summary().provenance_plan_mismatches(),
        0,
        "{text:?}"
    );
    selected.clone()
}

#[test]
fn shared_active_frames_reuse_core_and_declared_heads_across_sentence_shapes() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy target player.",
        "Sacrifice target player.",
        "You destroy target player.",
        "It sacrifices target player.",
        "Control target player.",
        "You control target player.",
        "Connive.",
        "It connives.",
        "Enter.",
        "It enters.",
        "You scry.",
        "It scries.",
        "Scry 2.",
        "Surveil X.",
        "Draw 2.",
        "Sacrifice target player and control target player.",
        "You sacrifice target player and control target player.",
    ] {
        assert_selected(&parser, &context, text);
    }

    let sentence = parser
        .parse_sentence("Sacrifice target player.", &context)
        .expect("declared transitive imperative parses");
    let Sentence::Imperative(imperative) = sentence else {
        panic!("bare predicate has an imperative envelope")
    };
    let Predicate::Atomic(VerbPhrase::TransitivePredicate(predicate)) = imperative.predicate()
    else {
        panic!("declared object verb uses the shared transitive product")
    };
    let TransitiveVerb::Declaration(head) = &predicate.head else {
        panic!("declared transitive identity remains open")
    };
    assert_eq!(head.id().name(), "Sacrifice");

    let sentence = parser
        .parse_sentence("Control target player.", &context)
        .expect("core transitive imperative parses");
    let Sentence::Imperative(imperative) = sentence else {
        panic!("bare predicate has an imperative envelope")
    };
    let Predicate::Atomic(VerbPhrase::TransitivePredicate(predicate)) = imperative.predicate()
    else {
        panic!("core object verb uses the shared transitive product")
    };
    assert!(matches!(
        &predicate.head,
        TransitiveVerb::Lexeme(CoreTransitiveVerb::Control)
    ));
}

#[derive(Default)]
struct LexicalVisitor(Vec<String>);

impl Visitor for LexicalVisitor {
    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        self.0.push(format!("declared:{}", declaration.name()));
    }

    fn visit_core_transitive_verb(&mut self, verb: CoreTransitiveVerb) {
        self.0.push(format!("core:{verb:?}"));
    }

    fn visit_common_noun(&mut self, noun: CommonNoun) {
        self.0.push(format!("noun:{noun:?}"));
    }
}

#[test]
fn shared_transitive_frame_preserves_visit_order_and_literal_claims() {
    let parser = parser();
    let context = context();
    let text = "You sacrifice target player and control target player.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = LexicalVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.0,
        [
            "declared:Sacrifice",
            "noun:Player",
            "core:Control",
            "noun:Player",
        ]
    );

    let ownership = parser
        .analyze(text, &context)
        .ownership()
        .expect("selected coordination has ownership")
        .parsed_claims()
        .iter()
        .map(|claim| {
            (
                claim.span().start,
                claim.span().end,
                claim.stable_owner_id().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        ownership,
        [
            (0, 3, "vocab:SubjectPronoun/You".to_owned()),
            (3, 13, "lexeme:keyword_action/Sacrifice/bare".to_owned(),),
            (
                13,
                20,
                "form:target_determiner_phrase/target_determiner_phrase/0".to_owned(),
            ),
            (20, 27, "lexeme:CommonNoun/Player/singular".to_owned()),
            (
                27,
                32,
                "structural:AndPredicateCoordination/members/separator/pair/0".to_owned(),
            ),
            (32, 39, "lexeme:CoreTransitiveVerb/Control/bare".to_owned()),
            (
                39,
                46,
                "form:target_determiner_phrase/target_determiner_phrase/0".to_owned(),
            ),
            (46, 53, "lexeme:CommonNoun/Player/singular".to_owned()),
            (
                53,
                54,
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ]
    );
}

#[test]
fn shared_active_frames_reject_complement_and_agreement_reciprocals() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy.",
        "Connive target player.",
        "Control.",
        "Enter target player.",
        "Scry target player.",
        "Scry 2 target player.",
        "Surveil.",
        "Surveil target player.",
        "Sacrifice 2.",
        "It sacrifice target player.",
        "You sacrifices target player.",
        "It connive.",
        "You connives.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong frame or agreement must reject {text:?}",
        );
    }
}
