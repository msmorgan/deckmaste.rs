use std::path::Path;

use macro_ron::v2::DeclarationKind;
use macro_ron::v2::GrammarRecipe;
use macro_ron::v2::SpellingPart;
use macro_ron::v2::read_builtin_v2;
use macro_ron::v2::read_str;

#[test]
fn builtin_v2_designations_preserve_identity_surfaces_and_definitions() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
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
        ["Commander", "Monarch"],
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

    let commander = &designations[0];
    assert_eq!(
        commander.spelling(),
        [SpellingPart::Literal("commander".to_owned())]
    );
    assert!(
        commander
            .body()
            .unwrap()
            .get_ron()
            .contains("scope: Object")
    );
    let monarch = &designations[1];
    assert_eq!(
        monarch.spelling(),
        [SpellingPart::Literal("the monarch".to_owned())]
    );
    assert!(monarch.body().unwrap().get_ron().contains("scope: Player"));
    assert!(
        monarch
            .body()
            .unwrap()
            .get_ron()
            .contains("uniqueness: PerGame")
    );

    let synthetic = read_str(
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
    assert_eq!(synthetic.identity().kind(), DeclarationKind::Designation);
    assert!(synthetic.body().unwrap().get_ron().contains("scope: Game"));
    assert!(synthetic.body().unwrap().get_ron().contains("confers:"));
}
