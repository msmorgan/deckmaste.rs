use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_construction_core::macro_def::read_str;

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
            "goaded",
            "harnessed",
            "initiative",
            "leftHalfUnlocked",
            "level",
            "monarch",
            "monstrous",
            "prepared",
            "renowned",
            "rightHalfUnlocked",
            "ringBearer",
            "saddled",
            "sector",
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
        assert_eq!(
            declaration.grammar().unwrap().recipe(),
            &GrammarRecipe::FixedTerm
        );
        let [SpellingPart::Literal(spelling)] = declaration.spelling() else {
            panic!("{} has one literal spelling", declaration.identity())
        };
        assert_eq!(
            declaration.grammar().unwrap().surfaces()[0].text(),
            spelling
        );
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

    let monarch = &designations[9];
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
            .contains(r#"label: "the city's blessing""#)
    );
    assert!(
        citys_blessing
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: HeldBy(Player)")
    );

    // An enum-shaped designation declares one row per member [CR#731.1].
    let day_night = &designations[2];
    assert!(
        day_night
            .body()
            .unwrap()
            .get_ron()
            .contains(r#"label: "day", scope: HeldByGame"#)
    );
    assert!(
        day_night
            .body()
            .unwrap()
            .get_ron()
            .contains(r#"label: "night", scope: HeldByGame"#)
    );

    // An object-held designation is read on the battlefield.
    let goaded = &designations[4];
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

    let level = &designations[8];
    assert!(
        level
            .body()
            .unwrap()
            .get_ron()
            .contains(r#"label: "level""#)
    );

    let sector = &designations[16];
    for label in ["alpha sector", "beta sector", "gamma sector"] {
        assert!(
            sector
                .body()
                .unwrap()
                .get_ron()
                .contains(&format!(r#"label: "{label}""#))
        );
    }
}

#[test]
fn same_plugin_game_designation_can_retain_a_conferred_body() {
    let declaration = read_str(
        "same-plugin/FeaturedGame.ron",
        r#"Designation(
            name: "FeaturedGame",
            params: [],
            spelling: "the featured game",
            grammar: FixedTerm(surface: "the featured game"),
            body: DesignationDecl(
                name: "FeaturedGame",
                definition: Stored(
                    scope: Game,
                    shape: Flag,
                    uniqueness: PerGame,
                    persistence: Permanently,
                ),
                confers: [Continuous(This, Marker(Featured))],
            ),
        )"#,
    )
    .expect("a same-plugin game designation can retain a conferred body");
    assert_eq!(declaration.identity().kind(), DeclarationKind::Designation);
    assert!(
        declaration
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: Game")
    );
    assert!(declaration.body().unwrap().get_ron().contains("confers:"));
}
