---
needs: [semantics-v3-peer-files-and-toposort]
---
# A Template parameter mentioned in two arms of an `or` cannot be bound

Standard constraints apply. Split from `lean-v3-authoring-default-leaks` §3 (2026-10-25).

`objectsOfPlayer` (`lean/Semantics/Templates.lean:479-480`, "…that <player> controls or owns but
no player controls", its only caller *True-Name Nemesis*, `Cards/Choice.lean`) writes `player`
twice, once in each arm of an `.or`. ADR `docs/decisions/semantics-v3.md` §2.2 binds a Template
parameter mentioned more than once with `.firstMention`/`.laterMention`. Writing

```lean
semantic_template objectsOfPlayer (player : NounPhrase) : Predicate :=
  .or [ controlledBy (.firstMention "objectsOfPlayer.player" player),
        .and [ ownedBy (.laterMention "objectsOfPlayer.player" player),
               .not (controlledBy (a .isPlayer)) ] ]
```

makes *True-Name Nemesis* fail its card check with seven
`Refusal.bindingIdentity "objectsOfPlayer.player"`: the `.or` steps each arm from the common
header context, so the second arm's `.laterMention` cannot see the first arm's `.firstMention`.

Decide how one parameter identity is shared across disjuncts (for example, bind the parameter in
the common header before the arms, or let `laterMention` read a `firstMention` from a sibling
disjunct), implement it in the checker, then respell `objectsOfPlayer` as above and keep
*True-Name Nemesis* checking `[]`. Add a pin that the bound spelling checks and that a
`laterMention` with no `firstMention` in any arm is still refused.
