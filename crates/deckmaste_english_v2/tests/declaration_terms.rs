use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationIdentity;
use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarPosition;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_construction_core::macro_def::read_str;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::CounterKind;
use deckmaste_english_v2::ast::DeclaredCounterKind;
use deckmaste_english_v2::ast::DesignationTerm;
use deckmaste_english_v2::ast::KeywordAbility;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;

fn declaration(path: &str, source: &str) -> NormalizedDeclaration {
    read_str(path, source).expect("synthetic declaration is valid")
}

#[test]
fn fixed_declaration_term_codecs_reject_wrong_kinds_and_positions() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations load");
    declarations.extend([
        declaration(
            "/same-plugin/abilities/Quorbling.ron",
            r#"KeywordAbility(name:"Quorbling",spelling:"quorbling",grammar:FixedKeyword(surface:"quorbling"))"#,
        ),
        declaration(
            "/same-plugin/counter_kinds/Quest.ron",
            r#"CounterKind(name:"Quest",spelling:"quest",grammar:FixedTerm(surface:"quest"))"#,
        ),
        declaration(
            "/same-plugin/designations/FeaturedGame.ron",
            r#"Designation(name:"FeaturedGame",spelling:"the featured game",grammar:FixedTerm(surface:"the featured game"))"#,
        ),
        declaration(
            "/wrong-position/abilities/RunningWord.ron",
            r#"KeywordAbility(name:"RunningWord",spelling:"running word",grammar:FixedTerm(surface:"running word"))"#,
        ),
        declaration(
            "/wrong-position/counter_kinds/KeywordCounter.ron",
            r#"CounterKind(name:"KeywordCounter",spelling:"keyword counter",grammar:FixedKeyword(surface:"keyword counter"))"#,
        ),
        declaration(
            "/wrong-position/designations/KeywordDesignation.ron",
            r#"Designation(name:"KeywordDesignation",spelling:"keyword designation",grammar:FixedKeyword(surface:"keyword designation"))"#,
        ),
    ]);
    let environment = ParserEnvironment::try_from_declarations(declarations.clone())
        .expect("builtin and same-plugin declarations freeze");

    let mut keyword_abilities = 0;
    let mut lexical_counter_kinds = 0;
    let mut nonlexical_counter_kinds = 0;
    let mut designations = 0;

    for declaration in &declarations {
        let id = declaration.identity().clone();
        let position = declaration
            .grammar()
            .map(|grammar| grammar.recipe().position());
        match id.kind() {
            DeclarationKind::KeywordAbility if position == Some(GrammarPosition::FixedKeyword) => {
                KeywordAbility::new(&environment, id.clone()).unwrap_or_else(|| {
                    panic!("keyword ability did not load through its consumer: {id}")
                });
                keyword_abilities += 1;
            }
            DeclarationKind::CounterKind if position == Some(GrammarPosition::FixedTerm) => {
                DeclaredCounterKind::new(&environment, id.clone()).unwrap_or_else(|| {
                    panic!("counter kind did not load through its consumer: {id}")
                });
                lexical_counter_kinds += 1;
            }
            DeclarationKind::CounterKind if declaration.grammar().is_none() => {
                nonlexical_counter_kinds += 1;
            }
            DeclarationKind::Designation if position == Some(GrammarPosition::FixedTerm) => {
                DesignationTerm::new(&environment, id.clone()).unwrap_or_else(|| {
                    panic!("designation did not load through its consumer: {id}")
                });
                designations += 1;
            }
            _ => {}
        }
    }

    assert_eq!(
        keyword_abilities, 196,
        "195 builtins plus one same-plugin row"
    );
    assert_eq!(
        lexical_counter_kinds, 32,
        "31 builtins plus one same-plugin row"
    );
    assert_eq!(
        nonlexical_counter_kinds, 42,
        "two structured P/T identities plus forty semantics-only declarations stay grammar-less"
    );
    assert_eq!(designations, 20, "19 builtins plus one same-plugin row");

    let running_word = environment
        .declaration(DeclarationKind::KeywordAbility, "RunningWord")
        .expect("wrong-position ability remains indexed");
    let keyword_counter = environment
        .declaration(DeclarationKind::CounterKind, "KeywordCounter")
        .expect("wrong-position counter remains indexed");
    let keyword_designation = environment
        .declaration(DeclarationKind::Designation, "KeywordDesignation")
        .expect("wrong-position designation remains indexed");
    let quest = environment
        .declaration(DeclarationKind::CounterKind, "Quest")
        .expect("same-plugin counter remains indexed");
    let featured_game = environment
        .declaration(DeclarationKind::Designation, "FeaturedGame")
        .expect("same-plugin designation remains indexed");

    assert!(KeywordAbility::new(&environment, running_word.id().clone()).is_none());
    assert!(DeclaredCounterKind::new(&environment, keyword_counter.id().clone()).is_none());
    assert!(DesignationTerm::new(&environment, keyword_designation.id().clone()).is_none());
    assert!(KeywordAbility::new(&environment, quest.id().clone()).is_none());
    assert!(DeclaredCounterKind::new(&environment, featured_game.id().clone()).is_none());
    assert!(DesignationTerm::new(&environment, quest.id().clone()).is_none());
}

#[test]
fn counter_and_designation_terms_parse_render_visit_and_own_exactly() {
    #[derive(Default)]
    struct Declarations(Vec<(DeclarationKind, String)>);

    impl Visitor for Declarations {
        fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
            self.0
                .push((declaration.kind(), declaration.name().to_owned()));
        }
    }

    let mut declarations =
        read_builtin_v2(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("builtin-v2 declarations load");
    declarations.extend([
        declaration(
            "/same-plugin/counter_kinds/Quest.ron",
            r#"CounterKind(name:"Quest",spelling:"quest",grammar:FixedTerm(surface:"quest"))"#,
        ),
        declaration(
            "/same-plugin/designations/FeaturedGame.ron",
            r#"Designation(name:"FeaturedGame",spelling:"the featured game",grammar:FixedTerm(surface:"the featured game"))"#,
        ),
    ]);
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
    .expect("builtin and same-plugin declarations freeze");
    let parser = Parser::new(environment.clone()).expect("declaration-term grammar initializes");
    let context = ParseContext::new("Context Card", false, Onset::Consonant)
        .expect("synthetic card context is valid");

    for (text, kind, name) in [
        (
            "Put a shield counter on target creature.",
            DeclarationKind::CounterKind,
            "shieldCounter",
        ),
        (
            "Put a quest counter on target creature.",
            DeclarationKind::CounterKind,
            "Quest",
        ),
        (
            "You become the monarch.",
            DeclarationKind::Designation,
            "monarch",
        ),
        (
            "You become the featured game.",
            DeclarationKind::Designation,
            "FeaturedGame",
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let ability = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} selects: {analysis:?}"));
        assert_eq!(ability.render(&context, &environment), text);
        let ownership = analysis
            .ownership()
            .unwrap_or_else(|| panic!("{text:?} has selected ownership"));
        assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
        assert!(ownership.summary().covered(), "{text:?}: {ownership:?}");

        let mut visitor = Declarations::default();
        visitor.visit_ability(ability);
        assert!(
            visitor.0.contains(&(kind, name.to_owned())),
            "{text:?}: {:?}",
            visitor.0,
        );
    }
}

#[test]
fn structured_power_toughness_counters_remain_nonlexical_counter_kinds() {
    #[derive(Default)]
    struct CounterKinds(Vec<CounterKind>);

    impl Visitor for CounterKinds {
        fn visit_counter_kind(&mut self, value: &CounterKind) {
            self.0.push(value.clone());
        }
    }

    let environment = ParserEnvironment::try_from_parts(
        read_builtin_v2(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin"))
            .expect("builtin-v2 declarations load"),
        [CatalogProviderRows::new(
            CatalogProvider::CardNames,
            [CatalogProviderRow::new(
                "context-card",
                "Context Card",
                Onset::Consonant,
            )],
        )],
    )
    .expect("builtin declaration environment freezes");
    let parser = Parser::new(environment).expect("builtin environment supplies grammar rows");
    let context = ParseContext::new("Context Card", false, Onset::Consonant)
        .expect("synthetic card context is valid");

    let mut visitor = CounterKinds::default();
    for text in [
        "Put a +1/+1 counter on target creature.",
        "Put a -1/-1 counter on target creature.",
    ] {
        let ability = parser
            .parse(text, &context)
            .expect("structured counter parses");
        visitor.visit_ability(&ability);
    }

    assert!(matches!(
        visitor.0.as_slice(),
        [
            CounterKind::PositivePowerToughnessCounter(_),
            CounterKind::NegativePowerToughnessCounter(_),
        ]
    ));
}
