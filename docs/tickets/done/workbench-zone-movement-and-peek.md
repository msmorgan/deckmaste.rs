---
needs: []
---
# Write the multi-zone search, the agentive placement clause, and the peek residues

Cards move between zones and look at them: searching several zones at once,
naming the player who performs a placement (and where in a library it lands), and
the small reads left over the scry/surveil keyword actions. All three sit on
`Search`/`Move`/`ZoneExpr` and the library-position riders, so they are one
claimable unit.

## The three-zone search

`Search` takes one `ZoneExpr`, and sixteen cards across two otherwise unrelated
name families are blocked on nothing else. One construction, and the two families
should be counted together when it is scheduled.

### The Memoricide cluster — 11 cards

Ancient Vendetta, Unmoored Ego, The Stone Brain, Cranial Extraction, Memoricide,
Slaughter Games, Lost Legacy, Stain the Mind, Necromentia, Dispossess, Infinite
Obliteration: "Search target opponent's graveyard, hand, and library for …".
Nothing to do with names — the name half of these cards is spelled. Most of them
sit behind the narrowed chooser as well. (The same eleven are listed as a residue
under [workbench-name-match-family](workbench-name-match-family.md); the
construction is this sub-area's.)

### The same-name family's five — 5 more

Eradicate, Counterbore, Crumble to Dust, Deicide, and Bloodbond March's
per-player return: "Search its controller's graveyard, hand, and library for …".
The co-referential name half is spelled; this is their blocker.

## The agentive placement clause and the top-or-bottom disjunction

146 supported lines name the player who performs a placement, and `Move` has no
subject slot while `Does` has no "put" verb, so every one of them is unwritable.
The library-position disjunction is 38 more lines and 34 of them sit behind this
same clause, so the two go together.

### The AGENTIVE PLACEMENT CLAUSE — "[player] puts [it] into/onto [zone]", 146 lines

`Move` carries a patient and a destination and NO subject. The declarative
clause `Does` needs a `VerbName` tag, and that catalog's seven rows (Destroy,
Sacrifice, Exile, Discard, Mill, Scry, Surveil) have no "put". Measured
carriers:

- "the owner of target nonland permanent puts it into their library second from
  the top" — Deem Inferior and 5 more
- "that player puts that card into their library third from the top" — Lost
  Hours
- "its owner puts it on their choice of the top or bottom of their library" —
  Aether Gust and 30 more
- "that player puts it onto the battlefield" — 11 lines
- "puts it into their hand" — 6 lines

**The question the round must answer first is WHICH of two shapes it is**: a
`Put` row in `VerbName` with a `TagBody` over `Move` — which is how mill and the
three keyword actions reach their placements — or a subject slot on `Move`
itself. The evidence to weigh is that the imperative and the agentive spell the
SAME event, where Destroy's two frames already ride one `TagBody`.

Boundary: the destination's own possessive is rendering's business ([CR#400.3],
finding 34), so "their library" costs nothing here.

### The LIBRARY POSITION DISJUNCTION — "the top or bottom", 38 lines

Deliberately NOT built into the ordinal offset it half-touches. Two spellings:

- 32 lines name their chooser — "its owner puts it on their choice of the top or
  bottom of their library" (Aether Gust, Aetherspouts, Desynchronize, Dire
  Downdraft, Diver Skaab and 27 more).
- 6 write the disjunction against an OFFSET — "puts it into their library second
  from the top or on the bottom" (Deem Inferior, Happy Hogan, Lost Days,
  Temporal Cleansing, Trickster's Stratagem, Wan Shi Tong).

Three facts to start from:

1. The chooser phrase is SEPARABLE — Write into Being writes the bare
   disjunction ("put the other on the top or bottom of your library") — so "on
   their choice of" is its own slot, not part of the coordination.
2. The disjunction is of POSITIONS, not of destinations. The corpus's only
   two-destination lines are Illuna's and Green Sun's Twilight's "onto the
   battlefield or into your hand", 2 lines, a different and much smaller cell.
3. 34 of the 38 carry the agentive clause above and are blocked behind it. The
   four imperatives (Arashin Sovereign, Hinder, Not Forgotten, Write into Being)
   are what a round taking the disjunction alone could land, and three of those
   four name their chooser with "on your choice of".

## The peek residues scry and surveil left behind

The peek family itself is closed: `Scry` and `Surveil` are `VerbName` rows over
`ScryB`/`SurveilB`, whose bodies are [CR#701.22a]/[CR#701.25a]'s LOOK with the
partition recorded and not spelled. Four measured residues survive it (finding
674), each a small read over the keyword action rather than over its body.

### The scrying SUBJECT — 2 lines

The body fixes `You` per [CR#701.22a]'s "your library". "Target player scries 3"
and "…scries X" are the only two lines in 440 scry lines, and ZERO of 219
surveil lines. A subject slot bought for two lines has to justify itself against
that split.

### The PROCESS READ — 5 cards

"The number of cards looked at while scrying this way": a manner adverbial
naming the tag over an inner clause's participle.

### The PER-TURN STATE READ — 2 cards

"As long as you've surveilled this turn" (Darkblade Agent; Eye of Duskmantle) —
`Happened` over a keyword-action event NAME rather than over a game event.

### The COORDINATED EVENT — 2 cards

"Whenever you scry or surveil" (Matoya, Archon Elder; Planetarium of Wan Shi
Tong) — `AltEvent`'s shape over two keyword actions rather than over two
`GameEvent` rows.

### Recorded, so it is not re-discovered

- **The written-out look-and-partition frame was NEVER a gap.** It is built, and
  Impulse, Anticipate and their neighbours have been benched on it for many
  chapters. Do not re-derive it as the family's missing half.
- **FATESEAL IS DECLINED, not deferred.** [CR#701.29] is a single paragraph with
  no zero clause, no trigger clause and no neighbour, where its two siblings
  have four sub-rules each; the corpus matches — 2 lines, ZERO triggers, ZERO
  replacements, ZERO reads. It is the family's opponent-directed cell and it
  waits for machinery keyed on its name, which is the catalog's admission test.
  Not work.

## Consumption boundary

`idris/src/Experimental.idr` (`Search`, `ZoneExpr`, `Move`, `Does`, the placement
riders, `Happened`, `AltEvent`, the manner adverbial and the participle read),
`idris/src/Experimental/Words.idr` (`VerbName`, `verbAgentive`, `TagBody`, the
keyword-action names), `idris/src/Experimental/Events.idr` (the event-name side
of the coordinated read), `idris/src/Experimental/Macros.idr` (the placement
spellings, `ScryB`, `SurveilB`), the pin modules
`idris/src/Experimental/Proofs*.idr`, evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- One clause names the three zones; the possessor-anchored form ("target
  opponent's …", "its controller's …") is written once and shared by both
  families.
- Witnesses benched from both name families, not only from the larger one.
- The shape question is decided on the shared-event evidence and recorded, not
  left implicit: one `TagBody` over `Move`, or a subject slot on `Move`.
- The imperative and the agentive spell one event, not two.
- "On their choice of" is a separable slot; the bare disjunction (Write into
  Being) writes without it.
- The two-destination cell (2 lines) is NOT folded into the position
  disjunction.
- Witnesses benched from both the chooser spelling and the offset spelling.
- Each peek residue is taken or declined with its count on the record; the 2/440
  and 0/219 split for the scrying subject is what any subject slot argues
  against.
- Fateseal is not minted.
- The look-and-partition frame is untouched and its benched cards stay benched.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed (2026-08-21)

Counts below are distinct oracle lines from `mtg-rules corpus --match`.

### The three-zone search

`SearchScope` (Experimental.idr), one sum argument replacing `Search`'s
`ZoneExpr`: `OneZone` (carrying the old `SearchableZone`/`WholeZone` gates) and
`GraveyardHandLibraryOf (whose : Noun bs Player)` under `Possessor`. `searchZone`
returns `Nothing` for the sweep — it fixes no one zone for what it finds —
and `searchDelta` puts the possessor's binding in `effIntro`/`preIntro`/`annIntro`,
which is what lets "Then that player shuffles" read it back.

`Headed p` became `SearchDescribed sc p`, which still demands a head on
`OneZone` and WAIVES the requirement on the sweep — permissively: a headed
predicate is still admitted there, and no head is supplied to the semantics.
The waiver is what lets the plural forms' bare "cards with that name" write;
2 of the 33 sweep lines head singular instead ("for a card named The Spear of
Leonidas", Kassandra; Shaun & Rebecca), so the description is not uniformly
plural and the slot is left free rather than fixed. Macro `searchZonesOf`.

The possessor slot takes `SweepPossessor`, which refuses every `PlayerGroup`
where `Possessor` admits `YourOpponents`: neither "each player's" nor "each
opponent's" graveyard, hand, and library is attested.

Witnesses: `memoricideSearch` ("Search target player's …", target-anchored) and
`eradicateSearch` ("Search its controller's …", co-referential possessor) — one
from each name family. Pins: `badSweepAcrossPlayers` and
`badSweepAcrossOpponents`, the two group words.

Family size: 15, not the ticket's 16 — Bloodbond March has no sweep ("each
player returns all cards with the same name as that spell from their graveyard
to the battlefield"), so the same-name family is 4.

### The agentive placement clause

Taken as the `Put` row in `VerbName` with `PutB : TagBody Put (Move …)`, not a
subject slot on `Move`. `verbAgentive Put = True`, so `Composite Put …` is
refused and the imperative keeps the bare `Move`: the two frames spell one
event. `PutB` is gated on `PutAgentiveZone (zoneSort to)`: library, hand,
battlefield and graveyard are attested destinations, exile is 0 lines (the
exile tag owns it) and the stack is never written as one. Without the gate
`Does p Put (Move n exileZ)` would be a second spelling of `Does p Exile …`.

`verbedMarkingOk Put _ = False` — "put this way" is a measured zero; English
marks a placement patient by its destination verb.

Macro `puts`. Witnesses: `deemInferior`, `lostHoursPlacement`,
`aetherGustPlacement`. Pins: `badSubjectlessPut` (`Composite Put …`) and
`badPutIntoExile`.

### The top-or-bottom position disjunction

`LibPlace` (Experimental.idr) replaces `LibraryAt`'s `LibPos`: `OneEnd pos` and
`EitherEnd (chooser : Maybe (Noun bs Player))` under `EventAgent`. The chooser
is a separable slot on the disjunction alone, so the bare coordination writes
without it. The arrangement gate became `PlaceArrangementFits` and the ordinal
gate `PlaceOrdinalFits`; on `EitherEnd` the ordinal rides the top alternative,
which is the offset spelling.

Macros `topOrBottomZ`, `choiceOfTopOrBottom`, `nthFromTopOrBottomZ`. Witnesses:
`aetherGustPlacement` (chooser, agentive), `deemInferior` (offset),
`notForgottenPlacement` (chooser, imperative, "your choice"),
`writeIntoBeingPlacement` (bare). Pin: `badDisjunctionOrdered` — a disjunction
with an order rider.

`EitherEnd Nothing` is bought by a single attested line, Write into Being's
"put the other on the top or bottom of your library", and manifest is not in
the vocabulary — so that witness is synthetic: it stands an exile where the
card manifests, and only the placement half is the card's own words.

The two-destination cell ("onto the battlefield or into your hand", 2 lines) was
not folded in: `LibPlace` is library-internal and cannot name another zone.

### The peek residues

- **Scrying subject — DECLINED.** 2 of 369 scry lines ("Target player scries 3",
  "Target player scries X, then draws a card"); 0 of 150 surveil lines. A slot
  bought for one keyword's two lines does not carry the family.
- **Process read — DECLINED for now.** 3 lines ("where X is the number of cards
  looked at while scrying this way"). The read reaches into the keyword action's
  body, whose partition the row records without spelling.
- **Per-turn state read — DECLINED for now.** 2 lines, and they are not one
  shape: Darkblade Agent's is a condition, the other a relative clause on cards
  in a graveyard.
- **Coordinated event — DECLINED for now.** 2 lines.

The last three all wait on the same unmade decision — whether a keyword-action
NAME is admitted to the event-name taxonomy `Happened`/`AltEvent` are indexed
on. That decision is the catalog's admission test, the same one Fateseal waits
behind; it was out of this round's pinned scope. Fateseal is not minted. The
look-and-partition frame and `ScryB`/`SurveilB` are untouched.

### Stopped

- The 15 three-zone cards still do not write end to end: their search results are
  plural ("all cards" / "any number of cards" → "exile them") and `Search`
  introduces a singular binding. The search quantity is a separate gap; the
  benched witnesses are the search clause and the possessor read-back.
