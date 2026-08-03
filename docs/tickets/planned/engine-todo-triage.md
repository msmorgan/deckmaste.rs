---
needs: []
---
**Triage the engine's `todo!()`/`unimplemented!()` population.** 76 grep
hits in `crates/deckmaste_engine/src` (measured 2026-08-03; a handful sit
in inline test modules — regenerate and filter at claim time; 16 are the
`todo!("P0.…")` SEAM backlog already named in the tickets README's
priority-0 note). Each production site is a reachable process abort on
unsupported input — the engine's failure mode for an unmodeled card is a
crash, not a diagnostic — and there is no error type to graduate one into.
Examples: `legal.rs:89` (deontic legality rows unevaluated),
`target.rs:409` (phased-in status), `replace.rs:142` (uninterpreted
enters-replacement effect).

Inventory every site and classify:

1. **implement** — small enough to just close;
2. **fizzle** — authored-data-reachable sites convert to a graceful
   decline per `docs/decisions/invalid-authoring-fizzles.md` (log +
   no-op), the way `engine-unbound-eventobject-panics` does for its
   instance;
3. **fail-loud stays** — genuine unmodeled-mechanic seams on the curated
   corpus keep the diagnostic abort, but each message names a ticket slug.

Deliverables: the classified inventory (in this ticket or a follow-up per
class), the class-2 conversions, and one stated policy line (engine crate
doc or a decision doc) so the fail-loud posture reads as a documented
choice rather than an accident. `review-low-severity-followups` (the
decide.rs `todo!`) and `engine-unbound-eventobject-panics` own their
specific instances — don't duplicate them.
