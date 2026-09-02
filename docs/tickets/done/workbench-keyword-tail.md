# workbench-keyword-tail

The keyword umbrella's survivors, gathered at its close (both sub-rounds done
2026-08-28; details in done/workbench-keyword-{1,2}-*.md):

- **The COMPOUND keyword parameter** — one decision, two payers: equip's
  "[quality] [cost]" ([CR#702.6c]; 22 lines / 19 shapes; holds Luxior,
  Steelclaw Lance, Commander's Plate) and suspend's "N—[cost]"
  ([CR#702.62a]; Veiling Oddity's last blocker). Both need a new
  `KeywordParamShape` constructor + `paramShapeOf` cell + `keywordFacts`
  changes; keyword-1's no-fifth-shape pin deferred it to a round that arrives
  WITH the witnesses (they exist — mint the shape once for both). Craft is
  NOT this ([CR#702.167a]+[CR#118.1]: its two slots are one activation cost —
  ruled in keyword-1's close).
- **The becomes-attached/unattached `GameEvent` rows** — 11 + 4 lines, cheap
  now that `AttachTo`/`Unattach` exist.
- **Fence-deferred keyword sub-machinery** (recorded, waits for the verifier
  transition per the standing scope fence): Crew's full crewing, Station's
  symbols, the Class level ladder, the Case to-solve/solved pair, the Room
  door vocabulary.

Re-measure at claim.

- **Update from the transform round (2026-09-02):** the Room door items got
  counts — 28 "When you unlock this door" triggers, 2 unlock/lock
  instructions, 3 door-counting reads; the unlock designations
  (`LeftHalfUnlocked`/`RightHalfUnlocked` [CR#709.5c]) and
  `SpecialAction.UnlockDoor` are LANDED; what waits here is the door NOUN
  [CR#709.5j] and the unlock trigger header [CR#709.5h].

- **Routed from workbench-static-frame (close, 2026-09-02):** a `KeywordTerm`
  arm on the ability LOSS (Shay Cormac, Tolaria — 2 lines), pairing the
  class-term with `LosesAbilities`.

## As landed (2026-09-02)

Every count below was re-measured over `jq 'select(.supported)'` card lines.

**The COMPOUND keyword parameter — landed, minted once.**
`KeywordParamShape` gains ONE arm, `CompoundParam CompoundHead`, where
`CompoundHead = QualityHead | NumberHead` names what the word's rule writes
before the cost. The two rules disagree on one further fact and it is written
down separately as `compoundHeadOptional`: [CR#702.6c] says equip abilities
"MAY further restrict" their targets, so a quality head is one a line may
leave out, while [CR#702.62a] writes "Suspend N—[cost]" as the whole of what
the word means. That optionality is why `keywordParamFits` now compares
through a new `paramShapeFits` instead of shape equality — Steelclaw Lance and
Commander's Plate each print BOTH spellings, one under the other, so the two
must fit one `keywordFacts` row (`keywordFactsIn` gives a word exactly one).
`KeywordParam` gains `ParamQualityCost` (a `Predicate bs Object`, fixed at
`Object` because [CR#702.6c] restricts creatures and [CR#702.6e] a
planeswalker — no counterpart to [CR#702.16k]'s player) and `ParamNumberCost`.
`keywordCosts` now answers True for a compound too, on [CR#118.1]'s ground.
Craft is untouched and still `CostParam` — keyword-1's ruling stands.

- Equip re-measured: **21 lines over 19 distinct spellings** (ticket said 22 /
  19). 20 are [CR#702.6c] qualities, 1 is [CR#702.6e]'s "Equip planeswalker
  {1}"; both are written with the compound, and the docstring records that
  [CR#702.6e] is a variant ability rather than a target restriction — the row
  records the printed shape and leaves the reading to the word's rule.
- Suspend re-measured: **70 lines over 53 distinct N-and-cost pairs**, 5 of
  them "Suspend X—[cost]". The letter is NOT opened here: every X line writes
  {X} in the cost beside it, so it is the cost's announcement [CR#107.3a] and
  the count reads it. `abLetterDelta` gains no compound cell, and says why.
- `keywordFacts` rows: "Equip" moved to `CompoundParam QualityHead`; "Suspend"
  minted at `CompoundParam NumberHead`.
- Benches: `steelclawLance` (whole), `commandersPlateEquip`,
  `luxiorEquipLines`, `veilingOddity` (**whole** — the card's last blocker).

**The becomes-attached/unattached `GameEvent` rows — landed.**
`BecomesAttached` (host kind-indexed to object-or-player on [CR#701.3a]'s own
words) and `BecomesUnattached` (host `Object`; the narrowing is the corpus's,
not the rule's — [CR#701.3d]'s last sentence reaches a player). Two new
`EventName` arms, `Attachment` and `Unattachment`, on
`CounterPlacement`/`CounterRemoval`'s model; cells filled in `sameEventName`,
`eventHasMagnitude`, `lookbackSubjectOk`, `eventName`, `eventIntro`,
`eventAfter`, `eventSubjectPlur`.

- Re-measured: **10 "becomes attached" lines** (ticket said 11) — 7 trigger
  headers and 3 "As this Equipment becomes attached …" clauses that are NOT
  headers ([CR#614.1c,614.1e]'s "as" family at an event neither names;
  recorded, unbuilt) — and **4 "becomes unattached"**, all headers, all naming
  the host.
- Benches: `enormousEnergyBlade`, `brambleElemental`, `graftedWargear` (all
  three whole).

**The `KeywordTerm` arm on the ability LOSS — landed.**
`LosesAbilities`' list element becomes `AbilityLost bs`: `LostWritten` (an
`AbilityAt`, carrying `grantableAb` at the constructor) or `LostTerm` (a
`KeywordTerm`). One list, not a second slot — Shay Cormac coordinates a term
third and fifth between bare words, and the printed order is the list's. New
`keywordFacts` rows "Banding" [CR#702.22a] and "BandsWithOther" [CR#702.22c];
new terms `wardAbilities` and `bandsWithOtherAbilities`.

- Re-measured: **4 lines write a term at this seat** — Shay Cormac, Tolaria,
  and two "all 'bands with other' abilities" losses. Banding has 40 supported
  lines; "bands with other" 8 (6 grants, 2 losses).
- Benches: `shayCormacStrip`, `shelkinBrownie` (whole), `tolaria` (whole).

**The Room door items — DECLINED behind the scope fence, with the reasoning
recorded** (new ledger section at the tail of `Cards.idr`).

- The door NOUN [CR#709.5j] does not reduce to a subtype-word-like noun. "A
  door is a half of that permanent", and [CR#709.5b] makes each half part of
  ONE object's copiable values, so a `NounWord` at kind `Object` would say the
  opposite; and the locked/unlocked adjective cannot be a `HasDesignation`
  read on the door, because [CR#709.5c] gives the designations to the
  PERMANENT and names the half inside them. What it costs is a half-level
  referent, its relation to its permanent, and a half-keyed designation read.
- The unlock trigger header [CR#709.5h] (28 lines, all the identical string
  "When you unlock this door,") declines with it, on two counts: its patient
  is that same half-level referent, written as a deixis on the printed half;
  and it has no carrier — `SplitCard` is [CR#709.1]'s ordinary split card,
  where [CR#709.5]'s shared-type-line permanent (two static abilities, shared
  subtypes [CR#709.5a], copiable half existence [CR#709.5b]) is not modelled.
  [CR#709.5i]'s "whenever you fully unlock a Room" (16 lines, all Eerie) waits
  on exactly the same two things.
- Door counts CORRECTED against the standing pins: **4** unlock/lock
  instructions, not 2 (the pin at `SpecialAction`'s `UnlockDoor` is fixed);
  **2** counting reads, not 3; plus 2 mana-spend restrictions naming the act,
  which the landed `SpecialAction` row already spells.

**The fence-deferred four — re-recorded, not built**, in the same ledger
section: Crew's full crewing (180 "Crew N" lines, [CR#702.122a]); Station's
symbols (31 "Station" lines, 46 "N+" symbol rows, [CR#702.184b]); the Class
level ladder (68 "{cost}: Level N" bars, [CR#716.2]); the Case pair (13 "To
solve" and 13 "Solved", [CR#719.3,702.169a]).

**Gates:** `idris/scripts/build` clean from an empty `build/` —
`23/23: Building Cards (src/Cards.idr)`. `cargo xtask cite check
--list-noncompliant` reports 0; `cargo xtask cite check` reports 0 stale over
20095 citations; `cite bless` registered five newly cited rules
([CR#702.169a,702.22a,709.5a,709.5b,719.3]); the full round diff was piped
through `cite audit --diff` and every
rule read against its claim — which is what caught two over-claims
([CR#701.3d] does reach a player, and the attachment "as" clause is not
[CR#614.12]'s), both corrected.
