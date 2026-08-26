---
needs: []
---
# The counter family's distributive residues and its two table holes

`docs/tickets/done/workbench-counter-family-residues.md` closed the counter
region and left three constructions explicitly OUT of its scope, plus two table
holes named by neighbouring rounds. Nothing owns them now that the family ticket
is closed.

## 1. The player-side distributive `get` — Winding Constrictor, line 2

From that round's ledger:

> "If you would **get** one or more counters" is the player-side distributive —
> `GetsCounters`' twin of `PutCountersOfThoseKinds`, unbuilt.

`PutCountersOfThoseKinds` landed for the object side; this is the same
kind-blind distributive with a player recipient and the `get` verb.

## 2. Aragorn — the same anaphor, a different frame

> same anaphor as `PutCountersOfThoseKinds`, but the recipient "up to one other
> target creature" has no row and the clause is a trigger body rather than a
> replacement.

Two independent deltas: the "up to one other target" recipient shape, and the
distributive kind anaphor in a trigger body rather than under `Intercepts`.

## 3. "The same number and kind of counters" — Captain Marvel, Denry Klin

> "the same number and kind of counters" / "the same number of each kind of
> counter" are the "counter of that kind" axis, deliberately not swept in.

The counter round drew its boundary here on purpose. This is that axis.

## 4. `EntryCounterMark` has no "fewer" arm

`docs/tickets/done/workbench-pins-refuse-rules-impossibility-only.md` carries it
as consolidated open gap 12, with Nahiri's Compleated as the carrier. The entry
mark rows the more/additional direction only.

## 5. `effEq` ignores `LosesCounters`' amount slot

From `docs/tickets/done/workbench-closure-flip-risks.md`:

> `effEq` ignores `LosesCounters`' new amount slot, matching the neighbouring
> `GetsCounters` row which compares nothing at all. If `effEq` ever becomes
> load-bearing for counter clauses, both rows need the amount.

Latent, not currently wrong. Fix both rows or record why the equality is
deliberately coarse — do not fix one.

## 6. The Suspect counter kind

`docs/tickets/done/workbench-amount-comparison-and-quantity.md` names it as
Investigator's Journal's remaining blocker ("enters with a number of suspect
counters on it equal to …"), outside that ticket's counter scope. One catalog
row; take it with the rest.

## Consumption boundary

`idris/src/Experimental.idr` (`GetsCounters`, `PutCountersOfThoseKinds`,
`EntryCounterMark`, `effEq`, the recipient shapes), `idris/src/Experimental/Words.idr`
(the counter-kind catalog), the pin modules `idris/src/Experimental/Proofs*.idr`,
and the evidence bench `idris/src/Experimental/Cards.idr`. No Rust crate.

## Acceptance

- Winding Constrictor's second line and Aragorn each bench or are reduced to a
  named blocker that is not the distributive.
- The "same number and kind" axis lands or is named at its measured size.
- `effEq`'s two counter rows agree with each other, whichever way.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.

## As-landed

Standard constraints applied. `idris/scripts/build` PASS, 0 errors, 0
warnings; all `Proofs*` modules re-elaborated against the widened slots and
no pin silently passes. Counts below were measured this round with the
corpus scripts.

### 1. The player-side distributive `get` — LANDED, Winding Constrictor whole

Two pieces, because the effect twin alone does not carry the line:

- `CounterEvent`'s holder is now kind-polymorphic (`{k : Kind} ->
  (n : Noun bs k)`, gate `CounterKindNamed k kind`). [CR#122.1] places a
  counter "on an object **or player**", so the object-only holder was a
  modelling preference. This reverses the counter-family round's
  "Nothing here widens `CounterEvent`'s noun past `Object`". The kind gate
  is untouched, so a stat counter still cannot be got and a poison counter
  still cannot be put on a permanent (`badGetsBoostCounter`,
  `badPutPoisonOnCreature` both still refuse). `bareCounterEvent` and
  `manyBareCounterEvent` widened with it; the kind-named macros stay
  Object-side, where their `counterScope` gate already put them.
- New `Effect` row `GetsCountersOfThoseKinds : (who : Noun bs Player) ->
  (amt : Amount (nomIntro who)) ->
  {auto 0 ok : countOutcomes CountersPut bs = 1} -> Effect bs`, with rows in
  all nine total tables (`heldUntilOk`, `reflexEncloseUse`,
  `thisWayOutcomeOk`, `costActionOk`, `effEq`, `effIntro`, `preIntro`,
  `annIntro`, `deedDelta`). `costActionOk` is `False` for the same reason
  its object-side twin is: no cost announces a batch.

`windingConstrictor` benches **whole**, both lines. Line 1's "an artifact or
creature you control" needed nothing new. The player-side event family is
**one supported line** — Winding Constrictor's own; the four `--all` hits are
energy (out of scope by the counter round's rule) and one unsupported poison
replacement.

### 2. Aragorn — LANDED, no new declarations

Both named deltas already composed:

- "up to one other target creature" is `TargetGroup (upTo 1)
  (otherCreature thisCreature)`. Bare `Other` does not fit — it reads against
  an earlier targeted mention (Phantom Blade's shape), and Aragorn's "other"
  is anchored on the source, which is `OtherThan`'s job.
- the distributive in a trigger body needed nothing: `eventAfter` already
  gives a many-counter event's body the `CountersPut` outcome binding, so the
  anaphor's presupposition is discharged off the header exactly as it is
  under `Intercepts`.

`aragornDistributive` benches the second line. The card waits on the
Ring-tempts header its first line writes.

### 3. "The same number and kind of counters" — the announced-batch half
LANDED, the holder-sourced half named at size 1

Measured: **3** supported lines, 4 with `--all`. They split on where the
counters are read FROM, and that is the whole difficulty:

- **announced batch** — Captain Marvel, Apex Avenger and Bold Plagiarist (2
  supported). This is `PutCountersOfThoseKinds ThatMuch on` and needed
  nothing: "the same number and kind" is the distributive at the batch's own
  size, and Winding Constrictor's rulings confirm the per-kind reading the
  constructor documents. Benched as `captainMarvelSameKinds`. Its card waits
  on the intervening "if it's not a Kree".
- **holder-sourced** — Denry Klin, Editor in Chief (1 supported; Dominion
  Saboteur writes the same read into `ExceptEntersWithCounters`, unsupported).
  "put the same number of each kind of counter on that creature" reads the
  counters a holder HAS. [CR#122.8] performs that same read for its own
  leaves-the-battlefield case — "the player puts the same number of each kind
  of counter the first object had onto the second object" — and marks it as
  not a move [CR#122.5]: the source keeps what it has. **Not landed**: the source is
  unwritten on the surface (Denry names it only in the intervening-if), so
  whether it is a holder slot or an anaphor over the condition's mention is
  an open shape choice, not a mechanical row. Named here at size 1.

### 4. `EntryCounterMark`'s "fewer" arm — LANDED

`Fewer` joins `Fresh`/`Additional`, with `entersWithFewerCounters` beside the
other two macros. [CR#614.1c] makes all three the same kind of entry
replacement; [CR#306.5b] is the printed count `Fewer` subtracts from.
`nahiriCompleatedEntry` benches the reminder's entry clause. Remaining
blocker for the whole reminder, named: "If life was paid" reads back a
payment made while casting and has no condition row.
`docs/idris-workbench-closure-tables.md` updated at both `EntryCounterMark`
entries — the rules-inertness claim was about the plain/"additional" pair and
does not extend to `Fewer`.

### 5. `effEq`'s two counter rows — BOTH fixed to compare amounts

Both rows now compare kind AND amount, and both narrow to `You` to do it: an
`Amount` is indexed by its subject's discourse, so two amounts under
different subjects are not the same type. `Draw` narrows the same way for the
same reason, and conservative `False` off the narrow row is the table's house
pattern. `LosesCounters` loses its non-`You` equalities in the trade; `effEq`
has no consumers, so nothing depended on them.

### 6. The Suspect counter kind — LANDED, card whole

`CounterKind.Suspect`, `counterScope Suspect = Object`, `Eq` row.
`investigatorsJournal` benches **whole** — the entry count reuses the
already-landed `greatestCreaturesAPlayerControls` binder, and both activated
abilities needed nothing new.

### Cards and phrases benched

- **Winding Constrictor** (whole) — both distributive lines.
- **Investigator's Journal** (whole) — entry count, counter-removal cost, sacrifice draw.
- **Aragorn, Company Leader** (second line) — `aragornDistributive`.
- **Captain Marvel, Apex Avenger** (trigger, less the intervening-if) — `captainMarvelSameKinds`.
- **Nahiri, the Unforgiving** (compleated reminder's entry clause) — `nahiriCompleatedEntry`.

### Ledger

- The holder-sourced "same number of each kind of counter" ([CR#122.8]'s
  read): Denry Klin, plus Dominion Saboteur on the copy-entry frame. One
  supported line, blocked on an unmade shape choice, not on machinery.
- The Ring-tempts trigger header (Aragorn's first line).
- "If it's not a Kree" — a negated subtype intervening-if (Captain Marvel).
- "If life was paid" — a cast-time payment readback (compleated's reminder).
