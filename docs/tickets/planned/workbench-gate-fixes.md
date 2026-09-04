---
needs: []
---
**Close six independent core gates: attachment, the storm window, nested tap
costs, `SameAs` counters, three facts/expansion mismatches, and the open cost
predicates.** Cleanroom review 3, 2026-09-04, findings U4, U6, U7, U8, U13 and
D-Q14. Six independent edits.

- **U4 — `Words.attachedCheckOk` is constantly `True`.** Every row
  (`Enchanted`, `Equipped`, `Fortified`) returns `True`, so
  `Phrase.Predicate.IsAttached`/`AttachedBy`'s obligation proves nothing and
  `IsAttached Fortified : Predicate [] Object` admits any host.
  `Words.attachHeadOk` already encodes the rule: an Equipment can't legally be
  attached to anything that isn't a creature [CR#301.5], a Fortification to
  anything that isn't a land [CR#301.6]. Either restate the two obligations
  over `attachHeadOk` against the described noun's head, or delete
  `attachedCheckOk` and the obligations; pin a Fortification on a creature.
- **U6 — `Macros.stormExpansion` counts the wrong window.** It counts
  `SpellCast … ThisTurn`, but storm copies the spell for each other spell that
  was cast *before it* this turn [CR#702.40a], so spells cast in response to
  the trigger are counted today. Add `Words.Lookback.EarlierThisTurn` with its
  rows in `sameWindow` and `Events.sameLookback`, use it in the expansion, and
  pin `ThisTurn` in the storm body.
- **U7 — a nested `Compound` evades the one-tap gate.**
  `Effect.selfTapPayment (Compound _) = False`, so `costTapOnce` counts only
  top-level tap symbols; `ProofsMana.nestedCompoundCost` is an *admitted*
  activation cost carrying two `{T}`, one nesting level below the
  `badDoubleTapCost` pin. A permanent that's already tapped can't be tapped
  again to pay the cost [CR#107.5]. Count recursively.
- **U8 — `Effect.CounterKindSource.SameAs` bypasses scope and ignores the
  amount.** `counterSourceScope (SameAs _) k = counterHolderKind k` holds for
  any object or player, so `PutCounters (Lit 1) (SameAs thisCreature) (target
  AnyPlayer)` is admitted — a creature's `+1/+1` counters put on a player,
  which `ProofsCounters.badGetsBoostCounter` refuses for `PrintedKind`, since
  a `+X/+Y` counter modifies an object's power and toughness [CR#122.1a]. And
  `PutCounters (Lit 7) (SameAs …)` is admitted although "the same number"
  carries no literal. `SameAs` inherits the source noun's scope; the amount
  slot is absent, or pinned, for `SameAs` and `ThoseKinds`.
- **U13 — three mismatches of the same class.** `Macros.fateseal` is
  `Macros.scry` over the agent's own library, but to fateseal N is to look at
  the top N cards of *an opponent's* library [CR#701.29a], so
  `Cards.Keyword.spinIntoMyth` reads "an opponent scries 2"; give fateseal its
  own expansion over the `"Fateseal"` label with a library-possessor slot.
  `Macros.monstrosity` narrows [CR#701.37a]'s "this permanent" to
  `thisCreature`. The generated `FactsGen` row for `SplitSecond` leaves
  `functionsOnStack` `False` although split second functions while the spell
  is on the stack [CR#702.61a] — fix the generator's source, not the generated
  file.
- **D-Q14 — the cost predicates close silently.** `Effect.costActionOk`,
  `costPaidByYou`, `costOffBattlefield` and `costTapOnce` each end in
  `_ = False`, so a new `Instruction` constructor becomes "not a cost" without
  anyone noticing — the same shape U7 hides a nested `Compound` behind. Make
  them case-complete over `Instruction`, so a new row is a compile error.

Size: M. Done when: each of the six gates refuses its probe term with a
same-module positive twin; fateseal reads an opponent's library and
`spinIntoMyth` reads as printed; the `SplitSecond` row is regenerated from the
stub; the four cost predicates are case-complete; build at its module count.
Standard constraints apply, including the RON-shaped constraint.
