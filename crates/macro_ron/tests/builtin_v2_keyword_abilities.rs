use std::collections::BTreeMap;
use std::path::Path;

use macro_ron::v2::DeclarationKind;
use macro_ron::v2::GrammarRecipe;
use macro_ron::v2::NormalizedDeclaration;
use macro_ron::v2::SpellingPart;
use macro_ron::v2::SurfaceFeature;
use macro_ron::v2::read_builtin_v2;

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
    let parameterized = BTreeMap::from([
        ("Enchant", ("Subject", "enchant ", "enchant")),
        ("Equip", ("Cost", "equip ", "equip")),
        ("Fortify", ("Cost", "fortify ", "fortify")),
        (
            "Protection",
            ("Quality", "protection from ", "protection from"),
        ),
        ("Ward", ("Cost", "ward ", "ward")),
    ]);

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
        assert_eq!(grammar.recipe(), &GrammarRecipe::FixedKeyword);
        assert_eq!(grammar.surfaces().len(), 1);
        assert_eq!(grammar.surfaces()[0].feature(), SurfaceFeature::Fixed);
        if let Some((parameter, prefix, surface)) = parameterized.get(name.as_str()) {
            let params = declaration.params().expect("parameterized row has params");
            assert_eq!(
                params
                    .iter()
                    .map(macro_ron::v2::ParameterType::as_str)
                    .collect::<Vec<_>>(),
                [*parameter],
                "{name}",
            );
            assert_eq!(
                declaration.spelling(),
                [
                    SpellingPart::Literal((*prefix).to_owned()),
                    SpellingPart::Param(0)
                ],
                "{name}",
            );
            assert_eq!(grammar.surfaces()[0].text(), *surface, "{name}");
        } else {
            assert!(declaration.params().is_none(), "{name}");
            let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
                panic!("nullary keyword {name} has a parameter hole")
            };
            assert_eq!(grammar.surfaces()[0].text(), spelling);
        }
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
        assert_eq!(grammar.recipe(), &GrammarRecipe::FixedKeyword);
        let adjective = grammar
            .participial_adjective()
            .expect("attachment keyword contributes a participial adjective");
        assert_eq!(adjective.feature(), SurfaceFeature::Participle);
        assert_eq!(adjective.text(), surface, "{name}");
    }
}
