---
needs: []
---
# The chosen counter kind — a closed menu over kinds

Routed from `workbench-counter-kind-holder-source` (close, 2026-08-26).
Measured there: 21 distinct supported lines over 33 cards write a counter
whose KIND is chosen from a printed closed menu — 10 entry-side ("enters
with your choice of a +1/+1, first strike, or vigilance counter on it" —
riot's reminder text, the two-kind creature run, Denry Klin) and the rest
plain puts. Two menu spellings ("your choice of a X, a Y, or a Z counter";
Aragorn's "from among X, Y, Z, and W"), plus Bribe Taker's variant whose
second arm is a kind read off the board.

Neither existing seat reaches it: `EntersWithCounters` takes one
`CounterKind`; `EntersChoice`/`ChoiceDomain` offer only open-ended domains
(`QualitySort` has no counter-kind member), where these lines write a
closed menu. The shape question is whether the menu is a `ChoiceDomain`
extension (a counter-kind quality sort with a list domain) or a dedicated
menu slot on the counter rows — weigh against the chooser-position doctrine
in `workbench-choice-chosen-and-ascription` (which owns chooser POSITIONS;
this ticket owns only the kind menu).

This is the only remaining blocker for Denry Klin whole
(`denryKlinSameKinds` benches its trigger already).

## Consumption boundary

`idris/src/Experimental/Words.idr` (if the menu needs vocabulary),
`Phrase.idr`/`Effect.idr` (the seat), `Cards.idr` bench, `Proofs*.idr`.
No Rust crate.

## Acceptance

- Denry Klin benches whole; the menu is closed by design (a written list),
  not per-card.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Standard constraints applied. `idris/scripts/build` PASS from a clean
`build/` — 23/23 modules, 0 errors, 0 warnings; both new pins elaborate as
refusals and one existing pin's witness changed shape with the gate it
pins (`badPutPoisonOnCreature`, `Refl` → `Oh`), so none passes silently.

### The shape ruling

**The menu is the kind slot's content, not another row.**

> The two candidate shapes were a `ChoiceDomain` extension and a dedicated
> menu slot on the counter rows. Neither is what the corpus writes. A
> `ChoiceDomain` extension is the chooser's machinery — a `QualitySort`
> member, a binding, a mention and a read — and NO line in this family
> reads the picked kind back, so every part of that apparatus would be
> paid for and left unused. It would also cross the fence:
> `workbench-choice-chosen-and-ascription` owns chooser positions, and a
> counter-kind `QualitySort` member is a chooser position.
>
> What the lines write instead is one counter noun phrase whose KIND WORD
> is replaced by a written range. The verb, the amount, the recipient and
> every gate are the sentence's already: "put your choice of a +1/+1,
> first strike, or vigilance counter on it" is `PutCounters`' sentence
> with a menu where the word goes. So the slot widens and the row does
> not double — the same move `RemoveCounters`' `Maybe CounterKind` already
> makes on the same axis (whether the sentence NAMES a kind), and the same
> shape `NameSource` (`PrintedName`/`ChosenName`) and `ProducedMana`
> (`Runs`/`OfChosenColor`) take at their own slots. `PutCountersOfThoseKinds`
> is the contrast that shows where a row IS earned: there no kind is
> given at all, which is a different reading rather than a different
> spelling of the same slot.
>
> No chooser slot and no domain slot, both measured zeros: every line in
> the family writes "your" or writes no chooser at all, and [CR#109.5]
> fixes the first while [CR#608.2c] fixes the second. [CR#614.12a] fixes
> WHEN for the entry side — the pick is made before the permanent enters —
> so the entry menu needs no announcement either. Nothing here touches the
> cross-ability channel, so the forward-obligation ruling recorded on
> `workbench-choice-chosen-and-ascription` does not bite: this choice is
> made and spent inside one noun phrase.

### The vocabulary and the seats

`Words.idr`, beside `CounterKind`/`CounterKindNamed`:

```idris
data CounterKindSource : Type where
  PrintedKind : CounterKind -> CounterKindSource
  ChosenKind : (menu : List CounterKind) ->
               {auto 0 ne : NonEmpty menu} -> CounterKindSource

counterSourceScope : CounterKindSource -> Kind -> Bool
CounterSourceScope : CounterKindSource -> Kind -> Type
```

Two seats take it: `StaticEffect.EntersWithCounters`' `kind` and
`Effect.PutCounters`' `kind`, the latter with its scope gate restated over
the source (`counterScope kind = Object` → `CounterSourceScope kind Object`,
so every arm has to name a counter the recipient can hold [CR#122.1]).
`Macros.entersWithCounters`/`entersWithAdditionalCounters`/
`entersWithFewerCounters` keep their `CounterKind` parameter and wrap, so no
macro call site moved; the 77 direct constructor sites took `PrintedKind`.

Seats deliberately NOT widened, each with its count: `ExceptEntersWithCounters`
(copy entry) and `GetsCounters`/`GetsCountersOfThoseKinds`/`LosesCounters`
(player side) and `MoveCounters` and the move rider's `MkCounterRider` are
measured zeros; `RemoveCounters` has exactly one carrier (Ion Storm) and waits
for a second, as `BecomesAlso`'s chosen-type payload waits for Navigator's
Compass's.

Refusals, both pinned in `ProofsG`: an EMPTY menu (`badEmptyCounterMenu` —
[CR#122.1] places a counter of some name and an empty list names none) and a
menu with an arm at the wrong scope (`badMixedScopeCounterMenu`, the menu case
of `badPutPoisonOnCreature`). Tolerated overgeneration, named at its zero: a
ONE-arm menu and a REPEATED arm. Both are performable and both say what
`PrintedKind` says — [CR#122.1] makes counters with the same name
interchangeable — so neither is a rules impossibility, and no card writes
either. This follows `Or`'s gate, which refuses the empty coordination and
tolerates the singleton, rather than `chapterMarksOk`'s, whose distinctness
demand comes from [CR#107.15b] naming one ability twice.

### Family, re-measured

Measured this round over supported faces of `data/derived/cards.jsonl`, by
the operation rather than by one phrase. **21 supported lines over 21 cards**
write a counter whose kind comes from a printed closed menu:

- **Entry side, 8 lines** — Boot Nipper, Denry Klin, Ferocious Tigorilla,
  Flycatcher Giraffid, Grimdancer, Helica Glider, Wingfold Pteron, and
  Tizerus Charger (which spells the same rider on ESCAPE).
- **Put side, 13 lines** — Aragorn, Assaultron Dominator, Bribe Taker,
  Dwarven Armorer, Elspeth Conquers Death (III), Elspeth Resplendent (+1),
  Inspirit, Invoke the Ancients, Me the Immortal, Owen Grady, T-45 Power
  Armor, The Night of the Doctor (II), Vivien Monsters' Advocate.

Four spellings, one node: the shared determiner ("a +1/+1, first strike, or
vigilance counter"), the determiner repeated per arm ("a deathtouch counter
or a lifelink counter"), the postposed list ("a counter from among X, Y, Z,
and W") and the disjunction with no chooser written at all (Dwarven Armorer,
Elspeth Conquers Death).

**The routed count was 21 lines over 33 cards, and the card count was
riot's.** [CR#702.136a] states riot as "You may have this permanent enter
with an additional +1/+1 counter on it. If you don't, it gains haste" — a
MAY-clause with an otherwise, choosing between a counter and a keyword gain,
not between kinds. The words the parent round measured ("enters with your
choice of a +1/+1 counter or haste") are the keyword's REMINDER text, which
is not template surface at all. Riot's ~14 carriers are therefore out of this
family and no part of them is written here; the line riot itself would want is
a modal entry replacement and is nobody's ticket yet.

### Cards and phrases benched

- **Denry Klin, Editor in Chief** (whole) — `denryKlin`. The entry line is
  `EntersWithCounters` with `ChosenKind [+1/+1, FirstStrike, Vigilance]` at
  `Fresh`; the second line is `denryKlinSameKinds`, landed last round. The
  ticket's deliverable.
- **Helica Glider** (whole) — `helicaGlider`. The two-arm entry menu with the
  determiner repeated per arm, and the cheapest whole card in the family.
- **Me, the Immortal's combat trigger** — `meTheImmortalCounterMenu`. The put
  seat, four arms, shared determiner. Its other two lines (counters surviving
  a zone change, and a graveyard cast permission) have no rows and are not
  this ticket's.

### Ledger

- **Inspirit, Flagship Vessel** — "your choice of a +1/+1 counter or two
  charge counters". The arms carry DIFFERENT AMOUNTS, so what is coordinated
  is two whole counter noun phrases, not two kind words; the row's one `amt`
  slot cannot distribute into the menu. This is the counter seat's instance
  of the coordination ledger item `SetsType`/`BecomesAlso` already carry, and
  the family's only line that needs it. 1 line.
- **Grimdancer** — "enters with your choice of two DIFFERENT counters on it
  from among menace, deathtouch, and lifelink": a plural pick over the menu,
  with a distinctness demand between the picks. Neighbour of the plural
  chosen-quality read routed to `workbench-choice-chosen-and-ascription`; 1
  line, and it wants the amount slot and the menu to interact.
- **Ion Storm** — "Remove a +1/+1 counter or a charge counter from a permanent
  you control" as an activation cost: the menu at the REMOVE seat, whose slot
  is `Maybe CounterKind`. The seat's only carrier; widen on the second.
- **Bribe Taker** — "you may put your choice of a +1/+1 counter or a counter
  of that kind on this creature", under "for each kind of counter on permanents
  you control". One arm is a listed kind and the other is a kind READ off the
  board, so it needs the counter-kind read this round deliberately did not buy.
- **The board-read kind chooser, ~13 lines** — "choose a counter on target
  permanent … put a counter of that kind on it" and "choose a kind of counter
  on a creature you control …" (Animation Module, Aven Courier, Clockspinning,
  Contractual Safeguard, The Caves of Androzani, Dramatist's Puppet, Exotic
  Pets, Ichormoon Gauntlet, Maulfist Revolutionary, Powerful Broker, Quarry
  Hauler, Skyship Plunderer, Blue Loyal Raptor). A chooser with an OPEN domain
  read off the board plus a chosen-kind read — the chooser-position family's,
  not this slot's, and it is the largest thing behind the counter-kind sort.
- **Crystalline Giant** — "choose a kind of counter AT RANDOM that this
  creature doesn't have on it from among [ten kinds]. Put a counter of that
  kind on this creature." The printed menu as a CHOOSER's domain, with an
  exclusion filter and an at-random mode, plus the read. The menu vocabulary
  minted here is what that domain would carry; everything else is the chooser
  family's.
- **Aragorn, Company Leader and Elspeth Resplendent** — the postposed-list
  spelling's two carriers, both blocked elsewhere: Aragorn on the Ring-tempts
  trigger and its Ring-bearer condition, Elspeth on coordinating a fixed
  counter with a menu counter in one clause (the same coordination Inspirit
  wants).
- **Tizerus Charger** — the entry-side menu spelled on ESCAPE ("this creature
  escapes with your choice of …"). The menu writes; the escape entry frame
  does not.
