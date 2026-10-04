use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;

constructions! {
    pub mod fixture {
        category Label();
        category Group();
        category Huge();
        category Leaf();
        construction Leaf: Leaf { form [word: lexical(Noun)]; }
        construction Recognized: Label { form ["label"]; }
        construction Fallback: Label { cost 100; form ["label"]; }
        construction Group: Group {
            form [first: Label, optional: optional(Label), rest: repeat(Label, ",")];
        }
        construction Huge: Huge { cost 18446744073709551615; form ["huge"]; }
        construction Overflow: Group { form [child: Huge]; }
    }
}

fn label() -> fixture::Reading {
    fixture::Reading::Recognized { form: 0 }
}

#[test]
fn costs_rank_without_removing_readings() {
    let lexicon = Lexicon::new(vec![]).unwrap();
    let grammar = fixture::Grammar::default();
    let analyzed = lexicon.analyze("label");
    let forest = parse(&grammar, &lexicon, &analyzed, &fixture::Category::Label).unwrap();
    let mut readings: Vec<_> = grammar.readings(&forest).map(Result::unwrap).collect();
    assert_eq!(readings.len(), 2);
    readings.sort_by_key(|reading| reading.total_cost().unwrap());
    assert_eq!(readings[0].local_cost(), 1);
    assert_eq!(readings[1].local_cost(), 100);
    for reading in readings {
        assert!(reading.admit(&lexicon).is_ok());
    }
}

#[test]
fn costs_include_optional_and_repeated_children() {
    let group = fixture::Reading::Group {
        form: 0,
        first: Box::new(label()),
        optional: Some(Box::new(fixture::Reading::Fallback { form: 0 })),
        rest: vec![label(), label()],
    };
    assert_eq!(group.total_cost().unwrap(), 104);
    let empty = fixture::Reading::Group {
        form: 0,
        first: Box::new(label()),
        optional: None,
        rest: vec![],
    };
    assert_eq!(empty.total_cost().unwrap(), 2);
}

#[test]
fn cost_overflow_and_invalid_forms_are_errors() {
    let overflow = fixture::Reading::Overflow {
        form: 0,
        child: Box::new(fixture::Reading::Huge { form: 0 }),
    };
    assert!(matches!(
        overflow.total_cost(),
        Err(fixture::Error::CostOverflow)
    ));
    assert!(
        fixture::Reading::Recognized { form: 9 }
            .total_cost()
            .is_err()
    );
}

#[test]
fn lexical_leaves_add_no_cost() {
    let lexicon = Lexicon::new(vec![Lexeme::invariant(
        "card",
        "card",
        Category::Noun,
        Source {
            kind: SourceKind::Core,
            path: "synthetic".into(),
            owner: "card".into(),
        },
    )])
    .unwrap();
    let grammar = fixture::Grammar::default();
    let analyzed = lexicon.analyze("card");
    let forest = parse(&grammar, &lexicon, &analyzed, &fixture::Category::Leaf).unwrap();
    let readings: Vec<_> = grammar.readings(&forest).map(Result::unwrap).collect();
    assert_eq!(readings.len(), 1);
    assert_eq!(readings[0].total_cost().unwrap(), 1);
}
