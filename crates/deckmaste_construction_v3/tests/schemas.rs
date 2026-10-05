use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::{
    Category as LexicalCategory, Countability, Lexeme, LexicalReading, Lexicon, Source, SourceKind,
    WordForm,
};
use std::collections::BTreeSet;

constructions! {
    pub mod grammar {
        feature Eligible { Yes, No }
        category Nominal(number, Eligible);
        category OtherNominal(number, Eligible);
        category Adjective(Eligible);
        policy SameNumber {
            agree left.number = right.number;
            export number = left.number;
        }
        policy SingularSameNumber {
            agree left.number = right.number;
            require left.number = Singular;
            export number = left.number;
        }
        construction Noun: Nominal {
            form [head: lexical(Noun)];
            require head.countability = Count;
            export number = head.number;
            export Eligible = Yes;
        }
        construction OtherNoun: OtherNominal {
            form [head: lexical(Noun)];
            require head.countability = Count;
            export number = head.number;
            export Eligible = Yes;
        }
        construction Adjective: Adjective { form [head: lexical(Adjective)]; export Eligible = Yes; }
        schema Coordination {
            form [left: node, " and ", right: node];
            require left.Eligible = Yes;
            require right.Eligible = Yes;
            export Eligible = Yes;
        }
        instance Coordination<Result, Member = Self, Agreement = SameNumber>: [
            (Nominal),
            (OtherNominal, Self, SingularSameNumber),
        ] {
            bind left, right = Member;
            use Agreement;
        }
        instance Coordination: Adjective {
            bind left = Adjective;
            bind right = Adjective;
        }
    }
}

fn source(owner: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "schema-test".into(),
        owner: owner.into(),
    }
}
fn lexicon() -> Lexicon {
    Lexicon::new(vec![
        Lexeme::noun(
            "creature",
            "creature",
            vec![Countability::Count],
            source("creature"),
        ),
        Lexeme::noun(
            "artifact",
            "artifact",
            vec![Countability::Count],
            source("artifact"),
        ),
        Lexeme::invariant("red", "red", LexicalCategory::Adjective, source("red")),
        Lexeme::invariant(
            "green",
            "green",
            LexicalCategory::Adjective,
            source("green"),
        ),
    ])
    .unwrap()
}
fn leaf(lexicon: &Lexicon, id: &str, form: WordForm) -> grammar::Reading {
    let head = grammar::Word {
        value: LexicalReading::Word(
            lexicon
                .values()
                .find(|value| value.lexeme == id && value.form == form)
                .unwrap()
                .clone(),
        ),
        frame: None,
        countability: (form != WordForm::Invariant).then_some(true),
    };
    if form == WordForm::Invariant {
        grammar::Reading::Adjective { form: 0, head }
    } else {
        grammar::Reading::Noun { form: 0, head }
    }
}
fn readings(
    lexicon: &Lexicon,
    text: &str,
    category: grammar::Category,
) -> BTreeSet<grammar::Reading> {
    let grammar = grammar::Grammar::default();
    let input = lexicon.analyze(text);
    let forest = parse(&grammar, lexicon, &input, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|reading| {
            let reading = reading.unwrap();
            assert_eq!(reading.realize(lexicon).unwrap(), text);
            reading
        })
        .collect()
}

#[test]
fn shared_schema_roundtrips_independent_category_instances_and_lexical_traversal() {
    // Attested constituent shapes: "creature and artifact", "red and green".
    let lexicon = lexicon();
    for (category, left, right, form, text) in [
        (
            grammar::Category::Nominal,
            "creature",
            "artifact",
            WordForm::Singular,
            "creature and artifact",
        ),
        (
            grammar::Category::Adjective,
            "red",
            "green",
            WordForm::Invariant,
            "red and green",
        ),
    ] {
        let value = grammar::Reading::Coordination {
            category,
            form: 0,
            left: Box::new(leaf(&lexicon, left, form)),
            right: Box::new(leaf(&lexicon, right, form)),
        };
        assert_eq!(value.category(), category);
        assert_eq!(value.construction(), "Coordination");
        assert_eq!(value.realize(&lexicon).unwrap(), text);
        assert_eq!(
            readings(&lexicon, text, category),
            BTreeSet::from([value.clone()])
        );
        let mut owners = vec![];
        value
            .visit_words(&mut |word| {
                if let LexicalReading::Word(value) = &word.value {
                    owners.push(value.lexeme.clone());
                }
            })
            .unwrap();
        assert_eq!(owners, vec![left.to_owned(), right.to_owned()]);
        let mut constructions = vec![];
        value
            .visit(&mut |node| constructions.push(node.construction()))
            .unwrap();
        assert_eq!(constructions.len(), 3);
        assert_eq!(value.total_cost().unwrap(), 3);
    }
}
#[test]
fn instance_contract_rejects_wrong_child_categories_and_disagreed_features() {
    let lexicon = lexicon();
    let wrong_category = grammar::Reading::Coordination {
        category: grammar::Category::Adjective,
        form: 0,
        left: Box::new(leaf(&lexicon, "creature", WordForm::Singular)),
        right: Box::new(leaf(&lexicon, "artifact", WordForm::Singular)),
    };
    assert!(wrong_category.admit(&lexicon).is_err());
    assert!(
        readings(
            &lexicon,
            "creature and artifact",
            grammar::Category::Adjective
        )
        .is_empty()
    );
    let wrong_number = grammar::Reading::Coordination {
        category: grammar::Category::Nominal,
        form: 0,
        left: Box::new(leaf(&lexicon, "creature", WordForm::Singular)),
        right: Box::new(leaf(&lexicon, "artifact", WordForm::Plural)),
    };
    assert!(wrong_number.admit(&lexicon).is_err());
    assert!(
        readings(
            &lexicon,
            "creature and artifacts",
            grammar::Category::Nominal
        )
        .is_empty()
    );
    // Category participates in structural identity, not just formatting.
    let mut other = wrong_category.clone();
    if let grammar::Reading::Coordination { category, .. } = &mut other {
        *category = grammar::Category::Nominal;
    }
    assert_ne!(other, wrong_category);
    assert_eq!(
        other.admit(&lexicon).unwrap(),
        leaf(&lexicon, "creature", WordForm::Singular)
            .admit(&lexicon)
            .unwrap()
    );
}

#[test]
fn grouped_rows_retain_each_concrete_child_contract() {
    let lexicon = lexicon();
    let other_leaf = |id| {
        let grammar::Reading::Noun { head, .. } = leaf(&lexicon, id, WordForm::Singular) else {
            unreachable!()
        };
        grammar::Reading::OtherNoun { form: 0, head }
    };
    let value = grammar::Reading::Coordination {
        category: grammar::Category::OtherNominal,
        form: 0,
        left: Box::new(other_leaf("creature")),
        right: Box::new(other_leaf("artifact")),
    };
    assert_eq!(
        readings(
            &lexicon,
            "creature and artifact",
            grammar::Category::OtherNominal
        ),
        BTreeSet::from([value.clone()])
    );
    assert_eq!(value.realize(&lexicon).unwrap(), "creature and artifact");
    let mut wrong = value;
    if let grammar::Reading::Coordination { category, .. } = &mut wrong {
        *category = grammar::Category::Nominal;
    }
    assert!(wrong.admit(&lexicon).is_err());
}

#[test]
fn policy_rows_keep_explicitly_different_feature_constraints() {
    let lexicon = lexicon();
    assert_eq!(
        readings(
            &lexicon,
            "creatures and artifacts",
            grammar::Category::Nominal
        )
        .len(),
        1
    );
    assert!(
        readings(
            &lexicon,
            "creatures and artifacts",
            grammar::Category::OtherNominal
        )
        .is_empty()
    );
}
