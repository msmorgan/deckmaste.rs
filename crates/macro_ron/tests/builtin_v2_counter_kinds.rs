use std::path::Path;

use macro_ron::v2::DeclarationKind;
use macro_ron::v2::GrammarRecipe;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::SpellingPart;
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::read_builtin_v2;
use macro_ron::v2::read_str;

const EXPECTED_NAMES: &[&str] = &[
    "AgeCounter",
    "ChargeCounter",
    "DeathtouchCounter",
    "DefenseCounter",
    "DoubleStrikeCounter",
    "Energy",
    "Experience",
    "FadeCounter",
    "FirstStrikeCounter",
    "FlyingCounter",
    "HasteCounter",
    "HexproofCounter",
    "IndestructibleCounter",
    "LifelinkCounter",
    "LoreCounter",
    "LoyaltyCounter",
    "LuckCounter",
    "M1M1Counter",
    "MenaceCounter",
    "OilCounter",
    "P1P1Counter",
    "Poison",
    "ReachCounter",
    "ShadowCounter",
    "ShieldCounter",
    "TrampleCounter",
    "VigilanceCounter",
];

fn counter<'a>(declarations: &'a [NormalizedDeclaration], name: &str) -> &'a NormalizedDeclaration {
    declarations
        .iter()
        .find(|declaration| {
            declaration.identity().kind() == DeclarationKind::CounterKind
                && declaration.identity().name() == name
        })
        .unwrap_or_else(|| panic!("missing counter kind {name}"))
}

#[test]
fn builtin_v2_counter_kinds_preserve_open_phrases_scopes_and_conferrals() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");
    let counters = declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::CounterKind)
        .collect::<Vec<_>>();

    assert_eq!(
        counters
            .iter()
            .map(|declaration| declaration.identity().name())
            .collect::<Vec<_>>(),
        EXPECTED_NAMES,
    );
    for declaration in &counters {
        assert_eq!(declaration.params(), Some([].as_slice()));
        assert!(declaration.body().is_some());
        let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
            panic!("{} has one literal spelling", declaration.identity())
        };
        assert!(!spelling.ends_with(" counter"));
        if matches!(declaration.identity().name(), "P1P1Counter" | "M1M1Counter") {
            assert!(declaration.grammar().is_none());
        } else {
            let grammar = declaration
                .grammar()
                .expect("lexical counter kind contributes grammar");
            assert_eq!(grammar.recipe(), &GrammarRecipe::FixedTerm);
            assert_eq!(grammar.surfaces()[0].feature(), SurfaceFeature::Fixed);
            assert_eq!(grammar.surfaces()[0].text(), spelling);
        }
    }

    assert_eq!(
        counter(&declarations, "DoubleStrikeCounter").spelling(),
        [SpellingPart::Literal("double strike".to_owned())],
    );
    assert!(
        counter(&declarations, "Energy")
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: Player")
    );
    assert!(
        counter(&declarations, "DeathtouchCounter")
            .body()
            .unwrap()
            .get_ron()
            .contains("GainAbility(Keyword(Deathtouch))")
    );
    assert!(
        counter(&declarations, "P1P1Counter")
            .body()
            .unwrap()
            .get_ron()
            .contains("CounterCount(This, P1P1Counter)")
    );

    let synthetic = read_str(
        "same-plugin/Quest.ron",
        r#"CounterKind(
            name: "Quest",
            params: [],
            spelling: "quest",
            grammar: FixedTerm(surface: "quest"),
            body: Counter(name: "Quest"),
        )"#,
    )
    .expect("a same-plugin single-token counter kind needs no catalog membership");
    assert_eq!(synthetic.identity().kind(), DeclarationKind::CounterKind);
}
