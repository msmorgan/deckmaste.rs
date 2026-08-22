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

## As landed (2026-08-22)

**Signature** (`Experimental.idr`, in the `Effect` mutual block, beside
`Reflexively`, which is unchanged):

```idris
ThisWay : (body : Effect bs) -> (ev : GameEvent (effIntro body)) ->
          (trig : Effect (thisWayCtx body ev)) ->
          {auto 0 oc : ThisWayOutcome body} -> Effect bs
```

The ticket sketched `trig : Trigger (effIntro body)`; there is no `Trigger`
type. The outcome form names its event where the agent form ("when you do")
leaves it implicit, so the node carries the event explicitly as a `GameEvent`
over `effIntro body` — the `Delayed`/`HeldUntil` topology — and the trigger's
effect reads `thisWayCtx body ev = settleTargets (eventAfter ev)`, the
`reflexCtx` idiom shifted onto the event.

**CR delta — the two witnesses are not the same rule.** Inferno is a genuine
[CR#603.12] reflexive: the enclosure causes the event as it resolves and the
trigger is checked at once. "When it regenerates this way" is not: [CR#701.19a]
makes the enclosure create a *shield*, and Debt of Loyalty's rulings confirm the
regeneration comes later in the turn when that shield applies, so the trigger
waits as a delayed one [CR#603.7]. `ThisWay` carries the grammatical template
both share — a trigger bound to the event its enclosure caused — and the
constructor docstring records the split rather than claiming one rule for both.

**Enclosure test** `thisWayOutcomeOk` / `ThisWayOutcome = So (…)` — a Bool
gate, not an enum, because after the CR delta above only ONE ground refuses:
`Delayed`, whose deed belongs to a separate delayed ability [CR#603.7e], not to
the enclosure, so "this way" names nothing. `May _ body _ _` recurses into the
body. Everything else is admitted, including `HeldUntil`, `SkipsNext`,
`ExtraTurn`, `AdditionalPart`, `DoesntUntapNext` and every compound: an effect
that applies later *without* a second ability still caused what happens.
Deliberately weaker than `reflexEncloseUse`, which is an agent test — an
agentless or compound enclosure is fine here, and Inferno needs that
(`Continuously` is `EncAgentless`).

**Two new `GameEvent` constructors**, because neither witness's event existed:
`StatBecomes n c v` ("its power becomes 20", [CR#603.2e] licenses "becomes"
events) and `Regenerates n` ([CR#701.19a], distinct from creating the shield
[CR#701.19c]). Both gate on `ZoneFits (nounZone n) (Just Battlefield)` like
`StatusEvent`. `eventAfter (StatBecomes n _ v) = amtDelta v ++ selfSubjIntro n`
(the `Attacks … whom` idiom) so the trigger can say "it"; `eventAfter
(Regenerates n) = selfSubjIntro n`.

**Readers extended** (found by building):
- `Effect` (nine, mirroring `Unless`): `heldUntilOk` (False),
  `reflexEncloseUse` (`EncNotOneAction`), `costActionOk` (False), `effEq`
  (False), `effIntro`/`preIntro`/`annIntro`/`deedDelta` (all `… body`),
  `thisWayOutcomeOk` itself. `replacedCtx` needs none — its catch-all is right.
- `GameEvent` (four): `eventName`, `eventIntro`, `eventAfter`,
  `eventSubjectPlur`.
- `EventName` (`Events.idr`): two new names `StatValueChange` and
  `Regeneration`, with `sameEventName` and `lookbackSubjectOk` clauses.
  `Regeneration` admits an Object lookback subject; `StatValueChange` admits
  none, on `StatusChange`'s ground — a bare "became" names no event, since the
  value it reached is a complement and a lookback carries none.

**Witnesses** (`Experimental/Cards.idr`, both full `Card`s):
`infernoOfTheStarMounts` — `ThisWay` over `Macros.gets thisCreature (PtUp 1)
(PtUp 0) untilEndOfTurn`, event `StatBecomes It Power (Lit 20)`, trigger
`DealDamage It (Lit 20) (target AnyTarget)`; the whole card benches, including
`Static (ObjectCant Countered This)`, flying and haste. `matopiGolem` —
`ThisWay (Regenerate thisCreature) (Regenerates thisCreature) (PutCounters 1
minusOneMinusOne It)`.

**Two things the witnesses forced.** (1) `Golem` was not in the subtype
vocabulary (`Words.idr`: enum, `Eq`, `subtypeType`). (2) Matopi's event is
written `Regenerates Macros.thisCreature`, not `Regenerates It`: `effIntro
(Regenerate n) = nomIntro n`, and a `This`-headed noun contributes no fresh
mention, so no "it" antecedent exists after "Regenerate this creature." That is
the house convention across `SetStatus`, `RemoveFromCombat` and the rest — the
grammar routes source self-reference through `This`. Inferno keeps `It`, since
its `Continuously` enclosure goes through `selfSubjIntro`. Not changed here;
whether `effIntro (Regenerate n)` should mint the subject is a separate call.

**Pin.** `badThisWayOnDelayed` (`ProofsC.idr`): "At the beginning of your next
end step, draw a card. When you draw a card this way, draw a card." Probe:
`Can't find an implementation for So False`.

**Collateral fix.** `docs/tickets/done/workbench-unless-node.md:69` carried two
bare rule numbers that failed `cite check --list-noncompliant`; rewritten as
`[CR#118.12a,107.5]`.

**Gates.** Clean 18/18 exit 0 (`build/` removed first); cards implicit grep 0;
`cite check --list-noncompliant` 0, 0 stale of 16005; no `bless` needed (every
rule cited was already locked); `cite audit --diff` read on all 16 sites.
