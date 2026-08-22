---
needs: []
---
# An outcome-bound reflexive: "when [it happens] this way"

Ruling 2026-08-22. [CR#603.12] licenses two reflexive forms: "when you do"
(the body's agent; today's `Reflexively`) and "when [something happens] this
way" (the body's outcome). Inferno of the Star Mounts ("When its power becomes
20 this way"), Matopi Golem, Skeleton Scavengers, Soldevi Sentry ("when it
regenerates this way") are unrepresentable. Mint a second constructor beside
`Reflexively`, e.g.

```idris
ThisWay : (body : Effect bs) -> (trig : Trigger (effIntro body)) -> Effect bs
```

whose trigger reads the body's outcome mentions, with its own enclosure test
(does the body produce an outcome?) separate from `reflexEncloseUse`'s agent
test. Two constructors because the two forms bind differently (agent vs
outcome) — the same argument that made `If`/`OnlyIf` both core; an optional
agent on `Reflexively` would reintroduce the two-`Maybe`s-plus-a-pin shape.
Bench Inferno of the Star Mounts and one regenerate line.

## Consumption boundary
`idris/src/Experimental.idr`, `Words.idr`, `Macros.idr`, `Cards.idr`,
`Proofs*.idr`.

## Acceptance
`ThisWay` exists; both witnesses bench; `Reflexively` unchanged; build PASS;
no implicits in cards; cites 0/0. Standard constraints apply.
