use std::fs;
use std::path::Path;

use deckmaste_english_v2::ast::NounLexeme;
use deckmaste_english_v2::catalogs::ParserCatalogs;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::DeclarationId;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::ParserBuildError;
use deckmaste_english_v2::parser::TraceLimits;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use deckmaste_english_v2::visit::walk_ability;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::GrammarPosition;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::read_builtin_v2;
use macro_ron::v2::read_str;

fn declaration(path: &str, source: &str) -> NormalizedDeclaration {
    read_str(path, source).expect("synthetic normalized declaration is valid")
}

fn builtin_rows() -> Vec<NormalizedDeclaration> {
    read_builtin_v2(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"))
        .expect("integrated builtin-v2 declarations load")
}

fn production_environment() -> ParserEnvironment {
    let environment = ParserEnvironment::try_from_declarations(builtin_rows())
        .expect("builtin-v2 declarations freeze");
    ParserCatalogs::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"))
        .expect("canonical catalogs load")
        .attach_to(environment)
}

fn parser() -> Parser {
    Parser::new(production_environment()).expect("required declarations are present")
}

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card").expect("context is valid")
}

#[test]
fn parser_build_rejects_missing_wrong_recipe_and_missing_agreement_surface() {
    let missing = Parser::new(ParserEnvironment::try_from_declarations([]).unwrap())
        .expect_err("static declarations must exist before parsing");
    assert!(matches!(
        missing,
        ParserBuildError::MissingDeclaration { kind: DeclarationKind::KeywordAction, ref name }
            if name == "Destroy" || name == "Connive"
    ));

    let wrong_recipe = ParserEnvironment::try_from_declarations([
        declaration(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:FixedTerm(surface:"destroy"))"#,
        ),
        declaration(
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"connive",grammar:Verb(bare:"connive",valence:Intransitive))"#,
        ),
    ])
    .unwrap();
    assert!(matches!(
        Parser::new(wrong_recipe).expect_err("fixed-term Destroy is not a verb"),
        ParserBuildError::WrongGrammarPosition {
            expected: GrammarPosition::Verb,
            actual: Some(GrammarPosition::FixedTerm),
            ..
        }
    ));

    let missing_feature = ParserEnvironment::try_from_declarations([
        declaration(
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",third_person:Unavailable,valence:Transitive))"#,
        ),
        declaration(
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"connive",grammar:Verb(bare:"connive",valence:Intransitive))"#,
        ),
    ])
    .unwrap();
    assert!(matches!(
        Parser::new(missing_feature).expect_err("both agreement surfaces are required"),
        ParserBuildError::MissingSurfaceFeature {
            feature: SurfaceFeature::ThirdPersonSingular,
            ..
        }
    ));
}

#[test]
fn builtin_open_verbs_parse_and_render_both_agreements_exactly() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy target creature.",
        "That creature destroys target creature.",
        "Connive.",
        "It connives.",
    ] {
        let ability = parser.parse(text, &context).unwrap_or_else(|error| {
            panic!("builtin declaration surface must parse `{text}`: {error}")
        });
        assert_eq!(ability.render(&context, parser.environment()), text);
    }

    for text in ["It destroy target creature.", "That creature connive."] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong agreement parsed: {text}"
        );
    }
}

#[test]
fn category_homonym_does_not_replace_the_requested_action_identity() {
    let mut rows = builtin_rows();
    rows.push(declaration(
        "/synthetic/abilities/Destroy.ron",
        r#"KeywordAbility(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",valence:Transitive))"#,
    ));
    let environment = ParserEnvironment::try_from_declarations(rows).unwrap();
    assert_eq!(
        environment.readings(GrammarPosition::Verb, "destroy").len(),
        2,
        "the environment must retain both category-safe identities"
    );
    let environment = ParserCatalogs::load(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
    )
    .unwrap()
    .attach_to(environment);
    let parser = Parser::new(environment).unwrap();
    let trace = parser.trace(
        "Destroy target creature.",
        &context(),
        TraceLimits::new(usize::MAX),
    );
    let declaration_tokens = trace
        .tokens()
        .items()
        .iter()
        .filter(|token| token.terminal_name_v1().contains("Declaration"))
        .collect::<Vec<_>>();
    assert_eq!(declaration_tokens.len(), 1);
    assert!(
        declaration_tokens[0]
            .value_label_v1()
            .contains("KeywordAction")
    );
    assert!(
        !declaration_tokens[0]
            .value_label_v1()
            .contains("KeywordAbility")
    );

    let ability_only = ParserEnvironment::try_from_declarations([declaration(
        "/synthetic/abilities/Destroy.ron",
        r#"KeywordAbility(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",valence:Transitive))"#,
    )])
    .unwrap();
    assert!(matches!(
        Parser::new(ability_only).expect_err("another category cannot satisfy the request"),
        ParserBuildError::MissingDeclaration {
            kind: DeclarationKind::KeywordAction,
            ..
        }
    ));
}

#[derive(Default)]
struct IdentityVisitor {
    events: Vec<String>,
}

impl Visitor for IdentityVisitor {
    fn visit_declaration(&mut self, id: &DeclarationId) {
        self.events
            .push(format!("declaration:{:?}/{}", id.kind(), id.name()));
    }

    fn visit_noun_lexeme(&mut self, noun: NounLexeme) {
        self.events.push(format!("noun:{noun:?}"));
    }
}

#[test]
fn visitor_observes_owned_declaration_identity_in_form_order() {
    let parser = parser();
    let ability = parser
        .parse("Destroy target player.", &context())
        .expect("open declaration form parses");
    let mut visitor = IdentityVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.events,
        ["declaration:KeywordAction/Destroy", "noun:Player"]
    );

    let mut explicit = IdentityVisitor::default();
    walk_ability(&mut explicit, &ability);
    assert_eq!(explicit.events, visitor.events);
}

#[test]
fn destroy_and_connive_have_no_closed_member_or_handwritten_spelling_authority() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let constructions = fs::read_to_string(root.join("constructions.rs")).unwrap();
    let features = fs::read_to_string(root.join("features.rs")).unwrap();
    let scanner = fs::read_to_string(root.join("parser/scan.rs")).unwrap();
    let scanner = scanner
        .split_once("#[cfg(test)]")
        .map_or(scanner.as_str(), |(production, _)| production);

    let enum_body = constructions
        .split_once("lexeme VerbLexeme {")
        .and_then(|(_, tail)| tail.split_once('}'))
        .map(|(body, _)| body)
        .expect("closed verb enum declaration exists");
    for forbidden in ["Destroy", "Connive"] {
        assert!(
            !enum_body.contains(forbidden),
            "closed member remains: {enum_body}"
        );
    }
    for forbidden in ["\"destroy\"", "\"destroys\"", "\"connive\"", "\"connives\""] {
        assert!(
            !features.contains(forbidden),
            "handwritten inflection remains: {forbidden}"
        );
        assert!(
            !scanner.contains(forbidden),
            "handwritten scanner spelling remains: {forbidden}"
        );
    }
}
