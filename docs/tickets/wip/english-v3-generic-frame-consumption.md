---
needs: [english-v3-reading-test-helper]
---
# Consume ordered lexical Verb Frames compositionally

Replace whole-frame matching against a handwritten grammar alias catalogue with
shared composition over declared ordered Frame Slots. Retain lexical syntactic
selection, slot relations, Categories, marker ownership and chosen frame identity.
This is not permission for every verb to take arbitrary Complements, nor a request
to add an alias and named predicate variant for every newly encountered shape.

Pinned witness: Orcish Bowmasters' “Then amass Orcs 1.” The exact constituent
“amass Orcs 1” has zero Secondary Verb Phrase Readings, while “Orcs” as a Noun
Phrase and “1” as Amount each have one. The lexical NP-plus-Amount frame already
exists; its whole shape has no active grammar alias. Implement this through the
keyword action's existing macro-owned declaration, not duplicate vocabulary.

The exploration tree qymzymor (parent sqnmlrnz; grammar baseline vwtnryos,
covered 13,226) loads 120 verbs, 192 frame assignments and 51 distinct shapes.
Only 24 shapes / 160 assignments match active aliases; 27 shapes / 32 assignments
lack matches. These counts are an interface inventory, not promised coverage gains.
Audit every unmatched shape: legacy FrameComplement and ReplacementMarker payloads
need typed reconciliation before consumption. “Remove a +1/+1 counter from this
creature” from Triskelion also fails, but its legacy placeholder makes it a less
clean witness. Reins of the Vinesteed's “shares a creature type with that creature”
already parses through transitive-plus-Adjunct routes; restore the selected With
Complement analysis rather than counting mere recognition as success.

Design the shared consumer at the declaration/compiler and packed-admission layers:
consume typed ordered slots without constructing eager AST products, and generate
checked construction, realization and traversal from the same declarations.
Inspect exact-equality projection in deckmaste_construction_v3_core/src/emit.rs.
Preserve correlations needed by selected Complements and frame coordination;
coordinate with english-v3-frame-coordination without absorbing its sharing work.
CGEL Ch. 4 sections 1.1–1.2, pp. 216–228 distinguishes syntactic licensing from
semantic plausibility. Ordinary noun/verb meaning is not an admission guard.

Acceptance: independently construct the attested Amass Reading and prove both
roundtrip laws; test typed slot order and marker mismatches meaningfully; restore
selected roles for contrasting authentic constituents. Reconcile the unmatched
inventory and report remaining owners, identity-level corpus changes, all retained
Readings, internal failures and construction/declaration economy. Old scratch
reports in /tmp/english-v3-frame-probes are optional; regenerate evidence from the
implementation tree. Standard constraints apply.

## Scope bound

Three deliverables: the shared ordered-slot consumer, the Amass witness with the
contrasting selected-With constituent, and a report of every frame shape still
unmatched afterwards with the ticket that owns it. "Reconcile the unmatched
inventory" above means that report. Typed reconciliation of the legacy
`FrameComplement` and `ReplacementMarker` payloads is routed to an owner, not
done here; a shape that depends on them stays an explicit unsupported
diagnostic.

## Sequencing

This is the one queued ticket that changes how existing Readings are
represented, so it goes first among the grammar tickets and after the shared
test helper: write the pinned witnesses with the constituent assertion before
implementing, and use it for any existing test that has to be re-spelled.
The landing compares corpus Reading identities before and after on its own tree.

## Prior attempt

The bookmark `archive-english-v3-compact-grammar` contains an unfinished attempt
at this ticket, made in one change together with six others and never verified
against the corpus. It replaced thirteen ordinary predicate shape schemas with a single
`SelectedPredicate` schema, re-spelled the existing structural tests against it,
and reported 12 unsupported shapes / 13 assignments remaining; it ran no
full-corpus identity comparison and no gate. Read it for ideas if useful. Do not rebase onto
it, and treat every claim in its ticket notes as unverified.
