---
needs: []
---
# Unify action provenance (`castBy`, `castFrom`, `activatedBy`) and `-ThisWay` lookback conditions with the action-clause spine

With the `semantics-v3` Unified Action-Clause Spine (`does` / `would` / `did` /
`was` over `Subject` and `Action`), two families of bespoke action-provenance and
event-lookback constructors remain in `Semantics/Abilities.lean`:

1. **Object/spell/ability action-provenance predicates (`Abilities.lean:209-210, 264`):**
   - `Predicate.castBy (caster : NounPhrase) (rank : Option (Ordinal × Period))`
   - `Predicate.castFrom (zone : Location)`
   - `Predicate.activatedBy (activator : NounPhrase)`
   - Note that `castBy`'s `(rank : Option (Ordinal × Period))` (`"the second spell
     you cast each turn"`) also duplicates `GameEvent.nthOccurrence (ordinal : Ordinal)
     (per : Option TurnPart) (event : GameEvent)` (`Abilities.lean:454`, `"Whenever
     you cast your second spell each turn"`).
2. **One-off `-ThisWay` and flip-outcome `Condition` constructors (`Abilities.lean:424-428`):**
   - `Condition.choseThisWay (chooser : NounPhrase) (predicate : Predicate)`
     (*Sculpted Sunburst*, `Cards/Choice.lean:1494`: `"if you chose a creature
     this way"`) — directly expressible as
     `did chooser (choose (a predicate)) thisWay`.
   - `Condition.dealtThisWay (predicate : Predicate)` (*Screaming Nemesis* and
     *Sonic Shrieker*, `Cards/Damage.lean:744,758`: `"If a player was dealt damage
     this way"`) — expressible as `was (dealDamage (to := a predicate)) thisWay`
     once `Condition.did` publishes the recipient referent for `"that player"`.
   - `Condition.flipCalled (caller : NounPhrase) (call : FlipCall)` (*Goblin
     Archaeologist*, `Cards/Faces.lean:97,99`: `"If you win the flip"`) and
     `Condition.flipFace (face : CoinFace)` — expressible via `did`/`was` over
     `Action.flipCoins`, which already carries `(call : Option FlipCall)` and
     `(outcome : Option CoinFace)` (`Abilities.lean:1094`, matching *Chance
     Encounter* at `Cards/Faces.lean:107`).
   - `Condition.preventedFromSource (predicate : Predicate)` (*Shadowbane*,
     `Cards/Damage.lean:524`: `"If damage from a black source is prevented this
     way"`).

## Design Requirements

- Distinguish the three temporal regimes of `cast` / `activate` provenance
  without keeping separate `castBy` / `castFrom` / `activatedBy` constructors if
  `did` / `was` can carry them cleanly:
  1. **Proposed action** (e.g. `costShift (allOf (and [isArtifact, isSpell, ...]))`
     on *Foundry Inspector*, `Cards/Mana.lean:219`).
  2. **Current object incarnation** (e.g. `"when ~ enters, if you cast it from
     your hand"` on *Arcane Proxy* / *Mulldrifter*, `Cards/Faces.lean:604`,
     `Cards/Mana.lean:642`, or `"spend this mana only to cast a spell from your
     graveyard"`, `Cards/Mana.lean:324`).
  3. **Historical window** (e.g. `"a spell you cast this turn"`).
- Unify ordinal-per-period ranking (`"the second spell you cast each turn"` vs
  `"whenever you cast your second spell each turn"`) across `Predicate` and
  `GameEvent`.
- Retire `Condition.choseThisWay`, `Condition.dealtThisWay`,
  `Condition.flipCalled`, and `Condition.flipFace` in favor of `Condition.did` /
  `was` over `choose`, `dealDamage`, and `flipCoins`, preserving DRT referent
  publication and scope checks (`damageDealtInScope`, `coinFlipInScope`).
