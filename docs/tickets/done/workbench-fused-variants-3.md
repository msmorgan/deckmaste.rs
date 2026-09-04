---
needs: [workbench-clause-order-collapse]
---
**Collapse the separable fused variants, delete the dead proofs and unused
constructors, gate `StatOf`, and rename the overloaded constructors.** Fresh
workbench review 2026-09-03: F5, F6, F8, F10, F11, F12, F14, F15, F17, F18,
plus the `LookReq` collapse of ruling 8. One round, ten independent edits.

- **F10 — chosen/last-chosen quadruplet → one axis (S–M).**
  `Predicate.ChosenPlayer`/`TheLastChosenPlayer`, `OfChosen`/
  `OfTheLastChosen`, `ColorTerm.ThatColor`/`TheLastChosenColor` and
  `Amount.ChosenNumber`/`TheLastChosenNumber` differ only in the gate
  (`countChoice … = 1` versus the choice-stands variant). Add
  `data ChoiceRef = TheChoice | TheLatestChoice`, one constructor per family,
  `choiceRefOk ref n`. **"The last chosen" stays the printed lemma** — the
  macro spelling keeps that wording; only the constructor pair collapses.
  `TheLastChosen*` has 6 bench uses.
- **F11 — lookback triplet → one record (M).** `Predicate.HappenedTo`,
  `Condition.Happened` and `Amount.EventTally` each carry the same five
  `(ev, w, what, {cw : ComplementWritten what}, {sb : LookbackSubject ev k})`.
  `record Lookback bs k` holds them and the three constructors take it
  (`EventTally` adds `op`/`tallyOk`); `Macros.happened*`/`eventCount*` shrink.
- **F12 — `Phrase.Noun.PossessorOf`/`PossessorsOf` (S).** Fold into
  `PossessorOf ax n`; plurality comes from `n` and the `one` gate goes
  (`nounDelta` already branches on plurality).
- **F14 — small duplicates (S each).** `Cost.ScaledMana unit amt` ⊂
  `Cost.ScaledCost (Mana run) amt`: keep the readable `ScaledMana` spelling as
  a macro over `ScaledCost` (0 bench uses against 14). `Effect.SkipsAllOf` →
  `Continuously (Skips …) (Just RestOfGame)`. `RemoveCountersAmong` →
  `RemoveCounters` over a `SomeOf … grp` noun.
- **F15 — unused constructors (S).** `Effect.AsThoughLess`,
  `Effect.CostShiftRunWithFloor`, `Effect.Repetition.AgainExcept`,
  `Triggers.GameEvent.Triggers` and `Words.AttachWord.Fortified` (no Fortify
  row either) have zero bench uses. Delete each, or land the card that needs
  it with its pin — the landing record says which, by name.
- **F5 — `Phrase.Amount.StatOf` is not type-gated (S).** `Compare`/
  `Superlative` seed `comparedType` but `StatOf c n` takes any object noun:
  P2 `StatOf Power (target land)` and P7 `StatOf Loyalty (target (HasType
  Battle))` are both admitted, and a noncreature permanent has no power or
  toughness [CR#208.3]. Add `{auto 0 ty : So (fit of comparedType c against
  nounTy n)}`, `Nothing` permissive (animated lands are real). `Defense` on
  `Characteristic` [CR#109.3] is optional — no supported card compares it.
- **F6 — dead proofs in `Words` (S, ~90 lines).** `kindLteComm`,
  `kindLteAssocR`/`L`, `sameKindRefl`, `sameCardTypeEq` and `sameQEq` have no
  references, and `cardTypeAt`, `cardTypeAtIx`, `qualityAt`, `qualityAtIx` and
  `natEqSo` exist only to prove them. Keep `kindLteRefl`, `kindLteJoinL`/`R`
  and `kindLteInL`/`R`.
- **F8 — overloaded constructor names (S).** `Until` (`Triggers.Duration` /
  `Effect.Repetition`), `Named` (`Words.CounterKind` / `Phrase.Predicate`),
  `Define` (`Effect.StaticEffect` / `Effect.Effect`), `Up`/`Down`
  (`Words.Counter.Delta` / `Phrase.LifeOp`), `Bare` (`Words.Reach` /
  `Phrase.ZoneScope`) → `UntilCond`, `NamedCounter`, `DefinesLetter`,
  `LifeUp`/`LifeDown`, `BareScope`. The sixth pair (`StaticFirstDone`) is gone
  already — hence the dependency on `workbench-clause-order-collapse`.
- **F17 (S).** `Macros.Characteristic` → `Stat`, `CharAxis` → `StatAxis`: the
  CR's characteristics include name, colour and types [CR#109.3], which the
  grammar models as `Quality`; this sort is only the numeric quarter.
- **F18 — `scripts/check-eq-indexes` (S).** Keep the index-based-`Eq` check,
  drop the hard-coded count of 47.
- **Ruling 8 — `Macros.LookReq` (S–M).** Scry keeps its expanded body; the
  leaf-with-facts-row option is declined. `LookReq`'s four proof-carrying
  cases (`YourOneLookReq`, `YourManyLookReq`, `TheirOneLookReq`,
  `TheirManyLookReq`, `Macros.idr:1385–1446`) exist only because the agent
  noun cannot be re-read after `agentIntro`. Collapse them with an `Own`-style
  re-read of the agent's own delta, so one macro serves all four.

Also (copies-are-spells residue): after a one-shot `Copy` there is no spelling that reads the ORIGINAL spell (the copy binding carries `prov = Nothing`, and `TheVerbed "Copy"` is pinned unwritten); no bench card needs it yet — when one does, give the copy a provenance stamp or an `Other`-style exclusion rather than a new `Reach`.

Also (turn-parts residue): `Timing.BeforeAttackersDeclared` is a special case of a generic before-point timing; "only before the combat damage step" (Angus Mackenzie, Berserk, Blood Frenzy) needs `Timing.BeforePart (p : TurnPart)`; fold the special case into it and bench Berserk.

Also (levelers residue): `Words.keywordFacts` has no `LevelUp` row and there is no `Macros.levelUp`, so the three leveler witnesses spell "level up {cost}" as the ability it represents; add the row (the facts-from-ron generator's stub if one exists) and the macro, rewrite the three witnesses through it.

Done when: every constructor and proof named above is gone from the tree and
`grep` finds no residue; the collapsed families each have a bench witness for
both former spellings ("the last chosen" and "the chosen", a `Lookback` in
each of its three sorts, a singular and a plural possessor);
`StatOf Power (target land)` is refused and pinned, probed non-vacuous, while
an animated land still reads its power as a witness; `check-eq-indexes` passes
with no number in it; every renamed constructor's pins are re-spelled and
still refute; a scry card reads through the single `LookReq`; the build is
44/44 with 0 errors and 0 warnings. Standard constraints apply, plus the
RON-shaped constraint: a core constructor is admissible only if the RON
re-emitter can produce it from a RON node, and a macro only if it names a RON
macro (`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).

## As landed

- **F18** — `idris/scripts/check-eq-indexes` keeps the index-based-`Eq` check
  and prints the count it found; the hard-coded 47 is gone. The tripwire was
  already stale: the tree has 42 `Eq` instances.
- **F6** — deleted `kindLteComm`, `kindLteAssocR`, `kindLteAssocL`,
  `sameKindRefl`, `sameCardTypeEq`, `sameQEq` and their helpers `cardTypeAt`,
  `cardTypeAtIx`, `qualityAt`, `qualityAtIx`, `natEqSo` from `Words.idr`
  (102 lines). `kindLteRefl`, `kindLteJoinL`/`R`, `kindLteInL`/`R`,
  `natEqRefl`, `sameQRefl` and `sameLetterRefl` kept.
- **F15** — three of the five deleted, two kept:
  - deleted `Effect.AsThough.AsThoughLess` (no supported card says "as though
    its power/toughness were N less"; `AsThoughGreater` covers the printed
    "as though its power were 2 greater" family), with its
    `ProofsPiles.lessAsThoughCondition` exercise;
  - deleted `Effect.CostShift.CostShiftRunWithFloor` (no supported card pairs
    a mana-run cost shift with a floor), with `ProofsPiles.manaRunReductionFloor`;
  - deleted `Effect.Repetition.AgainExcept` (Forgotten Lore and Shrouded Lore,
    the only "repeat this process except" cards, are `AgainExcludingChosen`),
    with `ProofsPiles.repeatWithIndependentException`;
  - **kept `Triggers.GameEvent.Triggers`** — the review's "zero uses" is stale:
    four uses landed since (`Cards/Static.idr` and `Cards/Trigger.idr` bench
    cards under `TriggersAdditionally`, plus `ProofsTrigger` and
    `ProofsCounters` pins);
  - **kept `Words.AttachWord.Fortified` and landed its card**: `Darksteel
    Garrison` (`Cards/Cost.idr`), which spells "Fortified land has
    indestructible", the fortified-land tap trigger and `Fortify {3}`. The
    `Fortify` `keywordFacts` row the ticket says is missing already exists
    (`FactsGen.idr`). Its two pins `ProofsStatic.badFortifiedCreature` and
    `ProofsFaces.badFortifiedCreatureNoun` keep refuting and are now
    non-vacuous against a landed positive.
- **F17** — `Words.Characteristic` → `Stat`, `Words.ProjAxis.CharAxis` →
  `StatAxis`, and the index function `characteristicIx` → `statIx` for
  coherence, across `src/Experimental/` only. `Semantics.idr`'s own
  `namespace Characteristic` (the v1 model's full CR characteristic set) is
  untouched. The `Characteristics` record, `ACharacteristic`,
  `ComparesCharacteristic` and the pin names that read "characteristic" keep
  their spellings — the ticket names the sort and the axis only.
- **F8** — five overloaded pairs disambiguated, renaming the member whose
  sibling constructors already read that way:
  `Effect.Repetition.Until` → `UntilCond` (`Triggers.Duration.Until` keeps the
  name; the repetition constructor had no use outside its declaration),
  `Words.CounterKind.Named` → `NamedCounter` (125 sites; `Phrase.Predicate.Named`
  keeps the name — it takes a `NameSource`, the counter one a label string),
  `Effect.Static.StaticSpec.Define` → `DefinesLetter` (19 sites, beside its
  `DefinesPt` neighbour; `Effect.Instruction.Define` keeps the imperative
  spelling), `Phrase.LifeOp.Up`/`Down` → `LifeUp`/`LifeDown` (19 sites;
  `Words.Counter.Delta.Up`/`Down` keep), `Phrase.ZoneScope.Bare` → `BareScope`
  (35 sites; `Words.Reach.Bare` keeps). The sixth pair (`StaticFirstDone`) was
  already gone. Every pin over a renamed constructor is re-spelled and still
  refutes (whole-tree build).
- **F12** — `Phrase.Noun.PossessorsOf` folded into `PossessorOf ax n`; the
  `one` gate is gone from the constructor and from `Macros.controllerOf`/
  `ownerOf`, `nounPlur (PossessorOf _ n) = nounPlur n`, and `nounDelta` binds
  the possessor at the noun's plurality. `Effect.CtrlOverrideOk.PerMemberController`
  re-indexes on `PossessorOf`. Bench witnesses for both spellings: singular
  `Macros.controllerOf`/`ownerOf` (widely used, e.g. `Cards/Turn.idr`
  Kismet-style statics), plural `openTheVaults` (`Cards/Anaphora.idr`, "under
  their owners' control"), now reading through `Macros.ownerOf`.
  `ProofsChoice.badGroupOwner` refuted exactly the retired gate, so it is
  re-spelled as the positive `okGroupOwners` ("Their owners each lose 1 life"),
  which is what the fold deliberately admits; `badGroupPower` (the `StatOf`
  `one` gate) is untouched and still refutes.
- **F14** — two of the three folded; the third is a STOP.
  - `Cost.ScaledMana` deleted from the core; the readable spelling survives as
    `Macros.scaledMana unit amt = ScaledCost (Mana <run of unit>) amt`
    (`GenericUnit` ↦ `Mana [generic 1]`, `RunUnit run` ↦ `Mana run`). All 15
    bench sites and the three `ProofsMana` terms now read through the macro,
    keeping the same `ForEachAmount` gate, so `badLiteralScaledMana` and
    `badBareCountScaledMana` refute the same thing at the same slot.
    `ScaledCost`, which the review recorded as unused, in fact already had one
    use (`Macros.cumulativeUpkeep`).
  - `Effect.RemoveCountersAmong` deleted; both bench sites (Galloping Lizrog,
    Novijen Sages) are `RemoveCounters (Just q) kind (Macros.among grp)`, with
    one new macro `among grp = SomeOf WholeSlice Nothing grp` carrying the
    printed "from among" partitive (its `PartitiveBase` gate is the one the
    deleted constructor carried).
  - **STOP — `Effect.SkipsAllOf` is NOT `Continuously (Skips …) (Just RestOfGame)`.**
    Both bench cards read "skips all combat phases **of their next turn**"
    (Empty City Ruse, False Peace — `data/derived/cards.jsonl`), not for the
    rest of the game, so the proposed fold restates the printed duration. The
    duration the grammar does have for it, `Duration.DuringNextTurnOf who`,
    cannot be reached from the span slot: `Continuously se span` types the span
    at `staticIntro se`, and `staticIntro (Skips _ _) = bs`, so the span cannot
    re-read the skipping player and would have to announce a second target.
    `SkipsAllOf` is kept unchanged. Resolving this needs either a
    `staticIntro (Skips who _) = nomIntro who` change (which moves every
    existing `Static (Skips …)` bench card) or a duration that re-reads the
    static's subject — neither is in this ticket.
- **F5** — `Phrase.Amount.StatOf` gains `{auto 0 ty : So (statHeadTysOk c
  (nounHeadTys n))}`. `Words.statHeadTysOk c alts` is permissive when
  `comparedType c` is `Nothing` (mana value) and otherwise asks every
  head-type alternative to carry the stat's type. **Deviation from the
  ticket's letter**: the gate reads `nounHeadTys` (the alternatives already
  computed for damage), not `nounTy`. `nounTy (And [land, creature])` is
  `seedTyAll`, which returns the FIRST type — so a `nounTy` gate would refuse
  an animated land whenever `land` is written before `creature`, and admit it
  when written after. The head-type form is order-independent, keeps the
  `[]`-is-unknown permissiveness the ticket asks for, and is the same function
  F1 will use. Pins in `ProofsDescription`: `badLandPower` ("the power of
  target land", [CR#208.3]) and `badBattleLoyalty` ("the loyalty of target
  battle", [CR#209.1,210.1]), both probed non-vacuous (creature / planeswalker in the
  same slot each turns "not a valid impossible case"); positive twin
  `okAnimatedLandPower` = `StatOf Power (target (And [land, creature]))`, the
  animated land, kept beside them. `Macros.dealsDamageOwnPower` threads the
  obligation to its caller. Two `ProofsAnaphora` witnesses named an artifact
  and read its power — the anaphora subject is unchanged, so they are
  re-spelled with a creature (`okItReadsTheOnlyBareSingular`,
  `badOwnTwoInDelta`). `Defense` on `Stat` was declined (optional in the
  ticket; no supported card compares it).
- **F10** — `Words.ChoiceRef = TheChoice | TheLatestChoice` with
  `choiceRefOk TheChoice n = n = 1` and `choiceRefOk TheLatestChoice n =
  ChoiceStands n` (plus `choiceRefIx` and an index-based `Eq`). Four
  constructors survive, each taking the ref positionally:
  `Predicate.ChosenPlayer`, `Predicate.OfChosen`, `ColorTerm.ThatColor`,
  `Amount.ChosenNumber`; `TheLastChosenPlayer`, `OfTheLastChosen`,
  `TheLastChosenColor` and `TheLastChosenNumber` are gone. Both printed lemmas
  keep their wording as macros — `chosenPlayer`/`theLastChosenPlayer`,
  `ofChosen`/`ofTheLastChosen`, `thatColor`/`theLastChosenColor`,
  `chosenNumber`/`theLastChosenNumber` — and every bench and pin site reads
  through them (~130 sites). Bench witnesses for both spellings survive, e.g.
  `Cards/Turn.idr` (`theLastChosenNumber`), `Cards/Choice.idr`
  (`ofTheLastChosen CardName`), `Cards/Static.idr` (`ofChosen`),
  `Cards/Mana.idr` (`chosenNumber`, `thatColor`).
- **F11** — `Phrase.LookbackClause bs k` (constructor `MkLookback ev w what`
  with the `ComplementWritten`/`LookbackSubject` gates, plus `lbEvent`/
  `lbWindow`) now carries the clause the three sorts shared.
  `Predicate.HappenedTo lb`, `Condition.Happened who lb` and
  `Amount.EventTally op who lb {qm : So (tallyOk op (lbEvent lb))}`; 50 direct
  bench and pin sites re-spelled, and the ten `Macros.happened*`/`eventCount*`
  bodies build the clause. **Deviation**: the record is a single-constructor
  `data`, not a `record`, and the ten macro *signatures* are unchanged — they
  are the printed lemmas and every bench card reaches the family through them,
  so only their bodies shrink. Bench witnesses in all three sorts survive:
  `Cards/Damage.idr` and `Cards/Faces.idr` (`HappenedTo`), `Cards/Trigger.idr`
  and `Cards/Keyword.idr` (`Happened`), `Cards/Counters.idr` (`EventTally`).
- **Ruling 8 (`Macros.LookReq`)** — partially closed, with a recorded limit.
  `LookReq`'s four proof-carrying cases are now two, `OneLookReq` and
  `ManyLookReq` (the scry-1 and scry-N printed sentences, which the ticket
  keeps expanded), and the You/anyone-else axis moved into a nested two-case
  witness `LookAgent agent amt` (`LookYou`, gated `agent = You`; `LookThem`,
  gated `nounIsYou agent = False` plus the two `PlayerW` counts). The
  agent-dependent gates are declared once instead of twice per cardinality,
  and `lookAgentTop`/`lookedAgentTop` replace the `lookedTop`/`agentLookedTop`
  pair inside the requirement. The two constructors are mutually exclusive by
  their gates, so `scry`/`surveil` stay single-solution (whole-tree build, and
  every existing site — `Macros.scry You (Lit 1)`, `Macros.scry You (Lit 2)`,
  `Macros.scry They (Lit 1)`, `Macros.scry (Macros.target AnyPlayer) (Lit 3)`,
  `Macros.surveil You (Lit 1)`/`(Lit 2)` — still elaborates).
  **STOP — the bodies do not collapse to one clause.** `scryBody`/`surveilBody`
  still need four clauses each, because they must match `la` to a constructor:
  `may`'s decider is typed `Instruction (agentIntro decider)`, and `move`'s
  `Movable` search needs concrete bindings, so a stuck `lookAgentWho la` /
  `lookAgentTop amt la` leaves both goals unreducible (`Can't find an
  implementation for Movable (SomeOf (CountedSlice (Range Nothing Nothing))
  Nothing (Pro Bare ManyOf))`, seen when the many case was written
  agent-generically). The `Own`-style re-read the ticket proposes cannot serve
  both branches: `nounDelta You = []`, so `You` introduces no binding for an
  `Own`-style constructor to reach. Closing it fully needs the review's R6
  grammar-level fix — a core `Noun bs Player` "same as the agent" read, or an
  `agentIntro` that binds the agent uniformly — which this ticket does not
  authorise.
- **Levelers residue** — the `LevelUp` `keywordFacts` row the ticket says is
  missing already exists (`FactsGen.idr`, `paramShapes := [CostParam]`,
  `paidCost := True`), added by the facts-from-ron generator. Added
  `Macros.levelUp c = KeywordAbility "LevelUp" (Just (ParamCost c)) Nothing`
  ("level up [cost]" [CR#702.87a]) and rewrote the three leveler witnesses —
  Brimstone Mage, Student of Warfare, Kargan Dragonlord (`Cards/Faces.idr`) —
  through it. They had spelled the ability the keyword represents
  (`activatedOnlyDuring <cost> (PutCounters 1 (PrintedKind (NamedCounter
  "Level")) thisCreature) AsSorcery`); they now read as printed.
- **Turn-parts residue** — `Triggers.Timing.BeforeAttackersDeclared w` folded
  into `BeforePart (p : TurnPart) (w : Maybe (Noun bs Player))`, gated by
  `Words.beforePartOk` (every part but `Turn`: a turn is made of its phases
  [CR#500.1], so a point before the turn one is taking is not a window inside
  it). Three sites re-spelled as `BeforePart DeclareAttackers`. Bench witness
  for the new reading: **Angus Mackenzie** (`Cards/Cost.idr`), "{G}{W}{U},
  {T}: Prevent all combat damage that would be dealt this turn. Activate only
  before the combat damage step." Pin `ProofsMana.badBeforeTheTurn` with twin
  `okBeforeCombatDamage`, probed non-vacuous (`Upkeep` in the same slot turns
  "not a valid impossible case").
  **Not done: Berserk.** Berserk and Blood Frenzy say "**Cast** this spell only
  before the combat damage step", and `Timing` is reachable only from
  `Effect.AbilityAt.Activated`; `Card`/`Spell` has no cast-timing slot at all,
  so the spell half of the residue needs a new slot on the spell ability, which
  this ticket does not name. Angus Mackenzie is the activated-ability instance
  the same residue lists, and it is benched.
- **Copies-are-spells residue** — no code change: the residue records a ruling
  for the next card that needs it (stamp the copy's provenance or use an
  `Other`-style exclusion, never a new `Reach`), and states that no bench card
  needs it yet. Confirmed still true — `Cards/Copy.idr` reads the copy, never
  the original.

## Landing record

Measured on change `pyrzxqrm` (working copy at the time of writing), against
parent `kszvoonz` (`kata: claim workbench-fused-variants-3`).

**Numbers before → after**

| | before | after |
| --- | --- | --- |
| modules in `mtg.ipkg` | 46 | 46 |
| core constructors (`Words`, `Phrase`, `Effect`, `Triggers`, `Card`, `Events`) | 800 | 791 |
| `Unspellable` pins | 606 | 608 |
| `: Card` bench witnesses | 801 | 803 |
| `Eq` instances (`check-eq-indexes`) | 42 (asserted 47) | 43 (no assertion) |
| diffstat | — | 34 files, 989 insertions, 843 deletions |

Nine core constructors net: 11 deleted or folded away (`AsThoughLess`,
`CostShiftRunWithFloor`, `AgainExcept`, `PossessorsOf`, `ScaledMana`,
`RemoveCountersAmong`, `TheLastChosenPlayer`, `OfTheLastChosen`,
`TheLastChosenColor`, `TheLastChosenNumber`, and `HappenedTo`/`Happened`/
`EventTally`'s five shared fields collapsing into `MkLookback`), against two
added (`Words.ChoiceRef`'s `TheChoice`/`TheLatestChoice`) plus
`Phrase.LookbackClause.MkLookback` and `Macros.LookAgent`'s two cases.
102 lines of dead proofs left `Words.idr`.

**Gate lines**

- `cd idris && rm -rf build && ./scripts/build` → `46/46: Building Cards
  (src/Cards.idr)`, exit 0, 1 m 22 s wall; a second run greps 0 lines matching
  `warning` (case-insensitive).
- `idris/scripts/check-eq-indexes` → `Eq instances checked: 43 index-based or
  derived`.
- `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` → `checked 14225 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `cargo xtask cite bless` → `blessed 1420 rules at cr_date 2026-08-07`,
  newly registering [CR#702.87a] (read against `Macros.levelUp`; it is the rule
  that defines "level up [cost]"). No entries pruned.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` → `audited 11
  citation site(s)`. Each read against its claim; one was corrected in the
  process — `badBattleLoyalty` first cited [CR#109.3] (the characteristic
  list, which does not say which objects have which), and now cites
  [CR#209.1] (loyalty is printed on planeswalkers) and [CR#210.1] (a battle
  has defense).

**Assurance counts**

- restored: 0 (nothing was failing at the start; the tree was green at 46/46).
- re-spelled: 5 named, plus the mechanical sweeps.
  - `ProofsChoice.badGroupOwner` → `okGroupOwners` (its subject, the
    `PossessorOf` `one` gate, is what F12 retires; the sentence it refused is
    now admitted deliberately, so it becomes the positive of the same card).
  - `ProofsAnaphora.okItReadsTheOnlyBareSingular` and `badOwnTwoInDelta`:
    artifact → creature, so the F5 stat gate is not what they trip on; their
    anaphora subjects are unchanged.
  - `ProofsMana.okForEachScaledMana`, `badLiteralScaledMana`,
    `badBareCountScaledMana`: same slot, same `ForEachAmount` gate, now
    through `Macros.scaledMana`.
  - Mechanical sweeps that preserve every subject: ~130 F10 sites, 50 F11
    sites, 125 `NamedCounter`, 35 `BareScope`, 19 `LifeUp`/`LifeDown`, 19
    `DefinesLetter`, 15 `Macros.scaledMana`, 3 `BeforePart`, 2 `Macros.among`,
    3 leveler witnesses.
- ignored with a blocker: 0 (`#[ignore]` has no Idris analogue; no `failing`
  block was disabled).
- added: 3 pins (`badLandPower`, `badBattleLoyalty`, `badBeforeTheTurn`), 3
  positive twins (`okAnimatedLandPower`, `okBeforeCombatDamage`,
  `okGroupOwners`), 2 bench cards (Darksteel Garrison, Angus Mackenzie).
- removed: 4, each named.
  - `ProofsPiles.lessAsThoughCondition`, `manaRunReductionFloor`,
    `repeatWithIndependentException` — bare positive exercises of the three
    constructors F15 deletes. They are not refusals, so no refutation is lost;
    with the constructor gone there is no replacement shape to re-spell them
    against.
  - `ProofsChoice.badGroupOwner` — counted as removed only in the sense that
    the refusal is gone; the definition survives as `okGroupOwners`, see
    "re-spelled".
- pin non-vacuity probes run (mis-state, watch the message change): 3 of 3 new
  pins. `badLandPower` (land → creature), `badBattleLoyalty` (battle →
  planeswalker), `badBeforeTheTurn` (`Turn` → `Upkeep`) each turn into
  `… Oh is not a valid impossible case.`

**Deviations and additions**

1. **F5 gate reads `nounHeadTys`, not `nounTy`** (the ticket's letter).
   `nounTy (And [land, creature])` is `seedTyAll`, which returns the first
   type, so a `nounTy` gate is order-sensitive: it would refuse an animated
   land written `And [land, creature]` and admit `And [creature, land]`. The
   head-type alternatives keep `[]`-is-unknown permissiveness and are the same
   function F1 will use for damage. Rationale in the F5 entry above.
2. **F5's `Defense` on `Stat` declined** — the ticket marks it optional and no
   supported card compares it.
3. **F11 uses a single-constructor `data`, not a `record`**, and the ten
   `Macros.happened*`/`eventCount*` signatures are unchanged (only their
   bodies build the clause). They are the printed lemmas the bench reaches the
   family through.
4. **F15: two of the five constructors kept.** `Triggers.GameEvent.Triggers`
   has four uses that landed after the review was written; `Words.AttachWord.Fortified`
   is kept and its card landed (Darksteel Garrison) rather than deleted, which
   also keeps its two pins non-vacuous.
5. **F17 also renames `characteristicIx` → `statIx`** (coherence with the
   sort); `Characteristics`, `ACharacteristic` and `ComparesCharacteristic`
   keep their spellings.
6. **New macros beyond the ticket's letter**: `Macros.among` (F14c needs a
   name for `SomeOf WholeSlice Nothing grp`, the printed "from among"
   partitive), `Macros.scaledMana` (F14a, named), `Macros.levelUp` (levelers
   residue, named), and the eight F10 choice-reference spellings
   (`chosenPlayer`/`theLastChosenPlayer`, `ofChosen`/`ofTheLastChosen`,
   `thatColor`/`theLastChosenColor`, `chosenNumber`/`theLastChosenNumber`),
   which is what "the macro spelling keeps that wording" requires.
7. **New gate beyond the ticket's letter**: `Words.beforePartOk` on
   `Timing.BeforePart`, so the generic before-point cannot name `Turn`
   [CR#500.1].
8. **Helper deleted**: `Macros.lookAgentWho`, added while drafting ruling 8
   and left unused once the bodies had to match `la` — deleted rather than
   shipped dead.

**STOPs taken**

1. **F14b (`Effect.SkipsAllOf` → `Continuously (Skips …) (Just RestOfGame)`)** —
   not done. The fold restates the printed duration: Empty City Ruse and False
   Peace both say "skips all combat phases **of their next turn**". The
   duration the grammar has for it, `DuringNextTurnOf who`, is unreachable
   from `Continuously`'s span, which is typed at `staticIntro se` and
   `staticIntro (Skips _ _) = bs`. Resolution: `SkipsAllOf` kept unchanged;
   closing it needs either a `staticIntro (Skips who _) = nomIntro who` change
   (which moves every existing `Static (Skips …)` bench card) or a duration
   that re-reads the static's subject. Neither is in this ticket. Full detail
   in the F14 entry.
2. **Ruling 8 (`Macros.LookReq`)** — half done. `LookReq` is two constructors
   instead of four and the agent axis is a shared `LookAgent` witness, but
   `scryBody`/`surveilBody` still need four clauses each: `may`'s decider is
   typed `Instruction (agentIntro decider)` and `move`'s `Movable` search
   needs concrete bindings, so an agent-generic body leaves both goals stuck
   (observed as `Can't find an implementation for Movable (SomeOf
   (CountedSlice (Range Nothing Nothing)) Nothing (Pro Bare ManyOf))`). The
   `Own`-style re-read the ticket proposes cannot serve the `You` branch —
   `nounDelta You = []`, so there is no binding to reach. Resolution: landed
   the achievable half; a full collapse needs the review's R6 grammar-level
   fix (a core "same as the agent" `Noun bs Player`, or an `agentIntro` that
   binds the agent uniformly), which this ticket does not authorise.
3. **Turn-parts residue: Berserk not benched.** "Cast this spell only before
   the combat damage step" needs a cast-timing slot on the spell ability;
   `Triggers.Timing` is reachable only from `Effect.AbilityAt.Activated` and
   `Card`/`Spell` has no timing slot. Resolution: benched **Angus Mackenzie**,
   the activated-ability card the same residue names, which exercises
   `BeforePart CombatDamage` end to end. The spell-side slot is a separate
   shape the ticket does not name.

**Follow-ups for live tickets**

- A spell-level cast-timing slot (`Cast this spell only …`): Berserk, Blood
  Frenzy, Berserker's Frenzy, Master Warcraft, Rapid Fire.
- `Effect.SkipsAllOf`'s duration (F14b above).
- The `LookReq` body collapse (ruling 8 / review R6 above).
