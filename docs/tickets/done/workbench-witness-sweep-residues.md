---
needs: []
---
**Close the shapes the witness sweep could not spell.** Residue of
`workbench-witness-mismatch-sweep` (2026-09-04), whose landing record names
eighteen STOPs with the exact goal each (its "Reference not as printed —
STOP" and "Dropped clause … — STOP" blocks). Three are core gaps; the rest
are single shapes. Work them as independent bullets, each re-spelling the
named witness to its printed text once the shape lands:

- **Own-stat damage off a `This` subject.** `DealDamage`'s amount is
  indexed at `nomIntro src`, empty for `This`, so "deals damage equal to
  its power" has no pronoun spelling (Karplusan Yeti). Index the amount at
  `selfSubjIntro src` or add an own-stat amount row.
- **"The exiled card."** `Phrase.uniquifies (ExiledWith _)` is `False`;
  decide whether imprint's exile uniquifies (at most one card) and add the
  row, or publish the imprint binding across the card's abilities (Drach'Nyen,
  Phyrexian Ingester, Chrome Mox).
- **`TriggerWord.After`** for the dice template ("After you roll a die…",
  Xenosquirrel's Shift); `ShiftResult` then reads the roll in scope.
- **Plural agent over an outer object.** Vaevictis Asmadi, the Dire: after
  the `ForEachOf` loop now publishes its group, "Those players sacrifice those
  permanents" is refused at `EachStackOk` — a distributive deed moving an
  object bound outside the agent phrase. Extend the `EachClosesOwnParts`
  case to a group the same loop published (residue of
  `workbench-foreach-group-survives`, 2026-09-04); bench the card whole.
- **The last two `Exchanged` arms.** `workbench-exchange-row` (2026-09-04)
  landed life totals, control, cards and zones; the numerical-values arm
  (Vedalken Squirrel-Whacker's "exchange one result with this creature's
  base power") and the text-box arm at the end of [CR#701.12] are not
  modelled. Add both as positional arms of `Exchanged`, bench the
  Squirrel-Whacker, pin an exchange of a value with itself.
- **Secretly chosen numbers.** `Predicate.ChoseExtreme` (Menacing Ogre,
  "each player who chose the highest number") carries no scope obligation;
  it is not a vote [CR#701.38c], so the vote gate does not apply and
  `ChosenNumber`'s singular gate does not transfer (the choice is plural:
  `countManys (Quality Number) bs = 1`). Add the plural-choice gate, pin
  the read with no choice in scope (residue of
  `workbench-vote-scope-and-order`, 2026-09-04). The ballot-label read on
  `VotesFor` stays open for the same reason the round recorded.
- The remaining fifteen as recorded: a condition's nouns joining the read
  stack (Answered Prayers), the `AttackedBy` attacker for "that opponent is
  attacking" (Tahngarth), an as-you-activate rider (Keeper of the Flame), a
  defending-player predicate (Iron Mastiff), "their controller" under a cost
  (War Tax), the `Exchange` row's use (Vedalken Squirrel-Whacker, after
  `workbench-exchange-row`), an ability-valued quality (Gabriel Angelfire),
  an agentless counter replacement (Pir), `searchZonesOf`'s hard-wired
  `exactly 1`, "all cards with the same name", "other" in a granted ability,
  and the rest named in the record.

Size: L (bulleted, independent). Done when: every named witness reads its
printed card in full or the record says which rule refuses it; pins probed;
build at its module count. Standard constraints apply, including the
RON-shaped constraint.

## Rulings 2026-09-04 (wayfinder session)

- Own-stat damage: index `DealDamage`'s amount at `selfSubjIntro src`; no
  own-stat amount row.
- "The exiled card": `ExiledWith This` uniquifies when the exiling ability's
  quantity is one, read off the card's own abilities; no cross-ability
  binding. The implementer reads the exile-with rule in `data/rules/cr.txt`
  and cites it; `research-imprint-uniqueness` supplies the corpus count.
- A condition's nouns do NOT join the read stack; "if … it …" reads the
  clause subject.
- Combat relations take a kind-polymorphic argument (`Player | Object` via
  `Joined`); no separate defending-player predicate.
- A cost under a deontic clause is typed at the enclosing clause's intro;
  `Cost` gets no agent index.
- The agentless counter replacement row takes a positional `Maybe` agent.
- `QualitySort.AbilityQ` with the chosen-quality read; not a `Modal`.
- `Exchanged` gains `Values a b` (two amounts) and `TextBoxes a b`; the
  text-box arm lands with a pin only if `research-exchange-textbox` finds no
  supported card.
- `EachClosesOwnParts` extends to parts the same `ForEachOf` published.
- "As you activate": a positional `Maybe` timing rider on the announced
  choice, not on the comparison.
- `SameNameAs n` through the `CardName` quality; `OtherThan` accepts the
  granting ability's subject binding. Two S shapes.

## As landed

- **Own-stat damage off a `This` subject.** `Effect.DealDamage`'s amount is
  indexed at `selfSubjIntro src` (was `nomIntro src`); the recipient stays at
  `amtDelta amt ++ nomIntro src`, so the subject's own binding is confined to
  the amount slot and is not published after the clause. `Macros.dealsDamageOwnPower`
  reads its `Top` window over `selfSubjDelta src ++ nounDelta src`.
  `Cards.Damage.karplusanYeti` and `ProofsDamage.okThatCreatureAfterDamage`
  now write "deals damage equal to its power" through the macro (was
  `StatOf Power Macros.thisCreature`, a re-description). Pins re-spelled:
  `ProofsAnaphora.badOwnEmptyDelta` (subject `This`, which publishes nothing —
  probe `Can't find an implementation for 0 = 1.`; twin
  `ownSurvivesSecondSingular`) and `ProofsCounters.badThreeArmHeaderReadback`
  (the header read is now `That (TypeW Creature) OneOf`, which a `SelfD`
  binding cannot answer — probe `countReach (Word (TypeW Creature)) OneOf
  (selfSubjIntro thisCreature) = 1`; twin `okAltHeaderAgreeingReadback`).
- **A condition's nouns and the read stack.** Confirmed: no core change. The
  tree already excludes a condition's nouns — `Phrase.condDelta` publishes
  only the condition's own subject (`nounDelta n ++ selfSubjDelta n` for
  `Matches`, `selfSubjDelta who` for `Happened`/`ChoseThisWay`), never
  `predDelta` of the described predicate. What was missing was a *scoped*
  read: new `Macros.itCondSubject c` reads `Pro Bare OneOf (Top (length
  (condDelta c)))` over `condIntro c`, the `Macros.itPrior` idiom.
  `Cards.Keyword.answeredPrayers` now writes the printed "it" through it (was
  a re-description as `thisEnchantment`). Twin/pin added in `ProofsAnaphora`:
  `okCondSubjectRead` and `badCondUnwindowedRead` (an unwindowed `It` after an
  earlier clause published a creature — probe `Can't find an implementation
  for countReach Bare OneOf (condIntro (NotCond (Matches thisEnchantment
  creature))) = 1.`).
- **Combat relations' kind-polymorphic argument.** `Phrase.combatRelOk
  AttackedBy` now gates on a new `Phrase.combatPartyKind` (`Player` always;
  `Object` on the battlefield; a `\/` join of both), replacing `km == Object &&
  zoneIsB z Battlefield` — the attacker of an attacked player, planeswalker or
  battle may be the attacking player [CR#506.2]. No defending-player predicate
  was added. `Cards.Choice.tahngarthChoosesDefender` is re-indexed at
  `eventIntro Trigger.tahngarthHeader` (so "that opponent" has its antecedent —
  `AttacksWith`'s attackers are indexed over `nomIntro who`) and now reads
  "choose a player or planeswalker that opponent is attacking";
  `Cards.Static.tahngarthAttacksThatJoin` still chains off it.
  `Cards.Description.ironMastiffIgnore` now reads "roll a d20 for each player
  being attacked" (`Macros.countOf (And [AnyPlayer, CombatRel AttackedBy
  TheAttackingPlayer])` as the die count). `Cards/Choice.idr` and
  `Cards/Static.idr` gained an `import Experimental.Cards.Trigger` (an earlier
  family, per the documented order). Twin/pin in `ProofsTurn`:
  `okAttackedByPlayer` and `badAttackedByGraveyardRelatum` (probe `Can't find
  an implementation for So False.`).
- **A cost under a deontic clause.** Confirmed: no core change. `Effect.Compulsion.GatedBy`
  already types its cost at `Effect.gatePayer :: selfSubjIntro n` — the enclosing
  clause's intro plus the payer — and `Cost` carries no agent index.
  `Cards.Description.warTaxScaledPayment` was a bare `Cost [letterB X]` fragment;
  it is now the whole printed sentence, `Instruction [letterB X]` =
  `Continuously (deontic (allOf creature) (GatedBy …) ["Attack"] Agent
  NoDeonticPatient) (Just ThisTurn)`, so "for each attacking creature they
  control" reads `HasPossessor ControllerAx They` off the payer. Twin/pin in
  `ProofsDeontic`: `okGatedCostReadsPayer` (and `okBareScaledCost`, the `Cost`-typed
  twin the pin lint requires) with `badBareCostReadsPayer` (probe `Can't find an
  implementation for countReach (Word PlayerW) OneOf (amtIntro (LetterVal X)) = 1.`).
- **The agentless counter replacement.** Confirmed: no core change.
  `Triggers.CounterEvent` already carries its agent as a positional
  `(by : Maybe (Noun bs Player))`, and `Macros.bareCounterEvent` is the agentless
  spelling. `Cards.Keyword.pirDistributive` now intercepts
  `bareCounterEvent CounterPut ManyCounters (a (And [Permanent, HasPossessor
  ControllerAx (PlayerGroup YourTeam)]))` — the printed "if one or more counters
  would be put on a permanent your team controls" — instead of the invented
  agent of `manyBareCountersPutBy You` (that macro keeps its three other, printed,
  users in `Cards/Counters.idr`). Twins/pin at the end of `ProofsCounters`:
  `okAgentlessCounterReplacement`, `okAgentlessCounterPutEvent` and
  `badCounterEventAgentAndEffect` (an event claiming both an agent and
  by-effect — probe `Can't find an implementation for So False.`).
- **`QualitySort.AbilityQ`.** `Words.QualitySort` gains `AbilityQ` (`qualityIx`
  5; `SubtypeQ t` renumbered to `6 + cardTypeIx t`), `chosenQualityReadOk
  AbilityQ = True` and `Effect.deckComparable AbilityQ = True` (abilities are a
  characteristic [CR#109.3], the rule the function's own comment applies).
  `Words.KeywordTerm` gains `TheKeywordWith k n` so "rampage 3" is a nameable
  option (`knownKeywordTerm` checks the keyword takes a `NumberParam`;
  `keywordTermBare` is `False`), plus `Words.allKnownKeywordTerms`.
  `Phrase.ChoiceDomain` gains `AbilitiesAmong (ks : List KeywordTerm)` at
  `QSort AbilityQ`, and `Effect.AbilityAt` gains the read
  `ThatAbility (ref : ChoiceRef)` gated on `countChoice (QSort AbilityQ) bs`
  (`grantableAb` True; not a card line, not an emblem ability, no regime).
  Not a `Modal`. `Cards.Keyword.gabrielAngelfire` now chooses among the four
  printed abilities and gains `ThatAbility TheChoice` until your next upkeep
  (was a hard-coded "Flying"). Twin/pin at the end of `ProofsKeyword`:
  `okThatAbilityAfterChoice` and `badThatAbilityWithoutChoice` (probe `Can't
  find an implementation for countChoice (QSort AbilityQ) [] = 1.`).
- **`EachClosesOwnParts` over a loop's parts.** `Effect.EachStackOk`'s
  `EachClosesOwnParts` obligation widens from `out = partsClosed outer` to
  `So (closesOwnParts outer out)`, where the new `Words.closesOwnParts` admits
  either the old parts-closure or `Words.spentDistributively outer out` — an
  in-place re-placement that keeps every binding's determiner, kind and
  plurality and moves only plural ones (one object per agent, never one the
  table shares [CR#701.21a]). "Those players sacrifice those permanents" now
  typechecks after the loop that published the permanents.
  `Cards.Anaphora.vaevictisAsmadiTheDire` is benched WHOLE (flying + the whole
  trigger, including "each player who sacrificed a permanent this way reveals
  the top card of their library, then puts it onto the battlefield if it's a
  permanent card"); the third sentence's "it" reads at the card slot
  (`Macros.itsACard` and the new noun macro `Macros.ItCard`), because the
  lookback complement "a permanent" is the other singular object in scope.
  Twin `ProofsChoice.okDistributedLoopParts`; the three existing pins
  (`badDistributedRestOfSharedGroup`, `badDistributedRestOfSingularChoice`,
  `ProofsZone.badDistributedZoneMoveRead`) still refuse — re-probed, each
  `Can't find an implementation for EachStackOk …`.
- **The as-you-activate rider.** `Effect.Choose` gains a positional
  `(when : Maybe (Concurrent bs))` after `disc` — the announcement's timing, not
  a condition on what may be chosen — and all 29 core call sites plus
  `Macros.choose`/`chooses`/`secretlyChooses`/`proliferate` pass `Nothing`. New
  macro `Macros.chooseWhile n w`. Spelling "this ability" needed a noun:
  `Words.MarkerWord` gains `AbilityMarker` (`markerZone` `Stack`),
  `Phrase.nounIsAbility (AsMarker AbilityMarker _) = True`, and
  `Macros.thisAbility = AsMarker AbilityMarker This`.
  `Cards.Damage.keeperOfTheFlame` now reads "…as you activate this ability"
  (`chooseWhile … (WhileDoing (Activates You thisAbility))`). Twin/pin at the
  end of `ProofsChoice`: `okChoiceWithActivationRider` and
  `badChoiceRiderNotUnderway` (a death is not an event a choice happens during
  — probe `Can't find an implementation for So False.`).
- **`SameNameAs` and `searchZonesOf`'s quantity slot.** `SameNameAs` needed no
  change: `Phrase.NameSource` already carries it beside `ChosenName`, the
  `CardName`-quality read, and all six uses (Eradicate among them) already write
  it. `Macros.searchZonesOf` gains a positional `(q : Quantity bs)` (was a
  hard-wired `exactly 1`); its three call sites pass their printed count.
  `Cards.Anaphora.eradicateSearch` now searches for a plural of the same-named
  cards and exiles them (`ItVerbed "Search" ManyOf`). STOP: the printed "all
  cards with the same name" reads `Macros.anyNumber` — `Phrase.Quantity` is
  `Range | UpToOf | ExactlyOf` and has no mandatory-"all" arm; "all" is a
  DETERMINER in this grammar (`AllDet`), so adding an `AllOf` quantity would be
  a second spelling of one meaning. Not taken; the exact quantity of a search
  wants its own ticket.
- **`OtherThan` over a granting ability's subject.** Confirmed: no core change.
  `Phrase.complementAnchorsOk` already accepts an anchor the enclosing ability
  announced — `OtherThan (TheVerbed "Exile" CardW Attributive OneOf)` typechecks
  under the {T} ability that exiled and granted to that card. Twin/pin at the end
  of `ProofsCounters`: `okOtherThanExiledByThisAbility` and
  `badBareOtherWithoutAnchor` (probe: the `otherAnchorOk` goal, `anyTargeted
  Object (publicOnly (costIntro TapSymbol))`). STOP: `Cards.Counters.alaundoTheSeer`
  is NOT re-spelled — writing "each other card you own in exile" as printed needs
  the whole {T} ability, whose granted ability's second sentence ("If you cast a
  creature spell this way, it gains haste until end of turn") has no readable
  antecedent: `condDelta (Happened who _)` publishes only the condition's own
  subject (`You`), never the lookback complement, so the printed "it" cannot be
  written. Everything else of that ability typechecks (probed).
- **`TriggerWord.After`.** `Events.TriggerWord` gains `After` (one cite line: it
  is the dice template's word; the body reads the event's outcome, which
  `Triggers.headerCtx` already supplies from `eventAfter`). No gate needed a new
  case. `Cards.Description.xenosquirrelsShift` is now Xenosquirrels' whole
  printed ability — `Triggered After (RollsDice You OneDie AnyDie AnyResult) …
  (May You (RemoveCounters …) (Just (shiftResult (Lit 1))) Nothing)` — replacing
  the invented `rollDice You 1 6`. Twin/pin at the end of `ProofsTrigger`:
  `okAfterRollShift` and `badShiftWithoutRoll` (probe `Can't find an
  implementation for countOutcomes RollResult (instrIntro (RemoveCounters …)) = 1.`).
- **The secret-number gate.** `Phrase.Predicate.ChoseExtreme` takes
  `{auto 0 nc : So (not (countManys (Quality Number) bs == Z))}` — the plural
  choice, the shape `workbench-vote-scope-and-order`'s STOP measured; it is not
  a vote [CR#701.38c], so no `VoteHeld` gate. `Cards.Counters.menacingOgre` is
  unchanged (it already chooses the numbers first) and now carries the
  obligation. Twin/pin at the end of `ProofsChoice`:
  `okChoseExtremeAfterNumbers` and `badChoseExtremeWithoutChoice` (probe `Can't
  find an implementation for So (not (equalNat (countManys (Quality Number) []) 0)).`).
  The ballot-label read on `VotesFor` stays open, as the ticket records.
- **"The exiled card."** Amended ruling followed (coordinator, 2026-09-04, off
  `docs/memory/scratch/2026-09-04-imprint-uniqueness.md`): the uniquification is
  the LINKED-ABILITY rule, not a quantity gate. `Phrase.uniquifies (ExiledWith _)`
  is now `True`, with a two-line cite: the read is linked to the exiling ability
  printed on the same object [CR#607.2a], and a singular reference still resolves
  when that ability exiled several cards [CR#607.3]. No quantity-one gate was
  added — the research file names three supported cards (Advanced Reconstruction,
  Rashmi and Ragavan, Reno and Rude) that read singular after a repeatable exile,
  and one would have refused them. `Cards.Keyword.drachNyen`,
  `Cards.Choice.phyrexianIngesterPump` and `Cards.Mana.chromeMoxMana` now read
  "the exiled card" (`Macros.the (ExiledWith …)`, was `Macros.a`). Twin/pin at the
  end of `ProofsZone`: `okTheExiledCard` and `badTheCardInExile` ("the card in
  exile", which nothing links — probe `Can't find an implementation for So False.`);
  the existing `badExiledWithOtherSource`/`badExiledWithDescribedSource` still
  refuse a source that is not this object, which is [CR#607.2a]'s link.
  STOP: the pin the amendment asks for — refusing "the exiled card" on a card with
  NO exiling ability — is not landed. It is a `Card`-level law of the
  `modalFrameOk` shape (`not (anyReadsExiledWith as) || anyExiles as`), and its
  first half needs a deep traversal of `Instruction`/`Noun`/`Predicate` looking for
  an `ExiledWith` anywhere in an ability; the tree has no such traversal (every
  existing card law matches only top-level ability shapes or reads published
  bindings), so writing one is its own ticket. The three witnesses' cards each do
  print the linked exiling ability.
- **The last two `Exchanged` arms.** `Effect.Exchanged` gains
  `Values (a : Amount bs) (b : Amount (amtIntro a))` [CR#701.12g] and
  `TextBoxes (a : Noun bs Object) (b : Noun (nomIntro a) Object)` [CR#701.12h],
  both positional, with `exchangedIntro`/`exchangedCostOk` rows. Two new gates in
  `Effect`: `settableValue` (a life total, a power or toughness, or a rolled
  result — the values an exchange can set) and `selfExchanged` (two halves naming
  one value). `Cards.Anaphora.vedalkenSquirrelWhackerReroll`'s replacement body is
  no longer a no-op: it rolls, then offers `Exchange (Values (TheOutcome
  RollResult) (StatOf Power thisCreature))`. The text-box arm has printed
  witnesses (research file: 2 supported cards), so it lands with a bench witness,
  not a pin alone: `Cards.Keyword.deadpoolTradingCard` is benched WHOLE (Deadpool,
  Trading Card — "As Deadpool enters, you may exchange his text box and another
  creature's", plus both other lines). Twins/pins at the end of `ProofsZone`:
  `okExchangeTwoValues` (Tree of Redemption's sentence), `badExchangeValueWithItself`
  (the ticket's asked-for pin), `badExchangeLiteralValue`, `okExchangeTextBoxes`
  and `badExchangeTextBoxInGraveyard` — each probed, `Can't find an implementation
  for So False.`
  STOP: Vedalken Squirrel-Whacker's printed "or base toughness" is not written —
  the second half of "one result with this creature's base power or base
  toughness" is a disjunction over an `Amount`, and `Amount` has no disjunction
  (nor a base-vs-current power distinction). The witness writes the power half.

## Landing record

Measured on change `nwpnnlrn` (working copy of
`.workspaces/workbench-witness-sweep-residues`), parent `ovkpmowx`
(`kata: claim workbench-witness-sweep-residues`).

**Numbers before → after**

| figure | before | after |
|---|---:|---:|
| `Cards/*.idr` top-level declarations | 1 626 | 1 628 |
| `Proofs*.idr` top-level declarations | 1 336 | 1 364 |
| `Unspellable` pin declarations, all `Proofs*.idr` | 620 | 633 |
| `Exchanged` arms | 4 | 6 |
| `QualitySort` constructors | 6 | 7 |
| `TriggerWord` constructors | 3 | 4 |
| `KeywordTerm` constructors | 2 | 3 |
| `MarkerWord` constructors | 4 | 5 |
| `Effect.Choose` positional slots | 4 | 5 |
| modules in the full gate | 46 | 46 |

The +2 bench declarations are `Cards.Anaphora.vaevictisAsmadiTheDire` and
`Cards.Keyword.deadpoolTradingCard`; nothing left the bench (the three removed
`Cards/*` signature lines are re-spellings whose type changed:
`xenosquirrelsShift`, `warTaxScaledPayment`, `tahngarthChoosesDefender`). The
+28 proof declarations are 15 twins and 13 pins, each named in the As landed.

Coverage lock, construction count, the permitted licensing-checker total and
the selection census are not applicable: the diff is `idris/`, this ticket and
`cr-citations.lock`, and touches no Rust crate and no `cargo xtask coverage`
input — so there is no coverage wall time and no per-byte thread-CPU line
either. The workbench gate's own figure is below.

**Gate artifacts**

- `cd idris && rm -rf build && ./scripts/build` — exit 0, last line
  `46/46: Building Cards (src/Cards.idr)`, 0 lines matching `Error` or
  `Warning`. Wall clock 68 s for the clean 46-module build.
- `cargo xtask cite check --list-noncompliant` — `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check` — `checked 14431 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless` — `blessed 1432 rules`; newly registered
  [CR#701.12g] and [CR#701.12h], nothing pruned.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` — `audited 24 citation site(s)`, each rule read against
  its claim: [CR#506.2] ×2 (the active player is the attacking player, so an
  attacked player's attacker may be a player), [CR#109.3] (abilities are a
  characteristic), [CR#701.21a] ×2 (a sacrifice is by the permanent's own
  controller, so a plural agent's deed is one object per agent),
  [CR#701.38c] ×2 (a "vote" reference means an actual vote), [CR#607.2a] ×3
  and [CR#607.3] ×2 (linked abilities; a singular "the exiled card" resolves
  after a multi-card exile), [CR#701.12g] ×5 and [CR#701.12h] ×2 (numerical
  values, text boxes). No citation was found pointing the wrong way.

**Assurance counts**

- restored: 0 — nothing was failing at the start of the round.
- re-spelled: 17 — 14 bench witnesses to their printed text
  (`karplusanYeti`, `answeredPrayers`, `tahngarthChoosesDefender`,
  `ironMastiffIgnore`, `warTaxScaledPayment`, `pirDistributive`,
  `gabrielAngelfire`, `keeperOfTheFlame`, `eradicateSearch`,
  `xenosquirrelsShift`, `vedalkenSquirrelWhackerReroll`, `drachNyen`,
  `phyrexianIngesterPump`, `chromeMoxMana`), 1 pin twin
  (`ProofsDamage.okThatCreatureAfterDamage`) and 2 pins whose subject this
  round deliberately made writable (`ProofsAnaphora.badOwnEmptyDelta`,
  `ProofsCounters.badThreeArmHeaderReadback`) — both re-spelled to the same
  asserted refusal, neither deleted.
- ignored with a blocker: 0. The Idris gate has no skip.
- added: 35 — 28 `Proofs*` declarations (15 twins, 13 pins), 2 whole bench
  cards, 4 macros (`itCondSubject`, `chooseWhile`, `ItCard`, `thisAbility`)
  and 1 macro slot (`searchZonesOf`'s quantity).
- removed: 0.
- Every pin added or re-spelled was probed non-vacuous by restating it as an
  ordinary definition in a scratch module and reading the message; each message
  is quoted beside its pin in the As landed.

**Deviations and additions**

1. `Effect.DealDamage`'s recipient is indexed at `amtDelta amt ++ nomIntro src`,
   not at `amtIntro amt`, so the subject's own binding is confined to the amount
   slot and is not published after the clause. Without that, Karplusan Yeti's
   second sentence ("that creature deals damage equal to its power") loses its
   `It` read to a two-candidate ambiguity. The ruling names only the amount's
   index; this is what makes that index safe.
2. `Cards/Choice.idr` and `Cards/Static.idr` gained
   `import Experimental.Cards.Trigger` so `tahngarthChoosesDefender` can be
   indexed at its own trigger header. Trigger is earlier in the documented
   family order, so the rule the split states is respected.
3. `Effect.EachClosesOwnParts`'s obligation changed from the propositional
   `out = partsClosed outer` to `So (closesOwnParts outer out)`, a disjunction
   with the new `Words.spentDistributively`. Two constructors, not three, so the
   three existing pins keep their two `impossible` clauses; all three were
   re-probed and still refuse.
4. `Words.KeywordTerm` gained `TheKeywordWith k n` and `Words.MarkerWord` gained
   `AbilityMarker`. Neither is named in the rulings; both are the vocabulary the
   named shapes need ("rampage 3" as a choosable ability, "this ability" as the
   as-you-activate rider's patient), and neither adds a second spelling of an
   existing meaning.
5. `Effect.deckComparable AbilityQ = True`. The function's own comment gives the
   criterion ("a number and a counter kind are not characteristics"); abilities
   are a characteristic [CR#109.3], so `False` would have contradicted it.
6. The exchange round asked for one pin ("an exchange of a value with itself");
   three landed, because `settableValue` refuses a second thing worth pinning (a
   literal half) and the text-box arm has its own zone gate.
7. `Cards.Keyword.deadpoolTradingCard` is a new whole-card bench witness rather
   than a re-spelling: the text-box arm had no witness at all, and the research
   file names two supported cards. Exchange of Words, the other one, exchanges
   the text boxes of a PLURAL ("those creatures"), which the two-noun arm cannot
   say.

**STOP**

1. **Alaundo the Seer is not re-spelled.** `OtherThan` over the granting
   ability's subject is confirmed working (twin `okOtherThanExiledByThisAbility`),
   but writing "each other card you own in exile" as printed needs the whole {T}
   ability, and its granted ability's second sentence ("If you cast a creature
   spell this way, it gains haste until end of turn") has no readable antecedent:
   `Phrase.condDelta (Happened who _)` publishes only the condition's own subject
   (`You`), never the lookback complement, so the printed "it" cannot be written.
   Every other clause of that ability typechecks (probed).
2. **"All cards with the same name" reads `Macros.anyNumber`.**
   `Phrase.Quantity` is `Range | UpToOf | ExactlyOf`; there is no mandatory-"all"
   arm, and "all" is a DETERMINER in this grammar (`AllDet`), so adding an
   `AllOf` quantity would be a second spelling of one meaning. `searchZonesOf`
   now takes the quantity slot the STOP asked for; the exact count of a search
   wants its own ticket.
3. **The card-level linked-exile law is not landed.** The amendment asks for a
   pin refusing "the exiled card" on a card with NO exiling ability. That is a
   `Card` law of the `modalFrameOk` shape, and its "does any ability read
   `ExiledWith`" half needs a deep traversal of `Instruction`/`Noun`/`Predicate`;
   every existing card law matches only top-level ability shapes or reads
   published bindings, so the traversal is its own ticket.
4. **Vedalken Squirrel-Whacker's "or base toughness" is not written.** The second
   half of "one result with this creature's base power or base toughness" is a
   disjunction over an `Amount`, and `Amount` has no disjunction (nor a
   base-vs-current power distinction). The witness writes the power half.
5. **Tahngarth, First Mate is still benched in three pieces.** The defender
   restriction is now writable and written; the ability's first sentence ("you
   may have that opponent gain control of Tahngarth until end of combat") and its
   "If you do" are still off the witness, as they were before this round.
