---
needs: []
---
**The keyword-ability census and the squad repetition shape.** Split from
`semantics-v2-keyword-ability-residues` (done 2026-10-08), whose routed items
landed (offspring bodied; squad's second ability pinned; an impending canon
card) and whose opening paragraph did not.

- **Census.** `semantics-v2-macro-bodies-keyword-abilities`' second landing
  record (`docs/tickets/done/`) left 87 keyword-ability declarations
  bodyless. Name each one with its STOP cause and the ticket that owns it
  (`semantics-v2-subject-param-kind`, `semantics-v2-macro-param-kinds`,
  `semantics-v2-macro-capture-and-plurality`, or this ticket), then take the
  unowned remainder Lean-first: each shape added to `lean/Semantics` with its
  pins, mirrored, then the declaration bodied. Offspring is no longer in the
  list.
- **Squad's first ability needs a ruling.** "you may pay [cost] any number of
  times" [CR#702.157a] has no spelling, since `addedCost` carries no
  repetition. Candidate (a): a `PayTimes` field on `addedCost`, shared with
  replicate [CR#702.56a] and multikicker [CR#702.33c]. Candidate (b):
  `addedCost(perform(pay(cost, AnyNumberOfTimes)), true)`, which needs no
  model change but makes `timesPaid` count one payment of a compound cost.
  Once decided, body squad with the second ability already pinned
  (`Proofs/KeywordDefinitionRegimes.lean`, `okSquadSecondAbility`).

Standard constraints apply.
