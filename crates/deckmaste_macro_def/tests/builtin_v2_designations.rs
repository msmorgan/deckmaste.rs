use std::path::Path;

use deckmaste_macro_def::DeclarationKind;
use deckmaste_macro_def::GrammarRecipe;
use deckmaste_macro_def::SpellingPart;
use deckmaste_macro_def::read_builtin_v2;
use deckmaste_macro_def::read_str;

fn expected_recipe(name: &str) -> Option<GrammarRecipe> {
    match name {
        "commander" | "monarch" | "initiative" | "citysBlessing" | "ringBearer" => {
            Some(GrammarRecipe::Noun)
        }
        "prepared" | "saddled" | "monstrous" | "renowned" | "solved" => {
            Some(GrammarRecipe::Adjective)
        }
        "goaded" | "suspected" | "harnessed" | "foretold" | "plotted" => None,
        _ => Some(GrammarRecipe::FixedTerm),
    }
}

#[test]
fn builtin_v2_designations_preserve_identity_surfaces_and_definitions() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let designations = declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::Designation)
        .collect::<Vec<_>>();

    assert_eq!(
        designations
            .iter()
            .map(|declaration| declaration.identity().name())
            .collect::<Vec<_>>(),
        [
            "citysBlessing",
            "commander",
            "dayNight",
            "enduringStory",
            "foretold",
            "goaded",
            "harnessed",
            "initiative",
            "leftHalfUnlocked",
            "level",
            "monarch",
            "monstrous",
            "plotted",
            "prepared",
            "renowned",
            "rightHalfUnlocked",
            "ringBearer",
            "saddled",
            "solved",
            "suspected",
        ],
    );
    // `planar controller` deliberately has no nursery row: no supported card
    // text reads or writes it, so a declaration would contribute no consumer.
    assert!(
        designations
            .iter()
            .all(|declaration| declaration.identity().name() != "planarController")
    );
    for declaration in &designations {
        assert_eq!(declaration.params(), Some([].as_slice()));
        let expected = expected_recipe(declaration.identity().name());
        assert_eq!(
            declaration
                .grammar()
                .map(deckmaste_macro_def::GrammarRow::recipe),
            expected.as_ref()
        );
        let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
            panic!("{} has one literal spelling", declaration.identity())
        };
        if let Some(grammar) = declaration.grammar() {
            let singular = if grammar.recipe() == &GrammarRecipe::Noun {
                spelling.strip_prefix("the ").unwrap_or(spelling)
            } else {
                spelling
            };
            assert_eq!(grammar.surfaces()[0].text(), singular);
        }
    }

    let commander = &designations[1];
    assert_eq!(
        commander.spelling(),
        [SpellingPart::Literal("commander".to_owned())]
    );
    // The commander is an attribute of the card, retained across zone changes
    // [CR#903.3], and no instruction confers it.
    assert!(
        commander
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: HeldByCard")
    );
    assert!(
        commander
            .body()
            .unwrap()
            .get_ron()
            .contains("effectful: false")
    );

    let monarch = &designations[10];
    assert_eq!(
        monarch.spelling(),
        [SpellingPart::Literal("the monarch".to_owned())]
    );
    assert!(
        monarch
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: HeldBy(Player)")
    );

    let citys_blessing = &designations[0];
    assert!(
        citys_blessing
            .body()
            .unwrap()
            .get_ron()
            .contains(r#"label: Named(name: "citysBlessing")"#)
    );
    assert!(
        citys_blessing
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: HeldBy(Player)")
    );

    // A declaration covering several designations lists its members, one
    // designation per member [CR#731.1], with the declaration's columns.
    let day_night = &designations[2];
    assert!(
        day_night
            .body()
            .unwrap()
            .get_ron()
            .contains("members: [day, night]")
    );
    assert!(
        day_night
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: HeldByGame")
    );

    // An object-held designation is read on the battlefield.
    let goaded = &designations[5];
    assert!(
        goaded
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: HeldBy(Object)")
    );
    assert!(
        goaded
            .body()
            .unwrap()
            .get_ron()
            .contains("zone: Battlefield")
    );

    // Foretold and plotted are states of a card in exile, which the CR never
    // calls designations [CR#702.143d,702.170a]; the registry carries them as
    // object-held designations read in exile.
    for state in [&designations[4], &designations[12]] {
        let body = state.body().unwrap().get_ron();
        assert!(body.contains("scope: HeldBy(Object)"), "{}", state.identity());
        assert!(body.contains("zone: Exile"), "{}", state.identity());
    }

    let level = &designations[9];
    assert!(
        level
            .body()
            .unwrap()
            .get_ron()
            .contains(r#"label: Named(name: "level")"#)
    );
}

#[test]
fn same_plugin_game_designation_declares_its_columns() {
    // Re-spelled from `same_plugin_game_designation_can_retain_a_conferred_body`:
    // a designation declaration no longer writes a body (the meta builds the
    // definition from its columns and name), and a designation confers
    // nothing [CR#701.15b], so the same-plugin game designation declares its
    // columns and the derived body carries them under its own name.
    let declaration = read_str(
        "same-plugin/FeaturedGame.ron",
        r#"Designation(
            name: "FeaturedGame",
            params: [],
            spelling: "the featured game",
            grammar: FixedTerm(surface: "the featured game"),
            scope: HeldByGame,
            effectful: true,
        )"#,
    )
    .expect("a same-plugin game designation declares its columns");
    assert_eq!(declaration.identity().kind(), DeclarationKind::Designation);
    assert!(
        declaration
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: HeldByGame")
    );
    assert!(
        declaration
            .body()
            .unwrap()
            .get_ron()
            .contains(r#"label: Named(name: "FeaturedGame")"#)
    );
}
