use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Numeral;

constructions! {
    pub mod grammar {
        category Cardinal(number);
        category Singular();
        category Plural();
        category Ordinal();
        category NumberedOrdinal(number);
        category NumberedRoman(number);
        category Grouped();
        category Large();
        construction Cardinal: Cardinal {
            form [word: lexical(Numeral)];
            require word.numeral_kind = Cardinal;
            export number = word.number;
        }
        construction Singular: Singular {
            form [value: Cardinal];
            require value.number = Singular;
        }
        construction Plural: Plural {
            form [value: Cardinal];
            require value.number = Plural;
        }
        construction Ordinal: Ordinal {
            form [word: lexical(Numeral)];
            require word.numeral_kind = Ordinal;
        }
        construction NumberedOrdinal: NumberedOrdinal {
            form [word: lexical(Numeral)];
            require word.numeral_kind = Ordinal;
            export number = word.number;
        }
        construction NumberedRoman: NumberedRoman {
            form [word: lexical(Numeral)];
            require word.numeral_kind = Roman;
            export number = word.number;
        }
        construction Grouped: Grouped {
            form [word: lexical(Numeral)];
            require word.numeral_kind = GroupedArabic;
        }
        construction Large: Large {
            form [word: lexical(Numeral)];
            require word.numeral_kind = GroupedArabic;
            require word.numeral_size = Large;
        }
    }
}

fn readings(text: &str, category: grammar::Category) -> Vec<grammar::Reading> {
    let lexicon = Lexicon::new([]).unwrap();
    let grammar = grammar::Grammar::default();
    let input = lexicon.analyze(text);
    let forest = parse(&grammar, &lexicon, &input, &category).unwrap();
    grammar.readings(&forest).map(Result::unwrap).collect()
}

#[test]
fn numeral_number_is_correlated_with_cardinal_magnitude_and_inapplicable_to_ordinals() {
    use grammar::Category;
    for (text, singular) in [
        ("one", true),
        ("negative one", true),
        ("zero", false),
        ("two", false),
    ] {
        assert_eq!(!readings(text, Category::Singular).is_empty(), singular);
        assert_eq!(!readings(text, Category::Plural).is_empty(), !singular);
    }
    assert!(!readings("second", Category::Ordinal).is_empty());
    assert!(readings("second", Category::Cardinal).is_empty());
    assert!(readings("first", Category::NumberedOrdinal).is_empty());
    assert!(readings("second", Category::NumberedOrdinal).is_empty());
    assert!(readings("I", Category::NumberedRoman).is_empty());
    assert!(readings("II", Category::NumberedRoman).is_empty());
}

#[test]
fn numeral_style_constraints_preserve_the_selected_notation_even_without_visible_commas() {
    use grammar::Category;
    for text in ["2", "1,000"] {
        let values = readings(text, Category::Grouped);
        assert_eq!(values.len(), 1);
        let grammar::Reading::Grouped { word, .. } = &values[0] else {
            panic!("wrong root")
        };
        assert!(matches!(
            word.value,
            LexicalReading::Numeral {
                notation: Numeral::Arabic(true),
                ..
            }
        ));
    }
    assert!(readings("1000", Category::Grouped).is_empty());
    assert!(readings("999", Category::Large).is_empty());
    assert!(!readings("1,000", Category::Large).is_empty());
}
