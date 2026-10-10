use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading};
use deckmaste_lexical::LexicalReading;

fn assert_named_sentence_list(card: &str, text: &str) {
    let expected: Vec<_> = text
        .lines()
        .filter_map(|line| {
            line.strip_prefix("• ")
                .and_then(|line| line.split_once(" — "))
        })
        .collect();
    let readings = common::readings_on_card(card, text, Category::Document);
    assert!(!readings.is_empty(), "{card}");
    let mut observed = Vec::new();
    for reading in readings {
        let mut actual = Vec::new();
        let mut sentence_list = false;
        reading
            .visit(&mut |node| {
                if let Reading::ModalItem { header, modes, .. } = node {
                    assert_eq!(header.category(), Category::SentenceParagraph);
                    assert_eq!(modes.category(), Category::NamedModes);
                    sentence_list = true;
                }
                if let Reading::Mode { body, .. } = node {
                    let Reading::FlavorWordHead { head, body, .. } = body.as_ref() else {
                        panic!("every sentence-list Mode must have a Flavor Word head")
                    };
                    let LexicalReading::FlavorWord { label } = &head.value else {
                        panic!("the bullet must be outside the label")
                    };
                    let Reading::OrdinaryAbility { body, .. } = body.as_ref() else {
                        panic!("the Mode body must retain its ordinary Paragraph")
                    };
                    assert_eq!(body.category(), Category::Paragraph);
                    actual.push((label.clone(), body.realize(common::lexicon()).unwrap()));
                }
            })
            .unwrap();
        assert!(sentence_list);
        assert_eq!(
            actual
                .iter()
                .map(|(label, _)| label.as_str())
                .collect::<Vec<_>>(),
            expected.iter().map(|(label, _)| *label).collect::<Vec<_>>()
        );
        observed.push(actual);
    }
    assert!(
        observed.contains(
            &expected
                .iter()
                .map(|(label, body)| ((*label).to_owned(), (*body).to_owned()))
                .collect::<Vec<_>>()
        )
    );
}

#[test]
fn siege_sentence_line_hosts_named_modes_with_paragraph_bodies() {
    let text = SIEGE;
    assert_named_sentence_list("Barrensteppe Siege", text);
    for plain_label in ["• Abzan — ", "• Mardu — "] {
        let mixed = text.replace(plain_label, "• ");
        assert!(
            common::readings_on_card("Barrensteppe Siege", &mixed, Category::Document).is_empty()
        );
    }
    let (header, list) = text.split_once('\n').unwrap();
    let header = common::readings(header, Category::SentenceParagraph)
        .into_iter()
        .next()
        .unwrap();
    let modes = common::readings(list, Category::NamedModes)
        .into_iter()
        .next()
        .unwrap();
    let expected = Reading::ModalItem {
        category: Category::Ability,
        form: 1,
        header: Box::new(header),
        modes: Box::new(modes),
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert!(common::readings(text, Category::Ability).contains(&expected));
}

#[test]
fn typhoid_mary_sentence_paragraph_hosts_all_three_named_modes() {
    assert_named_sentence_list("Typhoid Mary, Fractured", MARY);
}

const SIEGE: &str = "As this enchantment enters, choose Abzan or Mardu.\n• Abzan — At the beginning of your end step, put a +1/+1 counter on each creature you control.\n• Mardu — At the beginning of your end step, if a creature died under your control this turn, each opponent sacrifices a creature of their choice.";
const MARY: &str = "Whenever Typhoid Mary attacks, choose one at random. If you discarded a card this turn, you choose one instead.\n• Mary — Create a Treasure token.\n• Typhoid Mary — Draw a card.\n• Bloody Mary — Each opponent loses 2 life and you gain 2 life.";
