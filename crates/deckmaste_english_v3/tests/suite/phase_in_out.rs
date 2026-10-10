use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, FrameValue, Reading, Word};
use deckmaste_lexical::{
    FrameItem, LexemeId, LexicalReading, Lexicon, Number, Person, SurfaceCase, WordForm,
};

use crate::common;

fn selected_particles(card: &str, text: &str, category: Category, expected: &[&str]) {
    let values = common::readings_on_card(card, text, category);
    assert!(!values.is_empty(), "{card}: {text}");
    for value in values {
        let mut particles = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::SelectedPredicate {
                    head, complements, ..
                } = node
                {
                    let LexicalReading::Word(verb) = &head.value else { return };
                    if verb.lexeme.as_str() != "core-verb:Phase" {
                        return;
                    }
                    let [FrameValue::Marker(particle)] = complements.as_slice() else {
                        panic!("phase requires exactly its selected particle");
                    };
                    let LexicalReading::Word(particle) = &particle.value else {
                        panic!("the selected particle retains its lexical identity");
                    };
                    assert_eq!(particle.form, WordForm::Invariant);
                    particles.push(particle.lexeme.to_string());
                }
            })
            .unwrap();
        assert_eq!(particles, expected, "{card}: {text}");
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
}

#[test]
fn ticket_witnesses_retain_phase_and_its_obligatory_complement() {
    for (card, text) in [
        ("Rainbow Efreet", "{U}{U}: This creature phases out."),
        (
            "Reality Ripple",
            "Target artifact, creature, or land phases out.",
        ),
        (
            "Taniwha",
            "At the beginning of your upkeep, all lands you control phase out.",
        ),
        (
            "March of Swirling Mist",
            "Up to X target creatures phase out.",
        ),
        (
            "Spatial Binding",
            "Pay 1 life: Until your next upkeep, target permanent can't phase out.",
        ),
    ] {
        selected_particles(card, text, Category::Document, &["vocab:Preposition/Out"]);
    }
    selected_particles(
        "Spatial Binding",
        "It can't phase in.",
        Category::Sentence,
        &["vocab:Preposition/In"],
    );
    selected_particles(
        "Change of Plans",
        "You may have any number of them phase out.",
        Category::Sentence,
        &["vocab:Preposition/Out"],
    );
}

#[test]
fn ten_newly_covered_faces_have_the_selected_particle_analysis() {
    for (card, text, particles) in [
        (
            "Blink Dog",
            "Double strike\nTeleport — {3}{W}: This creature phases out.",
            1,
        ),
        (
            "Clever Concealment",
            "Convoke\nAny number of target nonland permanents you control phase out.",
            1,
        ),
        (
            "Creeping Inn",
            "Whenever this creature attacks, you may exile a creature card from your graveyard. If you do, each opponent loses X life and you gain X life, where X is the number of creature cards exiled with this creature.\n{4}: This creature phases out.",
            1,
        ),
        (
            "Crystal Golem",
            "At the beginning of your end step, this creature phases out.",
            1,
        ),
        (
            "Divine Smite",
            "Target creature or planeswalker an opponent controls phases out. If that permanent is black, exile it instead.",
            1,
        ),
        (
            "Dream Fighter",
            "Whenever this creature blocks or becomes blocked by a creature, this creature and that creature phase out.",
            1,
        ),
        (
            "Ertai's Familiar",
            "Phasing\nWhen this creature phases out or leaves the battlefield, mill three cards.\n{U}: Until your next upkeep, this creature can't phase out.",
            2,
        ),
        (
            "Frenetic Efreet",
            "Flying\n{0}: Flip a coin. If you win the flip, this creature phases out. If you lose the flip, sacrifice this creature.",
            1,
        ),
        (
            "Galadriel's Dismissal",
            "Kicker {2}{W}\nTarget creature phases out. If this spell was kicked, each creature target player controls phases out instead.",
            2,
        ),
        (
            "Guardian of Faith",
            "Flash\nVigilance\nWhen this creature enters, any number of other target creatures you control phase out.",
            1,
        ),
    ] {
        selected_particles(
            card,
            text,
            Category::Document,
            &vec!["vocab:Preposition/Out"; particles],
        );
    }
}

fn word(owner: &str, form: WordForm) -> Word {
    let mut value = common::lexicon()
        .values()
        .find(|value| {
            value.lexeme.as_str() == owner
                && value.form == form
                && value.variant == 0
                && (form != WordForm::Present && form != WordForm::Preterite
                    || value.features.number == Some(Number::Singular)
                        && value.features.person == Some(Person::Third))
        })
        .unwrap()
        .clone();
    value.capitalization = SurfaceCase::Declared;
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability: None,
    }
}

#[test]
fn independently_constructed_inflections_and_particles_roundtrip() {
    // Attested constituents: phase in/out; phases out (Rainbow Efreet);
    // phased out (Out of Time). PH1 requires both past inflections.
    let lexicon = common::lexicon();
    for particle in ["In", "Out"] {
        for (form, category, surface) in [
            (WordForm::Plain, Category::SecondaryVerbPhrase, "phase"),
            (WordForm::Present, Category::FinitePredicate, "phases"),
            (WordForm::Preterite, Category::FinitePredicate, "phased"),
            (
                WordForm::PastParticiple,
                Category::SecondaryVerbPhrase,
                "phased",
            ),
        ] {
            let mut head = word("core-verb:Phase", form);
            head.frame = Some(
                lexicon.lexemes()[&LexemeId::new("core-verb:Phase")]
                    .properties
                    .frames
                    .iter()
                    .position(|frame| {
                        frame.kind == "Predicate"
                            && frame.items
                                == vec![FrameItem::Marker {
                                    vocabulary: "Preposition".into(),
                                    member: particle.into(),
                                }]
                    })
                    .unwrap(),
            );
            let value = Reading::SelectedPredicate {
                category,
                form: 0,
                head,
                complements: vec![FrameValue::Marker(word(
                    &format!("vocab:Preposition/{particle}"),
                    WordForm::Invariant,
                ))],
            };
            value.admit(lexicon).unwrap();
            let text = format!("{surface} {}", particle.to_lowercase());
            assert_eq!(value.realize(lexicon).unwrap(), text);
            assert!(common::readings(&text, category).contains(&value));
        }
    }
}

#[test]
fn phase_has_no_bare_frame_or_free_particle_adjunct_analysis() {
    for text in [
        "This creature phases.",
        "This creature phases up.",
        "This creature phases out out.",
        "This creature rounds out.",
    ] {
        assert!(
            common::readings(text, Category::Sentence).is_empty(),
            "{text}"
        );
    }
    assert!(common::noun_phrase_readings("phases out").is_empty());
    let lexicon = common::lexicon();
    assert!(
        !lexicon
            .lexemes()
            .contains_key(&LexemeId::new("vocab:Adverb/Out"))
    );
    for frame in &lexicon.lexemes()[&LexemeId::new("core-verb:Phase")]
        .properties
        .frames
    {
        assert!(
            matches!(frame.items.as_slice(), [FrameItem::Marker { vocabulary, member }]
            if vocabulary == "Preposition" && ["In", "Out"].contains(&member.as_str()))
        );
    }
}

#[test]
fn noun_phase_and_existing_verb_controls_keep_their_readings() {
    let without_phase = Lexicon::new(
        common::lexicon()
            .lexemes()
            .values()
            .filter(|lexeme| lexeme.id.as_str() != "core-verb:Phase")
            .cloned(),
    )
    .unwrap();
    for (text, category) in [
        ("your main phase", Category::NounPhrase),
        ("combat phase", Category::Nominal),
        ("your main phases", Category::NounPhrase),
        ("This creature rounds up.", Category::Sentence),
        ("This creature rounds down.", Category::Sentence),
        ("Put that card back.", Category::Sentence),
        ("This creature can't block.", Category::Sentence),
        ("Put that card in your hand.", Category::Sentence),
    ] {
        let expected: BTreeSet<_> = common::readings_with_lexicon(&without_phase, text, category);
        assert!(!expected.is_empty(), "{text}");
        assert_eq!(common::readings(text, category), expected, "{text}");
    }
}
