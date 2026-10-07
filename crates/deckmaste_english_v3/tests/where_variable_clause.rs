mod common;

use common::{assert_constituents, readings};
use deckmaste_english_v3::grammar::Category;

fn witness(text: &str, host: &str, definition: &str) {
    let values = readings(text, Category::Document);
    assert!(!values.is_empty(), "no Reading for {text:?}");
    let host_sentence = format!("{host}.");
    assert_eq!(
        values.len(),
        readings(&host_sentence, Category::Document).len()
            * readings(&format!("where {definition}"), Category::PrepositionPhrase).len(),
        "supplement must preserve exactly the host and definition Readings for {text:?}"
    );
    for value in &values {
        let mut attachments = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::Supplementation {
                    category,
                    host: anchor,
                    supplement,
                    ..
                } = node
                {
                    attachments.push((
                        *category,
                        anchor.realize(lexicon()).unwrap(),
                        supplement.realize(lexicon()).unwrap(),
                    ));
                }
            })
            .unwrap();
        assert_eq!(
            attachments,
            vec![(
                Category::Sentence,
                host.to_owned(),
                format!("where {definition}")
            )]
        );
    }
    assert_constituents(
        text,
        Category::Document,
        &[
            (Category::Clause, host),
            (Category::PrepositionPhrase, &format!("where {definition}")),
            (Category::FiniteClause, definition),
        ],
    );
    common::assert_constituent_occurrences(
        text,
        Category::Document,
        &[(
            Category::NominativePhrase,
            "X",
            text.matches('X').count() - 1,
        )],
    );
}

#[test]
fn chameleon_colossus_defines_its_adjustment() {
    witness(
        "This creature gets +X/+X until end of turn, where X is its power.",
        "This creature gets +X/+X until end of turn",
        "X is its power",
    );
}

#[test]
fn wild_beastmaster_retains_the_trigger_and_effect_clause() {
    witness(
        "Whenever this creature attacks, each other creature you control gets +X/+X until end of turn, where X is this creature's power.",
        "Whenever this creature attacks, each other creature you control gets +X/+X until end of turn",
        "X is this creature's power",
    );
}

#[test]
fn hemosymbic_mite_defines_a_targeted_adjustment() {
    witness(
        "Whenever this creature becomes tapped, another target creature you control gets +X/+X until end of turn, where X is this creature's power.",
        "Whenever this creature becomes tapped, another target creature you control gets +X/+X until end of turn",
        "X is this creature's power",
    );
}

#[test]
fn elenda_retains_the_variable_token_quantity() {
    witness(
        "When Elenda dies, create X 1/1 white Vampire creature tokens with lifelink, where X is Elenda's power.",
        "When Elenda dies, create X 1/1 white Vampire creature tokens with lifelink",
        "X is Elenda's power",
    );
}

#[test]
fn tip_the_scales_defines_a_negative_adjustment() {
    witness(
        "When you do, all creatures get -X/-X until end of turn, where X is the sacrificed creature's toughness.",
        "When you do, all creatures get -X/-X until end of turn",
        "X is the sacrificed creature's toughness",
    );
}

#[test]
fn firebending_student_supplements_the_keyword_label_once() {
    let text = "Firebending X, where X is this creature's power.";
    assert_eq!(readings(text, Category::Document).len(), 1);
    assert_constituents(
        text,
        Category::Document,
        &[
            (Category::KeywordPhrase, "Firebending X"),
            (
                Category::PrepositionPhrase,
                "where X is this creature's power",
            ),
            (Category::FiniteClause, "X is this creature's power"),
        ],
    );
}

#[test]
fn supplementary_licence_excludes_integrated_and_locative_routes() {
    assert_eq!(
        readings("where X is its power", Category::PrepositionPhrase).len(),
        1
    );
    assert!(readings("where X is its power", Category::NounPhrase).is_empty());
    for (text, category) in [
        ("where X is its power", Category::Clause),
        ("end of turn where X is its power", Category::NounPhrase),
        (
            "This creature gets +X/+X until end of turn where X is its power.",
            Category::Document,
        ),
        (
            "Where X is its power, this creature gets +X/+X until end of turn.",
            Category::Document,
        ),
        ("where X are its power", Category::PrepositionPhrase),
        ("where X be its power", Category::PrepositionPhrase),
        ("X is 2", Category::FiniteClause),
    ] {
        assert!(readings(text, category).is_empty(), "{text}");
    }
}

use common::lexicon;
use deckmaste_english_v3::grammar::{FrameValue, Reading, Word};
use deckmaste_lexical::{LexicalReading, Number, Person, SurfaceCase, WordForm};

fn word(id: &str, form: WordForm) -> Word {
    let value = lexicon()
        .values()
        .find(|value| {
            value.lexeme.as_str() == id
                && value.form == form
                && (form != WordForm::Present
                    || (value.features.number == Some(Number::Singular)
                        && value.features.person == Some(Person::Third)))
        })
        .unwrap()
        .clone();
    let lexeme = &lexicon().lexemes()[&value.lexeme];
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability: (!lexeme.properties.countability.is_empty()).then_some(true),
    }
}

fn keyword_supplement() -> Reading {
    let mut copula = word("core-verb:Be", WordForm::Present);
    copula.frame = Some(0);
    let mut keyword = word("lexeme:keyword_ability/firebending", WordForm::Invariant);
    let LexicalReading::Word(value) = &mut keyword.value else { unreachable!() };
    value.capitalization = SurfaceCase::Initial;
    Reading::Supplementation {
        category: Category::Ability,
        form: 0,
        host: Box::new(Reading::AmountKeyword {
            form: 0,
            head: keyword,
            amount: Box::new(Reading::ScalarVariable {
                category: Category::MeasurePhrase,
                form: 0,
                head: word("vocab:Variable/X", WordForm::Invariant),
            }),
        }),
        supplement: Box::new(Reading::ClauseComplementPreposition {
            form: 0,
            head: word("vocab:Preposition/Where", WordForm::Invariant),
            complement: Box::new(Reading::FiniteClause {
                form: 0,
                subject: Box::new(Reading::ScalarVariable {
                    category: Category::NominativePhrase,
                    form: 0,
                    head: word("vocab:Variable/X", WordForm::Invariant),
                }),
                predicate: Box::new(Reading::SelectedPredicate {
                    category: Category::FinitePredicate,
                    form: 0,
                    head: copula,
                    complements: vec![FrameValue::Argument(Box::new(Reading::NominalComplement {
                        form: 0,
                        phrase: Box::new(Reading::CasePhrase {
                            category: Category::AccusativePhrase,
                            form: 0,
                            head: Box::new(Reading::GenitiveNounPhrase {
                                form: 0,
                                possessor: Box::new(Reading::DeterminedNounPhrase {
                                    form: 0,
                                    determiner: word(
                                        "vocab:SingularDemonstrative/This",
                                        WordForm::Invariant,
                                    ),
                                    head: Box::new(Reading::Noun {
                                        form: 0,
                                        head: word("lexeme:type/creature", WordForm::Singular),
                                    }),
                                }),
                                marker: word("vocab:Genitive/Default", WordForm::Invariant),
                                head: Box::new(Reading::Noun {
                                    form: 0,
                                    head: word("lexeme:CommonNoun/Power", WordForm::Singular),
                                }),
                            }),
                        }),
                    }))],
                }),
            }),
        }),
    }
}

#[test]
fn independently_constructed_keyword_supplement_roundtrips_structurally() {
    // Firebending Student, after reminder removal.
    let value = keyword_supplement();
    value.admit(lexicon()).unwrap();
    let text = value.realize(lexicon()).unwrap();
    assert_eq!(text, "Firebending X, where X is this creature's power.");
    let parsed = readings(&text, Category::Ability);
    assert_eq!(parsed, std::collections::BTreeSet::from([value.clone()]));
    let observed = parsed.get(&value).unwrap();
    let (mut nodes, mut parsed_nodes, mut words, mut parsed_words) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    value.visit(&mut |node| nodes.push(node.clone())).unwrap();
    observed
        .visit(&mut |node| parsed_nodes.push(node.clone()))
        .unwrap();
    value
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    observed
        .visit_words(&mut |word| parsed_words.push(word.clone()))
        .unwrap();
    assert_eq!(nodes, parsed_nodes);
    assert_eq!(words, parsed_words);
}

#[test]
fn independent_supplement_rejects_an_unlicensed_preposition() {
    let mut value = keyword_supplement();
    let Reading::Supplementation { supplement, .. } = &mut value else {
        unreachable!()
    };
    let Reading::ClauseComplementPreposition { head, .. } = supplement.as_mut() else {
        unreachable!()
    };
    *head = word("vocab:Preposition/After", WordForm::Invariant);
    assert!(value.admit(lexicon()).is_err());
    assert!(value.realize(lexicon()).is_err());
}
