---
needs: []
---
# Targeting relative clause and the disjunction's arms — becomes-the-target, Leaves un-gated

Split from `workbench-event-zone-and-cast-provenance` on 2026-08-26 (see
`docs/tickets/planned/workbench-event-zone-and-cast-provenance.md` for the
umbrella). Sub-round 3 of 4. **Size: M. Independent.**

## 0. Corrections the parent ticket needed — carried verbatim

The parent ticket's pointers have drifted. An implementer who trusts the
parent's names will chase symbols that no longer exist.

| Ticket says | Tree says |
|---|---|
| "adding one moves ten tables — `sameZone`, `searchableZone`, `putSourceZoneOk`, `putDestZoneOk`, `publicZone`, `exposableZone`, `destTypeOk`, `visibilityOk`, `playableFrom`, `zoneFits`" | **`sameZone` and `searchableZone` do not exist.** `visibilityOk` (`Words.idr:1370`) is `ExposeVerb -> VisibleThing -> Bool` and does not mention `Zone` at all. `zoneFits` (`Words.idr:1909`) delegates to `Eq Zone`. The real list is in §1.4 and is **eleven** entries. |
| "`badCastInGraveyard` still stands" / "the pin modules … (`badCastInGraveyard`…)" | **No such pin.** The `CastFrom` refusals are `badCastFromBattlefield` (`ProofsG.idr:126`) and `badCastFromStack` (`ProofsG.idr:134`). |
| "The `eventUse` / `eventSpan` / `ReplUse` triple over 26 `EventName` rows (`Events.idr:107`–`659`)" | **`eventUse`, `eventSpan`, `replUseOk` and `triggerWordOk` have all been retired** — zero hits across `idris/src`. `ReplUse` survives as a bare 2-value type (`Events.idr:256`) with no table over `EventName`. `EventName` is **34 rows**, not 26, and `Events.idr` is 635 lines, not 659. |
| "[the closure tables](../../idris-workbench-closure-tables.md) §2.2" | That document carries `> **Historical snapshot (demoted 2026-08-22).**` on line 3, and its §2.2 describes the retired tables above. **Cite it as history; re-derive the live grid from the source.** |
| "`VerbName` has had `Mill` for many chapters" | The type is `VerbLabel = String` (`Words.idr:764`), an **open** vocabulary backed by the `verbFacts` data list (`Words.idr:788`), whose docstring states outright: "A label needs no row in a total table and no rules entry of its own." `Mill` is `MkVerbFacts "Mill" (Just "milled")`. This is load-bearing for sub-round 1's pin, not this one. |
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
catch-alls — but see the `negatable (CastBy _)` re-measure trigger, which
this sub-round in particular must honor (§4).

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

- *"The TARGETING relative clause — 19 lines"*:
  > "No targeting predicate exists AT ALL — nothing in `Predicate` describes an
  > object by what it targets — so this is a vocabulary row and not a
  > composition. Its complement is an ordinary noun; the relation is what is
  > missing."
- Routed item (2026-08-26, from `workbench-payment-events-and-replacement-disjunction`),
  its arms half:
  > "waits only on its ARMS' vocabulary, which this ticket owns: a 'becomes the
  > target of' event row and a `Leaves` not gated to the battlefield (Giggling
  > Skitterspike, Trouble in Pairs, Syr Konrad, Repeated Reverberation,
  > Illusionary Mask — 5 lines, 3 seats). Mint the arms here, then the slot per
  > the recorded shape."

**Why together.** The targeting *description* and the becomes-the-target
*event* are the two voices of one relation — the same pairing the grammar has
already made twice, at `BlockerOf`/`BlockedBy` and at
`CouldBlock`/`CouldBeBlockedBy`. Deciding the relation once and spelling it at
both seats is the whole economy of this sub-round. Splitting them would have
two rounds independently invent "what targets what."

## 3. Corpus, re-measured — the routed item drastically understates this

- **"becomes the target of" as a trigger header: 98 supported lines.** Not one
  arm of one disjunction. Sample shapes, all from local data:
  - "When enchanted creature becomes the target of a spell or ability, destroy
    that creature. It can't be regenerated."
  - "Whenever a creature an opponent controls becomes the target of a spell or
    ability you control, put a bounty counter on that creature."
  - "Whenever a creature you control becomes the target of a backup ability,
    copy that ability."
  - **Fblthp, the Lost**: "When Fblthp becomes the target of a spell, shuffle
    Fblthp into its owner's library."
- **`Leaves` off the battlefield: 34 supported header lines.** Overwhelmingly
  the graveyard: "Whenever one or more cards leave your graveyard, …" (many),
  "Whenever a creature card leaves your graveyard, amass Goblins 1",
  "Whenever a creature card leaves an opponent's graveyard, …",
  "Whenever one or more artifact and/or creature cards leave your graveyard,
  you gain 1 life" (Dredger's Insight). Against **251** total `leaves`
  headers, so the un-gating is a 34-line family, not a single Syr Konrad arm.
- **A whole-card witness for the becomes-target row exists independent of the
  disjunction**: Fblthp's second ability quoted above is a complete, simple
  ability. The routed item's framing ("gated on its arms' vocabulary") made
  the arms look like they had no carrier of their own. They do. The round is
  "two attested vocabulary families that happen to also unblock a slot," not
  "unblock a slot."

## 4. Settled rulings that bind it

From `docs/tickets/done/workbench-payment-events-and-replacement-disjunction.md`
("Verdict 2"), and now written into the source docstrings at
`Triggers.idr:164–182` (the `GameEvent` head) and `Triggers.idr:624–627`
(`AltEvent`) — **do not re-derive, do not re-open**:

- The n-ary event disjunction is a **SEAT slot**, never a `GameEvent` row.
  `eventName` is total, so a row would leave the classifier naming one of n
  events. That objection is decisive against the row shape and does *not*
  reach a slot.
- The only name-keyed use on any prospective seat is
  `Interceptable ev = So (interceptOk (eventName ev))`, and it **distributes**
  — every arm interceptable. `Delayed` and the header consult `eventName` not
  at all.
- The body reads **whole-agreement discourse** (`sameBindings`) **folded over
  a list**, not applied to a pair. `headerCtx` (`Triggers.idr:637`) is the
  binary version to generalise; partial agreement names nothing determinate.
- **Widening the binary `AltEvent` to three is explicitly NOT the move.** The
  slot is an arm *list*, gated arm by arm, and the same slot is what `Delayed`
  and `Intercepts` lack entirely.
- Arity is settled at three-plus from three seats: header (Giggling
  Skitterspike, Trouble in Pairs, Syr Konrad), delayed (Repeated
  Reverberation), replacement `would` (Illusionary Mask).

Also bears on this round: `negatable (CastBy _)` is False, recorded as a
re-measure trigger, not an item — see the umbrella ticket. This sub-round
mints a `Predicate` row and so must **not** touch `negatable`'s `CastBy`
cell; if it happens to mint the agentless passive ("target spell you control
that wasn't cast"), the cell is re-measured rather than assumed.

## 5. Pins — SETTLED (ratified 2026-08-26, pinned — do not reopen)

1. **One relation in two voices, or two independent rows?** The event
   (`BecomesTarget`, subject = the targeted thing, complement = the
   targeter) and the description ("spells … that target this creature",
   subject = the targeter, complement = the targeted). `BlockerOf`/
   `BlockedBy` are two `Predicate` rows sharing one gate; `CouldBlock`/
   `CouldBeBlockedBy` likewise. But here one voice is an `EventName`+
   `GameEvent` and the other is a `Predicate` — a genuinely new
   configuration. Open question was whether the `EventName` also gets
   `lookbackSubjectOk`/`lookbackComplementOk` cells (i.e. whether "creature
   that became the target of a spell this turn" is in scope) or whether the
   retrospective side is refused at its count.

   **SETTLED:** both voices land. The retrospective lookback cells are **in
   scope**, measured live (not assumed).

2. **What sort is the targeter complement?** The corpus writes "a spell", "an
   ability", "a spell or ability", "a spell or ability you control", "a spell
   or ability an opponent controls", "a backup ability". `Ability` is already
   a `Kind` (`lookbackComplementOk AbilityActivation Player Ability = True`),
   so "a spell or ability" is a joined-kind noun `Object \/ Ability` — open
   question was whether `kindOfW`/`Joined` reaches it, and whether the
   joined-complement refusal recorded on `lookbackComplementOk` (sub-round
   2's routed item) is live here.

   **SETTLED:** the targeter complement is the **joined kind `Object \/
   Ability`**, reached via the existing join machinery. Measure the
   joined-complement cell **live**, per this report's own numbers — do not
   assume the refusal carries over unmeasured.

3. **How is `Leaves` un-gated?** Three shapes: drop the `ZoneFits` gate and
   let `nounZone` carry the zone (Syr Konrad's "leaves your graveyard" is
   then a zone-scoped noun); add a `from : Maybe (EventSource bs)` slot
   mirroring `PutInto`; or mint a second row. Note `eventAfter (Leaves n) =
   moveIntro Nothing n Nothing` — the destination is already unknown, so the
   row is *already* agnostic about where the object went; only the source is
   pinned.

   **SETTLED:** un-gate via a **`from : Maybe (EventSource bs)` slot
   mirroring `PutInto`**.

4. **Does the disjunction slot land in this sub-round or the next?** The
   routed item says "Mint the arms here, then the slot per the recorded
   shape." The slot is a real build — `AltEvent` binary → arm list,
   `headerCtx` fold, plus new slots on `Delayed` and `Intercepts` which today
   have none.

   **SETTLED: ARMS ONLY.** The n-ary seat slot stays ledgered with its
   recorded shape (§4 above); do **not** build the `AltEvent` list widening,
   the `headerCtx` fold generalisation, or the `Delayed`/`Intercepts` slots
   in this sub-round.

## 6. Files

`Experimental/Events.idr` (the `BecomesTarget` name and its seven tables),
`Experimental/Triggers.idr` (the `GameEvent` row and its four tables;
`Leaves`'s `from` slot — **not** `AltEvent`/`headerCtx`/`Delayed`/
`Intercepts`, per pin 4), `Experimental/Phrase.idr` (the targeting
`Predicate` row and its seven tables), `Experimental/Cards.idr`,
`Experimental/ProofsG.idr`.

## 7. Sequencing

All four sub-rounds of this split are semantically independent — none needs
another's output, and each has its own whole-card witness. But three of them
(1, 2, 3) add constructors to `EventName` and clauses to the same seven
tables in `Events.idr`, plus rows in the same four tables in `Triggers.idr`;
running two in parallel workspaces guarantees a textual conflict at every
table. Recommended: **serialise 1 → 2 → 3** (all S/S/M, same table block);
sub-round 4 may run **in parallel** with any of them (its zone tables live
mostly in `Words.idr` and its reader payload in `Phrase.idr`, colliding with
the others in only two functions). **If only one sub-round can be run:
sub-round 1** — its pin was architectural and the discard family is 96
lines, the largest single unblock in the bundle.

Standard constraints apply.
