use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::Onset;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceCase;

fn row(id: &str, spelling: &str) -> Lexeme {
    Lexeme::invariant(
        id,
        spelling,
        Category::Adjective,
        Source {
            kind: SourceKind::Core,
            path: "pronunciation-fixture".into(),
            owner: id.into(),
        },
    )
}

#[test]
fn normalized_pronunciation_respects_overrides_spelling_variants_and_numerals() {
    let mut alternative = row("alternative", "heir");
    alternative.forms[0].surfaces = Some(vec!["heir".into(), "unicorn".into()]);
    let mut overridden = row("override", "xenic");
    overridden.onsets.insert("xenic".into(), Onset::Vowel);
    let lexicon = Lexicon::new([alternative, overridden, row("default", "xenic")]).unwrap();
    for value in lexicon.values() {
        let reading = LexicalReading::Word(value.clone());
        let surface = lexicon.realize(&reading).unwrap();
        let properties = lexicon.surface_features(&reading).unwrap();
        let expected =
            if value.lexeme == "override" || value.lexeme == "alternative" && value.variant == 0 {
                Onset::Vowel
            } else {
                Onset::Consonant
            };
        assert_eq!(properties.onset, Some(expected), "{value:?}");
        assert_eq!(
            properties.initial,
            value.capitalization == SurfaceCase::Initial
        );
        assert_eq!(
            properties.interior,
            value.capitalization == SurfaceCase::Declared
        );
        assert!(
            lexicon
                .analyze(&surface)
                .matches
                .iter()
                .any(|m| m.reading == reading)
        );
    }
    for (value, expected) in [
        (1, Onset::Consonant),
        (8, Onset::Vowel),
        (11, Onset::Vowel),
        (18, Onset::Vowel),
        (80, Onset::Vowel),
        (180, Onset::Consonant),
        (800, Onset::Vowel),
        (-8, Onset::Consonant),
    ] {
        let reading = LexicalReading::Numeral {
            value,
            notation: Numeral::Arabic(false),
            capitalization: SurfaceCase::Declared,
        };
        let properties = lexicon.surface_features(&reading).unwrap();
        assert_eq!(properties.onset, Some(expected));
        assert!(properties.initial && properties.interior);
    }
}

#[test]
fn invalid_pronunciation_declarations_fail_loading() {
    let mut missing = row("missing", "uniform");
    missing.onsets.insert("unicorn".into(), Onset::Vowel);
    assert!(
        Lexicon::new([missing])
            .unwrap_err()
            .to_string()
            .contains("no declared spelling")
    );
    let mut not_an_article = row("adjective", "uniform");
    not_an_article
        .article_onsets
        .insert("uniform".into(), Onset::Vowel);
    assert!(
        Lexicon::new([not_an_article])
            .unwrap_err()
            .to_string()
            .contains("determinative")
    );
    let mut incomplete = row("article", "a");
    incomplete.category = Category::Determinative;
    incomplete.forms[0].surfaces = Some(vec!["a".into(), "an".into()]);
    incomplete
        .article_onsets
        .insert("a".into(), Onset::Consonant);
    assert!(
        Lexicon::new([incomplete])
            .unwrap_err()
            .to_string()
            .contains("every determinative spelling")
    );
}

#[test]
fn multiword_names_and_bound_forms_take_the_first_pronounced_onset() {
    for (spelling, expected) in [
        ("Eight-and-a-Half-Tails", Onset::Vowel),
        ("One with Nothing", Onset::Consonant),
        ("Urza's Saga", Onset::Vowel),
        ("island", Onset::Vowel),
        ("forest", Onset::Consonant),
    ] {
        let mut entry = row("onset", spelling);
        entry.category = Category::Catalog;
        entry.surface_structure = deckmaste_lexical::SurfaceStructure::Opaque;
        if matches!(spelling, "island" | "forest") {
            entry.binding = deckmaste_lexical::Binding::Prefix;
        }
        let lexicon = Lexicon::new([entry]).unwrap();
        for value in lexicon.values() {
            let reading = LexicalReading::Word(value.clone());
            assert_eq!(
                lexicon.surface_features(&reading).unwrap().onset,
                Some(expected)
            );
        }
    }
}
