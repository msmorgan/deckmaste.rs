# trigger-1: the header's heads, windows and routed header items

Sub-round 1 of [workbench-trigger-and-interception-residues](workbench-trigger-and-interception-residues.md)
(the umbrella — authoritative for measurements, pins and acceptance). Owns its
sections: "The BARE CARD HEAD", "The header windows with no `TurnPart` row",
"The planeswalker and battle defenders" (record-keeping only — the disjunction
half is the join family's), "The double counted group" (recorded, nothing to
build), plus these routed items from the umbrella's tail: the Ring-tempts
trigger header; the negated-subtype intervening-if (Captain Marvel); the
"After" trigger word; the `AttacksWith` defender slot + the ordinal reset
window; the `AnyTriggered` head; the WINDOWLESS lookback (Fblthp family); the
CAUSER-voiced act + by-source agent phrase + the attacker-declaration
condition; and Bioplasm's typed-read halves (`verbedWordOk (TypeW _)`'s
`wasField` gate + the "creature card" head word — which may fall out of the
bare card head this round builds).

Key pins (umbrella authoritative): `badHeaderMainPhaseWindow` is NOT retired —
the MainPhase row landed and the pin refuses the SPECIFIC row; the six delayed
main-phase headers stay `PartUnclaimed` (this round does not buy the ordinal);
Stonebinder's Familiar benches or the destination reading's refusal is
recorded; declined rows carry their counts.

Note for the coordinator: the periphrastic causer complex is sub-round 3's,
but its Modular/AbilityClass half pairs with `AnyTriggered` — land the head
here so 3 can compose. Standard constraints apply.

## As landed (2026-08-27)

All counts re-measured over `data/derived/cards.jsonl` through
`jq 'select(.supported)'`. Gate: `idris/scripts/build` clean, 23/23, exit 0.
`cargo xtask cite check --list-noncompliant` empty; `cite check` 0 stale over
18,488 citations; `cite bless` added no rule (all cited rules already
registered); `jj diff --git … | cargo xtask cite audit --diff` read 55
citation sites.

### Built

- **The bare card head** — `Predicate.IsCard`, on [CR#109.2]'s own list of
  the four words that take a description off the battlefield, [CR#108.2] for
  what the word means and [CR#108.2b] for the token exclusion; [CR#109.2a]
  stays `InZone`'s (the word written WITH a zone). It is PLACELESS beside
  `IsSource`: `headIsPlaceless` covers it, so the phrase defaults to no zone
  rather than to the battlefield. A `cardTokenClashOf` cell refuses
  `IsCard` conjoined with `IsToken` [CR#108.2b].
  **Stonebinder's Familiar benches whole** — the destination-licensing
  reading was not needed and was not built. Family re-measured: 10 "one or
  more cards are put into" headers, 6 "a card is put into" headers; the
  wider bare-card-subject header family is 43 lines, of which the 27
  "cards leave your graveyard" lines already wrote through `InZone`.
- **The typed card word** — `NounWord.TypedCardW CardType`, [CR#109.2a]'s
  reading of "the exiled creature card" against `TypeW`'s [CR#109.2] one.
  The `wasField` gate on `verbedWordOk (TypeW _)` is RIGHT and was not
  loosened (deviation from the routed item's framing): what was missing was
  the other rule's word. `bioplasmTypedReadStillRefused` stands;
  `bioplasmTypedCardReadWrites` is its positive twin. **Bioplasm benches
  whole.**
- **The attack header's defender** — `AttacksWith` gains an
  `AttackDefender` threaded before the attackers (`defenderIntro`), on
  [CR#508.1b]. Family 6 lines. **Oath of Kaya benches whole** — so the
  PLANESWALKER defender lands, and the umbrella's "the header and the
  cross-kind disjunction arrive together or not at all" is corrected: only
  Rigo, Streetwise Mentor's "a player or planeswalker" needs the join; Oath
  of Kaya, Trouble in Pairs, Everett K. Ross, Akiri and Bitter Work name a
  single-kind defender. Battles stay at zero headers.
- **The ordinal's reset period** — `NthOccurrence` gains a
  `Maybe TurnPart`. It is NOT a `TriggerWindow` and could not be: a window
  says when the header may trigger and `windowOk` refuses a bare turn there,
  which is exactly why a period bounding a COUNT needs its own slot.
  Family: 176 supported header lines write an ordinal with an "each turn"
  reset. Resolute Veggiesaur's "each turn", previously benched as
  `DuringWindow Turn (Just EachPlayers)`, is now the reset it always was.
  **Trouble in Pairs' header writes all three arms.**
- **`AbilityClass.AnyTriggered`** — [CR#113.3c,113.9,115.1d], the same three
  rules that open `AnyActivated`. 22 "activated or triggered ability" lines
  plus 3 "target triggered ability" lines. **Stifle and Disallow bench
  whole.** Sub-round 3's Modular/`AbilityClass` half can compose on it.
- **The Ring's temptation** — no event row: [CR#701.54] makes it a KEYWORD
  ACTION and [CR#701.54d] states the trigger on it in the rules' own words,
  so it is one `verbFacts` row ("The Ring Tempts You", patientless on
  [CR#701.54a,701.54c]) read through `VerbedEvent`, exactly as the scry is
  [CR#701.22b]. 8 supported headers. Nazgûl's header benches; the card is
  not whole (its second line INSTRUCTS the temptation, and its last line is
  a deck-construction rule with no seat).
- **The negated-subtype intervening "if"** — the routed premise was WRONG:
  it writes, and always did. **Captain Marvel, Apex Avenger benches whole**
  (1 line; 5 sibling "if it isn't a mana ability" lines).
- **The attacker-declaration condition** — `Predicate.BeingDeclaredAttacker`
  on [CR#508.1a,508.1f,508.1k]: the tap [CR#508.1f] performs during the
  declaration happens before [CR#508.1k] makes the creature an attacker, so
  `Not Attacking` would say the wrong thing. 2 lines. **Verity Circle
  benches whole.**
- **The windowless lookback** — `Lookback.Triggering`, the occurrence the
  ability triggered on, on [CR#603.2,603.2c]; it writes no word. Family 9
  lines. **Archfiend's Vessel benches whole** (its "you cast it from your
  graveyard" disjunct needs no window at all — that is the object's own
  casting history, which `CastBy`/`CastFrom` already read).
- **The double counted group** — recorded AND witnessed:
  `hostileInvestigatorHeader` puts a `CountedGroup` in both slots of one
  verbed act, the first a player, the second the new bare card head. The
  card's block is `investigate`, unchanged.

### Declined, with counts

- **"During the declare attackers step"** — 26 supported lines: 4 trigger
  headers (Misleading Signpost, Portal Mage, Portal Manipulator, Windshaper
  Planetar) and 22 cast/activation restrictions. ZERO benchable carriers.
  The 4 headers all want "reselect which player or permanent … is
  attacking", which no `Effect` row writes (`BecomesAttacking` states a new
  defender, it does not reselect), and Portal Manipulator additionally wants
  "their opponents", an opponent-of-a-named-player that `Opponent`
  (opponent-of-You) does not spell. The 22 restrictions want a spell-level
  cast-timing seat, which does not exist — `Timing` sits on activated
  abilities alone — and most want an "only if you've been attacked this
  step" lookback window besides. Every existing `TurnPart` arm has bench
  uses; an unreachable arm would be the first. Add it when a carrier
  reaches the bench.
- **The per-their-turn possessor** — 1 supported header line (Valgavoth,
  Harrower of Souls; the other two "during each of their turns" lines are
  static play permissions). Its carrier cannot bench on the possessor
  alone: the header is a LIFE-LOSS event, and `LifeLoss` has no `GameEvent`
  producer (the umbrella's own non-interceptable partition records the pair
  as `EventUnclaimed`).
- **The CAUSER-voiced act** — 1 supported line (Karmic Justice). `Causer` is
  a nullary marker (`AnEffect`) and the printed causer is a DESCRIBED "spell
  or ability an opponent controls", so the arm needs `Causer` widened to a
  noun at the object/ability join — the same widening sub-round 3's
  periphrastic causer complex needs, and the umbrella pins that complex to
  land together or not at all. Routed on to sub-round 3.
- **The by-source agent phrase on a lookback** — 1 supported line (Cobra
  Trap, "was destroyed this turn by a spell or ability an opponent
  controlled"). Still WAITING at its L1/OPEN-2 ledger tag: no [CR#609.7]
  by-source rider exists in the tree (grep: zero sites).

### Pins and zeros

- `badHeaderBareTurnWindow` still refuses `DuringWindow Turn Nothing` and
  was not retired — the ordinal reset is a separate slot precisely so that
  pin keeps its argument. (**Deviation:** the umbrella names this pin
  `badHeaderMainPhaseWindow`; no pin of that name exists in the tree, and
  `badHeaderBareTurnWindow` is the one that carries the argument the
  umbrella describes.)
- The six delayed main-phase headers stay `PartUnclaimed`; no ordinal work
  was bought for them.
- "**After**" is not a fourth `TriggerWord` and not a spelling of one, and
  the refusal is recorded on `data TriggerWord` itself: [CR#603.1] and
  [CR#113.3c] close the list, and the 4 printed "After" lines are not
  triggered abilities — 2 are roll MODIFIERS ([CR#706.2,706.2b];
  Xenosquirrels, Night Shift of the Living Dead) and 2 are the added-phase
  statement (Full Throttle, World at War), which is `AdditionalPart`.
- `badFlipEvent`, `badUnflipInstruction` and every other pin still refuse:
  full 23/23 build with the `Proofs*` modules is clean.
