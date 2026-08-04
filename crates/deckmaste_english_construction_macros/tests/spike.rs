//! Milestone-0 spike: a function-like macro must be able to emit a sealed
//! type (private fields in a generated child module) plus validated
//! construction, with the type usable at the call site via re-export.

use deckmaste_english_construction_macros::spike_sealed;

spike_sealed!(basic);

#[test]
fn try_new_accepts_valid_members() {
    let sealed = SpikeCoordination::try_new(2).expect("2 members is valid");
    assert_eq!(sealed.members(), 2);
}

#[test]
fn try_new_rejects_zero_members() {
    assert_eq!(SpikeCoordination::try_new(0), Err(SpikeError));
}

#[test]
fn deserialize_routes_through_the_validator() {
    let ok: SpikeCoordination = ron::from_str("(members: 3)").expect("valid payload");
    assert_eq!(ok.members(), 3);

    let err = ron::from_str::<SpikeCoordination>("(members: 0)")
        .expect_err("zero members must be rejected by the same validator as try_new");
    assert!(
        err.to_string().contains("members must be at least 1"),
        "error should carry the validator's message, got: {err}"
    );
}
