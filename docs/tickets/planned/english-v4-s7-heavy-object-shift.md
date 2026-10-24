---
needs: []
---
# Stratum 7: Heavy Object shift past a selection PP

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read an Object postposed past a source or selection PP of a monotransitive verb: *You choose from it [a nonland card with mana value 3 or less and a card with mana value 4 or greater]* (Distended Mindbender) and *Each player chooses from among the permanents they control [an artifact, a creature, an enchantment, and a land]* (Cataclysm). Both were split out of `english-v4-s7-selection-idioms`.

The existing Valence `heavy_shift` postposes an Object only past a frame's selected PP Complements, so a monotransitive *choose* gets no shifted order.

## Settled

A first attempt added a grammar Production postposing any multi-leaf Object past any Predicate-hosted PP with an NP or PP Complement (`ObjectAfterAdjunct`, cost 2). It read Distended Mindbender but added 2,144 Readings across 475 already-read units. Most of those shift a light NP (*this turn*, *this game*) across a long PP (*enter as a copy of … this turn*), and that analysis became the cheapest Reading of The Fourteenth Doctor. Do not reland it in that form.

Postposing is driven by weight (CGEL, Ch. 16 §4, pp. 1382–1384): the postposed Object must be heavy relative to what it crosses. The construction needs a weight condition that excludes the light shifts above. That condition might be relative weight, or a postmodified or coordinated Object. Alternatively, a *from*-PP frame for *choose* could let the existing `heavy_shift` apply, provided that frame is justified. Measure either option (principle 5).

## Further faces (triage of the unread faces, hypotheses to confirm against the code and a parse)

- **Tragic Arrogance, Guided Passage, Global Ruin**: the same *chooses [from among …] [an artifact, a creature, …]* shift as Cataclysm, over a *from among* PP.
- **Pyxis of Pandemonium**: *turns face up all cards they own exiled with this artifact*. The heavy Object shifts past the particle *face up* rather than a PP, so the weight condition must also cover a particle.
- The *Choose from among them a card with flying, …* face listed in `english-v4-s7-gapping-remnants` shows the same shift; its card is carried by `english-v4-s7-gapping-remnants`, and a fix here should be checked against it.

Each diagnosis is a hypothesis: confirm it against the code and a parse before changing anything.

## Witness cards

- Distended Mindbender
- Cataclysm
- Tragic Arrogance
- Guided Passage
- Global Ruin
- Pyxis of Pandemonium

## Estimated faces

About 6 faces.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
