---
needs: [english-v2-homograph-feature]
---
Locative/temporal licensing as declared data — RE-ISSUE after a REJECTED
landing (locative-licence-landing-review.md, 2026-09-03; reverted). The
first attempt built the mechanism correctly (relayed per-noun licences,
no locative defaults, complement licences, existential guard) but never
populated the DATA: 0/10 types, 1/462 subtypes, 0/31 counter kinds
declared a licence, so 539 printed cards were retired as "forbidden
readings" and the ticket's own positives were flipped into negative
assertions. None of that is acceptable.

DELIVERABLE ORDER (binding):
1. Data pass FIRST: for every noun declaration (types, subtypes, counter
   kinds, turn parts, closed core nouns) derive its locative/temporal and
   `of` licences from the corpus census (which prepositions attach to
   this head in printed text) and declare them explicitly; the 19 turn
   parts are the model. No inventory-level default value of any kind —
   the compiler rejects an undeclared licence on a noun (like the
   preposition class check). Relational `of` via Relationality, declared
   per head. Record the census per noun class in the landing record.
2. Then the mechanism (already designed; re-land from the reverted change
   `xlnzwtro` where correct): relay, no defaults, H2 complement licences,
   H3 existential guard, M3 attachment as a decision (a locative attaches
   to the NP when the head licenses it, else to the predicate — "Destroy
   each creature on the battlefield" attaches to the NP).
3. ZERO-RETIREMENT PRECONDITION: coverage must not drop below 16,174 at
   integrate. Any unit that would be lost is a STOP with the unit text and
   its readings — the coordinator decides retirement per unit; the
   authenticated retire path is never used without that per-unit ruling.
   If retirements are needed, they are a separate ticket.
4. Positives are positives: "Creatures you control of the chosen type",
   "a nontoken creature of their choice", "from among them", "counters on
   this artifact", "cards in your graveyard", "creature on the
   battlefield", "the top card of your library", "a copy of target
   creature" all SELECT; the 13 review negatives all REJECT (see the
   preposition-class review). A printed-card witness is never asserted as
   a failure.
Class D routing (14 hardwired preposition rows) and the collision-metric
row-by-row rule carry over from the first issue. Deviations section
mandatory; landing record numbers read from your own gate output.
Standard constraints apply.
