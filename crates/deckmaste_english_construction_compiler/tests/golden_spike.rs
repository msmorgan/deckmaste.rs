//! Golden pipeline spike: emission must pretty-print deterministically and
//! byte-stably, and the committed golden is the review surface for
//! generated code.

use deckmaste_english_construction_compiler::spike_emit;

#[test]
fn spike_emission_matches_committed_golden() {
    let formatted = spike_emit::format_emission(spike_emit::spike_sealed_tokens());
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/goldens/spike_sealed.rs");
    if std::env::var_os("UPDATE_GOLDENS").is_some() {
        std::fs::write(path, &formatted).expect("write golden");
    }
    let golden = std::fs::read_to_string(path)
        .expect("golden missing: rerun with UPDATE_GOLDENS=1 and review the diff");
    assert_eq!(
        formatted, golden,
        "regenerate with UPDATE_GOLDENS=1 and review the diff"
    );
}

#[test]
fn spike_emission_is_byte_stable() {
    let first = spike_emit::format_emission(spike_emit::spike_sealed_tokens());
    let second = spike_emit::format_emission(spike_emit::spike_sealed_tokens());
    assert_eq!(first, second);
}
