use std::path::Path;

use deckmaste_english_v2::ast::Amount;
use deckmaste_english_v2::ast::CounterKind;
use deckmaste_english_v2::ast::DeclaredCounterKind;
use deckmaste_english_v2::ast::Designation;
use deckmaste_english_v2::ast::KeywordAbility;
use deckmaste_english_v2::ast::NegativeCounterMagnitude;
use deckmaste_english_v2::ast::NegativePowerToughnessCounter;
use deckmaste_english_v2::ast::NumberAmount;
use deckmaste_english_v2::ast::PositiveCounterMagnitude;
use deckmaste_english_v2::ast::PositivePowerToughnessCounter;
use deckmaste_english_v2::ast::ScalarNumber;
use deckmaste_english_v2::environment::ParserEnvironment;
use macro_ron::v2::DeclarationKind;
use macro_ron::v2::GrammarPosition;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::read_builtin_v2;
use macro_ron::v2::read_str;

fn declaration(path: &str, source: &str) -> NormalizedDeclaration {
    read_str(path, source).expect("synthetic declaration is valid")
}

fn number_amount(magnitude: u32) -> Amount {
    Amount::Number(NumberAmount {
        number: ScalarNumber { magnitude },
    })
}

#[test]
fn fixed_declaration_term_inventories_load_through_their_generated_consumers() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
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
    let mut structured_counter_kinds = 0;
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
                structured_counter_kinds += 1;
            }
            DeclarationKind::Designation if position == Some(GrammarPosition::FixedTerm) => {
                Designation::new(&environment, id.clone()).unwrap_or_else(|| {
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
        lexical_counter_kinds, 26,
        "25 builtins plus one same-plugin row"
    );
    assert_eq!(
        structured_counter_kinds, 2,
        "the two structured P/T identities stay grammar-less"
    );
    assert_eq!(designations, 3, "two builtins plus one same-plugin row");

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
    assert!(Designation::new(&environment, keyword_designation.id().clone()).is_none());
    assert!(KeywordAbility::new(&environment, quest.id().clone()).is_none());
    assert!(DeclaredCounterKind::new(&environment, featured_game.id().clone()).is_none());
    assert!(Designation::new(&environment, quest.id().clone()).is_none());
}

#[test]
fn structured_power_toughness_counters_remain_nonlexical_counter_kinds() {
    let positive = PositivePowerToughnessCounter::new(vec![
        PositiveCounterMagnitude::PositiveCounterMagnitude(number_amount(1)),
        PositiveCounterMagnitude::PositiveCounterMagnitude(number_amount(1)),
    ])
    .expect("two positive magnitudes form one structured counter kind");
    let negative = NegativePowerToughnessCounter::new(vec![
        NegativeCounterMagnitude::NegativeCounterMagnitude(number_amount(1)),
        NegativeCounterMagnitude::NegativeCounterMagnitude(number_amount(1)),
    ])
    .expect("two negative magnitudes form one structured counter kind");

    assert!(matches!(
        CounterKind::PositivePowerToughnessCounter(positive),
        CounterKind::PositivePowerToughnessCounter(_)
    ));
    assert!(matches!(
        CounterKind::NegativePowerToughnessCounter(negative),
        CounterKind::NegativePowerToughnessCounter(_)
    ));
}
