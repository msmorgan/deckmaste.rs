use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationIdentity;
use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::SubtypeCategory;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_construction_core::macro_def::read_str;
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

fn declarations() -> Vec<deckmaste_construction_core::macro_def::NormalizedDeclaration> {
    read_builtin_v2(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"))
        .expect("builtin-v2 declarations load")
}

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card", false, Onset::Consonant).expect("context is valid")
}

fn environment(
    declarations: Vec<deckmaste_construction_core::macro_def::NormalizedDeclaration>,
) -> ParserEnvironment {
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
        ("Kicker {2}", vec!["Kicker"]),
        ("Cycling {2}", vec!["Cycling"]),
        ("Cycling—{2}", vec!["Cycling"]),
        ("Ward {2}", vec!["Ward"]),
        ("Ward—{2}", vec!["Ward"]),
        ("Ward—Pay 3 life.", vec!["Ward"]),
        ("Ward—{2}, Pay 2 life.", vec!["Ward"]),
        ("Reinforce 3—{1}{G}", vec!["Reinforce"]),
        ("Reinforce 3 {1}{G}", vec!["Reinforce"]),
        ("Toxic 2", vec!["Toxic"]),
        ("Enchant creature you control", vec!["Enchant"]),
        ("Enchant creature or Vehicle", vec!["Enchant"]),
        ("Enchant artifact an opponent controls", vec!["Enchant"]),
        ("Enchant creature card in a graveyard", vec!["Enchant"]),
        ("Enchant creature with power 3 or less", vec!["Enchant"]),
        (
            "Enchant creature with another Aura attached to it",
            vec!["Enchant"],
        ),
        ("Enchant creature without flying", vec!["Enchant", "Flying"]),
        ("Affinity for artifacts", vec!["Affinity"]),
        ("Protection from black", vec!["Protection"]),
        ("LEVEL 1-3\n4/4", vec!["LevelUp"]),
        ("Protection from everything", vec!["Protection"]),
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
fn declared_quality_prepositions_stay_inside_keyword_abilities() {
    let environment = environment(declarations());
    let parser = Parser::new(environment.clone()).expect("keyword-line grammar initializes");
    let bare_analysis = parser.analyze_oracle_text("Affinity for Equipment", &context());
    let bare_decision = bare_analysis
        .decision()
        .expect("bare affinity has a selection decision");
    assert_eq!(bare_decision.selected(), Some(0));
    assert_eq!(bare_decision.survivors(), &[0]);

    let text = "Spells you cast have affinity for artifacts.";

    assert_exact_document(&parser, &environment, text);
    let analysis = parser.analyze_oracle_text(text, &context());
    let decision = analysis
        .decision()
        .expect("keyword grant has a selection decision");
    let selected = decision
        .candidates()
        .iter()
        .find(|candidate| Some(candidate.ordinal()) == decision.selected())
        .expect("keyword grant has one selected candidate");
    assert!(
        selected
            .construction_path()
            .iter()
            .any(|name| name == "QualifiedKeywordLineItemQualifiedKeywordLineItem"),
        "{:#?}",
        selected.construction_path(),
    );
    assert!(
        !selected
            .construction_path()
            .iter()
            .any(|name| name == "ReferencedQualityKeywordAbilityReferencedQualityKeywordAbility"),
        "{:#?}",
        selected.construction_path(),
    );

    for mismatched in ["Affinity from artifacts", "Protection for black"] {
        assert!(
            parser.parse_oracle_text(mismatched, &context()).is_err(),
            "{mismatched:?} must not override its declared parameter preposition",
        );
    }
}

#[test]
fn sentence_shaped_keyword_cost_requires_its_constituent_dash() {
    let environment = environment(declarations());
    let parser = Parser::new(environment).expect("keyword-line grammar initializes");
    let context = context();

    assert!(
        parser
            .parse_oracle_text("Ward Pay 3 life.", &context)
            .is_err()
    );
}

#[test]
fn keyword_lines_compose_in_every_document_block_position() {
    let environment = environment(declarations());
    let parser = Parser::new(environment.clone()).expect("keyword-line grammar initializes");

    for text in [
        "Flying\nDraw a card.",
        "Draw a card.\nFlying",
        "Draw a card.\nFlying\nDraw a card.",
    ] {
        assert_exact_document(&parser, &environment, text);
    }
}

#[test]
fn attachment_participial_adjectives_remain_declaration_backed() {
    let environment = environment(declarations());
    let parser = Parser::new(environment.clone()).expect("keyword-line grammar initializes");

    for (text, expected) in [
        ("Enchanted creature has flying.", "Enchant"),
        ("Enchanted creature can't block.", "Enchant"),
        ("An enchanted creature can't block.", "Enchant"),
        ("Target enchanted permanent can't block.", "Enchant"),
        ("Enchanted creatures can't block.", "Enchant"),
        ("This creature is enchanted.", "Enchant"),
        ("Equipped creature has haste.", "Equip"),
        ("Equipped creature can't block.", "Equip"),
        ("Equipped creatures can't block.", "Equip"),
        ("This creature is equipped.", "Equip"),
    ] {
        let parsed = assert_exact_document(&parser, &environment, text);
        let mut visitor = DeclarationVisitor::default();
        visitor.visit_oracle_text(&parsed);
        assert!(
            visitor
                .0
                .iter()
                .any(|(kind, name)| *kind == DeclarationKind::KeywordAbility && name == expected),
            "{text:?} visits its declaration-backed participial adjective",
        );
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
