use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::{
    Category, Lexeme, LexicalReading, LexicalValue, Lexicon, Source, SourceKind, SurfaceCase,
    WordForm,
};
use std::collections::BTreeSet;

constructions! {
    pub mod fixture {
        closed_word Marker: Subordinator = "old:marker";
        lexical_class Markers: Subordinator | Preposition;
        category Direct();
        category Policy();
        category Member();
        category Optional();
        category OptionalMember();
        policy Identity<x> { require x is Marker; }
        policy Membership<x> { require x in Markers; }
        category RequiredIdentity();
        construction RequiredIdentity: RequiredIdentity { form [head: lexical(Subordinator)]; require head is Marker; }
        category PolicyMember();
        construction PolicyMember: PolicyMember { form [head: lexical(Subordinator)]; use Membership(head); }
        category SchemaWord();
        schema SchemaWord { form [head: lexical_word(Marker)]; }
        instance SchemaWord: SchemaWord {  }
        category SchemaIdentity();
        schema SchemaIdentity { form [head: lexical(Subordinator)]; }
        instance SchemaIdentity: SchemaIdentity { require head is Marker; }
        category SchemaMember();
        schema SchemaMember { form [head: lexical(Subordinator)]; }
        instance SchemaMember: SchemaMember { require head in Markers; }
        category SchemaPolicyIdentity();
        schema SchemaPolicyIdentity { form [head: lexical(Subordinator)]; }
        instance SchemaPolicyIdentity: SchemaPolicyIdentity { use Identity(head); }
        category SchemaPolicyMember();
        schema SchemaPolicyMember { form [head: lexical(Subordinator)]; }
        instance SchemaPolicyMember: SchemaPolicyMember { use Membership(head); }
        category OptionalPolicyIdentity();
        construction OptionalPolicyIdentity: OptionalPolicyIdentity { form ["(", head: optional(lexical(Subordinator), " "), ")"]; use Identity(head); }
        category OptionalPolicyMember();
        construction OptionalPolicyMember: OptionalPolicyMember { form ["(", head: optional(lexical(Subordinator), " "), ")"]; use Membership(head); }
        category OptionalSchemaIdentity();
        schema OptionalSchemaIdentity { form ["(", head: optional(lexical(Subordinator), " "), ")"]; }
        instance OptionalSchemaIdentity: OptionalSchemaIdentity { require head is Marker; }
        category OptionalSchemaMember();
        schema OptionalSchemaMember { form ["(", head: optional(lexical(Subordinator), " "), ")"]; }
        instance OptionalSchemaMember: OptionalSchemaMember { require head in Markers; }
        category OptionalSchemaPolicyIdentity();
        schema OptionalSchemaPolicyIdentity { form ["(", head: optional(lexical(Subordinator), " "), ")"]; }
        instance OptionalSchemaPolicyIdentity: OptionalSchemaPolicyIdentity { use Identity(head); }
        category OptionalSchemaPolicyMember();
        schema OptionalSchemaPolicyMember { form ["(", head: optional(lexical(Subordinator), " "), ")"]; }
        instance OptionalSchemaPolicyMember: OptionalSchemaPolicyMember { use Membership(head); }
        category FormLocal();
        construction FormLocal: FormLocal {
            form [head: lexical(Subordinator)] require head is Marker;
            form ["[", head: lexical(Subordinator), "]"] require head in Markers;
        }
        construction Direct: Direct { form [head: lexical_word(Marker)]; }
        construction Policy: Policy { form [head: lexical(Subordinator)]; use Identity(head); }
        construction Member: Member {
            form [head: lexical(Subordinator)];
            form [head: lexical(Preposition)];
            require head in Markers;
        }
        construction OptionalMemberAfter: OptionalMember {
            form ["(", head: optional(lexical(Subordinator), " "), ")"];
            require head in Markers;
        }
        construction OptionalMemberBefore: OptionalMember {
            form ["(", head: optional(lexical(Subordinator), before(" ")), ")"];
            require head in Markers;
        }
        construction Optional: Optional {
            form ["(", head: optional(lexical(Subordinator), " "), ")"];
            require head is Marker;
        }
    }
}

fn marker_lexeme() -> Lexeme {
    Lexeme::invariant(
        "old:marker",
        "to",
        Category::Subordinator,
        Source {
            kind: SourceKind::Core,
            path: "fixture.ron".into(),
            owner: "old:marker".into(),
        },
    )
}

fn lexicon() -> Lexicon {
    let mut words = vec![marker_lexeme()];
    let mut other = Lexeme::invariant(
        "plugin:other",
        "to",
        Category::Subordinator,
        Source {
            kind: SourceKind::Plugin,
            path: "synthetic".into(),
            owner: "plugin:other".into(),
        },
    );
    other.properties.classes.insert("Markers".into());
    words.push(other.clone());
    other.id = "plugin:preposition".into();
    other.category = Category::Preposition;
    words.push(other);
    Lexicon::new(words).unwrap()
}

fn readings(
    lexicon: &Lexicon,
    text: &str,
    category: fixture::Category,
) -> BTreeSet<fixture::Reading> {
    let grammar = fixture::Grammar::default();
    let forest = parse(&grammar, lexicon, &lexicon.analyze(text), &category).unwrap();
    grammar.readings(&forest).map(Result::unwrap).collect()
}

fn word(id: &str) -> fixture::Word {
    fixture::Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: WordForm::Invariant,
            features: deckmaste_lexical::FeatureBundle::default(),
            capitalization: SurfaceCase::Declared,
            variant: 0,
        }),
        frame: None,
        countability: None,
    }
}

#[test]
fn identity_checks_parse_admit_and_realize_without_accepting_homographs() {
    let lexicon = lexicon();
    for (category, expected, wrong) in [
        (
            fixture::Category::Direct,
            fixture::Reading::Direct {
                form: 0,
                head: word("old:marker"),
            },
            fixture::Reading::Direct {
                form: 0,
                head: word("plugin:other"),
            },
        ),
        (
            fixture::Category::Policy,
            fixture::Reading::Policy {
                form: 0,
                head: word("old:marker"),
            },
            fixture::Reading::Policy {
                form: 0,
                head: word("plugin:other"),
            },
        ),
    ] {
        assert_eq!(
            readings(&lexicon, "to", category),
            BTreeSet::from([expected.clone()])
        );
        assert_eq!(expected.realize(&lexicon).unwrap(), "to");
        assert!(wrong.admit(&lexicon).is_err());
        assert!(wrong.realize(&lexicon).is_err());
    }
}

#[test]
fn data_membership_intersects_categories_and_keeps_form_indices() {
    let lexicon = lexicon();
    let expected = BTreeSet::from([
        fixture::Reading::Member {
            form: 0,
            head: word("plugin:other"),
        },
        fixture::Reading::Member {
            form: 1,
            head: word("plugin:preposition"),
        },
    ]);
    assert_eq!(
        readings(&lexicon, "to", fixture::Category::Member),
        expected
    );
    for value in expected {
        assert_eq!(value.realize(&lexicon).unwrap(), "to");
    }
}

#[test]
fn optional_identity_checks_absence_and_only_the_present_declared_word() {
    let lexicon = lexicon();
    let absent = fixture::Reading::Optional {
        form: 0,
        head: None,
    };
    let present = fixture::Reading::Optional {
        form: 0,
        head: Some(word("old:marker")),
    };
    assert_eq!(
        readings(&lexicon, "()", fixture::Category::Optional),
        BTreeSet::from([absent.clone()])
    );
    assert_eq!(
        readings(&lexicon, "(to )", fixture::Category::Optional),
        BTreeSet::from([present.clone()])
    );
    assert_eq!(absent.realize(&lexicon).unwrap(), "()");
    assert_eq!(present.realize(&lexicon).unwrap(), "(to )");
    let homograph = fixture::Reading::Optional {
        form: 0,
        head: Some(word("plugin:other")),
    };
    assert!(homograph.admit(&lexicon).is_err());
    assert!(homograph.realize(&lexicon).is_err());
}

#[test]
fn optional_class_checks_independent_readings_with_either_separator_position() {
    let lexicon = lexicon();
    for (before, text) in [(false, "(to )"), (true, "( to)")] {
        let build = |head| {
            if before {
                fixture::Reading::OptionalMemberBefore { form: 0, head }
            } else {
                fixture::Reading::OptionalMemberAfter { form: 0, head }
            }
        };
        let present = build(Some(word("plugin:other")));
        assert_eq!(
            readings(&lexicon, text, fixture::Category::OptionalMember),
            BTreeSet::from([present.clone()])
        );
        assert_eq!(present.realize(&lexicon).unwrap(), text);
        let nonmember = build(Some(word("old:marker")));
        assert!(nonmember.admit(&lexicon).is_err());
        assert!(nonmember.realize(&lexicon).is_err());
        let absent = build(None);
        assert_eq!(absent.realize(&lexicon).unwrap(), "()");
    }
}

#[test]
fn word_selector_preserves_capitalization_and_surface_variants() {
    let mut words = vec![marker_lexeme()];
    words[0].forms[0].surfaces = Some(vec!["to".into(), "unto".into()]);
    let lexicon = Lexicon::new(words).unwrap();
    let mut head = word("old:marker");
    let LexicalReading::Word(value) = &mut head.value else { panic!() };
    value.variant = 1;
    value.capitalization = SurfaceCase::Initial;
    let expected = fixture::Reading::Direct { form: 0, head };
    assert_eq!(
        readings(&lexicon, "Unto", fixture::Category::Direct),
        BTreeSet::from([expected.clone()])
    );
    assert_eq!(expected.realize(&lexicon).unwrap(), "Unto");
}

#[test]
fn required_lexical_constraints_admit_independent_readings_through_every_expansion() {
    type Build = fn(fixture::Word) -> fixture::Reading;
    let lexicon = lexicon();
    let cases: &[(fixture::Category, bool, Build)] = &[
        (fixture::Category::Direct, false, |head| {
            fixture::Reading::Direct { form: 0, head }
        }),
        (fixture::Category::Policy, false, |head| {
            fixture::Reading::Policy { form: 0, head }
        }),
        (fixture::Category::Member, true, |head| {
            fixture::Reading::Member { form: 0, head }
        }),
        (fixture::Category::RequiredIdentity, false, |head| {
            fixture::Reading::RequiredIdentity { form: 0, head }
        }),
        (fixture::Category::PolicyMember, true, |head| {
            fixture::Reading::PolicyMember { form: 0, head }
        }),
        (fixture::Category::SchemaWord, false, |head| {
            fixture::Reading::SchemaWord {
                category: fixture::Category::SchemaWord,
                form: 0,
                head,
            }
        }),
        (fixture::Category::SchemaIdentity, false, |head| {
            fixture::Reading::SchemaIdentity {
                category: fixture::Category::SchemaIdentity,
                form: 0,
                head,
            }
        }),
        (fixture::Category::SchemaMember, true, |head| {
            fixture::Reading::SchemaMember {
                category: fixture::Category::SchemaMember,
                form: 0,
                head,
            }
        }),
        (fixture::Category::SchemaPolicyIdentity, false, |head| {
            fixture::Reading::SchemaPolicyIdentity {
                category: fixture::Category::SchemaPolicyIdentity,
                form: 0,
                head,
            }
        }),
        (fixture::Category::SchemaPolicyMember, true, |head| {
            fixture::Reading::SchemaPolicyMember {
                category: fixture::Category::SchemaPolicyMember,
                form: 0,
                head,
            }
        }),
    ];
    for (category, member, build) in cases {
        let (allowed, denied) = if *member {
            ("plugin:other", "old:marker")
        } else {
            ("old:marker", "plugin:other")
        };
        let expected = build(word(allowed));
        assert!(expected.admit(&lexicon).is_ok(), "{category:?}");
        assert_eq!(expected.realize(&lexicon).unwrap(), "to");
        assert!(readings(&lexicon, "to", *category).contains(&expected));
        let wrong = build(word(denied));
        assert!(wrong.admit(&lexicon).is_err(), "{category:?}");
        assert!(wrong.realize(&lexicon).is_err(), "{category:?}");
    }
}

#[test]
fn optional_lexical_constraints_apply_only_when_present_through_policies_and_instances() {
    type Build = fn(Option<fixture::Word>) -> fixture::Reading;
    let lexicon = lexicon();
    let cases: &[(fixture::Category, bool, Build)] = &[
        (fixture::Category::OptionalPolicyIdentity, false, |head| {
            fixture::Reading::OptionalPolicyIdentity { form: 0, head }
        }),
        (fixture::Category::OptionalPolicyMember, true, |head| {
            fixture::Reading::OptionalPolicyMember { form: 0, head }
        }),
        (fixture::Category::OptionalSchemaIdentity, false, |head| {
            fixture::Reading::OptionalSchemaIdentity {
                category: fixture::Category::OptionalSchemaIdentity,
                form: 0,
                head,
            }
        }),
        (fixture::Category::OptionalSchemaMember, true, |head| {
            fixture::Reading::OptionalSchemaMember {
                category: fixture::Category::OptionalSchemaMember,
                form: 0,
                head,
            }
        }),
        (
            fixture::Category::OptionalSchemaPolicyIdentity,
            false,
            |head| fixture::Reading::OptionalSchemaPolicyIdentity {
                category: fixture::Category::OptionalSchemaPolicyIdentity,
                form: 0,
                head,
            },
        ),
        (
            fixture::Category::OptionalSchemaPolicyMember,
            true,
            |head| fixture::Reading::OptionalSchemaPolicyMember {
                category: fixture::Category::OptionalSchemaPolicyMember,
                form: 0,
                head,
            },
        ),
    ];
    for (category, member, build) in cases {
        let (allowed, denied) = if *member {
            ("plugin:other", "old:marker")
        } else {
            ("old:marker", "plugin:other")
        };
        for (head, text) in [(None, "()"), (Some(word(allowed)), "(to )")] {
            let expected = build(head);
            assert!(expected.admit(&lexicon).is_ok(), "{category:?}");
            assert_eq!(expected.realize(&lexicon).unwrap(), text);
            assert_eq!(
                readings(&lexicon, text, *category),
                BTreeSet::from([expected])
            );
        }
        let wrong = build(Some(word(denied)));
        assert!(wrong.admit(&lexicon).is_err(), "{category:?}");
        assert!(wrong.realize(&lexicon).is_err(), "{category:?}");
    }
}

#[test]
fn lexical_constraints_stay_on_their_own_form() {
    let lexicon = lexicon();
    for (form, allowed, denied, text) in [
        (0, "old:marker", "plugin:other", "to"),
        (1, "plugin:other", "old:marker", "[to]"),
    ] {
        let expected = fixture::Reading::FormLocal {
            form,
            head: word(allowed),
        };
        assert!(expected.admit(&lexicon).is_ok());
        assert_eq!(expected.realize(&lexicon).unwrap(), text);
        assert_eq!(
            readings(&lexicon, text, fixture::Category::FormLocal),
            BTreeSet::from([expected])
        );
        let wrong = fixture::Reading::FormLocal {
            form,
            head: word(denied),
        };
        assert!(wrong.admit(&lexicon).is_err());
        assert!(wrong.realize(&lexicon).is_err());
    }
}
