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
