---
needs: []
---
**Lowering: make the refusal channel a `Result`, not an unwind.** Law 12 of
[Core is explicit regions](../../decisions/core-explicit-regions.md) makes
lowering the compiler and R1/R2 refusals per-card `Diagnostic`s. The value is
already the right shape; the transport is not.

Today `crates/deckmaste_lowering/src/lib.rs:15` documents `Lower::lower` as
"the total map", so a context-sensitive refusal has nowhere to go and travels
as a panic: `region::refuse` (`region.rs:151`) is a `panic!`, and `lower_card`
(`lib.rs:161`) takes the PROCESS-WIDE panic hook, replaces it with a silencer
(`lib.rs:166-167`), runs the walk under `catch_unwind` (`lib.rs:168`), restores
the hook (`lib.rs:184`) and downcasts the payload back into a string
(`lib.rs:185-190`, `panic_message` at `lib.rs:193`). `region.rs` catches five
more times (`:127,160,174,233,261`) to keep its scope stack balanced across the
same unwinds.

The hook swap is the sharp end: it is global process state, so a corpus
compile racing anything else in the same process silences that thread's panics
too, and a genuine bug inside lowering is reported as a lowering diagnostic
against whichever card was being walked.

Scope:

- One explicit compiler context threaded through lowering. `lower_card` returns
  `Result<deckmaste_card::Card, Diagnostic>` as it does now; the refusal
  reaches it as a value.
- Per-node recursive lowering stays internal — this does not turn `Lower` into
  a fallible trait for every arm. Pin the shape at pickup and record it: the
  refusal channel lives on the context, and only the walk entry points that can
  observe it change signature.
- No `set_hook`, no `take_hook`, no `catch_unwind` anywhere in the crate. The
  scope-stack unwinding `region.rs` currently relies on becomes ordinary early
  return.
- `Diagnostic` keeps what it carries today: the card name and the R1/R2
  antecedent detail that collided.

Out of scope: the R1/R2 resolution rules themselves (law 12's discourse-tier
semantics are unchanged), and the engine's runtime abort surface, which is
[[foundations-engine-panic-closure]].

Acceptance:

- `grep -rn 'set_hook\|take_hook\|catch_unwind' crates/deckmaste_lowering/`
  prints nothing.
- `cargo test -p deckmaste_lowering` green, with the refusal tests asserting a
  returned `Diagnostic` rather than a caught unwind.
- `cargo xtask idris-check <plugin> --differential` reports the same verdict
  per card as before the change — the resolver/certifier pairing law 12 names
  is the gate that a refusal did not quietly become a success.

Standard constraints apply. Effort: **M**.
