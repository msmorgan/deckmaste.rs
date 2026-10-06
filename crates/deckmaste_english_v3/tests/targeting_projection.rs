mod common;

use std::collections::BTreeSet;

use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::Numeral;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

fn word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn noun(owner: &str, number: Number) -> Reading {
    let mut head = word(owner);
    let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
    value.form = if number == Number::Singular { WordForm::Singular } else { WordForm::Plural };
    value.features.number = Some(number);
    head.countability = Some(true);
    Reading::Noun { form: 0, head }
}

fn creature(number: Number) -> Reading {
    noun("lexeme:type/creature", number)
}
fn targeted(head: Reading) -> Reading {
    Reading::TargetedNominal {
        form: 0,
        marker: word("vocab:TargetingMarker/Target"),
        head: Box::new(head),
    }
}
fn direct(head: Reading) -> Reading {
    Reading::TargetNounPhrase {
        form: 0,
        marker: word("vocab:TargetingMarker/Target"),
        head: Box::new(head),
    }
}
fn bare(head: Reading) -> Reading {
    Reading::BarePlural {
        form: 0,
        head: Box::new(head),
    }
}
fn counted(head: Reading) -> Reading {
    counted_number(head, 2)
}
fn counted_number(head: Reading, value: i32) -> Reading {
    Reading::CountedNounPhrase {
        form: 0,
        quantity: Box::new(Reading::CardinalDeterminer {
            form: 0,
            value: Box::new(Reading::Cardinal {
                form: 0,
                head: Word {
                    value: LexicalReading::Numeral {
                        value,
                        notation: Numeral::Cardinal,
                        capitalization: SurfaceCase::Declared,
                    },
                    frame: None,
                    countability: None,
                },
            }),
        }),
        head: Box::new(head),
    }
}
fn coordinate(category: Category, owner: &str, left: Reading, right: Reading) -> Reading {
    Reading::Coordination {
        category,
        form: 0,
        left: Box::new(left),
        coordinator: word(owner),
        right: Box::new(right),
    }
}
fn flying(head: Reading) -> Reading {
    Reading::PostmodifiedNominal {
        form: 0,
        head: Box::new(head),
        modifier: Box::new(Reading::KeywordComplementPreposition {
            form: 0,
            head: word("vocab:Preposition/With"),
            complement: Box::new(Reading::BareKeyword {
                form: 0,
                head: word("lexeme:keyword_ability/flying"),
            }),
        }),
    }
}

fn exact(text: &str, category: Category, expected: BTreeSet<Reading>) {
    let actual = readings(text, category);
    assert_eq!(actual, expected, "{text}");
    for value in expected {
        value.admit(lexicon()).unwrap();
        assert_eq!(value.realize(lexicon()).unwrap(), text);
        let parsed = actual.get(&value).unwrap();
        let mut nodes = vec![];
        let mut parsed_nodes = vec![];
        value.visit(&mut |node| nodes.push(node.clone())).unwrap();
        parsed
            .visit(&mut |node| parsed_nodes.push(node.clone()))
            .unwrap();
        assert_eq!(parsed_nodes, nodes);
        let mut words = vec![];
        let mut parsed_words = vec![];
        value
            .visit_words(&mut |word| words.push(word.clone()))
            .unwrap();
        parsed
            .visit_words(&mut |word| parsed_words.push(word.clone()))
            .unwrap();
        assert_eq!(parsed_words, words);
    }
}

#[test]
fn bare_target_projection_uses_the_declared_number_boundary() {
    // Blinding Beam: "Tap two target creatures." The plural constituent uses
    // the nominal modifier; the singular constituent occurs in Murder.
    exact(
        "target creature",
        Category::NounPhrase,
        BTreeSet::from([direct(creature(Number::Singular))]),
    );
    exact(
        "target creatures",
        Category::NounPhrase,
        BTreeSet::from([bare(targeted(creature(Number::Plural)))]),
    );
    exact(
        "target creatures",
        Category::Nominal,
        BTreeSet::from([targeted(creature(Number::Plural))]),
    );
    exact(
        "creatures",
        Category::NounPhrase,
        BTreeSet::from([bare(creature(Number::Plural))]),
    );
    // Identical marker, nominal, Agreement and Targeting do not by themselves
    // license a second determiner function in the plural. Neither identity is
    // normalized or quotiented after parsing.
    let duplicate = direct(creature(Number::Plural));
    let retained = bare(targeted(creature(Number::Plural)));
    let Reading::TargetNounPhrase { marker, head, .. } = &duplicate else {
        unreachable!()
    };
    let Reading::BarePlural { head: nominal, .. } = &retained else {
        unreachable!()
    };
    let Reading::TargetedNominal {
        marker: other_marker,
        head: other_head,
        ..
    } = nominal.as_ref()
    else {
        unreachable!()
    };
    assert_eq!(marker, other_marker);
    assert_eq!(head, other_head);
    assert_ne!(duplicate, retained);
    assert!(duplicate.admit(lexicon()).is_err());
    assert!(duplicate.realize(lexicon()).is_err());
    // Magnetic Theft: "Attach target Equipment to target creature."
    // The invariant singular/plural spelling retains both grammatical Numbers,
    // each through its independently constructed route.
    exact(
        "target Equipment",
        Category::NounPhrase,
        BTreeSet::from([
            direct(noun("lexeme:artifact_subtype/equipment", Number::Singular)),
            bare(targeted(noun(
                "lexeme:artifact_subtype/equipment",
                Number::Plural,
            ))),
        ]),
    );
}

#[test]
fn numerals_and_noun_premodifiers_keep_the_targeted_nominal() {
    // Blinding Beam and Sanguine Indulgence (inside "up to two ...").
    exact(
        "two target creatures",
        Category::NounPhrase,
        BTreeSet::from([counted(targeted(creature(Number::Plural)))]),
    );
    let cards = Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::NounPremodifier {
            form: 0,
            head: {
                let Reading::Noun { head, .. } = creature(Number::Singular) else {
                    unreachable!()
                };
                head
            },
        }),
        head: Box::new(noun("lexeme:CommonNoun/Card", Number::Plural)),
    };
    exact(
        "two target creature cards",
        Category::NounPhrase,
        BTreeSet::from([counted(targeted(cards))]),
    );
    // Go for the Throat: the noun premodifier stays below targeting.
    let nonartifact = Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(Reading::NounPremodifier {
            form: 0,
            head: {
                let Reading::Noun { head, .. } = noun("lexeme:type/artifact/non", Number::Singular)
                else {
                    unreachable!()
                };
                head
            },
        }),
        head: Box::new(creature(Number::Singular)),
    };
    exact(
        "target nonartifact creature",
        Category::NounPhrase,
        BTreeSet::from([direct(nonartifact)]),
    );
    assert!(readings("creature target card", Category::Nominal).is_empty());
}

#[test]
fn postmodifier_scopes_above_and_below_targeting_remain_distinct() {
    // Oran-Rief Recluse and Aerial Volley supply these constituents.
    exact(
        "target creature with flying",
        Category::NounPhrase,
        BTreeSet::from([direct(flying(creature(Number::Singular)))]),
    );
    let below = targeted(flying(creature(Number::Plural)));
    let above = flying(targeted(creature(Number::Plural)));
    assert_ne!(above, below);
    exact(
        "target creatures with flying",
        Category::NounPhrase,
        BTreeSet::from([bare(below.clone()), bare(above.clone())]),
    );
    exact(
        "two target creatures with flying",
        Category::NounPhrase,
        BTreeSet::from([counted(below), counted(above)]),
    );
}

#[test]
fn targeted_genitives_preserve_the_possessor_and_coordinated_head() {
    // Bojuka Bog and Dwarven Thaumaturgist.
    let genitive = |possessor, head| Reading::GenitiveNounPhrase {
        form: 0,
        possessor: Box::new(possessor),
        marker: word("vocab:Genitive/Default"),
        head: Box::new(head),
    };
    exact(
        "target player's graveyard",
        Category::NounPhrase,
        BTreeSet::from([genitive(
            direct(noun("lexeme:CommonNoun/Player", Number::Singular)),
            noun("lexeme:CommonNoun/Graveyard", Number::Singular),
        )]),
    );
    exact(
        "target creature's power and toughness",
        Category::NounPhrase,
        BTreeSet::from([genitive(
            direct(creature(Number::Singular)),
            coordinate(
                Category::Nominal,
                "vocab:Coordinator/And",
                noun("lexeme:CommonNoun/Power", Number::Singular),
                noun("lexeme:CommonNoun/Toughness", Number::Singular),
            ),
        )]),
    );
}

#[test]
fn coordination_keeps_shared_and_local_targeting_scopes() {
    // Naturalize: "Destroy target artifact or enchantment."
    exact(
        "target artifact or enchantment",
        Category::NounPhrase,
        BTreeSet::from([direct(coordinate(
            Category::Nominal,
            "vocab:Coordinator/Or",
            noun("lexeme:type/artifact", Number::Singular),
            noun("lexeme:type/enchantment", Number::Singular),
        ))]),
    );
    // Hull Breach: "Destroy target artifact and target enchantment."
    let artifact = noun("lexeme:type/artifact", Number::Singular);
    let enchantment = noun("lexeme:type/enchantment", Number::Singular);
    exact(
        "target artifact and target enchantment",
        Category::NounPhrase,
        BTreeSet::from([coordinate(
            Category::NounPhrase,
            "vocab:Coordinator/And",
            direct(artifact),
            direct(enchantment),
        )]),
    );
}

#[test]
fn quantifiers_retain_targeting_below_the_determiner() {
    // Reasonable Doubt supplies the singular counted constituent beneath
    // "up to"; both cardinal lexical analyses remain distinct.
    let nominal = targeted(creature(Number::Singular));
    let mut one = word("vocab:CardinalDeterminative/One");
    let LexicalReading::Word(value) = &mut one.value else { unreachable!() };
    value.features.number = Some(Number::Singular);
    exact(
        "one target creature",
        Category::NounPhrase,
        BTreeSet::from([
            counted_number(nominal.clone(), 1),
            Reading::DeterminedNounPhrase {
                form: 0,
                determiner: one,
                head: Box::new(nominal),
            },
        ]),
    );
}

#[test]
fn adjective_and_participial_premodifiers_stay_below_targeting() {
    // Surge of Righteousness: "target black or red creature ...".
    let adjective = |owner| Reading::Adjective {
        form: 0,
        head: word(owner),
    };
    let colored = Reading::PremodifiedNominal {
        form: 0,
        modifier: Box::new(coordinate(
            Category::AdjectivePhrase,
            "vocab:Coordinator/Or",
            adjective("vocab:ColorWord/Black"),
            adjective("vocab:ColorWord/Red"),
        )),
        head: Box::new(creature(Number::Singular)),
    };
    exact(
        "target black or red creature",
        Category::NounPhrase,
        BTreeSet::from([direct(colored)]),
    );
    // Kill Shot; Arrow Volley Trap's "any number of target attacking
    // creatures".
    let attacking = |number| {
        let mut head = word("core-verb:Attack");
        let LexicalReading::Word(value) = &mut head.value else { unreachable!() };
        value.form = WordForm::GerundParticiple;
        value.features.finiteness = Some(deckmaste_lexical::Finiteness::Nonfinite);
        head.frame = Some(0);
        Reading::ParticipialPremodifier {
            form: 0,
            modifier: Box::new(Reading::VerbalPremodifier { form: 0, head }),
            head: Box::new(creature(number)),
        }
    };
    exact(
        "target attacking creature",
        Category::NounPhrase,
        BTreeSet::from([direct(attacking(Number::Singular))]),
    );
    exact(
        "target attacking creatures",
        Category::NounPhrase,
        BTreeSet::from([bare(targeted(attacking(Number::Plural)))]),
    );
    assert!(readings("attacking target creature", Category::Nominal).is_empty());
}

#[test]
fn plural_coordination_retains_wide_and_first_conjunct_targeting() {
    // Kaboom!: "any number of target players or planeswalkers".
    let players = noun("lexeme:CommonNoun/Player", Number::Plural);
    let planeswalkers = noun("lexeme:type/planeswalker", Number::Plural);
    exact(
        "target players or planeswalkers",
        Category::NounPhrase,
        BTreeSet::from([
            bare(targeted(coordinate(
                Category::Nominal,
                "vocab:Coordinator/Or",
                players.clone(),
                planeswalkers.clone(),
            ))),
            coordinate(
                Category::NounPhrase,
                "vocab:Coordinator/Or",
                bare(targeted(players)),
                bare(planeswalkers),
            ),
        ]),
    );
}
