use std::collections::BTreeMap;
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

/// A body's text with every space taken out, so a comparison is about the
/// value written and not about how the expander spaced it: a derived body is
/// assembled by substitution, and its inter-field spacing is `macro_ron`'s
/// business rather than the declaration's.
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

fn creature_type<'a>(
    declarations: &'a [NormalizedDeclaration],
    name: &str,
) -> &'a NormalizedDeclaration {
    declarations
        .iter()
        .find(|declaration| {
            declaration.identity().kind() == DeclarationKind::Subtype(SubtypeCategory::Creature)
                && declaration.identity().name() == name
        })
        .unwrap_or_else(|| panic!("missing creature type {name}"))
}

fn surfaces(declaration: &NormalizedDeclaration) -> Vec<(SurfaceFeature, &str)> {
    declaration
        .grammar()
        .expect("creature types must contribute noun grammar")
        .surfaces()
        .iter()
        .map(|surface| (surface.feature(), surface.text()))
        .collect()
}

fn catalog_stem(spelling: &str) -> String {
    let mut stem = String::new();
    let mut capitalize = true;
    for character in spelling.chars() {
        match character {
            ' ' | '-' => capitalize = true,
            '\'' => {}
            character => {
                if capitalize {
                    if stem.is_empty() {
                        stem.extend(character.to_lowercase());
                    } else {
                        stem.extend(character.to_uppercase());
                    }
                    capitalize = false;
                } else {
                    stem.push(character);
                }
            }
        }
    }
    stem
}

#[test]
fn builtin_v2_creature_type_nursery_matches_catalog_and_attested_morphology() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let catalog_path = workspace_root.join("data/gen/catalogs/creature-types.txt");
    let catalog =
        fs::read_to_string(&catalog_path).expect("creature-type catalog must be readable");
    let expected = catalog
        .lines()
        .map(|spelling| (catalog_stem(spelling), spelling.to_owned()))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(expected.len(), 324, "canonical creature-type count changed");

    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 declarations must load");
    let creature_types = declarations
        .iter()
        .filter(|declaration| {
            declaration.identity().kind() == DeclarationKind::Subtype(SubtypeCategory::Creature)
        })
        .collect::<Vec<_>>();
    assert_eq!(creature_types.len(), 324);

    for declaration in creature_types {
        let spelling = expected
            .get(declaration.identity().name())
            .unwrap_or_else(|| panic!("unexpected creature type {}", declaration.identity()));
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
        // Since `semantics-v2-definition-bodies` a subtype declaration carries
        // its `Definition` node as its body. The node is the declaration's own
        // identity and nothing else — no creature type infers a semantic
        // conferral from its name.
        let body = declaration
            .body()
            .unwrap_or_else(|| panic!("{} must define its subtype", declaration.identity()));
        assert_eq!(
            compact_ron(body.get_ron()),
            compact_ron(&format!(
                "Subtype(subtype: Of(host: Creature, label: \"{spelling}\"), rules: [])"
            )),
            "{} must not infer a semantic conferral",
            declaration.identity()
        );
    }

    assert_eq!(
        surfaces(creature_type(&declarations, "goblin")),
        [
            (SurfaceFeature::Singular, "Goblin"),
            (SurfaceFeature::Plural, "Goblins"),
        ]
    );
    assert_eq!(
        surfaces(creature_type(&declarations, "elf")),
        [
            (SurfaceFeature::Singular, "Elf"),
            (SurfaceFeature::Plural, "Elves"),
        ]
    );
    assert_eq!(
        surfaces(creature_type(&declarations, "mouse")),
        [
            (SurfaceFeature::Singular, "Mouse"),
            (SurfaceFeature::Plural, "Mice"),
        ]
    );
    assert_eq!(
        surfaces(creature_type(&declarations, "merfolk")),
        [
            (SurfaceFeature::Singular, "Merfolk"),
            (SurfaceFeature::Plural, "Merfolk"),
        ]
    );
    assert_eq!(
        surfaces(creature_type(&declarations, "timeLord")),
        [
            (SurfaceFeature::Singular, "Time Lord"),
            (SurfaceFeature::Plural, "Time Lords"),
        ]
    );
    assert_eq!(
        surfaces(creature_type(&declarations, "ctan")),
        [(SurfaceFeature::Singular, "C'tan")]
    );
    assert_eq!(
        surfaces(creature_type(&declarations, "child")),
        [(SurfaceFeature::Singular, "Child")]
    );
    assert_eq!(
        surfaces(creature_type(&declarations, "leech")),
        [(SurfaceFeature::Singular, "Leech")]
    );
}

#[test]
fn creature_type_nursery_rejects_a_redundant_default_plural_override() {
    let error = read_str(
        "Goblin.ron",
        r#"Subtype(
            category: Creature,
            name: "Goblin",
            spelling: "Goblin",
            grammar: Noun(singular: "Goblin", plural: "Goblins"),
        )"#,
    )
    .unwrap_err();

    assert_eq!(
        error.validation(),
        Some(&ValidationError::RedundantOverride {
            field: "plural",
            surface: "Goblins".to_owned(),
        })
    );
}
