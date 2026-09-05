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

## Landing record

**STOP — not complete and not eligible for integration.** The OPEN-1
too-strict acceptance ruling contradicts §A.3's exact equality test on the
required corpus witnesses. This change preserves the safe implementation and
the evidence; it does not choose which authority to weaken.

### What the safe change contains

- Root-only identity-free bucket construction followed by pairwise Move A / Move
  S verification at candidate finalization. A failed verification remains two
  candidates; there is no silent merge.
- One always-present `AdmissibleSites` entry per declared mobile role, containing
  `AttachmentSitePath`s relative to the node that owns the slot. A declared
  frame role is never emitted as a site, and paths remain within that subtree.
- The representative is the member that places each mobile at its highest
  admissible site. **OPEN-3 amendment:** when no member jointly realizes every
  highest site, packing proceeds per mobile, outermost first; already-packed
  mobiles are retained while the next mobile's representative is selected.
- Generated traversal exposes construction/role/conjunct boundaries to the
  materializer. Ordinary leaves are unchanged, while the selected construction
  path is the hoisted representative's path.
- Focused witnesses cover byte-exact rendering, leaf-count and construction-path
  traversal, nested mobiles, slot population, and the too-loose construction-
  identity direction.

### STOP proof

The R7 eleven-unit fixture gives these raw-to-packed root-candidate counts under
the exact §A.3 verifier:

| Identity | Raw | Packed |
| --- | ---: | ---: |
| Grafdigger's Cage | 2 | 1 |
| Weathered Runestone | 2 | 1 |
| Ground Seal | 2 | 1 |
| Silent Gravestone | 2 | 1 |
| Grand Abolisher | 4 | 4 |
| Mass Manipulation | 2 | 1 |
| Fury | 2 | 2 |
| Reprocess | 2 | 2 |
| Lich-Knights' Conquest | 2 | 2 |
| Malevolent Witchkite | 2 | 2 |
| Boltbender | 4 | 4 |

Six witnesses that OPEN-1 says **must pack** therefore do not pack. Fury is the
smallest concrete contradiction. Its two candidates have these declared-mobile
span/subtree multisets:

```text
A: (14..17, determiner, UnqualifiedReferenceDeterminedNominal)
   (17..18, first, NominalModifiedPluralCoordinationNominalValue)
B: (16..17, preposition, PrepositionalPhrasePrepositionalPhrase)
   (17..18, first, NominalModifiedPluralCoordinationNominalValue)
   (18..21, modifier, PostmodifiedReferenceRelationalQualifiedReference)
```

They are not equal entry-for-entry. Section A.3 consequently forbids their
merge, while OPEN-1 requires it.

Aquatic Alchemist // Bubble Up exposes the other side of the same contradiction:
the live grammar produces one raw root candidate, so collapse has no pair to
pack. The phase-1 landing routed the absent possessive-coordination arm here.
Adding that general arm experimentally produced two candidates, but their
claimed leaves differ (`36..40` including surrounding spaces versus `36..39`,
and `40..47` versus `39..47`) and their declared-mobile inventories differ (the
shared-head derivation alone has the `8..9` mobile head). Making them pack
required normalizing claim boundaries and erasing a nonmatching mobile — both
direct violations of §A.3. The experimental arm and relaxations were backed
out.

Decision wanted: amend §A.3 to define a lawful normalization/equivalence for
retyped mobile inventories and claim boundaries, or narrow OPEN-1 and route the
missing derivations to an authoritative grammar phase. This implementer cannot
choose between recorded rulings.

### PROVE / DISCLOSE / REPORT state at STOP

- Refresh: `kata refresh english-v2-scope-device-collapse` completed cleanly on
  `uslnxquu`; the focused materializer suite then passed 19/19.
- The amended affected-subset method was adopted after the coordinator's
  interruption. A 15-card witness/migration subset (16 faces) reported 15
  selected, 4 unique, 11 specificity-resolved, 0 unresolved ties, 1 parse
  failure, and 0 internal failures with 8 workers.
- The final full corpus set was **not run** after this STOP. Consequently there
  is no final coverage `--check`/`--bless` delta, no final per-unit selection
  classification, no final roundtrip artifact, and no final full ambiguity
  artifact to present as green. The coverage lock was not changed by this
  change. No citations changed.
- Pre-STOP diagnostic provenance only, measured on refreshed change IDs
  `mzlqyrqp` (parent) and `uslnxquu` (feature), both at lock `covered = 19,002`:
  the census moved from 14,943 unique / 4,059 specificity-resolved to 15,613 /
  3,389, with 0 exception-resolved, 0 unresolved ties, and 0 internal failures
  on both. Candidate counts fell in 1,208 units by 2,672 candidates. The
  specificity-to-unique movement includes the
  `PostmodifiedReferencePrepositionalQualifiedReference` /
  `ExistentialPredicateAdjunctExistentialPredicateAdjunct` pair. Selected
  construction paths changed in 772 units; they remain deliberately unclaimed
  as accepted because STOP occurred before the required identity-by-identity
  reading and classification.
- The same diagnostic enumerated **734 packed units** in
  `~/Dump/english-v2-scope-device-collapse-parent-refreshed/packed-census.txt`.
  This is above §F's predicted order of 150–250 and is provenance, not a fitted
  gate. The scratch census is deleted at handoff, so the number and the STOP
  record, rather than a new diagnostic/JSON surface, are retained here.
- Diagnostic performance for the root-only implementation was 123,025 ns/B on
  the parent and 124,290 ns/B on the feature (+1.03%, below OPEN-4's ~10%
  threshold), with 8 workers. Parent wall time was 139.878 s at host load
  5.02/6.86/7.25; feature wall time was 131.210 s at host load
  3.99/4.80/5.90. These are pre-STOP advisory measurements, not final gates.
- Refreshed declaration provenance: 397 constructions, 21 permitted licensing
  checkers, 0 forbidden licensing checkers, no selection exceptions. Newly or
  no-longer covered identities, the homograph inventory, and the
  form-literal/vocabulary overlap inventory were not restamped after STOP.
- Assurance: 0 restored, 11 re-spelled, 0 ignored, 5 added, 0 removed. The
  re-spellings adapt existing slot consumers and selection/path assertions to
  the per-role slot shape and hoisted representative; the five additions are
  the materializer's collapse, render, nesting, traversal, and identity-failure
  witnesses.

### Deviations and additions

- The per-role slot shape required generator/runtime/visitor changes outside
  `materialize.rs`, as the ticket explicitly allowed before any consumer read
  the phase-1 slot. Public traversal hook types were re-exported only so emitted
  visitors can name them.
- No construction, AST category, selection exception, word-naming guard,
  census gate, or diagnostic/JSON surface was added. The temporary census
  reporter and the experimental possessive-coordination arm were removed.
- The final full gate set and complete changed-selection review are omitted
  because the recorded ruling contradiction is itself the mandatory STOP.

Glossary gaps remain: Scope, Attachment, Head, Premodifier, Peripheral,
Bracketing, and Mobility.
