use std::collections::BTreeSet;

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Relation;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::SurfaceStructure;
use deckmaste_lexical::WordForm;

use crate::common::LEXICON;

fn argument(relation: Relation, category: &str) -> FrameItem {
    FrameItem::Argument(FrameSlot {
        relation,
        category: category.into(),
    })
}

fn frame(kind: &str, items: Vec<FrameItem>) -> Frame {
    Frame {
        kind: kind.into(),
        items,
    }
}

fn value(owner: &str, form: WordForm, number: Option<Number>) -> LexicalReading {
    LexicalReading::Word(LexicalValue {
        lexeme: owner.into(),
        form,
        features: FeatureBundle {
            number,
            ..Default::default()
        },
        variant: 0,
        capitalization: SurfaceCase::Declared,
    })
}

fn lexical_laws(owner: &str, text: &str, expected: LexicalReading) {
    assert_eq!(LEXICON.realize(&expected).unwrap(), text);
    let actual: BTreeSet<_> = LEXICON
        .analyze(text)
        .matches
        .into_iter()
        .filter(|matched| matched.start == 0 && matched.end == text.chars().count())
        .filter_map(|matched| match &matched.reading {
            LexicalReading::Word(word) if word.lexeme.as_str() == owner => Some(matched.reading),
            _ => None,
        })
        .collect();
    assert_eq!(actual, BTreeSet::from([expected]));
}

#[test]
fn frames_preserve_objects_selected_complements_and_existing_plugin_frames() {
    assert_eq!(
        LEXICON.lexemes()["core-verb:End"].properties.frames,
        [
            frame("Predicate", vec![]),
            frame("Predicate", vec![argument(Relation::Object, "NounPhrase")]),
        ]
    );
    assert_eq!(
        LEXICON.lexemes()["core-verb:Remove"].properties.frames,
        [
            frame(
                "Predicate",
                vec![
                    argument(Relation::Object, "NounPhrase"),
                    argument(Relation::Complement, "SourceComplement")
                ]
            ),
            frame(
                "Passive",
                vec![argument(Relation::Complement, "SourceComplement")]
            ),
        ]
    );
    let put = &LEXICON.lexemes()["core-verb:Put"].properties.frames;
    assert_eq!(put.len(), 7);
    assert_eq!(
        put[6],
        frame(
            "Predicate",
            vec![
                argument(Relation::Object, "NounPhrase"),
                FrameItem::Marker {
                    vocabulary: "Preposition".into(),
                    member: "Back".into()
                },
                argument(Relation::Complement, "LocativeComplement"),
                FrameItem::Optional(Box::new(FrameItem::Marked {
                    vocabulary: "Preposition".into(),
                    member: "In".into(),
                    slot: FrameSlot {
                        relation: Relation::Complement,
                        category: "MannerComplement".into()
                    },
                })),
            ]
        )
    );
    assert_eq!(
        LEXICON.lexemes()["lexeme:keyword_action/transform"]
            .properties
            .frames,
        [
            frame("Predicate", vec![argument(Relation::Object, "NounPhrase")]),
            frame("Predicate", vec![]),
            frame(
                "Predicate",
                vec![FrameItem::Marked {
                    vocabulary: "Preposition".into(),
                    member: "Into".into(),
                    slot: FrameSlot {
                        relation: Relation::Complement,
                        category: "NounPhrase".into()
                    },
                }]
            ),
        ]
    );
    assert_eq!(
        LEXICON.lexemes()["lexeme:keyword_action/surveil"]
            .properties
            .frames,
        [
            frame(
                "Predicate",
                vec![argument(Relation::Complement, "MeasurePhrase")]
            ),
            frame("Predicate", vec![]),
        ]
    );
}

#[test]
fn noun_declarations_have_independent_singular_and_plural_values() {
    for (owner, singular, plural, structure) in [
        (
            "lexeme:CommonNoun/Flip",
            "flip",
            "flips",
            SurfaceStructure::Word,
        ),
        (
            "lexeme:CommonNoun/LifeTotal",
            "life total",
            "life totals",
            SurfaceStructure::Multiword,
        ),
    ] {
        let noun = &LEXICON.lexemes()[owner];
        assert_eq!(noun.category, Category::Noun);
        assert_eq!(noun.properties.countability, [Countability::Count]);
        assert_eq!(noun.surface_structure, structure);
        for (text, form, number) in [
            (singular, WordForm::Singular, Number::Singular),
            (plural, WordForm::Plural, Number::Plural),
        ] {
            lexical_laws(owner, text, value(owner, form, Some(number)));
        }
    }
    assert_eq!(
        LEXICON.lexemes()["lexeme:CommonNoun/LifeTotal"]
            .properties
            .features["ScalarDenotation"],
        "Yes"
    );
}

#[test]
fn mana_ability_compound_inherits_the_ability_heads_declared_plural() {
    let owner = "lexeme:CommonNoun/ManaAbility/compound-noun";
    let compound = &LEXICON.lexemes()[owner];
    assert_eq!(compound.category, Category::Noun);
    assert_eq!(compound.properties.countability, [Countability::Count]);
    assert_eq!(compound.surface_structure, SurfaceStructure::Multiword);
    assert_eq!(
        compound.properties.features["CompoundHead"],
        "lexeme:CommonNoun/Ability"
    );
    assert_eq!(compound.source.owner, "lexeme:CommonNoun/ManaAbility");
    assert_eq!(
        compound.source.path,
        "crates/deckmaste_lexical_source/lexicon/core.ron"
    );
    for (text, form, number) in [
        ("mana ability", WordForm::Singular, Number::Singular),
        ("mana abilities", WordForm::Plural, Number::Plural),
    ] {
        lexical_laws(owner, text, value(owner, form, Some(number)));
    }
}

#[test]
fn temporal_and_intransitive_preposition_licences_preserve_the_bare_combat_boundary() {
    for (owner, feature, expected) in [
        ("lexeme:CommonNoun/Game", "NominalAdjunctClass", "Temporal"),
        (
            "vocab:Determinative/Any",
            "NominalAdjunctDependent",
            "Temporal",
        ),
    ] {
        assert_eq!(
            LEXICON.lexemes()[owner].properties.features[feature],
            expected
        );
    }
    assert_eq!(
        LEXICON.lexemes()["lexeme:turn_part/combat"]
            .properties
            .features
            .get("BarePrepositionUse"),
        None
    );
    for (owner, text) in [
        ("vocab:Preposition/Back", "back"),
        ("vocab:Preposition/There", "there"),
    ] {
        let preposition = &LEXICON.lexemes()[owner];
        assert_eq!(preposition.category, Category::Preposition);
        assert_eq!(
            preposition.properties.features["PrepositionComplement"],
            "Intransitive"
        );
        assert_eq!(preposition.properties.features["LocativeUse"], "Yes");
        assert_eq!(
            preposition.properties.features["PrepositionFunctionLicence"],
            "VerbComplement|PredicativeComplement"
        );
        lexical_laws(owner, text, value(owner, WordForm::Invariant, None));
    }
    assert!(!LEXICON.lexemes().contains_key("vocab:Adverb/Back"));
    assert_eq!(
        LEXICON.lexemes()["vocab:DummyPronoun/There"].category,
        Category::Pronoun
    );
    assert_eq!(
        LEXICON.lexemes()["vocab:DummyPronoun/There"]
            .properties
            .features["ExistentialUse"],
        "Yes"
    );
}
