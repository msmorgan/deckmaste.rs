---
needs: [english-v2-frame-complement-coordination, english-v2-scope-device-collapse]
---
# Conjunct-boundary anchoring in a frame-complement-pair Coordination

**Routed 2026-09-05 from the `english-v2-frame-complement-coordination` (B7a)
landing review, finding R1; pinned 2026-09-05 by coordinator referral.**
Authority: `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md`
§D and §A.3-S7 ("Conjunct-boundary anchoring, and a correction").

## Defect

When a Conjunct of the frame-complement-pair Coordination has an Object that is
itself a Coordination joined by the same Coordinator, the string carries two
derivations, no tie is reported, and the generic specificity ordering picks one.

Witness — Arwen, Mortal Queen (`f9afecf8…`), `Put a +1/+1 counter and a lifelink
counter on that creature and a +1/+1 counter and a lifelink counter on Arwen.`
With the three Coordinator tokens written `c1`, `c2`, `c3` in linear order: the
selected derivation lets `c3` anchor the cluster and `c2` anchor a Coordination
inside the first Conjunct's marked Complement, so one Conjunct absorbs the next
Conjunct's Object; the unselected derivation lets `c2` anchor the cluster and
`c1`/`c3` anchor the two Conjuncts' Objects. Forest node
`AndFrameComplementPairCoordinationMembersSequencePair` carries both families;
candidates 19 and 20 differ only in that bracketing. Not a regression — the
parent selected the same grouping under the pre-B7a spelling. B6's principle (ii)
does not reach it: the swallowed material carries no role preposition. One corpus
identity is affected today.

## Pin

**English is ambiguous here and it packs.** §A.3-S7 records the judgement and why
each candidate boundary rule was refused: a same-Coordinator prohibition on a
Conjunct's marked Complement is false against attested English, a leftmost-split
rule is a preference weight and wrong, full structural parallelism between
Conjuncts is too strong, and "a Conjunct ends at its marked Complement" is true of
**both** derivations. No fact of English fixes the boundary; a reader
disambiguates on rules meaning, which Semantics carries and the grammar does not.

The pair already satisfies the scope device's partition-anchored equality
(§A.3-S1..S6) unchanged: the Coordination anchors pair one-for-one and each
difference region decomposes into a contiguous prefix and suffix at the
Coordination's edges. No new move, no new equality, no widening of the scope
class.

Two deltas are this ticket's work.

1. **Lift A.3-S3a for this sub-case.** The collapse admits a Move S pair only
   when a non-empty difference region is the yield of a declared mobile role;
   that condition separates the shared-Constituent sub-case (which the collapse
   ships) from this conjunct-boundary sub-case (which it defers). Lift it here,
   for pairs whose difference regions are edge-contiguous and whose anchor
   bijection is total.
2. **Add `AdmissibleAnchorings`.** A second derived value on the outer
   Coordination node, sibling to `AdmissibleSites` and under the same rules:
   derived, never parsed, no `RulePosition`, no specificity tier, no leaf, no
   bytes, always present, empty when unambiguous, no new AST Category. Its domain
   is the set of alternative **anchor tuples** — the Coordinator leaf positions
   that anchor the outer Coordination in an alternative member. One tuple is a
   complete description: the outer Coordinator fixes the Conjunct split, each
   Conjunct is parsed against the frame's declared role sequence, and the
   remaining Coordinators anchor whatever that parse requires. `AttachmentSitePath`
   cannot express this — nothing changes host — which is why the shared-Constituent
   sub-case's "no second derived value" does not extend here.

**Representative.** Both members' outer Coordinations span the whole cluster, so
§C.3's dominance rule does not order them. Canonicalize on the
lexicographically earliest outer anchor tuple. State in the landing record that
this is a normal form and not a claim about meaning; that it delivers the
coherent reading on the one witness is a side effect, and Semantics still reads
`AdmissibleAnchorings`.

## Witnesses

- **Packs, one candidate:** Arwen, Mortal Queen (`f9afecf8…`) — assert the pack,
  the representative's anchor tuple, `AdmissibleAnchorings` carrying the other
  tuple, byte-exact render, unchanged leaf count, and `unresolved_ties == 0`.
- **Must NOT pack:** a flat *n*-ary Coordination against a nested binary one over
  the same Coordinator tokens. The flat node's anchor is the tuple of all its
  Coordinators, so A.3-S2's bijection fails. Assert it; this is the fence.
- **Non-regressing sample of B7a's 85 gains:** the three gains whose cluster
  region carries a second connective (all `or`, structurally unable to split, per
  the B7a record) plus a uniform sample of at least twenty of the remainder.
  Assert every selected path unchanged.
- **The collapse's witnesses are unaffected:** re-run phase 3's must-pack and
  must-not-pack sets and assert no movement.

## Also carried here (B7a review R3, LOW)

The compiled-consumer fixture (`crates/deckmaste_construction/tests/compiled_consumer.rs`,
`declaration_verb_fixture`) has no `FrameComplementPair` case; adding one needs a
positional checked sequence the fixture does not yet have. The emitted seam is
already exercised end to end by `deckmaste_english_v2/tests/predicate_grammar.rs`
(`declared_frame_complement_pairs_coordinate_for_every_coordinator`) and at
generation level by `emit/build.rs::checked_sequence_reads_a_structurally_matched_frame_role`.

## Fences

- No `require`, `checked by`, comment or literal naming a lexeme, Construction,
  verb, noun, preposition or card; a permitted guard reads a declared feature or
  a declared role property.
- No `SELECTION_EXCEPTIONS` entry, no dominance edge between named Constructions,
  no new specificity weight or tier, and no boundary preference. The device is
  elimination plus collapse.
- No new AST Category, and no field holding an alternative subtree.
- `packed_units` is never a tie's escape hatch; a non-scope tie is still a STOP.
- Attestation is provenance: the single corpus witness sets neither the value's
  domain nor the rule's reach.

## Report

The corpus census for the landing: units gained, lost, or with a changed selected
path, and `packed_units` before and after with its identities.

Standard constraints apply. Gate scope: `cargo test --workspace`.
