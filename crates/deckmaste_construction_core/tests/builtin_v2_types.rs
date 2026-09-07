use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::NormalizedDeclaration;
use deckmaste_construction_core::macro_def::SpellingPart;
use deckmaste_construction_core::macro_def::SurfaceFeature;
use deckmaste_construction_core::macro_def::ValidationError;
use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_construction_core::macro_def::read_str;

struct ExpectedType {
    name: &'static str,
    singular: &'static str,
    plural: Option<&'static str>,
    permanent_type: bool,
    body: &'static str,
}

const EXPECTED_TYPES: [ExpectedType; 10] = [
    ExpectedType {
        name: "Artifact",
        singular: "artifact",
        plural: Some("artifacts"),
        permanent_type: true,
        body: r#"TypeDef(name:"Artifact",permanent_type:true)"#,
    },
    ExpectedType {
        name: "Battle",
        singular: "battle",
        plural: Some("battles"),
        permanent_type: true,
        body: r#"TypeDef(name:"Battle",permanent_type:true)"#,
    },
    ExpectedType {
        name: "Creature",
        singular: "creature",
        plural: Some("creatures"),
        permanent_type: true,
        body: r#"TypeDef(
            name:"Creature",
            permanent_type:true,
            confers:[
                Static(Conditionally(
                    Matches(This,Permanent),
                    Role(who:Ref(This),role:Combatant)
                ))
            ]
        )"#,
    },
    ExpectedType {
        name: "Dungeon",
        singular: "dungeon",
        plural: Some("dungeons"),
        permanent_type: false,
        body: r#"TypeDef(name:"Dungeon",permanent_type:false)"#,
    },
    ExpectedType {
        name: "Enchantment",
        singular: "enchantment",
        plural: Some("enchantments"),
        permanent_type: true,
        body: r#"TypeDef(name:"Enchantment",permanent_type:true)"#,
    },
    ExpectedType {
        name: "Instant",
        singular: "instant",
        plural: Some("instants"),
        permanent_type: false,
        body: r#"TypeDef(
            name:"Instant",
            permanent_type:false,
            confers:[Static(May(Cast(what:Ref(This),window:InstantSpeed)))]
        )"#,
    },
    ExpectedType {
        name: "Kindred",
        singular: "kindred",
        plural: None,
        permanent_type: false,
        body: r#"TypeDef(name:"Kindred",permanent_type:false)"#,
    },
    ExpectedType {
        name: "Land",
        singular: "land",
        plural: Some("lands"),
        permanent_type: true,
        body: r#"TypeDef(
            name:"Land",
            permanent_type:true,
            confers:[Static(May(Play(what:Ref(This))))]
        )"#,
    },
    ExpectedType {
        name: "Planeswalker",
        singular: "planeswalker",
        plural: Some("planeswalkers"),
        permanent_type: true,
        body: r#"TypeDef(name:"Planeswalker",permanent_type:true)"#,
    },
    ExpectedType {
        name: "Sorcery",
        singular: "sorcery",
        plural: Some("sorceries"),
        permanent_type: false,
        body: r#"TypeDef(name:"Sorcery",permanent_type:false)"#,
    },
];

const CR_TYPES_OUTSIDE_MODELED_TYPE_LINE: [&str; 5] =
    ["Conspiracy", "Phenomenon", "Plane", "Scheme", "Vanguard"];

fn compact_ron(source: &str) -> String {
    let source = ron::value::RawValue::from_ron(source)
        .expect("expected semantic body must be valid RON")
        .trim()
        .get_ron();
    let mut compact = String::with_capacity(source.len());
    let mut quote = None;
    let mut escaped = false;

    for character in source.chars() {
        if let Some(delimiter) = quote {
            compact.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == delimiter {
                quote = None;
            }
            continue;
        }

        match character {
            '"' | '\'' => {
                quote = Some(character);
                compact.push(character);
            }
            ')' | ']' => {
                if compact.ends_with(',') {
                    compact.pop();
                }
                compact.push(character);
            }
            character if character.is_whitespace() => {}
            character => compact.push(character),
        }
    }

    compact
}

fn type_rows(declarations: &[NormalizedDeclaration]) -> BTreeMap<&str, &NormalizedDeclaration> {
    declarations
        .iter()
        .filter(|declaration| declaration.identity().kind() == DeclarationKind::Type)
        .map(|declaration| (declaration.identity().name(), declaration))
        .collect()
}

#[test]
fn semantic_body_comparison_preserves_whitespace_inside_strings() {
    assert_ne!(
        compact_ron(r#"TypeDef(name: "Arti fact", permanent_type: true)"#),
        compact_ron(r#"TypeDef(name: "Artifact", permanent_type: true)"#),
    );
}

#[test]
fn builtin_v2_types_load_with_exact_semantics_and_noun_surfaces() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let declarations = read_builtin_v2(workspace_root.join("plugins_v2/builtin"))
        .expect("builtin-v2 type declarations must load through the production reader");
    let types = type_rows(&declarations);

    assert_eq!(
        types.keys().copied().collect::<BTreeSet<_>>(),
        EXPECTED_TYPES
            .iter()
            .map(|expected| expected.name)
            .collect(),
        "the modeled type registry is deliberately smaller than the CR card-type catalog"
    );

    for expected in &EXPECTED_TYPES {
        let declaration = types
            .get(expected.name)
            .unwrap_or_else(|| panic!("missing modeled type {}", expected.name));
        assert_eq!(
            declaration.spelling(),
            [SpellingPart::Literal(expected.singular.to_owned())],
            "{} spelling",
            expected.name
        );
        let grammar = declaration
            .grammar()
            .unwrap_or_else(|| panic!("{} must contribute noun grammar", expected.name));
        assert_eq!(grammar.recipe(), &GrammarRecipe::Noun);
        let mut expected_surfaces = vec![(SurfaceFeature::Singular, expected.singular)];
        if let Some(plural) = expected.plural {
            expected_surfaces.push((SurfaceFeature::Plural, plural));
        }
        assert_eq!(
            grammar
                .surfaces()
                .iter()
                .map(|surface| (surface.feature(), surface.text()))
                .collect::<Vec<_>>(),
            expected_surfaces,
            "{} nominal/plural surfaces; the noun recipe also licenses attributive use",
            expected.name
        );
        assert_eq!(declaration.params(), Some([].as_slice()));
        assert!(declaration.is_graduated());
        assert_eq!(
            compact_ron(
                declaration
                    .body()
                    .unwrap_or_else(|| panic!("{} must retain its TypeDef body", expected.name))
                    .get_ron()
            ),
            compact_ron(expected.body),
            "{} permanent_type={} and conferrals",
            expected.name,
            expected.permanent_type
        );
        assert_eq!(
            declaration.provenance().path(),
            workspace_root
                .join("plugins_v2/builtin/macros/types")
                .join(format!("{}.ron", expected.name))
        );
    }

    for outside in CR_TYPES_OUTSIDE_MODELED_TYPE_LINE {
        assert!(
            !types.contains_key(outside),
            "{outside} is a CR card type, but not a modeled semantic type-line member"
        );
    }
}

#[test]
fn an_open_type_noun_normalizes_without_closed_membership() {
    let declaration = read_str(
        "Chronicle.ron",
        r#"Type(
            name: "Chronicle",
            spelling: "chronicle",
            grammar: Noun(singular: "chronicle", plural: Unavailable),
        )"#,
    )
    .unwrap();

    assert_eq!(declaration.identity().kind(), DeclarationKind::Type);
    assert_eq!(declaration.identity().name(), "Chronicle");
    assert_eq!(
        declaration.grammar().unwrap().recipe(),
        &GrammarRecipe::Noun
    );
    assert_eq!(
        declaration
            .grammar()
            .unwrap()
            .surfaces()
            .iter()
            .map(|surface| (surface.feature(), surface.text()))
            .collect::<Vec<_>>(),
        [(SurfaceFeature::Singular, "chronicle")]
    );
    assert!(!declaration.is_graduated());
}

#[test]
fn type_declarations_reject_legacy_template_fields_by_name() {
    let error = read_str(
        "Chronicle.ron",
        r#"Type(
            name: "Chronicle",
            spelling: "chronicle",
            template: "chronicle",
            grammar: Noun(singular: "chronicle", plural: Unavailable),
        )"#,
    )
    .unwrap_err();

    let message = error.to_string();
    assert!(
        message.contains("`template`")
            && (message.contains("unknown field")
                || message.contains("not one of this macro's parameters")),
        "legacy field must be named in the diagnostic: {message}"
    );
}

#[test]
fn type_nouns_reject_redundant_default_plural_overrides() {
    let error = read_str(
        "Artifact.ron",
        r#"Type(
            name: "Artifact",
            spelling: "artifact",
            grammar: Noun(singular: "artifact", plural: "artifacts"),
        )"#,
    )
    .unwrap_err();

    assert_eq!(
        error.validation(),
        Some(&ValidationError::RedundantOverride {
            field: "plural",
            surface: "artifacts".to_owned(),
        })
    );
}
