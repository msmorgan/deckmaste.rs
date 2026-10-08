use crate::common;
use deckmaste_english_v3::grammar::Category;

#[test]
fn shortened_names_need_their_cards_scope() {
    for text in [
        "Chandra deals 2 damage to any target.",
        "Jace draws two cards.",
        "Ajani deals 2 damage to any target.",
        "Gandalf deals 2 damage to any target.",
        "Niv-Mizzet deals 1 damage to any target.",
    ] {
        assert_eq!(
            common::readings(text, Category::Sentence).len(),
            0,
            "{text}"
        );
    }
}

#[test]
fn shortened_names_keep_one_card_identity_and_roundtrip_in_its_scope() {
    use deckmaste_english_v3::grammar::Reading;
    use deckmaste_english_v3::parse;
    use deckmaste_lexical::LexemeId;
    use std::collections::BTreeSet;

    let lexicon = common::lexicon();
    let grammar = common::ENVIRONMENT.grammar();
    for (full, short, text) in [
        (
            "Chandra, Pyromaster",
            "Chandra",
            "Chandra deals 2 damage to any target.",
        ),
        ("Jace, Cunning Castaway", "Jace", "Jace draws two cards."),
        (
            "Ajani Vengeant",
            "Ajani",
            "Ajani deals 2 damage to any target.",
        ),
        (
            "Gandalf the Grey",
            "Gandalf",
            "Gandalf deals 2 damage to any target.",
        ),
        (
            "Niv-Mizzet, Parun",
            "Niv-Mizzet",
            "Niv-Mizzet deals 1 damage to any target.",
        ),
    ] {
        let owner = LexemeId::new(&format!("catalog:card-names.txt/{full}"));
        let scope = BTreeSet::from([owner]);
        let input = lexicon.analyze_source_scoped(text, None, &scope);
        let forest = parse(grammar, lexicon, &input, &Category::Sentence).unwrap();
        let values: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
        assert_eq!(values.len(), 1, "{text}");
        for value in values {
            assert_eq!(value.realize(lexicon).unwrap(), text);
            let context = deckmaste_english_v3::grammar::RealizationContext::new_scoped(
                grammar, lexicon, text, &scope,
            );
            assert_eq!(context.realize(&value).unwrap(), text);
            assert!(
                deckmaste_english_v3::grammar::RealizationContext::new(grammar, lexicon, text,)
                    .realize(&value)
                    .is_err()
            );
            let mut names = Vec::new();
            value
                .visit_words(&mut |word| {
                    if let deckmaste_lexical::LexicalReading::Word(value) = &word.value
                        && value.lexeme == owner
                    {
                        names.push(value.clone());
                    }
                })
                .unwrap();
            assert_eq!(names.len(), 1);
            assert_eq!(names[0].variant, 1);
        }
        // Independently construct the Name, preserving the declared variant.
        let value = lexicon
            .values()
            .find(|value| value.lexeme == owner && value.variant == 1)
            .unwrap();
        let name = Reading::CatalogName {
            form: 0,
            head: deckmaste_english_v3::grammar::Word {
                value: deckmaste_lexical::LexicalReading::Word(value.clone()),
                frame: None,
                countability: None,
            },
        };
        assert_eq!(name.realize(lexicon).unwrap(), short);
        let context = deckmaste_english_v3::grammar::RealizationContext::new_scoped(
            grammar, lexicon, full, &scope,
        );
        assert_eq!(context.realize(&name).unwrap(), short);
        assert!(
            deckmaste_english_v3::grammar::RealizationContext::new(grammar, lexicon, full,)
                .realize(&name)
                .is_err()
        );
        let input = lexicon.analyze_source_scoped(short, None, &scope);
        let forest = parse(grammar, lexicon, &input, &Category::Name).unwrap();
        assert_eq!(
            grammar
                .readings(&forest)
                .map(Result::unwrap)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([name])
        );
        assert_eq!(common::readings(full, Category::Name).len(), 1);
    }
    for (full, text) in [
        ("Jace, the Mind Sculptor", "Jace draws two cards."),
        ("Ajani Goldmane", "Ajani deals 2 damage to any target."),
    ] {
        let scope = BTreeSet::from([LexemeId::new(&format!("catalog:card-names.txt/{full}"))]);
        let input = lexicon.analyze_source_scoped(text, None, &scope);
        let forest = parse(grammar, lexicon, &input, &Category::Sentence).unwrap();
        assert_eq!(grammar.readings(&forest).count(), 0);
    }
    assert_eq!(
        common::readings(
            "Search your library for a Chandra planeswalker card.",
            Category::Sentence
        )
        .len(),
        4
    );
}

#[test]
fn type_lines_use_subtype_nouns_but_never_plural_or_negative_forms() {
    for text in [
        "Legendary Planeswalker — Chandra",
        "Creature — Human Wizard",
        "Artifact — Junk",
        "Battle — Siege",
        "Enchantment — Aura",
        "Basic Land — Forest",
        "Instant — Arcane",
    ] {
        assert_eq!(
            common::readings(text, Category::TypeLine).len(),
            1,
            "{text}"
        );
    }
    for text in [
        "Creature — Humans",
        "Creature — non-Human",
        "Creature — human",
    ] {
        assert_eq!(
            common::readings(text, Category::TypeLine).len(),
            0,
            "{text}"
        );
    }
}

#[test]
fn plural_full_name_subject_and_token_name_keep_their_intended_identity() {
    use deckmaste_english_v3::grammar::Reading;
    use deckmaste_lexical::{LexemeId, LexicalReading};
    for (text, category, identity) in [
        (
            "Aang and Katara enter.",
            Category::Sentence,
            "catalog:card-names.txt/Aang and Katara",
        ),
        ("named Wasp", Category::NamePredicate, "lexeme:Name/Wasp"),
    ] {
        let values = common::readings(text, category);
        assert_eq!(values.len(), 1, "{text}");
        for value in values {
            assert_eq!(value.realize(common::lexicon()).unwrap(), text);
            let mut names = Vec::new();
            value
                .visit_words(&mut |word| {
                    if let LexicalReading::Word(value) = &word.value
                        && value.lexeme == LexemeId::new(identity)
                    {
                        names.push(value.clone());
                    }
                })
                .unwrap();
            assert_eq!(names.len(), 1);
            assert_eq!(names[0].variant, 0);
            if category == Category::NamePredicate {
                let Reading::PassiveNamePredicate { complement, .. } = value else {
                    panic!("expected named complement");
                };
                assert_eq!(
                    *complement,
                    Reading::CatalogName {
                        form: 0,
                        head: deckmaste_english_v3::grammar::Word {
                            value: LexicalReading::Word(names[0].clone()),
                            frame: None,
                            countability: None,
                        }
                    }
                );
            }
        }
    }
    assert!(common::readings("Aang and Katara enters.", Category::Sentence).is_empty());
}
