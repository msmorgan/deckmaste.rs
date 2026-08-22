---
needs: []
---
# `costActionOk`: admit or cite every row

Ruling 2026-08-22. `costActionOk` (`idris/src/Experimental.idr`) still has ~35
`False` rows with no pin and no rule, left by
`workbench-pins-refuse-rules-impossibility-only` (its gap 10). The five
structural rows — `Continuously`, `InsteadOf`, `Delayed`, `HeldUntil`,
`Reflexively` are not actions a player takes [CR#118.1] — are pinned already.
For each remaining row: admit it (a cost is any action a player can take
[CR#118.1]; destroying, exiling, milling, drawing, revealing, putting counters,
dealing damage as a cost are all rules-meaningful), or refuse it with the rule
that makes it meaningless and a biting pin. Bench one printed card per newly
admitted action where one exists (`corpus`/`card`). Doctrine:
`docs/memory/rulings/measurements-live-in-pins.md`.

## Consumption boundary
`idris/src/Experimental.idr`, `idris/src/Experimental/Proofs*.idr`,
`idris/src/Experimental/Cards.idr`, `idris/src/Experimental/Macros.idr`.

## Acceptance
No `costActionOk` row is `False` without a rule and a pin; build PASS;
`Cards.idr` binds no implicits; cites 0/0. Standard constraints apply.

## As landed (2026-08-22)

`costActionOk` now reads as one rule: [CR#118.1] makes a cost an action a
player carries out, so every instruction a payer can follow is admitted, and
`costNounOk` — unchanged, and now reaching every row that names a noun — keeps
its own hold on which nouns a cost may name. Compound nodes delegate to their
bodies; only nodes that instruct nothing at payment are refused.

| action | verdict | rule | witness / pin |
|---|---|---|---|
| `DealDamage` | admitted (`costNounOk src`) | [CR#118.1] | — |
| `DoesntUntapNext` | admitted (`costNounOk n`) | [CR#701.43a] — exert is a player choosing to have a permanent not untap | — |
| `SkipsNext` | **refused** | [CR#614.10] a skip is a replacement effect, not an action [CR#118.1] | `badSkipAsCost` (new) |
| `ExtraTurn` | admitted (`costNounOk who`) | [CR#500.7] | — |
| `AdditionalPart` | admitted | [CR#500.8] | — |
| `Distribute` | admitted (`costNounOk among`) | [CR#601.2d] | — |
| `Fights` | admitted (`costNounOk a`) | [CR#118.1] | — |
| `SetStatus` (all eight values) | admitted (`costNounOk n`) | [CR#118.1] | "{1}{G}, {T}, Tap an untapped creature you control:" (already benched shape) |
| `GetsCounters`, `LosesAllCounters` | admitted (`costNounOk who`) | [CR#118.1] | — |
| `RemoveFromCombat`, `Regenerate` | admitted (`costNounOk n`) | [CR#118.1] | — |
| `CantBe` | delegates to its body | [CR#118.1] — the rider rides an action; the action decides | — |
| `GainsDesignation` | admitted (`costNounOk n`) | [CR#118.1] | — |
| `GameBecomes` | admitted | [CR#118.1] | — |
| `GameDrawn` | admitted | [CR#104.4c] — an effect may state the game is a draw, as `Concludes` already did | — |
| `CopyStack`, `ChooseNewTargets` | admitted (`costNounOk what`) | [CR#118.1] | — |
| `Choose` | admitted (`costNounOk n`) | [CR#118.1] | behold's expansion, "choose a creature you control or reveal a creature card from your hand" — unbenchable, see gaps |
| `AddMana` | admitted (`costNounOk who`) | [CR#118.1] | — |
| `Expose` (both verbs) | admitted (`costNounOk who`) | [CR#118.1] | opens `Does _ Scry` / `Does _ Surveil`, whose bodies are `Expose LookAt` |
| `Search` | admitted (`costNounOk who`) | [CR#601.2h] pays costs that move objects from the library to a public zone | — |
| `Shuffle` | admitted (`costNounOk whose`) | [CR#601.2h] pays costs that involve random elements | — |
| `Create` | admitted (`costNounOk agent`) | [CR#118.1] | — |
| `Composite` (all verbs) | delegates to its body | [CR#118.1] | the `Composite _ _ = False` catch-all was uninhabited — `NonAgentive` admits only `Destroy` and `Exile` |
| `Does` (all eight verbs) | delegates to its body | [CR#118.1] | **Void Maw** — `Does _ Put` was refused and the printed line is a bare cost |
| `Pay` | **refused** | [CR#602.1a] the activation cost is everything before the colon and its payer is fixed | `badPayAsCost` (new) |
| `May` | delegates to its body and both arms | [CR#118.8b] some additional costs are optional | **Gorex, the Tombshell** — unbenchable, see gaps |
| `If` | delegates to its body and else-arm | [CR#118.1] | — |
| `WhereLetter` | delegates to its body | [CR#118.1] | **Urgent Necropsy** — unbenchable, see gaps |
| `ForEachOf` | delegates to its body | [CR#118.1] | — |
| `Repeat` | **refused** | [CR#118.1] wants an action; "repeat" names one by anaphora, and [CR#601.2h] leaves no part preceding another | `badRepeatAsCost` (new) |
| `Sequentially` | **refused** | [CR#601.2h] pays a cost's parts in any order | `badSequentialCost` (new) |
| `Simultaneously` | **refused** | [CR#601.2h] pays them one at a time | `badSimultaneousCost` (new) |
| `Modal` | delegates to every mode | [CR#118.1] | **Bullseye, Death Dealer** |
| `Continuously`, `InsteadOf`, `Delayed`, `HeldUntil`, `Reflexively` | refused, unchanged | [CR#118.1] | `badContinuousAsCost`, `badInsteadAsCost`, `badDelayedAsCost`, `badHeldUntilAsCost`, `badReflexiveAsCost` |
| `Concludes`, `CounterSpell`, `ChangeLife`, `Draw`, `GetsEmblem`, `Move`, `PutCounters`, `RemoveCounters` | admitted already, untouched | — | — |

### Cards benched

- **Void Maw** — "Put a card exiled with this creature into its owner's
  graveyard: This creature gets +2/+2 until end of turn." A bare agentive
  placement as the whole activation cost; `Does _ Put` had refused it.
- **Bullseye, Death Dealer** — "{3}, {T}, Sacrifice an artifact or discard a
  nonland card: Bullseye deals 2 damage to any target." The cost the player
  chooses between, which `Modal` had refused.

### Model gaps listed

- **No additional-cost node.** `Cost` reaches the grammar through `Activated`,
  `AltCost`, `GatedBy`, `ParamCost` and `Pay` only; "As an additional cost to
  cast this spell, …" has no home. Three printed witnesses for rows admitted
  here therefore stay unbenched: **Gorex, the Tombshell** ("you may exile any
  number of creature cards from your graveyard") for `May`, **Urgent
  Necropsy** ("collect evidence X, where X is the total mana value of the
  permanents this spell targets") for `WhereLetter`, and **Final Payment**
  ("pay 5 life or sacrifice a creature or enchantment") as a second `Modal`.
- **Phyrexian Splicer** ("{2}, {T}, Choose flying, first strike, trample, or
  shadow:") is the printed `Choose` cost; choosing among a written list of
  keywords has no noun to choose.
