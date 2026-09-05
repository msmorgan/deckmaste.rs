---
needs: [english-v2-scope-device-collapse]
---
# The scope device, phase 5: census and diagnostics

**B7 phase 5, the device's last phase.** Packing becomes visible: a REPORT
figure beside the resolution counts, a probe block, and a JSON field.

Design: `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`
(gitignored), §E (consumers and census) and §G.2 phase 5. The Q1-Q6 rulings, the
OPEN rulings and the routed residues live on
`english-v2-underspecified-adjunct-attachment` (phase 1) as inherited context.

## Letter

- `AmbiguitySummary` gains **`packed_units`** — the count of selected rows whose
  selected candidate carries at least one non-empty slot, emitted with its
  identities listed.
- `english_v2 probe` prints one candidate for a packed unit plus a **`packed`
  block** listing, per mobile, its site paths.
- The `ambiguity --json` row gains a **`packed_sites`** field.

≈150 lines in xtask (§G.2).

## The partition identity is untouched (Q4)

`packed_units` is REPORT provenance. It **does not enter** the partition
identity `selected == unique + specificity_resolved + exception_resolved`:
packing happens before selection, so a packed unit still resolves `Unique`,
`Specificity` or `Exception` like any other. The existing `validate()` equations
stay exactly as they are. A packed candidate is one candidate — not a tie, never
a STOP — and a tie that is not a scope tie remains a STOP.

Neither the probe block nor the JSON field is a gate.

## Re-pointed `needs:` edges (coordinator, 2026-09-04)

`english-v2-attachment-class-declared` (R1) and
`english-v2-locative-coordination-arms` (R7) carry a `needs:` edge on
`english-v2-underspecified-adjunct-attachment`, which is now only phase 1 of the
device. **Their real blocker is this ticket** — the device's last phase. Both
sit in parked workspaces, so their frontmatter was not rewritten when B7 was
split; their claimants re-point the edge to this slug when they resume, and
neither re-measures or lands before this ticket is done.

`english-v2-frame-complement-coordination` (B7a) is separately a hard
prerequisite of R1's re-measure, through principle (ii).

## Acceptance

- `packed_units` is emitted with its identities listed and does not appear in
  any partition equation.
- `unresolved_ties` stays 0 and keeps its meaning.
- The corpus does not move: this phase adds reporting, not grammar. Coverage,
  the census and the roundtrip are byte-identical to the claimed baseline.

## Fences (§G.4)

- No `require`, `checked by`, comment or literal naming a lexeme, Construction,
  verb, noun, preposition or card.
- No `SELECTION_EXCEPTIONS` entry, no dominance edge between named
  Constructions, no new specificity weight or tier.
- `packed_units` is never a tie's escape hatch.
- No `run_in_background` on a gate; report positive artifacts.

Standard constraints apply. Gate scope: `cargo test --workspace`.

## Baseline

Measured on `lpvvplmyynul`, phase 1's base — re-measure at claim (phases 3 and 4
will have moved every corpus figure). Lock `covered` 17,601; constructions 387;
32,641 units; census 13,759 unique / 3,842 specificity-resolved / 0
exception-resolved / 15,040 parse failures / 0 unresolved ties. Expected
`packed_units` order 150-250 (§F) — provenance, never a target.

## Glossary gaps

Terms the design needs that `docs/contexts/oracle-english/CONTEXT.md` does not
define: Scope, Attachment, Head, Premodifier, Peripheral, Bracketing, Mobility.
Listed as gaps; none is coined into the tracked glossary by this ticket.

## Landing record

### PROVE

Fork `pwokwpvzmoyl`; measured feature tip `outzwxoyywzk`.  With matched
`--require-resolved --workers 8` flags, deleting the new `packed_sites` member
from every ambiguity JSON row made the fork and feature reports byte-identical.
The feature-only member is reported separately: 1,384 selected units have one
or more packed sites.  Therefore every selected construction, resolution,
selection lock, and traversal count is unchanged by this display-only census.

`DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
--workers 8` reported the same 20,002 covered selections on both sides, with
the lock byte-unchanged.  Both sides reported 888,873 nonterminal and visited
constructions, 310,547 expected and visited leaves, and 0 traversal failures.
`cargo xtask english_v2 roundtrip --require-clean --workers 8` reported 20,002
clean, 0 mismatched, and 12,639 not parse accepted.  The JSON-shape unit test
pins `packed_sites` as the sole new JSON field.

### DISCLOSE

The selection census is identical on fork and feature: 20,002 selected; 16,631
unique; 3,371 specificity-resolved; 0 exception-resolved; 0 unresolved ties;
and 12,639 parse failures.  `packed_units=1,384` is a report-only count and is
not a partition term.  No identity was newly covered, retired, or reselected.
Construction count is 396, unchanged by this xtask-only landing.  Inventory
reports 23 permitted licensing checkers, 0 forbidden checkers, 2 homographs,
9 overlaps, and longest literal 11.

Deviation and additions: the grammar crate exposes only the already-selected
diagnostic trace value; collection, JSON, summary, probe, and inspect display
remain in `crates/xtask/src/english_v2/`.  There is no grammar or
`materialize.rs` change, census gate, named lexical guard, selection exception,
or construction change.  Assurance counts: 0 restored, 0 re-spelled, 0
ignored, 2 added (JSON shape and coverage packed-unit census), 0 removed.
Glossary gaps are the pre-existing ticket list; none was added or coined.

Gate-scope ruling (coordinator, 2026-09-05): this ticket's legacy
`cargo test --workspace` sentence is superseded by the tracked reverse-
dependency closure rule; the closure gate below was run.

### REPORT

`cargo xtask gate --changed --run --clippy` printed:

```text
cargo test -p deckmaste_english_v2 -p xtask
cargo clippy -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings
```

It passed: `deckmaste_english_v2` 468 passed, 0 failed, 1 ignored in 42 s;
`cargo-xtask` 13 passed, 0 failed in 0 s; determinism 1 passed in 4 s;
flavor words 1 passed in 0 s; english-v2 doc tests 2 passed in 0 s; xtask doc
tests 0 passed, 0 failed in 0 s; clippy passed in 21 s.  `cargo fmt --all`
passed.  Citations unchanged; no cite gate was needed.

Performance advisory (workers 8; concurrent-process count unavailable to the
sandbox): coverage feature run 205 s, 255,879 ns/B, host load 20/25/17;
ambiguity feature run 135 s, 182,582 ns/B, host load 13/19/17; roundtrip 146 s,
192,527 ns/B, host load 19/19/18.  The 16 s advisory ceiling was exceeded under
shared-host load and is reported, not fitted.  STOP: none remaining; the prior
gate-scope STOP was resolved by the coordinator ruling above.  No citations
changed and no decision is wanted.
