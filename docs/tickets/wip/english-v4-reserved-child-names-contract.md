---
needs: []
---
# Document the reserved child names as the grammar contract (R2)

From `english-v4-child-name-control-and-category-fission-audit`, findings A
and B. The owner ruled R2 in round 9.

## Owner's ruling (round 9, R2)

"B-a. Keep the three renames (nominal on DistributiveCoordinationDetermined;
quantity on TransparentQuantity, TemporalQuantity), comment each, and document
on ProductionDecl the reserved child names the engine inspects (per the audit:
head, first, last, rest, adjunct, modifier, supplement, complement, object,
shared) and what each triggers."

R5.5 follows this ruling: the `Targeting` value and the TargetingMarker
Category stay as they are. B-b (a per-Production opt-out) and B-c (an explicit
`Agree` overriding head agreement; probe P2 loosened the partitives, +264
Readings in 19 units) are rejected.

A child's name is its grammatical function (`Production::names`), and the
engine keys behaviour on some of them. Nowhere is that listed. Authors find out
by accident, and rename a head to dodge implicit agreement. The renamed child
then also loses its head lexeme in Readings and its place in the head line.

## Change (documentation only)

1. Document the reserved names on `ProductionDecl` in
   `crates/deckmaste_syntax/src/grammar.rs`. This doc comment is the contract.
   List each name the engine inspects, what it triggers, and the code site of
   each:
   - `head`: implicit agreement (`compile.rs`, `add`; the `Equation` doc);
     preposition and extension inheritance (`step.rs`, `resolve`); the head
     line; the Reading head lexeme (`reading.rs`, `head_word`); host words for
     Right Association; function-Category wrapping.
   - `first`, `last`, `rest`: the head line; conjunct balance and the
     coordination Attachment Class; the fronted-scope rule
     (`comma_scope_excludes`).
   - `adjunct`, `modifier`: unselected dependents for Right Association and
     the preposed-dependent Attachment Class. `adjunct` also marks the
     post-verbal Adjunct host that an auxiliary Verb Phrase never is
     (`post_adjunct_host`).
   - `supplement`: reversed Right Association.
   - `complement`: carries a delimited Clause coordination up.
   - `object`, `shared`: the Reading functions of postposed Modifiers and of
     Right Node Raising (`reading.rs`, `locate`).

   State that a Production that names its semantic head otherwise forgoes
   every `head` item. Point the `SymbolDecl` doc and the `Equation` doc's
   "a child named `head`" sentence to this list. In the `deckmaste_syntax`
   README, replace any list of names with a pointer to `ProductionDecl`.
2. In `crates/deckmaste_english_v4/grammar/english.ron`, give each of the
   three renames a comment. Name the features it keeps out of head agreement,
   say why, and say what it forgoes (the head lexeme and the head line):
   - DistributiveCoordinationDetermined (`nominal`): Number, NounKind,
     Countability and Determination. The Noun Phrase is singular from its
     distributive determiner, while its *and*-coordination is plural (CGEL,
     Ch. 5 §18.4, p. 508).
   - TransparentQuantity (`quantity`): Number and NounKind come from the
     oblique (`ObliqueNumber`, `ObliqueNounKind`; CGEL, Ch. 5 §3.3,
     pp. 349–351).
   - TemporalQuantity (`quantity`): as TransparentQuantity, with NounKind
     `Temporal` from the oblique.
3. Fix comment drift in `english.ron`:
   - the LocativeHead comment sits above SymbolHead (about lines 130–131);
   - the FusableDeterminer comment sits above PositionPhrase (about lines
     195–196);
   - the NounPhrase comment stops mid-sentence ("… never passes up a Measure
     Gap, or", about line 157).

No grammar or engine behaviour changes. The census and judgment set are
identical.

## Acceptance (round 9: `docs/decisions/english-v4.md` §4.4, item 9)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (item 7). No Attachment Class, Production Cost tweak or
residual ticket is added to win a rank (item 8).

Standard constraints apply.

Census delta (measured on `okprvpoz` -> this landing): documentation and comment changes only; faces read 28,801, total Readings 245,825 and Surplus 106,859 before and after; lost faces: none.
Judgment set (measured on `okprvpoz`): not affected; `cargo xtask english-v4 strata --check` passes and the crate tests pass.
