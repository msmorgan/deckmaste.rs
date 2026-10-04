use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

fn word(owner: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn cards() -> Reading {
    let mut head = word(
        "lexeme:CommonNoun/Card",
        WordForm::Plural,
        FeatureBundle {
            number: Some(Number::Plural),
            ..Default::default()
        },
    );
    head.countability = Some(true);
    Reading::AccusativePhrase {
        form: 0,
        head: Box::new(Reading::BarePlural {
            form: 0,
            head: Box::new(Reading::Noun { form: 0, head }),
        }),
    }
}

fn prep(owner: &str) -> Reading {
    Reading::SelectedPrepositionHead {
        form: 0,
        head: word(owner, WordForm::Invariant, FeatureBundle::default()),
    }
}

fn coordinator(owner: &str) -> Word {
    word(owner, WordForm::Invariant, FeatureBundle::default())
}

fn shared(head: Reading) -> Reading {
    Reading::SharedPrepositionComplement {
        form: 0,
        head: Box::new(head),
        complement: Box::new(cards()),
    }
}

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &analyzed, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|reading| {
            let reading = reading.unwrap();
            assert_eq!(reading.realize(&LEXICON).unwrap(), text);
            reading
        })
        .collect()
}

fn roundtrip(value: &Reading, category: Category) {
    let text = value.realize(&LEXICON).unwrap();
    assert!(readings(&text, category).contains(value), "{text}");
}

#[test]
fn authentic_mutate_preposition_heads_share_np_complement() {
    let head = Reading::SelectedPrepositionHeadCoordination {
        form: 0,
        left: Box::new(prep("vocab:Preposition/Over")),
        coordinator: coordinator("vocab:Coordinator/Or"),
        right: Box::new(prep("vocab:Preposition/Under")),
    };
    roundtrip(&head, Category::SelectedPrepositionHead);
    roundtrip(&shared(head), Category::PrepositionPhrase);
    assert!(
        !readings(
            "over or under target creature you own",
            Category::PrepositionPhrase
        )
        .is_empty()
    );
}

#[test]
fn shared_preposition_heads_require_oxford_and_intersect_permissions() {
    let tail = Reading::SelectedPrepositionHeadSeriesEnd {
        form: 0,
        left: Box::new(prep("vocab:Preposition/Over")),
        coordinator: coordinator("vocab:Coordinator/Or"),
        right: Box::new(prep("vocab:Preposition/Under")),
    };
    let serial = Reading::SerialSelectedPrepositionHead {
        form: 0,
        left: Box::new(prep("vocab:Preposition/On")),
        rest: Box::new(tail),
    };
    roundtrip(&serial, Category::SelectedPrepositionHead);
    roundtrip(&shared(serial), Category::PrepositionPhrase);
    let mixed = shared(Reading::SelectedPrepositionHeadCoordination {
        form: 0,
        left: Box::new(prep("vocab:Preposition/Over")),
        coordinator: coordinator("vocab:Coordinator/And"),
        right: Box::new(prep("vocab:Preposition/Of")),
    });
    roundtrip(&mixed, Category::PrepositionPhrase);
    assert!(
        Reading::LocativeComplement {
            form: 0,
            phrase: Box::new(mixed)
        }
        .realize(&LEXICON)
        .is_err()
    );
    assert!(
        shared(prep("vocab:Preposition/Over"))
            .realize(&LEXICON)
            .is_err()
    );
    for text in [
        "over, on or under",
        "over, or under",
        "over nor under",
        "over and draw",
    ] {
        assert!(
            readings(text, Category::SelectedPrepositionHead).is_empty(),
            "{text}"
        );
    }
    assert!(readings("over or under they", Category::PrepositionPhrase).is_empty());
}
