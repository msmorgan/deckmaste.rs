use std::path::Path;

use deckmaste_construction_core::macro_def::{
    DeclarationKind, NegativePrefixJoin, SurfaceFeature, TypeWordGrammar, ValidationError,
    read_builtin_v2, read_str,
};

#[test]
fn authored_type_classes_expand_lexical_recipes_without_changing_nouns() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin");
    let declarations = read_builtin_v2(root).unwrap();
    let mut type_count = 0;
    let mut subtype_count = 0;
    for declaration in &declarations {
        let join = match declaration.identity().kind() {
            DeclarationKind::Type => {
                type_count += 1;
                NegativePrefixJoin::Joined
            }
            DeclarationKind::Subtype(_) => {
                subtype_count += 1;
                NegativePrefixJoin::Hyphenated
            }
            _ => {
                assert_eq!(declaration.type_word(), None);
                continue;
            }
        };
        assert_eq!(
            declaration.type_word(),
            Some(TypeWordGrammar {
                noun_modifier: true,
                negative_prefix_join: join
            })
        );
    }
    assert!(type_count > 0);
    assert!(subtype_count > 0);
    // Existing authored irregular plurals remain grammatical noun forms.
    let sorcery = declarations
        .iter()
        .find(|row| {
            row.identity().kind() == DeclarationKind::Type && row.identity().name() == "sorcery"
        })
        .unwrap();
    let surfaces = sorcery.grammar().unwrap().surfaces();
    assert!(surfaces.iter().any(
        |surface| surface.feature() == SurfaceFeature::Plural && surface.text() == "sorceries"
    ));
    let merfolk = declarations
        .iter()
        .find(|row| row.identity().name() == "merfolk")
        .unwrap();
    assert!(
        merfolk
            .grammar()
            .unwrap()
            .surfaces()
            .iter()
            .any(|surface| surface.feature() == SurfaceFeature::Plural
                && surface.text() == "Merfolk")
    );
}

#[test]
fn type_word_recipes_preserve_elidable_grammar_but_reject_present_non_nouns() {
    let declaration = read_str(
        "artifact.ron",
        r#"Type(name: "artifact", spelling: "artifact")"#,
    )
    .unwrap();
    assert!(declaration.grammar().is_none());
    assert_eq!(
        declaration.type_word(),
        Some(TypeWordGrammar {
            noun_modifier: true,
            negative_prefix_join: NegativePrefixJoin::Joined
        })
    );
    let error = read_str(
        "artifact.ron",
        r#"Type(name: "artifact", spelling: "artifact", grammar: FixedTerm(surface: "artifact"))"#,
    )
    .unwrap_err();
    assert_eq!(
        error.validation(),
        Some(&ValidationError::TypeWordGrammarMismatch)
    );
}
