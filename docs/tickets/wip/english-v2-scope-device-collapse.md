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

## Superseded STOP record (2026-09-05)

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
  threshold), with 8 workers. Parent wall time was 139,878 ms at host load
  5.02/6.86/7.25; feature wall time was 131,210 ms at host load
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

## Superseded STOP record (second phase-3 round, 2026-09-05)

**STOP — not complete and not eligible for integration.** The mandatory
changed-selection review found a wrong selected analysis for corpus identity
`Cynical Loner`. Its `search your library for a card` clause changed from the
declared `object … for … object` frame to a transitive predicate plus a generic
prepositional Predicate Adjunct. That contradicts principle (ii): a declared
frame role is not an admissible attachment site. A focused `probe` confirmed
that the generic Adjunct analysis is selected. Restoring the earlier host-span
condition in Move A did not restore the frame analysis, so the defect lies in
the amended collapse's affected-region equality or representative selection.
Per the 2026-09-05 work order, a wrong analysis in either sample stratum is a
STOP naming the identity; no further relaxation or corpus fitting was made.

### Resolved STOP history

- The phase-2 slot-shape STOP was resolved by the 2026-09-04 amendment: the
  always-present slot is per declared mobile role, not per element. Phase 3
  consumes that shape as `AdmissibleSites` containing `AttachmentSitePath`s.
- The first phase-3 STOP proved that ordinary §A.3 could not pack Fury or
  Aquatic Alchemist // Bubble Up because Move S changes mobile inventories and
  claim boundaries. The coordinator resolved it with A.3-S1 through A.3-S6.
  This change implements anchor-bijection bucketing, peripheral difference
  regions, scope-confined identity/mobile/claim-boundary erasure, the empty-
  difference fence, and failed-verification degradation to two candidates.
- The concurrent B7a note appended A.3-S3a after the named work-order sections
  had been read. Phase 3 now also requires a peripheral difference region to be
  fully yielded by declared mobile material. Its binary-rebracketing witness is
  asserted, while all named Move S packing witnesses still pass.

### Implemented safe work

- Root-only bucket-and-verify collapse runs at candidate finalization. The
  ordinary identity-free bucket handles Move A; the leaf-sequence plus ordered
  Coordinator-anchor bucket admits Move S pairs without pre-filtering on their
  different mobile inventories.
- The selected representative hoists mobiles to the highest admissible site.
  Slots are populated per mobile role, outermost first, and Move S alternatives
  use `Conjunct(role, ordinal)` paths. Leaves and rendered bytes are unchanged.
- The general possessive Coordination arm is restored. Aquatic Alchemist //
  Bubble Up produces two raw candidates and one packed representative with two
  populated role slots.
- Positive witnesses cover Fury, Grand Abolisher, Reprocess, Lich-Knights'
  Conquest, Malevolent Witchkite, Boltbender, Aquatic Alchemist // Bubble Up,
  Grafdigger's Cage, Weathered Runestone, Ground Seal, Silent Gravestone, Mass
  Manipulation, the Seedborn class, nesting, render invariance and traversal.
  Negative witnesses cover a live neutral family swap, flat-versus-nested
  Coordination associativity, binary rebracketing without a mobile yield, and
  construction identity with the Identity specificity tier disabled.

### PROVE / DISCLOSE / REPORT state at STOP

- The coordinator interrupted the earlier run after excessive full-corpus
  development loops; all subsequent development used the 2,323-card affected
  subset. Immediately before this STOP it reported 739 selected, 548 unique,
  191 specificity-resolved, 0 unresolved ties and 0 internal failures; its
  round trip was 739/739 clean.
- A full ambiguity pass was run before the concurrent A.3-S3a section was
  noticed: 19,524 selected, 16,256 unique, 3,268 specificity-resolved, 0 ties,
  and 1,000 selected trees with populated slots. The pass was repeated because
  the newly binding tightening changed code. On that diagnostic tree it was
  19,524 selected, 16,133 unique, 3,391 specificity-resolved, 0 ties and 934
  selected trees with populated slots. Both figures are pre-STOP diagnostics,
  not acceptance measurements; the current safe backout differs.
- Against the refreshed parent (19,469 selected; 15,271 unique; 4,198
  specificity-resolved), that diagnostic tree had 55 newly selected units, 854
  specificity-to-unique movements, 942 changed selected paths, and 3,178
  candidates removed across 1,520 units (mean 2.09). The >2-drop stratum held
  233 units; the uniform remainder sample was prepared at 60 of 799. Review
  stopped at the first confirmed wrong analysis, as required, so no correctness
  rate is claimed.
- The restored possessive arm accounted for all 55 diagnostic newly selected
  units; every selected path contained the general possessed-Coordination arm.
  Its required coverage delta was not run after STOP, so breadth beyond the
  shared-head acceptance family remains unresolved and must be classified on a
  resumed run.
- Diagnostic performance after S3a was 160,942 ns/B with 8 workers, 125,785 ms
  wall time, and host load 8.82/9.56/11.62. The refreshed parent was 125,831
  ns/B with 8 workers, 120,329 ms wall time, and host load 20.35/12.00/11.21.
  The diagnostic cost was +27.9%, beyond OPEN-4's ~10% threshold, which is an
  independent STOP; no per-node implementation was substituted.
- Because the wrong selection and performance STOPs occurred during mandatory
  classification, the final coverage `--check`/`--bless`, final roundtrip,
  final workspace test, strict clippy, construction/checker inventory and
  final-tree census were not run. No coverage lock or citations were changed.
- Focused positive artifacts before STOP: materializer library 153/153 and
  nominal grammar 32/32 passed; after A.3-S3a, the named Move S and associativity
  witnesses passed. The current tree intentionally remains unclaimed as green.
- Assurance: 0 restored, 13 re-spelled, 0 ignored, 9 added, 0 removed. The added
  ninth assurance is A.3-S3a's binary-rebracketing-without-mobile-yield fence.

### Deviations and additions

- The general possessive Coordination arm and `mobile(rest)` declarations on
  the first singular/plural Modifier roles are additions explicitly ordered by
  the 2026-09-05 amendment. Their complete coverage classification is blocked
  by the current STOP.
- A temporary, non-serialized packed-unit census hook was used only during the
  ambiguity diagnostic and removed. No census gate, JSON field, selection
  exception, AST Category, word-naming guard, specificity weight, or principle
  (iv) work remains in the tree.
- The final full-set budget was exceeded by one ambiguity pass only because the
  concurrent A.3-S3a design section became visible after the first pass. No
  other full corpus gate was repeated.

Glossary gaps remain: Scope, Attachment, Head, Premodifier, Peripheral,
Bracketing, and Mobility.

## Superseded STOP record (third phase-3 round, 2026-09-05)

**STOP — not complete and not eligible for integration.** On the refreshed
third-round tree, the mandated back-to-back quiet-host comparison measured the
parent at 126,940 ns/B and the root-only collapse at 143,683 ns/B: **+13%**,
above OPEN-4's approximately 10% bar. The implementation remains root-only;
no per-node collapse or corpus fitting was substituted. Per the binding work
order, this measurement is a STOP.

### Resolved STOP history

- The first phase-3 STOP (ordinary §A.3 rejecting the required Fury and Aquatic
  Alchemist // Bubble Up packs) was resolved by A.3-S1 through A.3-S6 and
  A.3-S3a. The implementation has the second Move-S bucket, Coordinator-leaf
  bijection, peripheral declared-mobile difference region, confined erasure,
  ordered claim comparison, and the empty-difference fence.
- The second-round wrong-selection STOP for Cynical Loner was resolved by
  §C.4a. Declared custom Verb Frame roles create opaque collapse domains;
  candidate pairs built by different frames are rejected before erasure. Both
  readings of `Search your library for a card.` remain distinct, specificity
  selects the declared frame, and the B6 preemption widening remains routed to
  its follow-up ticket.
- The second-round 28% performance STOP was resolved in implementation by the
  OPEN-4 root-only sequence: mobile-inventory pre-gate, lazy render/claim
  capture, fixed structural-digest bucket keys, and lazy projection/mobile
  caches. Those changes reduced the measured cost materially, but the final
  acceptance comparison below still exceeds the amended bar.
- The earlier phase-2 slot-shape issue remains resolved by the 2026-09-04
  amendment: `AdmissibleSites` is always present per declared mobile role and
  contains ordered `AttachmentSitePath`s.

### Implemented safe work

- Candidate finalization performs root-only bucket-and-exact-verify collapse.
  Ordinary Move A uses the mobile-span multiset plus identity-free recursive
  skeleton bucket; Move S uses the leaf-sequence plus Coordinator-anchor bucket.
  Verification retains identity outside affected regions, and every failed
  verification leaves both candidates visible to ordinary selection.
- Representatives hoist every mobile to its highest admissible host. Slots are
  populated outermost first; Move-S alternatives use existing
  `Conjunct(role, ordinal)` path steps. No declared frame-role filler becomes a
  mobile or site, and no path crosses an opaque frame boundary.
- The general possessed-Coordination arm is restored. Its previously measured
  55-unit delta was entirely the shared-head possessive family; no broader arm
  coverage was found in that run.
- Positive witnesses cover the complete ordered Move-A/Move-S set, including
  Fury, Grand Abolisher, Reprocess, Lich-Knights' Conquest, Malevolent Witchkite,
  Boltbender, Aquatic Alchemist // Bubble Up, Grafdigger's Cage, Weathered
  Runestone, Ground Seal, Silent Gravestone, Mass Manipulation, Seedborn Muse,
  nesting, render invariance, unchanged leaf traversal, and the hoisted
  construction path. §C.4a witnesses cover Cynical Loner, `Draw a card for each
  Island you control.`, a class-B move within a frame Object, and empty frame-role
  site inventories. Negative witnesses cover a neutral family swap,
  Coordination associativity, A.3-S3a binary rebracketing, and construction
  identity with the Identity specificity tier disabled.
- The closure gate exposed one too-strict Move-A check on a host whose residual
  span became empty after its other mobile role was pruned. The declaration
  compiler already proves the role is right-peripheral, so the redundant
  nonempty-host-span condition was removed generically. The new frame-Object
  class-B witness then passed, and the existing nominal fixture returned from
  two candidates to one.

### PROVE / DISCLOSE / REPORT state at STOP

- Working-method amendment: after the coordinator interruption, development
  used the affected subset only; full-corpus commands were reserved for the
  refreshed final tree. The closure gate then exposed the residual-span defect;
  because code changed, one replacement final ambiguity/performance pass was
  permitted and is the measurement that triggered this STOP.
- Affected subset, 3,231 cards, 8 workers: 1,066 selected, 592 unique, 474
  specificity-resolved, 0 unresolved ties, 0 internal failures; roundtrip was
  1,066/1,066 clean. The focused frame-opacity/class-B materializer witness
  passed 1/1 after the fix.
- Refreshed parent ambiguity census: 32,641 total; 19,845 selected; 15,678
  unique; 4,167 specificity-resolved; 12,796 parse failures; 0 exception uses,
  unresolved ties, or internal failures. Feature census: 19,900 selected;
  16,580 unique; 3,320 specificity-resolved; 12,741 parse failures; 0 exception
  uses, unresolved ties, or internal failures. This is +55 selected, +902
  unique, and -847 specificity-resolved. Candidate counts fell in 1,562 units
  by 3,367 candidates; 247 units lost more than two candidates; 925 selected
  construction paths changed; no selected unit was lost.
- Final-tree performance, same refreshed tree pair, back-to-back, 8 workers:
  parent 113 s wall, 126,940 ns/B, load 2/4/6 before and 5/5/6 after; feature
  113 s wall, 143,683 ns/B, load 4/5/6 before and 4/5/6 after; delta +13%.
  `uptime` reported 6 users; the sandbox cannot observe sibling-process counts.
- The earlier final-tree coverage/roundtrip run was superseded when the closure
  gate caused a code change and a second refresh. Because the replacement
  performance measurement is a mandatory STOP, replacement coverage
  `--check`/`--bless`, roundtrip, changed-selection classification, packed-unit
  listing, construction/checker inventory, and closure-gate rerun were not
  performed. The current lock contains 19,900 identities after the harmony
  keep-both resolution, but it is not claimed as checked on this STOP tree.
  No citations changed.
- Closure-gate command before the residual-span fix printed:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`.
  It reached 420/420 construction-core tests and 155/155 English-v2 library
  tests, then correctly failed an existing nominal-grammar candidate-count
  assertion that the focused fix restored. Strict clippy was therefore not
  reached and the closure gate is not claimed green.
- Assurance: 0 restored, 13 re-spelled, 0 ignored, 10 added, 0 removed. The
  tenth addition is the residual-empty-host class-B witness found by the closure
  gate.

### Deviations and additions

- The general possessed-Coordination arm and mobile declarations are expressly
  ordered additions. Generator/runtime/visitor changes implement the already
  ordered per-role slot shape and §C.4a frame boundaries.
- No AST category, selection exception, specificity weight, word/card/
  construction-naming guard, per-preposition switch, census gate, serialized
  diagnostic surface, or principle (iv) work was added. Verification failure
  still degrades to two candidates.
- Acceptance artifacts and the requested stratified/all-55 classification are
  incomplete solely because OPEN-4 requires stopping at the failed final-tree
  performance measurement.

Decision wanted: amend the OPEN-4 bar or authorize another root-only
optimization round. Per-node collapse remains out of scope.

Glossary gaps remain: Scope, Attachment, Head, Premodifier, Peripheral,
Bracketing, and Mobility.

## Landing record

**Reviewed and corrected; see `### Review corrections`.** Implementer tip
`yxopsuzvpnur` on fork point `kutoxkxyosox`; reviewed tree refreshed onto fork
point `mzlqyrqpluuq`, whose only difference from `kutoxkxyosox` is two ticket
folder moves. PROVE and DISCLOSE below are the implementer's, stamped to
`yxopsuzvpnur`; `### Review corrections` carries the findings, the fixes and
every figure re-measured on the corrected tree, and supersedes any figure it
restates. The report-mode lock covers 19,900 identities.
Final-round wall clock: 2026-09-05 09:13 PDT to 2026-09-05 09:53 PDT;
review round 2026-09-05 09:55 PDT to 2026-09-05 11:20 PDT.

### What landed and why

- Candidate finalization now performs the root-only scope collapse. A cheap,
  identity-free fixed-width digest key buckets Move-A candidates, a second
  Coordinator-anchor key buckets Move-S candidates, and exact pair
  verification retains construction identity outside the affected region.
  Failed verification leaves the two candidates intact for ordinary selection;
  there is no silent merge.
- Exact Move-S verification implements A.3-S1 through A.3-S6 and A.3-S3a:
  Coordinator leaves form a total positional bijection; only an edge-adjacent
  contiguous difference region is eligible; erasure is confined to that scope
  region; Coordination/touched-Conjunct identity, mobile inventory, and ordered
  claim boundaries are compared by value; an empty difference uses ordinary
  A.3; existing `Conjunct(role, ordinal)` steps express the alternatives.
- The representative hoists each declared mobile role to its highest admissible
  site, packs mobiles outermost first, and records remaining sites in its
  always-present per-role `AdmissibleSites` slot as ordered
  `AttachmentSitePath`s. Nested paths stay in their minting subtree. Leaves and
  rendered bytes are unchanged, and the selected construction path belongs to
  the hoisted representative.
- Declared custom Verb Frame fillers are opaque boundaries: they are neither
  mobiles nor sites, no region or host set crosses them, and candidates built by
  different frames are rejected before identity erasure. The general
  possessed-Coordination arm was restored as expressly ordered.
- OPEN-4 fixes are all present: (a) fewer than two mobile-bearing candidates
  returns early; (b) `render_with_claims` is inside exact verification and
  memoized; (c) bucket keys use fixed-width structural digests; (d)
  `ScopeProjection`, mobile inventory, and rendered claims are lazy. No per-node
  collapse or additional optimization was introduced.

### PROVE

- The closure gate printed exactly
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`
  and then the matching strict-clippy line. Principal positive artifacts were
  `test result: ok. 44 passed`, `test result: ok. 420 passed`,
  `test result: ok. 155 passed`, `test result: ok. 49 passed`,
  `test result: ok. 32 passed`, `test result: ok. 112 passed`, and
  `test result: ok. 468 passed; 1 ignored`; every other emitted test suite also
  reported `test result: ok`. Strict clippy completed with `-D warnings`.
- Report-mode coverage `--check` and `--bless` both reported 32,641 total,
  19,900 selected/covered, 0 selected-uncovered, 12,741 parse failures, 0
  unresolved ties, 0 internal failures, 0 exception uses, 0 roundtrip
  mismatches, and 0 ownership failures. The checked lock delta was `+0/-0` on
  the refreshed feature tree; versus the fork it is +55 covered and 0 lost.
  No identity stopped being covered, and no wrong analysis or negative oracle
  became covered.
- `ambiguity --json --require-resolved --workers 8` reported 19,900 selected:
  16,580 unique, 3,320 specificity-resolved, 0 exception-resolved, 0 unresolved
  ties, and 12,741 parse failures. `roundtrip --require-clean --workers 8`
  reported 19,900 accepted, 19,900 clean, 0 mismatched. `cargo xtask catalogs
  check` reported `catalogs are up to date`.
- Coverage traversal was balanced corpus-wide: 887,312 nonterminal nodes and
  887,312 visited constructions (887,316 before the review corrections); 309,941 expected and 309,941 visited leaves;
  0 construction- or leaf-traversal failures. Raw-versus-packed witnesses also
  assert equal leaf counts and that the construction path is the hoisted
  representative's path. Every packed witness and the corpus-wide roundtrip
  render byte-identically.
- The Move-A, Move-S, nesting, render, traversal, and both OPEN-1 failure
  directions are asserted. Must-pack witnesses include Fury, Grand Abolisher,
  Reprocess, Lich-Knights' Conquest, Malevolent Witchkite, Boltbender, Aquatic
  Alchemist // Bubble Up, Grafdigger's Cage, Weathered Runestone, Ground Seal,
  Silent Gravestone, Mass Manipulation, Seedborn Muse, Doorman, and the complete
  §G.5 set. Neutral family-swap, Coordination-associativity,
  binary-rebracketing, and construction-identity fixtures do not pack; the
  Identity-disabled Daretti/Nightmare fixtures remain visible ties inside the
  fixture rather than silent merges.
- §C.4a witnesses pass: Cynical Loner retains two distinct candidates,
  specificity selects its declared-frame reading, and it is not packed; `Draw a
  card for each Island you control.` has one host on this tree — one candidate
  before and after the collapse, verified against the fork point — so its
  Predicate-Adjunct derivation survives untouched, which is what §C.4a's
  conditional witness requires; both the behind-an-
  opaque-edge and inside-frame-Object class-B units pack locally; the exhaustive
  packed census found zero site paths into a declared frame role. `Search your
  library for a card.` correctly resolves by specificity with both candidates
  present and distinct; preemption widening remains routed to B6.

### DISCLOSE

- Coordinator interruption changed the development method: subsequent work used
  only the affected subset, and the refreshed full corpus set was run once at
  closure. The fork and feature ambiguity commands used identical
  `--json --require-resolved --workers 8` flags; the temporary fork working copy
  was abandoned afterwards.
- Fork census: 19,845 selected, 15,678 unique, 4,167 specificity-resolved,
  12,796 parse failures. Feature census: 19,900 selected, 16,580 unique, 3,320
  specificity-resolved, 12,741 parse failures. Net: +55 selected, +902 unique,
  -847 specificity-resolved. Resolution transitions were 894
  specificity-to-unique, 47 newly specificity-resolved, and 8 newly unique.
  Candidate counts fell in 1,562 units by 3,367 candidates; 247 units lost more
  than two; 925 already-selected paths changed; no selected unit was lost. The
  largest named construction pair where specificity share moved was
  `PostmodifiedReferenceUnqualifiedPostmodifiedReference` versus
  `PostmodifiedReferenceRelativeQualifiedReference` (192 comparisons); the next
  was the corresponding prepositional-qualified pair (170).
- All 247 units in the greater-than-two-candidate-drop stratum were read per
  identity: 79 correct with the selected path unchanged (32%), 168 correct but
  recanonicalized (68%), 0 misselections, 0 wrong analyses. The remainder
  universe held 1,096 identities; a deterministic uniform sample of 60 was read:
  23 correct/unchanged-or-newly-selected (38%), 37 correct but recanonicalized
  (62%), 0 misselections, 0 wrong analyses. Of the high-drop stratum, 176 were
  packed and 71 only lost non-selected candidates.
- The possessed-Coordination arm's selected-path coverage is 0 to 56. All 55
  newly covered identities are the shared-head possessive family and were read;
  8 resolve uniquely and 47 by specificity. The 56th, Frenzied Saddlebrute, is
  a correct-to-correct recanonicalization over the same shared possessor. There
  is no breadth beyond that family.
- The packed census is 1,060 unit identities. It was obtained with a temporary,
  non-serialized visitor over the exhaustive affected universe (every unit with
  a candidate-count or selected-path/status change); the hook was removed. The
  count exceeds the earlier 934 observation after the refreshed lexical and
  quality-surface changes and is REPORT-only, never fitted to. The complete
  identity list follows below.
- OPEN-4 performance advisory, same refreshed tree pair back-to-back, 8 workers:
  parent 113 s, 126,940 ns/B, load 2/4/6 before and 5/5/6 after; feature 113 s,
  143,683 ns/B, load 4/5/6 before and 4/5/6 after; delta +13%. `uptime` showed 6
  users; sandbox isolation hid sibling-process count. The four pinned cheap
  fixes (a)-(d) are all applied, so the existing pair is retained as ruled. The
  independent final coverage/ambiguity/roundtrip advisories were respectively
  113 s at 141,248 ns/B and load 9/7/6, 120 s at 143,919 ns/B and load 5/8/7,
  and 112 s at 138,314 ns/B and load 5/7/7, all with 8 workers. The 16 s
  quiet-host ceiling was exceeded under load and is reported, not gated.
- Inventory pins: 396 constructions, 23 permitted licensing checkers, 0
  forbidden checkers, 2 licensed vocabulary/lexicon homographs, 9 form-literal
  overlaps, longest form literal 11 bytes.
- Assurance: 0 restored, 15 re-spelled, 0 ignored, 11 added, 0 removed. The
  fourteenth re-spelling updates the existing right-periphery witness to assert
  the hoisted representative plus its retained last-Conjunct alternative; the
  fifteenth, added at review, re-spells the shared-premodifier ownership
  assertion in `common_noun_modifiers_compose_under_a_shared_target_selector`
  against the hoisted representative. The eleventh added test is the review's
  `every_absorbed_member_leaves_an_alternative_site`.

### Review corrections

Reviewer findings and the fixes applied in this workspace. Every number in
PROVE, DISCLOSE and REPORT above and below has been re-measured on the
corrected tree; the pre-correction figures are retained inline where they
differ.

- **HIGH - a packed member's alternative host was discarded whenever the two
  hosts shared a construction identity.** `populate_component_sites` dispatched
  on `host.identity` equality, so a Move-A pair whose high and low hosts are two
  instances of one construction at two heights - the commonest shape in the
  class, `the number of X you control` among them - fell into
  `alternative_path_within_host`, which describes a difference *inside* one
  host and returns nothing for a difference *between* two. The member was then
  dropped with an empty `AdmissibleSites` slot: a single tree with no signal,
  the exact failure §A.3-S7 and §C.4a name. Evidence: on the implementer tip,
  `{T}: Add X mana of any one color, where X is the number of Allies you
  control.` (Harabaz Druid) fell from two candidates to one with every slot
  uninitialised; four of the eleven probeable units in the reviewer's uniform
  sample of the changed-path set behaved the same way, and five more lost some
  of their paths. Fix: dispatch on the attachment relation, which is what makes
  one host the alternative of the other, and fall through to the partition
  cases only when the two attachments are equal.
- **HIGH - a member could be absorbed even when no alternative could be
  expressed.** The collapse dropped every verified pack member unconditionally,
  so an inexpressible alternative was silently deleted rather than degrading to
  two ordinary candidates. Fix: `populate_component_sites` now returns the
  members it could name a host for, and only those are dropped. 69 units keep a
  second candidate on the corrected tree that the implementer tip had absorbed
  without a signal; 47 of them return from `unique` to `specificity`. This is
  the design's own failure mode - verification failure keeps both candidates -
  applied to representation as well as to equality. New witness:
  `every_absorbed_member_leaves_an_alternative_site`.
- **MEDIUM - a test assertion was deleted rather than re-spelled.** The
  shared-target-selector witness in `nominal_grammar.rs` lost its
  `CoordinatedNominalModifierCoordinatedModifierMember` ownership assertion when
  the premodifier became mobile. Fix: re-spelled against the hoisted
  representative (`NominalModifiedSingularCoordinationNominalValue`), which is
  the new canonical owner for all ten surfaces in the loop.
- **MEDIUM - the record claimed a scope pack `Draw a card for each Island you
  control.` does not have.** The unit has one candidate on the fork point and
  one on the feature; §C.4a's witness is conditional on two hosts existing.
  Fixes: the claim is corrected above, and the witness now asserts the raw and
  packed candidate sets are the same size so it can no longer pass vacuously.
- **MEDIUM - the cite gate was not run and was red.** Four wall-time figures in
  the superseded STOP records were written as `NNN.NNN s`, which
  `cargo xtask cite check --list-noncompliant` reads as rule numbers. Fix:
  rewritten as integer milliseconds; the checker now reports
  `0 non-compliant citation-looking string(s)` and `0 stale`.
- **LOW - `emit/visit.rs` hardcodes the grammar's `Object` category name.**
  `frame_atom_matches` resolves `VerbFrameAtom::ObjectNounPhrase` to
  `field.terminal() == "Object"`; the other three sealed frame atoms resolve to
  their own names. A rename would silently stop marking frame roles rather than
  failing to build, and opacity would degrade without a diagnostic. Left as is -
  it is exercised by the Cynical Loner witness - and recorded here for the
  declaration-compiler lane.

**Re-measured on the corrected tree** (fork point `mzlqyrqpluuq`, both sides
`--json --require-resolved --workers 8`):

- census 19,845 selected / 15,678 unique / 4,167 specificity-resolved on the
  fork against 19,900 / 16,533 / 3,367 on the feature; +55 selected, 0 lost,
  847 specificity-to-unique. Before the corrections the feature read
  16,580 / 3,320 with 894 specificity-to-unique.
- 3,257 candidates removed over 1,493 units (was 3,367 over 1,562); the
  greater-than-two-drop stratum holds 245 units (was 247); 922 already-selected
  paths changed (was 925). No selected unit was lost and no unit gained
  candidates against the fork.
- `coverage --check` exits 0 with the lock exactly current: 32,641 total,
  19,900 selected and covered, 0 selected-uncovered, 12,741 parse failures, 0
  unresolved ties, 0 internal failures, 0 exception uses, 0 roundtrip
  mismatches, 0 ownership failures, 23 permitted and 0 forbidden licensing
  checkers. `roundtrip --require-clean` reported 19,900 accepted, 19,900 clean,
  0 mismatched. `cargo xtask catalogs check` reported `catalogs are up to date`.
- Gate: `cargo xtask gate --changed` printed
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`,
  which ran green - principal artifacts `test result: ok. 44 passed`,
  `test result: ok. 420 passed`, `test result: ok. 156 passed`,
  `test result: ok. 49 passed`, `test result: ok. 32 passed`,
  `test result: ok. 112 passed`, `test result: ok. 468 passed; 1 ignored`, and
  `test result: ok` on every other emitted suite. Strict clippy over the same
  four crates finished clean under `-D warnings`.
- Reviewer's own fork-versus-feature performance pair, 8 workers: fork
  125,093 ns/B in 116 s at load 9/5/4; feature 144,157 ns/B in 112 s at load
  10/10/8 (coverage), 149,745 ns/B in 117 s at load 8/9/8 (ambiguity),
  149,461 ns/B in 116 s at load 9/10/8 (roundtrip). The delta is +15 per cent
  against the implementer's stamped +13 per cent, measured under heavier
  contention on the fork side, so it corroborates rather than replaces that
  figure. **Contention stamp**: one concurrent codex executor and one other
  Opus reviewer were active on this host throughout both the implementer's and
  the reviewer's runs; the implementer's `uptime`-only figure could not see
  them.
- The packed-unit identity list below is stamped to the implementer tip
  `yxopsuzvpnur` and is an upper bound on the corrected tree. The 84 units whose
  candidate count or selected path moved under the review corrections are:

  - `Ajani, the Greathearted` (4->6)
  - `Angel of Deliverance` (2->4)
  - `Autumnal Gloom // Ancient of the Equinox` (2->4)
  - `Aven Heartstabber` (1->2)
  - `Backwoods Survivalists` (1->2)
  - `Beastie Beatdown` (1->2)
  - `Bloodbraid Marauder` (1->2)
  - `Brine Seer` (3->4)
  - `Cinder Seer` (3->4)
  - `Corrupted Grafstone` (1->2)
  - `Crop Sigil` (2->4)
  - `Deathcap Cultivator` (1->2)
  - `Demolisher Spawn` (2->4)
  - `Descend upon the Sinful` (1->2)
  - `Desperate Sentry` (1->2)
  - `Drag to the Roots` (1->2)
  - `Dragon's Rage Channeler` (2->4)
  - `Dragonclaw Strike` (5->6)
  - `Dusk Feaster` (1->2)
  - `Extricator of Sin // Extricator of Flesh` (2->4)
  - `Floodpits Drowner` (1->2)
  - `Foul Watcher` (1->2)
  - `Geist of the Lonely Vigil` (1->2)
  - `Gibbering Fiend` (2->4)
  - `Gnarlwood Dryad` (1->2)
  - `Graveyard Shift` (1->2)
  - `Grim Flayer` (1->2)
  - `Hand That Feeds` (1->2)
  - `Hound of the Farbogs` (1->2)
  - `Impossible Inferno` (1->2)
  - `Inquisitor's Ox` (1->2)
  - `Ishkanah, Grafwidow` (2->4)
  - `Ivy Seer` (3->4)
  - `Jasmine Seer` (3->4)
  - `Kessig Dire Swine` (1->2)
  - `Kindly Stranger // Demon-Possessed Witch` (1->2)
  - `Let's Play a Game` (1->2)
  - `Manic Scribe` (2->4)
  - `Matzalantli, the Great Door // The Core` (1->2)
  - `Metalworker` (3->4)
  - `Might Beyond Reason` (1->2)
  - `Mindwrack Demon` (2->4)
  - `Moldgraf Scavenger` (1->2)
  - `Moorland Drifter` (1->2)
  - `Mournwillow` (4->8)
  - `Nightshade Seer` (3->4)
  - `Obsessive Skinner` (2->4)
  - `Paranoid Parish-Blade` (1->2)
  - `Patchwork Beastie` (2->4)
  - `Raving Visionary` (1->2)
  - `Reaper of Flight Moonsilver` (1->2)
  - `Rofellos's Gift` (3->4)
  - `Sanguine Spy` (2->4)
  - `Scent of Brine` (3->4)
  - `Scent of Cinder` (3->4)
  - `Scent of Ivy` (3->4)
  - `Scent of Jasmine` (3->4)
  - `Scent of Nightshade` (3->4)
  - `Scour the Laboratory` (1->2)
  - `Scourge Wolf` (1->2)
  - `Scuttletide` (1->2)
  - `Sewer Crocodile` (1->2)
  - `Shifting Grift` (1->4)
  - `Shifting Woodland` (1->2)
  - `Snooping Newsie` (1->2)
  - `Soul Swallower` (2->4)
  - `Spineseeker Centipede` (2->4)
  - `Stallion of Ashmouth` (1->2)
  - `Syndicate Infiltrator` (1->2)
  - `Tainted Indulgence` (1->2)
  - `Tangle Wire` (4->6)
  - `The Swarmweaver` (1->2)
  - `Thraben Foulbloods` (1->2)
  - `Thranduil, the Elvenking` (1->2)
  - `Tooth Collector` (2->4)
  - `Topplegeist` (2->4)
  - `Unholy Heat` (1->2)
  - `Unnatural Growth` (10->12)
  - `Violent Urge` (1->2)
  - `Whispers of Emrakul` (1->2)
  - `Wickerfolk Thresher` (2->4)
  - `Wildfire Wickerfolk` (1->2)
  - `Wojek Apothecary` (4->5)
  - `Zopandrel, Hunger Dominus` (10->11)

  The `packed_units` figure becomes a first-class, re-derivable census in
  `english-v2-scope-device-census` (phase 5); no throwaway hook was re-added to
  restate it here.

- **Post-refresh gates on the integrated tree.** `english-v2-landwalk-quality-keyword`
  integrated during the review, so the workspace was refreshed again and every
  gate the incoming diff can reach was re-run. On that tree: `coverage --check`
  exits 0 with the lock exactly current - 32,641 total, 20,002 selected and
  covered, 0 selected-uncovered, 12,639 parse failures, 0 unresolved ties, 0
  internal failures, 0 exception uses, 0 roundtrip mismatches, 0 ownership
  failures, 888,873 nonterminal nodes against 888,873 visited constructions,
  310,547 expected against 310,547 visited leaves, 23 permitted and 0 forbidden
  licensing checkers; `ambiguity --require-resolved` exits 0 at 20,002 selected,
  16,631 unique, 3,371 specificity-resolved, 0 unresolved ties;
  `roundtrip --require-clean` exits 0 at 20,002 accepted, 20,002 clean, 0
  mismatched; `catalogs check` reports `catalogs are up to date`; the closure
  gate is green (`test result: ok. 44 passed`, `421 passed`, `156 passed`,
  `49 passed`, `32 passed`, `112 passed`, `468 passed; 1 ignored`, every other
  suite `ok`); strict clippy over the four crates is clean; `cite check
  --list-noncompliant` reports `0 non-compliant citation-looking string(s)` and
  `cite check` reports 0 stale over 14,490 citations. The +102 covered
  identities and the movement from 19,900 to 20,002 are landwalk's, not this
  landing's - this landing's effect is the fork-point pair stamped above.
  Refreshed-tree performance advisory, 8 workers: coverage 119 s at
  141,396 ns/B, load 8/9/8; ambiguity 117 s at 151,573 ns/B, load 12/10/9;
  roundtrip 113 s at 153,743 ns/B, load 7/10/9. The 16 s quiet-host ceiling is
  exceeded under that load and is reported, not gated.
- **Reviewer's independent classification.** A uniform sample of 15 of the 925
  changed selected paths was read from the fork point against the feature.
  Twelve are a single `PostmodifiedReference*QualifiedReference` node moving
  earlier in the pre-order - the same postmodifier at the higher of its two
  admissible hosts. Kellan, Vernal Sovereign, Unnatural Growth and Silent
  Gravestone are Move-S shared constituents hoisted over a Coordination, and
  Harald is a shared auxiliary over a Predicate Coordination. All 15 are
  correct-but-recanonicalized; 0 misselections and 0 wrong analyses. The
  implementer's 247/247 and 60/1,096 strata are corroborated, with the
  qualification that on the implementer tip the recanonicalization was in
  several cases *not* accompanied by a recorded alternative - the HIGH above.

### STOP history and resolutions

- STOP 1, required flat-versus-nested Move-S packs rejected by ordinary A.3:
  resolved by the A.3-S1..S6/A.3-S3a amendment and implementation.
- STOP 2, Cynical Loner wrong selection: resolved by §C.4a frame opacity;
  preemption widening was explicitly routed away from this ticket.
- STOP 3, the initial +28% root-only cost: resolved by OPEN-4 fixes (a)-(d).
- STOP 4, the residual +13% after those fixes: coordinator ruling made this a
  CLAUDE.md performance advisory under REPORT, not a gate. No STOP remains.

### Deviations, additions, and glossary gaps

- The general possessed-Coordination arm and its declared mobile roles are the
  only construction-side addition, expressly ordered by the 2026-09-05 work
  order. Generator/runtime/visitor changes carry the already-ordered per-role
  slot and frame boundary through the construction interface. The one existing
  right-periphery test was re-spelled after the closure gate showed its old
  one-candidate shape was no longer the canonical highest-site representation.
- Two further declarations beyond the arm: `modified_singular_nominal` and
  `modified_plural_nominal` gain `first: mobile(rest)`, which is §A.2's
  attributive-Modifier family over a Coordination and is what gives Aquatic
  Alchemist // Bubble Up its shared-Modifier slot beside its shared head.
- The `AdmissibleSites` slot changed shape from one per element to one per
  mobile role, as the phase-1 review's amendment reserved to this phase; the
  paths are populated after the tree is built, through a `OnceLock` that
  accepts one write.
- Added at review: the site-path dispatch fix, the never-absorb-without-a-signal
  guard, `every_absorbed_member_leaves_an_alternative_site`, a raw-versus-packed
  size assertion on the `Draw a card for each Island you control.` witness, and
  the re-spelled shared-premodifier ownership assertion in
  `nominal_grammar.rs`. See `### Review corrections`.
- No AST category, selection exception, specificity weight, word/card/
  construction-naming guard, per-preposition or per-construction switch, census
  gate, serialized diagnostic/JSON surface, or principle (iv) work was added.
- Glossary: `docs/contexts/oracle-english/CONTEXT.md` defines **Modifier** and
  **Postmodifier** and uses *head* throughout ("a phrase headed by a verb", "a
  relation licensed by a head") without an entry of its own, so **Head** and
  **Premodifier** are groundable in existing convention rather than unowned.
  **Scope**, **Attachment**, **Peripheral** (as in right-peripheral),
  **Bracketing** and **Mobility** remain true gaps. Per the design none is
  coined into the tracked glossary here, and no new synonym was introduced.

### Newly covered identities (55)

Every identity below selects a path containing
`UnqualifiedReferencePossessedCoordinationReference`; all are the shared-head
possessive family. The eight unique resolutions are Dread Tiller, Harness
Infinity, Jace, the Living Guildpact, Morality Shift, Riveteers Confluence,
Swift Warkite, Temporal Cascade, and Trenzalore Clocktower; the other 47 resolve
by specificity.

- `83ccbef11e4dcd85d5f17181abce99b8c0a46802dfcd74f20f30cf35177ef8b6` — Ajani's Aid
- `54e2d49e4859b0a3ebd364239a296f13c23ad25b20e937d03cfb1004bc976851` — Angrath's Fury
- `c5abc86714ff8fe63013bb80f5db9e23710a2b2e1305d5d888cdce4d4e8295f1` — Arachnus Spinner
- `8c5208115a67729b771f3ee9245d77f0ea66cc767103955a0f427e78cad13d56` — Ashiok's Forerunner
- `1621bbb2d64ec0fc2cb29e681255795abc968f738d9d1d49a2a96c0ea509d198` — Auditore Ambush
- `9ba2f7370fb74d9d502f3d68a37bc38fcb7fd8046f78fcf7b6ad26715ce155a3` — Basri's Aegis
- `e6abb930ff695698ffdfacff6d7ad76cbec6b760a0e629311b0c6ac33c15f73d` — Boonweaver Giant
- `7908b8fe581d6d4ed1c49c11d3e9aed69eedf9589577255e6e075e3cbdc61ae5` — Chandra's Firemaw
- `3f8aa022c37beef9d674c918289cb74d458fcac8dddcbce5af54707554aecf08` — Chandra's Flame Wave
- `fe6b2f91ca2db593e2224bb0c6dd2935a9f8e83019ea5382a193468eeef2c474` — Chandra's Outburst
- `f30fc24b88b83b16bfed8dcc4719685b48cdc955d43385550b6df7bc5808a0c8` — Danitha, Benalia's Hope
- `1eb135d0dc0578598a4f2563729af62ebdf1f99e91ad96c2591491306df3c55c` — Dark Supplicant
- `3a84f5f44a0c4f965d7dda1c40730a9a137d364bffce2fe6aaad87128a83db82` — Delivery Moogle
- `ccd05401be058da6437ff9886e670eda76a3cb8b360030ccec90bd481196cf76` — Dina's Guidance
- `5d803f3348ec0319271ea4b58c35f02289b5e7d0158504c56b785d3db61dcb0d` — Domri's Nodorog
- `39b1bee4ea16496faf630d45325966961219d38c0476ff6e6bea03b5d1f59c48` — Dovin's Dismissal
- `9353cfe353c7387f08384e72c563f46f7f3c538b7b8a6042c066257861108ace` — Dread Tiller
- `cae1cc612fb7692548b3336993f40577cb1842638ab39c8d54ea782b4e7fe7ff` — Elspeth's Devotee
- `dec71b9adbb1b434f3fe9ccaf99fb82ce969190cc1cdb8fcb0e14f2d97a44e5f` — Ethereal Elk
- `f1bd3bbb45958e7db4466093824de0386bcb4fc7c54062f001950f86a2f25c8a` — Fang-Druid Summoner
- `74b301b1711ed3e2d07517eb111e3bf116214ec9ca2aeb72291041123f39eea9` — Garruk's Warsteed
- `3644881835ed676ff109f5fb20e9f4ecf270a668b03036eb9fd3eb3a6df6d4f3` — Gideon's Battle Cry
- `44955eeb43554a789e477a913ca2970b8f7475ac49a89a1a99199d04a675cc61` — Gideon's Resolve
- `2b91de3d89f847b0c660a101a71608534f8ab851e458e861c09697fb35436608` — Goldmane Griffin
- `1786963c2cb6781e016ee905ac79cbf84a03665c4741097b94adcd9bee875f53` — Harness Infinity
- `a0beb8e22e477e36339fd87c1854069ff3183cab21f19e045827789f5fca8a0b` — Invasion of Ikoria // Zilortha, Apex of Ikoria (Invasion of Ikoria)
- `9c00a72ca33c983949e9f134a1d56a6a177cf0d307a6b183105012c38e065710` — Jace's Ruse
- `089107730883e79c0738eb1c9d994eda1e0abc1c2ea57ab145c26aab18e39bc3` — Jace, the Living Guildpact
- `28ec5d76b35f3498852fa1c7e5b4072d010b67fb5ba2bdf62713fd6e803b1eca` — Journey for the Elixir
- `b7f52fafcfba78a55a0bdd9379990ffa17d78645953616c77393941c0a5bf50c` — Kassandra, Eagle Bearer
- `bb1baed3b1f4e72028d31d160c081dba5d88b744fcebd646a4e1490102448418` — Liberated Livestock
- `484c4b493dfb7f3fc58265b08e8ed9f644aa2d84ba104e4403283e029a0f4681` — Liberating Combustion
- `0823e818a41cc9b1cd9087fe84eedd3bcf631dd7dc396e8724dba90a996f2a63` — Liliana's Influence
- `2370d0c4b15b17aa74ff6c8e567a8ae5db65c3ad67ee2408d018a931d07641b4` — Liliana's Scorn
- `86eba36ffae86b3e740fc2740f6929dd8c79959819425814f3d924b568fe1c77` — Morality Shift
- `bb4a63547aba54a2212430dc2c22d73e4a0e4080f79a0acab098c0f5a9826925` — Niambi, Faithful Healer
- `2afc355c059ce90a3a43706b69812ec9f6a2ff2b4bfa6439619315fbe777f3f0` — Nissa's Encouragement
- `937bcb4350d3b8376f44e37109f7ed6d670136d16953f83f08467601b3e2b14a` — Ral's Dispersal
- `1851e183010f34c5926ba29c7bfcbe1eeaa3de3899a5994d4208b001d9557545` — Raven Clan War-Axe
- `1f9930058d5460c123d3b91551dcf5160d3f9b4ce611093d3d9be60a13735914` — Riveteers Confluence
- `f82d9902ee7f898cdd3152143cee696518954917278616431b2c99c486b32d4c` — Rowan's Stalwarts
- `7d67045f22a186d38aaa8098b0e949bb5e18d71f409faa0dd8c3bc645cf77875` — Runed Crown
- `396ef0833d44c1708f62e60de67b95c63a2198e5a0044d0b7b5ff0127da6a279` — Runeforge Champion
- `fa1815b78953c4c806369e34ed0572f7dd5f09c92e9aaf48bb615a89af939822` — Sorin's Guide
- `63a2218097327532b94731d00ffbb235b3f47827ac48880f4a0ab7457c1d8d74` — Swift Warkite
- `24a76c697b28eac01caeffa7e56c20d8a57a116c4858e91468e76e5dea5e6eb8` — Teferi's Wavecaster
- `932d38ff3a21e8e3d58dbbf657bd11b4e7455dfd656edbfbebb04143628e0d3c` — Temporal Cascade
- `5e63fd18fa55df8a88a79294570c2b29b27e18d90c74d4f49e1d57cd32f5823c` — Tezzeret's Betrayal
- `509b34d3ce4ce55cd5785b03cb6388d12621a99284301d795323e0d4443fa6bd` — The First Doctor
- `72aad856036b1880b0bbfb8a607ff90d6096cbdcb55b91252c54cfb50ec327b7` — Tower Winder
- `f14b7813b71560ebc8b42d1ce09a056a264c342b2d38f60c00ade765fbff1706` — Trenzalore Clocktower
- `1abf978f89980c75b2a2604c433ccdc2f91a782ac4a08ac37453756264bbe8b8` — Verdant Crescendo
- `5212c3d3cc5ef9cf088011f8135cbfeb4cfaf8807c50b2e248228ae9f6f7f1e3` — Vraska's Scorn
- `510c66e49b93adfc8ecdaec34ae841a917865645b293690ded753df9b36aec36` — Vraska's Stoneglare
- `e2b4ef454b8431df437092a651b76e41dabe75fccda13605a46b5e62a325ec19` — Yanling's Harbinger

### Packed-unit identities (1,060)

Each line is the corpus unit identity followed by card and, where applicable,
face name.

- `000d0cf2ae4d2b6ec7fb0859644a6f4449b71054ef0b843c8f8af904c42195e4` — Kellan, the Fae-Blooded // Birthright Boon (Kellan, the Fae-Blooded)
- `00a358c00fbb40cd5f274b0dd5817f1bda52d4324403baf49a17071468f71ce3` — Gaea's Might
- `00b2a50a265b79e5578e8428391e7ac580f6351472e97760e9c19bee2150293d` — Insurrection
- `00e7f0f770bcf19dc9127e9930b7fd7a69fee9888f55ba5c184235d9f5ebd938` — Sylvan Ranger
- `017b30728842b32dd6768eccebd2521e5011614d2d1297b3923364b21605d2a7` — Pictures of Spider-Man
- `02036687e2774a5a7b99fcfc27e057c3dea494d6d68a1dce161ed7a5b30172e1` — Goblin Matron
- `0256be61ac4aaf04949426df1313d3d6e247e1b79bc3535c0748069d6b7e3002` — Frogify
- `029affc11cf695191b39c5c6a4eb233d3aba96a8ac6d51b826a2f7ca2b3aa459` — Snakeform
- `02ceed6be19fa59a5c08ac5fa89eb4a11ecc40f4295561f5a92c5c46b0c5d472` — Rat King, Pale Piper
- `030a9820032ab82817348cd09993e47f5059e5fdf960741b6a56fa06267c3f93` — Director Nick Fury
- `036da6c8127f4cc84e23aeaffddcd9f2037cb5e568870faaadf7499ee7f68b53` — Boundary Lands Ranger
- `03a57188552ef974cf9733f9bbe3e5d547cbcc8cfbe597752a78e9e9032f9409` — Mistvein Borderpost
- `04045eac5c6934c7736a1606ca8b0fb78534819d1c1ca6a17675fb2f5129c75d` — Trail of Crumbs
- `041ea7d26041235045623e10edc01bff389bfe946f7a6771a99be872e23de55a` — Warteye Witch
- `0479bc1d425afee63bdbfc2860fc0b30b00becc3300b049758dbdbe058583f8c` — Blazing Torch
- `04801d0a2b37d254db96dee340d1de358d4a428197487b35184a988552fdb295` — Seascape Aerialist
- `04f060d5dbbf1c4cd155196e556ed71bbfda15bf92615188bb5ccfef64781189` — Ashenmoor Liege
- `0526978f27d66aeeb95ef2f834a4d938d768485202ec086b0ff5d4707b2a21ee` — Sentinel of the Eternal Watch
- `05a870e312c8fe0f74347707e282dfe251bd179600d01d03e32884e02115cf99` — Dockside Extortionist
- `05f2fbbcd965ffd175df03568173a1fc62db96d0a888b4e25753f36de3383309` — Thundertrap Trainer
- `05fd3f0b7e0e2ab02f5974b9c764b2197d03af5e33db11871388902ad38f03bd` — Sight of the Scalelords
- `06684dd122e667ff35f06053a739200297c02aa14b2b1830b9768536472fe030` — Sephara, Sky's Blade
- `070a43fd73992a64d6a31d73d93827fa1451e7a53fd866f28d642ce14047f6d4` — Systems Override
- `0710b224d765864b53ee4c7ccebfb9c34ce130d96c69b2fb12217c668c60747e` — Eclipsed Kithkin
- `074e0b885e302b9b40ef0af27d7741c504220d1a066c39b6e4357f032d457ca5` — Iname, Death Aspect
- `0794d06c06b402adb61dafc069ece1fefce4f90d501675b8a535ec1a4759f015` — Forging the Anchor
- `07a34c6888dd034b1746639c2251da2a849ba60cf5ae98368da095764082da9c` — Fear of Exposure
- `07ef6fa6c759b259fd22109ec455243a6a1bfc76987ccb1c26e1fb88143095ae` — Crown of Awe
- `0823e818a41cc9b1cd9087fe84eedd3bcf631dd7dc396e8724dba90a996f2a63` — Liliana's Influence
- `083bb4922e48a966bdea837696fca2600cb3881eb7819bc57b2092715e170315` — Vicious Rivalry
- `084a6280c3aaf8dd2f20523ee11b5f8b2f002d41cc6bd7d63d2f5afc28ff5570` — Leave No Trace
- `084c041d95bd3fc5440b08b574c79dbad56e7c750b0c90c705548c71e42e56fa` — Deluge of Doom
- `0881a26e0b254182ff36d8e7d6eb8b0d4ff9907b7bb0aac4e98f3392786e9a34` — Unwilling Recruit
- `08830f64f2439edd4ecf62a284bd385308d23d541b5ec2fa3bf299454b23bb5a` — Aang, at the Crossroads // Aang, Destined Savior (Aang, Destined Savior)
- `08a949136a24df4199bba95d6b2fb23ee8f900e4860d157f0d56c6a980a2e7c8` — Glister Bairn
- `08a998f8ce9ea98b3923b653103ff784130a48dece8ab7ae6b29f39f94124b22` — Avarice Totem
- `08b44321e475ea549439b2317756c99a92f0f105aeee93df05738bc47e8e4de3` — Nahiri's Machinations
- `08b99e1d38b5845aa75188a2aab28786484ef04697aedb22bb6e99d582879d1a` — Broadcast Takeover
- `08bd4f60b9b319cebf407c9bcca8d62b82d8b5f2fdb497409d69dfedf863cfb3` — Gatecreeper Vine
- `08c965f197f3f32a3a42135dd1117c93524d37d73377b310cfc6241009a65cbe` — Crown of Vigor
- `08db8af4e37c6a4079a050f1d7233c5141886334e2aa902f6da9cbcf66b39ae0` — Eaten by Spiders
- `08e7a2dc300b74fde66ccaec6bb5a992c15641cfc00cff6dcfe69ca0e01d2dc1` — Tribute Mage
- `08e95854b428cbdd675ce0e5169a6864e1f4e7f0565d1bfcde0250288e659ad4` — Higure, the Still Wind
- `092aa064ef23d4760963ae87796dedf0231a9bc55a6bd8bdfd6cd5310852b2da` — Liliana's Spoils
- `093d8b41e84d83ccf3fb74760b4e37a1e95b0dab02dc22f4faf3c529d988ecf9` — Slave of Bolas
- `094ad503faec545e6aa6f80cc9ea66c41eb3feb87db1b7acc7d3468f5ee8f377` — Soulquake
- `09a2e03bb8912fc289ccdd288e3e04137eacbca0e2ac585fc2ca4fae0b6b76a2` — Act of Authority
- `0a2be889ab5f50a719d1a6ff958c9e4ac47087c49a6c72777ec1b2b92cf959da` — Custody Battle
- `0a5c6907afd5b476c73140097a162caf9a6333142820445e18d30e7b57582efa` — Lizard, Connors's Curse
- `0a6ee770f50627a149d4b4ec8eee9ca990a38eccbd7b348faf1b06d866d27502` — Butcher of Malakir
- `0aacadf51d28ed9970618597a0eb12c02892f7ab60f1136d698fd64593bd5535` — Act of Aggression
- `0add1e6377b969b8bcca2604a38b3ae55df213fab9a4c5882630b438ec3b46ba` — Pilgrim's Eye
- `0b3171d02b5258ff770b9c1f4be88fdb216c9f1d8643a01454b07109b76caf26` — Reality Heist
- `0cc2c033a98a6e91d6263d6f79f018e855b9d270ab304f5ad3eec01b11856342` — I Am Iron Man
- `0d012832f5061e71fb46b5826dd556997ddc234d5c4e00a6798384708458e57d` — Pashalik Mons
- `0d0dea32d62d6a0a80a4e285b5d0eddd49cbcf785da05112c5c2fa82697b35e6` — Floriferous Vinewall
- `0d2064cc4c8b3cd05ffb01ea0ccb2e2faccd1cf7cae40bd9407cedd4f887b82a` — Liliana's Shade
- `0d2fd8cba5fbefa37b88c477fe47318c4cb22f6fb90e97bf6bf0c23092f45b11` — Ultimecia, Time Sorceress // Ultimecia, Omnipotent (Ultimecia, Time Sorceress)
- `0d58872f5699e35dcf38f509102dee2e5d56f5241822c49e82c4bb2a1bad14f7` — Chandra, Flame's Catalyst
- `0d5ce7d4ad40cf6a93b279549a80fd170d787422d97f5bd1aa1dff1a8fa1a7ef` — Nahiri, Heir of the Ancients
- `0d7b110049b203accf4a5c6ef4b2cfe1e2320e7e65ea0e0683cc1570cd2e1581` — Tuktuk Grunts
- `0dcba52f80c389129c34719af48596673f35ea4804d11ed2683c8666bdddbcd5` — Nyx Herald
- `0ddae77daf3241df7f2b2bd9d3910a09ec2e5182c5a14c6925b80c581a5ab40c` — Spreading Insurrection
- `0e2c9081df758bcd252a3472da5c1486f2f22b9e2319c857d0c6a1c3741dcca0` — Vernal Sovereign
- `0e6047990aeb0644ea3a23099f289047eb2dc258c37050de468ae224a95a8cc1` — Eclipsed Flamekin
- `0e91972d2e37dafaa182fe3f84ff45346738d567ffc23936462f324bca36bdc4` — Robaran Mercenaries
- `0f025f60451317d36d7402e16f48f0983b95af11594c40023aa903f4e3a1e158` — Incubation // Incongruity (Incubation)
- `0f8ca9e9ef63dffbb65f89946885e50a7fe640408ab85aa697de787bf2220257` — Thrun, Breaker of Silence
- `0fda95cc74c6a87510ebfcb69b9ad3c4f394e769ec70b040eac280f3cba54d01` — Evangelize
- `0fe3d4c519b81195e4d127fc662fb0012b7aa3436551785fae9fd5b1608ec9f0` — Scale Up
- `101d5df0bfc33045a1b3f6c6167f8f47227d0eaf49383ff2d13935516d0827cb` — Flagstones of Trokair
- `108e3c24168bccb99b7c7687a99a3a1593f03f9027d67f0544332ea2624d6f42` — Serpentine Ambush
- `109264ba298629dcd655d834407b48d5e278c031f3164536876fea088d4961ee` — Bloody Betrayal
- `10c9b03f93f7af5080943187d3f8f34ff0a6af14c5c85152a7c6ee73e58c4b06` — Corrupted Shapeshifter
- `112e6f1e39e8dbb458d0008f6a5ea54300c2c266d4d76d3913895ec9202aaf4f` — Conqueror's Flail
- `115553d6f81139ed8993d312a998341a7eeff2a1928c8a315b170ef5c4ff8911` — Goblin Chainwhirler
- `1167d1d52634820dbe5cd5b92ba2002d8bcdf6a7c29e60694e0e8ebe61dc7565` — April O'Neil, Hacktivist
- `11916b59d1ebbd1206b85b766c4ac334a0fb310037c06f1a9c80bf9b89fb998e` — Omen of Fire
- `12f7e141ebb6c0bff4b97cace4755fa6e2189bd63149f9497d251170dd7a47cf` — Commandeer
- `13423cdb56ded8aac0d3515d13ea0cb060726c5dcc96b4c93914d895e6d0b69b` — Highland Berserker
- `138f913e2a472a062cc2852edbb91527d0cd8c4cb39885e4f99bfded502dcecf` — Hijack
- `139b47dc9635d76941394bd8c3d56023926c1f15d0774155146284db7cc7efda` — Invoke the Winds
- `13a022015cccd4050854ea3ec162db76ae80f566dde8b02c853c96a7434d7579` — Oran-Rief Survivalist
- `13b2669326c4b4c08f44aab15080ed634654c09211fc8617a69f113c80df404a` — Eye of Nidhogg
- `13ddd1e399f2a4508ec513d7c7f8071041d51341117193f2b4b57b1a1068d58e` — Venerated Stormsinger
- `13fefdd06ed1f086733cb01013d41f38e72546c5ab58716275fe72ade205e641` — Voldaren Bloodcaster // Bloodbat Summoner (Voldaren Bloodcaster)
- `14132ea85e486008b0b2765a07bcbba0bf138a55301435df46fc5546135dffbb` — Goblin Rabblemaster
- `1437edb23b860dec4cc1146eb821e1719c1152607348f273b99c73a4f345b030` — Green Goblin, Back for More
- `1445cf242da838a8eebf154769544fddb29f6079e93e743c724e21355e62229c` — Wrong Turn
- `14df1933914a4338d654de2888071f5077120e111ef155b59d0f7292b1ee59fb` — Sokka, Swordmaster
- `157f12a2fd0b3f1f5c29119aa3726f09f4d62294ad86ac740f4eb832c31442d5` — Kemba, Kha Enduring
- `1580854bd659000e64e14855e81e626bb84ee46369ad12096f8970e3df15bf77` — Empress Galina
- `163681e2c2f42844ec09a0362066a4142dcfc952c9442a2e8c12c3f54defddf0` — Glen Elendra's Answer
- `175a11119270bf3751d50d68bb394af5a09f01dc71ca4cb206c52030b4dedbb9` — Kor Cartographer
- `176cd88f0fea7ca1cfa38039f3338fa11008ef124e5163a8d04f395fd11caf1c` — Fang of the Pack
- `178a1c22124b897ee2877aefe51d40c4e7cad795601358f4baca9259720432fa` — You Meet in a Tavern
- `17a911ce8ffdf059fd6c0431b98d5ee90bd496a72448abf7a25f1f6d018ea76c` — Combat Professor
- `17ef121bf815c829184e180135ae3aa0f350f10c37b060f0d88b830e41c2e8be` — Ogre-Head Helm
- `1813c4f130ef763d30610dca709b5f9d529ef4631cbc5a8bff09b13bfb090951` — Lux Artillery
- `1846dcd95bb12e79d163f0e1ede9e451099d52bc7fcaff94878fec8ce9ad087b` — Goblin Engineer
- `1851e183010f34c5926ba29c7bfcbe1eeaa3de3899a5994d4208b001d9557545` — Raven Clan War-Axe
- `18c414affb05f47fcc7066b9d60208292fa7aacb4b644f5e886a3527ab7128c2` — Invasion of New Capenna // Holy Frazzle-Cannon (Holy Frazzle-Cannon)
- `193af96f7d8953ff9c35cc1f8082949297d305dd0853eb1b2847dfb3cf58be22` — Fertile Imagination
- `19957dc62592063525e70008d0b9dc430be44f77cef31d14ae449a7ffc6ddbef` — Safe Passage
- `19d16c65f674d4fed273dc7421dfb5b2d2d25b6cd52f1a0e41b1248daef5cb56` — Kithkin Mourncaller
- `19ecf929c06ef1a238be81775e7fc5ca504d070fd47de3509ec11ff3b3722f7d` — Shimmercreep
- `1a3c30023977674bf25f47e46d041e34ad5de09948c76717fd3797f3a67463d3` — Conjurer's Mantle
- `1aff4f3d9995f3a52a39567c628f41cda40df5680a35e202f350724405adea5f` — Tiana, Ship's Caretaker
- `1b2343591c3ac112f85b1a196f2f575f2c6b693599390562a548048d3017692f` — Fury
- `1b4b50173e5a5de57b1e679c6f578e4f0bb5026ea22d760e9047e0c99e9c9efe` — Barret, Avalanche Leader
- `1bbcd280c5fe300b4d9b04f4ba3b440488a6a26c821dd429aca7aa7816477acf` — Mutagen Man, Living Ooze
- `1c65af26b68b31ae96b80ce54d98ed1d1de7f8a47ac215fbd232735c5c89e085` — Stiltzkin, Moogle Merchant
- `1caab1e3f12cf293d381230734ab36645e0f91522d21231ed2572452625a7f41` — Vivien's Grizzly
- `1cd24d5298f6c2fc46a926e249b7b058829cb16cf45e90d733c39624f1b7a891` — Dragonshift
- `1d2c84ed6b010311161b886948e501c033e4012f63ce0fa3224a05c83e0a0e1f` — Commune with Nature
- `1d42bda7a2aedd20006ad52a2df883eb0b397c6eee1fe720731668ec59a2dac2` — Honored Dreyleader
- `1d4bbc99855d5472d095f2f82837b8dd5c8d651d782fb5d847cace4ee5a27ff2` — Sokka and Suki
- `1d567de5527716e763afa9b09567e0326f25860f33a7992c9bf1fb7690f4f5bc` — Debt of Loyalty
- `1d599a2219f5205e7dd1502473a3bb40aa88bcaf7542ba65c286bb2ad71dbbfb` — Millicent, Restless Revenant
- `1d77ea4271f822b16e7a29f54cacf4f26e889612af14c7f09c1b7b652b1e33fe` — Szarekh, the Silent King
- `1deed43d44bc23b89e84fac1249067cc0ca9433ac8741665df525fcbda2cf1b3` — Urtet, Remnant of Memnarch
- `1e2313b2f7ee45c43083730ae953ad9fb99c897568fbf4f0b7e20bb9ef4f711a` — Heliod's Pilgrim
- `1e5be7ed0a4c9d7d7f97f48392fa5a9ed45490277a63f8817b44d85b28b4cd97` — Rootwater Matriarch
- `1e6720d590631a04aed36b80346952b23f340cf2051adc3a7f19275e442aab88` — Wayfaring Giant
- `1e6889cf524389680a20e53e92f745a9f735cf88fd2a95b6f5a57625c101b139` — Ayara, Widow of the Realm // Ayara, Furnace Queen (Ayara, Furnace Queen)
- `1e90b640a273729ff97c46ced20a36bac58760ca5732c79ca5996d6c877cec72` — Ashe, Princess of Dalmasca
- `1e93389f67fcdbf0a6fed94b140e37ad20fc95d7fcf95fb06bc32e3cba1f4ad2` — Blood Oath
- `1eadd59ea7505c2ffdbcf6d4f1304ad9e8b01bd3865e743500c439437ff98ba4` — Faerie Mechanist
- `1f4bb128cf55957258dcbfdca096430f7c978e2453b21706f4bf76516823a688` — Scourge of Valkas
- `1f4f31af5572b40c28182275238a6dc93b356886f7d8881b071772a371802cd2` — Mark of Mutiny
- `1f61ee9d0bd3f85ca7100a86ad97f5e82d633453ac02073750d4e2e3eeb65cce` — Fallaji Vanguard
- `1f953e14e53291b7d90b8f0904c8a97686d99c50244b27434772e5b3c2d58220` — Treasure Mage
- `201a3bc98d63f6db4967b7ee2a31273e80a2af6ed19d5db5288f1bf6cfbe7c87` — Wurmcoil Larva
- `201c14fb0281652ac504fb4c10afcd194bf5ea4ead787a262f82711a897754ad` — Sage of the Falls
- `20a2493c48d465235e71e3e9d60134d5ad0799c08091a82a305bb2960d88f435` — Sazh Katzroy
- `210148ff3642ba0c901845dd938621f4494e7646167cda1447283e15f9d1bbd1` — Fierce Empath
- `218c9efb4341f8683841c3544e92666d92bf8e21f03ca881454edb136e3bba41` — Tekuthal, Inquiry Dominus
- `219735bc1540cf44811c1a8d54c4dfef07491cab4316a253fe5cd8f7cd8b951d` — Voices from the Void
- `22346e2e8fbf5d549c68ffe9cbb5cde3da1764312b596e924320c192045d4f51` — Dire Mimic
- `226665bf46990c52125c76b83b5e04851d8096b3639dae676c31de41c60a5e89` — Noggle the Mind
- `229e3bb0add0521da6dd572ac2e67e444071dd22ff88253132ac56d4cbfdc0b6` — Ruhan of the Fomori
- `22d3db794ba0fee5164616af46d59928694bc3ffab36b2ff69c7c71292aa6298` — Cavalier of Flame
- `22d4679bd9c7738efec4f0a93feadc416ea9e3a1dcf7d9708c0c9575bbc20de0` — Imperial Hellkite
- `2310fbba3b5e95bc56c284488d15ab8d934ffefb0be199225045d9e3fb9f0469` — Swooping Pteranodon
- `23365fe27690ad025f6cfea056ee12d6ac8db5b0d09b2cf2efd9bcbd483b03dd` — Yavimaya Granger
- `233bf9c864a6fc8d4d2e15b762c1900723a61360ad98117506d85078d8f4f544` — Wirewood Herald
- `235993c28060e06f6bfa6509331926cad730d56290ba47462d5d8c1196fee0c8` — Wandering Mind
- `235ae8d41c5dd66d829f3fd80efe9ed2d1cdd924718b535d5b6ec9952afb69ce` — Stormscape Familiar
- `2370d0c4b15b17aa74ff6c8e567a8ae5db65c3ad67ee2408d018a931d07641b4` — Liliana's Scorn
- `2424a5c01a34f9643d1f47feb7dc916c1a932a61e299637a6db372a498d6d5ff` — Bloomvine Regent // Claim Territory // Bloomvine Regent // Claim Territory (Claim Territory)
- `24500df6ce8c7406a5eae13058cccc92fad12139b79346107828cc897fa489ef` — Adherent of Hope
- `245955da19fada83a6781abfc5c67f8c1aad30a541d31138df24c4ffe989389c` — Homeward Path
- `2477af7303c26887e52d3e293e2a89362765bea894f0104c1b1bb577ced31987` — The Spot, Living Portal
- `24a76c697b28eac01caeffa7e56c20d8a57a116c4858e91468e76e5dea5e6eb8` — Teferi's Wavecaster
- `24b5ad17f44685e18708445aad0678d8a311e0eb0761851d91a3570478afcb50` — Yuffie, Materia Hunter
- `24d3bd877ec54570b59a953004601b78052242ee7ac8ecd788296a1acc15258a` — Lumbering Megasloth
- `24f72cc7b1c24d8910d1ce48632c31e433a3ff8879c79bbddba7f4ed0f4c2687` — Recruiter of the Guard
- `2549819445553ba5a0223addbb0d5aadba14d5ed5f9d641c9bb075fd9bf5254f` — Krosan Tusker
- `25e870b642d1b8e5d237c1d0b5c59926d60dd989051e2425f0d540a6b3cb5369` — Delayed Blast Fireball
- `2617e1deb5768a90035d27bce983c950983597b0b86f64c167eeac8ec114d9ee` — Zidane, Tantalus Thief
- `2632ce3b0191b9b1549128c5df00c0ecc9539946f880ba7529c1a9bef4648bca` — Umara Raptor
- `26751cb860907fdc8c5de74bad641414d0db64e637ed029bc0b2ebec4c2b35bd` — Kithkeeper
- `268e0b3917a6576f67a677c670e59ed73ada98072083d76bf91e6d12428428eb` — Agent of Erebos
- `26f02774177514de25ff27a7ca19d37ed0a8767441df9297301558b329959167` — Bosk Banneret
- `27054f56483c88ebfff5d7f1b702b93dd09c0804c1af7ea4dfd96d1b15b0e507` — Hazoret's Favor
- `272a7c1b68c1c2bc96e780048f6b20a171b11e4dffbe049a2a30862797260e8b` — Gossip's Talent
- `2731a838dc04bcc6c7f7ac1c84685f37cb264d2f8ab69c20e0a1e99465c3e53b` — Pious Evangel // Wayward Disciple (Pious Evangel)
- `274381b656e31bc2a982c14363927fe6abbcfcce4e2caae3684138d7ca425485` — Tony Stark // The Invincible Iron Man (The Invincible Iron Man)
- `27b773a50c4c428b5e5d2d4a347dde1fc1fcf0a8eb21910cae710fbb8acb1917` — Contaminated Landscape
- `2805ace8485f2a1abe85ffac426ba13d817d9d507d738dee369fd8af542e7877` — Jeering Instigator
- `282065719fa8c99bd089310391739094845577aaaa15d0253ee2899eb02e2e82` — Tribute to Horobi // Echo of Death's Wail (Echo of Death's Wail)
- `284ca69813ed88a215feddfeb332c325e41aca70dbb0743dc1d6d90db75a35ee` — Draconic Muralists
- `28529d190dc3c9c511be7b26320a5a3f49b3da387eefd0e18c76922972d1ade3` — Sidequest: Catch a Fish // Cooking Campsite (Sidequest: Catch a Fish)
- `285798758dddb8ee0c41cb9089563a8400368f3897c6803f939253fd264cd015` — Undead Augur
- `28813106d6448f15b9e7da2932f58ebe915620066b7cf445c291e449ff3ab145` — Starfield Shepherd
- `29654ad2e3c89786ea14b11415eb27b652d2c581e55d2ae28740581ac8512ac7` — Rammas Echor, Ancient Shield
- `29965a387e300449faebdd10dc9584daff9bf6aab5862148e2ac6c95697b9028` — Willbreaker
- `29f166ca37c0fbbad994e0cd718815e8eb0636f9230f8e043da67704edebde04` — Brotherhood Spy
- `29fb4d31d24d0a6c5b5d4bac10bae0835887be5ea9afb3bc3d087447c2fd330f` — Humbler of Mortals
- `2a15566bbeae046fcd56ebe4e80a341bf52636feed30f8a5a001c83d9530efee` — Chromium, the Mutable

- `2a380b818b22601bd3d097bb86e1c471123dcf9ea4c2afb350fb1fbabf7c101c` — Ezekiel Sims, Spider-Totem
- `2aef6fd65252968295acf88fe59dedbbaa2895534c12000fca295565d8d9eebd` — Totem-Guide Hartebeest
- `2b91de3d89f847b0c660a101a71608534f8ab851e458e861c09697fb35436608` — Goldmane Griffin
- `2bad6ec53416f5c342b478de98843d8c44fcf7cfd3fb338b1f1c806ea06b7d1b` — Deadeye Quartermaster
- `2bce3adff29b2903160fdfc761df2493cf1025ac485edb88a70c51ca5067aea9` — Hand to Hand
- `2be6e9f37a370b7fd7add40ca8a9a3d23ce75c85c095d5b52283587a19667b8e` — Doomed Artisan
- `2c22b4cf171318674a03d264802acdde65b9ae28b6fba9c3a903a12954e15085` — Kapsho Kitefins
- `2c3c02cb81a1832ff96a263608761bf7689918303ad2b2a3f661eba5dbb9595f` — Forgeborn Oreads
- `2cc00ee2c0b67ae267ff736ee4eb59428b09a835839c251e0a49abbede1a9dc6` — Turn Against
- `2cfa24963c5603f7e4f58519280ab68cb27f604b8c2cfe67a4c0fcd19f1270ca` — Domineering Will
- `2dde98aeb158dba7ec2aeb1513b27322cc275d1dddb399684359353a3650bd14` — Peel from Reality
- `2e390ce1b53a526f66512118c9768aac8ce9051bb0bb7e2122346762e5d5943d` — Susurian Voidborn
- `2e4daf83a0269d3d4be663316d386646cdbee4776593715f8235c02733cc7bdf` — Scalelord Reckoner
- `2e4fceda4692c07d8e09c56dc149300855d32f23bec04ce1c2566a791f12d0b1` — Kazuul Warlord
- `2e56386dbd564fe14f8c639addcf5f227c1fa063e414d7cd8a95d8a9d2f25b06` — Sibling Rivalry
- `3005e51beb7df965bf9c670dbaddbd84238375ecb374b022bd3047c976a6a9d6` — Totentanz, Swarm Piper
- `300cbc30ac875f321e14a1c64706fbf460cb7f8ae544f250cb74d0ab22e3cd5a` — Omen of the Hunt
- `30337c973e40c07f8683c226681c95f459c95d87cc6c06d2158f1b3ec8f3295e` — Canopy Cover
- `3047ff21223933912fbacc1de1db9ab5abcf4cd548d9e39ba9f46c03358c1e92` — Pyrotechnic Performer
- `30727b694d82da282e3d3606cef0beea1009a4cd25c2962c08d1442a906e2762` — Self-Assembler
- `30905a8ef5d4fb2ba94f5a7d31f7c71a290db27b71dfd5e8bf7b14b552ce91eb` — Thieves' Guild Enforcer
- `3141632494281224e86bf8f74787a766f0fbb7095430af854367f28f65f41d7e` — Bringer of the Red Dawn
- `31429679f52e0a92c5aeac1b88b8a758310b7258581682054078150d2116ce5d` — Polymorphist's Jest
- `319457e9b855c2ee385526df543927ee8812f8e78d0a29a34ac8495b885bf83d` — Grovetender Druids
- `31958cdcbdd3ddd1e2ee88e6fc67e96afbde10bd635ed65a53d034b53227e765` — Quickbeam, Upstart Ent
- `31d905ed5412acc23db0cbb31ff95ece5a740b9a170cada0bdeea873f021c8a1` — Blex, Vexing Pest // Search for Blex (Blex, Vexing Pest)
- `321e8e8e58a6b20e48eb30b55ee076180bbee1aa8eb6175fb76635c00234a107` — Beatrix, Loyal General
- `3296034e37bd0a2fd447d28817c746180c50204a522d28fab24224cb08be0a59` — Centaur Rootcaster
- `32a8c25093856ea38321e1a5862e58aa7c88755010185cb28cf54721349e7f7c` — Gustcloak Runner
- `32c18aa9c508f967899a37793a49510a5ec501cfdcdc11ba57661f04ccbbe67e` — Palani's Hatcher
- `3312f68edde3fe3f8c131ee6cc07fc97b32694a06b60076444ae8f466a037cae` — Vial Smasher the Fierce
- `331c5312e60142d527363c4db1c87ad3aea7804dd491ed8499c6e487071dd539` — Pawpatch Recruit
- `3332f536907ef86d56fff5602a1abc821ae68316d1d4eb389773eef689f6eb41` — Reptil, Dinomorpher
- `333e32c9005adde4b60421144ca6d84cd97f6bb34c08b60a7cdd82019534a5af` — Altanak, the Thrice-Called
- `338957c2a4bb1cb3db487dd611b99f256f79ab199b9b06cd2f04d547429d4f27` — Dominate
- `338dd9e7ac9b4f46d228cbf918c806140e869cae09e94f5e6852fb8b5826d201` — Curse of the Pierced Heart
- `33b9a8c375e5f24e364318b3e13381946c563248a9255973ea183b07613ba0d2` — Brooding Saurian
- `33c0ffb31853e04872a91cf00d14a418f6555bc59e967716d71383e99e4956f0` — Lord Skitter, Sewer King
- `33c95e21b8fda2eeea321956a491233f31a95dee7fb495600f014ba9903e3b59` — Nebelgast Herald
- `33eb411099539a6937bff368273bd14faa32af04e87e8ddd89600c6b04530dcf` — Conjurer's Closet
- `3477cba4e38e42fc839d9ada181ec9c54ec620998f53ff35e2a314ba32d69951` — Pious Evangel // Wayward Disciple (Wayward Disciple)
- `3522cc998dbfa9c76074e0487e6ffa9e542e29a57bebc93e91ddacdb539f0d4d` — Scion of Opulence
- `354098acaec1b500e6c27763c685d23ac0d27740b4a28b558702797236b367a8` — Structural Collapse
- `35c56222f7c14e23df8cc12472c28ea8418eb2e4b1ff7b281691d1aebd2b13f7` — Growing Rites of Itlimoc // Itlimoc, Cradle of the Sun (Growing Rites of Itlimoc)
- `35e36022f6be0d42183b823de72871c84fe493cfbe9c9820cd963d97e00d4fdf` — Haywire Mite
- `360331a219837a67b3c585a878f1aedf9bc4055fbcda10bdd752e457d935c60b` — Sunbathing Rootwalla
- `363bab7519b2f8ad875e9de8d13f664139d8def08facf1b57c45bf21ab0095ce` — Descendants' Path
- `3644881835ed676ff109f5fb20e9f4ecf270a668b03036eb9fd3eb3a6df6d4f3` — Gideon's Battle Cry
- `364ea613fadd7a4311aaa2b9a8f3c2b04ea7f4c56ad8a24bfd618c4a968ba44e` — Bristly Bill, Spine Sower
- `36b6755b500f76902b08c4568b40f03222e34a651985f363627875579f244748` — Ouroboroid
- `36d6f938c02462b752de1cc205ab26fe4e5167f1d3478e3aab86a9f2c62b46a6` — Metamorphic Blast
- `371ca40985b5e838cafa13d5e9730db43ca5b5567a6e85c24ad1ed2490e7df95` — Crypt Lurker
- `3738bd26d1995710674070540fd22047cb6edcebd0a49eafbee5ada50ae52473` — Chaos Terminator Lord
- `382d206e9425582b3c4d3116fa7059dbfcfeff717e0ea5759e8b3dd55a137b50` — Vannifar, Evolved Enigma
- `3889d2d75b3581364ab41b7657d5e59841a7eafa77bf49b5b0ba50a7f999461e` — Eidolon of Inspiration
- `38d1e891b6c1b0334d649cb2e5b453703df0865cadaac63d1a6209b33c0e6194` — Animal Friend
- `38d80b954d2b10a9102c31d1859a22f741482844e0f1315f0955744418a6b0d6` — Furyblade Vampire
- `38e53960c64b9f72402f9cfb70c70688ea1bc4204ff4111a116d97952ef454ab` — Civic Wayfinder
- `38fe6ab2b3e4ec3d8e7ddc5d390d5b10c5c1cf7478300c62a66c6c0f0385582f` — Skalla Wolf
- `393dd6513c772fc542694033d16a0cea606bc599f1a00f9ec903e9b4ea80d86f` — Nogi, Draco-Zealot
- `396ef0833d44c1708f62e60de67b95c63a2198e5a0044d0b7b5ff0127da6a279` — Runeforge Champion
- `397398457e4d71edfeb689c7299e38de46a9b0ada728f599f31ca9af8c3545d6` — Master Thief
- `39b1bee4ea16496faf630d45325966961219d38c0476ff6e6bea03b5d1f59c48` — Dovin's Dismissal
- `39c38ab36a90336f32dfe9289bd370b9360448b778032c8902700c82c693610a` — Rally the Righteous
- `39c6218ba0321e99be465c47a1135a872ba8d7caf563e43e30b77d4bb97a36b6` — Pactdoll Terror
- `39deb5d278af87523a395d7ad88eb82bea8cf7a5a2d91acc01459a2405d1fea3` — Sky Swallower
- `39eb199399f5b48302829baaf807c5c08744037bd51156f6dcdb76c9e0ded02c` — Strength from the Fallen
- `3a0c8a2da67e32fb314463660da1c7fef011a861acdfd4b32e6bc73a8fd27e71` — Lucid Dreams
- `3a2daeb2e874ec2065cfecf17143ac9ff137ce87c0e2bb7f0261f34d3c5df26e` — Reckless Amplimancer
- `3a4e5b699546ecfb5c408081c9cbba9ecd2f9cc933e9aea9cc3f6b023955996e` — Mossdog
- `3b4afd3e2b291876e2ea8b9fc83f2479a94b0585c0ec331260334c23627f7d5b` — Slimy Dualleech
- `3b4c45d85ba11d5c26c7e6a89faea29f091f5c701d21017abf654af5a01d11ed` — Mila, Crafty Companion // Lukka, Wayward Bonder (Mila, Crafty Companion)
- `3bc434a89ff36aa5e06f3228e82c96ad27df4053f6d377127589156c5e9603db` — Exploding Borders
- `3c4c25a78753d95be17201555e8c6baf2cd9a248dd7160a32acff84d32ce0f7b` — Lignify
- `3c542733f762a73fda5e19fd63952654130edaec00922c86f82a0e00cabc0837` — Ritual of the Machine
- `3c99d7603c35305ec87210043d7da1a9d97bd699bd854a6bf0046cf567a0c6a5` — Whitewater Naiads
- `3d614dca5676c2f060d086ba2203ae4ef3b4fff2054682f48bce645662b35aee` — Yavimaya Elder
- `3d7c955aaafcc4aaedd465e990964b25620ab27b5a92d13f5a11b4bec75b5498` — Amrou Seekers
- `3dd4a07fc005dccfe070d429ce4cb539d1fa173cd076a74e1d20976ccaf69e50` — Vodalian Wave-Knight
- `3e4f5f566b3fd4e140e65f2faeb059b87e27b7e5197e34e5b35fca8ce551b805` — Wispweaver Angel
- `3ef393655875405c0717e8df0d10c5b24a19da4c209ead197b907fe8218df476` — Zimone's Experiment
- `3f8aa022c37beef9d674c918289cb74d458fcac8dddcbce5af54707554aecf08` — Chandra's Flame Wave
- `3fb3ac535f0b469ba96a3d2d607d43c6b813f3bbf1eb2bda3b15698006e5b0e2` — Eclipsed Elf
- `402dcdfe43d0662afa709f910c0c427293015f8f570f7af08fcbee2a4f0c2970` — Adaptive Omnitool
- `404e3d8b63cd2d48fa02a27940376d9b148b9f98be3ab5dc618e0aa905fadf45` — Evasive Action
- `40784ad7fc25c127252c2370eca48da82f740f23ee27604434a899ac4a8a17c5` — Cosmic Spider-Man
- `40a1f1793ad1ceda0e29874b22667bf1bd8024ad9ae2f85a66c8bbfd8cbf6964` — Spider-Ham, Peter Porker
- `40b338d14c769bbb7a5289894bd42ee4c58af5f20ba117b3b101f9944ff474f0` — Gruul Ragebeast
- `40c641addeab9674c1db7c0b0e5da139889a05cf4e0f56a28041c00be3c0586d` — Psychomancer
- `40eb94c18af20e111414f66d841e3a968eafc3e3269d8e1cadd1c92e56668181` — Howling Moon
- `4104e6c7ba726c4266046f9990cee4116970bf2d41a75f6b7419b045fe9098d4` — Kotori, Pilot Prodigy
- `41272642712f4a6c388cb8c757225f646330f231c0835b6df14d5357d767bb63` — Eclipsed Merrow
- `41530b32ddb2bb99abaeac05e813fae069eedab4bedfd8ff4e015080263646a9` — Startled Relic Sloth
- `415e5de4236f70522b280c245501a3cadaee18f0b323a03f6fe82f8de35cd7ef` — Halimar Excavator
- `41b9ad29821a992c23e3290e253b7be93e46615641480f34cfaaceea937172d5` — Mirkwood Channeler
- `41f9eacca23669de04235c4548e0ad6acf829512b4544c11e416055be8c3ac4a` — Veilstone Amulet
- `422d9ba7f21506a76baa16ab5bde998048f5235244cd0f48f9feff424321e32c` — Warg Rider
- `425ac62397e1cec012d7966e54a46225761a14b750b67e1bb604e5b596f2ef6b` — Traitorous Blood
- `4278452e8ba133e81981b354816380f03bf7e8d3068b77abcfec392cd36e8c6e` — Hagra Diabolist
- `4280a4a58300b0be7ffa22e39143328f880644394eb4d38a9f8ec5c30176d223` — Hound Tamer // Untamed Pup (Untamed Pup)
- `429ca5d38a92ee6e904286d028570d7acca85855fa4145cfcadef5b45aa41015` — Hadana's Climb // Winged Temple of Orazca (Hadana's Climb)
- `42bc787c6b9442e0a421d0fe80e49ce09c1da8edc6bca9010b577fce8db02b67` — Mycosynth Wellspring
- `42fd82d33e80ea4026d13bd2b656ac3c20a7e0a81966ced431c3da38917729dd` — Timber Protector
- `436ac5ac52958c91cceb8eee4eaedcecee8347bdca541f641e13f861ba8d3f1c` — Wojek Siren
- `436f8f9b1e6feaf8aa9518f1118430a93c1c2842712fa14268143d7b7b3f988d` — Veinfire Borderpost
- `437cde44b436a2fb22f336e656f9ea5b26471853e9e3cf7b43ff5d4336b2fd5f` — Thran Power Suit
- `44316589aadce824e04689bc773dd7991303b470ed6b02892148d2915fdbf256` — Ranger-Captain of Eos
- `44955eeb43554a789e477a913ca2970b8f7475ac49a89a1a99199d04a675cc61` — Gideon's Resolve
- `44b0321945150fb90f54759f9a0192d72a34df354fa04c3218eeb8f74df9aaeb` — Gustcloak Harrier
- `44bf13dff883bb4a67f59d5560ea5e270ac4947c08e82f93853109c861b617b7` — Deathrender
- `455623c795f0cc6745e5d78ed30bb3503f79e366e98e03bfcd5cc3bc42c32598` — Kolaghan Warmonger
- `45d6b4bf4e959e0b148ecf75f4afe7f223c2f95d9fb5198aeeb00c327c78df76` — Veil of Assimilation
- `46113455c0f60c286d95ea3bba06cdfc54d37c68341b989b7fa17f2f01639229` — Thorn Lieutenant
- `4674784e7169c846af1f52d487825e7a2835fd4485abf77c5cfd480b39c695b3` — Hulk, Strongest There Is
- `46fb5e57ddbbb0e606090e689c90dacc41123c527ce034460ac13382ea51cdbe` — Sanctum of Shattered Heights
- `470c1c04ad56e2cc1edc31eb3817b5a69e5ff03691a87eb1f9b46c2b4495faf6` — Trade the Helm
- `471a0a61c4269a10badd47d825264aefe49af40717fdfc9f71bf6b0ae0a0b6d9` — Ajani, Nacatl Pariah // Ajani, Nacatl Avenger (Ajani, Nacatl Pariah)
- `472aa0c7f70d315e690e9e4fd252991bf724ae4e4ccc7cb01b4193fbc1d914fe` — Nick Valentine, Private Eye
- `48161a467fc4432488eb34f3f67cc1b55c2cdd33628d110fa5dc290dfd8e1f45` — Rags // Riches (Riches)
- `484c4b493dfb7f3fc58265b08e8ed9f644aa2d84ba104e4403283e029a0f4681` — Liberating Combustion
- `4861cb8f3d599d3fa9de6c6c5dac89658df493868bc146f4be3d0963b70b275d` — The Indomitable
- `492cd53674f6fa46d875dd1c18811fdb3de0bc22916934e782af5def19501014` — Ultron, Machine Overlord
- `498e6664cbf8934daf7b01bb9d68e19ab786e49c7b022e71c7bf0f2ce6091b19` — Ordered Migration
- `49b47954ba18f8d3bff85c75f25813b24fe2ca4bc4b7a5ad4b22910fe9b843fb` — Shanna, Sisay's Legacy
- `4a003d1641e71a352a2d5596ff1fa9546b1e507f2bc75ffe7c5b43e9c6de6af9` — Packsong Pup
- `4a06df83686546d565a1d64f5fe75b40c2921b4ff601b14d3374091a4fe108c7` — Flash of Defiance
- `4aa5d66f3b1d0dad4f83e4628a6924f3a4cf46cc4ded06b53798a3d766d748c8` — Kalonian Hydra
- `4b2e07cf848202a5baadbfaeaba22640b08c2921998d612b507604b6483a7076` — Recruitment Officer
- `4b69ebaaadb7c658cb2d422524b7a61d64a0583bceb4ef3e202753306e95e5ee` — Rise // Fall (Rise)
- `4b96652dfe1d9c1d68e9cfbe411d585cbda567d158197fb3a8f0d6a8ab6e8f82` — Hubris
- `4c0a3861c18680d895782846d9e8baa2bf7ae39769dfe7e74d787342a2e29d97` — Yavimaya Sojourner
- `4c763de786fcc19c10e4b5ef97616e6c99b5e9100b52c3be91bb205c1895ef76` — Manaforce Mace
- `4cc38dd9689aae3fe8177327478041b9faceb64f06a7e2e0a76e4af10d0f982d` — Verdant Sun's Avatar
- `4cf2baa6ecbc0e26208615af5867c239d73b3d3c2f02545696ca743675147938` — Chandra, Flame's Fury
- `4d1f9c4d26f0e4d3a6ddc1898d624012a07cc22ed00eea6c30ff5cc4a83289bf` — Muldrotha, the Gravetide
- `4d2b036adc40fde01bd8d714f34ba52d05b3d64076ee6982670d9e60f48eae05` — Mirkwood Meditator
- `4d48c4e8c1378dacc4201c8cc0fc262c30156cdd4a11b58a812fa981d3241ddb` — Chong and Lily, Nomads
- `4d52259d547868690db7dce4ff5e3b0f2b2bec1b392506fd33a52dfd3319d21b` — Moon-Vigil Adherents
- `4d7448cee2999c40a8bae6e221ecdd9d739a835abf9f9607b28790b264d7839e` — Environmental Scientist
- `4dc9d7e7eab2e74b97d14136c1829959b56c380d8ff89051f65badbfed784d61` — Graypelt Hunter
- `4de7a60d2a9f7f824249e1ebe0efb70e048f030c5148a9667862fc0d547c6bcd` — Collapsing Borders
- `4df05179f8ccb7abec8bfa62c905c8dc705e8ad18f9ad14b86758b088d82db3b` — Malevolent Witchkite
- `4e3e1d28d52794a9a837a23412bbd03ba951065bc003b9bb43343e582b98194a` — Bogwater Lumaret
- `4ea0a021c8d852e2080493175c2fb92c8bfea434d5b3fa4bb13dfaa91e8e3508` — Welkin Hawk
- `4eca5c574ef0da2fb38bfbb1af558f623888aba6ef4b0768f30741681d964bd4` — Radha, Coalition Warlord
- `4f1432da22704d1b458cf6a34b5c0a3ca2c5132d7be281b1f4fc9a5080a3c0f6` — Kav Landseeker
- `4f15cb13c0736b9b2e897a00d079c510ce1ce2671691127f418cde3ab041e0b6` — Manifold Mouse
- `4f3f841c1f0ca15517f6b1e32c2ddf4a3062e917c006bae28caed1fba7961338` — Briar Hydra
- `4faef1ea0381bffdbdde8819ab35f35f868ba61d7f5219739aa0f944218cc7b1` — Battering Ram
- `50442fb71424557b0506e5a541f4c679ad5b5a1c94a860906ba355d683aa35cf` — Wojek Apothecary
- `507aee9f910565459786186a992c159cb26dc3d4d674540eadf3fc61fb06df87` — Basri's Lieutenant
- `50d5e68f80364a0897cf06425af3e950cc7121acba5fa6b76f1da6da7e887463` — Tamiyo, Collector of Tales
- `510c66e49b93adfc8ecdaec34ae841a917865645b293690ded753df9b36aec36` — Vraska's Stoneglare
- `511f096013ddc06f6a277fb41d418f4f73befa9d89c67eabc71ee23fbd7e47bd` — Tezzeret's Gatebreaker
- `51285e4773b530242b6dfffc3146b3627a3bfc1ab26fec2392fc4d9393a69673` — Living Brain, Mechanical Marvel
- `514de564643340c23f7da5fe3311cd66ed740b7cda4c48f9d8e116885e4758b4` — Lost Auramancers
- `51a8298ab453bc374042c4061e0536f48b902e18ee6dc47849535568af807b6f` — Warbeast of Gorgoroth
- `51d848a3f3924dedf6c9361c32acd7cc15286e618ad8ccd6fa8af9d5fcec06ca` — Changeling Wayfinder
- `5212c3d3cc5ef9cf088011f8135cbfeb4cfaf8807c50b2e248228ae9f6f7f1e3` — Vraska's Scorn
- `528bd9acb42335b0b5a3325e5769a8c010dd9da3aa9ad2defa5bb3c0fe69b674` — Chasm Guide
- `5304c21552ffe0bb18a47489622762b080ca8b6d30f8fdc5e3e53e248f50be2e` — Agent of Treachery
- `53691587528362cd352b0fbfdd5f2320250d931a4cc27344e1c05d1b61715ed5` — Karona, False God
- `536dbbf14324ebc2a450e5bf5e01060be0a1be3611df2033a2743a9cbc9dc681` — Woodland Bellower
- `53ae11cc8a1980a20d21f0410a56b06fc0ab1bc2cb388c084ddfaa35c4565b5e` — Kappa Cannoneer
- `53bba3e2b30eea868ae7aa1331e5b84a6686f4f5541d0af4eb1794085ffbc870` — Mox Amber
- `5426555bd091a7e0c5ccbe8b9b95f7b373f5877762c7cfccf694fac9791d6f6d` — Curse of Shaken Faith
- `544bdb2ce2423992f7c6855a2dd4db6f3b958ec1238864fd5146993cfbbe21b5` — Micromancer
- `5474beca0d7e8d56c74165b750bf8f6af44b17fbbb569775f13ab019084ba58b` — Deepwood Denizen
- `54d898c1d20e02a8bc07f1771391f70e70eb77f51c9ccdddfbafd347a7fe68d4` — Surrak, Elusive Hunter
- `54e2d49e4859b0a3ebd364239a296f13c23ad25b20e937d03cfb1004bc976851` — Angrath's Fury
- `54edc3bd3c43a1fa0ce2d96f71ba128161b26ccaa4c3426b74a7b0532cce4615` — Ballroom Brawlers
- `5520baab30cd70f525961d8a9ad0b24068b4bfb42f52cfdc32d1fc6cb342b67f` — Iroh, Dragon of the West
- `553cb2212935d8cbc5740d80d01302d716b1e059e5da6a240e5ef20f6e40f81f` — Embermage Goblin
- `5584295743024b53d1344dd55bca7acbbbd4f6cffc999a3c81ebc5dcfa9a24e8` — Scuttling Butler
- `559f8421a22708275f1a1e5d03c8f3f186d34855357f6e5b133ebf5f31440e4b` — Ranging Raptors
- `5616f598fc549b2ee250496b3be6571e0a1cc09986e9d3d347fa00c1c8d31cc0` — Kirri, Talented Sprout
- `563fe608a4620b4daaf2b3eff45daccd8aafe952950aabc75e1e14b9b171ef5a` — Screaming Seahawk
- `568c5df23120cdeff10d67957af75c5e3cd959c56ba96cc46b725fc9e19bdd8f` — Catch // Release (Catch)
- `569eb3c2a303f811e48e9ba6270ab760c731825f8bd00826038bb49bd655e848` — Daring Piracy
- `56af1ad133987e15cc9b7adc2d18dfd574c24e947957e612ca92e1e0d9023d3c` — Volcanic Offering

- `56bdda0a4b3d0c223da9e2ababdf753b99e17ef25293ca322b5c52883d639c6f` — Portent of Betrayal
- `56d59944df6d57b8b6f271161368a3eb2b82ff2a42d6b8fb941a0081da4e4d3e` — Selfless Glyphweaver // Deadly Vanity (Deadly Vanity)
- `578b167b1fb23dd398d69f6ce8703bfd82b016fcf2b2b8cbe12b0ca6e88cd0e8` — End the Festivities
- `57d583f531c5f5377295e91be0ab14e6425f474b028d53808759e3fe247a9b66` — Markov Waltzer
- `57d637153f0e587d95df4ed1edeba7e962c59b9d1cf5e3e9e3a60571b92986de` — Addle
- `57f38384e202f2da0f77edfd8b495c3c9fae920ca256602de37e0f6c354dbec0` — Coalborn Entity
- `57fc3fa7ce3e8735582b970d85d2981fd4b6f6ddee6a0264308d27b5dbec3d96` — Glissa, Herald of Predation
- `5825f72dc55632b096c5215449474679293cd7a0751a8358a6fa522851536024` — Polukranos Reborn // Polukranos, Engine of Ruin (Polukranos, Engine of Ruin)
- `584fbfa05a56cf3b173a3a8ca8235d5d0be63fc7dd0e16d5b7865e88ab156a74` — Blind with Anger
- `588b7dca853b4382b95b85b169959e0e39a44fd447baac62d89e0b478a0881b8` — Silverglade Elemental
- `58ce6f84318206b18aa7891a39f3b3334babc0cbe8c80a32dff9b59bfff91b11` — Hideous Taskmaster
- `590c18cf564932638a2e71cbae29686049369c9684211a678c3c83a17af539b7` — Old Hob, Alleycat Blues
- `591f6bfacdc3974265056d20df400313ea0cf297ac4bd7e7c71890b7ddaaabc3` — Exotic Curse
- `5921882f65a463e76ef00bad54ca4a3d5fb26082491efe132464b384361f4648` — Daring Thief
- `596c2b99797fa54d7790a6029ee8f90fc0b5acb4d379962d7162fc02f67d8937` — Fractured Loyalty
- `59759d881e1f0e7f80a90294af1b2067c96119a9f25ea820b514e4b063094867` — Sarkhan Vol
- `598226a1b4e7f5e6e918a9b633d0399c11a0c086dec2beadbcedc7e1683c8a2c` — Word of Seizing
- `59dcbe3977c603fe1ae76b0958dcefae6bdf0717b4523f7e6890728a0e6e1e5f` — Militia Bugler
- `59dd4586e8f3a7652071ed3504bf78ad7c867db455874ba31e0f079bc703156d` — Rowan, Fearless Sparkmage
- `5a598e92feed2f1184b070de4d0f2e57fe004e0d26b810275735ecbf76237142` — Sokenzan Smelter
- `5aa551409382a05bf7b0662f62653754468dbf3a82c1825ab1ae2f8f1ab0eb78` — Aurochs Herd
- `5ab5deffd2eb7ced5ca3c7a05f4a1d99c217b218aaa7e7c20be47b6227b538f2` — Alpine Guide
- `5ab8940b0ecda3200968978b6423944ccc042f1ef9d79ff55cca570fdfa64bb5` — Relentless Pursuit
- `5b2659c6fc26ea02bc46ade829bf9e0d45c0687979b703c38750ad8661fbb39e` — Workshop Elders
- `5b33e427a0765cd5ba37d092bac19b31fe92c2839d1bd455de2d02777fe5fc3a` — Tarnation Vista
- `5b82d112699dfacb8a7c45c35c55551fa8660696d6bbb001cd3231124ed1e978` — Toil to Renown
- `5b8c460b58fd69f27447ba0d8be09f4ae8c2ba5b4c9d442265e760169958030b` — Avarice Amulet
- `5bcd1f6f7c6a2dfcaefe01586ecb7ab3f1923a346385f14a2da90d5300020581` — Fight Rigging
- `5c29e65538826df9d1a1ca6fb556ce50aec05d2f72ed863c5c0abeed9bd9e6d3` — Might of Alara
- `5c9579f1d05c989349371a2afc1236abd130297f6a197962b999732738a02f85` — Incite Rebellion
- `5cb3e2d6b60715995685376b26c29640be67cff5050479901de598cd39dc1db2` — Battle-Rattle Shaman
- `5cb7aef4d1f5778ef51f6405e91cc2e3b09b582676c501a3699bef8588c4adad` — Zask, Skittering Swarmlord
- `5cc2f9ceedff4c1e117cd8d4bf038c55e6cdbe21b8df0076daa6a84afbf3ef83` — Harper Recruiter
- `5cc75622b9f5e63d0ade117e18ae96b61057da9a3b4a9b35d4b4b7a375458d03` — Whirlwing Stormbrood // Dynamic Soar (Whirlwing Stormbrood)
- `5cdab67144b6e1a34500961f221c115629e4d6f9b44285e3bacfd479210d485f` — Deceiver of Form
- `5d2e567b9c567147dd4695d67f5456dd5618b36dc4895b04bfaa50b42b91e083` — Thrull Champion
- `5d71b6e7c02098b985d81c3fbf29d60b3eeeb930be02b4323286e82856efb2fb` — Tajuru Beastmaster
- `5d803f3348ec0319271ea4b58c35f02289b5e7d0158504c56b785d3db61dcb0d` — Domri's Nodorog
- `5d8081c6e8d853a1b9b6c54d3c8b03cf60382773ae6553a269e659f8ca3c740e` — Makindi Patrol
- `5d8de03f0b82f1390d7f399f882f7ca3e1fc940d474a6855a77f172cb300d7bd` — Jinxed Ring
- `5d8ec9e3bc0e9cbd8777109b707403fc1310960ba8bbfd7b1541a0ddfc95eb57` — Foreboding Landscape
- `5dd0241fbe019288fc2e6ba9f577642f9f2666d080eb53158e41ba506fde6e50` — Thoughtweft Lieutenant
- `5e2aeb1cab14aa2064315376459a87a29bb392a0063c588f55649dcfadeb83a3` — Skirk Shaman
- `5e63fd18fa55df8a88a79294570c2b29b27e18d90c74d4f49e1d57cd32f5823c` — Tezzeret's Betrayal
- `5f1d9bf3fbbfde51d8e468ae45d2da06aa326ef1cf2a155f092a91330c3b929d` — Possession Engine
- `5f2bae7fbd20d241398ed9c142365d7bae59684454eac88643ed6efa9d9e1d4b` — Strength of Unity
- `5f42e5d1c181e65f727de13433f0593ab47e4f27a7a2407b84e154cef18b47ac` — Thornscape Familiar
- `5f44c986d900e8add1c39f5b1a8a7b665026604a5b1ada84ba42e68802cfca6a` — Tranquil Landscape
- `5fa2e8ebf81e6aa2f5fba50719d182f7702605163f718a5eac0ceeea101ad135` — Warden of the Woods
- `5fdc7fb086a57884397db404f2ac46682e376162e84f94b968a828feb2dddb90` — Additive Evolution
- `60a65623da9db87bd0982a62846331d2aa1e7be9105a5ca92b9c4313dc3241e8` — Corpse Connoisseur
- `60f1f7f3b7cd43cb6d919d24b2027ff4791a80a8a1f47502b033c5a21caed8b1` — Don & Leo, Problem Solvers
- `615c76ef405de20f36c1d8d136dac3c0f5c36f5b09133792af61554f7e834ff0` — Eldrazi Linebreaker
- `618e390f17b37d4cfd5b2640e8623df29df6c84950c0a78277e8376b59586e98` — Zealous Conscripts
- `6196786e5ae371cb1e3f36770978e83c212f7e386dda35757eabc63791168846` — Shattered Landscape
- `61d88a8de049c6a9da879429b1736a615cc550958cc776577dc50752f6d8b060` — Restoration Angel
- `61f3c37e84866ff46a6c54b4d007280a8a5d65ff73eac290a1102d1b6f48ad2f` — Hero of Goma Fada
- `62d47d53955d48ba12b784447b3744bb08daf49a8fa00047dfc492314cb2fa73` — Courageous Outrider
- `63833a4d254c194ccc4027df05fca73b14a7b1341f05d739f1e65fba8092669d` — Switcheroo
- `639c27a3b4d74801201a39aacc498d188d223f91e7c5033ad11a017f61cc9472` — Serah Farron // Crystallized Serah (Serah Farron)
- `640c0c0885687c1300b08c4be7841f1d6689033558c163c0eaadcac8a3607036` — Luke Cage, Hero for Hire
- `6416885d4888954a18d7493828a2ee1933f2bf916620dc4665d6276b225f9efd` — Makindi Shieldmate
- `64adc32188513785227a2cee0b3dff1dfc3d0a009e38d65818c8939b36539f9b` — Vengeful Strangler // Strangling Grasp (Vengeful Strangler)
- `64d384dd9d6850657f250dbf54c8554ffcb5f6c245e7892b9c48af0ee828395d` — Korvold, Gleeful Glutton
- `6515e6702f4150698d58284fa1d4c4f9db723739b3de58826bb076653fe25d90` — In the Presence of Ages
- `6517c47c247b5f96c07b53ce33d2c5167752873e15e910de656e679f4c9f4270` — Eidolon of Astral Winds
- `652fd0ab649b4cb1c93066b2adfc85d68264deb32e5291699887aaebbcd4d57a` — Startling Development
- `6573b00a1a34eb1d069557790c4a736756856b41d4866aacfc3adb3f62667d4e` — Namor, Scourge of the Seas
- `65ea2002044c46fd1bd0c1a7bc828cbd9812674c6ffd776a7b584c0c20524fca` — Might of the Ancestors
- `668968a69af5974b62535bd08a975e76368bd0eac6baafbe88dcaf6bcbbcfd3f` — Shefet Monitor
- `668ccae4d8e7a7698c69c06310c19a57e939b65a5999f64b95b82fd7505b8c42` — Cowabunga!
- `66abc7e578a5ddb0c3a1e9ed475cd5760b905fdb27a38cd062087b87c13eec3a` — Black Bolt, Inhuman King
- `66cb111d6a9c7d56f818a27a3db6a53b7b2e89c23685fd4df0f758b5fb124284` — Caldaia Guardian
- `66d2a8f25f581da6ad1028d519bfe5f6007b3fba5895ec9092dd4a415e25235f` — End Hostilities
- `67514616dfa98e6d939533df2c0c13079a82cf252175acf5993f8e8eeebb8c15` — Ayara, First of Locthwain
- `6788f386661080804e128c3825ccdc19523febb7d48b7155acdecc3965f781ec` — Secret Identity
- `67bae46c2c8869954b211040ce49f1735568bcc3e814c9b53d903109609e7d17` — Kavu Scout
- `67df5a2c80dfc6ccb43492035ad03967d3404136348fbdf9dde4c356a31399ea` — Ferocification
- `689494db881a262e25c632dac6749b96096488c70a6fa3c8be30667df2054760` — Triplicate Titan
- `68d5affdcab79e67331408fbf1fdb756dbc51220d604574f1305a56fd746c108` — Perilous Predicament
- `690590dde7e4416a9c7bdbac002000d2c9d2ef1dc39a3a40b4bbf9ced8cc84ba` — Printlifter Ooze
- `695d8ed3bf5bf854f39654b5a003489c30b2b063bfa6ee62b1c2a97dc801696e` — Ambuscade Shaman
- `696af6805a9f7f760c62c4a827afae31427867e24e9a3d0a10f51ae8746476fa` — Sword of the Animist
- `6a40dd214e36f0931a42cb83b6f84663a2d1071cfcc7608667e8ea3136582268` — Frost Titan
- `6a96e47ecbcca535dcf4a1119420fb6e561dd8679de06efd43b0f1c03ede0331` — Knight of the White Orchid
- `6acac9fea0beef4b94d7f2b8a274c8378390b86c5121d26db6cd602df90bddb2` — Wyleth, Soul of Steel
- `6b330b78c3f5582080a36eb6ed48936b18a4f46482e3501e45babf8bebcf5b55` — Duskana, the Rage Mother
- `6ba4ebca4fd1b46e124d7ce4f6348d82894d971e2cc11ea5c7b0f39e569740d8` — Battle Mammoth
- `6c44e99c18644f3625df995a83996247eca247beb3dc59b86b90fed66a59994a` — Prophet of Kruphix
- `6ca41be8884d0818d19947d5a78abbf2825f1c7e16a1da1a1960bc0416c71836` — Fiendslayer Paladin
- `6d1ff35cf8c138063113689f4e0a57068b34fdf45385c2fbb56484e6f42cee11` — Traitorous Greed
- `6d257317e6da2c9a4fb4fc85cc93878949e0f1f97a6e57ad0c634577d20fb0ac` — Slagstone Refinery
- `6d272dda5b5f771c7815974806daa40255aee9c863ef5c50eb792ecc8bbfbb5b` — Definitely Not a Turtle
- `6d3130b4b5f979275f4f0c9d8986404c446eb0d94ad60cf5e36863cf1ebcd7f9` — Harvestguard Alseids
- `6d379d72b899bfe15f5f5e38729f6c43f94434c4ef79297422d03ed2e745a6a3` — Chromeshell Crab
- `6d89a998c5a83aed90675bdc9b106868fed4a23398864624f7bd3b1aa042d19e` — Flickerform
- `6db054fe594a1438536cae319d2f2a097e7e380e9809334a0e338f40da4fcbc6` — Kalastria Highborn
- `6db94175c65d24460b18df567cb1ab909fd09c874bf88b673c0949c7ab793f38` — Backstreet Bruiser
- `6e135cb2b0271f402ee9b999f3fcfc1ac1304c7841cd0f2b42bd66f894408a41` — Eidolon of Blossoms
- `6ea8e764fdefb22298f259aa2a826fb8ea4cb5e7eb80fca66442522f8682d413` — Kor Celebrant
- `6f04f824fbdae742511bf5a5f0863037b4ff6429a13258669e9c6a6ca362792e` — Ashiok, Sculptor of Fears
- `6f84f419c74aa43ac7e93b7c8760062080631e51c59b42f31618911efba02b49` — Bloomvine Regent // Claim Territory (Bloomvine Regent)
- `6f956b5f783221cec2f8b7c4f99edf00b0ded72c34ed0cbf4ef852812ac7e934` — Mycoid Shepherd
- `703a653e84489e9633113076c8d114f5eed1c6894cbb48d268d78685ec5db678` — Explosive Prodigy
- `713e047ff8e2e8eb57be211a53ed5512a166f589203c446f10c3e88744aa9c88` — Song-Mad Treachery // Song-Mad Ruins (Song-Mad Treachery)
- `7197004db25e6dc1039c6a008a233e5e1e9d0ac69247af216e6de3cfad422543` — The World Spell
- `71bf7376396b63bb37db61c14446c36ceeb60f584e5d50e23f5e5a492c25cd17` — Cactarantula
- `71f137a65279ae7f8b3fd937ae34027c84a5d7ffb50d5d9b88477a6ad213f68d` — Ballyrush Banneret
- `71f26c959f8694f422e931e778721547606a093e4d236a202350c59cfd026f4e` — Cultural Exchange
- `72333827031a1dafe8e86c75938ee4d6a12d21271edd4878eed2dc766e45f6b1` — In Garruk's Wake
- `731aa3a0bc7868c073573987985f955bee9a0a2b113725f9c69de03db709f942` — Cloud Cover
- `736e5ab998e44cf55b9ece54a343290674d8a6c10604ebebf76be953ff06fa91` — Omnibian
- `73e5baffeaf9c792367d9a512688255a8ff9a90ad1e99554ca335a1c97c388e1` — Bazaar Trader
- `73edb1a854c663821dfa0f21d0ceb2d2092850c8998a2305ddec4d0f966adf8d` — Highway Robbery
- `742f4c5aeeb1a9f0f22b854b642a697d6244fa902bf4aeded157059d6dbcb104` — Aethersnatch
- `7453e31617d5fe9fd9f2e37ba695cffce7a752de289d83de0de2344518262101` — Eclipsed Boggart
- `74b301b1711ed3e2d07517eb111e3bf116214ec9ca2aeb72291041123f39eea9` — Garruk's Warsteed
- `74cfb31186a4ddd5e1ec2bfabebd809922d9fe4e3a488f4241a9f6feb788dc60` — Devastating Onslaught
- `74ee06362ab913930f9facafc5b9a87fa773cac045b097db377242267c807728` — Pack's Disdain
- `75707e9dd515f117a265850acd7a02adbfaf88b816cac4175135799e5b5f62f3` — Tibalt, the Fiend-Blooded
- `75c0da74dd5408b4a1272e586af2f37e491214e877c90cfde2da9abbdf1e9ab4` — Akroan Horse
- `75eb51331032b9efa5a23d8b79c00978f16278bedf5fb91925521159bdda67f8` — Fire Nation Turret
- `75f1346a1f2af2c8369315faccfe5459e62c4eb3f7b5662b5d239d89a5b9078b` — For the Ancestors
- `764cfbea3887b1a60d05f523495ac5b827ecb583cf148f6600fed76296f60e6f` — Archon of Redemption
- `76a9261f609e9ca6fe7b0e6a70904457bb68561ac0ab2b2c1f7ef506f617e74c` — Brightwood Tracker
- `77186b17c6e88b035bc767f008e2cf539541570446a493da756c81fad24dd9d1` — Wild Pack Squad
- `77381612b75b77b3e02cfddd6d3482e7497ec8355b58d7fa3ff2d11fd0c2b4bb` — Bill Ferny, Bree Swindler
- `775e7d3418398db9cc5ea01651fcdad9b34c1f642b8a498a4b0dbc2984130046` — Agonizing Remorse
- `77603104c0a6819ffd9b8bf4c53d7fce8d3451cea0f0214bd9f9fa0f95c9271c` — Kor Bladewhirl
- `77815b2e8175401c15b3110338c58f2ad67cf6ad67a28cd2e0c96c0e0e29e27e` — Ori, Plate Stacker
- `77bebea6c6f0b2109b09bf7b58219f66435eed28bf03cfa0556d9db4965b6489` — Aethershield Artificer
- `77ee5f27ec24170ad05be7183db6dee49bcbc267e3fdc3c9736b4c406c55c8f0` — Power Armor
- `77fafd2b20f0a07467ef349d21a7b8504561489fb6e26e7540892f03d2f63c77` — Rosa, Resolute White Mage
- `784bdf66de81fdb01b79cee4e6d7f2cc690c86eb97a601f7a1dbc21c141d78f6` — Xathrid Necromancer
- `78f6175842749a4439062ca142fdc12c98bf02282a6f065e2cd30c530388875c` — Mercurial Transformation
- `7908b8fe581d6d4ed1c49c11d3e9aed69eedf9589577255e6e075e3cbdc61ae5` — Chandra's Firemaw
- `7922430d0c8432f1c3f6624a978ec0f793dcba28628fd7d24914f0a56716b214` — Wildvine Pummeler
- `794fd1012448410fa8c2397a9bc243ad48d3ece206d6a65cb3eb9c01dcafe1f2` — Crag Saurian
- `795b6f64b0c842e8949dfa9fe3afe55913290fb7f2a5a1d5d23b09e964bec980` — Felidar Guardian
- `79bac0bd6b09f7f17aecf994f232247cdfce3f224961b332a62bbdfbdc0b1220` — Entrancing Melody
- `79e029fa10f3347ac8eadd1c3ed83d47c4c6863a412bde1aa7f26c7aea84db8a` — K'un-Lun Warrior
- `79efe7a772b6f7fbd6f04cc685537335a34cfa87cfefb7fe6a57a9a7f7164f73` — Jace, Ingenious Mind-Mage
- `79fac4b7b85b7e49aee0dfdb8247df31291bd76d33c7e924c28dbbbbff31c3a0` — Wandering Stream
- `79fcd99e9946e63d78860e169c1844da28d167e163acccc5c3d879b28d4a7ed3` — Quirion Trailblazer
- `7a1ae25c82ffb57c94b6a092b54fbaa9cab8517f2126a8cd81ee9c84f6d023c6` — Spider-Man No More
- `7a63bc6884fd63e5f5ad0c16995935c3e97a3f52869c5a095152be72ee7bbd4e` — Old Fat Spider
- `7a6d2ced3fe9c60c4f54ba898c9c378400490069f1e74571c2db5d8f6a2f3d73` — Gargantuan Leech
- `7a9ff1d9a306d54df435465dcd93c71016aaa6ebaced4da1606d284c412025a9` — Keldon Overseer
- `7ac162dcc45766e49d03a335afdc41c96e19818ec5e7cebd249fb39b9096d7ca` — Tajuru Archer
- `7b07233db7be49ce795784bed74ffe3cc10d880002ef4214f81afe7f232070e5` — Coveted Jewel
- `7b6d8a4a827076af775f1a2c8fae3bee6f94a383ccd1d3cc41eb423b79134dee` — Awaken the Sleeper
- `7b6eb1b34545f64e55de65370aee5c81f39efdee1fb7aa8eb8e49cfe2907540a` — Ranger of Eos
- `7ba66cb1d5e9ee21335b57cda2e81115df6d61f731e330c7d803fff6994553b2` — Starke of Rath
- `7bd392a7848d4e7b8400676d2bf01f1cfa217fe083a7802034134f8363f94184` — Shapers' Sanctuary
- `7bf610b8ce21e67ba188dcef31a284173c2b45fdcff1ca4a23bafba1403304ab` — Stoneforge Acolyte
- `7c61fb5b730b2731d301482821ae48eab3b8c472a439c7b47a425aaea9934116` — Qasali Slingers
- `7ca3a54bc4f23d2ee416274219079e3f1744b05b0afaa2a53354fa4aa38a5b5b` — Tajuru Warcaller
- `7cc8771a40a2188a1781ad53afc825e8ebdb35b13dc09c320eeb4610c146cc9e` — Hemlock Vial
- `7cde8a4114557d5bf102e23c8aa3b27089a7ea76cfb30188d3641563e5b9cfde` — Retro-Mutation
- `7d30e12b07c5c1831cd05bb25676cbf63fbde9005ab629daf2c4f6446359f3a9` — Tomik, Distinguished Advokist
- `7d67045f22a186d38aaa8098b0e949bb5e18d71f409faa0dd8c3bc645cf77875` — Runed Crown
- `7df89f6a0dbd624d1c932dbf1d6e7bb9e51f030b0a9ac6e1ad99fa1265ddde21` — Confusion in the Ranks
- `7e12660a61c8f17d3235fc75f2573eebc16df62477fdb5f9c4f83e48525c7535` — Wurmcoil Engine
- `7e6294510bfdaa7b5ff576294e3a61a100ed56215db80045115692e1c45fe34e` — Midnight Entourage
- `7ecdd32632b54b102168354bc289ce5f053671ae37c320ec40cafce1a56a155b` — Shackles of Treachery
- `7ef3fa7d8e05846103d1203ea88459f1962c915dbc60e88c5bd1ffb4cb6c9698` — Demolition Field
- `7f0acb594ae04f1c5018bc31d7c8a96da8cd1beb05f521a0a39eb37227ac4bf9` — Herald's Horn
- `7f4032c51f4913710e32f76f8a894a44ea441e1cebfb40caf04d7c7bb70f47e6` — Sparkshaper Visionary
- `7f6fddc4326b5de084fcfc54e12eca91a25fce12aaa88ef42deef71963dbd24e` — Ichthyomorphosis
- `7f7cd6fe1ccd0110c78316bb6a2de4877b3d6c32e1ad6d00e3ca7586fbac1d97` — Tectonic Hazard
- `7f9854d50698143e64064e0129a18785d016ac8083599e4a8d995a162285aa79` — Manabond
- `7fd7a48c13ea6206671163e50d6c70540c3d5a3ac2be0a51ab23d17c3643c4df` — Erkenbrand, Lord of Westfold
- `7ff4168c81e5eacd91700bc99509de0d5dcec48c178e9dedc949d51ae7030d40` — Tolarian Entrancer
- `8042e170f144ec5d30b736fac0a1676f006f392483e013a327025c0dc4df8bbf` — Leonin Vanguard
- `805350309161bbc39825a7b2dc02f37f2b6c821f56883c6fddd6fc98fede90f6` — Rise from the Wreck
- `805d3613c348b97776c9d6c3ca21c4be6b3abe1b2664223afae7fef3698b232f` — Daretti, Ingenious Iconoclast
- `8074423f1529c8c23c68c117d52459e1f23e625a3bdd4e5090e5ffb48cb64333` — Théoden, King of Rohan
- `8077d57171f29af74e3f8c887a96b0c8b21db204f4e6cb88124390bba2b81143` — Sarinth Steelseeker
- `81288286a15b4548eb233cb3fd0a0ebd03aaeb04221747514309baafc03514ce` — Legion Warboss
- `814353e0c0078eed0d7a581e582b43409ef2f6c7f269211d5423c12462efb853` — Nylea's Colossus

- `81a0fc1fd1293426dc6da28ec7d59a25107563d14492b7de05f13dbb4dc40b6f` — Fugitive of the Judoon
- `81d8ba375115db4b51604637804e982f0ddac8202e5119d0ce71da89258a9e01` — Act of Treason
- `820432924199cdc0dc4eded8339d158c7420959f80f5aad0b9dea16c9b424d1e` — Sunscape Familiar
- `829783d71929cafdf1afe8c4529a8e1afe0307dede337719745ff64099530735` — Slurrk, All-Ingesting
- `82c4539d9521eb237f96f3768265a138febe429dbc3397c92d83a862d6866b61` — Perilous Landscape
- `82ff51a4868ce2f7915d339a7258c763386a2b3ae60f0eaa9e05fcd7ab3983d3` — Gunner Conscript
- `830ec85a085503847c6eecade555be20e3d26b32f4e67fb26f6bc705a38c0634` — Nightscape Familiar
- `83a70afab65287eb246aa8bd9305c524e7419ca304d06bc09438c5944aed1bd8` — Djinn of Infinite Deceits
- `83ccbef11e4dcd85d5f17181abce99b8c0a46802dfcd74f20f30cf35177ef8b6` — Ajani's Aid
- `83cccadea63021a21f13570c8f252c22fb0908944aeffcc8255d947103cd51d5` — Perplexing Chimera
- `83db8192308810ca6bf9f8d9b84d2e73dc6c68b11179a6c6304b3d3a49fda456` — Occult Epiphany
- `83f54f22910b42b5ffe2ccd7e9c359c76b7f5c34c37dc578ab91b0616d13a55b` — Essence of Orthodoxy
- `841c03bf85bff8071e4dfdf13afca70a095e87eda8ed13182e01bfbd5549f1b8` — Farhaven Elf
- `84239901f8dea8cc65f57b092fa3b9778a32770e904e5f0a99408c5f53794550` — General Kudro of Drannith
- `84283380cfaa66df2637fe09be8edbe61ba99cb42d5601247c6e362250969b40` — Traitorous Instinct
- `8445da2df40d7670ef9b345e8d59e138d9ee3090fb6f98c0a6be821ef505f585` — Shifting Loyalties
- `846e8fe5318ccae911e8b17f7588e211b30570701de36b2d14c1ab250e456204` — Trickster's Elk
- `852f8ac8ed727263f5495b01121414f34092c515168fa1180fa49431f50cdac0` — Icon of Ancestry
- `857ae5301a81468f26dd5b4b58f9112890db6cd8cd42cc982537969a3337f9a2` — Mardu Woe-Reaper
- `85bc56f21074b0dd120dfb40ae8da1dc00640314726454531381f5da7e023530` — Vengeful Bloodwitch
- `86880e2d8308147ce24293746471b3a068651e742f248da2a5e210cdebf57d61` — Hunting Cheetah
- `86898fad164d71054a171aacd069cc942ee4bccf84e6e4f9519b8a06a0bb51b8` — Benefaction of Rhonas
- `86a2f68e7fa08ebb7aaf43369c263fd5c45ce3be294e2e3996ca2f453f92684b` — Wretched Throng
- `87245de62fc82a5e65a6927622ea63be1bd3f86b70365f15539bafedf1dd6cf5` — Duke Ulder Ravengard
- `872e19c3f69a6cdf8c67b69beda9d50873149410b319d71428e0b3d83a30fd24` — Wolf of Devil's Breach
- `87a191d2a192ba59df66e6fe81736c8079b257a66a090d71d33db8d36b97d20f` — Bess, Soul Nourisher
- `87daa8fba92bc951f410ab7d0ea63dd4e0fdecbfa661f89f1affae27ff74339d` — Doomwake Giant
- `88375576b61a7173681db305fc887917da62c97b2ea4a6cb69cbf3ffaa164a49` — Coward // Killer (Killer)
- `8865ec0c2936628972976b4028032a8d66df11f404a921b23a9f969a9228e243` — Dreadbringer Lampads
- `88b125c0febfdb478981c7afde948ee5055a9c56cfe02f556e860f756181256f` — Grand Abolisher
- `88ce83dbf29704d714e11e97907e836df2b4076d66600bfbb0e9f137bc3e5d46` — Tyranid Harridan
- `8905b5a177ca807caefdb4e3e0b03d469aa9b75845a6d85a387c9eb206ef9e11` — Threaten
- `89389c2b5730caf1395af534cd24d8645130c1882cdfb8a6251863a6170e0824` — Unwinding Clock
- `8939a3aa0707a1a363cd1b236a688e16506e81b384c90e36ee3cc01379f76cf6` — Avabruck Caretaker // Hollowhenge Huntmaster (Avabruck Caretaker)
- `89678019f14cc62c8778ae8a55dd83a4ad20371610ffdca3ef3e968568824b24` — Twists and Turns // Mycoid Maze (Mycoid Maze)
- `8977712196e263b41f0d8317f2e6ec0114b4b8761f1eed765d67f7e86f234a02` — Shield Mare
- `897aa91f02dbe6ced9cd5adda45a779e9ffbe129cbe2409525edf6e2d624339b` — Mantle of the Ancients
- `89935a333d636911d8d10f3842a7c3425bbd09c8512789e23160fea5125cf6ee` — Myr Kinsmith
- `89bebd2909118af3dd58489062e0ad463cfa4c05a4afadffc0869ed8f770c636` — Vengeful Possession
- `8a03b3391cbd473799285f29eb399634a6eb39cb41f6163824e5cb097bd39fa5` — Brazen Upstart
- `8b1a4f0ebf15c0ec9e2fdd1dc757a50813dc072d035ea1a4eb97ee0a78eb1c97` — Tapping at the Window
- `8bceea821cd5ae5a5510533f14d0f24cd6d8c8f62475cd37ab8fe237d425492e` — Harmless Offering
- `8bcfb05448bb4993860464c3df9edd7b3288b584bb2d098aea909533e806df7e` — Overtaker
- `8c2506f6467b5da69ab380aa4a35d3f16b3c6fe33877034d3e757a6987cac31b` — Shield-Wall Sentinel
- `8c51083a0e1da3e064bcef212471f8d973a196e666356ec9b5f887bf9b2a8a64` — Loxodon Battle Priest
- `8c5208115a67729b771f3ee9245d77f0ea66cc767103955a0f427e78cad13d56` — Ashiok's Forerunner
- `8c7e98486b403aac00fc0071d937b54ee40b178fb0115288ed5736704404b4d3` — Contagious Vorrac
- `8c85811f97f09bbb83f3271dfeb81d71955f818c28ac085914b59cdc8f724940` — Bloom Tender
- `8d0cb086a0085697592055f3980e017e8f3c532c0886477e405f2704dbbc2727` — Arahbo, the First Fang
- `8d26e45f532160aba456bfe234bedfb511672741f71ed4ed935c3817b8c64868` — Warlord's Elite
- `8d58498a843c95bc063f74a8b9ffa5f826acf93897d590f658512f889e488348` — Twisted Landscape
- `8e0835e3fc34e2b51afc52c53abeeb9f144331d2be969a133f00cd5baeda8933` — Nimana Sell-Sword
- `8e1ac36b730c0943fca3cf929de98f2a5b670e646bf0a8b2d313f12f67ee5789` — Shielding Plax
- `8e43255efe6dac87598b0ef41677e0975036e38444cfcda550902bc71e205702` — Planar Despair
- `8e73ab01245d0c490a20460b67c0d06a46380ea59c10690d0af86ec3f50aea31` — Ohabi Caleria
- `8eb914b352d19dc0d7225b2a9a51c50eb3a18fbc27287e6c12d3314a96b77de3` — Thassa's Devourer
- `8ecdbab4d938cf197d9e261ec74aaba7d2ea228664dc3b7431de679a1e331602` — Sunder the Gateway
- `8ee17452f6a7edc93bb7bfc156a04cd367fbce0042eaa97bd15091b0c19fa3e8` — Relic Seeker
- `8ee55eeecb78c5ab25bd31fabe92ec873d40b393e347d006c5a7ef1edafb549e` — Training Regimen
- `8ee9b3a161ca76725999de50a73511b8c88989d1316382d25572ce43bdc4f9b5` — Tentative Connection
- `8f0868ca5eefb2b7bd3b088b066290f82ccff516dc401e7cc605a677b192e025` — Invasion of Ixalan // Belligerent Regisaur (Invasion of Ixalan)
- `8f09200b8dcedf24b3068a94d8f70d22d2a2844d998f52f6d697673bf48cf503` — Wojek Embermage
- `8f0d79b5c53cc671dc29f45d2cc3a8c1e74807d940b09f0855fc1a521878b997` — Cruel Celebrant
- `8f69977965d31324ad174ceec3c34d36f4d003eb3df0bcf04481c3511afa8d8b` — Valley Questcaller
- `8f800fa9690a24a26ea260f32707cafad38c69b3d73b0e66cd2a7bed472259c9` — Stratadon
- `8f95a9f68eee8a125fe0521b1fcc46d70cb461ba5e45991181641cb2e130c965` — Goatnapper
- `9039bbb9824dba1af12fa29c581569853372135478907025097654632e8a40ce` — Kaalia, Zenith Seeker
- `908c3782f24e9b03787397187e33657c9c34f58d4613f40972e2286bd7480b49` — Aether Tradewinds
- `90bcf39e64c8e421b056985cfd35bd7193a3d463ce204670739627de48a4d73e` — Zahid, Djinn of the Lamp
- `90f81e9ef365b0c2e763e342b471513ec7c12921d417f18155739fccf9d50f08` — Magus of the Will
- `911dd69826351e584cbb2d78c22262f193746515d8ab00241cc64703d2218906` — Mordenkainen's Polymorph
- `91f3b73f4581b97e99a76d3c6069f3f68d38144253d3d1390b4c994bac2ba050` — Thalakos Deceiver
- `92163dd168b0132b43317b35f8f6cdb381dc803a5f30589c8a1e047c165a4aab` — Kalastria Healer
- `92199151df911d0dc72ab74c3206640210e61117ecb49c60a4c29c5f1c530d1a` — Quandrix Cultivator
- `92461efcf112bebe87cb0fc1f83294c1bfe7ae5b449a35ed3f1a229cbc4cdf29` — With Great Power . . .
- `926572ffb27b0af727512c9cd5477898be20ec8763c0efe262d34f3bbe37f635` — Temple Thief
- `927fbc7179a0b4076bb2c580324308b4021208659dd0ffc05e9a2976e262c2c1` — Cactusfolk Sureshot
- `92a8eebf0506a85d17e8623743d2304578a7e9e9f88db6915c3e70fbce8aafa3` — Flayer of Loyalties
- `92aafc8c58c61fe253dfc52671e5ec1d7e4e1bbd6b779b5ce21b81f10f3daa1a` — Gahiji, Honored One
- `92b7afae689503cf11e8eaeb64ac92cfb575364a7f0ab6f929c25591489b0bb6` — Panicked Bystander // Cackling Culprit (Cackling Culprit)
- `92cd7d6e34d9bdbd31d5442bcfed597b775ade6341c73a53e60d0d9bc8c40b69` — Primal Plasma
- `933d48f0f18c89c6aaa86dd13938637164c97f9009a5089a77192ff31fa4f7ae` — Jund Panorama
- `93738282cd31e507a40324254c9835cae33228da3377527a0ae19eb2691cce40` — Herald of War
- `937bcb4350d3b8376f44e37109f7ed6d670136d16953f83f08467601b3e2b14a` — Ral's Dispersal
- `93b50fcb0d6490ff5688446ba5d3afd24dc74358cd9b2bd147d24ef6225a1a6d` — Sacellum Godspeaker
- `93cf3f1a0555f2fb4d65384d30cc716b0708cb4a33c69dda9d0d11b3867633c6` — Pack's Betrayal
- `93d45ace2030e93472c83c8159cc59300b334120d63039d86e976a67af453a30` — Turntimber Ranger
- `9489a139b29b42466e91eeadc8483448ed49e5dba3a5653152e8a5b99fc7674f` — Contract Hero
- `94b39a28d39e3ef853d71e23b31dd2fa98eb9af5a446c1d19b38dd386eb9fb3b` — Spellseeker
- `9554cb3389b267bea8124f02e6d1c44f9e2bf4144076ea22c136ddc50aa91b14` — Rime Chill
- `959762aa3104118e34334eb68c02ae95c132526bda53d3b81fe52199716e0cb5` — Mark of Eviction
- `959ee8b2dec7ebaa76b26cb03cc4ded58112f3637d7ff1cdafffa4809682a51c` — Trench Gorger
- `95aee109132f39b36d2ffe4038553fdfa4796f9522818dbbcfd94e51644f14d2` — Dowsing Device // Geode Grotto (Dowsing Device)
- `960d64febd032dc6d5146f330bdbb564b1ffd4729749ea0315758f0c1c22956d` — Venat, Heart of Hydaelyn // Hydaelyn, the Mothercrystal (Hydaelyn, the Mothercrystal)
- `9629493f85583bfc74def2e399995cb7c0afdc0132159db4053c24d02bbdbdcc` — Bant Panorama
- `96579ca1eb032affa93b7a43544c66f6e875e4e57629c540c310aa5af1a952a9` — Role Reversal
- `9678d9921495ac647bc7fe49eb286b2da48fcb7c24273b21a6bfe26f7a125052` — Underworld Coinsmith
- `968b38c9b7b1dcbee8c42866ea66236409a7ac8edd5b57b4cad114c6c43cd719` — Thorin, King of Durin's Folk
- `9693021c9a88d47db0904b4d2dbcae19a3d592c6d8e081d7ecaa85d0a1cfeeba` — Rocco, Cabaretti Caterer
- `96dcc2a89352f8037abc842e265a36bf52b7810c6e8c093178acad8eb027c46c` — Waterlogged Teachings // Inundated Archive (Waterlogged Teachings)
- `96e9c1410c0bafab7b6124090f2327f5f43046b6e8da1328a704e1d05232634a` — Courier of Comestibles
- `97c17cea47d5236f655f717a9058b7665699329c4465c8e0110af00a070b814c` — Soulherder
- `97cb182094fde1c2b021c4402d1a966e394ec24005cc0f5febcfcff86a50507a` — Hallowed Haunting
- `9885a2114d1d1c0972f436a3df310c6470ed41758836385c8867030cdbf5a102` — Conquering Manticore
- `988f0b0b85863c1ea05281403931f2da6d041f6c55e103e3853dc91818e4afa1` — Trophy Mage
- `98a57bb208cfb234b66d9a8ff4479b93a98f092064849949d6be020386dcac9d` — Iron Spider, Stark Upgrade
- `98aab3a3dfb34db52d5a052e43075e1afff1e173796f7deace230848f0ea1e82` — Elfhame Sanctuary
- `98e1bc2ee0bc93c6f6600dd395404a29bad991b25882abaa7e56437bd65a9679` — Seedguide Ash
- `98e9076f2b8649dd31d4ce4a17ef4de48323ef42f6efa526801a0e3d9ff83c9c` — Aura Thief
- `990f744eacb10b01d6bdf9ce6ca5ec8fd87d3b60670cce7883553aa95b15cdff` — Momo's Heist
- `99141e59a86ecf3e6da0e124a9ca7ea757bd90b9691f624509399f522c151fca` — Ascendant Dustspeaker
- `993486d236cc83bcef8a32652b40bc48640c47da72d671d89e52bdd335b20523` — Invasion of Muraganda // Primordial Plasm (Primordial Plasm)
- `99b1eca4b48c9b9863e6253654abb84422a42d5991537409cb6218d97e19fee5` — Akoum Battlesinger
- `99c750b872e88887a7ab0f0943b22048a959f2e5050580672801ba77f43aaf16` — Wolfcaller's Howl
- `9a9c092dd9ab25d0aa3525928977a5748be770d4ec9ee725e3b0aeeef4501d4b` — Sleeper Agent
- `9aa6ff020f9e7df9f05e04d7381afc89ef78c76704b30663751ae62024f6c067` — Zur the Enchanter
- `9b3f0dcdac08d28ccc2966d92373fbf28658fd97506bea1b083af40267c4bbd1` — Lantern Scout
- `9ba2f7370fb74d9d502f3d68a37bc38fcb7fd8046f78fcf7b6ad26715ce155a3` — Basri's Aegis
- `9c00a72ca33c983949e9f134a1d56a6a177cf0d307a6b183105012c38e065710` — Jace's Ruse
- `9c194f5ba0c9158ac6e9457fe0255cd715f0e023cd4c25adfdf2a42a03092576` — Joraga Bard
- `9c1fdcf8422d88020ad5a5fc9f689d8b7836962bd9c40c74f11390d364e1e554` — Seedborn Muse
- `9cbbaa27e3602d735e7817a438009065db935536af9cac2c54452b90d131745f` — Space Marine Scout
- `9d269161dac8b828db12db2fcdb69b9f37ec81461bf50d364034bcf9be97354c` — Umaro, Raging Yeti
- `9d3454adf9688532fa67c8b52d61a930466ec604fbd0ccfec77e2b5de5958ba8` — Dance of the Skywise
- `9d6846260e7622c6b1ade4f298f664c62a76ffb088284176b223ebe8ea692a23` — Strip Bare
- `9d6b498109b47874b5f198331ff34739bf7e049a0c286038371dafccc9528c5e` — Exotic Disease
- `9de0b2a0278c55cf0868b5db91d3d7f100819a91aef061f976f0a1ba54abf4ed` — Jodah's Codex
- `9dfbe23fcdcc0816002504affff9a45b689531a93e83c14b4bd24c79d6bc42e3` — Eaten by Piranhas
- `9e213f404f13f79dd4bfb78de26d40fc5088b7e46b2e0ffa95586774b3f04fb1` — Wizard Mentor
- `9ec6fdf7509e68ab842466d8d7ea33f28aef4d5389b48099c6fb38944552e923` — Humble Defector
- `9f2ac2ee3247bd8ce675b22055e2243f44058e3ee5edfdf70b2fd2a1db971130` — Brighthearth Banneret
- `9f44b79865683d05dbb84c9fcedebe39dbfa85822907c13dfb11e99c20ec6e6b` — Alexios, Deimos of Kosmos
- `9f46658eeb9ef94f107b6b162a539f4c0745dffbf5a0247236baf915528cb04d` — Cabaretti Ascendancy
- `9f6905055f82498feae40bc091a774bb514c72bb0a9de1b08942bd98c16a52bc` — Keiga, the Tide Star
- `9f6b43c797e9d5f53efc6f84d376b659819e0f3192ffd899bb07cc3daf89bcea` — Soul of Ravnica
- `9fb08ca23608d60de905a00836c08a6676ec59949cbf9166b1e354c5d029de17` — Kor Entanglers
- `9fea8472562ac4ed0cb454b7e67d4ca21f9f532a8a49c905eb1797f1c099878a` — Farfinder
- `a02d4421641a3de80e34c6e9c1c3a0554c190c5903518447c22166e1c2246048` — Conspiracy Theorist
- `a0735b911a57b12c2358d160b89aafacaede64e2b1b047eea99be7e2a4218925` — Frogtosser Banneret
- `a0c8c7c22bbfb64b73428592211dd9dd16f9d823588150c184abc13784878fc2` — Viridian Emissary
- `a0d87593f029d3d5e9132371c04256efee9c96762a60a06ca378f643119fb9bb` — Peer Past the Veil
- `a11bcbf58bf81733ae47d31e7ac14813d59955652f8bc39af426c8142ddd5125` — Kestia, the Cultivator
- `a164627b452914cee3e87b58ea339861dcb3ef1504512d7da3ec9649e4417e01` — Silvergill Douser
- `a184ae9cccc6c5b56fa7e9a7c037ef12f210b1c784b7ff5c32953f939e9807f3` — Guru Pathik
- `a1b7a97ec24a7c74650e69e545e8f251b98b629a1a7060d838f37f7da4a725f2` — Snow Hound
- `a20df02e1a7b4f3dffc6b7efd8376c1b4c00e85ace78cc408dbe92c1c545581a` — Odric's Outrider
- `a309ee41921f28c7f610ef663acfab20ca9e83783adaacea0ba77e91898aa3ca` — Risen Reef
- `a32ba22d3655383b988bd9a867f529607aaa0518e2b4e862b15b8d9d6a2e2117` — Thousand-Year Elixir
- `a389cbebf2f921f1c3948c0cba12417a5acecb25abb3d76aa792f4840f1ecab3` — Brambleguard Captain
- `a3a0460ef252235f21f65349985da0ddbd783910cb2ea2f80db36b795ed625e0` — Nessian Hornbeetle
- `a3d661a74b35f6abad5d308a132c01c6c1c265dfe3cbdf2ce3e971bcc5bbd081` — Amulet of Safekeeping
- `a3f04d69bf3c393bb037dc6c006261e8c764d869f3a336a5801254e371fa89bb` — Bond of Flourishing
- `a453e22e403bf3ee7bb745a356e48622e879fcfcd38fd87170c7480d7d4b019f` — Malevolent Whispers
- `a457d4c4253458ea5bab1f67cac89250470eea789edfdb9b365aecd8c98154b7` — Dazzling Theater // Prop Room (Prop Room)
- `a4ec320a0f0a60a5166299c48223263de4a982a9a94e4f454252270adf5355f7` — Reckless Stormseeker // Storm-Charged Slasher (Reckless Stormseeker)
- `a50369fc8d1afcae960eba6d2a4e166f10812e3a3cd340ebd33df470e72ba7ba` — Goblin Cadets
- `a56ae422da3c1e41d5b788b634a0fff2a89ae5f58177a67f6f08eb5e65212947` — Aladdin
- `a63480812acf6a00c051595d5624cd69216488da6ef98c4c264c89127a0f6433` — Mind Transfer Protocol
- `a66a0396dcfc8b98e87cc1b562d9a6e42bae03089a8e312ad32a5294cb5a3e11` — Blood Mist
- `a6d47f9791cccaaa7e2f03509c1ff0cc8730b8b77df8179de52604f5734ae9e6` — Weathered Runestone
- `a77dea31ee44f2897bff47fe636932268048e6cf6550a736621e2275e80fc006` — Mindwrack Harpy
- `a789fcac4168b54ce2a24142e436e63914ce42743fc9ee3d406c1f79345f5a94` — Avabruck Caretaker // Hollowhenge Huntmaster (Hollowhenge Huntmaster)
- `a7ada0d6093b2750b5d949e14efaaeb950c7518bb0fdc049cba31ebb7a3215ae` — The Shattered States Era // Nameless Conqueror (The Shattered States Era)
- `a7d7c5962b0ad7e8f1c2ff17c12825bdca70b4cfb692c55a0bb876eef1e2b484` — Kenrith's Transformation
- `a7dc6f0c968a01f5eb5d31437730dbb909ab6332ba449c6005af88ea9607b9b4` — Amphibian Downpour
- `a7dcfe6e281892522e4944f62c0a846840da117720c5687fed7541a445dbd70d` — Riptide Entrancer
- `a7e1cef2abce5fea296461bf8b0fe30a7205b454f42f7673e7c140f258b51a52` — Demon of Death's Gate
- `a80085a91bb78ddc02297036af75b79f9f813a7648a38ba7a4f2e736d8439426` — Firemantle Mage
- `a8242394e24bf4f3491fc7b343a3c5c59f7ae5b2327c320c756a7bb25c5a479e` — Valduk, Keeper of the Flame
- `a83a6a2c989fa279b362acb1727708c34b3453a4346f99bb48d640401fb6056d` — Kefnet's Last Word
- `a845c24b7acfd9768e3cff9a10d23b72160e743421e031d865e554c264393ff8` — Master Piandao
- `a8c638ee20e3cc83e54713620592f90f7cd6934c5a84f2443443b98d01b6705e` — Reprocess
- `a8c94cd7e2380e0357458c5f13309029db2a44b2fe5cb14c9a23e342f6d9561e` — Genghis Frog
- `a8d40d965e7d5020058385ef78e9aabd57d6c4741b0d2daa555905c5cdac3eaf` — Samite Censer-Bearer
- `a8d79724b06eb1bde78ec331ff699b6dbfd9b698d92d403b3305728a13509709` — Frost Augur
- `a91f1c2d8ca14e51e4e96054c3397945b7c9e7da863246de6c1922164ce5f53e` — Lumbering Worldwagon
- `a940097ed07dfae3a8c1c0dec81cc35236312466be65c27a11f5f63f23975110` — Gilt-Leaf Archdruid
- `a95dea148dfa21b8bb0ef0c7e2f1983c183c6617614bc7258398469e1e6b637c` — Donate
- `a9b9769f545b140e6d03b6bdca3a67e9b5e0bf0e1b05dd830492c46d4efaf0e3` — Runic Armasaur
- `aa0d5125e1de367724f17fd1121fdfb7d6b6034ab4072e6459eb840763b68765` — Ulrich's Kindred

- `aa590184316adc6fcc307b13a84ae303ff543f1e427f4f97fb29e407bee1b8a7` — Rise of the Hobgoblins
- `aa738f7823d797a047d06ba0eeb195ebb2f11b6ea2c1245dfc6d75779730946d` — Shrewd Negotiation
- `aa802a4beb0253dd5923f294b5f96ad91710cd23480e01772a31ba8cefa85ef3` — Skybind
- `ab20ae85efdb00c9221cf5111a9f5a22877eed930118449f51375bf933e224bb` — Adventure Awaits
- `ab526ec584bac78c6f364ec6df30b9d1494f357d07053dfa99291b69421a099a` — Hawkeye, Trick Shot
- `aba38453efad2ea0f9cf51886de5342026a021b44f38bf926a75b3c5e31e98e9` — In Thrall to the Pit
- `abeb0626f29706874cadbf430ad6b1f95a4aa152a19375789b9caf4d2b18f604` — Molecular Modifier
- `abf7042fdaac003226a78f4a7d4323f667fe60cb36907ced75a0f7bdd2dd4de8` — Atalan Jackal
- `abfeb4f39cf3c0d42aa143bd435cdd328e31fbf483bdba8c349c37333197fde6` — Guardian of the Great Door
- `ac1eb5071e8ab2c9bd2b53e7e348cbc801b65bfd62fcdbb3e8e66e7255348e67` — Discerning Financier
- `ac3e24cbf975c27d01f4d085c055038cc41f6f9bec368f26c34a493bcb30022e` — Involuntary Employment
- `ac4a400b478c7cf1a4c0ed7e528a4dafaa73c0403587904dd18e85b15ef4104f` — Mystical Teachings
- `ac62ad5fd285eac4d613f625b49bcedf0fa65a198cb4a74ac97c6bfdd4cd80a7` — Skittering Surveyor
- `aca67db055aaba9fd63f1249846d436c4032117cec66b9c89ce03c44540cf63d` — Greasefang, Okiba Boss
- `ad263ca4f4f95abde8be043e47899f40085715122b1e71e336308d9ddfeeb03c` — Ondu Giant
- `ad64badf7b0bbd4fb5a2a5049056da44f4b64eab05a246106bc80a245a1a9947` — Esper Panorama
- `ad686d7f8fbb72179f30993aef658f1d78b62e4982d7274d3863d465502b90ca` — Mai and Zuko
- `adb3b9b66ef09512fa7aad7d222e36a390b3d20e374159fd93b957865798555f` — Hada Freeblade
- `adcdcd4baa516fb6f3d699732ed84d4bdaec857308b3bc8032af9da6f237aea4` — Surge of Zeal
- `adcedee0278ee1520d0c890d00b43e25ee9c3dd262d7e6d81471bf77be28e752` — Essence Reliquary
- `ae5d377b603483febc84582c79d6f84a0694d4750d800a142ecb892bd664558e` — Beastie Beatdown
- `ae778ce450e8813de07e906650a571842c0fcb9fa1c37ad061585c452e0b19e5` — Gustcloak Skirmisher
- `af0ec4a83714e5ba243268782e252a17e821ac39710ee5b5539837eb4722c5f2` — Alpine Houndmaster
- `af96c97a1a8e6529afff5afaa4464218ca12dd79b0ac852b978b7ad00b2d131a` — Reprobation
- `b0dac394a615df524e694e774f7b49c70e5a1c422ee30746daaf59419754d7f3` — Frontier Warmonger
- `b1033281a0a1c768ba6c0e1ed9dd8d971dcc7ade941a33cd0ccd614966dc8786` — Tallowisp
- `b1312db339e91fd7f6693f69dcd2df2ab0f9d53f2b918f6033f66783725db2b2` — Firbolg Flutist
- `b13568338c085b159a84b6997af139d0564ba59d38b6baa0cec0fd6f2c4b17bf` — The Weatherseed Treaty
- `b14c0ba583a3ca85b225ab7686d5e75502e3b1035c74a217ee12883fbd2e220c` — Dennick, Pious Apprentice // Dennick, Pious Apparition (Dennick, Pious Apprentice)
- `b14c183938b8d3c4d79c0d7de75922d14e23d11d240f1502b2189cd1a9a00e8e` — Myrel, Shield of Argive
- `b157d60d60e926035fed5667d4623a420045dbc2768054d7f8a4bc221034d65d` — Beastrider Vanguard
- `b19bc3d61eb350dbb466d9f63fa246f2d6093b9623c4afa413b50d6ef8004542` — Light of Day
- `b1c2a01f4dac04febbf208365fa2f2c12fd6974f24979e19ac53257387d2a27e` — Death Tyrant
- `b1c354e64ef2ba158b938881a7ffe78023799c9ce4dad181305cf1fee95aef74` — Gustcloak Sentinel
- `b24e71f4c481df02c6f0d056a2c0ca4451531cf18f943b320690c774767119ca` — Public Thoroughfare
- `b25a20d684eb932a4dec2a4a8bdecc1a4afe4a4b6e869a411002475e89a8c86f` — Connive // Concoct (Connive)
- `b26442715836f0cd8749c3ee7a60e108088b2574f0e4761a1614cae6b7b06819` — Wild Wanderer
- `b269992410d8555a9a40d11329bb23988fe7d00e62d7de4b9f4f4de27ad70e5e` — Flamewake Phoenix
- `b2d98b0a924eeef727f218ecb64d00219afd3aff375d38f9ce3351f17e6079b3` — Lossarnach Captain
- `b3646c5626a63db247d9f3b9f0ff3e5251d400fa750212d7ed82488363964053` — Roil Elemental
- `b421999c66bb597016a3950e7d4a27dc3844c5e238a49fe74fc8b1221b1ea6af` — Crown of Fury
- `b44003ccf840169a8f82b328634d5e9f1229506c8d9184d9e00db31e2547d3ce` — Ondu Cleric
- `b50c838b48c840c139855e989e7230b80cbdd7e6e4ebc91e9095b2053032b560` — Jecht, Reluctant Guardian // Braska's Final Aeon (Jecht, Reluctant Guardian)
- `b5546f59e79b223751f7afc25d58a206758563901af9178f29967fe1ac61d027` — Cleansing Beam
- `b59d46dfe59c9e09e6f8bcbe123a2c574929522c1ff5c05365eddf8a5c2b987c` — Valor Singer
- `b5f3b7c16dc26b27c47c1bcebd0291c5429e7e40e573db0ddc2e976b25da6289` — Avatar Kyoshi, Earthbender
- `b600067276d61d0c22e94adeaba79ad02643ab8c7ceacd474ab3ba15c3bb8798` — Citanul Stalwart
- `b623d038205d616570c3441afc350f031bd9b1b7447dc78883101fdea4db68c5` — Skemfar Avenger
- `b65f7d4656699699d4e8a3833917da478ca82ea1b9f385fd4875184be02ac685` — Ajani's Comrade
- `b669532acd890cbd146975b449aca29a46e502607820d023b80d5f20bd0a249c` — Land Tax
- `b6a37ad784d0bfc505ae5494b3d7cba86246b15f77b50f4474d96a19997a7bbb` — Bilbo's Burglaring
- `b6ba76a64793e90e6e1a4c500b716b2fa60b2b8cd31f3b6e0696c71619f0c7fe` — Pine Walker
- `b6e8e4a9766aae982a08f1db8b2d5d37853d2202152fe486b4c16bdfc74ef42d` — Claim the Firstborn
- `b705091422d35536cf6c2a27a2fde8940c8a1a91fb8af7672df2d23f0f3f75da` — Nightpack Ambusher
- `b70aa9556ecc6752b636293395196f978fb9dcc10deb6201aa164d35d5c0a291` — Web of Life and Destiny
- `b71f4e16dae3240e4dc4c4f3eb4cc4b15704f92c73799630128a43e93481726b` — Ascendant Acolyte
- `b7257b3cd9c76a7b4081a24c08a97dab5e768b937dfde91f802bfc4c1f55cc65` — Rampart Architect
- `b74987075fda813d03b5b5e74b7bb1b22f8885233f886395973f45edcd88a570` — Diffusion Sliver
- `b755114963c2a088a5967cb33c46a94a4aa81a851a52bb348d38378e416529c4` — Mind Flayer
- `b76b98b773999c378c2fce3fba5976fbd48d19d29e6750ea79306d3873c8b8da` — Thunderbreak Regent
- `b78512e698bfbdd3196dec18d57bb50ba2d49b9311c9336e88ec884631900c76` — Corrosive Ooze
- `b79d4ca8974d200d3563a3a6d8ab37df61adfcd622a577d36f6812c7a5b8115e` — Unlucky Cabbage Merchant
- `b7b27d80d0b96e2863aa7751608cdc3513ed6b6c2e4d41eb69ce5b501196c225` — Living Lightning, Charged Up
- `b8068d76228e06fba87193c42c4b557d6d9e1016a4317670cb1393a72975d0b0` — Trostani's Summoner
- `b8426425231e56dd536d0f30760c469c61ff79dc1fb4694e17263e501847ad7c` — Bond of Passion
- `b847f85e4521d13dfec3580ebc462ed84ea671caee1c321e7fe40d94cfcabeb9` — Metallic Mastery
- `b852f3062d93cc0efcc7b55e7be48e208ded3421c5ff5144b42b6fb7a97d2d12` — Kari Zev's Expertise
- `b8a09aeb925e357287e2a8c3b214de5f3a72ecdf4b905d15d370bc2a7e3bdbdb` — Friendly Rivalry
- `b8c174cb902e042d21f799b3b3c8617e47086551e5203fd6dcd872cd23e47f51` — Squad Rallier
- `b930999513f8bc5551251824c9a2132f4d8d580df3a63860ca26412ce0dcb2d0` — Skyfire Kirin
- `b95c14d3b4b47f84e77443095be4c5e47febce552ba062cf676b2bc986ad0158` — Horn of the Mark
- `b9dd3d8f2b42b67e0777b42c691f81fed3c7189a6969a414a042bbc7dae6abee` — Wildfield Borderpost
- `ba3a50f068493675fc07c1e8c07318d5e2f474c91ea5adf2d3271035ad00771c` — Territory Culler
- `ba52ea0fbf55bfc2d111fd91c928f7c38d3a6e37d3d663bedcd25f496cabbe28` — Ratcatcher
- `bae817a9e634e6a7646ef2b4ca507926ea55bd9dd6de1001cfbac97e047d35f9` — Gift of Tusks
- `baf961e022a1efcd888070971b19363216e210d07910973d013554c53b7deae3` — Amareth, the Lustrous
- `bb354c0d6ebe4af5a926e8d0d41d58a9b3dca8ee330381b01fbbd800fd74e979` — Thunderscape Familiar
- `bb4a63547aba54a2212430dc2c22d73e4a0e4080f79a0acab098c0f5a9826925` — Niambi, Faithful Healer
- `bc0ba62011ba7c8faba5ae650b941f2eda09491b7b036b9e6d6744e30295f4ba` — Rundvelt Hordemaster
- `bc37306a0d401a0c7255be82a209d177ed0a3953be67a86c288064dff1e88970` — Thalia's Lancers
- `bd1b6297c83404c2376c49eac48f6418022c1dfd6e276b263cf19cb3798099ce` — Sorin the Mirthless
- `bdbc59fedbb54176d795a552f1771f749d3718aeaa8c3200374c33141d4ff639` — Archmage's Charm
- `be26559776491f93da6d9c392a5f627db8016c2771e672945c652f31608c6f17` — Vision of Love
- `be62af0f7e6fa87cd73391e8d25edb62f3567dd6c3b6407e2ae0c342a9018c95` — Display of Dominance
- `beb0a491cba07860381de5fcd3be5df069bf0d183f78e31e7f58128fe634a713` — General Tazri
- `bede0b5816e041d840a442b713ebd3b30f887722de7cb14a8e6ab2c0af9194eb` — Capricious Efreet
- `bee1829354f15f8968907f0bc76788cb1cfb85a80d1e7873eb91f435725d36e9` — Uncovered Clues
- `bf301d97b4645f7687ecd9a7df26c01c50867f247e006a130f3edb5b9b63f0b1` — Bojuka Brigand
- `bf5126b17beaa5d3a16c5eb8b4b2413a630b19d8fd9efef26bbf8c8f863657b7` — Traveling Botanist
- `bf621cbadc18793116e47e0515226d6417d901e4e9ef1d0a523148e8f0193ec5` — Drooling Ogre
- `bfdf294b805b9f0d5c7c244bd0d045670f2f0095ae3b3349e20ea19226625162` — Dire Fleet Warmonger
- `c03db8273b83d25fde9b41bb9dc2884cdf179d0f83534f186616a705f51920e7` — Deepfathom Echo
- `c0e5dc4549b6bf52805c9ac3e245f06647571cf8501e59b1922c5394e1c0e18d` — Mass Mutiny
- `c0eba3c8013e5a03ce0b7393f58afc5a2511a01c96df6ff3207b0cfb19c156c8` — Karumonix, the Rat King
- `c16d30b81da1da78f81cdb5fcda3982648c46f488dee26fef874142c3ce053b5` — Molten Primordial
- `c1700e82ffc4acf4c62a1294a50e08c383da8ea64f46ef569f824f58d80e1023` — Inventory Management
- `c17bd8fe375c166cc2c861a5310637a56386819fcb7ca87b2624bf13b14e7725` — Anointed Deacon
- `c18c7baf089e565fad89c612aa4245645b4905ba52c59c738b7f45392ef5fe64` — Bilbo, Luckwearer // Burglar's Plot (Burglar's Plot)
- `c1a5531329c0a883d7b017f718b66dcfa20f31aa65b9090839c4b20f69225066` — Gustcloak Savior
- `c1d0abc3ab807af20fb45d90cefaa6f4c1a546633e25665c1e6af60bf673bd1d` — Harried Dronesmith
- `c2509005f036452fede2ad1c0c15bc0e912caa0fc5473e78aab147ab8f33ecff` — Mimic
- `c26385da950068cec36fd6987edad9ed54711cdf8294dc910121a2c104e7d878` — Kate Stewart
- `c288cdebf51159235e97a7f93e68b5eaba48ed771160e347db87b9f5a61ee137` — Captain America, Liberator
- `c331b3df318244243da55ae93cef951208d934b4b78ded988960fd3ba282458a` — Borderland Ranger
- `c35754a45e33f79b240115251fdcf043e17796f27aa61b901f70f098b631a3e5` — Wolfkin Outcast // Wedding Crasher (Wedding Crasher)
- `c384c252018c84afa971ebab83ba7cfb0b83e708887dbd10b8e6cc24800a5e48` — Herd Migration
- `c3c778822be9f3d854278e024a3aab8cc584b4531801c66e0bb817b657f75096` — Chimeric Mass
- `c3c86fb652089cc5935a90637bf1369d8cd31a54dc713c0c9909b3238485f596` — Boltbender
- `c41f843b0be3c5dbe3993fdb61dc74db6c942e4ced59f6f812169006cd6addc4` — Eriette's Tempting Apple
- `c43b4b8b80e1bf4ec2ef30ffb7ad4f5d81a20740e2d77216699898a40062ab38` — Gift of the Gargantuan
- `c45716e3a8546f16e4b2f54a4e125502c840083b4af64c550d50cbf70db06c69` — Sphinx Summoner
- `c4a3ffd0cae0ce878b3d4fc51ea2a985feaacd57d6b8d22b21500023d6bbf717` — Glint-Nest Crane
- `c4c03c14b41d0fe57c6f8e6aaf80a36e30433dbe801353cab3838e08adfef1cc` — Alpha Status
- `c51e684f38f6eff22dbfc9da20a3b106e19721e64388e913161387c8841d46d4` — Warden of the First Tree
- `c54a5a979fb0a2384d4384b80f7da564212ecff37692a07b1cdb1bb1a8faf635` — Moonveil Regent
- `c5525a05251a73311832779e026cadc5294a8aacba44c18817960d8c4db95571` — Lead the Stampede
- `c594a444ddd6617d0cbf9e2fdcd7c683e5a018c95be35339ca63e80c118069bc` — Dragonlord Silumgar // Dragonlord Silumgar (Dragonlord Silumgar)
- `c5be3a6779c4c9c76f403c2d27133b3a868c986a9623f9a61efd203690afa641` — Eternal Scourge
- `c5f6d9468f523d8f3e03b1ddc85effa5d79c96e23eea018fd3c7994921db6a82` — Ajani's Influence
- `c6491c082208b7ae645d4f70b8b0a9ce0a21c6aedd45392b969dcda5626912ec` — Sarkhan, Dragonsoul
- `c649456b9b279a8ef5326e1fc28bd32e36345b1840e1404327783c41c9f082d1` — Nighthawk, Dark Defender
- `c64ba973840e9e1fefebecde2f35739a93941c9ec61d7179ee248e300beafb96` — Baxter Stockman
- `c65567fbabb707d2710bd373867f6a4c675e8b92820d0d7a7bffa324c2aee052` — Soul Nova
- `c68217c326368aa5c4c8e7cbb0da0715b315d9ebd10610ae0bb103c9171bff92` — Bloomvine Regent // Claim Territory // Bloomvine Regent // Claim Territory (Bloomvine Regent)
- `c6b74d419eb7bbc12422912cd13b158b303aaf1e602f550c1c6ddc76b91faba8` — Bountiful Landscape
- `c759dadbe9fc67d3e2def6dc1d9beb5224fb80bd79374fcfd5125346842bb945` — Bitterthorn, Nissa's Animus
- `c792286b7b5cc4fe7897d33daa19f5d84347b5b840b40dd76b84fbbbb98c4fdb` — Puca's Mischief
- `c840ca44d046aa43d926b73aae7c378039721e98b99a28a914fd6cc7dad9d58d` — Hammer of Nazahn
- `c8b2bea7f64af643146e9004e7577ed7ce51dec047f179d54d2b9ec4e0a870b7` — Pawn of Ulamog
- `c94d6d0855c67e175bdde8111e1df5185b1592fc9db39b7b012be1df774e7017` — Gustcloak Cavalier
- `c957595447254bebc13405baa4fcba5bba68ca2f874c2c42000e469e09dfc86c` — Dryad Greenseeker
- `ca1bda43aa494350cf12162e9e58067bd0c14541599babe30df725e444ea8c3a` — Jinxed Idol
- `ca2f0bf78e843693a14c288282cfdda7014c7395c8c4ce638c1d0534ac11211c` — Floodpits Drowner
- `cae1cc612fb7692548b3336993f40577cb1842638ab39c8d54ea782b4e7fe7ff` — Elspeth's Devotee
- `cb1e6182e137cfcff80507cbde944b6180205ee1d9d15a8513c6cea6979c2585` — Lich-Knights' Conquest
- `cb7970e1d14d6769d26cb4b04145f191447186b1472e5fae08f9b64e9441411b` — Nessian Wanderer
- `cc41ae893a805bcfecab9bef6aecbadc001ae95ac604611d752205901b89464b` — Commune with Beavers
- `cc4c8a80f454a7ccad5fba624722135a60fa232a6e2711ba01e9aae5e1d41aa9` — Catch // Release (Release)
- `cc821cdc7fb2f5271014e3330bea3b297ecf60e97ea88988e5792423c6200b9a` — Piper of the Swarm
- `cc87833fcace3f68f12d2ee23b22ab27af2e898fac3f542ef711baabd29f805d` — Call for Aid
- `ccbb78b9294104c7815e6cf3b9725e959a0aeb431cc7d291844e69121875fe03` — Grafdigger's Cage
- `ccd4628b88e1651fbb0e7664ae1433e28491fd45918c2d6ff203eb05bb6442ec` — Luxknight Breacher
- `cd158128052043cc3df27f769f7812d5699511246137974d0feb4744fe548a76` — Portal of Sanctuary
- `cd74762e2a02595239e0bc24dc0c3f75f07b9236f043ce539598d936ae492b12` — Crown of Suspicion
- `cddeef9bf2fcdc4a26ab99a51d26e5fb52846cd5218765ad88bb3c611816949a` — Allied Strategies
- `ce1a335fa8aa08e9ec3e2ce4f58c37785bb4bd150a6d6b1a9d83de96f5e62db2` — Seething Landscape
- `ce8b37b9a17d404b47eb81290be4c3caecf7f81b95ec7a1826c93e41763a255b` — Weldfast Engineer
- `cf89eaaa587b14571e5f58644fd9459c5787b98f98500d881bd35e9a2bd8659d` — Rainbow Vale
- `cfbd57507d5ab820a1b9e5a1b240eec6b73a7909892ab9ca470f06d38af3209e` — Martyr's Bond
- `d048faf3ba32195392bc5d32d48ba1c9da57c978e549f99fe7f133c0c16dafb2` — Temmet, Vizier of Naktamun
- `d05ad6cc46bd8f9f7989021674b423af28fbf51e6c3488fc6cd6d5259abf8330` — Mass Manipulation
- `d078e669dd528fef83abc7af00c194de187a339b6ecbeb66e80a18466cdc7517` — Gavel of the Righteous
- `d09021995ed8b2f8b71805c98f0ca461800e25c80f084cfecaa593d3de3c8233` — Jackdaw Savior
- `d0be66d05788ba67544e4b084224c8de1c07419561a3658c52da26b39b7bc21f` — Wandering Goblins
- `d0c5e3296afed4116eeac16755f1c716e7dad00b0b96e90369bf75d6e477ab46` — Thoughtrender Lamia
- `d0e0571bae0d38bdd27651bf0c8a652f071e6f1180a5bce665c9c58e3f2d7815` — Tangle Wire
- `d11adfd20f98a759344b63fb1bf646d4d2026a09d68a03bbf0601488bf148df0` — Sludge Titan
- `d1513fc23ddf9cf39909c79861b3b98d925456621665dded506d8c9d0fa1739e` — Sarkhan's Dragonfire
- `d163040a72f8cae5e116fc2a0393003fedefbd5a85f10d26a5d0cf7f0e926693` — Karrthus, Tyrant of Jund
- `d1a3740864395c01747955fab4a8362c472656b20b9d758afed9c2da19cbc552` — Heliod's Emissary
- `d1c96092a403f82ffd312a81b75b2131297ae7a56e36f66534abeddd09334725` — Wrangle
- `d26320f71668a64a5a8275cee0eb862062624e32f9c2cf64956010a846d5f9d0` — Unbreathing Horde
- `d2978737b949f390c545a47ba5c6a7f915eb3a2d3aee290ed997a2745529d1fa` — Haradrim Spearmaster
- `d29aaf6b41b9bf3c81d57d2be77e3cc67cc65d492faa8a09aa190fe7931445ae` — The Trickster-God's Heist
- `d32348915d9532f9f1b6f7c5e399f8c67129335b34b2665f8258f6254e19fa76` — Gigantiform
- `d3252874a0754b44c696e8652057701995f97b99e5c89186acaf09f37151580f` — Saradoc, Master of Buckland
- `d3478c7edcf2913092b23cb52d8b7d000fc42d1d8abfd2ada96bbeebf92377b3` — Grab the Reins
- `d35c8e786fab3f52de0027d57a383b466636c3ea3b27922b0770d2fdada34f2b` — Sheltering Landscape
- `d3a212c31d92d258ab1f1810576ce509730b6fcf2c292c3df164704dbc3bbd5f` — Resolute Blademaster
- `d3b0be27d8750abf7fd0f8bca51120415e69fbd604d0d20dad746d8e45268f7f` — Oakheart Dryads
- `d3d2e379be510b75afe3987074fde48daa990c3b91cd5531df972a1afe14df3d` — Thassa's Emissary
- `d3dd4665ab4bf9e1a237ef3470b39a5368ca93adedda627c2e9b9d7b059c8d88` — Crown of Ascension
- `d3e5bbbf6bcde98ab0ef3012551f76e3c36fbf3e06821298cd52a4d56e557e42` — Host of the Hereafter
- `d3f67544a7f7f27ad8318b180bfe2257d33ce5663a625f0f9e3972701836ec74` — Pulsar Squadron Ace
- `d4361c4dd70832d4d23a24817048bf881ea56ecdeb40998b7290eb5b54f734e2` — Ardenn, Intrepid Archaeologist
- `d46698e731542ec980f601857bb8e125c9f194e3eec055d714687d07fc726109` — Frenzied Saddlebrute
- `d4753ca381acaaa783c52e4cd7b1ceb9813e8bbca0da1c27c39bf7a9d8adad1c` — Brand
- `d483f02f4b95f433a9140bad86cab1855caf389ea0f54aaab2a9bab95595c24b` — Tromp the Domains
- `d4d9b59e342e6903cbeaee2824c9c6adcbe87414dfd4a37062354798a65d6e9f` — Silver Surfer, Galactus's Herald
- `d4f0ce89d1b1c4c4d73f686f317e1a94b8056fc00e88ec6bc9963d1e947f423d` — Treasure Nabber

- `d4ff40e071565578b487632d1cc37558735cc637db1e22e1a59c933563c73bf8` — Sliver Overlord
- `d51dcc2493281c8536cf00784a88361486ec21926856631d647b9fc61b978fa1` — Deceptive Landscape
- `d581c7b757f6976bff4a7f86809a495c7f617ee489811bfefdcb3f863fa330b8` — Wake the Dragon
- `d5b80eabc5cee39303c82db91699765fabca0e39cfbe5839ffdcc732cf1230c1` — Loran, Disciple of History
- `d5dc11f28a4ebd1253af1700902a4e0f40ff6853a01d636993b8271aabdc0571` — Multani, Yavimaya's Avatar
- `d64817d2c55be0c141b55973da54529772664e2cf2eb4f85a5234ce04f914dec` — Infectious Bloodlust
- `d6588e485d136ccb38c262998e6cb8d0452aaaa3813ba562838836e428d5a9c0` — Incite Hysteria
- `d66d331d9bdae27fe11d543a9d2eada883030c02933860c6f85e99114d8c37a8` — Vines of Vastwood
- `d6ec3007699422fc14c21ff37b3ea0017ab15af160e62658468319f0f8e7f88b` — Kain, Traitorous Dragoon
- `d71c1e02167dcb2059b430f5e23dc9ee4809b406879cbe34d2dcaf441a59029c` — Hoarding Dragon
- `d82736e372e298a48f5ce79986a8806c4f16e534be0d5aec654a4b277b3ec8a7` — Merchant Raiders
- `d83ef9a4d03670d93d4079e5881526721e20262dba0dc3a68e84eca29df99c75` — Song of Serenity
- `d85c1e210c942f02f97299cea8a9ba8a67bca8e3c225649272bfdc9cea83ba30` — Mass of Mysteries
- `d8a56722031be01b85f8ec251e7296bda686f568fb55fc14db9f1af9734ad678` — Stalwart Valkyrie
- `d91e75b7e7eb5565c9aeb25faec0b6e27ca05e8fc1bd31b84a3f57fcd015a7a8` — Circle of the Moon Druid
- `d95d7107154404b91d9b5f4d2ebf764005191b4cc1ac00fd940ebdaf58549447` — Boom // Bust (Boom)
- `d9a58f2e54f36653531cb1e7da889b9aa63bec2c3ac7e3ca194fe3478e2053cc` — Elephant-Mandrill
- `d9e642bde057bbc16e55eaa86c4928bb56b631ddfee6dcc3e007cf0e628d3b9e` — Tsabo's Web
- `da0c2f0ad070163e2e079f94fb58b3c33bb77bd4b6010538198a9f4f2d158139` — Murasa Pyromancer
- `da9a2d2bcf393a3c85569fc20cb5399c9d22a691a32457fbe4633e7348b4bbf7` — Sauron, the Lidless Eye
- `dab0e95c0233d96651b460f65d6ea500f931789ae8fe778466fe95c7628a1f00` — Bolg, Erebor's Reckoning
- `dab54ffb1db27c4f01b56aa443504b1eebfb9ded0510893da7afede19e3ce3ec` — Mob Rule
- `dacfe160e05824f672530168d8cd7738d35863637435c3a11efc946af7843b3a` — Fey Steed
- `db2b9ee91dd183bef2901b3a67db42394c625bf5327344fb535ac1f8dc896825` — Zulaport Cutthroat
- `db4bc99657aa416bf98c407e8e3c08cc55eb2bd000b4a793f9c209f1b5cec755` — Dragonlord Silumgar
- `dbd50c585985d597a5716575e07dbcf8acae586fdfff379161407ee8bc0edded` — Warm Welcome
- `dc1c781aa3cabbb6ecd22aab9b4fcf08b7c5d07ccb98bed22d1c55e8b6956aa5` — Strixhaven Skycoach
- `dc2bab88aedb09c7c44c2aa0c73ca2912d367d6e67c864d29839a85e8846c2ec` — Keeper of the Accord
- `dcda24da1fbecd2b7fed3598887612b8d212a9e60ef3a700840fe4ae2642a96e` — Contested War Zone
- `dce5daa63c75b91516f57ae4edf1f89a95ef1f269020d2f0f36da24e03afd624` — Stoneforge Mystic
- `dd49ab79dd4f628769fc5829c4922822be6cbdd1dd87c6f7e8039fce6c6c196c` — Harald, King of Skemfar
- `ddaa1f492cbe669342071436cbf24d67e59eab573a81f56c1cda97495ccc8f05` — Daru Cavalier
- `ddc2a7cf88c7b525615022c1c70819fc5182bcee3c86a1660454c836df5306da` — Reckless Stormseeker // Storm-Charged Slasher (Storm-Charged Slasher)
- `de248c3a29c45e38edf4724416d5762ebe56693681a4cc3eb77cec90a1d65a22` — Primal Druid
- `de37ca9dfaabda64de17366247dd57067cb3289e599705fe1bc6b4bdb739cc02` — Turn // Burn (Turn)
- `de65ce61db035e04fa02d2723b61acf41742085fa05b8d400ed19923597b51b9` — Boughside Wanderers
- `de71cf0b9163b0ce83c09a5c9ba7e0d7aac7ea9bf1de81b3f98a43906f9fbb67` — Beregond of the Guard
- `dec71b9adbb1b434f3fe9ccaf99fb82ce969190cc1cdb8fcb0e14f2d97a44e5f` — Ethereal Elk
- `decd8aa25a9196176ce1bac24fed607c774a624b977a476871ed84ea9854965d` — Darksteel Splicer
- `ded001e223fdf7876bba4b9a4a0be4e7ca0beee1b06af597de5a3ba7e78acc08` — Unexpected Request
- `def4b0eff5044817a0ece33ce0ed7d6f5974fd474402620d6f178b23bca5ab2b` — Thorn Mammoth
- `df11054427f4534960c96d05763fa2930c18f7bd0443f1ce83957472171a66b3` — Riling Dawnbreaker // Signaling Roar (Riling Dawnbreaker)
- `df1da3e243f60f60fa81ddadc78a653b4f8217f43ded7819b31ca8c7e0a46fc0` — Territorial Witchstalker
- `df67d23326836ca4853170ded8bb189599ce57819f29a83c35f63fccb30a9045` — Furnace Reins
- `df9c10eda7a770f9b1a7c4875550c88776bf835b389551f36a4197695d2b1835` — Nasty Little Rabbit
- `dfee69bb7f3abc810e5eec4123e983dcb43ecc48138ba15dc39b0e24c7a05836` — Trostani Discordant
- `dffb21b90dfb620627a197794b486fd1e46100c9621df6765a4a6ce60ce68736` — Siege Veteran
- `e092960be3fe621add780264f42630d5a20e8b3cadcecc99b7d1560c7703afb5` — Chamber of Manipulation
- `e09e01b42264d7fe7eba6735c3ec4dc99cf3d085300d2274f6f59a1982f1d623` — Search for Azcanta // Azcanta, the Sunken Ruin (Azcanta, the Sunken Ruin)
- `e0a5ef26f071fdc2f13b011bf3b1207c6be07f2526915a9df580bb7b17011a3c` — Fieldmist Borderpost
- `e140e64fabdcc8318d131c1094ba70debc7c6cf4b16c037f257a56eaa014706a` — Shrine Steward
- `e15de54c130e2c3305292b6665f7bdcbc9ba457299b1eae3564ddb66d34db21b` — The Antiquities War
- `e17b913b5cc9f1bda0b31e306e1fdcf85286517aaed6c26f9d456ecea992abe5` — Shipwreck Sifters
- `e270b4645d959ad5f82c1a68b708a4d6b89fec750e5eea34ee2ef0c0b8ca4a84` — Blatant Thievery
- `e271153e6ac53261800a6c50c1f6b0a6affb324baa7d3dc8352abd87d4e95cd5` — The Fall of Lord Konda // Fragment of Konda (The Fall of Lord Konda)
- `e2a1022751a73163a1fafb77839b82e671417d1759fc4e0027f6cd80ef998b8e` — Smelt-Ward Gatekeepers
- `e2b4ef454b8431df437092a651b76e41dabe75fccda13605a46b5e62a325ec19` — Yanling's Harbinger
- `e3663987918249bdf12711931aee031b9fa48d81ae291c406f16bba4d10957fc` — Sword of the Meek
- `e37ac102933bacdaec15990b25ca9b26ae536d07a8d7f4d497439ff89694cbaa` — The Mighty Thor, Jane Foster
- `e381c60f9a327b532088724c18fac73e34b056fdbe6f107d0d6ef0fee4ef817a` — Avarax
- `e3cb8203fe4490f7cb57977d7d390655be9bb56b7b4ca93594c751aaa0627dfc` — Headless Rider
- `e3cc7aba0336a994aad1b53131fb7475c18d014a80a5226581774180e9ed656b` — Frontier Seeker
- `e4b8ece22c670fb657d81f46e54b386b674cc7ef234305836acc1dcf81330837` — Choose Your Weapon
- `e4d2086c16adbaf176dee3560e70ca9c1b983cc17ae0a3865b7a5b1010f68b1b` — Kazandu Blademaster
- `e524ae2c39cb034ec2d8a8ce6dd77e26330283640fed1ec6b9e4367ae1be35d8` — Lose Calm
- `e57343592684d6dd1767bf641b6854c845d5bb4bc749b20aea5e85cbb0bc9437` — Pilgrim of the Ages
- `e5befa83958b05222b9c2fd42e7ff6cbbd47af201858f6235c6d225b240d3cc4` — Casey Jones, Jury-Rig Justiciar
- `e5c3278970006dda3aa88634b6af83df8be7523c1fe6b3ddc66f60c43a338f97` — Drumbellower
- `e6087fd5aed4309532bf55a673e8f1f25982f12a8995d5b7e5beb8825171ec45` — Kulrath Knight
- `e609b36c3918bb3b8f70f17acb8dab2550f5d50cc04e6c06035f84d652772341` — Spore Burst
- `e6561f8fc606702aa76c71efde3b24a091289fd72c42411f5f78a2d3fa16cd8b` — Bucky Barnes, Eager Ally
- `e6abb930ff695698ffdfacff6d7ad76cbec6b760a0e629311b0c6ac33c15f73d` — Boonweaver Giant
- `e6c71fa1e530bb36a7406bdc228ee85bf32f94aede869a323bd750f0db477e59` — Naya Panorama
- `e6eb6d9b2bb634cb3a84ab20fe7897d0cffd2ef26eb89afce731b9c9ed793afd` — Caparocti Sunborn
- `e72159f5ca5433ea0500f21919a483573781440947ce55c53aff99619938729f` — Colfenor, the Last Yew
- `e73768c2699c0ef5123295b7a7b22e8bac7d0773a81c603b1f84064311826928` — Sethron, Hurloon General
- `e7398ae6b38847d63fc894ae114bff3059fc755ddf0adad0d745c6af6ee970fa` — Jinxed Choker
- `e73e22bbe38bb2c255948f59adb5dfd3a98f234142c55f258566d6c9b6f4a948` — Dreamshackle Geist
- `e74434c6a24f22616b38753ceb18f67a6420d837aed0130529505fde06b468ec` — Shifting Grift
- `e7da74ac4d17c783c8e31ec97d2438a4ab501af280b4402b6f4ee10247e71229` — Enthralling Victor
- `e7e04575027cdbec94cb9d8ec23be5ef82680d8744ac3f03324ba3b986985a52` — Magistrate's Veto
- `e7e95d0bb0cd031db7e6f83ba6f22bc8cbdb79563060ed26b9618ac664b64cdf` — Tribal Flames
- `e7f60024b6013824a18707560df479645fc9081904a41bc5f731cbb541b83ed9` — Timothar, Baron of Bats
- `e84b11cfe5ba9701322065073fe99d4d49e7908d07b3c79677f6949f5d6d367d` — Steamcore Scholar
- `e858bebcbe90170b96aee57eb18109fdf04d0a1dca866a8b021dd1c5e8f94efe` — Rite of Undoing
- `e8adfaa288a413b2454f87da6dd6af01253a9f21de4d9c69d5a393c91e19fff6` — Oath of the Ancient Wood
- `e8b6920fd2301c12b9cdf3a92908e80ea0fdcfbafb5a97db1ea9d0ce18c365cd` — Cerebral Eruption
- `e8f5e5428a0292a370fc76744cb6175bf1bb557fddbef882cb4cac4d5ff9ea07` — Endure
- `e9fd4afa5113899730e9c65e0518fe97d41684a1696245a472257e6068e991e6` — Kolvori, God of Kinship // The Ringhart Crest (Kolvori, God of Kinship)
- `ea2c342758fc2b885c6680acd068a661443288b4374290debeb3c15d36cfeb5f` — Heavenly Blademaster
- `eae4fc42b00d9ccbb6e4552b227dd0d27820612b03e0d355444b1c8e832b258a` — Wellspring
- `eb3e8c6af123587255ca209b6af87dcfa2bbb3a81739ab7cf23e8bb17b730e5c` — Go-Shintai of Life's Origin
- `ed23a635b1343aa7e8282f9ce6925d369a28c71d0f70031f4d7f897f1a74ffae` — Living Phone
- `eda3022087dbe5d66179c86e7fdfe116c60e321e6fafb6ccdb9c94596154769f` — Bitterheart Witch
- `ee15ca95279c35b2f9c733750adc30121815e151fd23866389bf328ad2e2a266` — Nissa, Vastwood Seer // Nissa, Sage Animist (Nissa, Vastwood Seer)
- `ee23c58244327daa67e9bf8957d168a2569eed63cd325d4830b0f82f0432f9e0` — Basic Conjuration
- `ee499e3d42f2af9d1cc61feb535d921b1b203f6c047d75110994eb6fd307d30a` — Silent Gravestone
- `ee817bc1636e584f9b72c3b86101884c48de277dd0a0d3511cc6b4c68d39249a` — Drag Down
- `ee94e46dde716e2679aeb9c56a12177cf7c83bf673750da89d8265dcf885a9d1` — Firewild Borderpost
- `ef245fbe82c1b9f83383b7ffba4a78d4531ac1a3bd243d534d734b997dc89d6e` — Trail of Mystery
- `efe81107e11a5623cc5c756bed31bc51b82c1cf6a423b31b9a5b6ff309da2d99` — Turn to Frog
- `f01f24b4d719d13281df4694ae9a3618ddc996ca9d4cd67f987723fb434dfd34` — Iron Man, Futurist Paragon
- `f064b467319b61a2ea1d13df40115a59f6814921f1bfdf446fc8ae77b28e1f24` — Force of Will
- `f0c913dbaf008dc80d3360ecd407ec71a2c6257cba76836e44826a4309da121b` — Primeval Herald
- `f0e1dc71f28f255f4b4c8cf321769a5714c3c054789a211d700a55e1136b5090` — Solemn Simulacrum
- `f1b00492bd342342a857498798237cdec7b46ba2ec2d696dcb0531e51d6c9486` — Imotekh the Stormlord
- `f1bd3bbb45958e7db4466093824de0386bcb4fc7c54062f001950f86a2f25c8a` — Fang-Druid Summoner
- `f1c262fb2c12c10404fca0105c6c4b6d93259323b90b7b0a24ce0524bbfa2c62` — Doctor Doom, King of Latveria
- `f1e707f790fb216b33a9bf4b635530a8b2c701a14886d3c25e2fc01a7b048f24` — Tempted by the Oriq
- `f232a46a2d460ef02824ed3dcdc721a0fb537de1a931a3a7e5aaf25d8f6b10c5` — Yawgmoth's Will
- `f24e262a97decad4e4f5f6882bbd4364f2e06ff2508cfbd51e96060f46da90b0` — Primeval Titan
- `f2e36a14e0930dd9807145a5b9e801b6acc62134e847b8735d45eb5ff715d199` — Luminarch Aspirant
- `f33c6ed71155a4914ead0d998d7460efc56044bbc1f61b5d03a0f0a96a8971dc` — Sorin, Vampire Lord
- `f36177eb6b1075604323cc3e8fa6988b66dcfe4ebd4b68b0d5f96b11039bbe18` — Diluvian Primordial
- `f38506c59bf1387761984eabbf4e69face3338197363b66b68250a2e64d9d44f` — Rampart Smasher
- `f3c25e4b12ec70973aa201a08db8e164afde27da931e21c09e0151e8eae04d34` — Suit Up
- `f3ded5dd4007f91b440fab35d58de5ce723b037212ee918d7c55d773651eb4e4` — Seeker
- `f443cb7e8436a231f81758f3be18e39c4882ebc032cd11a9032477987b1e8996` — Shadows' Verdict
- `f45ed7198581f7b2af341d3ada70f30db5a5665e4354af1d2ca0902f61208475` — Champion of the Flame
- `f484f328482b4f63a6893c606e80e004ca64ba8aae65dede80cbc35d29e8c9ed` — Word of Undoing
- `f49491eee927d211d37afff1748dfe3b4bbb8ce47e0b71061ce0d41c46431478` — Elvish Soultiller
- `f4a1ffa81c1723f17ab3ddd46ef892ef62a7562003189751df7298f1231e8bd3` — Limits of Solidarity
- `f4a760a0f4de4e5291a44ca0012141718150b95b924ea29689035836035019df` — Gaea's Will
- `f520312710b60c6ba54a49f64ffd440e8ca9ef63f3ea732c84afc072eccc17f0` — Grim Guardian
- `f53ed273c59c19061036f29853c13c3762a36ad029c4bfbb81d3dbaf5ad08cf8` — Skinshifter
- `f585b050614792c3685bd96581dcf65182a59a2411848b53f21eccf3dac9fe70` — Divine Deflection
- `f593175559c32c01e4e8ea9b46f18c497114e589608b19f1143c00235e0add9f` — Phyrexian Infiltrator
- `f5951acd474fc14a3f7fd554cbe76ef71ea6b39c2b35df29cfbce5c3c5492500` — Sudden Substitution
- `f6173baf73668e21dc75744decac7ed4e7b7311c8b36f81772a682131c9e94c4` — Cathartic Parting
- `f63de465b25115019a7e4a7d653127bbc6f6ef253c78483da8b357355c935933` — Ground Seal
- `f643d289b3d1ec663458426e9955a41fac6dce01fc6ea0417d76cba49dcd4796` — Ondu Champion
- `f675268934f6f3d23dd0136d937c2cd98f6b746ec7f1c94c8b8c94e5d47e72c2` — Ichneumon Druid
- `f677df0d226992030eed4b26a0879bd4b5fbcb35e435330f503e75a3153f015d` — Vela the Night-Clad
- `f6828f09fc45e1f8e5e5487210360040300ea2b5fa79572343b962300d73aaf5` — Crackdown Construct
- `f735f9179954fe3fd9a8d82ff5de5a7e2c6e0b649b0a338e6b9d553151d2e54f` — Transplant Theorist
- `f7654f280ac976fa332954f7b33f71f96a5325e88e7d6e64be7fd773ca1d7380` — Cynical Loner
- `f77ebfadb7e88d7260265b21391177f9483ae31a0e96f8f7c6b672175b3d2e07` — Omnath, Locus of Rage
- `f7d2cd35c72e3e0e04a52cb526888cc7bd721777978a7bcc5b2632367adab9dc` — Grixis Panorama
- `f82d9902ee7f898cdd3152143cee696518954917278616431b2c99c486b32d4c` — Rowan's Stalwarts
- `f84aa26a31d61ca38998a90311a646c536e6a5ab35df376af2161a488dbb4303` — Earthen Ally
- `f871d39ab307e4e9e8777cc0f22d7ec9d33d4c0a753b790ae6684eb71118cab8` — Trinket Mage
- `f8f46e4d41a5d58dcb05edc128f6f98771a95bf244f7ee6084067c6781bcb2f0` — Archghoul of Thraben
- `f91517f49bfcf767639a2b6c49c6ed18e236bd160744d8019cf15e9c3b5f0488` — Ursine Monstrosity
- `f91b28869b75c9ac324879677e9514cd93a183f50f5042575a38850ada6731af` — Immolation Shaman
- `f9393f0fadea1aeff38f8cbb9c9d11e3c43094a9054c00e24281e405e19d61b0` — Thousand Moons Smithy // Barracks of the Thousand (Thousand Moons Smithy)
- `fa1815b78953c4c806369e34ed0572f7dd5f09c92e9aaf48bb615a89af939822` — Sorin's Guide
- `fa8c36527b531f1527b3ac59ab376e0449127cbb9d2599f860bde9a352fbeb9e` — Agent Bishop, Man in Black
- `fb4b37e90184a3fd246e3d5f956345365419adca8ddc478c830897997160aa13` — Orah, Skyclave Hierophant
- `fb518918ed2007cca8b95ac65c501dffc3448d38e30b57e7a708864e139668f5` — Faeburrow Elder
- `fbefb75485f3ab3cac191ae57eb0217daf3194cde7d0cd7c1016003a35301363` — Long Feng, Grand Secretariat
- `fc00bcc285e2d84a75358df5b8e1aa6b1c5a51d8084d4fbbc5779bda67a61f95` — Dragonologist
- `fc341bc68e0971124119922ce809aab9de5263946d23773fb5f7a62250f3db19` — Sentinel of Lost Lore
- `fc3d9d1f929cbe551cd4dca58f7487df2f24b1ea7e970a0013c4ddfa8b2a6924` — Al Bhed Salvagers
- `fc94a341faf4eef67c96203ccca0bd1a1d869571f919007cda2db564647060ec` — Spinal Embrace
- `fcee7b10ba38e46db639972f633f7be5ef2f247f213773d47ae8db7d1da7b149` — Samite Pilgrim
- `fdaca58b9bab2357263429bced6c2b4853e5b22aa6170278aafb805586e93688` — Savanti Romero, Time's Exile
- `fdf29c42259a2321bf0251cd7c74d1339c688316b28204673f055b03946e6aee` — Epic Fight
- `fdf92154b4ac531e044f5792a051da3f0a1ed0b6088dceceb90a6def8004043f` — Chandra, Bold Pyromancer
- `fe48f0aede49f0479a8a7dde79af26506133927d3933aec48b584f70cbd22d4b` — One with Nature
- `ff49caa60a1fd0adf6c91f82a534b33db0a50e5154f67ff0b6623c7e1e7f51e8` — Appa, the Vigilant
