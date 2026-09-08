use std::collections::BTreeMap;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_construction_core::macro_def::read_str;

/// The declaration name a catalog surface is written under: the surface's
/// words joined in camelCase, which is Lean's macro name for the same ability
/// word (`willOfTheCouncil`).
fn declaration_name(surface: &str) -> String {
    let joined = surface
        .split(' ')
        .map(|word| word.replace('\'', ""))
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(chars)
                .collect::<String>()
        })
        .collect::<String>();
    let mut chars = joined.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_lowercase)
        .chain(chars)
        .collect()
}

#[test]
fn builtin_v2_ability_word_nursery_matches_the_independent_catalog() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let catalog =
        std::fs::read_to_string(workspace_root.join("data/gen/catalogs/ability-words.txt"))
            .expect("the canonical ability-word catalog must load");
    let expected = catalog
        .lines()
        .map(|surface| (declaration_name(surface), surface.to_owned()))
        .collect::<BTreeMap<_, _>>();
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let actual = declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::AbilityWord)
        .map(|declaration| (declaration.identity().name().to_owned(), declaration))
        .collect::<BTreeMap<_, _>>();

    assert_eq!(expected.len(), 61);
    assert_eq!(actual.len(), 61);
    assert_eq!(
        actual.keys().collect::<Vec<_>>(),
        expected.keys().collect::<Vec<_>>()
    );

    let mut actual_surfaces = Vec::new();
    for (name, declaration) in &actual {
        assert_eq!(declaration.params(), None, "{name} must have no params");
        assert_eq!(declaration.body(), None, "{name} must have no body");
        let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
            panic!("{name} must have one literal spelling")
        };
        assert_eq!(spelling, &expected[name]);
        let grammar = declaration
            .grammar()
            .expect("every ability word must contribute grammar");
        assert_eq!(grammar.recipe(), &GrammarRecipe::FixedTerm);
        let [surface] = grammar.surfaces() else {
            panic!("{name} must contribute exactly one surface")
        };
        assert_eq!(surface.feature(), SurfaceFeature::Fixed);
        assert_eq!(surface.text(), spelling);
        actual_surfaces.push(surface.text().to_owned());
    }

    let mut expected_surfaces = catalog.lines().map(str::to_owned).collect::<Vec<_>>();
    actual_surfaces.sort();
    expected_surfaces.sort();
    assert_eq!(actual_surfaces, expected_surfaces);
}

#[test]
fn same_plugin_ability_word_needs_no_catalog_membership() {
    let synthetic = read_str(
        "same-plugin/Momentum.ron",
        r#"AbilityWord(
            name: "Momentum",
            spelling: "Momentum",
            grammar: FixedTerm(surface: "Momentum"),
        )"#,
    )
    .expect("a same-plugin ability word needs no catalog membership");

    assert_eq!(synthetic.identity().kind(), DeclarationKind::AbilityWord);
    assert_eq!(synthetic.params(), None);
    assert_eq!(synthetic.body(), None);
    assert_eq!(
        synthetic.grammar().unwrap().recipe(),
        &GrammarRecipe::FixedTerm
    );
}
