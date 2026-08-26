---
needs: []
---
# Read events by verb and zone, and finish the cast relation's provenance

One unit over the zone-change event, the event-history lookback and the cast
relation. These belong together because they share the zone catalog (the command
zone is asked for by both the put-into headers and the cast-history lookback),
share the coordination gap at zone phrases, and share the cast relation itself —
`CastBy` seeds the stack for the history read, `CastFrom` is the provenance
qualifier both frames hang off, and the ordinal binder blocks the keyword grants
that the trigger side keeps routing here.

## The mill event, and the verbed event generally — 4 + 2 lines

The put-into-a-zone event landed and this did **not** fall out of it, which is
the point. [CR#701.17a] makes milling "puts that many cards from the top of
their library into their graveyard", so the transition IS `PutInto`'s with a
library source and a graveyard destination — and the corpus still spells it with
the VERB.

Four supported lines write the header ("Whenever one or more nonland cards are
milled": Mirelurk Queen, Screeching Scorchbeast, The Wise Mothman; Saruman
writes the reflexive "when one or more cards are milled this way"), and two
write the replacement ("If an opponent would mill one or more cards, they mill
twice that many instead", Bruvac, The Water Crystal).

`VerbName` has had `Mill` for many chapters and `verbedMarkingOk Mill` is open,
so what is missing is the **EVENT reading of a verb** — an event named by the
keyword action rather than by the zone change it entails. That is core's
`EventFilter::Act { verb, … }` and has no row here. Screeching Scorchbeast is
the witness and also wants the counted group's size as an amount ("that many"),
so it stays two gaps until that lands.

## The command zone — 2 headers, and the history lookback's 21

`Zone` has five of [CR#400.1]'s zones and core's `Command` row "is still not
ported" (the catalog's own comment). Two put-into headers name it: "Whenever
your commander is put into the command zone from anywhere" (Myth Unbound) and
"Whenever a creature you control dies or is put into the command zone"
(Reyhan) — and **neither can be refused by `putDestZoneOk`, because there is no
value to refuse.** Commander is already treated as card-held and zone-surviving,
so the zone and the designation are one round's work.

The same row is what 21 of the cast-history lookback lines want (below): `data
Zone = Battlefield | Graveyard | Exile | Hand | Library | Stack` has no seventh,
and adding one moves ten tables — `sameZone`, `searchableZone`,
`putSourceZoneOk`, `putDestZoneOk`, `publicZone`, `exposableZone`, `destTypeOk`,
`visibilityOk`, `playableFrom`, `zoneFits` — whose cells are unmeasured.

## The coordinated source and destination — 5 lines

Authority: [The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
— the coordinated source and destination are ordinary coordination at the zone
sort, not a marked union row.

The coordinated EVENT landed and this half did not, because it is a disjunction
of ZONE PHRASES rather than of clauses: "from your hand or library" (Desert
Warfare), "from graveyards and/or the battlefield" (Ketramose), "from your
library and/or your graveyard" (Laelia), and on the destination side Kaya,
Spirits' Justice's "one or more creatures you control and/or creature cards in
your graveyard".

The `Or` predicate's shape may already reach it — a `ZoneExpr` disjunction is a
smaller thing than an event one — but **nothing has measured which of `Or`'s
four gates a zone phrase would have to answer**. Five lines in all, so it is a
row rather than a round on its own, which is why it rides here.

## What the history read and the cast-zone provenance already landed

The event-history read's third slot landed as a defaulted `EventComplement` on all
three readers, gated by `lookbackComplementOk` (event x subject sort x complement
sort) and `bareLookbackOk` (whether the slot may stand empty — two measured False
cells, both pinned); `Predicate.CastFrom (z : ZoneExpr)` landed beside it, gated
by `playableFrom` (SHARED with the play permission, every cell re-measured, none
moved) and `WholeZone`, seeding no zone so `badCastInGraveyard` still stands, with
the negation costing nothing (ordinary `Not`, two attested spellings). What is
left is one region of reads with named blockers.

### Sizing that must not be re-lost

- The cast history read alone is 141 lines, of which only TWO write the bare
  form; the complement was the construction, not its residue.
- The by-source damage family is 48. The block relation's by-complement is 1 line
  (Joven's Ferrets) — the other eight are present-tense `BlockerOf`/`BlockedBy`
  descriptions, a different row, or the coordination below.
- Three `lookbackSubjectOk` cells reopened with that landing: `CombatDamage
  Object`, `TokenCreation Player`, `BlockedDeclaration Object`.
- The cast-provenance family is 185 over FIVE frames — trigger header 80,
  self-condition 37, history lookback 32, description 29, mana-spend purpose 7 —
  beside 148 permission lines that are a different axis.

## The dealer-subject read of damage IN GENERAL — 17 lines

"Target creature that dealt damage to you this turn" (Reciprocate, Retaliate,
Spear of Heliod, Giltspire Avenger, Brine Hag, Giant Albatross, Aegar, Hawkeye,
Wolverine, Dunerider Outlaw, Whirling Dervish…). It has NO `EventName`:
`DamageTaken` is the victim's side of the happening and `CombatDamage` is the
combat dealer's; the general dealer's side is neither. These lines say "damage",
not "combat damage" — an earlier comment counted two of them as combat damage and
that correction is what exposed the gap.

## The SECOND complement sort — 19 lines and 3

The placement's ZONE (19 lines, "if a creature card was put into your graveyard
from anywhere this turn", most naming a source zone as well as a destination) and
the counter pair's KIND (3 lines). This slot's axis is `Kind` and neither is a
noun of any sort, so `Placement`, `CounterPlacement` and `CounterRemoval` keep
their False cells and would want a complement over `ZoneExpr` and one over
`CounterKind`. THE DESIGN QUESTION: one mechanism with a sorted payload, or three
tables.

## The history LOOKBACK frame of a cast's origin — 32 lines, split 21/11

- 21 are the commander family and want a COMMAND ZONE row (the ten-table cost is
  recorded above).
- The other 11 want the origin as a RIDER on `Happened`/`EventCount` and nothing
  else — the cheaper half, landable without the zone row: the six "if you haven't
  cast a spell from your hand this turn" cards, Laboratory Drudge, Impending
  Flux, Surge of Brilliance, Spider-Man 2099, Approach.

## The zone DISJUNCTION on a cast's origin

"From your graveyard or from exile" (Doc Aurlock; Aven Interrupter; Soulless
Jailer's "from graveyards or exile") — two origin clauses coordinated, the
standing coordination gap at a new site.

## The LIBRARY cell of `CastFrom` — open, probed, no cheap carrier

The whole family writes it twice: Melek, Izzet Paragon's trigger header and
Fblthp, the Lost's self-condition. Both carry a second unbuilt thing besides
(Melek a play permission over the top of a library, Fblthp a coordinated
entered-or-was-cast condition). The witness is owed.

## The four delayed end-of-combat forms

Unchanged and confirmed: their independent second blocker is the "At this turn's
next end of combat" delayed shell, which no current machinery reaches.

## Not this round's, recorded so they are not folded in

- The MAGNITUDE read (`EventSum`) — "if you gained 3 or more life this turn" (15
  lines) and Gollum, Obsessed Stalker's "life equal to the amount of life you
  gained this turn". A complement is a participant and this is a quantity; it
  stays ledgered on `EventCount`'s own row.
- The coordinated history read, 4 lines — "blocked or was blocked by a Zombie
  this turn" (Time to Reflect, You Cannot Pass!, Venomous Breath, Sea Troll): ONE
  complement shared across two events, which is the coordinated EVENT and not
  this slot.
- The PROHIBITION family, 8 lines — "Players can't cast spells from graveyards or
  libraries" (Grafdigger's Cage, Weathered Runestone, Kunoros, Soulless Jailer,
  Ashes of the Abhorrent, Drannith Magistrate, Avatar's Wrath, Experimental
  Frenzy). This is the PERMISSION's negation and belongs to `MayPlay`/`PlaySource`.
- The mana-SPEND purpose, 7 lines — a zone-qualified `SpendPurpose`, which
  belongs with the spend restrictions that name a cost.
- Continue?, whose zone-change surface is a different spelling from the past verb
  these rows write; and Bill Ferny, still blocked on the predefined-token catalog
  ([CR#111.10]) alone.

## What chapters sixty-four and sixty-five landed on the cast relation

Chapter sixty-four landed the cast relation (`CastBy`, a predicate beside
`ControlledBy` rather than a verb on it, seeding the stack) and the controller
relation's two-zone domain (`zoneAdmit`), and chapter sixty-five removed the
union head — never a construction, only `Or`'s mis-documented coordinator, so
Goblin Electromancer, Arcane Melee, Mana Matrix and Aura of Silence landed there.
97 of the 99 plain cost-modification lines are structurally writable now (the 2
face-down heads are the remainder). Two pieces stayed out.

### The TARGETING relative clause — 19 lines

"Spells your opponents cast that target this creature cost {2} more to cast"
(Icefall Regent and 8 siblings; 4 more write "spells you cast that target …").
No targeting predicate exists AT ALL — nothing in `Predicate` describes an object
by what it targets — so this is a vocabulary row and not a composition. Its
complement is an ordinary noun; the relation is what is missing.

### The ORDINAL cast — 5 lines, plus 20 in trigger headers

"the first spell you cast each turn" (Maelstrom Nexus, Rain of Riches,
Wild-Magic Sorcerer, The Twelfth Doctor, Zimone), plus 20 more writing "your
first spell" in a trigger header. An ELEMENT BINDER over a turn's casts — the
same shape
[workbench-amount-comparison-and-quantity](workbench-amount-comparison-and-quantity.md)'s
relativized per-element count wants — and the per-turn reset is a second unbuilt
thing beside it.

The next-spell and first-spell keyword GRANTS ("the next spell you cast this turn
has cascade") wait on this binder. They are [CR#611.2f]'s regime, a continuous
effect that begins to apply when the player next puts an appropriate spell on the
stack; they are not the keyword row's own gap.

### Recorded measurement, not a gap

The AGENTIVE negated cast ("a spell you didn't cast") is ZERO lines, so
`negatable (CastBy _)` is False — but what the phrase would mean is attested one
card over through an agentless passive ("target spell you control that wasn't
cast", Errant, Street Artist), which is why the cell is REFUSED and not pinned. A
round that mints the passive must re-measure the cell rather than assume it.

### Closed, do not redo

The cast-from-zone origin qualifier landed as `Predicate.CastFrom` (chapter one
hundred thirty-three), hung exactly where this work said to. Its count scoped one
frame — 24 for the cost-modification and keyword-grant statics, 29 today with
recent prints (Aven Interrupter, Emet-Selch, Zhulodok, Quandrix) — and never the
family, which is 185 lines over five frames; a recon reading the 24 as a
family-wide undercount had it backwards. DOC AURLOCK still does not land, on the
zone DISJUNCTION ("from your graveyard or from exile"), which belongs to
[workbench-conditional-and-coordination](workbench-conditional-and-coordination.md);
PATRICIAN GEIST is benched as this frame's single-zone witness instead.

## The closure grid this family owns

Ranked below the workbench's fifteen top flip risks — the grid's many zeros are
independent measurements, so a printing moves one cell and the closure survives —
but every cell this family moves is one of them. A widening names the cell it
moved and re-reads the grid rather than defaulting it, and a cell left closed
says whether a rule or a count is closing it. Rows, evidence and widening costs:
[the closure tables](../../idris-workbench-closure-tables.md) §2.2.

The `eventUse` / `eventSpan` / `ReplUse` triple over 26 `EventName` rows
(`Events.idr:107`–`659`).

## Routed ledger items

Items from closed round tickets that this ticket owns. One line each, citing
the done ticket that recorded them.

- **`lookbackComplementOk` refuses every joined complement, on a corpus zero.**
  Under `kindOfW JoinW = Object \/ Player` a joined complement is now writable,
  so the refusal is live rather than unreachable; 34 distinct "dealt damage by"
  lines, five carrying "or", none writing a cross-kind union complement, so the
  clause was not widened and the fact was recorded rather than gated —
  `docs/tickets/done/workbench-union-family-macros.md`.
- **Once Upon a Time's history identity read** ("if this spell is the first
  spell you've cast this game") — neither built nor pinned; no rule makes it
  meaningless and no buildable card pays it —
  `docs/tickets/done/workbench-amount-comparison-and-quantity.md`.
- **The per-member event count inside `AggregateOver`'s binder body** — Thought
  Sponge's "the greatest number of cards an opponent has drawn this turn" and
  the Windfall / Jace's Archivist cycle's "cards a player discarded this way";
  the body admits an `Amount` but no per-member event subject exists —
  `docs/tickets/done/workbench-amount-comparison-and-quantity.md`.

## Consumption boundary

`idris/src/Experimental.idr` (`PutInto`, `putDestZoneOk`, `Zone`, `ZoneExpr`,
the `Or` predicate, `Predicate.CastFrom`, `Happened`, `EventCount`, `EventSum`,
`EventComplement`, `Predicate`, `CastBy`, `negatable`, `zoneAdmit`),
`idris/src/Experimental/Events.idr` (`EventName` and the event table's verb
cells, `lookbackComplementOk`, `bareLookbackOk`, `lookbackSubjectOk`,
`playableFrom`, `WholeZone`), `idris/src/Experimental/Words.idr` (`VerbName`,
`verbedMarkingOk`, the zone words, origin and participle spelling), the pin
modules `idris/src/Experimental/Proofs*.idr` (`badCastInGraveyard`, and pins in
`ProofsD.idr`, `ProofsE.idr` and `ProofsG.idr`), evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate. Core's `EventFilter::Act {
verb, … }` is named as the shape precedent only.

## Acceptance

- The verbed event reads an event by its keyword action without duplicating the
  put-into transition; the four headers and Bruvac's replacement write, and
  Screeching Scorchbeast's second gap (the counted group as an amount) is
  recorded rather than papered over.
- `Command` is a `Zone` row, `putDestZoneOk` can refuse it, and the commander
  designation is settled in the same round; Myth Unbound and Reyhan bench.
- The zone-phrase disjunction either answers `Or`'s four gates with the answers
  measured, or is declined with the five lines recorded.
- The general dealer's side of damage gets its own `EventName` or a stated
  reason it does not; the victim and combat-dealer rows are unchanged.
- The second complement sort is decided as one sorted payload or as separate
  tables, with the 19 and the 3 both accounted for.
- If the command-zone row is taken, all ten named tables are measured in the same
  round; if it is not, the cheaper 11 land as a rider without it.
- `playableFrom`'s shared cells stay shared, and `badCastInGraveyard` still
  stands.
- The targeting predicate is a vocabulary row taking an ordinary noun complement,
  not a composition over existing predicates.
- The ordinal binder and the per-turn reset are answered together, or the reset
  is deferred with its own statement of what remains.
- `negatable (CastBy _)` stays a refusal and stays unpinned unless the passive is
  minted in the same round, in which case the cell is re-measured.
- `Predicate.CastFrom` and Patrician Geist's benched line are untouched.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-combat-assignment-and-forced-attack (close, 2026-08-26):** no attack-declaration event with a PLAYER subject — `Attacks`'s subject is `Noun bs Object`, so "Whenever an opponent attacks with one or more creatures" (Tahngarth, First Mate) never binds the opponent; [CR#508.3d] is the rule. An event-subject row, so it lands here.
