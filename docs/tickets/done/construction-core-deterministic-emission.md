Make grammar emission byte-reproducible (terminal-kind landing review M2/M3).
`emit/build.rs:370` iterates `role_following_onsets: HashMap<…>`, so the
onset-unification fold order follows hash order and two runs of the same
binary on the same tree differ by ~250 hunks in `cargo xtask english_v2
expand`. Every "generated output byte-identical" acceptance clause has
therefore been unverifiable, and `declaration_verb_expansion_is_
deterministic` (single process, one-role fixture) gives false assurance.
Replace hash-ordered iteration in emission with deterministic order
(BTreeMap or explicit sort by stable key) everywhere generated text
depends on it; add a cross-process determinism test (expand twice in
separate processes, compare bytes) over the production grammar. Also
close the remaining terminal-kind wildcard: `semantic.rs` (~:1865)
`terminal_has_feature` has `_ => false` over `TerminalPlan` — make it
exhaustive so a new declaration kind carrying features is a compile
error, not silently featureless. Standard constraints apply.

## Landing record (coordinator, from deterministic-emission-landing-review.md, 2026-09-02)

Executor wrote none. Two HashMap->BTreeMap fixes: emit/build.rs
role_following_onsets (named) and emit/ast.rs zeroable_types (found by
the executor, unnamed in the ticket). Reviewer enumerated every hash
container in emit/*: no hash-ordered emission remains. Cross-process
determinism test verified genuine (pre-image fails 6/6, landed passes
4/4; expand output now 7,504,179 bytes, identical md5 every run).
terminal_has_feature exhaustive. Undisclosed: deleted
declaration_verb_expansion_is_deterministic (360 -> 359 tests) — the
crate now has no in-process determinism pin. Loose ends: assert_eq on
7.5MB Vec<u8> dumps 64MB on failure (assert on md5 + first differing
offset instead); add a comment that separate processes are required
(per-process RandomState); semantic.rs:1612/1620 name an arbitrary
HashMap entry in an internal error (nondeterministic diagnostic text).
