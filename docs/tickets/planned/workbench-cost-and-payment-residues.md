---
needs: []
---
# Close the cost and payment vocabulary's counted residues

Everything the cost region still refuses, taken as one claimable unit: the five
populations the alternative-cost static leaves outside it, the two large
alternative-cast surfaces (free cast and keyword cost catalog), the four pieces
the cost-statement row left out, cumulative upkeep's `Cost`-side constructions,
the cost gate's four remaining gaps, the cost-shaped-mana families, and the
scaled payment's three ledgered units. They are one round because they share the
same declaration region — `Cost`, `ManaCost`, `CostsToCast`, `AltCost`,
`ScaledMana`, the keyword catalog's cost columns — and closing any one of them
moves slots the others read.

## The five populations the alternative cost row refuses

The alternative-cost static landed over **123 supported lines**. Five counted
populations — 54 lines in all — sit outside it, each refused by a different slot
of the same region (the row's subject, the declined cost's value, the play
permission, the cost action, the designation's scope).

- **The 14 generic grants** — As Foretold, Fist of Suns, Dream Halls: "the mana
  cost **for spells you cast**", a subject slot this row has none of, and
  `CostsToCast`'s shape.
- **The 11 non-mana declined costs** — equip 3, cycling 2, echo, crew, power-up,
  and two single-mana ones. They want the declined cost to be a **value**, which
  is `Pay`'s standing decline.
- **The 23 pronoun-tail lines** — Worldheart Phoenix's "by paying … rather than
  paying its mana cost": play permissions with an alternative-cost rider, so
  `MayPlay`'s and not this row's.
- **Invigorate**, 1 line, 1 cell — its payment is a life **gain** and
  `costActionOk` admits only the loss. The table was deliberately not widened.
- **The 5 commander-gated free spells** — Deflecting Swat, Fierce Guardianship,
  Deadly Rollick, Flawless Maneuver, Obscuring Haze — blocked because
  `CommanderD`'s scope is `HeldByCard` and `HasDesignation` reads only the
  object- and player-held rows.

### Two corrections carried so they are not re-bought

- **Fall of the Titans' Surge line is not this gap.** It is Surge's *reminder
  text*, verbatim-shared with ten other Surge cards and part of a 956-line,
  25-keyword reminder population. The alternative-cost round does not unblock it,
  and the two were never one round.
- **"If you've cast another spell this turn, you may pay {1}{U} rather than pay
  this spell's mana cost" is not a corpus line** — **zero** supported lines gate a
  "rather than pay" offer on having cast another spell. It reads as a reminder
  condition or a printed gate moved across the phrasing boundary. Do not scope
  against it.

## The alternative cast costs and the keyword cost catalog

Two large measured surfaces sit outside the cost vocabulary: the free cast of a
card that is not this one, and the keyword alternative cost whose text is almost
entirely reminder. They share the keyword catalog and the same question — what an
alternative cost substitutes for. The keyword parameter's remaining binding rides
along, being the same catalog's other slot.

### The free-cast family — 296 non-reminder lines

"you may cast that card without paying its mana cost" (Aetherworks Marvel,
Chancellor of the Spires, Breaching Dragonstorm): a licence to cast a
**different or later** card for nothing, where an alternative cost substitutes a
payment for the spell being cast.

The whole "without paying … mana cost" surface is **547** lines: **235** are
keyword reminder text (Suspend 64, Cascade 46, Plot 37, Rebound 37, Discover 25,
Cipher 15, Paradigm 5), **16** are [CR#118.9]'s own second phrasing on the self
and already landed, and the remaining **296** are this entry.

### The keyword alternative cost — 956 lines across 25 keywords, 955 reminder

"you may cast this spell for its [keyword] cost [if …]": Flashback 213, Morph
150, Madness 61, "for its mana cost" 56, Foretell 51, Bestow 41, Disguise 38,
Escape 34, Mutate 34, Warp 32, Megamorph 31, Evoke 28, Overload 27, Disturb 24,
Dash 22, Miracle 20, Blitz 17, Freerunning 13, Cleave 12, Harmonize 12, Prowl 11,
Spectacle 11, Surge 11, Impending 5, Mayhem and Web-slinging 1 each.

Reminder text is not a card's own line, so **none of it is bench-payable on its
own** — but the keywords are catalog rows and their costs are what Fall of the
Titans and its ten Surge siblings actually want. Surge, Prowl and Freerunning
share one template ("for its ⟨keyword⟩ cost if ⟨you-did-X-this-turn⟩"); the rest
carry their own condition and zone templates.

### The keyword parameter's where-clause binding

The parameter slot and the number shape both landed (Renown, whose payload is an
ordinary `Amount`), so a letter-valued parameter is
`ParamNumber (DefinedLetter LetterX)` and the only question left is the
**where-clause that binds it**. The static rider is ready for it. Witnesses:
"Ulamog has annihilator X, where X is the number of +1/+1 counters on it",
"mobilize X" (Avenger of the Fallen, Infantry Shield), "Monstrosity X" (2 cards)
— and [CR#701.37c] is the rule that makes the monstrosity case a linked value
rather than a fresh read.

## The four pieces the cost-statement row left out

The cost-modification row landed with 334 self lines and 18 unqualified class
lines (`CostsToCast`, `StaticKind.CostModification`). Four pieces stayed out,
each small, each its own thing, and all four in the same declaration region.

Authority:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md)
— for the third piece: whether an ability becomes a referent sort is a question
about the kind index, judged by lowerability and algebra.

### 1. The conditional self-reduction — 127 lines

"This spell costs {1} less to cast **if** you control a Wizard" (Academy
Journeymage). The wrapper already composes — the 3 "as long as" lines land free,
probed — so this is a **marking word**, and `CondMarking` was measured as the
closed pair "as long as"/"unless". **Before adding a third, settle whether it is
the same construction at all**: [CR#601.2f] determines a total cost once and
locks it in, where [CR#611.3a] applies a static ability's effect "at any given
moment", so the "if" may be a one-time test rather than a standing one.

### 2. The coloured payload — ~25 lines

"costs {2}{R} more to cast"; Strive's "costs {R} more to cast for each target
beyond the first". The magnitude slot holds an `Amount` — the generic component —
because the X-rider family needs a letter to read there. **A coloured run is a
second payload shape, not a widening of this one.**

### 3. The activation variant — 45 self lines plus the class lines

[CR#602.2b] extends the whole cost machine to activation costs in one sentence,
so the rules cost nothing and the blocker is the **subject**: "This ability costs
{1} less to activate" needs a noun for an **ability**, and every `Noun` in this
grammar is an object or a player. A referent sort, not an extension. Agatha of
the Vile Cauldron additionally writes a floor rider ("This effect can't reduce
the mana in that cost to less than one mana").

### 4. The durational cost line — 1 line

"Spells with the chosen name cost {1} less to cast this turn" (Cheering
Fanatic). Its chosen-name subject is spelled, so **the span cell is that card's
only blocker**. `admitsSpan CostModification` is False in all ten cells, and
flipping the cell would rename a `SpanUse` row.

### Not this round's

Abaddon the Despoiler's neighbour line ("spells you cast … have cascade") is not
a cost residue: the grant row landed, and what that line waits on is cascade —
one of the nine words core cannot resolve — plus the from-zone qualifier and a
duration.

## Cumulative upkeep's cost-side residues: a second cost-action table and a cost disjunction

`Keyword.CumulativeUpkeep` landed at `CostParam` ([CR#702.24a] writing
"Cumulative upkeep [cost]" as [CR#702.21a] writes ward), with GLACIAL CHASM,
ABOROTH, SHELTERING ANCIENT, POLAR KRAKEN, YAVIMAYA ANTS, ILLUSIONARY FORCES,
VEXING SPHINX and MANA CHAINS benched. What is left is two `Cost`-side
constructions serving every cost carrier, plus a counted ledger of blocked
carriers. Measured surface: 80 cards print the line, 5 grant it in a quotation, 1
names it in a spend restriction (that one is the spend-restriction entry's below,
not this entry's).

### A second `costActionOk` table, over `Cost` — 4 cards

Four cards write a cost action this line's cell disagrees with: Braid of Fire
("Add {R}", `AddMana`), Psychic Vortex ("Draw a card", `Draw`), Varchild's
War-Riders ("Have an opponent create a 1/1 red Survivor creature token",
`Create`) and Wall of Shards ("An opponent gains 1 life", `ChangeLife Up`).

The table's zeros were measured AT THE COLON and hold there. These four are the
same table read by a SECOND carrier, so closing them wants a second table over
`Cost` — not a widened cell. NONE IS PINNABLE: every one is attested.

### The mana-OR-mana cost — 4 cards

Arctic Nishoba {G} or {W}, Earthen Goo {R} or {G}, Jötun Owl Keeper {W} or {U},
Krovikan Whispers {U} or {B}. English "or" between two RUNS, not a hybrid symbol:
`ManaCost` is a list and `Compound` an AND-join. This is a `Cost`-side
disjunction serving every cost carrier, and [CR#702.24a]'s own example spells out
what it means per age counter.

### The blocked carriers, each with its one gap

- Karplusan Minotaur — no coin-flip verb in the vocabulary at all.
- Herald of Leshrac — a one-shot control change, where `GainsControl` is a
  `StaticEffect`.
- Jötun Grunt — "a single graveyard", a uniqueness phrase over the zone's
  possessor; the move itself clears the cost gate.
- Balduvian Shaman — describes a permanent by NOT having the keyword, and
  `HasKeyword` demands `KeywordParamless`.
- Phyrexian Soulgorger — the sacrifice cell's other card, blocked on the
  `Phyrexian` name-straddle in its type line, not on this row.
- Cover of Winter — its second line prevents X combat damage where X reads the
  age tally: a prevention size read off a count. Its third line is benched.

### Out of scope, recorded so it is not folded in

The longhand mirror without the keyword, 2 cards — Cyclone and Phantasmal Sphere
write the whole escalating procedure out ("put a wind counter…, then sacrifice it
unless you pay {G} for each wind counter on it"). That is a boundary rather than
this row's scope, and three more cards (Myr Prototype, Primordial Ooze, Rogue
Skycaptain) share the escalation while swapping the consequence.

## The cost gate's four remaining gaps

The scaled payment (`Cost.ScaledMana`) landed and was never the gate family's
only blocker. The family is 30 lines over 29 cards, 25 of them scaled, and NONE
is benched but Hipparion: every one of the 25 carries a second gap. Those gaps
are four distinct pieces of work, counted here so nobody scopes a round that
unblocks nothing.

- **The DEFENDING-PLAYER noun**, 15 lines, and the piece to take FIRST — it is
  what Propaganda and Ghostly Prison wait on, and the marquees are here.
  "Creatures can't attack you unless their controller pays…": Propaganda,
  Ghostly Prison, Windborn Muse, Koskun Falls, Elephant Grass, Baird, Archon of
  Absolution, Dáin, Forbidding Spirit, Norn's Annex, Onakke Oathkeeper, Sphere
  of Safety, Summon: Yojimbo, Collective Restraint, Archangel of Tithes. The
  patient slot is `Noun bs Object` and `deonticPatientOk GateT Attack Agent` is
  `PatientRefused`; both have named this case in their own comments since
  chapter 40. Awesome Presence's "defending player" is the same noun read at the
  BLOCK role.
- **The ANAPHORIC COUNT**, 9 lines ("pays {1} for each of THOSE creatures").
  `CountOf` takes a `Predicate`; "those creatures" is a plural mention and
  `Those (TypeW Creature)` is a `Noun`, so the count has nothing to run over.
  Overlaps the defending-player set — a round closing both closes 15+ lines at
  once.
- **The COORDINATED DEED**, 3 lines ("can't attack or block" — Myr Prototype,
  Cowed by Wisdom, Whipgrass Entangler). Chapter 38's coordinated-event gap at
  this site, unchanged.
- **The PAYER NOUN INSIDE AN ACTION COST**, 2 lines (Heat Wave, Sivitri Dragon
  Master). The gate DERIVES its payer and spells it; a life payment is a clause
  that writes its own payer noun, and "their controller" is not a noun the
  cost's context can see. The scaled LIFE payment itself needs nothing — it
  composes today.
- Also still here, not a fifth piece: the Aura carrier (Brainwash, Oppressive
  Rays, Awesome Presence, Cowed by Wisdom).

## Naming a cost from a mana sentence: cost-shaped spend purposes, storage counters, cost-valued runs

Three refused families share one missing notion — a COST as something a mana
sentence can name, rather than an object it can point at. `SpendPurpose` takes a
spell or an ability's source because those are objects; a cost is not, and a cost
SHAPE is a third thing again.

### The spend restrictions that name a cost — 9 of 164, with their shapes

- 6 restrict to a cost's CONTENTS: "on costs that contain {X}" (Rosheen
  Meanderer, Nexos, Elementalist's Palette) and "to pay cumulative upkeep costs"
  (Adarkar Unicorn). The landed `Keyword.CumulativeUpkeep` row does NOT unblock
  Adarkar Unicorn: the restriction names a cost SHAPE, which is this gap and not
  the keyword's.
- 2 disjoin a cost arm with a cast arm — Qarsi Deceiver, Unblinking Observer.
- 1 names a SPECIAL ACTION — "to turn permanents face up", Overgrown Zealot.

### The zone-qualified spend purpose — 7 lines

"Spend this mana only to cast spells from your graveyard" (Rootcoil Creeper, Lord
of the Forsaken, Interdimensional Web Watch, Altar of the Lost, Mm'menon) and the
two negative "This mana can't be spent to cast spells from your hand" (Karolina
Dean, Vhal). A third `SpendPurpose` shape, and it belongs with the restrictions
above rather than with the cast-zone provenance row that measured it.

### Mana named by a cost — 2 lines

The surface [CR#106.8..106.11] exist for: "Add mana equal to enchanted
permanent's mana cost" and Ice Cauldron's "Add this artifact's last noted type
and amount of mana". A produced run is `ColorOrColorless` by [CR#106.1b]; these
two name a PRINTED COST and let the rules translate it — a different row, NOT a
widening of `ProducedRun`.

### The storage counter and the Mana Batteries — 36 lines over 18 cards

The five Mana Batteries write the family's shape twice over: "{T}, Remove any
number of charge counters from this artifact: Add {B}, then add an additional {B}
for each charge counter removed this way". Two things are missing:

- an ANY-NUMBER cost quantity, and
- a for-each over what a COST removed.

NOT missing: "an additional". Its 36 lines are all the second add in a chain or a
trigger off another production, so it is a discourse spelling and no slot.

## The scaled payment's three ledgered units

Three counted residues of the scaled-payment round, each too small for its own
round and all three the same shape of purchase: a unit or a factor the magnitude
cannot hold. Take them together or not at all.

- **The COLORED scaled payment**, 3 lines: Cyclone's "{G} for each wind counter
  on it", Thelon's Curse's {U}, Norn's Annex's {W/P}. `ScaledMana` carries an
  `Amount` and no symbol, exactly as `CostShift` does — the two ledgers should
  be closed together if either is.
- **The {X} PER-UNIT**, 4 lines: Collective Restraint, Sphere of Safety, War
  Cadence, War Tax. Wants a product of two amounts, where `Times` takes a
  written numeral [CR#118.4].
- **The "PLUS AN ADDITIONAL" compound**, 3 lines: Rune Snag, Spell Stutter,
  Concerted Defense. A fixed base beside a scaled one under a coordinator that
  `Compound`'s comma does not spell.

### Measured zeros to preserve

- Zero lines write a scaled ALTERNATIVE cost. The row reaches `AltCost`
  structurally and no card asks, so it is owed nothing.
- Zero lines write a scaled non-mana ACTION cost — "as an additional cost … for
  each" returns nothing.

From the v1 comparison (2026-08-24): the cost-tag readback channel (`CostTag`, `PaidCost`, `CastWith`, `TimesPaid`, `WasPaidWith`) is not this ticket's and is scheduled as [workbench-cost-tags-and-paid-readbacks](workbench-cost-tags-and-paid-readbacks.md).

## Routed ledger items

Items from closed round tickets that this ticket owns. One line each, citing
the done ticket that recorded them.

- **`costActionOk`'s remaining ~40 `False` rows.** Five structural ones were
  pinned on [CR#118.1] (`Continuously`, `InsteadOf`, `Delayed`, `HeldUntil`,
  `Reflexively`); the rest — `DealDamage`, `Fights`, `Distribute`, the
  turn-structure rows, the non-tap `SetStatus` rows, `GetsCounters`,
  `LosesAllCounters`, `RemoveFromCombat`, `Regenerate`, `CantBe`,
  `GainsDesignation`, `GameBecomes`, `GameDrawn`, `CopyStack`,
  `ChooseNewTargets`, `Choose`, `AddMana`, `Expose LookAt`, `Search`, `Shuffle`,
  `Create` and the remaining labeled-action tags — are attestation and wait for
  this round (consolidated open gap 10) —
  `docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md`.
- **The "As an additional cost to cast this spell" frame** has no construction;
  it is Burn at the Stake's remaining blocker —
  `docs/tickets/done/workbench-verb-labels-open.md`.
- **Cavern-Hoard Dragon's `{X} less` cost-reduction rider.** The phrase itself
  is landed and benched; the whole card needs the rider —
  `docs/tickets/done/workbench-amount-comparison-and-quantity.md`.

## Consumption boundary

`idris/src/Experimental.idr` (`AltCost`, `CostsToCast`, `MayPlay`,
`costActionOk`, `HasDesignation`, `Pay`; the cast licence, the alternative-cost
payload, `ParamNumber`/`DefinedLetter` and the where-clause rider;
`StaticKind.CostModification`, the payload slot, the `Noun` sorts; `Cost`,
`ManaCost`, `Compound`, `Keyword.CumulativeUpkeep`, `GainsControl`, `HasKeyword`;
the gate's patient/deontic tables and the cost context; `SpendPurpose`,
`ProducedRun`, `Cost` and its quantities, the for-each over a paid cost;
`ScaledMana`, `CostShift`, `Times`), `idris/src/Experimental/Words.idr`
(`CommanderD`; the keyword catalog and its cost/parameter columns; the
defending-player noun and the anaphoric count's predicate reading; cost and
spend-restriction spelling), `idris/src/Experimental/Events.idr` (`admitsSpan`,
`SpanUse`) for the durational cell, the pin modules
`idris/src/Experimental/Proofs*.idr` — including
`idris/src/Experimental/ProofsD.idr` for the cost-modification row's pins and
`idris/src/Experimental/ProofsF.idr` / `idris/src/Experimental/ProofsG.idr` for
the alternative-cost pins — evidence bench `idris/src/Experimental/Cards.idr`.
No Rust crate.

## Acceptance

- Each of the five alternative-cost populations is either written or left refused
  with its count recorded against the slot that refuses it — no population
  silently absorbed by the landed row.
- The free cast distinguishes the card being licensed from the spell being cast;
  the self phrasing already landed is not re-minted.
- Keyword costs land as catalog columns, with the shared Surge/Prowl/Freerunning
  template written once and the per-keyword condition and zone templates kept
  distinct; Fall of the Titans and its ten siblings are the payment test.
- The reminder-text lines stay un-benched, and the count that says so is
  recorded where the catalog rows are.
- The monstrosity parameter reads as a linked value, not a fresh read.
- The "if" is settled as either the same construction as the closed marking pair
  or a distinct one-time test, with the rules reading written down where the
  marking is defined.
- The coloured payload lands beside the `Amount` payload, not in place of it —
  the letter must still be readable there.
- Flipping `admitsSpan CostModification` renames the `SpanUse` row deliberately,
  and the other nine cells stay measured.
- Cheering Fanatic pays a bench line if the span cell lands.
- The four disagreeing cost actions are admitted through a second table keyed by
  the `Cost` carrier; the colon-measured zeros of the original table are
  unchanged.
- The cost disjunction sits on `Cost` and serves every carrier, not on
  `ManaCost` as a widened list.
- Braid of Fire and at least one mana-OR-mana carrier bench.
- Propaganda and Ghostly Prison bench whole; the 15 defending-player lines and
  the 9 anaphoric counts write.
- `deonticPatientOk GateT Attack Agent` opens only for the measured case; the
  comments naming this case since chapter 40 are updated, not left stale.
- A cost shape is a named thing a spend purpose can take; the object-taking
  `SpendPurpose` cells are unchanged, and the zone-qualified purpose reuses the
  same third shape rather than minting a fourth.
- The cost-valued run is its own row and `ProducedRun`'s `ColorOrColorless` is
  not widened.
- "An additional" gets no slot.
- A Mana Battery benches, or the any-number cost quantity is recorded as the one
  remaining blocker.
- The scaled payment's 10 ledgered lines write; the symbol-carrying half closes
  `ScaledMana` and `CostShift` together.
- Every measured zero above — the two scaled-payment zeros and the
  "cast another spell this turn" zero — survives the round as a zero, with a pin
  apiece rather than a silence.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-payment-event-residues (close, 2026-08-26):** the repeated-payment OFFER — "you may pay this cost one or more times" / "up to three times" (the 5 Adversaries, Tranquil Frillback). Not an event ([CR#603.12a] makes the trigger a reflexive); what is missing is cost-side: `Pay`/`Cost` carry no repetition and `Repeated` takes only a definite count. [CR#702.56a] writes the offer in rules language. Two Effect-side follow-ons gated on the offer landing: narrow `reflexEncloseUse (Repeated _ _)` for a repeated cost payment per [CR#603.12a], and the reflexive seat's restating spelling.

- **Routed from workbench-cost-tags-and-paid-readbacks (close, 2026-08-26):**
  the READ side landed (`PaidCost`/`TimesPaid`, sorted by `PaidCostName`); four
  cost-DECLARATION residues follow it here. (1) The un-keyworded ADDITIONAL cost
  — "As an additional cost to cast this spell, you may …" [CR#118.8] — has no v2
  declaration row, so its 13 measured readback lines ("If this spell's
  additional cost was paid") cannot write; landing it also lands a fourth
  `PaidCostName` arm beside `TheAlternative`. (2) The keyword cost catalog:
  `keywordFacts` gained `Kicker` and `Multikicker` only, the two the bench
  needed; Madness, Prowl, Surge, Spectacle, Emerge, Freerunning, Mayhem, Sneak,
  Warp, Buyback, Dash, Evoke, Blitz, Awaken, Cleave, Impending, Harmonize,
  Replicate, Conspire, Casualty, Squad, Offspring, Gift and Compleated are all
  measured readback payers waiting on rows. (3) Entwine and escalate write NO
  readback anywhere in the corpus — all 19 entwine and 7 escalate lines are the
  keyword line plus reminder text — so the crate's `ModalCostRider` is a cost
  declaration on the modal clause and belongs here, not to a readback family.
  (4) Verrak, Warped Sengir — "if life was paid to activate it, you may pay that
  much life again" — is the corpus's one true life-payment readback; it sorts by
  no cost NAME and is anchored to an ABILITY rather than an object, so
  `PaidCost` cannot reach it and `PaysLife` is an event header, not a state read.
  (5) Karai, Future of the Foot — "if her sneak cost was paid **this turn**", the
  family's one turn-scoped payment read; `PaidCost` carries no window slot and
  the window belongs with the payment channel.

- **Routed from workbench-card-cost-letters (close, 2026-08-27):** the
  [CR#107.3k] boundary is unenforced — an activated ability's activation-cost X
  is independent of the card's own cost letters, but nothing refuses reading the
  card's letter from inside the ability. 3 supported cards (Chamber Sentry,
  Defenders of Humanity, Wren's Run Hydra); the fix belongs to the ability's own
  telescope, which is this bundle's machinery.

- **Routed from workbench-choice-c-chooser-positions (close, 2026-08-27):** the
  ADDITIONAL-COST chooser position and its missing row — a chooser announced
  from a spell's additional cost, one of the two positions the umbrella never
  named (10 cards over 7 positions, recounted from 5). Cost machinery, so it
  lands here.
