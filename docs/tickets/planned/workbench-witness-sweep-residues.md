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
