---
needs: [core-anaphor-surface, cards-corpus-dry-run]
---
**The render-back fidelity gate (strong form) + the `Target(n)` sunset.** The
tree must stay tethered to the text: the check no type system performs, and
the dominant real-world failure mode of hand encodings (adjunct drift —
encodings that silently drop what the oracle sentence prints).

## `cargo xtask fidelity` (strong form)

- Renders every canon card to templated English and diffs against oracle text
  (`data/mtgjson` snapshot).
- The gate REQUIRES every printed adjunct: enter riders ("tapped", "under its
  owner's control"), "another", "an opponent controls", `where_x` ("where X
  is …") clauses. A missing adjunct is a gate failure, not a warning.
- Intended divergence needs an inline `waiver:` annotation on the card (with a
  reason string); waivers are enumerable (`cargo xtask fidelity --waivers`).
- Green-on-canon becomes a standing CI requirement from this ticket forward
  (each later grammar/macro ticket keeps it green).

## `Target(n)` sunset (a binding ruling)

`Target(n)`/`Targets(n)` were kept legal-but-deprecated when the anaphor
surface landed ([[core-anaphor-surface]]). Now, with the parsers regenerated
to emit anaphors and the R2 gate frozen ([[cards-corpus-dry-run]]):

1. One-time MECHANICAL rewrite of canon (and any other hand-authored plugin):
   `Target(n)` → `It`/`That(sort)`/`They`, or `Label` + `The` where the R2
   gate requires an explicit name. No semantic edits — the elaborated IR of
   every rewritten card must hash identically (`cards.elab.lock` unchanged
   except spellings pinned via `cargo xtask elaborate --dump` review).
2. Verified by the fidelity gate: rendered English identical before/after.
3. Then REMOVE the `Target(n)`/`Targets(n)` spellings from the accepted
   grammar — one spelling in the wild. `Label`/`The` remain the permanent
   explicit-fallback vocabulary for genuinely ambiguous prose.

## Done

- `cargo xtask fidelity` implemented, wired into CI, green on canon with an
  empty (or reviewed) waiver list.
- Canon rewrite landed; `Target`/`Targets` reference variants deleted from
  `deckmaste_core`; a reject fixture pins that the old spelling no longer
  loads.

## Verification

- `cargo xtask fidelity` — zero unwaivered diffs on canon.
- `cargo xtask elaborate --lock` — no resolution drift from the rewrite
  (lock diff empty or reviewed-and-blessed).
- `rg 'Target\(' plugins/builtin plugins/canon plugins/testing plugins/demo`
  — empty.
- `cargo test --workspace`; `cargo xtask cite check` — 0 stale,
  `--list-noncompliant` empty.
