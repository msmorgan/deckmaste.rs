use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::WordForm;

use crate::common;

fn witness(text: &str, count: usize, frame: usize, object: &str, category: Category) {
    let values = common::readings(text, Category::Document);
    assert_eq!(values.len(), count, "{text}");
    let objects = common::readings(object, category);
    for value in values {
        let mut found = 0;
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme.as_str() == "core-verb:Get"
                {
                    found += 1;
                    assert_eq!(head.frame, Some(frame));
                    assert_eq!(complements.len(), 1);
                    let FrameValue::Argument(actual) = &complements[0] else {
                        panic!("expected one selected argument: {complements:?}");
                    };
                    assert!(objects.contains(actual.as_ref()), "{actual:?}");
                    assert_eq!(actual.realize(common::lexicon()).unwrap(), object);
                }
            })
            .unwrap();
        assert_eq!(found, 1, "{text}");
    }
}

#[test]
fn attune_with_aether() {
    witness("You get {E}{E}.", 1, 2, "{E}{E}", Category::CostSymbols);
}

#[test]
fn phyresis_outbreak() {
    witness(
        "Each opponent gets a poison counter.",
        1,
        1,
        "a poison counter",
        Category::AccusativePhrase,
    );
}

#[test]
fn crypt_cobra() {
    witness(
        "Whenever this creature attacks and isn't blocked, defending player gets a poison counter.",
        2,
        1,
        "a poison counter",
        Category::AccusativePhrase,
    );
}

#[test]
fn scrapper_champion() {
    witness(
        "When this creature enters, you get {E}{E}.",
        1,
        2,
        "{E}{E}",
        Category::CostSymbols,
    );
}

#[test]
fn vexing_radgull() {
    witness(
        "Whenever this creature deals combat damage to a player, that player gets two rad counters if they don't have any rad counters.",
        3,
        1,
        "two rad counters",
        Category::AccusativePhrase,
    );
}

fn word(owner: &str, form: WordForm) -> Word {
    Word {
        value: LexicalReading::Word(
            common::lexicon()
                .values()
                .find(|value| {
                    value.lexeme.as_str() == owner
                        && value.form == form
                        && (form != WordForm::Present
                            || (value.features.number == Some(Number::Singular)
                                && value.features.person == Some(Person::Third)))
                })
                .unwrap()
                .clone(),
        ),
        frame: None,
        countability: None,
    }
}

fn energy_symbol_word() -> Word {
    let mut symbol = word("vocab:FixedCostSymbol/E", WordForm::Invariant);
    symbol.countability = Some(false);
    symbol
}

#[test]
fn independently_constructed_energy_object_preserves_both_laws_and_traversal() {
    // Predicate fragment from Attune with Aether.
    let symbol = Reading::NamedCostSymbol {
        form: 0,
        symbol: energy_symbol_word(),
    };
    let symbols = Reading::SymbolSequence {
        category: Category::CostSymbols,
        form: 0,
        first: Box::new(symbol.clone()),
        rest: vec![symbol.clone()],
    };
    let mut head = word("core-verb:Get", WordForm::Plain);
    head.frame = Some(2);
    let expected = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: head.clone(),
        complements: vec![FrameValue::Argument(Box::new(symbols.clone()))],
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), "get {E}{E}");
    assert_eq!(
        common::readings("get {E}{E}", Category::SecondaryVerbPhrase),
        BTreeSet::from([expected.clone()])
    );
    let mut nodes = vec![];
    expected
        .visit(&mut |node| nodes.push(node.clone()))
        .unwrap();
    assert_eq!(nodes, [expected.clone(), symbols, symbol.clone(), symbol]);
    let mut words = vec![];
    expected
        .visit_words(&mut |leaf| words.push(leaf.clone()))
        .unwrap();
    assert_eq!(
        words,
        [
            head,
            energy_symbol_word(),
            energy_symbol_word(),
        ]
    );
}

#[test]
fn power_toughness_adjustment_keeps_its_single_frame() {
    // Attested finite-predicate fragment; its Object alternative must not read.
    let mut head = word("core-verb:Get", WordForm::Present);
    head.frame = Some(0);
    let pair = common::readings("+1/+1", Category::SlashPair);
    assert_eq!(pair.len(), 1);
    let expected = Reading::SelectedPredicate {
        category: Category::FinitePredicate,
        form: 0,
        head,
        complements: vec![FrameValue::Argument(Box::new(
            pair.first().unwrap().clone(),
        ))],
    };
    assert_eq!(
        common::readings("gets +1/+1", Category::FinitePredicate),
        BTreeSet::from([expected])
    );
    assert_eq!(
        common::readings("Creatures you control get +1/+1.", Category::Document).len(),
        1
    );
}
