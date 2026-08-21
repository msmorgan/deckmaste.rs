---
needs: []
---
# Decide the kind join's shape: flat, pair-carrying, or powerset

The semantics core takes the join-lattice orientation — a cross-kind mention is
a join (`AnObject \/ APlayer`), not a marked constructor. That direction is
settled by
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
and is **not** what this ticket decides. What it decides is the first of the
three execution shapes that ADR pins open: **what the join evaluates to and what
the joined kind carries.**

The evidence is already measured, and it is inlined here because it is the whole
specification for the decision:

- Flat `_ \/ _ = Anything` **loses which union**. Site 4 measures the
  demonstrative echo copying its antecedent's kind pair **33 of 33** with no
  crossing in either direction, and the other 15 readbacks — those with no pair
  to echo — write the generic "permanent or player". A flat join cannot spell 33
  of the 48 readbacks at all.
- A naive powerset join spells them **wrongly**: it gives the class word
  [CR#115.4] the set `{Creature, Player, Planeswalker, Battle}`, whose faithful
  echo is "that creature, player, planeswalker, or battle", which no card
  writes, against the 10 class-word readbacks that write the generic pair.
- Site 2's 2×4 admissible-pair grid survives under a pair/powerset join **as a
  subset restriction**, and is destroyed under flat.

The assembled verdict is that flat is off the table for this grammar — no site
was found where flatness helps — and that, if the lattice is taken, it is a pair
or powerset join. That is a strong prior, not the decision; take the decision on
the tables.

## Scope

- Define the join on `Kind` in `idris/src/Experimental.idr`: the operator, what
  the joined kind carries, and the algebraic laws (idempotence, commutativity,
  associativity) stated as proofs or as pinned refusals.
- Record the chosen carrier's consequence for the 33/33 echo and for site 2's
  8-cell grid — either derivable from the carrier, or explicitly routed to
  `workbench-unhomed-union-gates` as a table that must be written down.
- Do **not** migrate any construction here. `AnyTarget`, `KindJoin`, `YouAnd`
  and the `That`/`UnionP` anaphor stay exactly as they are; this ticket lands
  the index and its laws underneath them.

## Consumption boundary

`idris/src/Experimental.idr` (the `Kind` index and its readers) and
`idris/src/Experimental/Words.idr`. No Rust crate is touched.

## Acceptance

- The join is total, its laws are stated, and the decision names which of
  flat/pair/powerset was taken, with the measurements that decided it.
- Every existing witness and pin still holds: `idris/scripts/build` PASS with no
  witness lost and no pin silently passing.
- The decision is written where a claimant of the downstream tickets will find
  it — a docstring on the join, not a note elsewhere.

Standard constraints apply.
