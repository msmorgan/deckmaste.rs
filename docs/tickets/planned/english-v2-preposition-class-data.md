---
needs: [english-v2-pp-construction]
---
Fix the preposition-class data and close the reopened overgeneration
(pp-landing-review H1/H2/M2/M3). The general PP mechanism stands; its data
and guards do not.

- H2 first: every `vocab Preposition` member carries an EXPLICIT declared
  class — no member may inherit the vocab default. Assign per the
  2026-09-02 amendment (in, on, under, at, during adjunct-capable; to,
  into, onto, of selected/postmodifier) and MEASURE each assignment
  against the corpus before declaring it; a member whose corpus behaviour
  contradicts the amendment's list is a STOP with the census, not a quiet
  reassignment. Split the conflated enum into two orthogonal features:
  PrepositionAttachment (adjunct-capable | postmodifier-only |
  selected-only) x BareLocativeComplement (yes | no), so `in` can be both
  adjunct-capable and bare-licensing.
- H1: the temporal trigger prefix and every preposed/adjunct site require
  the adjunct-capable attachment feature; "Into your graveyard, draw a
  card.", "Of your library, …", "To target player, …", "Under your
  control, …", "Among them, …" must all reject. §6 probe #24 ("Draw a card
  at the beginning of your end step.") must parse.
- M2: delete the zero-data `NounComplement` feature (a duplicate of the
  live `Relationality` feature). License `of` (and any noun-selected
  preposition) by relaying the head noun's declared complement licence up
  the NP stages to the postmodifier site — the stages already relay
  agreement/number/onset that way — so "Creatures you control of the
  chosen type" and "a nontoken creature of their choice" parse while
  "Draw a card of your library.", "Sacrifice a creature of your hand.",
  "You gain 2 life of your library.", "Destroy target creature on your
  hand." reject. Add those four as permanent negative oracles.
- M3: implement the attachment rule, don't inherit it from specificity —
  "Sacrifice a creature during your upkeep." and "Draw a card for each
  creature you control." each currently yield two readings with nothing
  pinning the survivor; the ruling (nearest licensing constituent) must be
  a derived structural decision, and a genuine two-survivor case is a
  STOP. Record the unique/specificity counts before and after.
Acceptance: coverage >= 15,966 with zero counters, all named oracles in
both directions, landing record with deviations. Standard constraints
apply.
