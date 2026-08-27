---
needs: []
---
# The ordinal cast's element binder, and the per-member event count inside `AggregateOver`

Routed from `workbench-event-zone-and-cast-provenance`'s §6 dispositions
(split 2026-08-26; see
`docs/tickets/planned/workbench-event-zone-and-cast-provenance.md`). The
splitter's disposition table names `workbench-amount-comparison-and-quantity`
as the target for both items below, but that ticket is
**`docs/tickets/done/workbench-amount-comparison-and-quantity.md` — closed**.
Done tickets are not plans, so both items mint here instead.

## The ordinal cast's residual 5 — a noun-phrase element binder over a turn's casts

From the split report: *"Half delivered, half misrouted."*
`GameEvent.NthOccurrence` (`Triggers.idr:348`) landed in the event-algebra
round and its own docstring gives the cast spelling verbatim — the 20
trigger-header lines ("your first spell") write today. The residual 5
(Maelstrom Nexus, Rain of Riches, Wild-Magic Sorcerer, The Twelfth Doctor,
Zimone) want a noun-phrase read — "the first spell you cast each turn" —
which the parent ticket itself described as:

> "An ELEMENT BINDER over a turn's casts — the same shape
> `workbench-amount-comparison-and-quantity`'s relativized per-element count
> wants — and the per-turn reset is a second unbuilt thing beside it."

The next-spell and first-spell keyword GRANTS ("the next spell you cast this
turn has cascade") wait on this binder. They are [CR#611.2f]'s regime, a
continuous effect that begins to apply when the player next puts an
appropriate spell on the stack; they are not the keyword row's own gap.

### The Once Upon a Time re-check

Also routed from the split report's §6 (originally from
`workbench-conditional-and-coordination`, closed): Once Upon a Time's
history identity read ("if this spell is the first spell you've cast this
game") is **dormant** — "neither built nor pinned; no rule makes it
meaningless and no buildable card pays it" (parent's own line). The split
report notes it is "worth one cheap re-check now that `NthOccurrence`
exists — 'the first spell you've cast this game' is an ordinal read, and the
ordinal wrapper landed after this item was recorded." **Assign the re-check
to whoever takes the ordinal binder** — it is not a round of its own.

## The per-member event count inside `AggregateOver`'s binder body

From the split report's §6: "Belongs with the element binder, same as the
ordinal residue." Thought Sponge's "the greatest number of cards an opponent
has drawn this turn" and the Windfall / Jace's Archivist cycle's "cards a
player discarded this way" need a per-member event subject inside a binder
body; the body admits an `Amount` but no per-member event subject exists
today. Route to this ticket together with the ordinal 5, per the split
report.

## Consumption boundary

`idris/src/Experimental/Phrase.idr` (`AggregateOver` and its binder body,
the element-binder machinery), `idris/src/Experimental/Events.idr`
(`GameEvent.NthOccurrence`, event-count readers the binder wraps),
`idris/src/Experimental/Cards.idr` bench, `idris/src/Experimental/Proofs*.idr`.
No Rust crate.

## Acceptance

- The ordinal cast's noun-phrase reading lands over the same element-binder
  shape as `AggregateOver`'s relativized per-element count, or ends in a
  written rule-backed verdict naming what blocks it; the per-turn reset is
  answered together or explicitly deferred with its own statement of what
  remains.
- Maelstrom Nexus, Rain of Riches, Wild-Magic Sorcerer, The Twelfth Doctor
  and Zimone are benched, or the residual blocker is named per-card.
- Once Upon a Time's history identity read is re-checked against the landed
  `NthOccurrence` ordinal wrapper and either benched or re-recorded as
  dormant with the current reason.
- `AggregateOver`'s binder body gains a per-member event subject, or the gap
  is re-recorded with the reason it does not fit this round; Thought Sponge
  and the Windfall/Jace's Archivist cycle bench or are named as the residual.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## Routed ledger items

- **Routed from workbench-anaphora-mentions-and-creation (split into five
  sub-tickets, 2026-08-27):** the plural read-back mention — "Each player
  scries N" is unspellable: `Each` mints a plural (`ManyOf`) player binding
  where the anaphor wants a single `Player` mention. Same per-member
  binder-read shape as this ticket's own `AggregateOver` item above; land
  together.

## As landed

Two of this ticket's four items rested on a premise that is wrong against the
code, and the correction is the round's main result: **`AggregateOver`'s
binder body already admits a per-member event subject.** The item that was
costed as unbuilt writes today.

**The per-member event count inside `AggregateOver` — DELIVERED, premise
corrected.** The ticket says "the body admits an `Amount` but no per-member
event subject exists today". It does: the domain binds one member at
`TheD`/`OneOf`, the body reads it back as `They`, and `EventCount`'s own
`who` slot takes a `Noun bs k`. Both named readings write with no grammar
change at all:

- `greatestCardsAnOpponentDrew : Amount []` —
  `AggregateOver MaxOf Opponent (eventCount CardDrawn They ThisTurn)`, and
  **Thought Sponge benches whole** (Flash, the entry counters at that count,
  and the death trigger's power-equal draw).
- `greatestCardsAPlayerDiscardedThisWay : Amount []` — the Windfall /
  Jace's Archivist cycle's read, at a `VerbedAct "Discard"` event and the
  `ThisWay` window, with the complement written because
  `bareLookbackOk (VerbedAct v) Player` is `not (actNamesPatient v)` and
  discarding names a patient.

**The ordinal cast's residual 5 — a written verdict, with the blockers
named.** Measured and confirmed at 5 supported cards: Maelstrom Nexus, Rain
of Riches, The Twelfth Doctor, Wild-Magic Sorcerer, and **Zimone, Infinite
Analyst** (the ticket wrote "Zimone" unqualified; that is the one — "The
first spell you cast with {X} in its mana cost each turn costs {1} less").
Three separate things block it, and only the first is what the ticket
described:

1. **The read cannot be `GameEvent`-keyed.** `Experimental.Triggers` imports
   `Experimental.Phrase`, so a `Noun` row can never take a `GameEvent`, and
   `NthOccurrence` — the landed ordinal wrapper — is a `GameEvent`
   constructor. A noun-phrase ordinal read must be keyed by `EventName` plus
   a subject and complement, the way `HappenedTo` and `EventCount` already
   are. `Ordinal` itself is in `Words` and so is available at the noun sort;
   the wrapper is not.
2. **There is no recurring window.** `Lookback` is
   `ThisTurn | ThisCombat | LastTurn | ThisGame | ThisWay` — every arm names
   one span looked back over. "each turn" is a window that RESETS, and that
   is the ticket's own "second unbuilt thing beside it". All five lines write
   it, so it is not optional scope: no member of the family is writable
   without it.
3. **The grant regime is a third thing.** The five landed lines are static
   abilities [CR#611.3]; the next-spell GRANTS the ticket routes behind this
   binder ("the next spell you cast this turn has cascade") are
   [CR#611.2f]'s regime instead — an effect that begins to apply when the
   player next puts an appropriate spell on the stack. Two regimes, one
   phrase shape.

Not minted here: an `EventName`-keyed ordinal element row plus a recurring
window sort plus the continuous-effect regime is a round of its own, and
building the noun without the window would write no card. **Remainder for
the coordinator.**

**Once Upon a Time — re-checked, still dormant, and no longer independent.**
"If this spell is the first spell you've cast this game" is an IDENTITY test
at the condition seat: it asks whether a named object IS the ordinal-selected
member of a cast history, not whether an ordinal-selected event happened.
`NthOccurrence` answers the second question and only at the event sort, so
its landing does not reach this. What would reach it is exactly the ordinal
ELEMENT read item 1 above wants, written as a `Matches` complement — so this
is no longer "dormant, nothing would build it" but "waits on the same
element read", and it should travel with that remainder rather than being
re-recorded separately. No pin: nothing about the read is rules-impossible.

**Routed ledger item — the plural read-back mention: measured, blocker
named.** "Each player scries N" measures at ONE supported card, Eager
Construct ("each player may scry 1"); no supported line writes the
unhedged form. Recorded as pins rather than prose:
`eachPlayerBindsNoSingular : countOnes Player (nomIntro (Each AnyPlayer)) = 0`
and `eachPlayerBindsAGroup : countManys Player … = 1`. `playerScries` and
`playerSurveils` both gate on `countOnes Player (nomIntro agent) = 1`,
because [CR#701.22a] and [CR#701.25a] each name ONE player and read that
player's own library. The fix is the per-member binder shape this round just
confirmed works at the AMOUNT sort, lifted to the EFFECT sort so a
distributive agent's body is typed at a context where the member binds
`TheD`/`OneOf`. Not taken: that changes how every `Each`-headed body is
typed. **Remainder for the coordinator**, and it did NOT "land together"
with the `AggregateOver` item, because that item needed nothing.
