use std::collections::BTreeMap;
use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Lexicon;

constructions! {
    pub mod nullable {
        category Empty();
        category Item();
        category OptionalPair();
        category Sequence();
        construction Empty: Empty { form []; }
        construction Bare: Item { form []; }
        construction Nested: Item { form [child: Empty]; }
        construction OptionalPair: OptionalPair { form [left: optional(Item), right: optional(Item)]; }
        construction Sequence: Sequence { form [items: repeat(Item, ",")]; }
    }
}

#[test]
fn nullable_optional_and_consuming_repetition_preserve_every_independent_value() {
    use nullable::Reading;
    let lexicon = Lexicon::new([]).unwrap();
    let grammar = nullable::Grammar::default();
    let items = [
        Reading::Bare { form: 0 },
        Reading::Nested {
            form: 0,
            child: Box::new(Reading::Empty { form: 0 }),
        },
    ];
    let choices = [
        None,
        Some(Box::new(items[0].clone())),
        Some(Box::new(items[1].clone())),
    ];
    let mut values = vec![];
    for left in &choices {
        for right in &choices {
            values.push(Reading::OptionalPair {
                form: 0,
                left: left.clone(),
                right: right.clone(),
            });
        }
    }
    for length in 0..=4 {
        for bits in 0..(1 << length) {
            values.push(Reading::Sequence {
                form: 0,
                items: (0..length)
                    .map(|i| items[usize::from(bits & (1 << i) != 0)].clone())
                    .collect(),
            });
        }
    }
    let mut expected = BTreeMap::<_, BTreeSet<_>>::new();
    for value in values {
        expected
            .entry((value.category(), value.realize(&lexicon).unwrap()))
            .or_default()
            .insert(value);
    }
    assert_eq!(
        expected[&(nullable::Category::OptionalPair, String::new())].len(),
        9
    );
    assert_eq!(
        expected[&(nullable::Category::Sequence, String::new())].len(),
        3
    );
    for ((category, text), expected) in expected {
        let forest = parse(&grammar, &lexicon, &lexicon.analyze(&text), &category).unwrap();
        let actual: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
        assert_eq!(actual, expected);
        for reading in actual {
            assert_eq!(reading.realize(&lexicon).unwrap(), text);
        }
    }
}
