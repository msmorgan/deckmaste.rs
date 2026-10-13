use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::{Grammar as _, Symbol};
use deckmaste_lexical::Lexicon;

constructions! {
    pub mod fixture {
        feature Flag { No, Yes } combine all;
        category All(Flag);
        category Any(Flag combine any);
        category Same(Flag combine same);
        category YesOnly();
        category NoOnly();
        schema YesValue { form ["yes"]; export Flag = Yes; }
        schema NoValue { form ["no"]; export Flag = No; }
        instance YesValue<Result>: [(All), (Any), (Same)] {}
        instance NoValue<Result>: [(All), (Any), (Same)] {}
        schema Pair { head none; form [left: node, " + ", right: node]; }
        coordination_forms Pairs { binary Pair; }
        coordinable All using Pairs { forms binary; }
        coordinable Any using Pairs { forms binary; }
        coordinable Same using Pairs { forms binary; }
        construction YesOnly: YesOnly { form [item: All]; require item.Flag = Yes; }
        construction NoOnly: NoOnly { form [item: All]; require item.Flag = No; }
    }
}

fn run(production: usize, children: &[fixture::Summary]) -> Option<fixture::Summary> {
    let grammar = fixture::Grammar::default();
    let mut states = grammar.begin(production);
    let mut children = children.iter();
    for (dot, symbol) in grammar.productions()[production].symbols.iter().enumerate() {
        let child = if let Symbol::Literal(_) = symbol { None } else { children.next() };
        states = states
            .iter()
            .flat_map(|state| grammar.advance(production, dot, state, child))
            .collect();
    }
    let results: Vec<_> = states
        .iter()
        .filter_map(|state| grammar.complete(production, state))
        .collect();
    assert!(results.len() <= 1);
    results.into_iter().next()
}

#[test]
fn strict_combiners_check_every_present_and_absent_pair_in_the_generated_runtime() {
    use fixture::Category::{All, Any, NoOnly, Same, YesOnly};
    let grammar = fixture::Grammar::default();
    let lexicon = Lexicon::new(vec![]).unwrap();
    let yes = fixture::Reading::YesValue {
        category: All,
        form: 0,
    }
    .admit(&lexicon)
    .unwrap();
    let no = fixture::Reading::NoValue {
        category: All,
        form: 0,
    }
    .admit(&lexicon)
    .unwrap();
    let values = [yes, no, fixture::Summary::default()];
    let probe = |category| {
        grammar
            .productions()
            .iter()
            .position(|p| p.category == category)
            .unwrap()
    };
    for (category, expected) in [
        (
            All,
            [
                Some(true),
                Some(false),
                None,
                Some(false),
                Some(false),
                None,
                None,
                None,
                None,
            ],
        ),
        (
            Any,
            [
                Some(true),
                Some(true),
                None,
                Some(true),
                Some(false),
                None,
                None,
                None,
                None,
            ],
        ),
        (
            Same,
            [
                Some(true),
                None,
                None,
                None,
                Some(false),
                None,
                None,
                None,
                None,
            ],
        ),
    ] {
        let pair = grammar
            .productions()
            .iter()
            .position(|p| p.category == category && p.symbols.len() == 3)
            .unwrap();
        for (i, left) in values.iter().enumerate() {
            for (j, right) in values.iter().enumerate() {
                let actual = run(pair, &[left.clone(), right.clone()]).map(|summary| {
                    let yes = run(probe(YesOnly), std::slice::from_ref(&summary)).is_some();
                    let no = run(probe(NoOnly), &[summary]).is_some();
                    assert_eq!((yes, no), if yes { (true, false) } else { (false, true) });
                    yes
                });
                assert_eq!(actual, expected[i * 3 + j], "{category:?}: inputs {i}, {j}");
            }
        }
    }
}

#[test]
fn generated_pair_keeps_the_shared_constructor_and_bidirectional_reading() {
    let lexicon = Lexicon::new(vec![]).unwrap();
    let expected = fixture::Reading::Pair {
        category: fixture::Category::Any,
        form: 0,
        left: Box::new(fixture::Reading::NoValue {
            category: fixture::Category::Any,
            form: 0,
        }),
        right: Box::new(fixture::Reading::YesValue {
            category: fixture::Category::Any,
            form: 0,
        }),
    };
    assert_eq!(expected.realize(&lexicon).unwrap(), "no + yes");
    let grammar = fixture::Grammar::default();
    let forest = deckmaste_english_v3::parse(
        &grammar,
        &lexicon,
        &lexicon.analyze("no + yes"),
        &fixture::Category::Any,
    )
    .unwrap();
    assert_eq!(
        grammar
            .readings(&forest)
            .collect::<Result<Vec<_>, _>>()
            .unwrap(),
        vec![expected]
    );
}
