use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Relation;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn selected_slot() -> FrameItem {
    FrameItem::Marked {
        vocabulary: "Preposition".into(),
        member: "With".into(),
        slot: FrameSlot {
            relation: Relation::Complement,
            category: "VerblessClause".into(),
        },
    }
}

fn witness(card: &str, text: &str, subject_text: &str, category: Category) {
    let values = common::readings_on_card(card, text, category);
    assert!(!values.is_empty(), "missing Reading of {text}");
    let mut selected = Vec::new();
    for value in values {
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme == "lexeme:keyword_action/play"
                    && let [FrameValue::Marked { marker, argument }] = complements.as_slice()
                {
                    let frame = &common::lexicon().lexemes()[&word.lexeme].properties.frames
                        [head.frame.unwrap()];
                    assert_eq!(frame.items, vec![selected_slot()]);
                    assert_eq!(common::lexicon().realize(&marker.value).unwrap(), "with");
                    let Reading::SubjectParticipialClause {
                        subject, predicate, ..
                    } = argument.as_ref()
                    else {
                        panic!("the selected Complement must retain Subject and predicate");
                    };
                    assert_eq!(subject.category(), Category::AccusativePhrase);
                    assert_eq!(subject.realize(common::lexicon()).unwrap(), subject_text);
                    assert_eq!(predicate.category(), Category::SecondaryVerbPhrase);
                    assert_eq!(predicate.realize(common::lexicon()).unwrap(), "revealed");
                    selected.push(node.clone());
                }
            })
            .unwrap();
    }
    assert!(
        !selected.is_empty(),
        "coverage without the selected clause: {text}"
    );
}

#[test]
fn telepathy_selects_a_plural_subject_predicate_complement() {
    witness(
        "Telepathy",
        "Your opponents play with their hands revealed.",
        "their hands",
        Category::Document,
    );
}

#[test]
fn enduring_renewal_selects_the_clause_in_an_imperative() {
    witness(
        "Enduring Renewal",
        "Play with your hand revealed.",
        "your hand",
        Category::Document,
    );
}

#[test]
fn field_of_dreams_retains_the_subjects_nominal_structure() {
    witness(
        "Field of Dreams",
        "Players play with the top card of their libraries revealed.",
        "the top card of their libraries",
        Category::Document,
    );
}

#[test]
fn future_sight_selects_the_clause_in_an_imperative() {
    witness(
        "Future Sight",
        "Play with the top card of your library revealed.",
        "the top card of your library",
        Category::Document,
    );
}

#[test]
fn sen_triplets_selects_the_clause_in_a_coordinated_finite_predicate() {
    witness(
        "Sen Triplets",
        "This turn, that player can't cast spells or activate abilities and plays with their hand revealed.",
        "their hand",
        Category::Document,
    );
}

fn without_new_frames() -> Lexicon {
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    let play = lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id.as_str() == "lexeme:keyword_action/play")
        .unwrap();
    play.properties.frames.truncate(1);
    Lexicon::new(lexemes).unwrap()
}

#[test]
fn unrelated_predicates_transitive_frames_and_the_card_name_keep_their_values() {
    let baseline = without_new_frames();
    for text in [
        "You may play that card.",
        "Play lands.",
        "Players can't play lands.",
        "Players attack.",
        "You may look at that card with it revealed.",
        "Players attack with their hands revealed.",
        "It enters with a +1/+1 counter on it.",
        "Play with Fire deals 2 damage to any target.",
    ] {
        let before = common::readings_with_lexicon(&baseline, text, Category::Document);
        assert!(!before.is_empty(), "missing baseline control: {text}");
        assert_eq!(common::readings(text, Category::Document), before, "{text}");
    }
}

fn word(owner: &str, form: WordForm, frame: Option<usize>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability: None,
    }
}

fn independently_selected_clause() -> Reading {
    let play = &common::lexicon().lexemes()["lexeme:keyword_action/play"];
    let frame = play
        .properties
        .frames
        .iter()
        .position(|frame| frame.items == vec![selected_slot()])
        .unwrap();
    Reading::SelectedPredicate {
        form: 0,
        category: Category::SecondaryVerbPhrase,
        head: word("lexeme:keyword_action/play", WordForm::Plain, Some(frame)),
        complements: vec![FrameValue::Marked {
            marker: word("vocab:Preposition/With", WordForm::Invariant, None),
            argument: Box::new(Reading::SubjectParticipialClause {
                form: 0,
                subject: Box::new(
                    common::readings("their hands", Category::AccusativePhrase)
                        .into_iter()
                        .next()
                        .unwrap(),
                ),
                predicate: Box::new(
                    common::readings("revealed", Category::SecondaryVerbPhrase)
                        .into_iter()
                        .next()
                        .unwrap(),
                ),
            }),
        }],
    }
}

#[test]
fn independently_selected_complement_preserves_both_roundtrip_laws() {
    // Revelation supplies the attested bare constituent of Players play ... .
    let expected = independently_selected_clause();
    expected.admit(common::lexicon()).unwrap();
    let text = expected.realize(common::lexicon()).unwrap();
    assert_eq!(text, "play with their hands revealed");
    assert!(common::readings(&text, Category::SecondaryVerbPhrase).contains(&expected));
    let mut words = Vec::new();
    expected
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    assert_eq!(
        words.first().unwrap(),
        &word(
            "lexeme:keyword_action/play",
            WordForm::Plain,
            expected_head_frame(&expected)
        )
    );
    assert_eq!(
        words[1],
        word("vocab:Preposition/With", WordForm::Invariant, None)
    );
}

fn expected_head_frame(value: &Reading) -> Option<usize> {
    let Reading::SelectedPredicate { head, .. } = value else {
        panic!("expected a selected predicate");
    };
    head.frame
}

#[test]
fn admission_requires_the_declared_verb_frame_and_marker_licence() {
    let expected = independently_selected_clause();
    assert!(expected.admit(&without_new_frames()).is_err());
    let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    lexemes
        .iter_mut()
        .find(|lexeme| lexeme.id.as_str() == "vocab:Preposition/With")
        .unwrap()
        .properties
        .features
        .insert(
            "PrepositionFunctionLicence".into(),
            "Modifier|Adjunct|PreposedAdjunct".into(),
        );
    assert!(expected.admit(&Lexicon::new(lexemes).unwrap()).is_err());
}

#[test]
fn the_clause_is_neither_a_free_adjunct_nor_an_object_postmodifier() {
    let phrases = common::readings("with their hands revealed", Category::PrepositionPhrase);
    assert!(!phrases.is_empty());
    assert!(phrases.iter().all(|value| {
        let mut clauses = Vec::new();
        value
            .visit(&mut |node| {
                if node.category() == Category::VerblessClause {
                    clauses.push(node.clone());
                }
            })
            .unwrap();
        clauses.is_empty()
    }));
    assert!(common::readings("play with able", Category::SecondaryVerbPhrase).is_empty());
    assert!(
        common::readings(
            "play with their hands revealing",
            Category::SecondaryVerbPhrase
        )
        .is_empty()
    );
}

#[test]
fn stromgald_spy_selects_the_clause_under_causative_have() {
    witness(
        "Stromgald Spy",
        "have defending player play with their hand revealed",
        "their hand",
        Category::SecondaryVerbPhrase,
    );
}
