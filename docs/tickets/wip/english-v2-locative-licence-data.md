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
   kinds, turn parts, closed core nouns) declare its locative/temporal and
   `of` licences from the noun's CLASS SEMANTICS (zones license their
   locative preposition; turn parts their temporal ones; object nouns —
   permanent/card/spell types and every subtype — license the object
   attachments `of`-partitive/`of the chosen type`/`from among`/`on` that
   objects take; counter kinds license `on <host>`), with the corpus
   census used ONLY as a completeness check that no attested attachment
   lacks a row — never as the authority on admissibility ("Draw a card
   of target player." must still reject by the head's class, not by an
   absent census row). Mechanism for 462 subtypes: a per-CLASS declared
   value inherited by the class's members, declared once — that is
   declared data, not an inventory default; an undeclared class is a
   compile error. Relational `of` via Relationality, declared
   per head. Record the census per noun class in the landing record.
2. Then the mechanism (already designed; re-land from the reverted change
   `xlnzwtro` where correct): relay, no defaults, H2 complement licences,
   H3 existential guard, M3 attachment as a decision (a locative attaches
   to the NP when the head licenses it, else to the predicate — "Destroy
   each creature on the battlefield" attaches to the NP).
3. ZERO-RETIREMENT PRECONDITION: coverage must not drop below 16,237 at
   integrate AND the lock diff shows -0 rows (that is the check that
   catches loss). Any unit that would be lost is a STOP with the unit text and
   its readings — the coordinator decides retirement per unit; the
   authenticated retire path is never used without that per-unit ruling.
   If retirements are needed, they are a separate ticket.
4. Negatives, inline (all must REJECT): "Draw a card of target player.",
   "Draw a card of a Goblin.", "Destroy target creature on an artifact.",
   "Destroy target creature on target player.", "Draw a card of your
   library.", "Sacrifice a creature of your hand.", "You gain 2 life of
   your library.", "Destroy target creature on your hand.", "Under your
   control, draw a card.", "Into your graveyard, draw a card.", "Of your
   library, draw a card.", "There is a creature into your graveyard.",
   "Draw a card for from your graveyard.". Also from the genitive landing:
   "You gain life equal to that creature's power." and "…that card's mana
   value." must SELECT (the genitive family relays the licence).
   Re-land the mechanism from the reverted change `xlnzwtro`
   (`jj diff -r xlnzwtro`), correcting it per this ticket.
5. Positives are positives: "Creatures you control of the chosen type",
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
