---
needs: []
---
# The verbed event — mill, discard, and the EVENT reading of a verb

Split from `workbench-event-zone-and-cast-provenance` on 2026-08-26 (see
`docs/tickets/planned/workbench-event-zone-and-cast-provenance.md` for the
umbrella). Sub-round 1 of 4. **Size: S. Independent.**

## 0. Corrections the parent ticket needed — carried verbatim

The parent ticket's pointers have drifted. An implementer who trusts the
parent's names will chase symbols that no longer exist.

| Ticket says | Tree says |
|---|---|
| "adding one moves ten tables — `sameZone`, `searchableZone`, `putSourceZoneOk`, `putDestZoneOk`, `publicZone`, `exposableZone`, `destTypeOk`, `visibilityOk`, `playableFrom`, `zoneFits`" | **`sameZone` and `searchableZone` do not exist.** `visibilityOk` (`Words.idr:1370`) is `ExposeVerb -> VisibleThing -> Bool` and does not mention `Zone` at all. `zoneFits` (`Words.idr:1909`) delegates to `Eq Zone`. The real list is in §1.4 and is **eleven** entries. |
| "`badCastInGraveyard` still stands" / "the pin modules … (`badCastInGraveyard`…)" | **No such pin.** The `CastFrom` refusals are `badCastFromBattlefield` (`ProofsG.idr:126`) and `badCastFromStack` (`ProofsG.idr:134`). |
| "The `eventUse` / `eventSpan` / `ReplUse` triple over 26 `EventName` rows (`Events.idr:107`–`659`)" | **`eventUse`, `eventSpan`, `replUseOk` and `triggerWordOk` have all been retired** — zero hits across `idris/src`. `ReplUse` survives as a bare 2-value type (`Events.idr:256`) with no table over `EventName`. `EventName` is **34 rows**, not 26, and `Events.idr` is 635 lines, not 659. |
| "[the closure tables](../../idris-workbench-closure-tables.md) §2.2" | That document carries `> **Historical snapshot (demoted 2026-08-22).**` on line 3, and its §2.2 describes the retired tables above. **Cite it as history; re-derive the live grid from the source.** |
| "`VerbName` has had `Mill` for many chapters" | The type is `VerbLabel = String` (`Words.idr:764`), an **open** vocabulary backed by the `verbFacts` data list (`Words.idr:788`), whose docstring states outright: "A label needs no row in a total table and no rules entry of its own." `Mill` is `MkVerbFacts "Mill" (Just "milled")`. This is load-bearing for this sub-round's pin. |
| "The `Or` predicate's shape may already reach it — a `ZoneExpr` disjunction is a smaller thing than an event one" | **False on the type.** `Or : (ps : List (Predicate bs k)) -> …` (`Phrase.idr:272`). `ZoneExpr` is not a `Predicate`, and the put-into event's source slot is `Maybe (EventSource bs)`, not a predicate either. There is no "which of `Or`'s four gates would a zone phrase answer" question to measure; the question is whether to mint a coordination **at the `ZoneExpr`/`EventSource` sort**. |
| "the commander designation is settled in the same round" | `CommanderD` **already exists** (`Words.idr:2466`) with rows in `designationScope`, `Eq`, `designationChecked`, `designationGiven`, `designationSeedZone` (`Nothing`), `designationSeedType`. The designation is not the gap; the `Zone` row is. |
| "core's `Command` row 'is still not ported' (the catalog's own comment)" | No such comment survives on `data Zone` (`Words.idr:701`). The *core* type is a **different** `Zone`, in `Semantics.idr`, and it already has both `Command` **and** `Sideboard` — `EmitTables.idr:50 zoneName` covers eight rows. **Nothing in `EmitTables.idr` or `Semantics.idr` is in scope for any sub-round of this split.** |
| "The ORDINAL cast — 5 lines, plus 20 in trigger headers" | The 20 headers are **delivered**. `GameEvent.NthOccurrence` (`Triggers.idr:348`) landed in the event-algebra round, and its own docstring gives the spelling: "at a cast event, 'Whenever [who] cast(s) [whose] [ord] spell [window]'". |

## 1. Constraints binding every sub-round of this split — carried verbatim

### 1.1 A new `EventName` row costs seven tables

`EventName` (`Events.idr:21`) is consumed by exactly seven total functions.
Three have **no catch-all** and will fail totality until a clause is added:

| Table | Line | Catch-all? |
|---|---|---|
| `sameEventName` | `Events.idr:97` | **no — must add** |
| `eventHasMagnitude` | `Events.idr:207` | **no — must add** |
| `lookbackSubjectOk` | `Events.idr:262` | **no for `Object` and `Player` — must add both**; `Quality`/`Outcome`/`Gap`/`TurnRef`/`Ability`/`LetterK` and the `\/` join are already covered |
| `interceptOk` | `Events.idr:186` | yes (`_ = True`) |
| `spanEventOk` | `Events.idr:193` | yes (`_ = True`) |
| `lookbackComplementOk` | `Events.idr:362` | yes (`_ _ _ = False`) |
| `bareLookbackOk` | `Events.idr:407` | yes (`_ _ = True`) |

House style states the catch-all-covered cells explicitly anyway, with the
rule that decides them (see the `FlipWin`/`DiceRoll`/`LifePayment` blocks).
Follow it.

### 1.2 A new `GameEvent` row costs four tables

All four in the `Triggers.idr` mutual block, none with a catch-all:
`eventName` (`:351`), `eventIntro` (`:385`), `eventAfter` (`:425`),
`eventSubjectPlur` (`:463`). `delayedCtx` and `Interceptable` derive.

### 1.3 A new `Predicate` row costs seven tables

`seedZone` (`Phrase.idr:~398`), `seedType` (`:~473`), `hasHead` (`:~542`),
`predEq` (`:~702`), `predSays` (`:~1166`), `predNegFree` (`:~1227`),
`predDelta` (`:~1527`). `zoneAdmit` (`:463`) and `negatable` (`:1140`) have
catch-alls — but see the `negatable (CastBy _)` re-measure trigger on the
umbrella ticket.

### 1.4 A new `Zone` row costs eleven tables — the real list

`data Zone` is `Words.idr:701`, six rows. Adding a seventh forces:

| Table | Site |
|---|---|
| `Eq Zone` | `Words.idr:704` (this is what the parent ticket calls `sameZone`; `zoneFits` at `:1909` rides it) |
| `publicZone` | `Words.idr:1340` |
| `exposableZone` | `Words.idr:1354` |
| `isCardZone` | `Words.idr:1506` |
| `onFieldZone` | `Words.idr:1516` |
| `onStackZone` | `Words.idr:1526` |
| `destTypeOk` | `Words.idr:3024` |
| `castComplementOk` | `Events.idr:559` |
| `playableFrom` | `Events.idr:595` (`complementLocates` at `:613` rides it — **shared**, per parent acceptance) |
| `putDestZoneOk` | `Triggers.idr:101` |
| `putSourceZoneOk` | `Triggers.idr:118` |

`data Possessable` (`Events.idr:631`) is a three-constructor type, not a total
table: a new zone simply has no constructor unless it is a per-player zone.

### 1.5 Fences that apply everywhere

- **Vocabulary-only fence**, wherever the parent draws one: a sub-round mints
  the row and its tables and benches its witness. It does not go build the
  second unbuilt thing a carrier happens to need (the parent records those:
  Screeching Scorchbeast's counted-group amount, Melek's top-of-library play
  permission, Fblthp's coordinated entered-or-was-cast condition, the delayed
  end-of-combat shell).
- **Pins refuse rules-impossibility only** (`docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md`).
  A corpus count is never a refusal. A measured zero is recorded on the cell
  with the count, not pinned.
- **Oracle text and rule numbers come from local data**, never memory:
  `data/derived/cards.jsonl` filtered `jq 'select(.supported)'` first
  (`docs/memory/operations/english-corpus-supported-filter.md`), and the CR via
  the `mtg-rules` skill's scripts. Every number below was taken that way and is
  marked where it disagrees with the parent.
- **Closure grid**: every cell this family moves is one of the graded ones. A
  widening names the cell it moved and re-reads the grid; a cell left closed
  says whether a rule or a count closes it. Because the §2.2 tables are
  retired (§0), re-derive the live grid from `Events.idr` rather than from the
  doc.
- Standard constraints apply (build, fmt, clippy, CR citations, wizards
  regen).

## 2. Items this sub-round owns

- *"The mill event, and the verbed event generally — 4 + 2 lines"*:
  > "what is missing is the **EVENT reading of a verb** — an event named by the
  > keyword action rather than by the zone change it entails. That is core's
  > `EventFilter::Act { verb, … }` and has no row here."
- Routed item (2026-08-26, from `workbench-distinct-kind-count`):
  > "no card-discard `EventName` — 'Whenever you discard a card' (All-Seeing
  > Arbiter) cannot be written. An event row, so it lands here."

**Why these two and nothing else.** Discard and mill are the *same*
construction. `Discard` and `Mill` are both `verbFacts` rows with participles
(`Words.idr:793–794`), both expand to a `PutInto` transition, and both are
written by the corpus with the verb rather than the transition. Minting a
mill event and *then* discovering discard needs the same mechanism would be
the round done twice.

## 3. Corpus, re-measured (supported faces only)

- `Whenever/When … is/are milled` header: **4 cards** — Mirelurk Queen,
  Saruman of Many Colors, Screeching Scorchbeast, The Wise Mothman. Matches
  the parent.
- `would mill` replacement: **2 cards** — Bruvac the Grandiloquent, The Water
  Crystal. Matches the parent.
- `When/Whenever … discard(s)` header: **96 supported lines**, of which nine
  are the "a spell or ability an opponent controls causes you to discard this
  card" madness-adjacent frame (a *causer*, a different shape) and the rest
  are the plain event ("Whenever an opponent discards a card, …" — Tourach;
  "Whenever you discard a card, …" — All-Seeing Arbiter). **The discard event
  is an order of magnitude larger than the mill event.** The brief must say
  this: the round is motivated by discard and witnessed by mill, not the
  reverse.
- All-Seeing Arbiter's exact line, from local data:
  > "Whenever you discard a card, target creature an opponent controls gets
  > -X/-0 until your next turn, where X is the number of different mana values
  > among cards in your graveyard."
  Its **tail is unbuilt** (`ValueAxis ManaValue` distinct-kind count, ledgered
  on `workbench-distinct-kind-count`). Bench the **header alone**, on
  `heartOfBogardanHeader`'s precedent
  (`docs/tickets/done/workbench-payment-events-and-replacement-disjunction.md`,
  "Carrier status"). Tourach ("Whenever an opponent discards a card, put a
  +1/+1 counter on Tourach") is the cheaper whole-card witness — check it
  first.

## 4. Pin — SETTLED (ratified 2026-08-26, pinned — do not reopen)

**Does the verbed event key on the open `VerbLabel` vocabulary, or does each
verb get its own closed `EventName` row?** This was the round's whole
architecture and the parent ticket did not decide it.

`VerbLabel` is deliberately **open**: `VerbLabel = String`, gated only by
`KnownVerb` against the `verbFacts` data list, and its docstring says a new
keyword action is "a new macro plus a new label, never a core enum row plus a
coverage re-decide." `EventName` is deliberately **closed**: 34 constructors,
`eventName` total, and per §1.1 a new row is a totality error on three
tables. A `GameEvent` row `VerbedEvent : (v : VerbLabel) -> … -> GameEvent bs`
needs `eventName (VerbedEvent v _) = ?`, and `?` cannot be a function of an
open `String` into a closed enum without a fallback row.

**SETTLED:** the verbed event is a **label-carrying `EventName` constructor**
(`VerbedAct`-style — the constructor itself carries the `VerbLabel`). The
name-keyed tables that need per-verb distinction (`eventHasMagnitude`,
`lookbackSubjectOk`, `interceptOk`, per §1.1) decide by consulting **new
fields on `VerbFacts`** (the `KeywordFacts` model), not by minting one
`EventName` per verb. Vocabulary stays open — per-verb distinctions are
preserved on `VerbFacts`, not on a closed enum: no partial map
(`actEventName : VerbLabel -> EventName`), no closed sub-catalog gate
(`So (verbIsEventable v)`) over `VerbLabel`.

The three candidate shapes the splitter considered are **rejected**, at their
own recorded costs, in favor of the above:

1. *One `EventName` (e.g. `VerbedAct`) + a `VerbLabel` slot on the row, with
   the name-keyed tables answering only by name.* Cheapest by table count
   (one set of seven cells), but every name-keyed table would then answer
   about *all* verbed acts at once — `eventHasMagnitude`,
   `lookbackSubjectOk`, `interceptOk` could not distinguish mill from
   discard. Bruvac's replacement requires `interceptOk` True and a lookback
   on "milled this turn" vs "discarded this turn" would share one subject
   cell. Rejected as stated; superseded by reading `VerbFacts` fields
   instead of the bare name.
2. *One `EventName` per verb* (`CardMilled`, `CardDiscarded`, …), lifted by a
   partial map `actEventName : VerbLabel -> EventName`. Precedent exists:
   `statusEventName`, `counterEventName`, `flipEventName`, `paymentEventName`
   are all exactly this lift. Cost: the map is total only over a *sub*-
   catalog, so it needs a gate (`So (verbIsEventable v)`) on `VerbLabel`'s
   model — which quietly re-closes part of the open vocabulary. Rejected:
   this is exactly the "core enum row plus a coverage re-decide" the
   `VerbLabel` docstring forbids a label from ever needing.
3. *Decline the general reading; mint `CardDiscarded` alone* and record mill
   at its 4+2. Cheapest to land, but the parent's acceptance explicitly asks
   for the general reading and Bruvac's replacement is the harder half.
   Rejected: does not deliver the acceptance.

## 5. Also to record, not to build

- Screeching Scorchbeast's second gap: the counted group's size as an amount
  ("that many"). Parent acceptance requires this be *recorded rather than
  papered over*.
- Bruvac's replacement needs `interceptOk` True on whatever name lands, and
  `ReplUse`-side machinery is retired (§0) — so the replacement's own gate is
  `Interceptable ev = So (interceptOk (eventName ev))` and nothing else.

## 6. Files

`Experimental/Events.idr` (the `EventName` row and its seven tables, and the
new fields on `VerbFacts`), `Experimental/Triggers.idr` (the `GameEvent` row
and its four tables), `Experimental/Words.idr` (the `VerbFacts` fields),
`Experimental/Cards.idr` (witnesses), `Experimental/ProofsG.idr` (refusals).

## 7. Sequencing

All four sub-rounds of this split are semantically independent — none needs
another's output, and each has its own whole-card witness. But three of them
(1, 2, 3) add constructors to `EventName` and clauses to the same seven
tables in `Events.idr`, plus rows in the same four tables in `Triggers.idr`;
running two in parallel workspaces guarantees a textual conflict at every
table. Recommended: **serialise 1 → 2 → 3** (all S/S/M, same table block);
sub-round 4 may run **in parallel** with any of them (its zone tables live
mostly in `Words.idr` and its reader payload in `Phrase.idr`, colliding with
the others in only two functions). **If only one sub-round can be run: this
one (sub-round 1)** — its pin was architectural and the discard family is 96
lines, the largest single unblock in the bundle.

Standard constraints apply.

## As landed (2026-08-26)

### The mechanism

One `EventName` arm and one `GameEvent` row, per the settled pin.

- `Events.idr`: `EventName` gains `VerbedAct VerbLabel` — the label rides
  the classifier, so no verb gets a row and no lift needs a sub-catalog
  gate.
- `Triggers.idr`: `VerbedEvent : (who : Maybe (Noun bs Player)) ->
  (v : VerbLabel) -> (what : Maybe (Noun (agentIntro who) Object)) ->
  {auto 0 kv : KnownVerb v} -> {auto 0 pt : VerbPatient v what} ->
  {auto 0 vc : VerbedVoice who what} -> GameEvent bs`.
  Two new gate types beside it: `VerbPatient` (patient written exactly
  where the act's rule takes one) and `VerbedVoice` (active names the
  actor, passive the patient, neither-dropped refused).
- `payerIntro` is generalised to `agentIntro` — the same helper now
  serves `PaysCost` and `VerbedEvent`; three sites.

### The new `VerbFacts` fields

`record VerbFacts` grows two rule-bearing fields beside the two spelling
ones, on `KeywordFacts`' model (its docstring is amended to say so):

- `actPatient : Maybe Kind` — the participant the act's own rule performs
  it on. `Just Object` for Destroy, Sacrifice, Exile, Discard, Mill, Tap,
  Put; `Nothing` for Scry, Surveil ([CR#701.22a,701.25a] take a number)
  and Search (what a printed line writes after the verb is a zone, which
  is no `Kind` here).
- `actDest : Maybe Zone` — where that rule leaves the patient.
  `Just Graveyard` for Destroy [CR#701.8a], Sacrifice, Discard
  [CR#701.9a], Mill [CR#701.17a]; `Just Exile` for Exile [CR#701.13a];
  `Nothing` for Tap [CR#701.26a], Put, Scry, Surveil, Search.

Read back by `actPatientOf`, `actNamesPatient`, `actDestOf`.

### The eleven table cells

`EventName`'s seven (§1.1), all stated explicitly including the
catch-all-covered ones:

| table | cell |
|---|---|
| `sameEventName` | label equality |
| `interceptOk` | True — [CR#614.1] over an act that would happen (Bruvac) |
| `spanEventOk` | True |
| `eventHasMagnitude` | False for the whole family: what varies is HOW MANY, and [CR#603.2c] makes a multi-card act one event with that many occurrences, which `EventCount` counts |
| `lookbackSubjectOk` | `Object` = `actPatientOf v == Just Object` ([CR#701.17c] finds "a milled card"); `Player` = True everywhere |
| `lookbackComplementOk` | actor's complement is the act's own patient; nothing from the patient's side |
| `bareLookbackOk` | `Player` = `not (actNamesPatient v)` — bare "discarded" drops the card [CR#701.9a] asks for, bare "scried" drops nothing |

`GameEvent`'s four (§1.2): `eventName` lifts the label; `eventIntro`
announces the patient, or the actor when there is none; `eventAfter`
stamps the patient with the label (so `theVerbed`/`thoseVerbed` read it
back) and moves it to `actDestOf v`, or leaves it where it was;
`eventSubjectPlur` takes the voice's surface subject.

**Mill and discard agree in every cell.** That is the honest reading of
the rules, not a collapse into the rejected shape 1: the tables are
per-act because they read `verbFacts`, and Scry/Surveil/Search already
answer differently there. The architecture is what the pin bought; the
mill/discard difference the pin anticipated does not exist on the rules.

### Corpus, re-measured (supported faces)

- `is/are milled` header: **4** — Mirelurk Queen, Screeching Scorchbeast,
  The Wise Mothman (all "Whenever one or more nonland cards are milled"),
  plus Saruman of Many Colors' "When one or more cards are milled this
  way". Matches §3.
- `would mill` replacement: **2** — Bruvac the Grandiloquent, The Water
  Crystal. Matches §3.
- discard-naming header clauses: **96** distinct card+line pairs. Matches
  §3. The causer frame is **11** lines, not nine: nine of "When a spell or
  ability an opponent controls causes you to discard this card" plus one
  each of "…causes you to discard cards this turn" and "…causes you to
  discard a card". Nine is the count of the single most common form.
- Verb-event headers this row also newly reaches: "Whenever you scry"
  (14), "Whenever you surveil" (6), "Whenever you cycle or discard a
  card" (11, needs the event disjunction).

### Witnesses benched

- `lilianasCaress` — whole card. "Whenever an opponent discards a card,
  that player loses 2 life." The round's whole-card check: header entire,
  and the body reads back the actor the header announced.
- `tourachDiscardTrigger` — Tourach, Dread Cantor's third line, the
  named witness at ability scope.
- `allSeeingArbiterHeader` — header only, on `heartOfBogardanHeader`'s
  precedent; its tail is `workbench-distinct-kind-count`'s.
- `mirelurkQueenTrigger` — the mill event's passive voice, with
  `triggeredOnlyOnce`.

Pins (`ProofsG.idr`): `badScryPatient` (a patient on an act whose rule
takes a number, [CR#701.22a]), `badVoicelessAct` (an act announced of no
one).

### Recorded, not built

- **Screeching Scorchbeast's counted group as an amount.** "Whenever one
  or more nonland cards are milled, you may create THAT MANY 2/2 …": the
  header's counted patient has no amount reader, so the size of the group
  the event announced cannot be named in the tail. The Wise Mothman
  ("where X is the number of nonland cards milled this way") and Bruvac
  ("they mill TWICE THAT MANY cards instead") want the same thing.
  Unbuilt; not papered over.
- **Bruvac the Grandiloquent's replacement.** Its gate is exactly
  `Interceptable ev = So (interceptOk (eventName ev))` and nothing else —
  `interceptOk (VerbedAct _) = True` supplies it. What still blocks the
  card is "twice that many", the counted-group amount above.
- **Tourach, Dread Cantor as a whole card.** Blocked on "target opponent
  discards two cards at random": `Indefinite` carries the `ChoiceMode`
  and is singular, `CountedGroup` carries a `Quantity` and no mode, so
  there is no plural at-random determiner. Not this round's item.
- **"Whenever an opponent searches their library"** (3 lines) stays
  unwritable: the act's printed object is a zone and `Kind` has no zone
  arm. Recorded on `actPatient`'s Search row.

### Ledger — needs routing at integrate

- **The verbed event overlaps three dedicated rows.** `VerbedEvent`
  reaches "Destroy" ([CR#701.8a], already `IsDestroyed`), "Tap"
  ([CR#701.26a], already `StatusEvent` on `TapC`) and "Put" (already
  `PutInto`). Two terms for one event, refused by no rule and left
  standing — named in the row's docstring. A later round should decide
  whether the dedicated rows retire into this one or the preference is
  recorded; the dedicated rows do carry gates this one does not
  (`ZoneFits` on the battlefield, `PutDest`/`PutSource`).
- **The plural at-random determiner** (Tourach's blocker above).
- **The counted-group amount** (Screeching Scorchbeast / Wise Mothman /
  Bruvac) — already ledgered on `workbench-distinct-kind-count`'s
  neighbourhood; confirm it has a live planned ticket.
- **The event disjunction at the header** — "Whenever you cycle or
  discard a card" (11 lines), "Whenever an opponent discards a card or
  mills one or more cards" (1). Sub-round 2/3 territory, not minted here.
