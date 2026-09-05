use std::collections::BTreeMap;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::FixedKeywordNominalNumber;
use deckmaste_construction_core::macro_def::FixedKeywordParameterGrammar;
use deckmaste_construction_core::macro_def::FixedLexeme;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::KeywordParameterClass;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::UnsupportedKeywordParameterClass;
use deckmaste_construction_core::macro_def::read_builtin_v2;

fn declaration_name(head: &str) -> String {
    if head == "∞" {
        return "Infinity".to_owned();
    }
    head.split([' ', '-'])
        .map(|word| word.replace(['\'', '!'], ""))
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(chars.flat_map(char::to_lowercase))
                .collect::<String>()
        })
        .collect()
}

fn ability<'a>(declarations: &'a [NormalizedDeclaration], name: &str) -> &'a NormalizedDeclaration {
    declarations
        .iter()
        .find(|declaration| {
            declaration.identity().kind() == DeclarationKind::KeywordAbility
                && declaration.identity().name() == name
        })
        .unwrap_or_else(|| panic!("missing keyword ability {name}"))
}

fn parameterized_keyword_params() -> BTreeMap<&'static str, &'static [&'static str]> {
    let mut parameterized = BTreeMap::<&str, &[&str]>::new();
    for name in [
        "Absorb",
        "Afflict",
        "Afterlife",
        "Amplify",
        "Annihilator",
        "Backup",
        "Bloodthirst",
        "Bushido",
        "Casualty",
        "Crew",
        "Devour",
        "Dredge",
        "Fabricate",
        "Fading",
        "Firebending",
        "Frenzy",
        "Graft",
        "Hideaway",
        "Mobilize",
        "Modular",
        "Poisonous",
        "Rampage",
        "Renown",
        "Ripple",
        "Saddle",
        "Soulshift",
        "Teamwork",
        "Toxic",
        "Tribute",
        "Vanishing",
    ] {
        assert_eq!(parameterized.insert(name, &["Amount"]), None);
    }
    for name in [
        "AuraSwap",
        "Bestow",
        "Blitz",
        "Buyback",
        "Cleave",
        "Craft",
        "CumulativeUpkeep",
        "Cycling",
        "Dash",
        "Disguise",
        "Disturb",
        "Echo",
        "Embalm",
        "Emerge",
        "Encore",
        "Entwine",
        "Equip",
        "Escalate",
        "Escape",
        "Eternalize",
        "Evoke",
        "Flashback",
        "Foretell",
        "Fortify",
        "Freerunning",
        "Harmonize",
        "Kicker",
        "LevelUp",
        "Madness",
        "Mayhem",
        "Miracle",
        "MoreThanMeetsTheEye",
        "Morph",
        "Mutate",
        "Ninjutsu",
        "Offspring",
        "Outlast",
        "Overload",
        "Plot",
        "Prowl",
        "Reconfigure",
        "Recover",
        "Replicate",
        "Scavenge",
        "Sneak",
        "Spectacle",
        "Squad",
        "Surge",
        "Transfigure",
        "Transmute",
        "Unearth",
        "Ward",
        "Warp",
        "WebSlinging",
    ] {
        assert_eq!(parameterized.insert(name, &["Cost"]), None);
    }
    for name in ["Affinity", "Landwalk", "Offering", "Protection"] {
        assert_eq!(parameterized.insert(name, &["Quality"]), None);
    }
    for name in ["Champion", "Enchant", "Gift"] {
        assert_eq!(parameterized.insert(name, &["Subject"]), None);
    }
    for name in ["Awaken", "Impending", "Reinforce", "Suspend"] {
        assert_eq!(parameterized.insert(name, &["Amount", "Cost"]), None);
    }
    assert_eq!(parameterized.insert("Splice", &["Quality", "Cost"]), None);
    for name in [
        "Boast", "Exhaust", "Forecast", "Infinity", "MaxSpeed", "PowerUp", "Solved", "Visit",
    ] {
        assert_eq!(parameterized.insert(name, &["Ability"]), None);
    }
    assert_eq!(parameterized.insert("Companion", &["Condition"]), None);
    assert_eq!(
        parameterized.insert("Prototype", &["Cost", "Power", "Toughness"]),
        None
    );
    assert_eq!(parameterized.len(), 106);
    parameterized
}

#[test]
fn builtin_v2_keyword_ability_nursery_is_complete_and_normalized() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let catalog =
        std::fs::read_to_string(workspace_root.join("data/gen/catalogs/keyword-abilities.txt"))
            .expect("the canonical keyword-ability catalog must load");
    let expected = catalog
        .lines()
        .map(|head| (declaration_name(head), head))
        .collect::<BTreeMap<_, _>>();
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");
    let actual = declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::KeywordAbility)
        .map(|declaration| (declaration.identity().name().to_owned(), declaration))
        .collect::<BTreeMap<_, _>>();
    let parameterized = parameterized_keyword_params();

    assert_eq!(actual.len(), 195);
    assert_eq!(
        actual.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>()
    );
    for (name, declaration) in &actual {
        assert!(
            !declaration.is_graduated(),
            "{name} must remain a nursery declaration"
        );
        assert_eq!(
            declaration
                .provenance()
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str()),
            Some(name.as_str()),
        );
        let grammar = declaration
            .grammar()
            .expect("every keyword stub contributes grammar");
        let parameter = match name.as_str() {
            "Affinity" => Some(FixedKeywordParameterGrammar::Quality {
                preposition: FixedLexeme("Preposition".to_owned(), "For".to_owned()),
                nominal_number: Some(FixedKeywordNominalNumber::Plural),
            }),
            "Protection" => Some(FixedKeywordParameterGrammar::Quality {
                preposition: FixedLexeme("Preposition".to_owned(), "From".to_owned()),
                nominal_number: None,
            }),
            _ => None,
        };
        assert_eq!(
            grammar.recipe(),
            &GrammarRecipe::FixedKeyword { parameter },
            "{name}",
        );
        assert_eq!(
            grammar.surfaces().len(),
            usize::from(name == "Landwalk") + 1
        );
        assert_eq!(grammar.surfaces()[0].feature(), SurfaceFeature::Fixed);
        if let Some(expected_params) = parameterized.get(name.as_str()) {
            let params = declaration.params().expect("parameterized row has params");
            assert_eq!(
                params
                    .iter()
                    .map(deckmaste_construction_core::macro_def::ParameterType::as_str)
                    .collect::<Vec<_>>(),
                *expected_params,
                "{name}",
            );
        } else {
            assert!(declaration.params().is_none(), "{name}");
        }
        let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
            panic!("keyword {name} declares compiler-owned layout in its spelling")
        };
        assert_eq!(grammar.surfaces()[0].text(), spelling, "{name}");
    }

    for (name, surface) in [
        ("Flying", "flying"),
        ("DoubleStrike", "double strike"),
        ("ForMirrodin", "for Mirrodin!"),
        ("MoreThanMeetsTheEye", "More Than Meets the Eye"),
        ("StartYourEngines", "start your engines!"),
        ("WebSlinging", "web-slinging"),
        ("Infinity", "∞"),
    ] {
        let declaration = ability(&declarations, name);
        assert_eq!(
            declaration.spelling(),
            [SpellingPart::Literal(surface.to_owned())]
        );
        assert_eq!(declaration.grammar().unwrap().surfaces()[0].text(), surface);
    }
}

#[test]
fn landwalk_declares_one_bound_suffix_surface() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");
    let grammar = ability(&declarations, "Landwalk")
        .grammar()
        .expect("Landwalk contributes keyword grammar");

    assert_eq!(grammar.surfaces()[0].text(), "landwalk");
    assert_eq!(grammar.surfaces()[1].feature(), SurfaceFeature::BoundSuffix);
    assert_eq!(grammar.surfaces()[1].text(), "walk");
}

#[test]
fn prepositional_quality_keywords_declare_their_selected_markers() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");

    for (name, member, nominal_number) in [
        ("Affinity", "For", Some(FixedKeywordNominalNumber::Plural)),
        ("Protection", "From", None),
    ] {
        assert_eq!(
            ability(&declarations, name).grammar().unwrap().recipe(),
            &GrammarRecipe::FixedKeyword {
                parameter: Some(FixedKeywordParameterGrammar::Quality {
                    preposition: FixedLexeme("Preposition".to_owned(), member.to_owned()),
                    nominal_number,
                }),
            },
            "{name}",
        );
    }
}

#[test]
fn unsupported_keyword_parameter_families_are_explicitly_deferred() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");

    for name in [
        "Boast", "Exhaust", "Forecast", "Infinity", "MaxSpeed", "PowerUp", "Solved", "Visit",
    ] {
        assert_eq!(
            ability(&declarations, name).keyword_parameter_class(),
            Some(KeywordParameterClass::Unsupported(
                UnsupportedKeywordParameterClass::Ability
            )),
            "{name}",
        );
    }
    assert_eq!(
        ability(&declarations, "Companion").keyword_parameter_class(),
        Some(KeywordParameterClass::Unsupported(
            UnsupportedKeywordParameterClass::Condition
        )),
    );
    assert_eq!(
        ability(&declarations, "Prototype").keyword_parameter_class(),
        Some(KeywordParameterClass::Unsupported(
            UnsupportedKeywordParameterClass::CostPowerToughness
        )),
    );
}

#[test]
fn attachment_keywords_declare_their_participial_adjective_surfaces() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");
    for (name, surface) in [
        ("Equip", "equipped"),
        ("Enchant", "enchanted"),
        ("Fortify", "fortified"),
    ] {
        let grammar = ability(&declarations, name)
            .grammar()
            .expect("keyword declaration contributes its primary grammar");
        assert_eq!(
            grammar.recipe(),
            &GrammarRecipe::FixedKeyword { parameter: None }
        );
        let adjective = grammar
            .participial_adjective()
            .expect("attachment keyword contributes a participial adjective");
        assert_eq!(adjective.feature(), SurfaceFeature::PAST_PARTICIPLE);
        assert_eq!(adjective.text(), surface, "{name}");
    }
}

#[test]
fn level_up_declares_its_distinct_block_label_surface() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load");
    let grammar = ability(&declarations, "LevelUp")
        .grammar()
        .expect("Level Up contributes keyword grammar");

    assert_eq!(grammar.surfaces()[0].text(), "level up");
    let label = grammar
        .block_label()
        .expect("Level Up contributes its level-band label");
    assert_eq!(label.feature(), SurfaceFeature::BlockLabel);
    assert_eq!(label.text(), "LEVEL");
}
