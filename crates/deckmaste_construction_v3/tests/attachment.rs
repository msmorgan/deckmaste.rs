use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Category;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

constructions! {
    pub mod attachment {
        category Expression();
        construction Leaf: Expression { form [word: lexical(Adjective)]; }
        construction Attach: Expression { form [left: Expression, " ", right: Expression]; }
    }
}

fn independent(length: usize) -> BTreeSet<attachment::Reading> {
    use attachment::Reading;
    if length == 1 {
        return BTreeSet::from([Reading::Leaf {
            form: 0,
            word: attachment::Word {
                value: LexicalReading::Word(LexicalValue {
                    lexeme: "unit".into(),
                    form: WordForm::Invariant,
                    features: FeatureBundle::default(),
                    variant: 0,
                    capitalization: SurfaceCase::Declared,
                }),
                frame: None,
                countability: None,
            },
        }]);
    }
    let mut trees = BTreeSet::new();
    for split in 1..length {
        for left in independent(split) {
            for right in independent(length - split) {
                trees.insert(Reading::Attach {
                    form: 0,
                    left: Box::new(left.clone()),
                    right: Box::new(right),
                });
            }
        }
    }
    trees
}

#[test]
fn consuming_recursive_attachment_retains_every_structure_and_both_roundtrips() {
    let lexicon = Lexicon::new([Lexeme::invariant(
        "unit",
        "x",
        Category::Adjective,
        Source {
            kind: SourceKind::Core,
            path: "synthetic".into(),
            owner: "unit".into(),
        },
    )])
    .unwrap();
    let grammar = attachment::Grammar::default();
    for (length, count) in [(1, 1), (2, 1), (3, 2), (4, 5), (5, 14)] {
        let expected = independent(length);
        assert_eq!(expected.len(), count);
        let text = vec!["x"; length].join(" ");
        for value in &expected {
            assert_eq!(value.realize(&lexicon).unwrap(), text);
        }
        let forest = parse(
            &grammar,
            &lexicon,
            &lexicon.analyze(&text),
            &attachment::Category::Expression,
        )
        .unwrap();
        let actual: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
        assert_eq!(actual, expected);
        for value in actual {
            assert_eq!(value.realize(&lexicon).unwrap(), text);
            let mut words = vec![];
            value
                .visit_words(&mut |word| words.push(word.value.clone()))
                .unwrap();
            assert_eq!(words.len(), length);
            assert!(words.windows(2).all(|pair| pair[0] == pair[1]));
            let mut visited = vec![];
            value
                .visit(&mut |node| visited.push(node.category()))
                .unwrap();
            assert_eq!(
                visited,
                vec![attachment::Category::Expression; 2 * length - 1]
            );
        }
    }
}
