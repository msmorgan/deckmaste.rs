//! The sealed own-mode fixture family, end to end through constructions!.
//!
//! This is the milestone's behavioral gate: the same declaration that
//! produced the committed golden (`fixture_coordination_group()` in
//! `deckmaste_construction_compiler`'s `tests/golden_real.rs`,
//! parsed from byte-identical DSL text in `parse.rs`'s `fixture_dsl()`),
//! compiled through the real `constructions!` proc macro and exercised at
//! runtime — not just checked as emitted source text.
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
    let data = &FIXTURE_COORDINATION_DECLARATION;
    assert_eq!(data.name, "fixture_coordination");
    assert_eq!(data.elements, &["fixture_member"]);
    let ids: Vec<&str> = data.constructions.iter().map(|c| c.id).collect();
    assert_eq!(ids, vec!["fixture_pair", "fixture_solo"]);
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
    let solo = &data.constructions[1];
    assert!(solo.deserialize);
    assert!(solo.selection_unique);
}
