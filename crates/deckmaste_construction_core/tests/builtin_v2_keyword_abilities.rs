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
        return "infinity".to_owned();
    }
    let stem = head
        .split([' ', '-'])
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
        .collect::<String>();
    let mut chars = stem.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_lowercase)
        .chain(chars)
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
        "absorb",
        "afflict",
        "afterlife",
        "amplify",
        "annihilator",
        "backup",
        "bloodthirst",
        "bushido",
        "casualty",
        "crew",
        "devour",
        "dredge",
        "fabricate",
        "fading",
        "firebending",
        "frenzy",
        "graft",
        "hideaway",
        "mobilize",
        "modular",
        "poisonous",
        "rampage",
        "renown",
        "ripple",
        "saddle",
        "soulshift",
        "teamwork",
        "toxic",
        "tribute",
        "vanishing",
    ] {
        assert_eq!(parameterized.insert(name, &["Amount"]), None);
    }
    for name in [
        "auraSwap",
        "bestow",
        "blitz",
        "buyback",
        "cleave",
        "craft",
        "cumulativeUpkeep",
        "cycling",
        "dash",
        "disguise",
        "disturb",
        "echo",
        "embalm",
        "emerge",
        "encore",
        "entwine",
        "equip",
        "escalate",
        "escape",
        "eternalize",
        "evoke",
        "flashback",
        "foretell",
        "fortify",
        "freerunning",
        "harmonize",
        "kicker",
        "levelUp",
        "madness",
        "mayhem",
        "miracle",
        "moreThanMeetsTheEye",
        "morph",
        "mutate",
        "ninjutsu",
        "offspring",
        "outlast",
        "overload",
        "plot",
        "prowl",
        "reconfigure",
        "recover",
        "replicate",
        "scavenge",
        "sneak",
        "spectacle",
        "squad",
        "surge",
        "transfigure",
        "transmute",
        "unearth",
        "ward",
        "warp",
        "webSlinging",
    ] {
        assert_eq!(parameterized.insert(name, &["Cost"]), None);
    }
    for name in ["affinity", "landwalk", "offering", "protection"] {
        assert_eq!(parameterized.insert(name, &["Quality"]), None);
    }
    for name in ["champion", "enchant", "gift"] {
        assert_eq!(parameterized.insert(name, &["Subject"]), None);
    }
    for name in ["awaken", "impending", "reinforce", "suspend"] {
        assert_eq!(parameterized.insert(name, &["Amount", "Cost"]), None);
    }
    assert_eq!(parameterized.insert("splice", &["Quality", "Cost"]), None);
    for name in [
        "boast", "exhaust", "forecast", "infinity", "maxSpeed", "powerUp", "solved", "visit",
    ] {
        assert_eq!(parameterized.insert(name, &["Ability"]), None);
    }
    assert_eq!(parameterized.insert("companion", &["Condition"]), None);
    assert_eq!(
        parameterized.insert("prototype", &["Cost", "Power", "Toughness"]),
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
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
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
        // Re-spelled by `semantics-v2-macro-bodies-keyword-abilities`, which
        // gives each keyword ability its own definition as the declaration's
        // body: the family is no longer a grammar-only nursery. What the
        // assertion protects is unchanged — a body never lands without the
        // signature that reads it, which is exactly what graduation means.
        assert_eq!(
            declaration.is_graduated(),
            declaration.body().is_some(),
            "{name}: a body and a signature land together"
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
            "affinity" => Some(FixedKeywordParameterGrammar::Quality {
                preposition: FixedLexeme("Preposition".to_owned(), "For".to_owned()),
                nominal_number: Some(FixedKeywordNominalNumber::Plural),
            }),
            "protection" => Some(FixedKeywordParameterGrammar::Quality {
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
            grammar
                .surfaces()
                .iter()
                .filter(|surface| surface.feature() == SurfaceFeature::Fixed)
                .count(),
            1,
            "{name}",
        );
        assert_eq!(grammar.surfaces()[0].feature(), SurfaceFeature::Fixed);
        assert!(
            grammar.surfaces()[1..]
                .iter()
                .all(|surface| surface.feature() == SurfaceFeature::BoundSuffix),
            "{name}",
        );
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
            // Re-spelled with the assertion above: a body may not land without
            // a signature (`ValidationError::BodyWithoutSignature`), so a
            // paramless keyword carrying its definition writes `params: []`
            // where a nursery record wrote nothing. Either reads as "this
            // keyword takes no argument".
            assert!(
                declaration
                    .params()
                    .map_or(true, |params| params.is_empty()),
                "{name}"
            );
        }
        let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
            panic!("keyword {name} declares compiler-owned layout in its spelling")
        };
        assert_eq!(grammar.surfaces()[0].text(), spelling, "{name}");
    }

    for (name, surface) in [
        ("flying", "flying"),
        ("doubleStrike", "double strike"),
        ("forMirrodin", "for Mirrodin!"),
        ("moreThanMeetsTheEye", "More Than Meets the Eye"),
        ("startYourEngines", "start your engines!"),
        ("webSlinging", "web-slinging"),
        ("infinity", "∞"),
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
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let grammar = ability(&declarations, "landwalk")
        .grammar()
        .expect("Landwalk contributes keyword grammar");

    assert_eq!(grammar.surfaces()[0].text(), "landwalk");
    assert_eq!(grammar.surfaces()[1].feature(), SurfaceFeature::BoundSuffix);
    assert_eq!(grammar.surfaces()[1].text(), "walk");
}

#[test]
fn prepositional_quality_keywords_declare_their_selected_markers() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");

    for (name, member, nominal_number) in [
        ("affinity", "For", Some(FixedKeywordNominalNumber::Plural)),
        ("protection", "From", None),
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
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");

    for name in [
        "boast", "exhaust", "forecast", "infinity", "maxSpeed", "powerUp", "solved", "visit",
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
        ability(&declarations, "companion").keyword_parameter_class(),
        Some(KeywordParameterClass::Unsupported(
            UnsupportedKeywordParameterClass::Condition
        )),
    );
    assert_eq!(
        ability(&declarations, "prototype").keyword_parameter_class(),
        Some(KeywordParameterClass::Unsupported(
            UnsupportedKeywordParameterClass::CostPowerToughness
        )),
    );
}

#[test]
fn attachment_keywords_declare_their_participial_adjective_surfaces() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    for (name, surface) in [
        ("equip", "equipped"),
        ("enchant", "enchanted"),
        ("fortify", "fortified"),
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
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let grammar = ability(&declarations, "levelUp")
        .grammar()
        .expect("Level Up contributes keyword grammar");

    assert_eq!(grammar.surfaces()[0].text(), "level up");
    let label = grammar
        .block_label()
        .expect("Level Up contributes its level-band label");
    assert_eq!(label.feature(), SurfaceFeature::BlockLabel);
    assert_eq!(label.text(), "LEVEL");
}
