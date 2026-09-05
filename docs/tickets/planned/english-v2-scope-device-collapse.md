---
needs: [english-v2-scope-device-mobility-declarations, english-v2-role-preemption-depth]
---
# The scope device, phase 3: bucket, verify, collapse

**B7 phase 3.** The load-bearing phase. Candidates that differ only in where a
mobile Constituent attaches collapse into one packed candidate; the
representative places every mobile at its highest admissible host and each slot
lists the hosts it did not take.

Design: `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`
(gitignored), §A.3 (the equality test), §A.4 (bucket then verify), §C.3
(hoisting and the representative), §C.4 (frame-role preemption), §C.5 (nesting),
§C.6 (render invariance) and §G.2 phase 3. The Q1-Q6 rulings, the OPEN rulings
and the routed residues live on `english-v2-underspecified-adjunct-attachment`
(phase 1) as inherited context.

`english-v2-role-preemption-depth` (B6, principle (ii)) is a hard prerequisite:
packing must never see the derivations that principle eliminates (§G.2 phase 0,
§C.4). §G.2 also forbids this phase sharing a round with phase 1.

## Letter

- **Bucket** by the identity-free key — the `mobiles` multiset (each mobile's
  leaf span and subtree key) plus the span-only skeleton. Near-linear,
  over-approximating.
- **Verify** each pair within a bucket against the move relation: identical
  outside the union of the move regions, each differing mobile's two hosts in a
  dominance relation, and a declared move licensing the relocation. A bucket
  collision that fails verification degrades to two ordinary candidates — the
  failure mode is never a silent merge.
- **Representative**: the candidate placing every mobile at its highest
  admissible host; each slot lists the remaining hosts, highest to lowest, as
  `AttachmentSitePath`s relative to the node carrying the slot (§C.2, §C.5).
- Where the collapse runs: **root-only first**, per OPEN-4's ruling; move to
  per-node during materialization only if the quiet-host ns/B line moves more
  than ~10%, and record the choice with its measurement either way.

≈300-400 lines with tests in `materialize.rs` (§G.2).

## Rulings this phase discharges

- **OPEN-1** (ruled 2026-09-04): identity erasure inside the affected region,
  with **both failure directions tested** — a too-strict witness (R7's eleven
  flat-versus-nested regressions and Aquatic Alchemist // Bubble Up must pack)
  and a too-loose witness (a genuine two-Construction ambiguity must not merge).
  Tighten by span structure only, never by naming.
- **OPEN-4** (ruled 2026-09-04): root-only collapse first, measured.
- **OPEN-3, residue carried from the phase-1 landing review (2026-09-04).**
  OPEN-3 asked whether two mobiles over one span can interact so that no single
  candidate places both at their highest host, and its ruling put the witness in
  phase 1. Phase 1 cannot carry it: phase 1 computes no admissible sites, so
  "highest host" has no content there, and phase 1's witness proves only that
  two nested mobile roles coexist and that one mobile's two attachment heights
  render identical bytes. The general question is therefore **open and owned
  here**. Settle it by construction before the representative rule is written;
  if it can be made to fail, the representative rule is "pack per mobile,
  outermost first", recorded as an amendment. Never an implementer call.
- **Slot arity, carried from the phase-1 landing review (2026-09-04).** Phase 1
  emits one `admissible_sites` field per *element*. §C.3 reads per *mobile role*
  ("each mobile's slot"), and §A.2's shared-Determiner families put several
  mobile roles on one element, where one shared slot cannot say which
  Constituent a path belongs to. Decide before populating any slot; changing the
  emitted shape belongs here, before a consumer reads it.

## Acceptance witnesses (§G.5)

Packed — one candidate, site paths listed:

- `Untap all permanents you control during each other player's untap step.`
- `each creature you control with a +1/+1 counter on it`
- `Destroy all artifacts and creatures with mana value X or less.`
- `Players can't cast spells from graveyards or libraries.`
- `Cards in graveyards can't be the targets of spells or abilities.`
- `Gain control of X target creatures and/or planeswalkers.`
- `Whenever you cast your first instant or sorcery spell each turn, …`

Negative:

- `unresolved_ties == 0`, keeping its meaning; a non-scope tie is still a STOP.
- byte-exact roundtrip on every packed unit (§C.6).
- zero site paths resolve to a role a Verb Frame declares (§C.4).
- no path crosses out of the subtree it was minted in (§C.5).

Disclose plainly: packing a wrong selection does not make it right — the two
prohibition-scope inversions stop asserting the wrong reading without asserting
the right one — and some canonical paths move with no meaning changing (Doorman,
and every unit whose correct reading was already the high one). Both are
selected-analysis changes under DISCLOSE, listed per identity, neither gains nor
losses.

## Fences (§G.4)

- No `require`, `checked by`, comment or literal naming a lexeme, Construction,
  verb, noun, preposition or card. A permitted guard reads a declared feature or
  a declared role property.
- No `SELECTION_EXCEPTIONS` entry, no dominance edge between named
  Constructions, no new specificity weight or tier. The device is elimination
  plus collapse; it is never a preference.
- No new AST Category, and no field holding an alternative subtree.
- `packed_units` is never a tie's escape hatch.
- Attestation is provenance: no arm, role mobility, feature value or list is
  chosen by a witness count.
- No `run_in_background` on a gate; report positive artifacts.

Standard constraints apply. Gate scope: `cargo test --workspace`.

## Baseline

Measured on `lpvvplmyynul`, phase 1's base — re-measure at claim. Lock
`covered` 17,601; constructions 387; 32,641 units; census 13,759 unique /
3,842 specificity-resolved / 0 exception-resolved / 15,040 parse failures / 0
unresolved ties. Expect `unique` to rise and `specificity_resolved` to fall;
expected `packed_units` order 150-250 (§F) — provenance, never a target.

## Glossary gaps

Terms the design needs that `docs/contexts/oracle-english/CONTEXT.md` does not
define: Scope, Attachment, Head, Premodifier, Peripheral, Bracketing, Mobility.
Listed as gaps; none is coined into the tracked glossary by this ticket.
