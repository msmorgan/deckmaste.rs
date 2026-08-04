//! Compiles the committed golden. A golden that stopped being valid Rust
//! (or drifted from the runtime types) would otherwise still pass its
//! byte-comparison test.

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct FixturePhrase;

#[derive(Debug, PartialEq, Eq)]
pub struct BoundMember {
    pub comma: Comma,
    pub phrase: FixturePhrase,
}

pub enum BoundPayload {
    Present,
}

use deckmaste_features::Comma;
use deckmaste_features::Conjunction;

include!("goldens/fixture_coordination.rs");

#[test]
fn golden_compiles_and_seals() {
    let node = FixtureSoloNode::try_new(FixturePhrase, None).expect("valid solo");
    assert_eq!(node.phrase(), &FixturePhrase);
    let violation =
        FixtureSoloNode::try_new(FixturePhrase, Some(FixturePhrase)).expect_err("alt must be none");
    assert_eq!(violation.requirement, "alt.is_none()");
    // Suppress unused-import pedantry honestly: the golden itself uses both.
    let _ = (Comma::Absent, Conjunction::And);
}
