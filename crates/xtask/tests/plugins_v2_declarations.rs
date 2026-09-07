//! The declaration file is the shared contract: `deckmaste_construction_core`
//! reads its spelling and grammar under a typed metadata, and
//! `deckmaste_semantics_v2` reads its params and body with that metadata
//! opaque. Neither crate depends on the other for the file, so nothing but
//! this test holds the two readings together.
//!
//! It fails on any file only one side accepts, and on any identity only one
//! side reports (`docs/decisions/semantics-v2.md` §11).

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_construction_core::macro_def::read_builtin_v2;
use deckmaste_semantics_v2::reader::Plugin;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root resolves")
}

fn builtin() -> PathBuf {
    workspace_root().join("plugins_v2/builtin")
}

/// Every stub declaration reads on both sides, and both sides name the same
/// declarations.
#[test]
fn both_readers_accept_every_builtin_declaration() {
    let root = builtin();
    let typed = read_builtin_v2(&root).expect("construction_core reads the builtin declarations");
    let opaque = Plugin::load(&root).expect("semantics_v2 reads the builtin declarations");

    let typed_names: BTreeSet<String> = typed
        .iter()
        .map(|declaration| declaration.identity().name().to_string())
        .collect();
    // `macros/meta/` holds the declaration meta-macros themselves, which the
    // typed reader consumes rather than reports; every other definition is a
    // declaration both sides see.
    let opaque_names: BTreeSet<String> = opaque
        .declarations
        .values()
        .filter(|declaration| {
            !declaration
                .path
                .starts_with(root.join("macros").join("meta"))
        })
        .map(|declaration| declaration.definition.name.to_string())
        .collect();

    let only_typed: Vec<&String> = typed_names.difference(&opaque_names).collect();
    let only_opaque: Vec<&String> = opaque_names.difference(&typed_names).collect();
    assert!(
        only_typed.is_empty() && only_opaque.is_empty(),
        "the two readings of plugins_v2/builtin disagree\n  \
         construction_core only: {only_typed:?}\n  semantics_v2 only: {only_opaque:?}"
    );
    assert!(
        !typed_names.is_empty(),
        "neither reader found a declaration; the tree moved or the test is looking in the \
         wrong place"
    );
}
