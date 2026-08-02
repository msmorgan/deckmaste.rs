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
  under `crates/deckmaste_plugin/tests/`.
- Gate + tables frozen; `cards.elab.lock` blessed.

## Verification

- The dry-run command reruns deterministically (two runs, identical metrics).
- `cargo test -p deckmaste_plugin` green (audit fixtures pinned).
- `cargo xtask elaborate --lock` idempotent after blessing.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.

## Completion notes — the calibration record (2026-07-03)

Measured by `cargo xtask elaborate --metrics` (the dry-run harness) over
plugins/{builtin,canon,testing,wizards}; deterministic across reruns.

- **(a) Gate-fire rate: 0 / 5785 encodable faces = 0.00%** (bar ≤ 3%).
  Denominator at measurement: builtin 10 + canon 52 + testing 16 + wizards
  5707 (every finished, parsing card/token file; wizards' 24,165 todos
  excluded). After seeding the two fight-family fixtures the frozen-state
  rerun is 0 / 5787 — still 0.00%. Zero E-BIND-AMBIGUOUS findings
  corpus-wide, so the fire-site distribution is empty. Context: 98 non-gate
  findings (91 E-BIND-EVENT — the regeneration macro's `Instead` shape; 7
  E-BIND-THAT — "sacrifice two/three X" group costs read singularly), plus
  1822 deprecated `Target(n)`/`GetTargets(n)` reads across 1533 files
  (1825 after the fixtures). The deprecated-slot count is load-bearing
  context: most multi-antecedent shapes still read explicit slots, so the
  gate's bite will be re-tested organically as cards-fidelity-target-sunset
  migrates them.
- **(b) Fallback rate: 0 reads / 0 faces = 0.00%.** No graduated face reads
  `The`/`TheGroup`, and no `Label` introduction exists outside the
  accept-fixture. (Plural/binder machinery IS live — e.g. Scry's macro reads
  `They` at a Chosen binder frame — but nobody has needed the R2 escape
  hatch yet.)
- **(c) Hand audit: 0 mis-bindings in 200 faces.** Sample = all 4
  hand-authored anaphor readers (canon: Arc Lightning, Brainstorm, Flame
  Rift, Pyroclasm) + 196 of the 227 anaphor-reading graduated wizards faces
  (deterministic md5-ordered draw, seed string in
  `crates/deckmaste_plugin/tests/r2_audit/sample.txt`). Every resolved
  binding points at the entity the oracle text names; 13 sampled faces fail
  elaboration LOUDLY (regen/group-cost findings above) rather than bind
  wrong. Resolution tables pinned by `tests/r2_audit.rs` against
  `tests/r2_audit/resolutions.txt` (re-bless = reviewed event).
- **Loosening decision: NOT adopted — the gate is FROZEN STRICT**
  (`exact_sort_precedence: false` in the emitted `sort-compat.ron`; pinned
  by `tables::tests::anaphor_tables_load`). The pre-approved exact-sort
  precedence path stays implemented behind the flag for Idris v2's mirror.
- Fight family, measured: `Label` cannot name a TARGET slot (labels name
  effect introductions), and even the single-target ETB fight trips the
  strict gate (the event object is a second exact-sort antecedent — the
  loosening would NOT have rescued it). The family's escape hatch is
  `Target(n)`; seeded as `plugins/testing/cards/Sorcery Fight Two
  Targets.ron` + `Creature ETB Fight One Target.ron`, not counted as gate
  failures. cards-fidelity-target-sunset inherits: sunsetting `Target(n)`
  needs an announce-list labeling story first.
