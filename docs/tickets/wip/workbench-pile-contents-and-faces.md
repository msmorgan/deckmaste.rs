# workbench-pile-contents-and-faces

The pile round's two big residues (close 2026-09-02; full residue list in
Cards.idr under "THE PILE PARTITION'S RESIDUES"):

- **The pile-CONTENTS predicate** — 13 lines / 12 cards, the family's single
  largest blocker (six more whole cards behind it). [CR#700.3b] leaves a pile
  containing nothing (not an object) and [CR#700.3c] blocks `InZone`, so
  membership needs its own read against the `PileP` payload.
- **The FACE-DOWN pile** — 17 lines / 16 cards; `FaceDown` is
  `OnBattlefield`-gated and these mark exile/library piles. Also carries the
  exile idiom's shuffle ([CR#701.24a] shuffles "a library or a face-down
  pile" in one sentence — the grammar's shuffle is library-typed; that gap is
  ours, not the rules').

Re-measure at claim.

## As landed (round of 2026-09-02)

Re-measured this round over `jq 'select(.supported)'` against
`data/derived/cards.jsonl`; where a count differs from the brief above,
the corrected figure is the one to carry forward. **38 supported cards
write the word "pile"** (39 match a naive substring — Dogpile is a card
name).

### Residue (a) — the pile-CONTENTS read: LANDED

**`Predicate.InPile`** (pile), gated at a new **`PileMention`** —
13 lines over 12 cards, confirmed. A row of its own because both
alternatives are refused by the pile rules by name: [CR#700.3c] says
grouped objects "don't leave the zone they're currently in", so a pile
is not a place and `InZone` narrows nothing (Death or Glory's two piles
are both in the graveyard, Liliana's both on the battlefield); and
[CR#700.3b] says "the pile is not an object", so `ExiledWith`'s
`LinkSource` has nothing to hold. It **seeds no zone**, for the same
[CR#700.3c] sentence — the description's other words go on saying where
its referents are. Its `predDelta` is the pile phrase's own, as
`ControlledBy`'s and `AttachedTo`'s are theirs.

`PileMention` is enumerated the way `LinkSource` is, three positive
cases and no computation: `PileOf` (the partitive), `That PileW`,
`Those PileW`. Every other phrase is refused by having no case — the
phrase-side reading of the refusal `wordReaches` already makes from the
payload side.

### Residue (b) — the FACE-DOWN pile: PARTLY landed, deliberately

Re-measured: **16 supported cards write "a face-down/face-up pile" over
16 sentences** (the brief said 17 lines/16 cards; three of the sentences
name two piles apiece, which is where the extra line came from). The
family splits in two and only one half was built.

- **LANDED — the face on the PARTITION.** A new **`PileFace`**
  (`FaceDownPile`/`FaceUpPile`), a **third field on `Payload.PileP`**,
  and a **`faces : List PileFace` slot on `SeparateIntoPiles`** gated at
  `FacesFit` (empty, or one face per pile). 6 cards / 6 lines: five
  write "a face-down pile and a face-up pile" (Atris, Curator of
  Destinies, Fortune's Favor, Riddles in the Dark, Sauron's Ransom) and
  Phyrexian Portal writes "two face-down piles". Per-pile because the
  commonest line gives its two piles DIFFERENT faces, which is the
  card's whole mechanism. The mention collapses to one face only where
  the piles agree (`pileMentionFace`), on `joinSeed`'s rule.
  **`PileFace` is its own type and not `StatusVal FaceC`**, on
  [CR#110.5d]'s own sentence: "although an exiled card may be face down,
  this has no correlation to the face-down status of a permanent". It is
  a term the rules use for the pile itself in three places and for three
  purposes — [CR#701.24a] (shuffle), [CR#400.5] (order), [CR#406.4]
  (keeping exiled cards separate) — so it is recorded where the pile's
  zone and size are.
  **One reader**: `pubB` answers False for a face-down pile whatever its
  zone, which is [CR#400.2]'s own exception ("except for those cards
  that some rule or effect specifically allow to be face down") and what
  [CR#406.3] spells out for the commonest carrier.
- **DECLINED — the single pile made by an EXILE**, 10 cards / 10 lines,
  and with it **the shuffle's pile arm**, 7 lines. No witness pays.
  `MoveRiders` cannot carry the pile (its field riders are
  battlefield-gated and no rider contributes a binding), and a partition
  row cannot say it either — five of the ten make two piles with two
  SEPARATE exiles and one splits an uncounted group. Every one of the
  ten is blocked elsewhere as well: seven want `cloak` or `manifest`,
  neither a verb nor a keyword this vocabulary knows. And the shuffle
  arm has **no carrier at all** without it: all 7 "shuffle that pile"
  lines shuffle a pile an exile made. [CR#701.24a] is on the cards'
  side and the gap is real — recorded, not built.
- **DECLINED — the face CHANGE on a pile**, 1 line (Hostile
  Negotiations, "turn a pile of your choice face up"), on a card blocked
  on the exile-made pile twice over.

### Benches — 6 whole cards

**RIDDLES IN THE DARK** (the face pair at its plainest), **FORTUNE'S
FAVOR** (the pair with the roles swapped and no choice after it),
**CURATOR OF DESTINIES** (the pair inside a trigger), **ATRIS, ORACLE OF
HALF-TRUTHS** (a targeted separator inside a trigger), **DO OR DIE** (the
membership read's marquee, and the shortest card that needs it) and
**LILIANA OF THE VEIL** (the membership read on a planeswalker, all
three loyalty abilities).

### Pins — 2, both in `ProofsG`

`badMembershipInANonPile` ([CR#700.3b]'s converse: a group of objects is
not a pile, so nothing is "in" it) and `badPileFaceAsAStatus`
([CR#110.5d] — `SetStatus FaceDown` over piles in the library, refused
by the battlefield gate that must stay). Neither refuses a count.

### Explicitly measured and NOT paid for

- **Boneyard Parley, 1 card** — everything but its last clause writes.
  Naming a pile's members mints a GROUP mention beside the piles, and
  `theRestOk` measures at most one group, so "the rest" is refused with
  two standing (`TheOther` by the same measure). The refusal is honest:
  the two plural mentions denote the same cards under two descriptions.
  The question is `theRestOk`'s, not the pile vocabulary's.
- **Truth or Tale, 1 card** — its pile half writes (`InPile
  (That PileW)` type-checks); it is blocked on "all other cards revealed
  this way", for which `Reveal` has no participle (`verbedMarkingOk`).
- **Fight or Flight and Stand or Fall, 2 lines** — both write their
  partition and their membership read with the landed rows, and are
  blocked on the EXCLUSIVE permission: `Compulsion`'s `Permit` spells
  "[n] can [deed]" and has no "ONLY [n] can" arm.
- **Phyrexian Portal** — the only card whose face need ("two face-down
  piles") this round meets and which still does not bench: it wants a
  group-valued `SearchScope` and the pile shuffle.
- **Sauron's Ransom** — the fifth face-pair card, blocked on "The Ring
  tempts you" as an instruction (the label exists as an event only).
- **The per-player partition (5 cards), the labeled/variable partition
  (2 cards), the top-of-a-pile read (2 cards), the empty-pile reminder
  (5 lines)** — unchanged from the partition round, re-stated in
  `Cards.idr` with this round's numbers.

The full residue list with the same counts is in
`idris/src/Experimental/Cards.idr` under "THE PILE FAMILY'S RESIDUES".

Gate: `idris/scripts/build` 23/23 from a clean `build/`, 0 errors,
0 warnings. `cargo xtask cite check --list-noncompliant` empty;
`cite check` 0 stale over 20628 citations; `cite bless` registered one
new rule ([CR#400.5]); the round's 36 citation sites were read against
their rule texts and two were rewritten for over-reach (a [CR#110.5d]
claim about the pile rather than its members' status, and a [CR#400.5]
cite doing no work at `faceOfThose`).

### Remainder for a later round

The `sphinxOfUthuun` docstring's aside — that Unesh, Criosphinx
Sovereign and Sphinx of Clear Skies are blocked on "a cost reduction;
a domain-counted X" — was reported stale during this round's recon
(`daruWarchief` writes the reduction, `tribalFlames` the domain X).
Not verified by type-check here and not touched; worth a bench attempt.
