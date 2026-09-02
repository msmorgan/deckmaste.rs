---
needs: []
---
# Write the transform verb, and settle the face layouts the card round left out

`docs/tickets/done/workbench-multiface-cards.md` gave the card record its faces,
its five layouts and its two missing printed boxes, and closed with a ledger of
named blockers. Five of them are one region — what a face *does* and which
multi-face shapes the layout catalog still refuses — and nothing owns them.

## 1. Transform as a verb — the largest of the five

Quoted from that ledger:

> `Action::Transform`'s counterpart — a `transform`/`convert` effect and its
> trigger event [CR#701.27,701.28]; every non-Siege TDFC's front face writes one
> in printed text.

The Siege witness that round benched does not need it, which is exactly why the
verb went unbuilt. Two carriers are waiting on it and named:

- **Arlinn Kord // Arlinn, Embraced by the Moon** — box law probed
  (`planeswalkerBackWithoutLoyaltyOk`); the whole card additionally wants "up to
  one target", an emblem body and a `Wolf` subtype row.
- **Garruk Relentless // Garruk, the Veil-Cursed** — same box law, same block;
  the front's state trigger writes the verb. Also wants a `Garruk` subtype row.

Note that the verb vocabulary is now open
(`docs/tickets/done/workbench-verb-labels-open.md`): a keyword action is a data
row plus a macro. Check whether the transform verb is that shape before
designing a constructor.

## 2. Meld

> out of scope, recorded in the `Card` docstring — a two-card object;
> [CR#712.4b] and [CR#712.21] make it a different shape from a face layout.

So this is not a sixth layout constructor by default. Decide what it is.

**RULED (user, 2026-08-27): the melded (reverse) face is DUPLICATED on each
of the two cards** — the shape JSON representations typically use. Not a
layout constructor, not a two-card object type, no cross-card reference for
now. The duplication is acknowledged tech debt: fix later or add a lint
asserting the two copies are really identical (the lint is the cheap option
and may land with the round that first benches a meld pair).

## 3. Rooms — the unlock designations and the special action

`SplitCard` already admits a shared type line ([CR#709.5]). What is missing:

> the "left/right half unlocked" designations [CR#709.5c] and the unlock special
> action [CR#709.5e].

## 4. Fuse

> a keyword row; chose Wax // Wane over Wear // Tear to avoid it — [CR#702.102].

## 5. The Adventure exile-and-recast rider

> the "then exile this card, you may cast the creature later from exile" rider
> is reminder text on the printed card and has no row — [CR#715.3d].

Its reminder-text status is the argument for leaving it out; make that the
verdict or overturn it, but do not leave it unstated a second time.

## Neighbour, so it is not re-measured here

The Adventure/Siege re-measure the card round asked for is routed to
[workbench-description-and-player-sorted-reads](workbench-description-and-player-sorted-reads.md),
which owns the eleven-card reading in question.

## Consumption boundary

`idris/src/Experimental.idr` (the effect row and its trigger event, the
designations, the special action), `idris/src/Experimental/Words.idr` (the
layout catalog, `Designation`, any label row the verb needs),
`idris/src/Experimental/Events.idr` for the event side, the pin modules
`idris/src/Experimental/Proofs*.idr`, and the evidence bench
`idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- The transform verb exists with its event, and both named planeswalker
  carriers are either benched or reduced to blockers that are not the verb.
- Meld, Rooms' unlock pair, fuse and the Adventure rider each end with a landed
  row or a written verdict naming its rule — none is left unmentioned.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

- **Routed from workbench-verb-label-residues (close, 2026-08-26):** manifest [CR#701.40a] — no face-down `TokenRider` arm, and nothing lists a face-down permanent's characteristics [CR#708.2]. A face/permanent-characteristics gap; manifest dread's STRUCTURE already composes from the landed look-partition, so this is its single blocker (Curator Beastie the carrier).

- **Routed from workbench-event-disjunction-seat (close, 2026-08-27):** "the creature that spell becomes as it resolves" (Illusionary Mask's subject) — the face-down/permanent-spell becoming read; a face question, so it lands here.

- **Routed from workbench-description-1 (close, 2026-08-28):** the MELD
  EFFECT vocabulary — "exile them, then meld them into [Z]" has no rows;
  seven ownership-condition cards (the umbrella's five plus Mishra and Urza)
  land nothing until it exists. The Meld representation is RULED (duplicated
  reverse face + identity lint, 2026-08-27); this is the verb/effect side.

## As landed

Corpus authority throughout: `data/derived/cards.jsonl` filtered
`jq 'select(.supported)'` (32,568 supported faces; 32,213 with text), all
counts re-measured 2026-09-02 with reminder text stripped where the number
is about printed rules text. CR text via `data/rules/`.

### 1. The transform verb — landed

The verb vocabulary being open, the check the ticket asked for came back
BOTH ways: the label is a `verbFacts` row plus a macro, and the BODY is a
new core row, because [CR#701.27a]'s "turn it over so that its other face
is up" has no expansion under it the way [CR#701.9a]'s discard has.

- `Effect.TurnOver` — the act itself, gated `OnBattlefield`. Not a
  `SetStatus`: [CR#110.5] closes a permanent's status at four categories
  and a side of a card is none of them, and [CR#701.27b] says outright
  that transforming and turning face up "are different game actions". A
  fifth `StatusCat` is refused for that reason and the refusal is written
  at the row.
- `verbFacts` rows `"Transform"` and `"Convert"` — one body, two labels,
  which is what [CR#701.28a] licenses by routing convert back through the
  transform rules. Both rows record **no participle**, and a rule rather
  than a printed zero says so: [CR#701.27g] has already given "transformed
  permanent" to a STATE, so a verbed anaphor spelling those words would
  name the wrong permanents. 0 supported lines write "the transformed
  [noun]" as a lookback.
- `Macros.transform` / `Macros.convert`. What the label buys here is the
  EVENT, not a stamp: `VerbedEvent` names the act by label, which is
  [CR#701.27e]'s trigger family.
- Counts: 221 supported faces write the transform imperative, 23 the
  convert one (14 cards, all Transformers). 39 faces write a trigger on
  the act, 37 of them narrowing it "into [name]".

**The transformed ARRIVAL, landed beside the verb and not as part of it.**
`TokenRider.EntersTransformed` + `Macros.returnToBattlefieldTransformed`.
[CR#712.14a] makes "put onto the battlefield transformed" an arrival
property — nothing is turned over — and at 94 supported faces it is the
largest single phrasing in the whole transform region, larger than the
imperative's carriers.

**Carriers.** Arlinn Kord // Arlinn, Embraced by the Moon is benched
WHOLE (`arlinnKord`); both faces write the verb. Garruk Relentless is
reduced to a blocker that is not the verb: its front-face header is a
STATE TRIGGER [CR#603.8], a condition where `Triggered` takes a
`GameEvent`. Its two loyalty abilities and its whole back face write
today. 1 supported face in the corpus writes a loyalty-counter state
trigger and it is this one, so the machinery has one carrier and is
declined here on that count. `planeswalkerBackWithoutLoyalty`'s note and
`predatoryWurm`'s note, both of which said "blocked on the transform
verb", are corrected.

### 2. Meld — landed per the ruling, verb side and card side

- **Card side.** No sixth `Card` constructor. Under the 2026-08-27 ruling
  each card of the pair is a `Transforming` card carrying its OWN copy of
  the combined back face. `Card`'s docstring now states the ruling, what
  the duplication throws away ([CR#712.4b]'s one face belonging to two
  cards) and the two further differences recorded rather than modelled
  ([CR#712.4c] no transform, [CR#712.21] two cards leaving).
- **The identity lint, landed and cheap.** `meldBackFacesAgree : Refl`
  between `chitteringHostOnScavengers` and `chitteringHostOnGrafRats` —
  two independently written copies, held identical by the typechecker.
  The build fails the moment they drift.
- **Verb side** (the routed meld-EFFECT item). `verbFacts` row `"Meld"`,
  `TokenRider.EntersMelded` ([CR#701.42a]'s "back faces up and combined",
  with the clause's own name slot), and `Macros.meldInto`. The exile is
  NOT part of the act — [CR#701.42a] says nothing about exiling — so it
  stays the instructing clause's `Sequentially` step; `meldThemInto`
  benches that two-step shape.
- **Bench.** Midnight Scavengers // Chittering Host, whole — the first
  meld card in the bench.
- **VERIFIED against the brief's premise:** all 21 meld faces in the
  corpus are supported, Gisela/Bruna/Brisela included. The brief's doubt
  was unfounded and no melder is excluded for support.
- **Honest remainder.** The seven ownership-condition cards still do not
  land, and the meld vocabulary is no longer why. Each writes "you both
  own and control [X] and [a Y named Z]" — one clause over TWO named
  objects — then reads them back as a single "them". `AndCond` writes the
  two existence claims; `Them` admits one `ManyOf` where they leave two
  `OneOf`s. The plural anaphor over two singular antecedents is the whole
  of the remaining blocker, shared by all seven. Recorded at
  `obeliskOfUndoing`, whose stale "no vocabulary for" note is corrected.

### 3. Rooms — the designations landed, the door declined with counts

- `Designation.LeftHalfUnlocked` / `RightHalfUnlocked` [CR#709.5c], on
  `Saddled`'s mold: six tables, `HeldBy Object`, seeded to the
  battlefield, checked and given. `designationSeedType` is `Nothing` and
  the rule says why — [CR#709.5c] names a permanent and no card type.
- The unlock special action [CR#709.5e] was already named at
  `SpecialAction.UnlockDoor` (the mana-purpose seat). Its docstring now
  records the action in full and which designations it gives.
- **The honest zero:** 0 supported lines name either designation. What a
  card prints is always the DOOR [CR#709.5j]. Declined with counts and
  recorded at the type: 28 faces write "When you unlock this door"
  [CR#709.5h], 2 write an unlock/lock instruction [CR#709.5f,709.5g], 3
  read the unlocked state by counting doors. All wait on the door noun
  that keyword-1's scope fence left out; none is reachable cheaply.

### 4. Fuse — landed

`keywordFacts` row `"Fuse"`, `NoParam` / `AtCasting` / spell-card only
[CR#702.102a]. 17 supported cards print it, one keyword line per half; all
34 halves are instants or sorceries (18 sorcery, 16 instant), so the
permanent cell is a measured zero and not a refusal — [CR#709.5] admits
permanent split cards. Benched as Profit // Loss, which is what the
earlier round avoided by choosing Wax // Wane. What the row does not buy
is the fused spell itself [CR#702.102b,702.102d].

### 5. The Adventure exile-and-recast rider — verdict: no row

The reminder-text reading is UPHELD and now written down, at
`AdventureInset`. [CR#715.3d] states the whole rider as a rule, and every
printed occurrence is reminder text: 108 supported adventure faces write
"(Then exile this card. …)" and all 108 are parenthesized, with no
unparenthesized occurrence anywhere in the supported corpus. The inset
frame is what a card prints; the rider is what the rule supplies, exactly
as [CR#310.12b]'s intrinsic Siege ability is.

### Pins

`badTurnOverOffField` — "Transform target creature card in your
graveyard"; [CR#701.27a] turns a PERMANENT over and [CR#110.1] leaves a
card in a graveyard none.
`badTransformedArrivalOffField` — a transformed arrival at a destination
other than the battlefield; [CR#712.14a] states the arrival for one
destination and [CR#712.14] leaves a card off the battlefield no face up.

### Recorded remainders (not in this round)

- The transform trigger's **intransitive active voice** and its "into
  [name]" complement [CR#701.27e]. `VerbedVoice` has an active voice that
  names an actor and a passive that spells "[what] is [participle]";
  "this creature transforms" is neither, and the label carries no
  participle by design. 39 faces, 37 of them with the complement.
- The **casting** "transformed" [CR#712.11a] — "cast it transformed",
  disturb — 62 supported faces. A stack-side reading, not this arrival.
- **"Transformed permanent" as a description** [CR#701.27g], 3 supported
  lines.
- **State triggers** [CR#603.8] — Garruk Relentless, 1 loyalty-counter
  carrier.
- The **plural anaphor over two singular antecedents** — the seven
  melders' shared blocker.
- The **door noun** [CR#709.5j] and the unlock trigger [CR#709.5h].
