//! The sealed fixture families, end to end through constructions!.
//!
//! `fixture_coordination` is the milestone's own-mode behavioral gate: the
//! same declaration that produced the committed golden
//! (`fixture_coordination_group()` in `deckmaste_construction_compiler`'s
//! `tests/golden_real.rs`, parsed from byte-identical DSL text in
//! `parse.rs`'s `fixture_dsl()`), compiled through the real `constructions!`
//! proc macro and exercised at runtime — not just checked as emitted source
//! text. `bind_probe` is the bind-mode sibling (CF-7): it exercises the
//! generated `build_*`/`parts_*` doors and the shared `DeclarationViolation`
//! type in the same way.
//!
//! This also closes the last open deletion-mapping row from Task 12: the
//! Milestone-0 spike's `ron` deserialize round-trip
//! (`spike.rs`'s `deserialize_routes_through_the_validator`-shaped test) was
//! retired with its successor explicitly deferred here.
//! `deserialize_routes_through_the_validator` below is that successor: a
//! round trip PLUS a genuine rejection through the real validating
//! `Deserialize` impl, not just a parse-and-discard.

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct FixturePhrase;

use deckmaste_features::Comma;
use deckmaste_features::Conjunction;

deckmaste_constructions_macro::constructions! {
    group fixture_coordination;

    element fixture_member {
        comma: opt lex Comma,
        phrase: hole FixturePhrase,
    }

    construction fixture_pair: FixturePair {
        own FixturePairNode {
            members: seq fixture_member,
            conjunction: lex Conjunction,
        }
        require members.len() >= 2;
        require conjunction in [And, Or];
        witness oxford = stored members.last.comma;
        form plain @ 0 when conjunction in [And] = members lex(conjunction);
        form fancy @ 1 when conjunction in [Or] = members "," lex(conjunction);
        dominates fixture_solo;
    }

    construction fixture_solo: FixturePair {
        own FixtureSoloNode {
            phrase: hole FixturePhrase,
            alt: opt hole FixturePhrase,
        }
        require alt.is_none();
        witness gap = free Comma;
        form only @ 0 = phrase alt;
        selection unique;
        deserialize;
    }

    construction fixture_tagged: FixturePair {
        own FixtureTaggedNode {
            phrase: hole FixturePhrase,
            tag: opt lex Conjunction,
        }
        require tag in [And, Or];
        form only @ 0 = phrase lex(tag);
    }
}

#[test]
fn try_new_enforces_every_require() {
    let member = || FixtureMember {
        comma: None,
        phrase: FixturePhrase,
    };
    let pair = FixturePairNode::try_new(vec![member(), member()], Conjunction::And)
        .expect("two members + and is admitted");
    assert_eq!(pair.conjunction(), &Conjunction::And);
    assert_eq!(pair.members().len(), 2);

    let short = FixturePairNode::try_new(vec![member()], Conjunction::And)
        .expect_err("one member violates the length require");
    assert_eq!(short.construction, "fixture_pair");
    assert_eq!(short.requirement, "members.len() >= 2");

    let bad_word = FixturePairNode::try_new(vec![member(), member()], Conjunction::Then)
        .expect_err("Then is outside the admitted set");
    assert_eq!(bad_word.requirement, "conjunction in [And, Or]");

    // Conjunction::Or is admitted too — the `In` check is a genuine set
    // membership test, not an accidental match on `And` alone.
    let or_pair = FixturePairNode::try_new(vec![member(), member()], Conjunction::Or)
        .expect("Or is also admitted by `conjunction in [And, Or]`");
    assert_eq!(or_pair.conjunction(), &Conjunction::Or);
}

#[test]
fn optional_in_is_vacuously_true_when_absent() {
    let absent = FixtureTaggedNode::try_new(FixturePhrase, None)
        .expect("reading B: an absent optional satisfies `in [...]` vacuously");
    assert_eq!(absent.tag(), &None);
    let admitted = FixtureTaggedNode::try_new(FixturePhrase, Some(Conjunction::Or))
        .expect("Or is in the admitted set");
    assert_eq!(admitted.tag(), &Some(Conjunction::Or));
    let rejected = FixtureTaggedNode::try_new(FixturePhrase, Some(Conjunction::Then))
        .expect_err("Then is present and outside the set");
    assert_eq!(rejected.requirement, "tag in [And, Or]");
}

#[test]
fn deserialize_routes_through_the_validator() {
    // Round trip: a valid payload deserializes through the real, generated
    // `Deserialize` impl (in-body `Raw` mirror -> `try_new`), not a bypassed
    // `Self { .. }` literal.
    let ok: FixtureSoloNode = ron::from_str("(phrase: (), alt: None)").expect("valid payload");
    assert_eq!(ok.phrase(), &FixturePhrase);
    assert_eq!(ok.alt(), &None);

    // Rejection: a structurally well-formed payload that violates the
    // construction's `require alt.is_none()` is rejected by `try_new`
    // through the deserializer, with the violation surfacing in the error.
    let err = ron::from_str::<FixtureSoloNode>("(phrase: (), alt: Some(()))")
        .expect_err("alt must be none");
    let message = err.to_string();
    assert!(
        message.contains("fixture_solo"),
        "error names the construction: {message}"
    );
    assert!(
        message.contains("alt.is_none()"),
        "error names the requirement: {message}"
    );
}

#[test]
fn declaration_data_traces_to_the_one_declaration() {
    use deckmaste_construction_compiler::runtime::AtomData;
    use deckmaste_construction_compiler::runtime::FieldKindData;
    use deckmaste_construction_compiler::runtime::WitnessClassData;
    let data = &FIXTURE_COORDINATION_DECLARATION;
    assert_eq!(data.name, "fixture_coordination");
    assert_eq!(data.elements, &["fixture_member"]);
    let ids: Vec<&str> = data.constructions.iter().map(|c| c.id).collect();
    assert_eq!(ids, vec!["fixture_pair", "fixture_solo", "fixture_tagged"]);
    let pair = &data.constructions[0];
    assert_eq!(pair.own_type, Some("FixturePairNode"));
    assert_eq!(pair.dominates, &["fixture_solo"]);
    assert!(!pair.deserialize);
    let ordinals: Vec<u16> = pair.forms.iter().map(|f| f.ordinal).collect();
    assert_eq!(ordinals, vec![0, 1]);
    assert_eq!(
        pair.forms[1].atoms,
        &[
            AtomData::Hole("members"),
            AtomData::Literal(","),
            AtomData::Lexeme("conjunction"),
        ]
    );
    assert_eq!(pair.fields.len(), 2);
    assert_eq!(pair.fields[0].name, "members");
    assert_eq!(
        pair.fields[0].kind,
        FieldKindData::Sequence {
            element: "fixture_member"
        }
    );
    assert_eq!(
        pair.fields[1].kind,
        FieldKindData::Scalar {
            codec: "Conjunction"
        }
    );
    assert_eq!(pair.witnesses.len(), 1);
    assert_eq!(pair.witnesses[0].name, "oxford");
    assert_eq!(
        pair.witnesses[0].class,
        WitnessClassData::Stored {
            path: "members.last.comma"
        }
    );
    let solo = &data.constructions[1];
    assert!(solo.deserialize);
    assert!(solo.selection_unique);
    assert_eq!(
        solo.witnesses,
        &[deckmaste_construction_compiler::runtime::WitnessData {
            name: "gap",
            class: WitnessClassData::Free { ty: "Comma" },
        }]
    );
    assert_eq!(
        solo.fields[1].kind,
        FieldKindData::Optional {
            inner: &FieldKindData::Subtree {
                category: "FixturePhrase",
                boxed: false
            },
        }
    );
}

#[derive(Debug, PartialEq, Eq)]
pub struct BoundPair {
    pub left: FixturePhrase,
    pub right: Option<FixturePhrase>,
}

deckmaste_constructions_macro::constructions! {
    group bind_probe;

    construction bind_pair: FixturePair {
        bind BoundPair {
            left: hole FixturePhrase,
            right: opt hole FixturePhrase,
        }
        require right.is_some();
        form both @ 0 = left right;
    }
}

#[test]
fn bind_builder_enforces_requires_and_destructures() {
    let pair = build_bind_pair(FixturePhrase, Some(FixturePhrase))
        .expect("present right side is admitted");
    let (left, right) = parts_bind_pair(&pair);
    assert_eq!(left, &FixturePhrase);
    assert_eq!(right, &Some(FixturePhrase));

    let violation =
        build_bind_pair(FixturePhrase, None).expect_err("absent right side violates the require");
    assert_eq!(violation.construction, "bind_pair");
    assert_eq!(violation.requirement, "right.is_some()");
}
