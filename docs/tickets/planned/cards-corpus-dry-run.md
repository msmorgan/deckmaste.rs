---
needs: [core-anaphor-surface]
---
**The corpus dry-run milestone: measure the R2 ambiguity gate at scale, then
freeze it.** The gate's real-world bite is unmeasured beyond prototype scale
and phrase-level census (≈1.07% of cards have 2+ target phrases, 0.16% combine
them with "it"); this milestone converts that unknown into numbers BEFORE the
gate's semantics are frozen and the lockfile becomes authoritative.

## The run

Elaborate ALL extracted corpus faces (~29,872): parser output where a parse
exists, synthetic reference chains where not. Report:

- (a) ambiguity-gate fire rate over encodable faces;
- (b) `Label`/`The`-fallback rate (how often authors would need the escape
  hatch);
- (c) mis-binding candidates — a hand audit of 200 sampled resolutions
  (via `cargo xtask elaborate --dump`), checking each bound antecedent against
  the oracle sentence's intent.

Fight-family cards ("target creature you control fights target creature you
don't control" — two same-sort targets) are EXPECTED `Label` users: seed them
as fixtures, do not count them as gate failures.

## Acceptance bar (a binding ruling — not renegotiable in-ticket)

- Gate fires on **≤ 3%** of encodable faces.
- **Zero mis-bindings** in the 200-resolution hand audit.
- If the bar is exceeded, the ONLY pre-approved loosening is **exact-sort
  precedence**: a nearer exact-sort match beats a farther wildcard-compatible
  antecedent without erroring. Any further loosening is a design pause with
  the user, not a judgment call.

## Freeze

Once the bar is met (with or without the pre-approved loosening):

- The R2 rule text and the compat/precedence tables are frozen; the adopted
  variant is reflected in the emitted tables so Idris v2
  ([[idris-core-v2]]) mirrors the calibrated rule, not the draft one.
- `cargo xtask elaborate --lock` blessed over the hand-authored plugins;
  from here on, lockfile churn is a reviewed event.
- Record the three measured numbers and the loosening decision in this
  ticket's completion notes — they are the calibration record of the gate.

## Done

- Dry-run harness (an xtask entry point or test-binary) runs over the full
  extracted corpus and prints the three metrics.
- Metrics meet the bar; audit sample archived (card name → resolution table)
  under `crates/deckmaste_cards/tests/`.
- Gate + tables frozen; `cards.elab.lock` blessed.

## Verification

- The dry-run command reruns deterministically (two runs, identical metrics).
- `cargo test -p deckmaste_cards` green (audit fixtures pinned).
- `cargo xtask elaborate --lock` idempotent after blessing.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
