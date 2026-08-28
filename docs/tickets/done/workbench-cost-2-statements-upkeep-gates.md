# cost-2: the cost statements, cumulative upkeep and the cost gates

Sub-round 2 of [workbench-cost-and-payment-residues](workbench-cost-and-payment-residues.md)
(the umbrella — authoritative). Runs after sub-round 1; may run parallel with
sub-round 3 (stay off `SpendPurpose`/`ProducedRun`/`ScaledMana` — its lane).
Owns: the four cost-statement pieces (the conditional-"if" marking — settle
FIRST whether [CR#601.2f]'s lock-in vs [CR#611.3a]'s at-any-moment makes it a
distinct one-time test, then land 127 lines or record; the COLOURED payload
as a second shape beside `Amount`; the ACTIVATION variant — RE-PROBE first,
prohibition-2 put ability subjects on `CostsToCast` and an `activator` on
`AbilityAt`, much may now write, Agatha's floor rider landed in
prohibition-2; the durational span cell — Cheering Fanatic, flip
`admitsSpan CostModification` by deliberately renaming the `SpanUse` row);
cumulative upkeep's residues (the SECOND `costActionOk` table over `Cost` —
Braid of Fire, Psychic Vortex, Varchild's, Wall of Shards; colon-measured
zeros unchanged; the mana-OR-mana `Cost` disjunction — 4 cards, serves every
carrier, never a widened `ManaCost` list; the blocked-carrier ledger stays
recorded); the cost GATE's pieces (the DEFENDING-PLAYER noun FIRST —
Propaganda and Ghostly Prison bench whole, 15 lines;
`deonticPatientOk GateT Attack Agent` opens only for the measured case with
the chapter-40 comments updated; the ANAPHORIC count over a plural mention —
9 lines, overlapping; the COORDINATED deed — probe first, prohibition-1's
deed LIST may carry it free; the payer noun inside an action cost — 2 lines;
the Aura carrier recorded); `costActionOk`'s ~40 attestation rows (attest
what a witness pays, record the rest); the [CR#107.3k] boundary (3 cards, the
ability's own telescope); Cavern-Hoard Dragon's rider.

Standard constraints apply; re-measure every count.

## As landed (2026-08-28)

Every count below was re-measured on `data/derived/cards.jsonl`, supported
cards only, reminder parentheticals stripped before matching. Where a count
or a premise differs from the umbrella's, the re-measure is authoritative
and the correction is written into the grammar beside the row it belongs to.

### The conditional "if" — SETTLED, then landed

**Decision: the same construction as the closed pair, a third marking
WORD.** [CR#601.2f] determines a total cost once and then locks it in, and
that is the lock-in an "if" reading would need — but the lock-in there is
the TOTAL COST's, and [CR#611.3a] denies a static ability's continuous
effect any of its own ("isn't 'locked in'; it applies at any given moment to
whatever its text indicates"). A cost-modification static is therefore read
exactly once, at [CR#601.2f]'s determination step, whichever word marks it,
so "if" and "as long as" say the same thing about the same statement. The
reading is written at `CondMarking`'s declaration.

`CondMarking` gains **`IfSo`**; `markingOk IfSo _ = True` (3 cost lines write
a negated condition under "if" — Avatar of Will, Hagra Mauling, Hunter's
Mark — so a positivity gate would refuse printed text). `Macros.ifSo` and
`Macros.onlyIfSo` are the leading and postposed spellings.

**Re-measure: 145 lines, not 127** — 139 postposed, 6 leading. **48** of them
write "if it targets ...", which is the described subject's own predicate
(Ghostfire Blade's landed idiom) and no part of this marking; **97** write
the game-state condition it is for.

**Measured zero, recorded not pinned:** no supported line conditions a
NON-cost static on a game state with "if". The 6 that write "if" beside
another static kind describe the SUBJECT ("if it's white", "if it has
flying", "if it devoured a creature"). [CR#611.3a] would read such a line as
a standing condition, so nothing refuses it and `markingOk` gates nothing.

Bench: **Academy Journeymage** (whole).

### The coloured payload

`CostShift` gains **`CostShiftRun (run : ManaCost) (rises : Bool)`** beside
the two `Amount` arms, and `staticIntro` reads `costShiftDelta` where it read
`amtDelta (costAmount sh)` — so the letter stays readable at the amount arms
(Ghalta, Cavern-Hoard Dragon) and a run opens none.

**Re-measure: 46 lines, not ~25** — 25 of them strive's "costs [run] more to
cast for each target beyond the first", the rest ordinary shifts (Alabaster
Leech, Derelor, Jade Leech, the five Defilers, Edgewalker, Bard Class, Avatar
Aang). **No floor arm:** zero lines pair a run with the "can't reduce ... to
less than" rider. Recorded remainder on the row: Bard Class and Edgewalker
print "This effect reduces only the amount of colored mana you pay" — 2
lines, a rider on the reduction and not a floor.

Bench: **Alabaster Leech** (whole).

### The activation variant — STALE PREMISE, already landed

The re-probe the ticket asked for: `CostSubject.AbilityCostSubject` has been
open since prohibition-2, and **nothing was owed**. Re-measured **89** lines
write "cost [amount] less/more to activate" — **49** about the object's own
ability, **12** at a keyword-named class — and Training Grounds, Power
Artifact, Suppression Field, Gloom, Bureau Headmaster, Fervent Champion and
Ghostfire Blade already bench the range. Agatha of the Vile Cauldron's floor
rider is `CostLess`' own slot, landed. The finding is written on
`CostSubject`.

New bench anyway, because it pairs the variant with a gate: **Oppressive
Rays** (whole).

### The durational span cell — STALE PREMISE, nothing to rename

`admitsSpan` and `SpanUse` **do not exist**: `SpanOk` is indexed by
`StaticKind` and asks the kind nothing, because [CR#611.2a] gives a clause
any stated duration. **Cheering Fanatic already benched** (`Continuously
(CostsToCast ...) (Just thisTurn)`), and it is the corpus's only durational
cost line. The deliberate `SpanUse` rename the ticket specified has no
target; the finding is recorded on `SpanOk`.

### Cumulative upkeep

**The second `costActionOk` table is NOT owed — stale premise.** The pins
round widened the table to [CR#118.1] ("a cost is an action ... a player
carries out the instructions"), and `AddMana`, `Draw`, `Create` and
`ChangeLife Up` all read `True` there already. Re-measured: **78 rows, 20
refuse**, and every refusal is the sentence's own — no row is refused for
want of a witness. [CR#118.1] answers for every carrier at once, so a second
table keyed by `Cost` would say nothing the first does not. The
colon-measured zeros are untouched. Attestation written on `costActionOk`.

Benches: **Braid of Fire** (whole), **Wall of Shards** (whole), Psychic
Vortex's upkeep line, Varchild's War-Riders' upkeep line.

**The mana-OR-mana cost:** `Cost` gains **`EitherCost l r`**, flat arms
(`NotCompound` each, `CostSeq`'s own refusal), announcing nothing —
the payment does not record which arm was taken. On `Cost` and not a widened
`ManaCost`, and not a hybrid symbol [CR#107.4e]. [CR#702.24a]'s own sentence
gives the per-age meaning. 4 cards. Bench: **Earthen Goo** (whole).

**The blocked-carrier ledger** (Karplusan Minotaur, Herald of Leshrac, Jotun
Grunt, Balduvian Shaman, Phyrexian Soulgorger, Cover of Winter) and the
out-of-scope longhand mirror (Cyclone, Phantasmal Sphere, and the three
consequence-swapping siblings) are recorded on the `keywordFacts` row.
Re-measured: 80 cards print the line, 5 grant it in a quotation.

### The cost gate — one enabling change, four pieces

`Deontic`'s modality is typed at **`selfSubjIntro n`** (`Gets`' seat, and for
`Gets`' reason: the subject is written before the cost and a deictic one
announces itself), and **`GatedBy`'s cost adds `gatePayer`** — the payer
[CR#508.1h] and [CR#509.1d] derive, announced so the cost can read it back.
The payer is still never a slot; announcing it is what lets the printed
"their controller"/"they" be a read.

- **The DEFENDING-PLAYER noun — already open, stale premise.**
  `deonticPatientOk _ ["Attack"] Agent (DefendingPlayer _)` is `True` and has
  been since the carrier unified (Blazing Archon benches it). What actually
  blocked Propaganda was the cost side: a predicate for "that's attacking
  you". New `Predicate` row **`AttackerOf`** — `AttackedBy`'s other voice and
  `BlockerOf`'s shape one step earlier, object-kinded with a [CR#506.3]
  kind-indexed relatum, not uniquifying because [CR#508.1b] gives each
  ATTACKER one defender and not the reverse. Re-measured **17** gate lines
  name a defender and a cost at once (the umbrella said 15).
  Benches: **Propaganda** (whole), **Ghostly Prison** (whole).
- **The ANAPHORIC count** — `GroupSize` over the plural the subject
  announced, reachable now that the cost sits after the subject. 9 lines.
  Bench: Archangel of Tithes' block line. **Remainder:** 8 of the 9 name
  "you or planeswalkers you control" — a DISJOINED defender, one mention at
  a joined kind, which no noun in the grammar spells; recorded on
  `DefendingPlayer`.
- **The COORDINATED deed — free, as probed.** The carrier's `Deeds` list is
  the coordination. Benches: **Myr Prototype** (whole), Oppressive Rays.
- **The PAYER NOUN inside an action cost — landed**, by the same
  announcement: a life payment writes `They`. Bench: **Heat Wave** (whole).
- **The Aura carrier — closed, not merely recorded.** Brainwash was benched
  already; **Awesome Presence** (whole) and **Oppressive Rays** (whole) join
  it, and Awesome Presence spends "defending player" at the BLOCK role
  through the same derived payer.
- Chapter-40 comments: none were left stale in the grammar. `DefendingPlayer`
  now carries the measurement and the disjoined-defender remainder.

### The [CR#107.3k] boundary — enforced

New `dropLetter : Letter -> Bindings -> Bindings` in `Words.idr`, and
`Activated`'s cost is typed at `dropLetter X bs`. [CR#107.3k] makes an
activated ability's activation-cost X independent of every other X on the
object, an explicit exception to [CR#107.3i], so the letter the printed cost
announced [CR#107.3a] is taken out of the ability's telescope and the
ability's own cost opens its own. The guard and the activator stay at `bs`.
Measured: **2** supported cards write both letters at once (Chamber Sentry,
Defenders of Humanity); Riptide Replicator writes an X in a body whose cost
has none and closes it with its own where-clause.
Bench: Chamber Sentry's damage ability. Pin: `badActivatedClosesCardLetter`.

### Cavern-Hoard Dragon's rider

Landed as `cavernHoardDragonRider` — Ghalta's telescope (`CostsToCast This
(CostLess (LetterVal X) Nothing)` then `Define X`) over the narrowed domain
the amount beside it already spelled.

### Gates

`idris/scripts/build` 23/23, 0 errors, 0 warnings.
`cargo xtask cite check --list-noncompliant` 0;
`cargo xtask cite check` 0 stale; `cite bless` registered 1 new rule
([CR#508.1h], the attack-cost determination); the round diff was audited with
`cargo xtask cite audit --diff` over 41 citation sites with each rule read
against its claim (one was tightened in the pass: the cost subject's stack
requirement now cites [CR#601.2a] for the placement beside [CR#601.2f] for
the determination).

### Explicit remainders

1. The DISJOINED defender, "you or planeswalkers you control" — 8 gate lines
   (Archangel of Tithes' attack line, Archon of Absolution, Baird,
   Forbidding Spirit, Norn's Annex, Sphere of Safety, Sivitri, Onakke
   Oathkeeper's planeswalker-only variant is `AttackerOf`'s kind index and
   writes). Wants a noun-level disjunction.
2. Norn's Annex's {W/P} scaled payment and Sphere of Safety / Collective
   Restraint's {X} per-unit are sub-round 3's ledger, untouched here.
3. "This effect reduces only the amount of colored mana you pay" — 2 lines,
   recorded on `CostShiftRun`.
4. Nils, Discipline Enforcer's per-creature counter read inside a gate cost
   ("pays {X}, where X is the number of counters on that creature") — a
   distributive read over the subject's members, not `GroupSize`.

### Deviations

- **Two of the ticket's pins were stale premises and are recorded rather
  than built:** the SECOND `costActionOk` table keyed by `Cost` (already
  answered by [CR#118.1]) and the deliberate `SpanUse` rename behind
  `admitsSpan CostModification` (neither name exists; Cheering Fanatic was
  already benched). Both findings are written into the grammar at the rows
  that hold them.
- The DEFENDING-PLAYER patient was likewise already open; the piece that
  actually blocked the marquees was the cost's own predicate, which is what
  `AttackerOf` supplies.
