---
needs: []
---
# "Up to [n]" as an Amount — the ceiling read off a described set

Routed from `workbench-payment-event-residues` (close, 2026-08-26). Shah of
Naar Isle's body "draw up to three cards" does not write: `UpToOf` is a
`Quantity` over a described set's mention, and `Draw`'s count is an
`Amount`, which has no ceiling arm. Decide whether the ceiling becomes an
`Amount` row (the drawer chooses a number up to the bound, [CR#608.2d]) or
the draw takes a `Quantity` — measure the "draw/mill/discard up to N" verb
surface in local card data first and shape for it, not one card.

## Consumption boundary

`idris/src/Experimental/Phrase.idr` (`Amount`/`Quantity`), `Effect.idr`
(the consuming verbs), `Cards.idr` bench, `Proofs*.idr`. No Rust crate.

## Acceptance

- Shah of Naar Isle benches whole; the shape is measured, not per-card.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

The ceiling became an `Amount` row, `UpTo`. The verbs keep their `Amount`
count slots.

### The surface, measured

Over the supported corpus (`data/derived/cards.jsonl`, `supported: true`),
"up to" appears on 1,492 cards. Almost all of them ceiling a described SET —
"up to two target creatures", "up to four cards from your graveyard", "up to
one target artifact" — which is what `Quantity`'s `UpToOf` already spells.

The ceiling that has no set to choose from, only a number, is a much smaller
family, and it is spread across THREE verbs that share one slot type:

- `Draw` — 8 cards: Arcane Denial, Diminishing Returns, Fatal Lore,
  Indentured Djinn, Shah of Naar Isle, Temporary Truce, Trade Secrets, Truce.
- `PutCounters` — 5: the four Clockwork creatures ("Put up to X +1/+0
  counters on this creature") and Terra, Magical Adept ("put up to three lore
  counters on it").
- `RemoveCounters` — 7: Glissa Sunslayer, Gremlin Mine, Heartless Act, Hex
  Parasite, Price of Betrayal, Render Inert, Sensational Spider-Man.

Neighbours that look like the family and are not: "discard up to two cards"
(12) and "cast up to two spells from among them" (11) both choose WHICH, so
they are set ceilings on the `Quantity` side. "Play up to two additional
lands" (3) and "can block up to two additional creatures" (1) ceiling a
PERMISSION, not a count slot; neither seat is an `Amount`. No supported card
writes "mill up to", "scry up to", "create up to", "deals up to" or "gains up
to [n] life".

Every measured bound is a numeral or X.

### The shape, and why

Three verbs, one slot type: an `Amount` row lands once and every count slot
inherits it, where widening the verbs to `Quantity` would be three signature
changes for one reading — and would carry `Range`'s floor along, so "draw two
to four cards" would become writable, which no card prints. So:

```idris
UpTo : (bound : Amount bs) -> Amount bs
```

[CR#608.2d] is the row's rule: an effect's own choices — the ones not already
made as the spell was cast — are announced while applying the effect, and an
illegal or impossible option can't be chosen. That is exactly a ceiling: the
number is not in the game state before resolution and it is not free above the
bound.

**The chooser follows the rules, and needs no slot.** [CR#121.2b] ("if an
effect offers the player a choice to draw multiple cards, the affected player
can't choose to do so") and [CR#121.3] ("an effect offers that player the
choice to draw a card, that player can choose to do so") both put the ceiling
draw's choice with the DRAWING player, not with the ability's controller. The
clause already names that player — `Draw`'s subject, `PutCounters`' actor — so
a chooser slot on the row would restate what the seat above it fixes, and
would let the two disagree. Shah of Naar Isle is the case that makes the
difference visible: "each opponent may draw up to three cards" hands both the
offer and the number to the opponent, and the term says so without a third
party.

The bound slot is ungated. "Put up to X +1/+0 counters" prints, and no rule
refuses a bound the game state supplies.

### The tables

`amtDelta`, `amtIntro`, `amtPlur`, `writtenBound` and `readAmount` each gained
their row.

- `amtDelta`/`amtIntro` pass through the bound's own: "up to X" introduces its
  letter like any other written X, exactly as `quantDelta (UpToOf a)` does.
- `amtPlur (UpTo b) = amtPlur b`. Agreement follows the WRITTEN bound, which
  is what prints — "up to three cards", "up to one card" — not the number the
  ceiling resolves to.
- `writtenBound (UpTo _) = False`. The bound is written; the number the
  ceiling stands for is not.
- `readAmount (UpTo _) = False`, which is the round's one new refusal: a
  ceiling is announced [CR#608.2d], not read off the game state, so it cannot
  be a comparison's subject. Pinned as `badCompareCeilingSubject`, beside
  `badCompareLiteralSubject`'s neighbouring reason.
- `boundEq` gained NO row, deliberately. It is the smallest honest referent
  relation, and its catch-all already answers `False`: two ceilings with equal
  bounds are not provably the same number, since each is announced on its own.

Tolerated overgeneration, named at its zero: a ceiling inside a ceiling
("up to up to three") and a ceiling in an amount position nothing announces
(`Compare`'s bound, `LibrarySlice`'s depth, `Times`' multiplicand). Neither is
written English; no rule refuses either, and `CompareAmt`'s bound slot is
already on record as open to any amount by decision.

### Landed

- `Experimental/Phrase.idr` — `UpTo` on `Amount` and its five table rows.
- `Experimental/Cards.idr` — `shahOfNaarIsleHeader` becomes `shahOfNaarIsle`,
  the whole card.
- `Experimental/ProofsG.idr` — `badCompareCeilingSubject`.

### Carrier status

- **Shah of Naar Isle** benches WHOLE, as a `Card`: trample, echo {0}, and the
  triggered ability entire —
  `triggered When (PaysCost Nothing Paid thisCreature "Echo")
   (may (Each Opponent) (Draw (Those PlayerW) (UpTo (Lit 3))))`.
  The header the parent ticket left is now the card's third ability.

One thing the bench had to spell that the card does not: `Each Opponent` binds
plurally, so the drawer inside the `May` body is the anaphor `Those PlayerW`
where the printed sentence simply shares its subject across "may" and "draw".
The co-reference is what the term needs to make provable; dropping the
repeated subject is the spelling layer's business.

### Ledger

- **The offeree's shared subject** — `May (Just d) body` types its body in
  `nomIntro d`, so a non-`You` decider must be re-mentioned by anaphor
  ("those players draw"), where the printed clause shares one subject. Not a
  semantic gap; a spelling-seat item for whoever owns `May`'s surface.
- **"Discard up to two cards, then draw that many cards"** — twelve carriers
  (Cathartic Pyre, Daretti, Fable of the Mirror-Breaker, Greasewrench Goblin,
  Jaya Ballard, Joshua, Kinetic Augur, Mind Bomb, Mishra's Command, Obsessive
  Astronomer, Sokka, Tersa Lightshatter). The ceiling there is a set ceiling
  (`UpToOf` over the discarded cards), and the tail's "that many" wants the
  number actually discarded — a magnitude the discard clause must announce.
  Not this row's business.
- **Ceilinged permissions** — "play up to two additional lands this turn"
  (Journey of Discovery, March of Reckless Joy, Summer Bloom) and "can block
  up to two additional creatures" (Yare). A ceiling on a permission's count,
  on seats that are not `Amount` slots.
- **Temporary Truce / Truce's tail** — "For each card less than two a player
  draws this way": a shortfall read against the ceiling. It reads the number
  NOT chosen, which nothing announces today.

### Gates

- `idris/scripts/build` — 23/23, 0 errors, 0 warnings (from a clean `build/`).
- `cargo xtask cite check --list-noncompliant` — 0 non-compliant.
- `cargo xtask cite check` — 17,275 citations, 0 stale; [CR#121.2b] blessed
  and read against its claim.
- `jj diff --git | cargo xtask cite audit --diff` — 6 sites, each read against
  its rule.
