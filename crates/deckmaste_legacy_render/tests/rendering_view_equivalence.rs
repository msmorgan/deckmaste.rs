//! The two notions of "the card" agree on the whole corpus.
//!
//! [`Plugin::card_from_str`] is the restricted read the engine loads; the
//! rendering view ([`Plugin::rendering_card_from_str`]) re-reads the same
//! source FREE, so identity-macro provenance is absent from it. The renderer
//! grades rules text against the second and the engine runs the first, which is
//! sound only while every variant-named macro mirrors its variant faithfully —
//! a per-definition property asserted per scaffold, never over the corpus.
//!
//! This is the corpus-level assertion: expanded, the two reads produce the same
//! value for every card in every hand-authored plugin. A non-identity macro
//! shadowing a variant name would break it here rather than in a fidelity
//! report that quietly grades a card the engine does not load.
//!
//! It lives in this crate because the rendering view is confined to it
//! (`deckmaste_plugin/tests/read_api_gate.rs`) — which is also where the risk
//! is.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_plugin::layout::CARDS_DIR;
use deckmaste_plugin::layout::is_todo_source;
use deckmaste_plugin::plugin::Plugin;
use deckmaste_plugin::plugin::read;
use deckmaste_plugin::plugin::ron_files_recursive;
use macro_ron::Expand;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

const COVERED: [&str; 4] = ["builtin", "canon", "testing", "demo"];

#[test]
fn the_loaded_card_and_the_rendering_view_agree_on_every_card() {
    let mut checked = 0usize;
    let mut disagreed = Vec::new();
    for plugin_name in COVERED {
        let dir = workspace_root().join("plugins").join(plugin_name);
        let plugin = Plugin::load_with_sibling_prelude(&dir).expect("plugin loads");
        for path in ron_files_recursive(&dir.join(CARDS_DIR)).expect("cards directory walks") {
            let source = read(&path).expect("card file is readable");
            if is_todo_source(&source) {
                continue;
            }
            let loaded = plugin
                .card_from_str(&source)
                .unwrap_or_else(|e| panic!("loading {}: {e}", path.display()));
            let rendering = plugin
                .rendering_card_from_str(&source)
                .unwrap_or_else(|e| panic!("rendering-view read of {}: {e}", path.display()));
            checked += 1;
            if loaded.semantic.expand_all() != rendering.expand_all() {
                disagreed.push(path.display().to_string());
            }
        }
    }
    // Vacuity floor: a broken walk would otherwise pass by checking nothing.
    assert!(
        checked > 0,
        "no cards checked — the corpus walk is broken, not the tree clean"
    );
    assert!(
        disagreed.is_empty(),
        "the restricted read and the rendering view disagree — some variant-named \
         macro is not a faithful identity mirror:\n{}",
        disagreed.join("\n")
    );
}
