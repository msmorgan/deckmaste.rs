# cost-1: the alternative costs, the keyword cost catalog and the payment channel

Sub-round 1 of [workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md)
(the umbrella — authoritative for measurements, corrections and acceptance).
Owns: the five populations the alternative-cost row refuses (generic grants
14; non-mana declined costs 11; the 23 pronoun-tail lines are `MayPlay`'s —
record; Invigorate's `costActionOk` cell; the 5 commander-gated free spells
and the `CommanderD` scope — note description-1 landed `HasCardDesignation`,
probe first); the FREE-CAST family (296 non-reminder lines — the licence to
cast a DIFFERENT card for nothing, distinct from the landed self phrasing);
the keyword ALTERNATIVE-COST catalog (the measured readback payers waiting on
`keywordFacts` cost rows — Madness, Prowl, Surge, Spectacle, Freerunning,
Dash, Evoke, Blitz, etc.; the shared Surge/Prowl/Freerunning condition
template written once; Fall of the Titans and its ten Surge siblings are the
payment test; reminder lines stay un-benched with the count recorded); the
where-clause binding for letter-valued parameters (annihilator X, mobilize X,
monstrosity X as a linked value [CR#701.37c]); and the routed payment-channel
items: the un-keyworded ADDITIONAL cost [CR#118.8] (+ its fourth
`PaidCostName` arm, its 13 readback lines, Burn at the Stake, the
additional-cost CHOOSER position, the Grandeur discard-cost shape); the
repeated-payment OFFER ([CR#702.56a]; the Adversaries) with its two gated
Effect-side follow-ons; entwine/escalate as a cost declaration on the modal
clause (zero readbacks — the `ModalCostRider` shape); Verrak's life-payment
readback (no cost name, ability-anchored); Karai's turn-scoped payment window;
Yidaro's cycling `PaysCost` count; the two carried corrections (Fall of the
Titans/Surge reminder; the zero "cast another spell this turn" gate — pin the
zeros per acceptance).

Standard constraints apply; re-measure every count.

## As landed (2026-08-28)

Everything below was re-measured on `data/derived/cards.jsonl`, supported
cards only, reminder parentheticals stripped before matching. Where a
count differs from the umbrella's, the re-measure is authoritative and
the correction is written into the grammar beside the row it belongs to.

### The keyword alternative-cost catalog

Seven `keywordFacts` rows: **Madness, Prowl, Surge, Spectacle,
Freerunning, Sneak, Mayhem**, all `CostParam`. Keyword lines / readback
lines: Madness 61/4, Sneak 28/4, Mayhem 15/1, Freerunning 12/1, Prowl
10/4, Surge 11/3, Spectacle 11/2. Each is bought by the READBACK channel
(`PaidCost` reaches a word through `keywordCosts`), which is emerge's
precedent; prowl and freerunning leave the "deliberately absent" list for
that reason and the note is rewritten.

The shared template is written ONCE, over four words rather than three:
[CR#702.76a], [CR#702.117a], [CR#702.137a] and [CR#702.173a] are each "a
static ability that functions on the stack" reading "You may pay [cost]
rather than pay this spell's mana cost if [something happened this
turn]", and all four route through [CR#601.2b,601.2f..601.2h]. Spectacle
is the fourth member the umbrella grouped elsewhere. Madness, mayhem and
sneak each carry their own zone or timing and template alone.

**Reminder text:** 1005 supported lines carry an alternative-cost
reminder parenthetical over 50 leading words (Flashback 202, Morph 150,
Suspend 55, ... Impending 5); ZERO survive reminder-stripping, so none is
bench-payable. Recorded in the catalog comment.

**Correction:** the umbrella's list of "measured readback payers" is
wrong. Twenty of those words — escape, foretell, bestow, disguise,
mutate, overload, disturb, dash, evoke, blitz, cleave, harmonize,
impending, awaken, buyback, casualty, squad, offspring, gift, replicate —
write ZERO readbacks. They print keyword lines and wait on a
keyword-line consumer, not on this channel.

**Correction:** Fall of the Titans was never blocked by its Surge
reminder text. Its {X} is the printed mana cost's ({X}{X}{R}), so
`costLetters` opens the letter before any line is read and [CR#107.3a]
gives the surge cost's X and the mana cost's X one announced value
[CR#107.3i]. It benches whole; `abIntro (KeywordAbility _ _) = bs`
stands.

Benches: **Fall of the Titans** (whole), Latchkey Faerie (whole), Tyrant
of Valakut (whole), Rafter Demon (whole).

### The un-keyworded additional cost [CR#118.8]

`AddedCost : Cost bs -> Bool -> StaticEffect bs` beside `AltCost`, with
`AddedPayment` refusing an on-battlefield cost. 315 supported lines write
"as an additional cost"; **308** are of this spell (**260** mandatory,
**48** offered, which is the `offered` slot [CR#118.8b]) and **7** name a
class of spells, which is the generic grants' subject gap.

`PaidCostName` gains **`TheAdditional`**, its fourth arm, for the **12**
readback lines ("if this spell's additional cost was paid"; the umbrella
said 13 — Katara writes "her additional cost", which is the twelfth).

The **additional-cost CHOOSER position** lands: `staticChoiceDelta
(AddedCost c _) = costChoiceDelta c`, [CR#607.2d]'s link at the cost.
**6** self lines announce a chooser from an additional cost.

Benches: **Caller of the Hunt** (whole — chooser position plus the
defining read), Voltage Surge's declaration, Requiting Hex's read, **Burn
at the Stake**'s any-number tap cost.

Remainder: Burn at the Stake's damage line reads the stamp the COST's
action left ("creatures tapped this way"). `staticChoiceDelta` has `bs`
at multiplicity 0 and `effDelta` needs it un-erased, so only the chooser
crosses the ability boundary. 13 more lines want the same read (an
any-number additional cost plus a for-each reduction counting it —
Dargo, Gorex, Explosive Singularity). Recorded on `AddedCost`.

### The repeated-payment offer

`PayTimes = PaidOnce | AnyNumberOfTimes | UpToTimes Nat` as a slot on
`Pay` [CR#702.56a]. **6** supported cards: the five Adversaries at "any
number of times", Tranquil Frillback at "up to three times". A repeating
payment leaves `RepeatCount`, which is what "that many" reads.

Both gated follow-ons land: `reflexEncloseUse (Repeated _ (Pay _ _ _))`
and its `May`-wrapped form narrow to `EncReflexive` per [CR#603.12a]'s
named exception, and the reflexive seat's restating spelling ("When you
pay this cost one or more times") is written on `PayTimes`.

Benches: Tainted Adversary's offer and reflexive trigger, Tranquil
Frillback's capped offer.

### The keyword parameter's where-clause

`abLetterDelta` opens the letter a granted keyword's `ParamNumber`
carries, so `Define` can close it. **4** supported lines: Ulamog's
annihilator X, Fumiko's bushido X, mobilize X on Avenger of the Fallen
and Infantry Shield. `Macros.gains`' duration is retyped at
`staticIntro (Gains n a)`.

Monstrosity is NOT among them: its five "{X}{X}{G}: Monstrosity X" lines
take X from the activation cost and [CR#701.37c] makes the other
abilities read the value X had as the permanent became monstrous — a
linked value, and no card writes "where X is" beside it. Two monstrosity
lines DO write a where-clause (Clay Golem's "where X is the result",
Maester Seymour's counter count) and are keyword-ACTION arguments, not
keyword parameters.

Bench: Fumiko the Lowblood's first line.

### The free cast of another card

`PlayPayment = ItsOwnCost | WithoutPaying` as a slot on `MayPlay`.
Measured: **312** non-reminder lines write "without paying its/their mana
cost"; **16** are the landed self phrasing [CR#118.9] and the remaining
**296** license a card the clause has named — the umbrella's count
confirmed exactly.

The third arm (a written alternative cost after the permission — the 23
pronoun-tail lines) is recorded on the type, deliberately not minted.

Benches: **Memory Plunder** (whole), **Omniscience** (whole).

### The five populations

- **Commander-gated free spells, 5 lines — WRITTEN.** `HasCardDesignation
  CommanderD` [CR#903.3] landed in description-1 and is exactly the
  predicate that was missing. Bench: Deflecting Swat's first line.
- **Invigorate, 1 line — WRITTEN, and the recorded blocker was stale.**
  `costActionOk (ChangeLife _ _)` already reads `True`, so the card wants
  nothing minted. Bench: Invigorate whole.
- **Generic grants, 13 lines (not 14) — REFUSED, recorded on `AltCost`.**
  They price a described CLASS of spells; the row has no subject slot.
- **Non-mana declined costs, 11 lines — REFUSED, recorded on `AltCost`.**
  They decline a cost named by a keyword (equip, cycling, echo, power-up,
  crew) or a pip inside one; the declined side wants to be a value.
- **Pronoun-tail lines, 23 — REFUSED, recorded on `AltCost`.** They are
  `MayPlay`'s, at `PlayPayment`'s unminted cost-carrying arm.

### The routed payment-channel items

- **Entwine and escalate:** `keywordFacts` rows, `CostParam`, `AtCasting`
  ([CR#702.42a], [CR#702.120a]). **32** entwine and **9** escalate
  keyword lines (the umbrella said 19 and 7); **zero** readbacks either
  side, and that zero is preserved as a measurement. Bench: **Borrowed
  Malevolence** whole. Recorded remainders: nothing gates the word to a
  modal spell [CR#700.2], and escalate's per-mode multiplier has no term.
- **Karai's turn-scoped window — WRITTEN.** `PaidCost` gains
  `window : Maybe Lookback`; 1 line uses it, 299 write `Nothing`. Bench:
  `karaiSneakPaidThisTurn`.
- **Grandeur — WRITTEN, and needed nothing minted.** **7** lines; the
  cost is `Named (PrintedName ...)` [CR#201.1] beside `OtherThan This`.
  Bench: `grandeurDiscardCost`.
- **Verrak — REFUSED, blocker recorded in full.** Three faults at once:
  no cost NAME to sort by, an ABILITY anchor where `PaidCost` takes an
  object, and an AMOUNT read where `TimesPaid` counts payments.
- **Yidaro — REFUSED, blocker recorded in full.** Cycling is not a
  keyword action [CR#702.29c], so the count is over cost PAYMENTS; what
  refuses it is that the count runs over payment events across every copy
  of a named card this game, where `TimesPaid` reads one object's casting
  state [CR#118.10] and `EventCount`'s complement cannot name a keyword's
  cost.

### The measured zeros

- **"If you've cast another spell this turn, you may pay {1}{U} rather
  than pay this spell's mana cost" — zero, RECORDED not pinned.**
  Confirmed zero; the condition appears only inside surge's own rule
  [CR#702.117a] and in 4 cost-reduction lines. [CR#118.9] puts no bound
  on what a card may condition an alternative cost on, so nothing refuses
  the sentence and the pin doctrine keeps it a recorded zero.
- **Entwine/escalate readbacks — zero, RECORDED** beside the rows.

### Pins

Two new refusals in `ProofsF.idr`, both rules-impossibility:
`badAddedCostTapSymbol` and `badAddedCostLoyaltySymbol` — an additional
cost is paid at [CR#601.2f..601.2h] while the spell is on the stack
[CR#601.2b], where [CR#107.5]'s "{T}" taps a permanent and [CR#606.2,606.3]'s
loyalty symbol is a permanent's activation cost.

### Gates

`idris/scripts/build` 23/23, 0 errors, 0 warnings.
`cargo xtask cite check --list-noncompliant` 0; `cargo xtask cite check`
0 stale; `cite bless` registered 6 new rules (surge, freerunning, mayhem
x2 and sneak x2); the round diff was audited with `cargo xtask cite audit
--diff` over 77 citation sites and every rule read against its claim (two
were corrected in the pass: a malformed subrule range, and a chosen-name
rule cited where a printed-name rule was meant).

### Explicit remainders

1. The cost-action stamp across the ability boundary (Burn at the Stake's
   damage line and 13 for-each reduction lines).
2. `PlayPayment`'s written-alternative arm (the 23 pronoun-tail lines).
3. `AltCost`'s subject slot (13 generic grants) and its value-valued
   declined cost (11 non-mana lines) — both named on the row.
4. The modal linkage for entwine/escalate and escalate's per-mode
   multiplier.
5. Verrak's ability-anchored life readback; Yidaro's cross-copy payment
   count.
