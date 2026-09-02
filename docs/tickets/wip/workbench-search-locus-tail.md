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

## As landed

Corpus authority: `data/derived/cards.jsonl` filtered
`jq 'select(.supported)'`, re-measured 2026-09-02. CR text via
`data/rules/`.

### 1. The event seat — DECLINED again, the count did not move

Re-measured, and it is the same **5** lines the round declined on:
1 `ThisWay` trigger (Prishe's Wanderings), 3 opponent-search headers,
1 replacement ("If an opponent would search a library"). Nothing moved
and no cheaper seat is available: the honest shape is still a positional
slot on `VerbedEvent`, and that row grew one this round already (the
transform tail's `becomes` complement), so the arity would go to six for
five lines.

A wrapper row on `NthOccurrence`'s model was considered and rejected on
its own terms rather than on cost: `NthOccurrence` earns its wrapper by
serving every countable event, and a locus serves exactly one event
family (`lookbackLocusOk` answers False for every name but a verbed
act), so a slot on the row that can carry it is the honest shape.
**Restated so the next round does not re-derive it.**

### 2. The self-possessive zone inside a description — DECLINED, count holds

Re-measured at **8** and the same eight cards. It is not a zone gap and
not a cheap one: nothing in the noun vocabulary names the DESCRIBED
player from inside their own description (`SelfD` exists at the object
kind for the source's own deixis and has no player counterpart), so it
wants a description-layer row plus its binding, and by the round's own
note none of the eight becomes a whole card even then — each
additionally wants a per-player optional search.

### 3. "Reveal it" — LANDED, and the carrier is not the one the ticket guessed

`Macros.foundCard` / `Macros.revealsIt`, at the **producing** verb's
stamp (`ItVerbed "Search"`), not at `CardSlot`.

**The ticket's `revealsIt`-at-`CardSlot` guess was measured and it does
not work.** `CardSlot` is a ZONE test (`slotZoneOk CardSlot = isCardZone`
over `bindingZone`), and a coordinated search fixes no zone for what it
finds — `searchZone (SomeZones _ _) = Nothing` — while all 15 supported
carriers write "search your library **and/or** graveyard". So the card
carrier reaches the searched mention at neither zone and
`countOnesAt CardSlot` is 0, not 1. [CR#701.20a]'s reveal does show a
card and [CR#701.23a]'s search does find one, but neither fact has a
zone to be spelled as here.

The carrier that works is already in the tree and is the rule's own:
[CR#701.23e] ties the two clauses together and names the referent —
"if the effect that contains the search instruction doesn't also contain
instructions to reveal **the found card(s)**, then they're not
revealed". `ItVerbed` narrows by the producing verb's stamp, which is
exactly what a search leaves, so **no new machinery was needed and no
recency ranking was minted** — the measurement the ticket asked for came
back "the reveal's carrier is not the honest one; the search's is".

The same carrier writes the following "put it into your hand", which had
the same two candidates.

- Bench: **Delivery Moogle whole** (`deliveryMoogle`) and **Tower Winder
  whole** (`towerWinder`) — the cycle's two non-optional members. The
  other 13 (Ashiok's Forerunner, Chandra's Firemaw, Domri's Nodorog,
  Elspeth's Devotee, Ethereal Elk, Fang-Druid Summoner, Garruk's
  Warsteed, Goldmane Griffin, Niambi Faithful Healer, Rowan's Stalwarts,
  Sorin's Guide, Teferi's Wavecaster, Yanling's Harbinger) write the
  identical three clauses under a "you may".
- The stale `deliveryMoogleSearch` fragment is gone, its content now in
  the whole card.

### 4. `Shuffle`'s locus cell — unchanged

Still 0 supported lines read it. Re-measured, still stated from
[CR#701.24a].

### Remainder

Garruk Relentless // Garruk, the Veil-Cursed is NOT benched whole, and
neither the state trigger nor the reveal is why any more — both landed
this round (`garrukRelentlessFlip`, `Macros.foundCard`). Its back face's
[−1] writes "Sacrifice a creature. **If you do**, search your library
…", and an "if you do" hanging off a MANDATORY instruction has no seat:
`mayThen`'s did-branch belongs to a "you may", and nothing else reads
whether a flat instruction was carried out. A separate gap in the
optionality family, not this one.
