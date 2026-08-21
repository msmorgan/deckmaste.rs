---
needs: []
---
# Finish the designation catalog, its readers, and the keyword expansion that confers one

The designation region as one claimable unit: the catalog's eight remaining
entries plus the two reader-side gaps (the card scope's reader, the absence
check), and the mechanism that lets a keyword's expansion body confer a
designation the bare instruction is still refused. They are one round because
both halves sit on the same declarations — `Designation`, `designationScope`,
`designationChecked`, `designationGiven`, `DesignationHolder` and the designation
predicate — and the giving cell the expansion needs is the same table the catalog
rows populate.

## The catalog's rows and its missing readers

Fourteen designation rows cover thirteen of the twenty-one catalogued entries;
eight are left, and the two reader-side gaps beside them are the same family. The
catalog is open, so most of this is rows and cells, not design.

### The catalog's eight remaining entries

- **No supported card text at all — 3**: protector, planar controller,
  archenemy.
- **Reminder-only — 1**: the three sector designations appear inside space
  sculptor's reminder text on Space Beleren and nowhere else, so like Mount they
  are neither rowed nor pinnable. Do not row them; do not pin them.
- **Written through a keyword action or an ability container — 4**, and so route
  with the keyword-action work rather than through either designation reader:
  harnessed ("Harness The Mind Stone", 2 cards, [CR#701.64a]); solved (a Case's
  "Solved —" label, 13 cards); level (a Class's level bars, 68 occurrences over
  34 cards); the unlocked pair (lock and unlock instructions, 110 occurrences
  over 79 cards, [CR#709.5c]).

None of the eight needs a design decision: each is a row and a handful of cells
on whatever round benches a carrier.

### The CARD scope's reader — two entries' worth behind one scope cell

The commander has a scope cell ([CR#903.3]: the designation "is an attribute of
the card itself", which is why it survives a zone change) and NO reader, because
neither form the corpus writes is one:

- **"your commander"** — 84 supported occurrences over 81 cards, written as a
  possessed NOUN in subject position. The biggest single designation family in
  the corpus; it wants a noun row, not a predicate.
- **"is your commander"** — 3 lines, all three inside a before-the-game static
  ability that finding 188 keeps unread.

### The designation ABSENCE check — 5 lines

"There is no monarch" asks whether ANY player holds a designation rather than
describing one that does: an existential over the holder, sibling to the other
absence checks. It is the only negative the designation family writes at all —
every "isn't the monarch"-shaped negation measures ZERO, with one "isn't
saddled" line as the exception, which rides here.

## The expansion body that confers a keyword-granted designation

Monstrosity's body cannot be written because the cell that refuses a bare
"becomes monstrous" instruction also refuses the keyword's own expansion. What is
needed is the mechanism that tells a bare sentence from an expansion body, for
all five keyword-conferred designations at once.

- The monstrosity tag was measured and **refused** (finding 526): the machinery
  keys on the DESIGNATION, which has had its row for many chapters, not on the
  action's name. "Monstrosity this way" is **0** lines and "would become
  monstrous" is **0** lines, so **no `VerbName` row is earned**.
- What is needed is the ability to WRITE [CR#701.37a]'s expansion, and the probe
  named the block precisely: `GainsDesignation This Monstrous` is refused
  because `designationGiven Monstrous` is False.
- That cell is **right about bare sentences** — no card writes "this creature
  becomes monstrous" as an instruction — and **wrong about an expansion body**.
  Telling the two apart is the mechanism question this answers.
- It is the same question the other four keyword-conferred givings ask: city's
  blessing, enduring story, renowned, saddled. All four cells stay False, and
  that was confirmed by benching three cards that print the keyword line and
  check the designation without ever instructing the conferral.
- 37 monstrosity lines, all of the form "{cost}: Monstrosity N."; **Chillerpillar**
  is the card it makes whole.

## Consumption boundary

`idris/src/Experimental/Words.idr` (`Designation`, `designationScope`,
`designationChecked`, `designationGiven`, `DesignationHolder`, the noun rows,
`VerbName`), `idris/src/Experimental.idr` (the designation predicate and the
absence checks, `GainsDesignation`, the designation rows, the keyword expansion
bodies), `idris/src/Experimental/Macros.idr` (the reader spellings and the
keyword line's spelling), the pin modules `idris/src/Experimental/Proofs*.idr`,
evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- The four keyword/container-written designations are rowed with their cells, or
  explicitly deferred to the keyword-action round; the three unattested ones and
  the reminder-only sectors stay unrowed and unpinned, with their zero recorded.
- "Your commander" reads as a possessed noun, not as a predicate.
- The 3 "is your commander" lines stay unread while finding 188 stands — this
  round does not open the before-the-game static.
- The absence check is an existential over the holder and admits the "isn't
  saddled" line; the zero on "isn't the monarch"-shaped negations survives.
- An expansion body confers the designation while the bare-instruction refusal
  still stands for all five designations — the mechanism distinguishes them
  rather than flipping `designationGiven`.
- No `VerbName` row is minted for monstrosity; the two zeros are recorded as the
  reason.
- Chillerpillar benches whole, and the three cards benched on the keyword-line-
  plus-check shape keep passing.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As landed (2026-08-21)

### The expansion-body mechanism

`GainsDesignation` now carries a positional **warrant** and no longer gates on
`designationGiven` at the constructor. `GivingWarrant : Designation -> Type`
(`Experimental/Words.idr`) has exactly two arms:

- `Instructed` — the bare sentence, which still requires
  `So (designationGiven d)`. Every cell keeps its old value; nothing was
  flipped.
- `InExpansionOf (w : ConferringWord)` — indexed by
  `conferredDesignation w`, so the designation is inferred from the keyword.

`ConferringWord = MonstrosityW` — one arm, bought by monstrosity's expansion
[CR#701.37a] and Chillerpillar. The other four are dropped; see below.

**What the mechanism actually guarantees.** `InExpansionOf` is a warrant a
keyword's own MACRO supplies when it spells that keyword's expansion body.
Nothing in the types ties it to an expansion: `GainsDesignation
Macros.thisCreature Monstrous (InExpansionOf MonstrosityW)` compiles as a bare
ability. The bare-sentence refusal therefore rests on **macro discipline plus
the `Instructed` pins**, not on structure — a card written through the card
language reaches the conferral only through `Macros.monstrosity`, and the
direct route is pinned as unspellable. A structural expansion-context index
would close the gap; minting one is a bigger design than this round, and it is
**a recorded gap**. `designationGiven` is byte-identical.

Monstrosity's own tag stays out of the core: `Macros.monstrosity` spells
[CR#701.37a]'s expansion body directly
(`If (put N +1/+1 counters, then gain the designation) (isn't monstrous)`),
because both re-read measurements are zero — no `VerbName` row was minted.

- Witness: **Chillerpillar** now benches whole (card, not a bare ability) —
  `{4}{S}{S}: Monstrosity 2.` plus the monstrous check.
- Pins (ProofsG): `badMonstrousInstruction` (object-held cell),
  `badBlessingInstruction` (player-held cell) — the bare instruction refused on
  both sides of the scope split.

**Arms dropped, follow-up for the keyword-action round.** Only `MonstrosityW`
has a macro, a witness and a pin, so only it was kept.

- `SaddleW` [CR#702.171a] and `AscendW` [CR#702.131a] — their expansions carry
  a span ("until end of turn", "for the rest of the game") that
  `GainsDesignation` cannot carry: `heldUntilOk (GainsDesignation _ _ _)` is
  `False`. They need the span first.
- `StoriedW` [CR#702.195a] — storied is a static ability, not an instruction,
  so its expansion is not an `Effect` at all.
- `RenownW` [CR#702.112a] — instruction-shaped, but its expansion is a whole
  triggered ability, and no `Macros.renown` with a witness was written this
  round. It is the cheapest arm to add next.

### "Your commander" as a possessed noun

`Noun.Designated (d : Designation) (whose : Noun bs Player)`, gated by
`designationScope d = HeldByCard` and the existing `Possessor` — so the row is
bounded by the scope table to `CommanderD` alone, with no new cell. `nounZone`
and `nounTy` are both `Nothing`: [CR#903.3] makes the designation an attribute
of the card, so the noun names no zone. `Macros.yourCommander` spells it.

- Pins: `badPossessedMonarch` (a player-held designation is not a possessed
  noun), `badGroupCommander` (the possessive anchors on one player).
- The 3 "is your commander" lines stay unread; finding 188 was not opened.

### The designation-absence existential

`Condition.NoHolder (d : Designation)`, gated by
`designationScope d = HeldBy Player` and `So (designationChecked d)`; spelled by
`Macros.thereIsNo`. The absence is the whole condition: `condNegatable` and
`condNegated` are both `False`, which bounds it at the measured
"there is no monarch" lines and keeps "unless there is no monarch" and the
double negation unwritten. (The corpus writes one more of them than the
ticket's supported count: "When your commander deals combat damage to a
player, if there is no monarch, you become the monarch" was unsupported only
because "your commander" had no reader, and it is this round's witness.)

- Witness: **Archivist of Gondor** (ability) — "When your commander deals
  combat damage to a player, if there is no monarch, you become the monarch",
  which exercises the possessed-noun row, the absence check and the
  `Instructed` warrant in one line.
- Pins: `badNoHolderOnObject`, `badNoHolderNegated`.
- `badNegatedMonarch` (the predicate-level negation zero) survives untouched.
- The condition-level designation negation the "isn't saddled" line needs
  (`NotCond (Matches _ (HasDesignation _))`, through
  `predNegFree (HasDesignation _) = True`) is exercised by Chillerpillar's
  "if this creature isn't monstrous" inside `Macros.monstrosity`. The saddled
  line itself is stopped — see below.

### The catalog's eight remaining entries

All four keyword/container-written designations are **deferred to the
keyword-action round**, each because the row would carry no reader, no writer
and no witness until the construct that writes it exists:

- **harnessed** [CR#701.64a] — written only by the keyword action
  "Harness [permanent]" (2 lines, both naming the permanent); the word
  "harnessed" occurs only in reminder text. Both `designationChecked` and
  `designationGiven` would be `False`, so the row would admit nothing.
- **solved** [CR#719.3a,719.3c] — conferred by a Case's "To solve — [condition]"
  container and read by its "Solved — [ability]" label. Both are
  ability-container syntax with no row in the workbench.
- **level** [CR#716.2b] — a level is an N-valued designation, read by
  comparison ("level N or greater"). `Designation` is a boolean marker sum whose
  readers test presence; rowing a level needs a number-carrying designation and
  a comparison reader, a design choice this round was not scoped to make.
- **unlocked** [CR#709.5c] — two designations indexed by a half of a permanent
  with a shared type line. The noun that names a half (a Room's door) has no
  row, so neither reader can be written.

Unrowed and unpinned: **protector**, **planar controller** and **archenemy**,
whose zero is a true zero — no supported card text at all.

The three **sector** designations are **deferred, not absent**, correcting this
ticket's body: `corpus --match sector` returns Space Beleren's own
non-reminder loyalty text on three abilities ("creatures in each sector",
"the sector of your choice" twice) alongside the space sculptor reminder line.
(The "Avalanche of Sector 7" lines are a card name, not the designation.) They
stay unrowed and unpinned this round — the designation is assigned by a
planeswalker's own board division and has no reader here — and route with the
keyword-action round.

### Stopped

- **Caustic Bronco**, the single "isn't saddled" line. The blocker is not the
  designation: it is "Otherwise, each opponent loses that much life". `If`'s
  otherwise-branch sits in `bs` rather than in the then-branch's outcome
  context, so `ThatMuch` has no antecedent
  (`Can't find an implementation for countOnes Outcome (nomIntro (Each Opponent)) = 1`).
  Reading an amount across the if/otherwise boundary is an anaphora design
  choice beyond this round's named one.

### Ledger

- `NoHolder` admits the four player-held checked designations (monarch, the
  initiative, city's blessing, enduring story); the corpus writes "there is no"
  for the monarch only. Bounded by the existing scope and checked tables — no
  new cell, and no pin, since the bound is already written down.
- `Designated CommanderD (PlayerGroup YourOpponents)` is spellable at 0 lines:
  `possessorOk` admits the opponent group. Noted for the possessor-bounding
  follow-up rather than pinned now, because narrowing it is a change to the
  shared `Possessor` gate, not to this row.

### Signature changes

- `Effect.GainsDesignation` — positional `(w : GivingWarrant d)` added,
  `{auto 0 at : So (designationGiven d)}` removed (it moved into `Instructed`).
  Three bench sites and two pins now write `Instructed`.

### Macros added

`Macros.monstrosity`, `Macros.yourCommander`, `Macros.thereIsNo`.
`Macros.monstrosity` is the only route to `InExpansionOf` the card language
offers; keeping it that way is the macro discipline the mechanism rests on.
