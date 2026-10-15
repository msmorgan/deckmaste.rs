//! The emitter preserves the documented Lean/Rust name correspondence.

use deckmaste_semantics_v2::lean_emit::rust_field;
use deckmaste_semantics_v2::lean_emit::rust_variant;

#[test]
fn the_name_mapping_is_the_documented_one() {
    assert_eq!(rust_variant("hasType"), "HasType");
    assert_eq!(rust_variant("return_"), "Return");
    assert_eq!(rust_variant("self"), "Self_");
    assert_eq!(rust_field("from_"), "from");
    assert_eq!(rust_field("type"), "r#type");
    assert_eq!(rust_field("by_"), "by");
    assert_eq!(rust_field("callerWidth"), "caller_width");
}
