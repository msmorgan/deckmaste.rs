use std::collections::BTreeSet;
use std::path::Path;
use std::sync::OnceLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Case;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

fn lexicon() -> &'static Lexicon {
    static LEXICON: OnceLock<Lexicon> = OnceLock::new();
    LEXICON.get_or_init(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let source = deckmaste_lexical_source::load_workspace(&root).unwrap();
        Lexicon::new(source.lexemes).unwrap()
    })
}

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let lexicon = lexicon();
    let input = lexicon.analyze(text);
    let forest = parse(&grammar, lexicon, &input, &category).unwrap();
    let mut values = BTreeSet::new();
    for value in grammar.readings(&forest) {
        let value = value.unwrap();
        assert_eq!(value.realize(lexicon).unwrap(), text);
        assert!(values.insert(value), "duplicate Reading for {text:?}");
    }
    values
}

#[test]
fn authored_vocabulary_composes_across_the_grammar() {
    for text in [
        "Draw cards.",
        "You draw cards.",
        "Each creature attacks.",
        "These creatures attack.",
        "Draw cards from them.",
        "If you draw cards, draw cards.",
        "Draw cards. If you do, draw cards.",
        "If you do, draw cards.",
        "Creatures that attack draw cards.",
        "Creatures you control attack.",
        "Creatures that you control attack.",
        "Creatures you draw and discard attack.",
        "You and they draw cards.",
        "You may draw cards.",
        "Cards are drawn.",
        "Cards have been drawn.",
        "Target creature attacks.",
        "Target white creatures attack.",
    ] {
        assert!(
            !readings(text, Category::Document).is_empty(),
            "no Reading for {text:?}"
        );
    }
}

#[test]
fn agreement_case_frames_and_local_ellipsis_are_admission_constraints() {
    for text in [
        "Each creatures attack.",
        "These creature attacks.",
        "Creatures attacks.",
        "You draws cards.",
        "Them draw cards.",
        "Draw they.",
        "Draw cards from they.",
        "Creatures that attacks draw cards.",
        "Creatures you controls attack.",
        "Draw.",
        "You may draws cards.",
        "White target creatures attack.",
        ".",
        "If you do, .",
        "Draw cards and .",
    ] {
        assert!(
            readings(text, Category::Document).is_empty(),
            "invalid Reading for {text:?}"
        );
    }
}

fn word(id: &str, form: WordForm, features: FeatureBundle) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        countability: None,
        frame: None,
    }
}

#[test]
fn independently_constructed_auxiliary_ellipsis_roundtrips_without_discourse_context() {
    let mut auxiliary = word(
        "core-verb:Do",
        WordForm::Present,
        FeatureBundle {
            number: Some(Number::Singular),
            person: Some(Person::Second),
            tense: Some(Tense::Present),
            finiteness: Some(Finiteness::Finite),
            ..FeatureBundle::default()
        },
    );
    auxiliary.frame = Some(1);
    let value = Reading::FiniteClause {
        form: 0,
        subject: Box::new(Reading::NominativePhrase {
            form: 0,
            head: Box::new(Reading::NominativePronoun {
                form: 0,
                head: word(
                    "vocab:SubjectPronoun/You",
                    WordForm::Invariant,
                    FeatureBundle {
                        number: Some(Number::Singular),
                        person: Some(Person::Second),
                        case: Some(Case::Nominative),
                        ..FeatureBundle::default()
                    },
                ),
            }),
        }),
        predicate: Box::new(Reading::FiniteBareAuxiliary {
            form: 0,
            head: auxiliary,
            complement: Box::new(Reading::BareEllipsis {
                form: 0,
                omission: Box::new(Reading::OmittedPlainActive { form: 0 }),
            }),
        }),
    };
    assert_eq!(value.realize(lexicon()).unwrap(), "you do");
    assert!(readings("you do", Category::FiniteClause).contains(&value));
    let mut before = vec![];
    value
        .visit_words(&mut |word| before.push(word.clone()))
        .unwrap();
    assert_eq!(before.len(), 2, "ellipsis invents no lexical material");
    let mut after = vec![];
    readings("you do", Category::FiniteClause)
        .get(&value)
        .unwrap()
        .visit_words(&mut |word| after.push(word.clone()))
        .unwrap();
    assert_eq!(before, after);
}
