use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_construction_core::macro_def::read_str;

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
    "SporeCounter",
    "StunCounter",
    "TimeCounter",
    "TrampleCounter",
    "VerseCounter",
    "VigilanceCounter",
];

const SEMANTIC_ONLY_NAMES: &[&str] = &[
    "BloodCounter",
    "BloodstainCounter",
    "BountyCounter",
    "BrickCounter",
    "DepletionCounter",
    "DivinityCounter",
    "DoomCounter",
    "DreamCounter",
    "EggCounter",
    "FinalityCounter",
    "FloodCounter",
    "FungusCounter",
    "FuseCounter",
    "GrowthCounter",
    "HoneCounter",
    "HourCounter",
    "IceCounter",
    "InterventionCounter",
    "KiCounter",
    "LevelCounter",
    "OmenCounter",
    "PageCounter",
    "PlagueCounter",
    "PlanCounter",
    "QuestCounter",
    "RadCounter",
    "RevCounter",
    "ScreamCounter",
    "SleightCounter",
    "SlimeCounter",
    "SoulCounter",
    "SpiteCounter",
    "StashCounter",
    "StorageCounter",
    "StrikeCounter",
    "StudyCounter",
    "SuspectCounter",
    "TideCounter",
    "WindCounter",
    "WishCounter",
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
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let counters = declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::CounterKind)
        .collect::<Vec<_>>();

    let mut expected = EXPECTED_NAMES.to_vec();
    expected.extend_from_slice(SEMANTIC_ONLY_NAMES);
    expected.sort_unstable();
    assert_eq!(
        counters
            .iter()
            .map(|declaration| declaration.identity().name())
            .collect::<Vec<_>>(),
        expected,
    );
    for declaration in &counters {
        assert_eq!(declaration.params(), Some([].as_slice()));
        assert!(declaration.body().is_some());
        let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
            panic!("{} has one literal spelling", declaration.identity())
        };
        assert!(!spelling.ends_with(" counter"));
        if matches!(declaration.identity().name(), "P1P1Counter" | "M1M1Counter")
            || SEMANTIC_ONLY_NAMES.contains(&declaration.identity().name())
        {
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
