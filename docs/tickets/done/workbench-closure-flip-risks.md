---
needs: []
---
# Decide the fifteen closures most likely to flip

The workbench's closures are ranked by how likely a future printing or round is
to move the cell. A **single-value slot** and a **closed-`Nat` table with a
near-miss cell** rank above a large attestation grid, because the grid's many
zeros are independent — one printing moves one cell and the closure survives —
where the slot and the small table have exactly one cell between them and a
redesign. The full ranking, its widening costs and the ten grids ranked below it
are in [the closure tables](../../idris-workbench-closure-tables.md) §1; this
ticket is the work the ranking implies.

Eleven of the fifteen are this ticket's. They fall into two shapes that share a
fix: a **hard-fixed value slot** (give the slot its parameter or its missing
cell, retire the pin that names the refusal, keep the measurement as a table
rather than as the constructor's shape) and a **catalog hole inside a rule-closed
frame** (state the frame from the rule, and let an attestation gate — not a
missing row — carry the "no card writes this yet" fact). Under the settled
direction both are the right address anyway: a measured phrasing fact is table
content, not type-level law, and not the absence of a constructor.

Authority:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md).

Several of the fifteen are **investigations**, not code changes. Acceptance for
those is a **recorded decision** at the site — widen, bless-as-permanent with a
rule (not a count) doing the closing, or re-measure — written into the
docstring, not a note elsewhere.

> **Ruling 2026-08-22 — pins refuse rules-impossibility only.** An earlier
> banner here read "a measured zero lands as a pin"; that was reversed the
> same day and is void. The governing doctrine is
> `docs/memory/rulings/measurements-live-in-pins.md`: the semantics admits
> anything the Comprehensive Rules make meaningful, a pin refuses a term only
> where a rule makes it meaningless and its docstring names that rule, and a
> count is never a refusal. Leaving a printed card unrepresentable is the
> worst outcome. Every "N of M" below is a demoted snapshot
> (`docs/idris-workbench-closure-tables.md`), useful only as a pointer to the
> cell; measurements that decided a flip go in this ticket's `## As landed`,
> never in a docstring. Line anchors below are stale — re-grep by name.

## The five hard-fixed value slots

- **`heldUntilOk` — the until-rider's single True cell** (Experimental.idr:3538).
  Exactly one True cell — `Composite Exile (Move with empty riders, no counters)`
  — closes the entire "until [event]" rider; the counters-present,
  riders-present and ordered-list variants are each pinned False *by name*
  (`badHeldUntilWithCounters`) rather than by any general rule. Widening cost: "a
  new exile rider shape would need its own True/False cell"; the [CR#610.4]
  phase-out family already has attested lines and no word.
- **`LosesAllCounters` — the always-all removal** (Experimental.idr:3472). The
  structural "no amount slot" decision rests on **four corpus lines total** (Final
  Act, Suncleanser, RadAway, Leeches), two per cell — the thinnest total in the
  workbench behind any structural decision. Widening means an amount slot, which
  would then need `CounterAmt`'s own gates at a fifth site.
- **`CreationVoice`'s fourth cell `CreatedByCauser`** (Experimental.idr:2110). A
  four-cell table whose fourth cell was added at ch81 **because all three attested
  causer-creation lines also happen to write the control phrase** — a bare
  causer-only line is 0 lines (`badCausedCreationBareControl`). One printing of
  "if an effect would create a token" without the control phrase forces a fifth
  cell, and the sort refusal beside it ("an effect would create" — no term
  exists, an effect is not a player) has to be answered rather than ledgered.
- **`TagBody`'s `ScryB`/`SurveilB` possessor, fixed at `You`**
  (Experimental.idr:4137). Both rows hardcode the looker as `You` against **2 of
  440 scry lines naming another scryer** (Bumi King of Three Trials, Kozilek's
  Command — already ledgered) and 0 surveil lines. Two attested counterexamples
  already exist for the fixed value; the sibling `MillB` has the same possessor as
  a free parameter, for the same reason inverted. Match that shape.
- **`GainsDesignation`'s four not-pinnable cells** (Experimental.idr:3393).
  City's blessing, enduring story, monstrous and renowned are refused as not
  pinnable because each is conferred by a keyword trigger whose only English is
  reminder text, and the docstring says outright these **will flip when the
  keyword round lands**, not a refusal a proof can hold — the closest thing in the
  workbench to a closure whose own authors expect it to break. The asymmetric,
  ungated subject range recorded beside it (monarch, 10 third-party / 25 self) is
  recorded overgeneration the widening must revisit.

## The four catalog holes inside rule-closed frames

A rule closes the frame, but the rows inside it were whittled to what the corpus
witnessed, so the absent members are a printing question wearing a rules-closed
hat. In three of the four the rule already tells us the full set.

- **`LibOrdinal`'s hole at Sixth** (Words.idr:369). A five-row closed vocabulary
  (Second/Third/Fourth/Fifth/Seventh) with a gap **between two populated cells**,
  over 30 supported cards distributed third 14, second 13, fourth/fifth/seventh 1
  each. Nothing in [CR#401.7] forbids "sixth from the top" — the gap is a
  printing accident, and the two neighbours at count 1 show how thin the evidence
  for a populated cell can be. Widening also touches the two `ordinalFits` cells
  beside it (offset × bottom, offset × order-rider), both currently 30/30 False.
- **`CardType`'s six absent rows** (Words.idr:9). Nine of the CR's fifteen types
  are present ([CR#205.2a]); six are wholly absent — conspiracy 29 cards, plane
  182, scheme 102, vanguard 107, phenomenon 21, dungeon 0 — held out only by the
  corpus's supported-card flag, a **project** decision and not a rule. Every total
  table over the card type (`combatant`, `deedType`, `typeRank`, `permanentType`,
  `spellType`, `retainable`, `ascribesAsType`, `castableTy`) takes a totality
  error when one lands, which is the real cost and the reason to decide it
  deliberately rather than on the day.
- **`Supertype`'s two unminted rows** (Words.idr:2049). [CR#205.4a,205.4b] closes
  the set at five; Legendary, Basic and Snow are witnessed, **World and Ongoing
  unminted**. Unlike `Color`, deliberately ported whole as a rules-fixed catalog,
  this one was whittled to witnessed rows — an inconsistency inside the same
  file. `HasSupertype`'s description reader takes the same two rows.
- **`keywordCounterOk`'s printing-backed Falses** (Words.idr:1978). The frame is
  rule-closed ([CR#122.1b]'s fifteen-name list) but **every False cell inside it
  is printing-backed** — Convoke, Improvise, Storm, Ward, Protection, Enchant,
  Equip, Ascend, Storied, Renown, Flash, CumulativeUpkeep and Skulk are each held
  out by a "zero supported lines" grep for "X counter". A single "ward counter"
  printing flips a cell with no rule to consult, and the table is a *forcing
  function*: every future `Keyword` row must add a cell before it can be a counter
  kind.

Deciding "port the rule's whole set" versus "keep witnessed rows" is the
substance of this half, and the answer may differ per frame — `Color` and
`Supertype` already differ today. Whichever way each goes, the file should stop
being inconsistent about it and should say which discipline it follows and why.

## The two chapter-level re-readings

Both rank in the fifteen and neither is a cell: what is at risk is an argument.

- **`grantableAb`'s three quotation cells** (Experimental.idr:5042). Three cells
  "flipped" when quotation was re-read as a **spelling** rather than a
  construction, on a 763-vs-0 split (non-keyword grants always quoted) and a
  thousands-vs-0 split (nullary keyword grants never quoted), with **exactly one
  crossing class** — the parameterized keywords, resolved by the parameter itself
  (Ward bare 49 all mana-cost / quoted 4 all em-dash). A single bare quoted-flying
  line, or a crossing Ward line, unpicks a whole chapter's re-reading. The
  widening cost is the spelling-not-construction argument itself, not a cell.
- **`keywordCardOk`'s Flash cell** (Experimental.idr:5389). `keywordStackRegime`
  says Flash is `AtCasting` per [CR#113.6e]; the printing table says 598/598
  permanent-only, 0 spell (`badFlashOnInstant`, "the word would be vacuous
  there"). **The two-table split exists purely because they disagree on this one
  keyword.** One printed instant carrying Flash collapses the reason for two
  tables; the cost is either a merged table (which ch45 already decided against)
  or a third table. The same shape recurs at `windowOk`/`headerWindowOk`'s
  six-cell disagreement, which this ticket should read at the same time.

## The four ranked cells this ticket does NOT own

Recorded here so the ranking stays whole and nobody re-derives them as gaps.

| rank | site | why not here |
|---|---|---|
| 2 | `PlayAsThough = HadFlash` (Events.idr:736) — a single-value slot for the whole "as though" counterfactual, resting on 87 of 88 supported sentences writing flash; the one exception (Shaman's Trance's zone counterfactual) exists today and is *parked*, not closed by rule | the second row is named and blocked on three unbuilt things and stays parked; `playSourceOk`'s "both contents never written together" (0/88) would have to be re-measured, not inherited |
| 4 | the five closed `{1,2}` count tables — `NextUntapCount` 66/1, `ExtraTurnCount` 32-sentence/2, `SkipCount` (2nd cell = Eater of Days alone), `PhaseCount` 47/1 (Full Throttle alone), `CapBound` 5/2 (Words.idr:1093–2616); five structurally identical two-row tables, each explicitly ruled **not** shared with its siblings despite the identical cell shape, each second cell held by one card, each carrying an explicit three-count pin (`badUntapNextThree`, `badThreeExtraTurns`, `badThreeAdditionalPhases`, `badUntapCapThree`) | `workbench-structure-phrase-and-toolchain` unifies them under one `AttestedCount` table carrying all five measurements, rather than widening them one at a time. A single "three extra turns" printing widens one and re-opens the not-shared ruling for all five |
| 8 | `YouAnd`'s player half (Experimental.idr:1315) — not a parameter at all, hard-fixed to `You` because all 35 supported sentences write second person and "target player and creatures they control" is 0 lines; widening costs a `Noun bs Player` argument plus `NotMixedGroup` re-derived (today [CR#109.5]'s "second person written once") and a re-measurement of the no-binding argument | belongs to the union family: `workbench-union-gate-spelling-rehome` and `workbench-union-family-macros` |
| 13 | `admitsSpan CostModification` (Events.idr:910) — 1 of 653 lines carries a duration (Cheering Fanatic) and the cell is deliberately not flipped, because flipping it would rename a `SpanUse` row; the closure is held by a naming cost, not by evidence | `workbench-docstring-evidence-audit` carries it as a live discrepancy with the evidence assembled |

## The ten grids ranked below the fifteen

Deliberately ranked below, because their many zeros are independent measurements
rather than a single load-bearing cell: a printing moves one cell and the closure
survives. Each is a real maintenance surface, and each has an owner. This ticket
does not work them; it records them so the ranking's tail is not lost.

| grid | owner |
|---|---|
| `spanUse`'s ~90-cell duration grid (Experimental.idr:2509), only four cells Unclaimed, the rest Unattested | `workbench-turn-structure-and-procedures` |
| the `Subtype` catalog's ~130 rows, grown one witnessed card at a time (Words.idr:1501) | `workbench-choice-chosen-and-ascription` |
| `ascribesAsSubtype`'s nine-self-naming-words census against every creature type (Words.idr:1866) | `workbench-choice-chosen-and-ascription` |
| `attachHeadOk`'s participle×head grid (Words.idr:2211) | `workbench-keyword-parameters-and-attachment` |
| `AndAlso`'s 1,433 sentences with no CR anchor at all (Experimental.idr:2888) | `workbench-static-frame-and-ability-values` |
| `KindJoin`'s full 2×4 grid over 326 occurrences (Experimental.idr:204) | `workbench-union-gate-spelling-rehome` |
| `HasCounters`' 221 frames (Experimental.idr:178) | `workbench-counter-family-residues` |
| the `eventUse`/`eventSpan`/`ReplUse` triple over 26 `EventName` rows (Events.idr:107–659) | `workbench-event-zone-and-cast-provenance` |
| `negatable`'s full-row census over every `Predicate` constructor (Experimental.idr:1024) | `workbench-conditional-and-coordination` |
| `pumpSignsOk`'s four disagreeing-zero cells (Experimental.idr:2705) — the four zeros are one canonicality fact rather than four independent ones, which is why it sits at the top of this list rather than the bottom of the previous one | `workbench-amount-comparison-and-quantity` |

## Measured cell (2026-08-21)

- `admitsSpan CostModification` (Events.idr) is held False on "exactly one" attested line; measured, 5 of 188 cost-modification lines carry a duration on the static (4 "until your next turn", Cheering Fanatic's "this turn"). Flip and rename the `SpanUse` row.
- `visibilityOk LookAt WholeHand = False` against 35 "look at <someone>'s hand" lines; its [CR#402.3] comment argues backwards — the rule denies what all 35 grant. Flip, and fix the cite.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Events.idr`; evidence bench
`idris/src/Experimental/Cards.idr`.

## Acceptance

- Each of the eleven ends as a parameterized slot with its measurement moved to a
  table cell, a widened catalog, or a **recorded decision** in the docstring at
  the site stating a rule (not a count) doing the closing.
- Every retired pin's refusal is still refused, by the table.
- Each rule-closed frame states its discipline once, in the catalog's own
  docstring — "port the rule's whole set" or "keep witnessed rows", and why.
- No total table silently loses totality: a widened catalog lands with every
  reader's new cells measured, not defaulted.
- The two chapter-level re-readings end with the argument restated or a
  re-measurement recorded, not with a cell quietly flipped.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed (2026-08-22)

Doctrine applied: `docs/memory/rulings/measurements-live-in-pins.md` (the
banner's earlier "measured zero → pin" rule is void). Five of the eleven were
already settled by
[Pins refuse rules-impossibility only](../done/workbench-pins-refuse-rules-impossibility-only.md);
six were widened here. No pin was retired, because none of the six refusals
lived in a pin — each was a constructor's shape or a catalog's missing row, and
the build confirms every existing pin still refuses (a widened gate that made a
pin's clause reachable would have failed `impossible`).

| cell | outcome |
|---|---|
| `heldUntilOk` — the until-rider's single True cell | **widened**. The counters/riders/ordered-list refusals and `badHeldUntilWithCounters` were already gone (pin sweep); what remained was `SetStatus _ _ = False`, which left [CR#610.4]'s phase-out family unspellable. `SetStatus PhasedOut` now takes the rider, and the docstring names both [CR#610.3] and [CR#610.4] as the only two one-shots the CR hangs "until" on |
| `LosesAllCounters` — the always-all removal | **widened**. Renamed `LosesCounters` with a `{default Nothing amt}` slot; unwritten is the "all" spelling. Grounded on [CR#728.1]'s own rules text ("removes one rad counter from themselves"), not a count. `optAmtIntro` added beside `amtIntro`; `effIntro`/`preIntro`/`annIntro` read the slot. Macros `Macros.losesCounters` for the counted form and `Macros.losesAllCounters` for the bare "all" spelling, so the bench never leaves the slot's absence unnamed |
| `CreationVoice`'s fourth cell `CreatedByCauser` | **resolved by** the pin sweep — `CreatedByCauserPlain` (a causer with no stated controller, [CR#111.2] supplying one) exists and `badCausedCreationBareControl` is gone. Nothing changed |
| `TagBody`'s `ScryB`/`SurveilB` possessor, fixed at `You` | **widened**. Each is now a two-row table over how the one player [CR#701.22a]/[CR#701.25a] names is written — second person, or the anaphor reading back the clause's subject (`ScryTheyB`/`SurveilTheyB`). Both rows repeat the same mention in looker and slice, which is the rule doing the tying. A free `Noun bs Player` was tried first and rejected: the body's context is `nomIntro who`, which auto-search cannot invert, so `Does You Scry …` stopped elaborating |
| `GainsDesignation`'s four not-pinnable cells | **resolved by** the pin sweep — `designationGiven` is True for CitysBlessing, EnduringStory, Monstrous and Renowned, and `GivingWarrant` carries `Instructed` vs `InExpansionOf`. No "will flip when the keyword round lands" text survives. Nothing changed |
| `LibOrdinal`'s hole at Sixth | **widened past the hole**. [CR#401.7] states the offset as "Nth from the top" for any N, so the five-name enum was the wrong shape entirely: `LibOrdinal` is now `Nth : (n : Nat) -> {auto 0 nz : IsSucc n}`. `Nth 0` is the one refusal (positions count from the top card). The `off` slot never had an offset×bottom or offset×arrangement gate to widen — the ticket's `ordinalFits` does not exist |
| `CardType`'s six absent rows | **widened**. [CR#205.2a]'s whole set is ported: Conspiracy, Dungeon, Phenomenon, Plane, Scheme, Vanguard. Nine total tables took the new rows — `combatant` ([CR#701.14a]), `attachHeadOk` (12 cells), `ascribesAsType`, `typePrintOrder`, `permanentType` ([CR#110.4]), `spellType` ([CR#112.1]), `retainable` (True — [CR#205.1a] exempts only instant and sorcery), `deedType` (24 cells), `castableTy` ([CR#311.2,312.2,313.2,314.2,315.3], [CR#309.2]) — plus `Eq CardType`. The catalog's docstring states the discipline |
| `Supertype`'s two unminted rows | **widened**. [CR#205.4a]'s whole set: World and Ongoing minted, `Eq` extended, discipline stated in the docstring beside `Color`'s |
| `keywordCounterOk`'s printing-backed Falses | **resolved by** the pin sweep — the docstring already grounds every zero on [CR#122.1b]'s fifteen-name enumeration, and re-reading the rule confirms each True is on that list and each False is off it. Nothing changed |
| `grantableAb`'s three quotation cells | **decision recorded at the site**. Quotation is rendering: [CR#613.1f] applies ability-adding effects over abilities as such and [CR#113.3] gives four kinds, neither restricting which kind may be added nor how the added text is written, so no cell indexes on quotation. The two Falses are structural — `Spell` is [CR#113.3a]'s spell ability, followed only while an instant or sorcery resolves; `AlsoForKeywords` is not an ability but the spelling that extends one line's keyword list |
| `keywordCardOk`'s Flash cell | **resolved by** the pin sweep (`SpellCard Flash = True`, `badFlashOnInstant` gone); **decision recorded** here. [CR#702.8a] restricts flash to no card type — on an instant the word is redundant, which is not meaningless, and on a sorcery it is not even redundant. The two-table split does not rest on a disagreement: `keywordStackRegime` says from which zone an ability functions, `keywordCardOk` says whether the word may sit on the card. `headerWindowOk` no longer exists and `windowOk`'s two Falses are [CR#500.1] plus a missing antecedent, so the sibling six-cell disagreement the ticket paired with this one is also gone |

The "four ranked cells this ticket does NOT own" and "the ten grids ranked
below the fifteen" sections were read and left untouched; nothing in them
changed.

### Bench

| widened cell | witness |
|---|---|
| `heldUntilOk` phase-out rider | `oubliette` — Oubliette, "When this enchantment enters, target creature phases out until this enchantment leaves the battlefield." (The bench spells the first sentence; the card's "Tap that creature as it phases in this way." rider is not modelled.) Three supported lines write a phase-out "until" rider, five across the whole corpus |
| `ScryB` non-second-person looker | `bumiScryMode` — Bumi, King of Three Trials, "Target player scries 3." Kozilek's Command writes the same shape |
| `Supertype` World | `concordantCrossroads` — Concordant Crossroads, "World Enchantment / All creatures have haste." (whole card) |
| `CardType` new rows | `selenia` — Selenia, a vanguard card whose whole printed text is "Creatures you control have vigilance." (whole card) |
| `Supertype` Ongoing | none benchable: every printed Ongoing card is an ongoing scheme, and every such card's text needs the unmodelled "abandon this scheme" verb. The row is [CR#205.4a]'s and lands without a witness |
| `LosesCounters` amount slot | none benchable: the one printed counted removal reaching a player is "Remove up to five counters from target artifact, creature, planeswalker, or opponent", blocked by the open disjunctive-head gap. The slot is [CR#728.1]'s and lands without a witness |
| `LibOrdinal` `Nth` | the existing ordinal cards re-bench unchanged (`Nth 2`/`Nth 3`/`Nth 5`/`Nth 7`) |
| `Conspiracy`/`Dungeon`/`Phenomenon`/`Plane`/`Scheme` rows | no whole card is spellable yet — schemes and phenomena need "abandon"/"encounter"/"planeswalk", conspiracies need hidden agenda, dungeons need venture. Selenia benches the shape for the family |

### Ledger

- **`CardClass` has no command-zone arm.** `cardClassOf` is binary, so a
  conspiracy, dungeon, phenomenon, plane, scheme or vanguard card classifies
  as `PermanentCard` and is read by `cardAbilityOk PermanentCard`. That is
  wrong for a card [CR#110.4] keeps off the battlefield, and it is why
  `selenia` typechecks. A third class belongs to a card-frame round, not to a
  catalog widening.
- **`typesCombinable` does not refuse a command-zone type beside another.**
  `[Conspiracy, Creature]` passes, because neither `permanentType` nor
  `spellType` is True for it. No rule refuses the combination outright, so
  this is tolerated overgeneration rather than a defect; a card-frame round
  can tighten it if a rule turns up.
- **`typePrintOrder`** gave the six rows positions 9–14 in printing order.
  It is spelling-only and no gate consumes it, so the values are provisional
  and belong with `workbench-type-line-order-is-spelling`.
- **Scry/surveil lookers are a two-row table, not a free parameter.**
  `ScryTheyB`/`SurveilTheyB` cover "target player scries N"; "each player
  scries N" is still unspellable, because the anaphor needs a single Player
  mention and `Each` introduces a plural one. A plural read-back mention is
  the missing piece, and it is the same gap `MillB` has.
- **`effEq` ignores `LosesCounters`' new amount slot**, matching the
  neighbouring `GetsCounters` row which compares nothing at all. If `effEq`
  ever becomes load-bearing for counter clauses, both rows need the amount.
- **The phase-out "until" lines** measured 2026-08-22 with the mtg-rules
  `corpus` script: three in the supported scope the workbench measures by
  (Oubliette, Out of Time, The Moment), five with `--all` (adding the
  plane Unyaro and the Attraction Ferris Wheel). The one printed
  counted player-counter removal ("Remove up to five counters from target
  artifact, creature, planeswalker, or opponent") likewise. Both are
  recorded here and nowhere else; neither count decided a flip — [CR#610.4]
  and [CR#728.1] did.
- **`Nth 0` is pinned.** `badZerothFromTop` (ProofsG) witnesses the
  `IsSucc` refusal, so the one rules-impossibility the widened `LibOrdinal`
  introduces cannot go silently reachable. `Nth 1` ("first from the top")
  is admitted and duplicates the plain `OnTop` spelling — [CR#401.7] makes
  it meaningful, so the duplicate is tolerated overgeneration.
- **`grantableAb (Spell _) = False` is narrower than [CR#113.3a] for one
  subject.** `Gains` takes a `Noun bs Object`, which can name a spell, and
  [CR#113.3a] makes text on a resolving instant or sorcery spell a spell
  ability. No printed line grants a quoted ability to a spell (0 corpus
  lines), so the cell refuses nothing printed; the docstring's argument
  ("an object handed one has nothing to follow") holds for a permanent
  subject only.
- **`attachHeadOk Enchanted _ = True` covers the six new types too**, so
  "enchanted conspiracy" is admitted. That is the pre-existing catch-all,
  which already admitted "enchanted instant"; a rule tightening it belongs
  with the attachment round.
