---
needs: []
---
# Write the combat-assignment declarations and the forced attack's missing cells

Combat assignment is one region: what a creature could block, what puts a
creature into or out of a blocking assignment, and what a forced attack takes as
its patient and its duration. The two sub-areas below share the deontic deed
catalog and the blocking rules, so they are one claimable unit.

## The could-block condition and the blocks/stops-blocking effect

Two combat-assignment declarations the grammar cannot write: a condition asking
whether a creature could block, and an effect that puts a creature into or out
of a blocking assignment. Same region, same packet.

- **The could-block condition.** Resolution-time. It reads blocking
  RESTRICTIONS and tapped-ness, and it does NOT read requirements or costs.
  That refusal is part of the specification, not an omission to fill in later.
- **The blocks / stops-blocking write.** The row must express the difference
  between removing a creature from combat and writing it back in, and merely
  stopping it from blocking [CR#509.3a].

Figures and witnesses in this sub-area are a prior session's measurements;
re-measure before building.

## The forced attack's next-turn duration and its named patient

The loyalty frame landed and Gideon Jura is down to one ability. Its [+2] wants
two cells that have nothing to do with planeswalkers, and both are probed rather
than assumed.

- **"During target opponent's next turn"** — a duration the `Duration` catalog
  has no row for (probed, refused).
- **The `DeonticPatient` cell** for a forced attack aimed at a *named permanent*.
  Recorded from an adjacent sweep: the deontic deed catalog is `Attack | Block`
  and its participant gate demands a card type, so the patient side is where the
  work is.

Corrections carried so no successor re-buys them: the card's [0] was long
diagnosed as wanting "the source as a damage recipient, `DamageRecipient`
refusing `This` bare and ascribed alike"; the ascribed half of that was **false**
— an ascribed self is a legal recipient and the corpus writes it **30 times**.
What refused was the planeswalker ascription at `DamageableTy`, one row, now
landed, and the [0] writes whole on Gideon, Ally of Zendikar. The prevention
machinery was never the blocker.

## Consumption boundary

`idris/src/Experimental.idr` (the condition and effect rows, `Duration`,
`DeonticPatient`, the deontic deed catalog),
`idris/src/Experimental/Words.idr` if the write needs a word of its own, and for
`Duration` vocabulary, the pin modules `idris/src/Experimental/Proofs*.idr`
(including pins in `idris/src/Experimental/ProofsD.idr`), and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- The could-block condition's reach into restrictions and tapped-ness is
  gated, and its refusal of requirements and costs is carried by a named
  witness rather than by silence.
- The remove-then-write reading and the stops-blocking reading are
  distinguishable in the written term, not collapsed into one row.
- Gideon Jura benches whole, or the round names what is still refusing it.
- The duration row is measured against the rest of the "next turn" surface before
  it is minted for one card.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
