use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SubtypeCategory;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::ValidationError;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_construction_core::macro_def::read_str;

#[derive(Clone, Copy)]
struct CategorySpec {
    directory: &'static str,
    catalog: &'static str,
    category: SubtypeCategory,
    expected_count: usize,
}

const CATEGORIES: [CategorySpec; 6] = [
    CategorySpec {
        directory: "artifact",
        catalog: "artifact-types.txt",
        category: SubtypeCategory::Artifact,
        expected_count: 22,
    },
    CategorySpec {
        directory: "battle",
        catalog: "battle-types.txt",
        category: SubtypeCategory::Battle,
        expected_count: 1,
    },
    CategorySpec {
        directory: "enchantment",
        catalog: "enchantment-types.txt",
        category: SubtypeCategory::Enchantment,
        expected_count: 13,
    },
    CategorySpec {
        directory: "land",
        catalog: "land-types.txt",
        category: SubtypeCategory::Land,
        expected_count: 17,
    },
    CategorySpec {
        directory: "planeswalker",
        catalog: "planeswalker-types.txt",
        category: SubtypeCategory::Planeswalker,
        expected_count: 80,
    },
    CategorySpec {
        directory: "spell",
        catalog: "spell-types.txt",
        category: SubtypeCategory::Spell,
        expected_count: 5,
    },
];

fn subtype<'a>(
    declarations: &'a [NormalizedDeclaration],
    category: SubtypeCategory,
    name: &str,
) -> &'a NormalizedDeclaration {
    declarations
        .iter()
        .find(|declaration| {
            declaration.identity().kind() == DeclarationKind::Subtype(category)
                && declaration.identity().name() == name
        })
        .unwrap_or_else(|| panic!("missing {category} subtype {name}"))
}

fn surfaces(declaration: &NormalizedDeclaration) -> Vec<(SurfaceFeature, &str)> {
    declaration
        .grammar()
        .expect("subtype declarations must contribute noun grammar")
        .surfaces()
        .iter()
        .map(|surface| (surface.feature(), surface.text()))
        .collect()
}

fn attested_plural(category: SubtypeCategory, name: &str) -> Option<&'static str> {
    match (category, name) {
        (SubtypeCategory::Artifact, "Attraction") => Some("Attractions"),
        (SubtypeCategory::Artifact, "Bobblehead") => Some("Bobbleheads"),
        (SubtypeCategory::Artifact, "Clue") => Some("Clues"),
        (SubtypeCategory::Artifact, "Contraption") => Some("Contraptions"),
        (SubtypeCategory::Artifact, "Equipment") => Some("Equipment"),
        (SubtypeCategory::Artifact, "Food") => Some("Foods"),
        (SubtypeCategory::Artifact, "Fortification") => Some("Fortifications"),
        (SubtypeCategory::Artifact, "Spacecraft") => Some("Spacecraft"),
        (SubtypeCategory::Artifact, "Treasure") => Some("Treasures"),
        (SubtypeCategory::Artifact, "Vehicle") => Some("Vehicles"),
        (SubtypeCategory::Battle, "Siege") => Some("Sieges"),
        (SubtypeCategory::Enchantment, "Aura") => Some("Auras"),
        (SubtypeCategory::Enchantment, "Curse") => Some("Curses"),
        (SubtypeCategory::Enchantment, "Role") => Some("Roles"),
        (SubtypeCategory::Enchantment, "Room") => Some("Rooms"),
        (SubtypeCategory::Enchantment, "Saga") => Some("Sagas"),
        (SubtypeCategory::Enchantment, "Shard") => Some("Shards"),
        (SubtypeCategory::Enchantment, "Shrine") => Some("Shrines"),
        (SubtypeCategory::Land, "Cave") => Some("Caves"),
        (SubtypeCategory::Land, "Desert") => Some("Deserts"),
        (SubtypeCategory::Land, "Forest") => Some("Forests"),
        (SubtypeCategory::Land, "Gate") => Some("Gates"),
        (SubtypeCategory::Land, "Island") => Some("Islands"),
        (SubtypeCategory::Land, "Mountain") => Some("Mountains"),
        (SubtypeCategory::Land, "Plains") => Some("Plains"),
        (SubtypeCategory::Land, "Swamp") => Some("Swamps"),
        (SubtypeCategory::Land, "Town") => Some("Towns"),
        _ => None,
    }
}

fn catalog_stem(spelling: &str) -> String {
    let mut stem = String::new();
    let mut capitalize = true;
    for character in spelling.chars() {
        match character {
            ' ' | '-' => capitalize = true,
            '\'' | '!' => {}
            character => {
                if capitalize {
                    stem.extend(character.to_uppercase());
                    capitalize = false;
                } else {
                    stem.push(character);
                }
            }
        }
    }
    stem
}

fn compact_ron(source: &str) -> String {
    let mut compact = source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    loop {
        let normalized = compact.replace(",]", "]").replace(",)", ")");
        if normalized == compact {
            return compact;
        }
        compact = normalized;
    }
}

#[test]
fn builtin_v2_noncreature_subtypes_match_each_supported_catalog_and_category() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2"))
        .expect("builtin-v2 declarations must load through the production reader");

    let mut total = 0;
    for spec in CATEGORIES {
        let catalog_path = workspace_root.join("data/gen/catalogs").join(spec.catalog);
        let catalog = fs::read_to_string(&catalog_path)
            .unwrap_or_else(|error| panic!("reading {}: {error}", catalog_path.display()));
        let expected = catalog
            .lines()
            .map(|spelling| (catalog_stem(spelling), spelling.to_owned()))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(
            expected.len(),
            spec.expected_count,
            "canonical {} catalog count changed",
            spec.directory
        );

        let directory = workspace_root
            .join("plugins/builtin_v2/macros/stubs/subtypes")
            .join(spec.directory);
        let authored_files = fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("reading {}: {error}", directory.display()))
            .map(|entry| {
                let path = entry.expect("subtype entry must be readable").path();
                assert_eq!(
                    path.extension().and_then(|value| value.to_str()),
                    Some("ron"),
                    "non-RON file in {}",
                    directory.display()
                );
                path.file_stem()
                    .and_then(|value| value.to_str())
                    .expect("subtype filename must be UTF-8")
                    .to_owned()
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(
            authored_files,
            expected.keys().cloned().collect(),
            "catalog entries and committed {} subtype files must be bijective",
            spec.directory
        );

        let category_rows = declarations
            .iter()
            .filter(|declaration| {
                declaration.identity().kind() == DeclarationKind::Subtype(spec.category)
            })
            .collect::<Vec<_>>();
        assert_eq!(category_rows.len(), spec.expected_count);
        for declaration in &category_rows {
            let spelling = expected
                .get(declaration.identity().name())
                .unwrap_or_else(|| {
                    panic!(
                        "unexpected {} subtype {}",
                        spec.directory,
                        declaration.identity()
                    )
                });
            assert_eq!(
                declaration.spelling(),
                [SpellingPart::Literal(spelling.clone())]
            );
            assert_eq!(
                declaration
                    .grammar()
                    .map(deckmaste_construction_core::macro_def::GrammarRow::recipe),
                Some(&GrammarRecipe::Noun)
            );
            let expected_surfaces =
                match attested_plural(spec.category, declaration.identity().name()) {
                    Some(plural) => vec![
                        (SurfaceFeature::Singular, spelling.as_str()),
                        (SurfaceFeature::Plural, plural),
                    ],
                    None => vec![(SurfaceFeature::Singular, spelling.as_str())],
                };
            assert_eq!(surfaces(declaration), expected_surfaces);
        }
        total += category_rows.len();
    }
    assert_eq!(total, 138);

    let subtype_root = workspace_root.join("plugins/builtin_v2/macros/stubs/subtypes");
    assert!(
        !subtype_root.join("planar").exists(),
        "planar subtypes are a named model gap, not covered inventory"
    );
    assert!(
        !subtype_root.join("dungeon").exists(),
        "dungeon subtypes are a named model gap, not covered inventory"
    );
}

#[test]
fn noncreature_subtype_surfaces_preserve_attested_number_boundaries() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2")).unwrap();

    assert_eq!(
        surfaces(subtype(
            &declarations,
            SubtypeCategory::Artifact,
            "Equipment"
        )),
        [
            (SurfaceFeature::Singular, "Equipment"),
            (SurfaceFeature::Plural, "Equipment"),
        ]
    );
    for (name, plural) in [
        ("Food", "Foods"),
        ("Clue", "Clues"),
        ("Treasure", "Treasures"),
    ] {
        assert_eq!(
            surfaces(subtype(&declarations, SubtypeCategory::Artifact, name)),
            [
                (SurfaceFeature::Singular, name),
                (SurfaceFeature::Plural, plural),
            ]
        );
    }

    assert_eq!(
        surfaces(subtype(&declarations, SubtypeCategory::Land, "Locus")),
        [(SurfaceFeature::Singular, "Locus")]
    );
    assert_eq!(
        surfaces(subtype(&declarations, SubtypeCategory::Enchantment, "Saga")),
        [
            (SurfaceFeature::Singular, "Saga"),
            (SurfaceFeature::Plural, "Sagas"),
        ]
    );

    assert_eq!(
        surfaces(subtype(&declarations, SubtypeCategory::Land, "PowerPlant")),
        [(SurfaceFeature::Singular, "Power-Plant")]
    );
    assert_eq!(
        surfaces(subtype(&declarations, SubtypeCategory::Land, "Urzas")),
        [(SurfaceFeature::Singular, "Urza's")]
    );
    assert_eq!(
        surfaces(subtype(
            &declarations,
            SubtypeCategory::Planeswalker,
            "Jace"
        )),
        [(SurfaceFeature::Singular, "Jace")]
    );
}

#[test]
fn spell_subtypes_share_one_category_without_inventing_other_subtype_domains() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2")).unwrap();
    let spell_names = declarations
        .iter()
        .filter(|declaration| {
            declaration.identity().kind() == DeclarationKind::Subtype(SubtypeCategory::Spell)
        })
        .map(|declaration| declaration.identity().name())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        spell_names,
        BTreeSet::from(["Adventure", "Arcane", "Lesson", "Omen", "Trap"])
    );
    assert_eq!(
        surfaces(subtype(&declarations, SubtypeCategory::Spell, "Adventure")),
        [(SurfaceFeature::Singular, "Adventure")]
    );
    assert_eq!(
        surfaces(subtype(&declarations, SubtypeCategory::Spell, "Arcane")),
        [(SurfaceFeature::Singular, "Arcane")]
    );
}

#[test]
fn rules_defined_conferrals_stay_on_their_subtype_declarations() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins/builtin_v2")).unwrap();
    let expected = [
        (
            SubtypeCategory::Artifact,
            "Equipment",
            "Subtype(name:\"Equipment\",types:[Artifact],confers:[Static(May(Attach(what:Ref(This),to:Type(Creature))))])",
        ),
        (
            SubtypeCategory::Artifact,
            "Fortification",
            "Subtype(name:\"Fortification\",types:[Artifact],confers:[Static(May(Attach(what:Ref(This),to:Type(Land))))])",
        ),
        (
            SubtypeCategory::Enchantment,
            "Aura",
            "Subtype(name:\"Aura\",types:[Enchantment],confers:[StateBased(condition:Not(LegallyAttached(This)),effect:Move(This,Graveyard))])",
        ),
        (
            SubtypeCategory::Enchantment,
            "Saga",
            "Subtype(name:\"Saga\",types:[Enchantment],confers:[Ability(Static(Replacement(Also(would:ThisEnters,also:PutCounters(This,LoreCounter,1))))),TurnBased(at:PrecombatMain,effect:PutCounters(This,LoreCounter,1)),StateBased(condition:And([Compare(GreatestWatchedThreshold(This),AtLeast,1),Compare(CounterCount(This,LoreCounter),AtLeast,GreatestWatchedThreshold(This))]),effect:Sacrifice(You,This))])",
        ),
    ];

    for (category, name, expected_body) in expected {
        let declaration = subtype(&declarations, category, name);
        let body = declaration
            .body()
            .unwrap_or_else(|| panic!("{name} must retain its rules-defined conferrals"));
        assert_eq!(compact_ron(body.get_ron()), compact_ron(expected_body));
        assert_eq!(declaration.params(), Some([].as_slice()));
        assert!(declaration.is_graduated());
    }

    for declaration in declarations.iter().filter(|declaration| {
        matches!(
            declaration.identity().kind(),
            DeclarationKind::Subtype(category) if category != SubtypeCategory::Creature
        )
    }) {
        if !matches!(
            (declaration.identity().kind(), declaration.identity().name()),
            (
                DeclarationKind::Subtype(SubtypeCategory::Artifact),
                "Equipment" | "Fortification"
            ) | (
                DeclarationKind::Subtype(SubtypeCategory::Enchantment),
                "Aura" | "Saga"
            )
        ) {
            assert!(
                declaration.body().is_none(),
                "{} must not infer a semantic conferral",
                declaration.identity()
            );
            assert!(!declaration.is_graduated());
        }
    }
}

#[test]
fn noncreature_subtype_nursery_rejects_a_redundant_default_plural_override() {
    let error = read_str(
        "Food.ron",
        r#"Subtype(
            category: Artifact,
            name: "Food",
            spelling: "Food",
            grammar: Noun(singular: "Food", plural: "Foods"),
        )"#,
    )
    .unwrap_err();

    assert_eq!(
        error.validation(),
        Some(&ValidationError::RedundantOverride {
            field: "plural",
            surface: "Foods".to_owned(),
        })
    );
}
