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
