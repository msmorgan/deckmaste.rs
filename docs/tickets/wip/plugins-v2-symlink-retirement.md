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

## Landing record

**Inventory (step 1).** `grep -rln 'plugins/builtin_v2' crates/ .gitignore`
returned exactly the ticket's 31 tracked `.rs` files plus `.gitignore` — no
difference from the ticket's list.

**Re-pointing.** 28 files got a straight `plugins/builtin_v2` →
`plugins_v2/builtin` substitution (11 `deckmaste_construction_core`
`builtin_v2_*.rs` tests, `deckmaste_english_v2`'s `src/environment.rs` +
`src/parser/scan.rs` + 12 integration tests, and `xtask`'s `english_v2.rs`,
`english_v2/report.rs`, `english_v2/flavor_words.rs`). `gate.rs` lost the
old-path arm from `is_plugins_v2_path`/`is_builtin_v2_path` (doc comments
updated) and the old `"builtin_v2"` needle from `BUILTIN_V2_NEEDLES`
(`&["plugins_v2/builtin"]` now), keeping its one still-old-spelled test
fixture (`builtin_declaration_path_starts_from_each_reader`) re-pointed to
the new path since the classifier no longer accepts the old one. `facts.rs`'s
module doc dropped the `builtin_v2` spelling (now "the builtin
declarations"); its other two occurrences (the generated Idris header
comment text, and the `GATE_COLUMNS` doc string) were re-pointed like
ordinary occurrences — the ticket names only the module doc as the deliberate
exception. `xtask/tests/plugins_v2_declarations.rs` had its
through-the-symlink test deleted outright (see Assurance below).

**`read_builtin_v2` narrowing (step 3).** `crates/deckmaste_construction_core/src/macro_def.rs`:
the root-name check now accepts only `"builtin"`; `ValidationError::InvalidBuiltinRoot`'s
message dropped "or `builtin_v2`"; the function's doc comment dropped the
symlink clause. Its co-located unit tests
(`crates/deckmaste_construction_core/src/macro_def/tests.rs`) build their
fixture roots under a temp directory named `"builtin_v2"` purely as an
incidental convention (none of them test root-name acceptance itself — that
is `builtin_reader_authenticates_kind_category_name_and_root`'s
already-present `"something_else"` rejection case, which is unaffected); all
nine `.join("builtin_v2")` call sites were renamed to `.join("builtin")` so
they keep exercising declaration-reading behavior rather than tripping the
newly-narrowed root check. This file is not one of the ticket's named 31 but
is the direct, necessary consequence of narrowing `read_builtin_v2` in step 3.

**Symlink removal (step 4).** `rm plugins/builtin_v2` (a symlink, so this
removes the link only — `plugins_v2/builtin/` is untouched, verified via
`ls`). Dropped the `!/plugins/builtin_v2` force-include line and its comment
from `.gitignore`. `jj st` showed `D plugins/builtin_v2`.

**Regenerated artifact.** `cargo xtask facts check` first failed because
`idris/src/Experimental/FactsGen.idr` is generated with a header comment that
embeds `render()`'s `plugins/builtin_v2/macros/stubs/...` string (now
`plugins_v2/builtin/...` after the facts.rs edit above); `cargo xtask facts
generate` regenerated it (only the header line's path spelling changed;
`lean/Semantics/Check/Facts.lean` was already up to date), and `facts check`
then passed clean.

**Gate (step 5).** Derived command:
`cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`.
All suites green: `deckmaste_construction` unit+integration all `ok`;
`deckmaste_construction_core` unit 428 passed, all 11 `builtin_v2_*` integration
suites passed (2+1+2+2+1+6+4+5+1+1+5 = 30 tests); `deckmaste_english_v2` unit
159 passed plus all 12 integration suites green; `xtask` unit 492 passed, 1
ignored (pre-existing, `templates.rs:589`, unrelated to this ticket — "run on
demand" against the live corpus), `xtask` bin tests 13 passed,
`english_v2_determinism` 1, `flavor_words` 1, `lean_check` 4,
`plugins_v2_declarations` 1 passed (the retired test's sibling
`both_readers_accept_every_builtin_declaration` still passes). Zero failures
across the whole run once `FactsGen.idr` was regenerated. `cargo fmt --all`
made no changes. `cargo clippy -p deckmaste_construction_core -p
deckmaste_english_v2 -p xtask --all-targets -- -D warnings` clean. `cargo
xtask facts check` clean. `cargo xtask lean-check`: 1 plugin, 2 cards, 0.3s,
baseline OK.

**Final grep (step 6).** `grep -rn 'plugins/builtin_v2' crates/ .gitignore
Cargo.toml lean/ .github/ idris/` — empty.

**Files changed.** 36: 35 modified (the 31 ticket-listed files, plus
`macro_def/tests.rs`, plus the regenerated `idris/src/Experimental/FactsGen.idr`,
plus `.gitignore` — `.gitignore` and `macro_def.rs` are both already inside
the ticket's 31) and 1 deleted (`plugins/builtin_v2`).

**Assurance counts.** Restored: 0. Re-spelled: 0. Ignored (new): 0 (one
pre-existing ignored test, unrelated, noted above). Added: 0. Removed: 1 —
`the_old_path_still_reaches_the_same_declarations` in
`crates/xtask/tests/plugins_v2_declarations.rs`, deleted per the ticket's
explicit instruction: its subject, the compatibility symlink at the old path,
is exactly what this ticket retires, so the test has no replacement subject
to re-spell against.

**Inventory difference from step 1.** None — the grep matched the ticket's
31-file list exactly.
