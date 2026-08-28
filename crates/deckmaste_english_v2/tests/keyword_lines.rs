use std::path::Path;

use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::OracleText;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::ParseAnalysis;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use macro_ron::v2::DeclarationIdentity;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::Onset;
use macro_ron::v2::SubtypeCategory;
use macro_ron::v2::read_builtin_v2;
use macro_ron::v2::read_str;

fn declarations() -> Vec<macro_ron::v2::NormalizedDeclaration> {
    read_builtin_v2(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"))
        .expect("builtin-v2 declarations load")
}

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card", false, Onset::Consonant).expect("context is valid")
}

fn environment(declarations: Vec<macro_ron::v2::NormalizedDeclaration>) -> ParserEnvironment {
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
    .expect("declaration environment freezes")
}

#[derive(Default)]
struct DeclarationVisitor(Vec<(DeclarationKind, String)>);

impl Visitor for DeclarationVisitor {
    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        self.0
            .push((declaration.kind(), declaration.name().to_owned()));
    }
}

fn assert_exact_document(
    parser: &Parser,
    environment: &ParserEnvironment,
    text: &str,
) -> OracleText {
    let context = context();
    let parsed = parser
        .parse_oracle_text(text, &context)
        .unwrap_or_else(|error| panic!("{text:?} parses: {error:?}"));
    assert_eq!(parsed.render(&context, environment), text);

    let analysis: ParseAnalysis<OracleText> = parser.analyze_oracle_text(text, &context);
    assert_eq!(
        analysis.selected(),
        Some(&parsed),
        "{text:?} selects exactly"
    );
    let ownership = analysis
        .ownership()
        .unwrap_or_else(|| panic!("{text:?} has selected ownership"));
    assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
    assert!(ownership.summary().covered(), "{text:?}: {ownership:?}");
    parsed
}

#[test]
fn declared_keyword_lines_parse_render_visit_and_own_exactly() {
    let environment = environment(declarations());
    let parser = Parser::new(environment.clone()).expect("keyword-line grammar initializes");

    for (text, expected) in [
        ("Flying", vec!["Flying"]),
        ("Flying, haste", vec!["Flying", "Haste"]),
        ("Equip {2}", vec!["Equip"]),
        ("Ward {2}", vec!["Ward"]),
        ("Protection from black", vec!["Protection"]),
        ("Protection from black and from red", vec!["Protection"]),
        (
            "Protection from Vampires, from Werewolves, and from Zombies",
            vec!["Protection"],
        ),
    ] {
        let parsed = assert_exact_document(&parser, &environment, text);
        let mut visitor = DeclarationVisitor::default();
        visitor.visit_oracle_text(&parsed);
        let keywords = visitor
            .0
            .iter()
            .filter_map(|(kind, name)| {
                (*kind == DeclarationKind::KeywordAbility).then_some(name.as_str())
            })
            .collect::<Vec<_>>();
        assert_eq!(keywords, expected, "{text:?}");

        if text == "Protection from Vampires, from Werewolves, and from Zombies" {
            let creature_subtypes = visitor
                .0
                .iter()
                .filter_map(|(kind, name)| {
                    (*kind == DeclarationKind::Subtype(SubtypeCategory::Creature))
                        .then_some(name.as_str())
                })
                .collect::<Vec<_>>();
            assert_eq!(creature_subtypes, ["Vampire", "Werewolf", "Zombie"]);
        }
    }
}

#[test]
fn keyword_line_vocabulary_is_open_but_declaration_backed() {
    let mut rows = declarations();
    rows.push(
        read_str(
            "/synthetic/keyword_abilities/Quorbling.ron",
            r#"KeywordAbility(name:"Quorbling",spelling:"quorbling",grammar:FixedKeyword(surface:"quorbling"))"#,
        )
        .expect("synthetic same-plugin keyword is valid"),
    );
    let environment = environment(rows);
    let parser = Parser::new(environment.clone()).expect("keyword-line grammar initializes");

    assert_exact_document(&parser, &environment, "Quorbling");
    assert!(
        parser.parse_oracle_text("Undeclared", &context()).is_err(),
        "an undeclared surface remains an ordinary silent parse failure",
    );
}
