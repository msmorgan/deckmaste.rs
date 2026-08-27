---
needs: []
---
# Event subjects — dealer-side damage and the player-subject attack

Split from `workbench-event-zone-and-cast-provenance` on 2026-08-26 (see
`docs/tickets/planned/workbench-event-zone-and-cast-provenance.md` for the
umbrella). Sub-round 2 of 4. **Size: S. Independent.**

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

- *"The dealer-subject read of damage IN GENERAL — 17 lines"*:
  > "It has NO `EventName`: `DamageTaken` is the victim's side of the happening
  > and `CombatDamage` is the combat dealer's; the general dealer's side is
  > neither."
- Routed item (2026-08-26, from `workbench-combat-assignment-and-forced-attack`):
  > "no attack-declaration event with a PLAYER subject — `Attacks`'s subject is
  > `Noun bs Object`, so 'Whenever an opponent attacks with one or more
  > creatures' (Tahngarth, First Mate) never binds the opponent; [CR#508.3d] is
  > the rule. An event-subject row, so it lands here."

**Why together.** Both are "an existing happening whose *other participant*
cannot be the grammatical subject." Neither adds a construction; both add a
subject seat and re-measure the `lookbackSubjectOk` / `lookbackComplementOk`
cells for one event. They will touch the same table blocks.

## 3. Corpus, re-measured

- **Dealer-side damage:** the parent's "17 lines". Re-measured to **14
  supported cards** carrying "dealt damage to", in **two frames**:
  - the relative-clause description (`Predicate`): "target creature that dealt
    damage to you this turn" — Reciprocate, Retaliate, Spear of Heliod,
    Giltspire Avenger, Otherworldly Escort, Brine Hag, Giant Albatross,
    Suffocation.
  - the condition (`Condition.Happened`): "if this creature dealt damage to an
    opponent this turn" — Dunerider Outlaw, Whirling Dervish, Wolverine,
    Hawkeye, Aegar, The Fallen.

  Both frames read the *same* new `EventName` with an `Object` subject and an
  `Object`-or-`Player` complement. The parent's line count is plausible
  (Suffocation writes two, several cards write it twice) but **must be
  re-measured before minting** — a new `EventName` is seven tables and the
  count is the whole justification.
- **Player-subject attack:** the parent's routed line understates it badly.
  **55 supported header lines** match `^(when|whenever)[^.]{0,60}attacks? with`
  — "Whenever you attack with one or more creatures", "Whenever a player
  attacks with five or more creatures" (Aurelia), "Whenever another player
  attacks with two or more creatures", plus Tahngarth. This is a large,
  well-attested family, not a one-card gap.
- Tahngarth's exact line, from local data:
  > "Whenever an opponent attacks with one or more creatures, if Tahngarth is
  > tapped, you may have that opponent gain control of Tahngarth until end of
  > combat. If you do, choose a player or planeswalker that opponent is
  > attacking. Tahngarth is attacking that player or planeswalker."
  Its **tail is unbuilt** — "a player or planeswalker that opponent is
  attacking" is the sibling routed item that stayed on
  `workbench-combat-assignment-and-forced-attack`
  ("No `Predicate` describes a player or planeswalker by what is attacking
  it"), and "Tahngarth is attacking that player or planeswalker" is a third.
  **Do not chase them** (vocabulary-only fence, §1.5). Bench the header
  alone, or find a cheaper whole-card carrier among the 55 — Aurelia and the
  plain "Whenever you attack with one or more Birds, scry 2" shape look far
  cheaper.

## 4. Settled rulings that bind it

- `lookbackSubjectOk AttackDeclaration Player = True` **already**
  (`Events.idr:279`), as do `AttackDeclaration Player Object`,
  `Player Player` and `Object Player` in `lookbackComplementOk`
  (`:370–372`), with `Object Object` a **named refusal** ("not English" —
  closure snapshot §2.2). So the retrospective side of the player-subject
  attack is done; **only the prospective `GameEvent.Attacks` subject seat is
  missing.** That fact halves the round and must be in the brief.
- `lookbackSubjectOk CombatDamage Player = False` (`:283`) is the *victim*
  side of the combat dealer's row and stays untouched. Parent acceptance:
  "the victim and combat-dealer rows are unchanged."
- The routed measurement from `workbench-union-family-macros` binds here:
  > "`lookbackComplementOk` refuses every joined complement, on a corpus zero.
  > Under `kindOfW JoinW = Object \/ Player` a joined complement is now
  > writable, so the refusal is live rather than unreachable; 34 distinct
  > 'dealt damage by' lines, five carrying 'or', none writing a cross-kind
  > union complement."
  Dealer-side damage's complement is exactly "to you"/"to an opponent"/"to it"
  — **re-measure the joined cell for the new name** rather than copying
  `DamageTaken`'s row, since this round is the one that makes the query new.

## 5. Pins — SETTLED (ratified 2026-08-26, pinned — do not reopen)

1. **Is the general dealer's side a new `EventName`, or a `Role` slot on the
   damage rows?** `data Role = Agent | Patient` already exists
   (`Events.idr:445`) with `counterRole`, used by the deed tables. A
   `DamageTaken`+`Role` shape would avoid seven new cells, but `CombatDamage`
   is already a separate name from `DamageTaken` for the same reason — the
   grammar has chosen "separate names per side" once. Parent acceptance
   allowed either: "gets its own `EventName` **or a stated reason it does
   not**."

   **SETTLED:** dealer-side damage gets its **own `EventName`** —
   separate-names-per-side, on `CombatDamage`'s own precedent. No `Role`
   slot.

2. **Does the player-subject attack widen `Attacks`'s subject to a `Kind`
   parameter, or mint a second row?** `Attacks` currently pins
   `(n : Noun bs Object)` with a battlefield `ZoneFits` gate and an
   `AttackDefender` slot threaded on `nomIntro n`. A player subject has no
   zone and takes no defender ("attacks with N creatures" names the
   *attackers* as a complement, not a defender). Kind-polymorphising the row
   would drag the gate and the defender slot into a `Kind`-indexed dance; a
   second row (`AttacksWith : (who : Noun bs Player) -> (with : Noun …
   Object) -> …`) maps 1:1 onto the corpus's own two spellings and onto the
   existing `lookbackComplementOk AttackDeclaration Player Object = True`
   cell.

   **SETTLED:** a **second row**, `AttacksWith (who : player) (with :
   creatures)` — not a kind-polymorphic widening of `Attacks`'s subject.

3. Whether the "attacks with" complement announces the attacking creatures
   for the tail to read back ("create that many … tokens", "the greatest
   power among those creatures" — both attested in the 55). This is
   `eventAfter`'s cell and it is the difference between a benched carrier and
   a header-only bench.

   **SETTLED:** the row **announces** the attacking creatures —
   `eventAfter` carries them, and the family reads them back.

## 6. Files

`Experimental/Events.idr`, `Experimental/Triggers.idr`,
`Experimental/Phrase.idr` (only if the dealer's side lands as a `Predicate`
frame beyond `HappenedTo`'s existing shape — it should not),
`Experimental/Cards.idr`, `Experimental/ProofsG.idr`.

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
