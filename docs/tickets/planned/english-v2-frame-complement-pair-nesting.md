---
needs: [english-v2-frame-complement-coordination]
design: true
---
# Nesting of a frame-complement-pair Coordination against a coordinated Object

**Routed 2026-09-05 from the `english-v2-frame-complement-coordination` (B7a)
landing review, finding R1.** Authority for the parent Construction: that
ticket and `docs/memory/scratch/plan09-postmortem/b7-scope-device-design.md` §D.

Defect. When a Conjunct of the frame-complement-pair Coordination has an Object
that is itself a Coordination joined by the same Coordinator, the string carries
two derivations and the Coordinator that separates the Conjuncts is not pinned.
Both are derivable, no tie is reported, and the generic specificity ordering
selects the incoherent one.

Witness — Arwen, Mortal Queen (`f9afecf8…`),
`Put a +1/+1 counter and a lifelink counter on that creature and a +1/+1
counter and a lifelink counter on Arwen.`

- Selected: `Put [[a +1/+1 counter and a lifelink counter] on [that creature
  and a +1/+1 counter]] and [[a lifelink counter] on [Arwen]]` — the second
  Conjunct's first Nominal is swallowed by the first Conjunct's Complement.
- Available and unselected: `Put [a +1/+1 counter and a lifelink counter on
  that creature] and [a +1/+1 counter and a lifelink counter on Arwen]`.

Evidence: `cargo xtask english_v2 inspect --id f9afecf8…  --json` — forest node
`AndFrameComplementPairCoordinationMembersSequencePair` spans the whole cluster
with two families, and selection candidates 19 (selected) and 20 differ only in
that bracketing. B6's principle (ii) does not reach it: the swallowed material
carries no role preposition, so frame-role preemption never fires.

Not a regression. The parent selected `VerbPhrasePutOn >
LocativeNounPhraseCoordination` for the same identity, which is the same
incoherent grouping under the pre-B7a spelling. B7a neither introduced nor
removed it. One corpus identity is affected; it is the only unit in the corpus
whose pair Coordination carries a second same-Coordinator Conjunct boundary.

Decision wanted (this is why the ticket is `design: true`). B7a's pin did not
say how the Conjunct boundary is fixed when a Conjunct's own material can
absorb the next Conjunct's Object, and inventing the device in review would
have been an unpinned design choice. Candidates, none endorsed:

- A right-periphery constraint on the Conjunct, in the shape of B6's
  `governed_material_has_no_selected_role_postmodifier`, but keyed on the
  Coordinator rather than on a role preposition.
- A declared preference that fixes the boundary at the leftmost licensed split.
- Leaving it to the scope device, if `english-v2-scope-device-collapse` gives
  the Conjunct boundary a mobile site.

Report the corpus census for whichever device lands: how many units gain, lose,
or change a selected path.

Also carried here from the same review (finding R3, LOW): the compiled-consumer
fixture (`crates/deckmaste_construction/tests/compiled_consumer.rs`,
`declaration_verb_fixture`) has no `FrameComplementPair` case. Adding one needs
a positional checked sequence inside that fixture, which the fixture does not
yet have. The emitted seam is exercised end to end by
`deckmaste_english_v2/tests/predicate_grammar.rs`
(`declared_frame_complement_pairs_coordinate_for_every_coordinator`, three
positive arms and a mismatched-marker negative) and at generation level by
`emit/build.rs::checked_sequence_reads_a_structurally_matched_frame_role`.

Standard constraints apply. Gate scope: `cargo test --workspace`.
