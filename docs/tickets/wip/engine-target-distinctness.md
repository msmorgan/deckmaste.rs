---
needs: []
design: true
---
Close the P0.W7 seam in `resolve.rs` (`target_spec_filter`): a
`TargetSpec::Distinct(sibling_indices, inner)` spec — the co-target
set-distinctness constraint ([CR#115.7e], Arc Trail's "any *other* target") —
currently trips `todo!` rather than being enforced. Per the core doc
(`target_spec.rs`), distinctness is evaluated on the FINAL target set (never a
fixed-binding exclusion): at announce ([CR#601.2c]) and at the [CR#608.2b]
resolution re-check, where retargeting may have swapped members.

Adjacent gap in the same seam, fold-in candidate (design call): the
`TODO(stage-4)` above it — `Target(quantity, f)` quantity is unenforced
(callers assume exactly one target slot), so plain multi-target specs ("two
target creatures", "up to three targets") aren't announceable either. Cross-
spec `Distinct` without within-spec quantity covers only the rarer half of
multi-targeting.

Touches: announce-time target legality/choice surface (`decide.rs`
ChooseTargets flow), the [CR#608.2b] re-check, retarget validation
(`ChooseNewTargets` — keep-current strategy in `sim.rs` assumes per-slot
independence), and the legal-set enumeration the runner sees (engine
enumerates every choice explicitly).

---

## Spec (design settled 2026-07-12)

Ruling: FULL quantity fold-in (not Distinct-only) — multi-target slots become
real. Conventions applied (no new rulings): enforcement = submission-reject +
whole-set [CR#608.2b] re-check; per-slot `legal` stays advisory (the decision
already carries `spec`, a client may narrow); authoring mistakes fizzle, never
panic. Cards unblocked: Fate Transfer (Distinct, end-to-end); Arc Lightning
announces 1–3 targets (its damage split stays gated on
engine-divided-distribution-as-you-choose — division is announced at cast
[CR#601.2d] and never changes on retarget [CR#115.7f]; that ticket owns it).

### 1. Storage reshape — slot-indexed target sets

`StackEntry.targets`, `PendingStackEntry.targets` (stack.rs:87,120), and
`Frame.anaphora.targets` (stack.rs:240) become `Vec<Vec<ObjectId>>` — one
inner set per `TargetSpec` slot, singleton for quantity-one slots. Update
every reader (map verified 2026-07-12): aura/host `targets.first()`
resolve.rs:109 → slot 0's first; frame clones resolve.rs:137,224,285;
`TargetCount` resolve.rs:3150 + target.rs:402 → FLATTENED count;
`Reference::Target(n)` resolve.rs:2802 → slot n (singleton read for
quantity-one; plural slots feed the existing plural/`They` readback);
`StatePredicate::Targets` target.rs:393 → any member of any slot; fizzle zip
resolve.rs:3590; announce assert resolve.rs:3577; retarget clones step.rs:376,
step.rs:2015; `keep_current_targets` sim.rs:451; writers decide.rs:822,864.
`BecameTarget` dedup (decide.rs:803) now flattens across slots.

### 2. Announce ([CR#601.2c])

`Decision::Targets` payload becomes per-slot (`Vec<Vec<ObjectId>>`).
Validation (decide.rs:710) per slot: (a) chosen count within the `Quantity`
bounds — eval `Count` bounds via the frame; `Range(None, hi)` = "up to", min
0; count-locking is inherent (one submission carries the counts); (b)
membership in `legal[i]`; (c) within-slot distinctness — the same target
can't be chosen twice for one instance of "target" [CR#601.2c] (the
two-target-creatures example); the same id ACROSS slots stays legal (the
artifact-land example) unless (d) a `Distinct(siblings, _)` spec makes that
slot's set disjoint from the union of the named sibling slots' final sets
[CR#115.7e]. Announce-gate satisfiability (activate.rs:388, trigger drop
trigger.rs:963): each slot must have enough legal candidates for its MINIMUM
count net of Distinct constraints — implement the smallest correct check for
authored shapes (quantity-one pairwise-Distinct = distinct-representatives
greedy) and comment its bounds.

### 3. Resolution re-check ([CR#608.2b])

`targets_still_legal` (resolve.rs:3554) goes whole-set: per-id checks as
today (existence, filter, not-forbidden), plus Distinct re-evaluated on the
final set (defense in depth — retarget validation should never let a
violation through). Counts are NOT re-checked (locked at announce
[CR#601.2c]). Partial fizzle per [CR#608.2b] (read the rule text before
implementing): the entry fizzles iff EVERY target across all slots is now
illegal; otherwise it resolves and illegal targets are excluded from
`Target(n)`/plural reads (mirror `targets_ignores_a_departed_target`).
`target_spec_filter` gets a real `Distinct` arm: peel to inner for the
FILTER (as the render-side twin fragment.rs:882 already does); the
distinctness constraint itself is enforced at the three set-level sites, not
in the filter.

### 4. Retarget ([CR#115.7e])

`ChooseNewTargets` legal/validation reshaped identically. Per-slot
current-target union (step.rs:2022) unchanged — keeping the ENTIRE current
set is always legal (final-set rule; announce-time validity carries over).
Submission validates the whole proposed set (counts unchanged, within-slot +
Distinct). `keep_current_targets` (sim.rs:445): present-entry path returns
the clone as today; the fallback (entry gone) picks per-slot minimums
greedily RESPECTING within-slot + Distinct constraints.

### 5. Authoring validation (Idris parity, runtime-side)

Idris proves `distinctOk` (sibling index < spec count), `NonZeroQ`, and
`OrderedRange` statically (Core.idr:1846-1909); Rust catches the same at
runtime as authoring mistakes → the spec is unsatisfiable → fizzle path
(uncastable / activation illegal / trigger drops), NEVER a panic: sibling
index out of range or self-referential; inverted bounds; max = 0. Idris
itself: NO changes.

### 6. Strategy / runner / TUI

Mechanical strategy (sim.rs / strategy.rs fallback): choose each slot's
minimum count, first legal candidates, greedy skip to satisfy within-slot +
Distinct. TUI (deckmaste_tui interact.rs:111-234): Targets picker allows up
to max picks per slot; cross-slot narrowing is optional client convenience —
submission is the enforcement. Update render.rs decision views and
idris_emit expectations only as the shape change forces.

### 7. Tests

- decide.rs: within-slot duplicate rejected [CR#601.2c]; same id across two
  separate specs accepted; count outside bounds rejected (0 and 4 against
  Between(1,3)); Distinct overlap rejected at announce; per-slot membership
  still enforced.
- resolve.rs: partial fizzle — one of two targets dies → resolves affecting
  survivor; all targets die → fizzles; Distinct arm of target_spec_filter
  peels (no panic).
- Retarget: Arc Trail/Redirect swap — full-set swap of a Distinct pair
  ACCEPTED (final-set rule, the [CR#115.7e] example); overlapping proposal
  rejected; keep-current always accepted.
- Authoring: out-of-range sibling index → uncastable, never panics.
- Canon end-to-end: Fate Transfer resolves moving counters between two
  distinct creatures (cards suite). Arc Lightning: announce accepts 2
  targets; resolution stops at the division seam — assert the announce half
  only, note the gate.
- sim.rs: mechanical strategy produces valid multi-slot submissions.

### 8. Gates

nightly fmt; clippy zero warnings; `cargo test -p deckmaste_engine`;
`cargo test -p deckmaste_cards`; `cargo test -p deckmaste_tui` (decision
shape change); cite check + list-noncompliant + bless/audit for new cites.
No Idris changes. No wizards regen unless a core type serialization changed
(the reshape is engine-internal; `Decision`/`PendingDecision` are engine
types — verify nothing RON-surfaced moved).
