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
