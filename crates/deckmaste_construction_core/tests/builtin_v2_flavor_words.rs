use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_data::mtgjson::AtomicCards;
use deckmaste_data::scryfall::Catalog;

#[test]
fn builtin_v2_flavor_word_nursery_matches_the_corpus_census() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let catalog_bytes = std::fs::read(workspace_root.join("data/catalogs/flavor-words.json"))
        .expect("the independent flavor-word catalog must load");
    let catalog = Catalog::parse(&catalog_bytes).expect("the flavor-word catalog must parse");
    let catalog = catalog
        .data
        .iter()
        .map(deckmaste_data::DataStr::as_str)
        .collect::<BTreeSet<_>>();
    let ability_word_text =
        std::fs::read_to_string(workspace_root.join("data/gen/catalogs/ability-words.txt"))
            .expect("the independent ability-word catalog must load");
    let ability_words = ability_word_text.lines().collect::<BTreeSet<_>>();
    let card_bytes = std::fs::read(workspace_root.join("data/mtgjson/AtomicCards.json"))
        .expect("the corpus snapshot must load");
    let cards = AtomicCards::parse(&card_bytes).expect("the corpus snapshot must parse");
    let expected = deckmaste_data::flavor_words::census(&cards, &catalog, &ability_words);

    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let actual = declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::FlavorWord)
        .map(|declaration| {
            let [SpellingPart::Literal(surface)] = declaration.spelling() else {
                panic!("every flavor word must have one literal spelling")
            };
            (surface.to_owned(), declaration)
        })
        .collect::<BTreeMap<_, _>>();

    assert_eq!(
        actual.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        expected,
    );
    for (surface, declaration) in actual {
        assert_eq!(declaration.params(), None, "{surface} must have no params");
        assert_eq!(declaration.body(), None, "{surface} must have no body");
        assert_eq!(
            declaration
                .provenance()
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str()),
            Some(declaration.identity().name()),
        );
        let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
            panic!("{surface} must have one literal spelling")
        };
        assert_eq!(spelling, &surface);
        let grammar = declaration
            .grammar()
            .expect("every flavor word must contribute grammar");
        assert_eq!(grammar.recipe(), &GrammarRecipe::FixedTerm);
        let [grammar_surface] = grammar.surfaces() else {
            panic!("{surface} must contribute exactly one surface")
        };
        assert_eq!(grammar_surface.feature(), SurfaceFeature::Fixed);
        assert_eq!(grammar_surface.text(), surface);
    }
}
