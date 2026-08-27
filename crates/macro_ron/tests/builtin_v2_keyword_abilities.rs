use std::collections::BTreeMap;
use std::path::Path;

use macro_ron::v2::DeclarationKind;
use macro_ron::v2::DeterminativeFusedHeadLicense;
use macro_ron::v2::DeterminativeNominalLicense;
use macro_ron::v2::DeterminativeNumberLicense;
use macro_ron::v2::DeterminativePhraseNumber;
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
        assert!(matches!(declaration.spelling(), [SpellingPart::Literal(_)]));
        let grammar = declaration
            .grammar()
            .expect("every keyword stub contributes grammar");
        assert_eq!(grammar.recipe(), &GrammarRecipe::FixedKeyword);
        assert_eq!(grammar.surfaces().len(), 1);
        assert_eq!(grammar.surfaces()[0].feature(), SurfaceFeature::Fixed);
        let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
            unreachable!()
        };
        assert_eq!(grammar.surfaces()[0].text(), spelling);
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
fn attachment_keywords_declare_their_participial_determinative_rows() {
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
        let determinative = grammar
            .determinative()
            .expect("attachment keyword contributes an open Determinative row");
        assert_eq!(
            determinative.number_license(),
            DeterminativeNumberLicense::SingularOnly
        );
        assert_eq!(
            determinative.nominal_license(),
            DeterminativeNominalLicense::BareSingularNoun
        );
        assert_eq!(
            determinative.fused_head_license(),
            DeterminativeFusedHeadLicense::NominalOnly
        );
        let [realization] = determinative.realizations() else {
            panic!("{name} contributes one participial realization")
        };
        assert_eq!(realization.surface(), surface);
        assert_eq!(
            realization.phrase_number(),
            Some(DeterminativePhraseNumber::Singular)
        );
    }
}
