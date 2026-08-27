---
needs: []
---
# The placement complement's subject row, and the zone gates' held questions

Routed from `workbench-event-zone-4-zone-catalog-and-reader-payload` (close,
2026-08-26), which landed the `Command` row and the `FromZones` origin
payload. Four remainders, one region:

1. **The placement zone complement — 61 measured lines** ("a creature card
   was put into your graveyard from anywhere this turn": ~35 relative-clause,
   ~13 condition, ~5 count). Structurally blocked, not deferred:
   `lookbackSubjectOk Placement` is `False` at both kinds, and the arrival
   gates (`putDestZoneOk`/`putSourceZoneOk`) live in `Triggers.idr`, which
   imports `Phrase.idr` — unreachable from `EventComplement`'s seat.
   `FromZones` is the mechanism it will use; the subject row and an
   event-keyed re-homing of the gates are the work.
2. **The negated origin** — "from anywhere other than your hand" (3 lines):
   a complement-side negation shape `FromZones` does not carry.
3. **`putDestZoneOk` drift (from the module split):** its docstring says the
   battlefield is excluded [CR#603.6a] while the cell reads
   `Battlefield = True`. Decide the direction and fix both to agree.
4. **The held `badCastFromBattlefield` pin:** no rule categorically refuses
   a battlefield cast, so `playableFrom Battlefield = False` may be a
   count-based refusal — a pins-doctrine defect if so. Settle it knowing the
   flip cascades through `complementLocates`' shared cells.

## Consumption boundary

`idris/src/Experimental/Events.idr`, `Triggers.idr`, `Phrase.idr`,
`Words.idr`; `Cards.idr` bench; `Proofs*.idr`. No Rust crate.

## Acceptance

- Each item lands or ends in a written rule-backed verdict; item 3's
  docstring and cell agree; item 4 ends with the pin retired or re-grounded
  on a rule.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-event-disjunction-seat (close, 2026-08-27):** an `EventSource` naming every zone but one — "from anywhere other than the battlefield" (9 sites; one printing excludes a SET, "a graveyard or exile"). Same negated-origin shape as item 2 here; UNBLOCKS Syr Konrad, the only whole-card three-armed header carrier.

- **Routed from workbench-shuffle-into-library (close, 2026-08-27):** an ENTRY-ORIGIN seat — `Enters` carries no source-zone slot and `lookbackOriginOk` admits an origin for `SpellCast` alone, so "entered from your library" (Fblthp's first ability, and the "entered from …" family) has no seat prospective or retrospective. The `FromZones` payload is the mechanism; opening `Entry`'s cells is this ticket's origin-gate work, so it lands here.

---

## As landed (2026-08-27)

### The shape, in one sentence

The origin phrase became ONE type serving both seats. `EventSource` moved
from `Triggers.idr` into `Phrase.idr` (beside `ZoneExpr`, where the
retrospective seat is declared), its `FromZone` arm now takes a LIST, and it
gained `FromAnywhereBut`. `EventComplement.FromZones` carries an
`EventSource` in place of its own zone list, so "from your graveyard",
"from your hand or library", "from anywhere" and "from anywhere other than
the battlefield" are one vocabulary written at the prospective `from` slot
(`PutInto`/`Leaves`/`Enters`) and at the retrospective payload alike.

### Item 1 — the placement complement (landed)

- `Events.idr`: `lookbackSubjectOk Placement Object = True`
  ([CR#400.7] — what was put somewhere is an object, never a player;
  the Player cell stays shut on the same rule).
  `bareLookbackOk Placement Object = False` — a placement writes at least
  one end of its move, and with neither written the clause names none.
- **The gates re-homed, event-keyed.** `putDestZoneOk`/`putSourceZoneOk`
  left `Triggers.idr` and became `placementDestOk`/`placementOriginOk` in
  `Events.idr`, reached through two event-keyed tables that both seats
  consult: `lookbackDestOk : EventName -> Zone -> Bool` (new) and
  `lookbackOriginOk` (extended). `Triggers.putDestOk`/`putSourceOk` are now
  one-liners over them, so the prospective header and the retrospective
  reader can no longer disagree.
- `Phrase.idr`: `EventComplement` gained `IntoZone (to) (what)` — the
  ARRIVAL zone, with the origin nested inside it because English writes
  the destination first ("into your graveyard from anywhere"). The
  one-way nesting is `complementSourced` (new) against `complementPlain`
  (a row added): a destination may wrap an origin, an origin may not wrap
  a destination, and neither may wrap its own kind.
- **Re-measured.** 50 instances over the supported corpus (32,213 cards)
  under `(was|were) put (into|there)` + a window, not the ticket's 61 —
  the SPLIT is the ticket's: 32 relative-clause, 12 condition, 5 count.
  The reading the ticket did not name is the one that matters for shape:
  **33 of the 50 write the arrival zone as "there"**, naming it by the
  described noun's own zone, and only 17 write a zone. A complement that
  writes the origin alone is that reading, so no zone anaphor was built.

### Item 2 — the negated origin (landed; Syr Konrad whole)

- `EventSource.FromAnywhereBut : List (ZoneExpr bs)` — one shape, both
  seats, gated by the same `lookbackSourceOk` as the positive list: every
  zone named (to admit or to exclude) must be one the event's clause may
  name as an origin, and a phrase naming none is no phrase.
  A list, not a single zone, because [CR#400.1] lists the zones a game
  has and naming the excluded ones picks out the rest exactly whether one
  or several are named. Every SUPPORTED printing excludes one; the
  set-excluding printing the routed append mentions ("a graveyard or
  exile") is `"Name Sticker" Goblin`, which is **not supported**, so the
  list is justified by the rule and not by that line.
- `FromAnywhere` needed a row-level gate, since it names no zone to ask a
  cell for: `eventNamesOrigin : EventName -> Bool` (derived from
  `lookbackOriginOk`, no new table), so "died from anywhere" stays
  refused exactly as "died from the battlefield" is.
- **Re-measured.** "from anywhere other than [zone]": 42 instances / 39
  distinct lines. By excluded zone — hand 32, battlefield 6, exile 1 (Rory
  Williams). The routed append's 9 sites for the battlefield is 6 here.
  The retrospective hand-origin trio the base ticket names (Impending
  Flux, Surge of Brilliance, Spider-Man 2099) is confirmed at 3.

### Item 3 — the `putDestZoneOk` drift (docstring wins; cell flipped)

`placementDestOk Battlefield = False`. The docstring was right and the
cell was wrong. The ground is not the count: [CR#603.6a] writes a
permanent's arrival as an event that puts it **onto** the battlefield and
gives that arrival its own ability form, which is `Entry`/`Enters` and not
this event — a construction boundary, not a measured zero. (The corpus
agrees at the surface: 1,070 supported lines write "onto the battlefield"
and none writes "into" it.) Nothing broke: no `PutInto` site in the tree
named the battlefield as a destination. New pin
`badPlacementIntoBattlefield`.

### Item 4 — the held `badCastFromBattlefield` pin (RETIRED)

**Verdict: the refusal was count-based and is gone.** No rule refuses a
battlefield cast. [CR#601.2a] moves the card "from where it is" and
excludes no zone; [CR#601.3] makes casting permission-gated and leaves the
zone set to "a rule or effect", so the rules close no zone against one;
[CR#604.6] has the notion "any zone that you could cast or play it from"
and enumerates nothing. `playableFrom (Just Battlefield) = True`, and
`badCastFromBattlefield` is deleted. `badCastFromStack` stands — its
[CR#112.1] ground is structural (a cast ENDS on the stack).

**The cascade, traced and paid.** The flip would have broken two standing
pins through the shared cell, so the sharing was the defect and it is
gone:

1. `complementLocates` was `= playableFrom` and is now its own table with
   the same seven values, each on the rule that gives the word its zone —
   [CR#112.1] for "spell", [CR#110.1] with [CR#305.1] for
   "creature"/"land"/"permanent", and [CR#701.5b] for why Garruk's Horde
   casts "creature spells" from a library without the two zones
   disagreeing. This keeps `playSourceOk`'s written-source clause and the
   "play lands from your graveyard" family intact.
2. `playSourceOk zn Nothing Nothing` now asks `complementLocates zn`
   rather than `playableFrom zn`: with no source phrase written the
   complement's own sort word is the only thing that could name one, so
   the question is whether that word locates at all. Same values, so
   `badPlayFromBattlefield` and `badPlayFromStack` still refuse — on the
   rule that actually backs them.

The previous round's ratified "`playableFrom`'s shared cells stay shared"
is deliberately overridden here, and this is the reason: the two readings
agree at six cells and disagree at the battlefield, and only one of them
had a rule for it.

### Item 5 — the entry origin (both halves landed; Fblthp still short)

- `Enters` gained a `from : Maybe (EventSource bs)` slot on `PutInto`'s
  model, gated by `EntrySource`/`entrySourceOk` over the same
  event-keyed table (66 call sites carried through as `Nothing`).
  The subject keeps its battlefield gate, which describes it where the
  event leaves it.
- `lookbackOriginOk Entry z = entryOriginOk z` (new table): every zone but
  the battlefield, on [CR#400.7] — a zone change is a move from one zone
  to ANOTHER. The stack is open on [CR#608.3] (a resolving permanent
  spell reaches the battlefield from there) though no line writes it; a
  measured zero is not a refusal. New pin `badEntryOriginBattlefield`.
- **Re-measured**: "enters from …" 13 lines, "entered from …" 8.
- **Fblthp is NOT whole, and the blocker moved.** Its first ability no
  longer wants the origin — "entered from your library" has both seats
  now — it wants a WINDOW. "If it entered from your library" scopes the
  reading to the entry that triggered the ability, and every `Lookback`
  value names a stretch of time or [CR#608.2c]'s "this way" cause, none of
  which this line writes. Its docstring in `Cards.idr` now says so.

### Witnesses benched (`Cards.idr`)

- `faithsReward` — WHOLE. "Return to the battlefield all permanent cards
  in your graveyard that were put there from the battlefield this turn."
  The relative-clause reading, the family's largest (33 of 50).
- `ichorShade` — WHOLE. The condition reading with both ends written, and
  a joined-kind subject ("an artifact or creature").
- `asmiraHolyAvenger` — WHOLE. The count reading.
- `syrKonradTheGrim` — WHOLE, both abilities. **Item 2's acceptance
  witness**: the corpus's only three-armed header whose tail reads nothing
  back, and arm 2 is the negated origin.
- `oscorpIndustriesReturn` — the entry origin's prospective seat.
- `theLostAndTheDamnedEntryArm` — the negated origin at the entry seat.

### Pins

Added: `badEmptyOriginExclusion`, `badDeathOriginAnywhere`,
`badPlacementIntoBattlefield`, `badNestedDestination`,
`badEntryOriginBattlefield`.
Deleted: `badCastFromBattlefield` (item 4).
Re-grounded: `badPlacementLookback` (ProofsE) — it refused on the
`Placement` subject cell, which is now open; it refuses on the bare
complement instead, and its docstring says why.

### Gate

Cold `idris/scripts/build` 23/23, 0 errors, 0 warnings.
`cargo xtask cite check --list-noncompliant` empty; `cite check` 17,837
citations, 0 stale (no bless needed — every rule cited was registered);
`cite audit --diff` 30 sites, each read against its rule.

### Ledger — needs routing to live planned tickets

1. **No bare card word.** "a creature card" outside a determinate zone has
   no spelling: a head with no zone predicate is battlefield-sorted, so
   Syr Konrad's arm 2, Disa the Restless and both Ultrons spell "a
   creature"/"an artifact" where the printing writes "card". Wanted: a
   card word that leaves the head placeless.
2. **The windowless lookback.** "If it entered from your library", "if it
   was kicked"-style readings that scope to the event that triggered the
   ability. `Lookback` has five values and all name a time or a cause.
   Fblthp, the Lost is its whole-card witness, and Archfiend's Vessel,
   Prized Amalgam, Breathless Knight, Grist and Kotis are the family
   (the "entered from … or was cast from …" disjunction, 8 lines).
3. **The counter pair's `CounterKind` complement** — carried forward from
   the parent round, untouched here.
4. **`castComplementOk (Just Battlefield) = False`** was not re-examined
   this round. It is a third reading over the same zone and is now the
   only battlefield refusal in the cast family that this round did not
   re-ground.
5. **Dimir Strandcatcher and Banon** — the negated origin at the
   RETROSPECTIVE seat writes, but neither card is whole (Strandcatcher's
   attack trigger with a counted surveil; Banon's once-per-turn cast
   permission from among graveyard cards). No whole-card witness for that
   combination was benched.
