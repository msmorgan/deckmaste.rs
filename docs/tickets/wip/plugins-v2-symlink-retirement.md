---
needs: [semantics-v2-crate]
---
**Re-point every reader at `plugins_v2/builtin` and delete the compatibility
symlink.** `semantics-v2-crate` moved `plugins/builtin_v2` to
`plugins_v2/builtin` and left a tracked symlink at the old path so the readers
that had not moved kept working (`docs/decisions/semantics-v2.md` §11: "with a
symlink at the old path until english_v2 and xtask read the new one"). The
symlink is the temporary half of that move; this ticket ends it.

31 tracked `.rs` files still spell `plugins/builtin_v2`:

- 11 `deckmaste_construction_core` integration tests (`tests/builtin_v2_*.rs`),
  each passing the old path to `read_builtin_v2`;
- `deckmaste_english_v2`'s `src/environment.rs` and `src/parser/scan.rs`, plus
  12 of its integration tests;
- 5 `xtask` sites: `english_v2.rs`, `english_v2/report.rs` (three), and
  `english_v2/flavor_words.rs`'s `--default-value` argument.

Two of the 31 name the old path on purpose and stay until the symlink goes,
then lose the old spelling with it: `gate.rs`'s changed-path classifier and
its `BUILTIN_V2_NEEDLES`, and `facts.rs`'s module doc. `xtask`'s
`tests/plugins_v2_declarations.rs` has a test that reads through the symlink
specifically; retire that test with the symlink rather than re-pointing it.

Then: delete `plugins/builtin_v2`, drop the `!/plugins/builtin_v2` force-include
from `.gitignore`, and narrow `read_builtin_v2`'s root check back to a single
accepted directory name (`builtin`), with its error message and doc comment
following. Standard constraints apply.
