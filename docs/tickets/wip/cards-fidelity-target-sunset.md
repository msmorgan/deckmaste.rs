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

## Completion notes (2026-07-03)

**The sunset's announce-list labeling story (the design decision).** The
corpus dry-run proved the fight family could not move off `Target(n)` with
the anaphors alone: two same-sort announce slots make any anaphoric read a
guess, and even the SINGLE-target ETB fight trips the frozen R2 gate because
the trigger's event object is a second exact-sort antecedent. Rather than a
carve-out keeping `Target(n)` legal for those shapes, the announce list
itself gained the labeling vocabulary: **`TargetSpec::As(label, spec)` names
an announce slot, and the body reads it back as `The(label)` (one target) /
`TheGroup(label)` (a plural slot)** — the same labeled-antecedent mechanism
`Label { as, effect }` already provides for effect introductions
([CR#608.2d]), extended to the [CR#601.2c] announce list where the
ambiguity actually arises. The frozen R2 gate is untouched by construction:
an `As`-named slot still participates in R1/R2 exactly like an unlabeled
one (the label only ADDS the explicit read, it never removes a candidate),
duplicate labels in one announce list are refused (`E-BIND-LABEL`), and the
post-sunset corpus run reproduces the dry-run's calibration exactly —
5787 encodable faces, 0 gate fires (0.00%), the same 98 non-gate findings.
So the explicit-fallback vocabulary is now uniformly `Label`/`The` for
effect introductions and `As`/`The` for announce slots: one mechanism,
never a guess, no index spelling anywhere.

**Where the labels landed.** Fight family (canon Pounce, the two testing
fixtures), Fate Transfer's two same-sort slots ("from"/"to"), every
triggered targeted body (canon Footlight Fiend / Goblin Medics, the
Modular/Soulshift/Mentor builtin macros, and every parser-emitted triggered
frame — the parser now labels the slot `As("target", …)` and reads
`The("target")`); spell/activated single-slot bodies read the plain `It`,
and Arc Lightning's plural slot reads back as `They`.

**Verification deviation.** The ticket's literal
`rg 'Target\(' plugins/{builtin,canon,testing,demo}` cannot be empty: the
regex also matches `TargetSpec::Target(quantity, filter)` slot DECLARATIONS
and the `Cant(Target(…))`/`Must(Target(…))` deed forms, which are different
grammar nodes and out of the sunset's scope. The precise check —
`rg 'Target\([0-9]|GetTargets\(|Targets\([0-9]'` over every plugin including
wizards — is empty, and a reject-suite pin
(`sunset_target_index_spellings_no_longer_parse`) holds it there.
