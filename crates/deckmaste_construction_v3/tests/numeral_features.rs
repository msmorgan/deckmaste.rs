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
        category Ungrouped();
        category Nonnegative();
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
        construction Ungrouped: Ungrouped {
            form [word: lexical(Numeral)];
            require word.numeral_kind = Arabic;
        }
        construction Nonnegative: Nonnegative {
            form [word: lexical(Numeral)];
            require word.numeral_kind = Arabic;
            require word.numeral_sign = Nonnegative;
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
fn numeral_style_constraints_preserve_visible_grouping_and_normalize_small_digits() {
    use grammar::Category;
    for text in ["1,000", "-1,000"] {
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
    assert!(readings("2", Category::Grouped).is_empty());
    assert!(readings("-999", Category::Grouped).is_empty());
    for (text, value) in [("2", 2), ("-999", -999), ("1000", 1000)] {
        assert_eq!(
            readings(text, Category::Ungrouped),
            vec![grammar::Reading::Ungrouped {
                form: 0,
                word: grammar::Word {
                    value: LexicalReading::Numeral {
                        value,
                        notation: Numeral::Arabic(false),
                        capitalization: deckmaste_lexical::SurfaceCase::Declared,
                    },
                    frame: None,
                    countability: None,
                },
            }]
        );
    }
    assert!(readings("1000", Category::Grouped).is_empty());
    assert!(readings("999", Category::Large).is_empty());
    assert!(!readings("1,000", Category::Large).is_empty());
}

#[test]
fn numeral_sign_constrains_parsing_and_independent_construction() {
    for text in ["0", "1", "999"] {
        assert_eq!(readings(text, grammar::Category::Nonnegative).len(), 1);
    }
    for text in ["-1", "-999", "-2147483648"] {
        assert!(readings(text, grammar::Category::Nonnegative).is_empty());
    }
    let mut values = readings("1", grammar::Category::Nonnegative);
    let grammar::Reading::Nonnegative { word, .. } = &mut values[0] else {
        panic!("wrong root")
    };
    let LexicalReading::Numeral { value, .. } = &mut word.value else {
        panic!("wrong leaf")
    };
    *value = -1;
    assert!(values[0].admit(&Lexicon::new([]).unwrap()).is_err());
}
