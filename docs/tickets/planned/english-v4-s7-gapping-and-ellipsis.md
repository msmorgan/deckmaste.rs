---
needs: []
---
# Stratum 7: Gapping and ellipsis

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read gapped verbs and predicates across conjuncts:
- Multiclass Baldric, Scion of Draco, Dominaria's Judgment and Tribal Golem;
- *and so on for ...* (Akroma, Vision of Ixidor; Selective Adaptation);
- *only ... and only ...* (Detention Vortex, Oft-Nabbed Goat);
- elliptical *but not* and *but only* (10 faces: Lightning Storm, Mana Cache, Soul Burn, Armageddon Clock, Drain Life, Endbringer's Revel, Haakon, Infinite Hourglass, Scandalmonger, Task Mage Assembly, Volrath's Dungeon, Well of Knowledge);
- the unlike serials (Grizzled Wolverine: predicate adjuncts *only during ..., only if ..., and only once each turn*; Invasion of Arcavios: an NP and a PP under *and/or*; Monument to Perfection; Hearth Elemental; Frantic Firebolt);
- Ulasht, the Hate Seed's elided second *with*;
- Dralnu's Pet (*enters with flying and with X counters*, a *with*-keyword phrase as a dependent of *enter*; a complement frame for *enter* breaks Master Biomancer's *with ... and as a Mutant*, and a predicate host for *with* plus a keyword grows 400 units, so find a general construction);
- Everything Comes to Dust (a fused-head *those* with a relative clause, and the scope of *except* in a serial object);
- Opportunistic Dragon and Gandalf, Westward Voyager (the scope of mixed-mood serials whose first conjunct ends in a subordinate clause that could take the rest);
- Worms of the Earth; Moritte of the Frost;
- gapped *get* over two Object + Adjunct strings (*each get +1/+1 for each creature you control and +1/+1 for each Aura you control*), Eidolon of Countless Battles. Today it reads only through *for each [creature you control and +1/+1]*. Add the gapped analysis beside that Reading. The other Reading is a grammatical attachment alternative, so it stays admitted (principle 1);
- Felisa, Fang of Silverquill: add the second analysis of *the number of counters it had on it* (*have* with an object and a locative *on*-phrase, *it had __ on it*), without a general *have NP on-PP* frame. The Stratum 6 reading (`RelativeClause "it had" gap "counters"`, `perfect_relative_adverbial_gap.txt`) stays admitted.

Gapping is general English and also the likeliest forest explosion: measure before and after (principle 5) and report.

## Grounding

- CGEL, Ch. 15 §4, p. 1338ff.
- CGEL, Ch. 17 §7.8.4, p. 1542.

## Witness cards

- Multiclass Baldric
- Dominaria's Judgment
- Lightning Storm
- Grizzled Wolverine
- Eidolon of Countless Battles
- Felisa, Fang of Silverquill

## Estimated faces

About 27 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

Likely.

## Pending judgments this slice makes hold

None are pending against this slice. Add a positive judgment for each witness (`crates/deckmaste_english_v4/judgments/`) asserting its intended analysis.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
