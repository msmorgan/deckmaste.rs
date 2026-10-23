---
needs: []
---
# `choose` refuses every coordinated noun phrase

Standard constraints apply.

## Defect (verified 2026-10-21)

`NounPhrase.agentChoosable` (`Check/Phrase.lean:349-354`) accepts a phrase only if its
determiner is `a`, `target` or `count`, or it is a `someOf`/`pileOf`. A coordination
(`.and`, `.coord .andOr`) has no determiner, so `choose` over any of them is refused with
`choiceClause` (`Check/Phrase.lean:3104`):

```lean
choose (both (target (and [isCreature, youControl])) (target (and [isCreature, non youControl])))  -- choiceClause
choose (andOr [a (and [isCreature, isCard]), a (and [isLand, isCard])])                           -- choiceClause
choose (target (and [isCreature, youControl]))                                                    -- clean
```

## Cards affected

None is authored yet; each is blocked by this refusal.

- "Choose target creature you control and target creature you don't control" (the fight and
  bite family): *Blizzard Brawl*, *Malamet Battle Glyph*, *Duel for Dominance*, *Tail Swipe*,
  *Joust*, *Grim Contest* (…an opponent controls), *Beastie Beatdown*.
- Other "choose target X and target Y": *Stolen Uniform*, *Cruel Entertainment* ("Choose
  target player and another target player").
- "Choose a creature card and/or a land card from among them": *Green Sun's Twilight*.

The targets are chosen as the spell is cast ([CR#601.2c]); "choose" then names which of the
already-chosen objects play which role, and an effect that offers further choices announces them
while applying it ([CR#608.2d]). Both halves of a coordination are chosen.

## Proposed fix

- `agentChoosable` on a coordination holds iff every member is agent-choosable (recursively
  through `inCaller`/`firstMention` as today). A `.or` of choosable members likewise.
- `choose` over a coordination introduces one mention per member, in textual order, with each
  member's own determiner and restriction, so "the creature you control" and "the chosen
  creatures" / "those creatures" read back the right referents. Check the plural "those
  creatures" and "each of those creatures" as well as the per-role singular.
- No new Core AST node: the coordination is the existing `.and` / `.coord .andOr`.

## Tests to add

- *Blizzard Brawl*, *Joust*, *Grim Contest* and *Stolen Uniform* bench items, each with the
  readback its Oracle text uses ("the creature you control gets …", "those creatures fight each
  other", "that Equipment").
- *Green Sun's Twilight*'s choose with `andOr`, and the "the chosen cards" readback.
- Refusal pins: a coordination with a member that is not choosable (`choose (both (the creature)
  (a creature))`) still refuses `choiceClause`; `choose` over a bare `you` or an unchosen
  superlative (`Check/AbilityRules.lean:136-138`) still refuses.

Gates: `./lean/scripts/build`, `cargo xtask lean check`.
