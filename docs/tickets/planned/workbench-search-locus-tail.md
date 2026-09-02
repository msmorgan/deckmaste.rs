# workbench-search-locus-tail

The three remainders of the search-locus round (workbench-event-zone-and-cast-
provenance, 2026-09-02). The condition voice of the which-zone reader landed
(`EventComplement.AtZone`, `VerbFacts.actLoci`, `Macros.happenedAt`); these
are what still blocks the and/or-search cycle whole. Re-measure at claim.

Standard constraints apply.

## 1. The searched-zone read's EVENT seat — 5 supported lines

`GameEvent.VerbedEvent` carries no zone slot, so the locus fact `actLoci`
already states cannot be written at the header:

- "When you search your library this way, put a +1/+1 counter on target
  creature you control" — Prishe's Wanderings, a `ThisWay` trigger.
- "Whenever an opponent searches their library, …" — 3 lines.
- "If an opponent would search a library, …" — 1 line, a replacement.

Shape: a fifth positional slot on `VerbedEvent`, `Maybe (ZoneExpr …)`, gated
`So (lookbackLocusOk (VerbedAct v) (zoneSort z))` — the same gate the
complement takes. Cost is the arity bump across 18 call sites plus the four
`GameEvent` tables. Declined on the count in the landing round, not on the
shape; the possessive "their library" is reachable here because the header's
subject is in scope where a description's is not (item 2).

## 2. The SELF-POSSESSIVE zone inside a description — 8 supported lines

"Then each player who searched **their** library this way shuffles." The
predicate writes (`Macros.happenedToAt`), but no noun names the described
player's own zone from inside the description describing them: `They` reads
an outer mention, and `Each p`'s predicate is elaborated in the outer
bindings. Boldwyr Heavyweights, From the Ashes, Hired Giant, Natural Balance,
New Frontiers, Noble Benefactor, Rootweaver Druid, Wave of Vitriol.

A description-layer gap and not a zone one; it will bind any later round that
wants "each player who controls their own X". Each of the eight additionally
wants a per-player optional search, so none is a one-item whole card.

## 3. "Reveal it" inside an enters trigger — the recency anaphor

`Enters this creature` leaves an Object mention, so a following "reveal **it**"
over a searched card puts `countOnes Object` at 2 and the pronoun is refused.
Delivery Moogle is blocked on this and nothing else, and so is the rest of the
and/or-search creature cycle: Ashiok's Forerunner, Chandra's Firemaw,
Domri's Nodorog, Elspeth's Devotee, Ethereal Elk, Fang-Druid Summoner,
Garruk's Warsteed, Goldmane Griffin, Niambi Faithful Healer, Rowan's
Stalwarts, Sorin's Guide, Teferi's Wavecaster, Tower Winder, Yanling's
Harbinger. The sorcery-voiced twins (Vraska's Scorn benched, Liliana's Scorn,
Tezzeret's Betrayal, Gideon's Battle Cry, Ral's Dispersal) write today,
because their first sentence names a player rather than a permanent.

The tree's existing narrowings are the place to look: `ItAt`'s slot carriers
narrow by the CONSUMING verb's own rule, and a reveal shows a CARD
[CR#701.20a], so a `revealsIt` at `CardSlot` may be all this costs. Measure
whether the reveal's carrier is the honest one before minting a recency
ranking — the tree refuses recency deliberately (`ItPrior`'s docstring).

## 4. Recorded, not a gap

`Shuffle`'s `actLoci` cell is `[Library]` from [CR#701.24a] and read by
nothing: 0 supported lines write "shuffled your library this way". Stated from
the rule, as the round's tables are.
