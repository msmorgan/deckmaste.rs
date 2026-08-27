---
needs: []
---
# The zone catalog, the sorted reader payload, and the cast-origin rider

Split from `workbench-event-zone-and-cast-provenance` on 2026-08-26 (see
`docs/tickets/planned/workbench-event-zone-and-cast-provenance.md` for the
umbrella). Sub-round 4 of 4. **Size: M. Independent.** (Its two halves have
an *internal* order: the `Command` row before the 21 commander lookback
lines. Nothing outside depends on it and it depends on nothing.)

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
| "The `Or` predicate's shape may already reach it — a `ZoneExpr` disjunction is a smaller thing than an event one" | **False on the type.** `Or : (ps : List (Predicate bs k)) -> …` (`Phrase.idr:272`). `ZoneExpr` is not a `Predicate`, and the put-into event's source slot is `Maybe (EventSource bs)`, not a predicate either. There is no "which of `Or`'s four gates would a zone phrase answer" question to measure; the question is whether to mint a coordination **at the `ZoneExpr`/`EventSource` sort** — settled below, §5 pin 3. |
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

This sub-round is the one that actually mints the seventh `Zone` row (per
pin 4 below), so these eleven tables are its own direct scope, not
background.

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

- *"The command zone — 2 headers, and the history lookback's 21"*:
  > "`Zone` has five of [CR#400.1]'s zones and core's `Command` row 'is still
  > not ported' … Two put-into headers name it … and **neither can be refused
  > by `putDestZoneOk`, because there is no value to refuse.**"
- *"The coordinated source and destination — 5 lines"*:
  > "the coordinated source and destination are ordinary coordination at the
  > zone sort, not a marked union row."
- *"The SECOND complement sort — 19 lines and 3"*:
  > "THE DESIGN QUESTION: one mechanism with a sorted payload, or three tables."
- *"The history LOOKBACK frame of a cast's origin — 32 lines, split 21/11"*:
  > "21 are the commander family and want a COMMAND ZONE row … The other 11
  > want the origin as a RIDER on `Happened`/`EventCount` and nothing else —
  > the cheaper half, landable without the zone row."
- *"The LIBRARY cell of `CastFrom` — open, probed, no cheap carrier"* —
  **recorded, not built**:
  > "The whole family writes it twice: Melek, Izzet Paragon's trigger header
  > and Fblthp, the Lost's self-condition. Both carry a second unbuilt thing
  > besides … The witness is owed."

**Why together.** The zone-phrase coordination, the placement's zone
complement and the cast-origin rider are **one question asked at three
seats**: *the retrospective readers and the put-into event need a
`ZoneExpr`-sorted payload, possibly coordinated.* One corpus line proves it —
"the number of cards that were put into your graveyard **from your hand or
library** this turn" is simultaneously an `EventCount`, a placement-zone
complement, a *source* zone, and a coordination. Deciding these in separate
rounds means the first one to land pre-empts the others' shape by accident.

## 3. Corpus, re-measured

- **Command zone.** 49 supported lines mention it. The put-into headers are
  exactly the parent's two — Myth Unbound ("Whenever your commander is put
  into the command zone from anywhere, draw a card") and Reyhan ("Whenever a
  creature you control dies **or is put into the command zone**, if it had
  one or more +1/+1 counters on it, …"). **Reyhan's header is an `AltEvent`
  coordination** and its body reads back "it"/"that many" — a much more
  expensive carrier than Myth Unbound. Bench Myth Unbound; record Reyhan's
  extra blockers.
- **The commander lookback family** ("for each time you've cast your
  commander from the command zone this game" and variants) — re-measured to
  **~17 supported lines**, not the parent's 21. Includes the Swarmlord,
  Jeska, Jirina Kudro, Jyoti, Captain Vargus Wrath, Myth Unbound, the two
  mana-dork "spend this mana to cast your commander" lines, and the
  blitz/instant-sorcery cost reducers. Note the family reads `EventCount
  SpellCast … ThisGame` with a **cast-origin rider**, so it is the
  `EventComplement` payload's shape, not a `Predicate.CastFrom`. **Re-measure
  before citing 21 — implementer must redo this count, not trust either
  number.**
- **The cheaper 11.** Confirmed: the six "At the beginning of your end step,
  if you haven't cast a spell from your hand this turn, …" lines are all
  present and identical in shape. These want the origin as a rider on
  `Happened` and need **no** `Command` row.
- **The placement's zone complement.** Parent says 19; a narrow regex (`was
  put into … this turn/game`) finds **10 supported lines** — "if a creature
  card was put into your graveyard from anywhere this turn", "if a land you
  controlled was put into a graveyard from the battlefield this turn", "if a
  permanent was put into your hand from the battlefield this turn", "if a
  card was put into exile this turn". Most name **both** a source and a
  destination, as the parent says. **Re-measure; the parent's 19 may include
  `would be put` replacements, which are a different reader.**
- **The counter pair's KIND complement.** Parent says 3. Narrow probes found
  none under the obvious phrasings. **Re-measure or drop the 3 from the
  round** — a design question justified by an unverified 3 is not worth a
  mechanism.

## 4. Settled rulings that bind it

- **Authority for the coordination**:
  `docs/decisions/kind-index-joins-union-marking-is-spelling.md` — the
  coordinated source and destination are **ordinary coordination at the zone
  sort, not a marked union row**. The brief must cite this.
- **`playableFrom`'s cells stay shared.** `complementLocates z = playableFrom
  z` (`Events.idr:613`) and the comment on it are load-bearing: "a
  complement's zone locates its object exactly when that zone could have
  been the play source." Adding `Command` re-measures **both** readings at
  once. Parent acceptance: "`playableFrom`'s shared cells stay shared."
- **`Predicate.CastFrom` is closed — do not redo.** It landed in chapter
  133, gated by `playableFrom` and `WholeZone`, seeding no zone. Its
  refusals are `badCastFromBattlefield` and `badCastFromStack` (**not**
  `badCastInGraveyard`, §0). Patrician Geist is its benched single-zone
  witness and is untouched.
- **`putDestZoneOk` excludes the battlefield deliberately** — "English
  writes 'put ONTO the battlefield', never 'into' it [CR#603.6a]" — so the
  `Command` cell must be argued on its own rule, not by analogy to the
  battlefield cell.
- **`Sideboard`**: `Semantics.Zone` has it; `Experimental.Words.Zone` does
  not. It is an [MTR] concept, not a [CR#400.1] zone. **Do not add it**
  while adding `Command`; if the implementer is tempted, record the refusal
  with the rule.

## 5. Pins — SETTLED (ratified 2026-08-26, pinned — do not reopen)

1. **THE parent's own named design question:** *"one mechanism with a sorted
   payload, or three tables."* Concretely — does `EventComplement` gain a
   second constructor carrying a `ZoneExpr` (and one carrying a
   `CounterKind`), gated by a new `lookbackZoneOk`-style table? Or do
   `Placement`/`CounterPlacement`/`CounterRemoval` keep their `False`
   `lookbackComplementOk` cells and get separate payload tables? The closure
   snapshot already records the answer's shape as owed: "the
   Placement/Counter pair's true complement (zone, kind) is a named but
   unbuilt second table — landing it would not touch this table's cells."

2. **Is the cast's origin a `rider` on the readers, or the same
   second-payload mechanism as (1)?** The parent framed them separately
   ("the origin as a RIDER on `Happened`/`EventCount` and nothing else").
   But "cast a spell **from your hand** this turn" and "a creature card was
   put into your graveyard **from anywhere** this turn" are both a
   `ZoneExpr` payload on a retrospective reader.

   **SETTLED, pins 1 and 2 together:** pins 1 and 2 are **ONE mechanism** —
   a **sorted second payload on `EventComplement` carrying a `ZoneExpr`**.
   The cast-origin rider is the **same mechanism**, answered together with
   the placement's zone complement, not separately. (The counter pair's
   `CounterKind` payload rides the same shape if its 3-line count survives
   re-measurement per §3; if it does not, name the reason it's dropped.)

3. **At what sort does the zone coordination live?** Per §0 it cannot be
   `Or` (which is over `Predicate`). Candidates named: a list in `ZoneExpr`,
   a list in `EventSource`, or a `Both`/`EitherOf`-style noun coordination
   lifted to the zone slot. The parent's premise that "nothing has measured
   which of `Or`'s four gates a zone phrase would have to answer" is a
   **non-question**.

   **SETTLED:** zone coordination is **ordinary coordination at the zone
   sort**, per `docs/decisions/kind-index-joins-union-marking-is-spelling.md`.
   The "which of `Or`'s four gates" question is a non-question — say so in
   the brief so the implementer does not go measure it.

4. **Does the `Command` row land at all?** Parent acceptance offered the
   escape: "If the command-zone row is taken, all ten named tables are
   measured in the same round; if it is not, the cheaper 11 land as a rider
   without it." The corrected count is **eleven** tables (§1.4) and the
   commander family is **~17** lines rather than 21.

   **SETTLED: the `Command` row LANDS.** Myth Unbound is the witness. All
   **eleven** named tables (§1.4) are measured in this round. Re-measure the
   ~17/21 commander-lookback count as part of the round's own work (§3) —
   do not carry forward either number unverified.

## 6. Files

`Experimental/Words.idr` (`Zone` and its seven local tables),
`Experimental/Events.idr` (`castComplementOk`, `playableFrom`,
`complementLocates`, `Possessable`, and `lookbackComplementOk`/
`bareLookbackOk` if the payload lands there), `Experimental/Triggers.idr`
(`putDestZoneOk`, `putSourceZoneOk`, `EventSource`), `Experimental/Phrase.idr`
(`EventComplement`, `ComplementWritten`, `complementDelta`, `HappenedTo`,
`Happened`, `EventCount`, `EventSum`, `ZoneExpr`), `Experimental/Macros.idr`
(the six `happened*`/`eventCount*` wrappers at `:1533–1592` take the
readers' argument list and will need the new slot), `Experimental/Cards.idr`,
`Experimental/ProofsD.idr` / `ProofsE.idr` / `ProofsG.idr`.

**Not in scope, explicitly:** `Semantics.idr` and `EmitTables.idr` — a
different `Zone` type that already has `Command` (§0).

## 7. Sequencing

All four sub-rounds of this split are semantically independent — none needs
another's output, and each has its own whole-card witness. Sub-rounds 1, 2
and 3 add constructors to `EventName` and clauses to the same seven tables
in `Events.idr`, plus rows in the same four tables in `Triggers.idr`, so
running two of them in parallel workspaces guarantees a textual conflict at
every table; they are recommended to serialise **1 → 2 → 3**. **This
sub-round (4) is parallel-safe with any of them** — its zone tables live
mostly in `Words.idr` and its reader payload in `Phrase.idr`, colliding with
the others in only two functions (`Events.idr`'s `playableFrom` and
`castComplementOk`). **If only one sub-round of the whole split can be run,
it is sub-round 1, not this one** — sub-round 1's pin was architectural and
the discard family is 96 lines, the largest single unblock in the bundle.

Standard constraints apply.
