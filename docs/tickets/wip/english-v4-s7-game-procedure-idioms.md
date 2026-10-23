---
needs: []
---
# Stratum 7: Game-procedure idioms

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read the game-procedure one-offs. Each witness gets a clean general production or a catalog-declared lexical idiom, with no policing flag (`...Use`, `...Licence`, `...Present`). If a witness cannot be read that way, stop and surface it to the user to choose between a catalog idiom, a general production or an explicit documented deferral.
- dice and coins: Berserker's Frenzy, Iron Mastiff, Banshee (*half X*), Fiery Gambit, Game of Chaos, Mr. House, Mathemagics (*2ˣ*), Scrying Glass;
- ties and life totals: Loxodon Peacekeeper, Celestial Convergence;
- game procedure: Karn Liberated (restart), Time and Tide (*Simultaneously*), Agent of Acquisitions (draft), Multiple Choice (*all of the above*), The Enigma Jewel (Craft parameter), Ob Nixilis, the Adversary, Tameshi, Reality Architect, Withering Wisps, Elenda, Saint of Dusk, Avatar of Woe, Spikeshell Harrier, Tempt with Mayhem, Tranquil Frillback, Illusionary Mask, Portal Manipulator, Midnight Crusader Shuttle (villainous choice dash alternatives).

## Grounding

- `docs/oracle-style-guide.md` §12, "Randomness, dice, coins, and voting". Per witness otherwise.

## Witness cards

- Berserker's Frenzy
- Banshee
- Karn Liberated
- Multiple Choice
- The Enigma Jewel

## Estimated faces

About 24 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

No. Grammar and lexicon.

## Pending judgments this slice makes hold

None are pending against this slice. Add a positive judgment for each witness (`crates/deckmaste_english_v4/judgments/`) asserting its intended analysis.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.
