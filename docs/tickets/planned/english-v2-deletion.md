---
needs: []
---
**Delete `deckmaste_english_v2` and the references it leaves behind.** The
crate is superseded by `deckmaste_english_v3` and has already left the Cargo
workspace: the root manifest lists it under `exclude`, not `members`, so it is
no longer built, tested, or required to be green. Its `environment.rs` refuses
to load — `keyword_actions`' `participle: "tapped"`/`"untapped"` declarations
raise a `LiteralLexiconCollision` — and that is accepted, not a defect to fix.
The directory survives on disk only because two live consumers read its
sources by path (below). This ticket removes the directory and every reference
that outlives it.

**What is already done** (change `xtask: english_v2 leaves the cargo
workspace`): the crate left `members` for `exclude`; its
`[profile.dev.package]` override is gone; `xtask` dropped both dependencies,
the `parser-metrics` feature, `src/english_v2.rs` and `src/english_v2/`, the
`english_v2` subcommand, and `tests/english_v2_determinism.rs`; the
`cargo xtask english_v2 flavor-words --check` step left CI; `gate.rs`'s
metadata fixture and closures name only surviving crates and pin that a
`crates/deckmaste_english_v2/**` path now maps to no crate.

**What deletion has to do.**

1. **Break the two path readers first — they are what keeps the directory
   alive.**
   - `crates/deckmaste_lexical_source/src/legacy/core.rs:26`
     (`GRAMMAR_PATH = "crates/deckmaste_english_v2/src/constructions.rs"`) and
     `crates/deckmaste_lexical_source/src/legacy/mod.rs:15`
     (`ENGLISH_V2_SOURCE_DIR = "crates/deckmaste_english_v2/src"`) — the
     `legacy` adapter reads the v2 grammar sources at runtime. Retire the
     adapter or repoint it at the v3 grammar; decide which as part of this
     ticket.
   - `crates/deckmaste_construction/tests/support/runtime_frames.rs:22`,
     `crates/deckmaste_construction/tests/compiled_consumer.rs:2758`, and
     `crates/deckmaste_construction/tests/compile_fail/structural_checked_constructor_accessor_collision.rs:27`
     all `include!` `crates/deckmaste_english_v2/src/parser/engine.rs` as the
     compiled-consumer fixture's runtime. These are `deckmaste_construction`'s
     own gates: give them a fixture that does not live in a deleted crate
     (vendor the engine into `tests/support/`, or re-spell against the v3
     runtime), never delete them.

2. **Delete the directory** `crates/deckmaste_english_v2/` and the
   `exclude = ["crates/deckmaste_english_v2"]` entry (with its comment) from
   the root `Cargo.toml`.

3. **Retire the doc references.** Each of these names the crate, its cutover,
   or one of its commands, and is stale once the directory is gone:
   - `CLAUDE.md` — the whole **Crate fates (english_v2 rewrite)** section
     (lines 38–85), and in **Gate scope for compiler changes** lines 149–154
     and 162 (the "english_v2 lane" closure, the
     `-p deckmaste_english_v2` example commands, the with-preposition
     anecdote's `-p english_v2 -p xtask` line). Line 197 (**Corpus work
     iterates on a subset**) describes the `english_v2` corpus commands —
     `coverage --check/--bless`, `ambiguity`, `roundtrip`, `--data` — none of
     which exist any more; re-spell it against v3's corpus tooling or drop it.
   - `docs/decisions/english-v2-rewrite.md` — the cutover plan itself. Its
     `xtask english_v2 …` contracts (lines 82, 829, 905) and the whole
     cutover-language frame ("deleted at cutover", "until cutover") describe
     an event that no longer happens: v2 is deleted outright, not cut over to.
     Supersede the ADR rather than editing it piecemeal.
   - `docs/decisions/README.md:46` — the index entry's "in-place `english_v2`
     grammar migration".
   - `docs/decisions/english-lean-design-workbench.md:4,54` — "migrate
     `english_v2`'s …", "Retain `english_v2` and its declaration-owned
     generated types".
   - `docs/decisions/semantics-v2.md:249,251,272` — english_v2 as a reader of
     `plugins_v2/builtin` and as a translation source.
   - `docs/decisions/kind-index-joins-union-marking-is-spelling.md:39` —
     "english_v2's construction declarations".
   - `docs/tickets/planned/english-v3-whole-grammar-activation.md:45`,
     `docs/tickets/planned/semantics-v2-keyword-action-residues.md:25`,
     `docs/tickets/planned/semantics-v2-subject-param-kind.md:12,17,20` — live
     tickets whose acceptance still refers to english_v2 (the last one puts
     english_v2 in its gate closure).
   - `.claude/hooks/pre-cutover-guard.sh` — the guard's refusal messages tell
     the reader to "put new architecture and features in
     `deckmaste_english_v2`". Its guarded crate list stays correct; only the
     rationale is stale, and it must be re-pointed at v3 in the same pass that
     amends `CLAUDE.md`, so the two do not disagree.
   - `plugins_v2/builtin/macros/keyword_abilities/champion.ron:13` — a comment
     that attributes a `NounPhrase` reading to english_v2's parse.

   Tickets under `docs/tickets/done/` are historical records and are left
   alone.

**Acceptance:** no path outside `docs/tickets/done/` names
`deckmaste_english_v2` or `english_v2`; `cargo build --workspace
--all-targets`, `cargo test --workspace`, and `cargo clippy --workspace
--all-targets` are clean; `deckmaste_construction`'s compiled-consumer,
runtime-frames and compile-fail gates still run with the same asserted
outcomes; and `cargo xtask cite check` reports 0 stale with
`--list-noncompliant` empty.

Related: [[english-v3-whole-grammar-activation]] (the successor grammar this
deletion clears the way for).
