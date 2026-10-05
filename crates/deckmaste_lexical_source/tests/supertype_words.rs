use std::collections::BTreeSet;
use std::path::Path;

use deckmaste_lexical::{
    Category, FeatureBundle, LexicalReading, LexicalValue, Lexicon, SurfaceCase, WordForm,
};

#[test]
fn supertype_adjectives_have_owned_positive_and_negative_values_without_duplicate_legendary() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let lexicon = Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap();
    // Attested modifiers: Cast Down (nonlegendary creature), Blood Moon
    // (Nonbasic lands), Into the North (snow land card). The declared class also includes world.
    for (owner, spelling) in [
        ("vocab:PredicativeAdjective/Legendary", "legendary"),
        ("vocab:Supertype/Basic", "basic"),
        ("vocab:Supertype/Snow", "snow"),
        ("vocab:Supertype/World", "world"),
    ] {
        for (id, surface) in [
            (owner.to_owned(), spelling.to_owned()),
            (format!("{owner}/non"), format!("non{spelling}")),
        ] {
            let entry = &lexicon.lexemes()[id.as_str()];
            assert_eq!(entry.category, Category::Adjective);
            assert_eq!(entry.source.owner, owner);
            for capitalization in [SurfaceCase::Declared, SurfaceCase::Initial] {
                let expected = LexicalReading::Word(LexicalValue {
                    lexeme: id.as_str().into(),
                    form: WordForm::Invariant,
                    features: FeatureBundle::default(),
                    variant: 0,
                    capitalization,
                });
                let text = if capitalization == SurfaceCase::Initial {
                    let mut chars = surface.chars();
                    format!("{}{}", chars.next().unwrap().to_uppercase(), chars.as_str())
                } else {
                    surface.clone()
                };
                assert_eq!(lexicon.realize(&expected).unwrap(), text);
                let readings: BTreeSet<_> = lexicon
                    .analyze(&text)
                    .matches
                    .into_iter()
                    .filter(|matched| matched.start == 0 && matched.end == text.chars().count())
                    .filter_map(|matched| match &matched.reading {
                        LexicalReading::Word(value) if value.lexeme == id.as_str() => {
                            Some(matched.reading)
                        }
                        _ => None,
                    })
                    .collect();
                assert_eq!(readings, BTreeSet::from([expected]));
            }
        }
    }
    let adjective_owners: BTreeSet<_> = lexicon
        .analyze("legendary")
        .matches
        .into_iter()
        .filter_map(|matched| match matched.reading {
            LexicalReading::Word(value)
                if lexicon.lexemes()[&value.lexeme].category == Category::Adjective =>
            {
                Some(value.lexeme)
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        adjective_owners,
        BTreeSet::from(["vocab:PredicativeAdjective/Legendary".into()])
    );
    assert_eq!(
        lexicon.lexemes()["vocab:Supertype/Legendary"].category,
        Category::Catalog
    );
}
