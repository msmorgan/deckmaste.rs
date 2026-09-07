use std::collections::BTreeMap;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::read_builtin_v2;

#[test]
fn turn_parts_contribute_to_the_noun_inventory_with_derived_plurals() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 turn-part declarations must load through the production reader");
    let rows = declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::TurnPart)
        .map(|declaration| (declaration.identity().name(), declaration))
        .collect::<BTreeMap<_, _>>();
    let expected = [
        (
            "BeginningOfCombatStep",
            "beginning of combat step",
            "beginning of combat steps",
        ),
        ("BeginningPhase", "beginning phase", "beginning phases"),
        ("Cleanup", "cleanup", "cleanups"),
        ("CleanupStep", "cleanup step", "cleanup steps"),
        ("Combat", "combat", "combats"),
        (
            "CombatDamageStep",
            "combat damage step",
            "combat damage steps",
        ),
        ("CombatPhase", "combat phase", "combat phases"),
        (
            "DeclareAttackersStep",
            "declare attackers step",
            "declare attackers steps",
        ),
        (
            "DeclareBlockersStep",
            "declare blockers step",
            "declare blockers steps",
        ),
        ("DrawStep", "draw step", "draw steps"),
        (
            "EndOfCombatStep",
            "end of combat step",
            "end of combat steps",
        ),
        ("EndStep", "end step", "end steps"),
        ("EndingPhase", "ending phase", "ending phases"),
        ("MainPhase", "main phase", "main phases"),
        (
            "PostcombatMainPhase",
            "postcombat main phase",
            "postcombat main phases",
        ),
        (
            "PrecombatMainPhase",
            "precombat main phase",
            "precombat main phases",
        ),
        ("UntapStep", "untap step", "untap steps"),
        ("Upkeep", "upkeep", "upkeeps"),
        ("UpkeepStep", "upkeep step", "upkeep steps"),
    ];

    assert_eq!(rows.len(), expected.len());
    for (name, singular, plural) in expected {
        let row = rows.get(name).unwrap_or_else(|| panic!("missing {name}"));
        let grammar = row
            .grammar()
            .unwrap_or_else(|| panic!("{name} noun grammar"));
        assert_eq!(grammar.recipe(), &GrammarRecipe::Noun);
        assert_eq!(
            grammar
                .surfaces()
                .iter()
                .map(|surface| (surface.feature(), surface.text()))
                .collect::<Vec<_>>(),
            [
                (SurfaceFeature::Singular, singular),
                (SurfaceFeature::Plural, plural),
            ]
        );

        let source = std::fs::read_to_string(row.provenance().path())
            .unwrap_or_else(|error| panic!("read {name} declaration: {error}"));
        assert!(
            !source.contains("plural"),
            "{name} must use recipe-derived number surfaces"
        );
    }
}
