use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Onset;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;
use deckmaste_lexical::{self as lexical};

fn source(id: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "article-fixture".into(),
        owner: id.into(),
    }
}

fn lexicon() -> Lexicon {
    let mut article = Lexeme::invariant(
        "article",
        "a",
        lexical::Category::Determinative,
        source("article"),
    );
    article.forms[0].surfaces = Some(vec!["a".into(), "an".into()]);
    article.forms[0].features.number = Some(Number::Singular);
    article.article_onsets = [("a".into(), Onset::Consonant), ("an".into(), Onset::Vowel)].into();
    article
        .properties
        .features
        .insert("DeterminerKind".into(), "Indefinite".into());
    let mut rows = vec![article];
    for (id, use_) in [
        ("creature", Countability::Count),
        ("artifact", Countability::Count),
        ("mana", Countability::Mass),
    ] {
        rows.push(Lexeme::noun(id, id, vec![use_], source(id)));
    }
    for id in ["unusual", "uniform", "hourlong", "one-eyed", "xenic"] {
        let mut adjective = Lexeme::invariant(id, id, lexical::Category::Adjective, source(id));
        if id == "xenic" {
            adjective.onsets.insert(id.into(), Onset::Vowel);
        }
        rows.push(adjective);
    }
    Lexicon::new(rows).unwrap()
}

fn word(id: &str, form: WordForm, variant: usize, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            variant,
            capitalization,
            features: lexical::FeatureBundle {
                number: (form == WordForm::Singular || id == "article").then_some(Number::Singular),
                ..Default::default()
            },
        }),
        frame: None,
        countability: (form == WordForm::Singular).then_some(id != "mana"),
    }
}

fn parse_all(lexicon: &Lexicon, text: &str, root: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let input = lexicon.analyze(text);
    let forest = parse(&grammar, lexicon, &input, &root).unwrap();
    grammar
        .readings(&forest)
        .map(|r| {
            let r = r.unwrap();
            assert_eq!(r.realize(lexicon).unwrap(), text);
            r
        })
        .collect()
}

#[test]
fn independent_article_values_preserve_onset_variants_and_modifier_attachment() {
    let lexicon = lexicon();
    for (noun, modifier, vowel) in [
        ("creature", None, false),
        ("artifact", None, true),
        ("creature", Some("unusual"), true),
        ("artifact", Some("uniform"), false),
        ("creature", Some("hourlong"), true),
        ("artifact", Some("one-eyed"), false),
        ("artifact", Some("xenic"), true),
    ] {
        for casing in [SurfaceCase::Declared, SurfaceCase::Initial] {
            let nominal = Reading::Noun {
                form: 0,
                head: word(noun, WordForm::Singular, 0, SurfaceCase::Declared),
            };
            let nominal = if let Some(modifier) = modifier {
                Reading::PremodifiedNominal {
                    form: 0,
                    head: Box::new(nominal),
                    modifier: Box::new(Reading::Adjective {
                        form: 0,
                        head: word(modifier, WordForm::Invariant, 0, SurfaceCase::Declared),
                    }),
                }
            } else {
                nominal
            };
            let expected = Reading::IndefiniteNounPhrase {
                form: 0,
                determiner: word("article", WordForm::Invariant, usize::from(vowel), casing),
                head: Box::new(nominal),
            };
            let article = match (vowel, casing) {
                (false, SurfaceCase::Declared) => "a",
                (true, SurfaceCase::Declared) => "an",
                (false, SurfaceCase::Initial) => "A",
                (true, SurfaceCase::Initial) => "An",
            };
            let text = format!(
                "{article} {}{noun}",
                modifier.map_or(String::new(), |m| format!("{m} "))
            );
            assert_eq!(expected.realize(&lexicon).unwrap(), text);
            assert_eq!(
                parse_all(&lexicon, &text, Category::NounPhrase),
                [expected.clone()].into()
            );
            let mut leaves = vec![];
            expected
                .visit_words(&mut |w| leaves.push(w.clone()))
                .unwrap();
            let mut expected_leaves = vec![word(
                "article",
                WordForm::Invariant,
                usize::from(vowel),
                casing,
            )];
            if let Some(modifier) = modifier {
                expected_leaves.push(word(
                    modifier,
                    WordForm::Invariant,
                    0,
                    SurfaceCase::Declared,
                ));
            }
            expected_leaves.push(word(noun, WordForm::Singular, 0, SurfaceCase::Declared));
            assert_eq!(leaves, expected_leaves);
            let mut wrong = expected;
            let Reading::IndefiniteNounPhrase { determiner, .. } = &mut wrong else {
                unreachable!()
            };
            let LexicalReading::Word(value) = &mut determiner.value else { unreachable!() };
            value.variant = usize::from(!vowel);
            assert!(wrong.admit(&lexicon).is_err());
        }
    }
}

#[test]
fn articles_reject_wrong_onset_plural_and_mass_uses() {
    let lexicon = lexicon();
    for text in [
        "an creature",
        "a artifact",
        "a unusual creature",
        "an uniform artifact",
        "a hourlong creature",
        "an one-eyed artifact",
        "a xenic artifact",
        "a creatures",
        "an artifacts",
        "a mana",
        "an mana",
        "a",
        "an",
    ] {
        assert!(
            parse_all(&lexicon, text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn homographs_keep_pronunciation_correlated_with_their_article_variant() {
    let mut rows: Vec<_> = lexicon().lexemes().values().cloned().collect();
    rows.push(Lexeme::invariant(
        "other-pronunciation",
        "xenic",
        lexical::Category::Adjective,
        source("other-pronunciation"),
    ));
    let lexicon = Lexicon::new(rows).unwrap();
    for (text, modifier, variant) in [
        ("an xenic artifact", "xenic", 1),
        ("a xenic artifact", "other-pronunciation", 0),
    ] {
        let expected = Reading::IndefiniteNounPhrase {
            form: 0,
            determiner: word(
                "article",
                WordForm::Invariant,
                variant,
                SurfaceCase::Declared,
            ),
            head: Box::new(Reading::PremodifiedNominal {
                form: 0,
                modifier: Box::new(Reading::Adjective {
                    form: 0,
                    head: word(modifier, WordForm::Invariant, 0, SurfaceCase::Declared),
                }),
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word("artifact", WordForm::Singular, 0, SurfaceCase::Declared),
                }),
            }),
        };
        assert_eq!(expected.realize(&lexicon).unwrap(), text);
        assert_eq!(
            parse_all(&lexicon, text, Category::NounPhrase),
            [expected].into()
        );
    }
}

fn workspace_lexicon() -> Lexicon {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(lexical_source(&root)).unwrap()
}

fn lexical_source(root: &std::path::Path) -> Vec<Lexeme> {
    deckmaste_lexical_source::load_workspace(root)
        .unwrap()
        .lexemes
}

#[test]
fn articles_and_case_reach_real_grammatical_hosts() {
    let lexicon = workspace_lexicon();
    let mut failures = vec![];
    for (text, root) in [
        ("Sacrifice an artifact", Category::CostComponent),
        ("an artifact", Category::NounPhrase),
        ("Draw a card.", Category::Document),
        ("Draw the card.", Category::Document),
        ("an amount of {G}", Category::NounPhrase),
        ("an 8/8 creature", Category::NounPhrase),
        ("a 1/1 creature", Category::NounPhrase),
        ("a +8/+8 creature", Category::NounPhrase),
        ("At the beginning", Category::PrepositionPhrase),
        ("in a graveyard", Category::PrepositionPhrase),
        ("a creature", Category::NounPhrase),
        ("Two target creatures", Category::NounPhrase),
        ("an Aura", Category::NounPhrase),
        ("Three Dog, Galaxy News DJ", Category::Name),
        ("Flying, trample", Category::Ability),
        ("Creatures have flying.", Category::Document),
        ("\"this creature can't be blocked\"", Category::QuotedText),
        ("\"Draw a card.\"", Category::QuotedText),
        ("\"flying.\"", Category::QuotedText),
        ("Creatures have \"flying.\".", Category::Document),
    ] {
        if parse_all(&lexicon, text, root).is_empty() {
            failures.push(format!("{text:?} at {root:?}"));
        }
    }
    for (text, root) in [
        ("draw the card.", Category::Document),
        ("Draw A card.", Category::Document),
        ("a 8/8 creature", Category::NounPhrase),
        ("an 1/1 creature", Category::NounPhrase),
        ("an +8/+8 creature", Category::NounPhrase),
        ("Draw a Card.", Category::Document),
        ("sacrifice an artifact", Category::CostComponent),
        ("Sacrifice an Artifact", Category::CostComponent),
        ("Flying, Trample", Category::Ability),
        ("flying, trample", Category::Ability),
        ("Creatures have Flying.", Category::Document),
        ("\"This creature can't be blocked\"", Category::QuotedText),
        ("\"draw a card.\"", Category::QuotedText),
        ("three Dog, Galaxy News DJ", Category::Name),
    ] {
        if !parse_all(&lexicon, text, root).is_empty() {
            failures.push(format!("unexpected {text:?} at {root:?}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn independently_supplied_names_preserve_exact_identity_and_case() {
    for (id, text) in [
        ("token-identity", "Prisoner Zero"),
        ("self-identity", "Three Dog"),
    ] {
        let mut row = Lexeme::invariant(id, text, lexical::Category::Catalog, source(id));
        row.properties
            .features
            .insert("IdentityUse".into(), "Name".into());
        let lexicon = Lexicon::new([row]).unwrap();
        let head = word(id, WordForm::Invariant, 0, SurfaceCase::Declared);
        let expected = Reading::CatalogName {
            form: 0,
            head: head.clone(),
        };
        assert_eq!(expected.realize(&lexicon).unwrap(), text);
        assert_eq!(
            parse_all(&lexicon, text, Category::Name),
            [expected.clone()].into()
        );
        let mut leaves = vec![];
        expected
            .visit_words(&mut |w| leaves.push(w.clone()))
            .unwrap();
        assert_eq!(leaves, [head]);
        assert!(parse_all(&lexicon, &text.to_lowercase(), Category::Name).is_empty());
    }
}
