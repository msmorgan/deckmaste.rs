---
needs: []
---
**Every keyword action body is read in its own reference scope, seeing only
its parameters and what it introduces itself.** Decided 2026-10-05: yes. The
owner accepted the recommendation "a keyword action body should be read in its
own scope, seeing only its parameters and what it introduces" ("your recs are
fine"). Found while prototyping the actor handoff
(`semantics-v2-actor-handoff`, 2026-10-05). Standard constraints apply.

## The defect

A keyword action body's own pronouns can collide with whatever the calling
card already mentioned. The `amass` declaration
(`plugins_v2/builtin/macros/keyword_actions/amass.ron`) reads its chosen
Army with a bare `that(Creature)` and `it`:

```ron
choose(actor, a(and([hasSubtype(army), creature, actorControls]))),
putCounters(Param(1), p1p1Counter, that(Creature)),
doIf(not(matches(it, hasSubtype(Param(0)))), establish(addSubtype(it, Param(0)))),
```

Azog, Moria's Ruin: "When Azog enters, destroy up to one other target
creature. Its controller amasses Goblins X, where X is that creature's power.
If you controlled that creature, draw a card." After the destroy, the
destroyed creature is a second candidate for the body's `it`.

**Pinned** (`lean/Semantics/Proofs/Actor.lean`):

- `okLiteralAmassBare`: the RON body's spelling, transcribed to Lean
  (`literalAmass`: bare `that (.type .creature)` and `itIsntA`), checks on
  its own.
- `okLiteralAmassHandedOff`: the same body under `act (target .opponent)`
  checks; the performer is a player, so no second object is in view.
- `badAmassBareItAfterDestroy`: "Destroy target creature. Its controller
  amasses Goblins 2" through the pre-existing explicit-agent Lean `amass`
  macro, whose reminder reads a bare "it", is refused with two
  `.anaphor .bare .one 2`. That macro has no handoff, so the defect
  predates the handoff; the handoff only makes the Azog shape writable.
- `okAzogControllerAmasses`: the same sentence through `act (controllerOf
  it) (Actor.amass "Goblin" 2)` checks, because `Actor.amass` reads the Army
  through `itPrior`, the choice's own introduction.

**Inferred, not pinned:** that the literal RON body is refused after
"destroy target creature" as the explicit-agent macro is. No pin puts
`literalAmass` after a destroy. Pin it (expected: the same anaphor
refusals) before relying on it.

## What exists

- The Lean macro layer avoids it with a computed window: `Actor.amass`
  (`lean/Semantics/Macros.lean`) reads the Army through `itPrior (choose (a
  army))`, the choice's own introduction (pin `okAzogControllerAmasses`). A
  RON template cannot compute a window
  (`docs/decisions/macros-are-declarative.md`), so the RON body cannot write
  this; `itPrior` is in the "It computes" bucket of §12.1 of
  `docs/decisions/semantics-v2.md`.
- Lean has scope machinery for macro bodies: the private expansion forms
  `withBindings` and `inCaller` and the `capture` annotation
  (`lean/CONTRACTS.md`, "`semantic_macro` accepts `capture NounPhrase`…").
  A captured subject is selected once and keeps its identity; parameter
  windows point at actual slots and add no ordinary pronoun candidate.

## The decision and what remains open

Decided 2026-10-05: a keyword action body is read in its own reference scope,
so that its pronouns see only its parameters and what the body itself
introduces, never the calling card's mentions. [CR#701.47a] defines amass as a self-contained
procedure ("Choose an Army creature you control. Put N +1/+1 counters on that
creature."), so "that creature" there cannot mean anything the card said
before. Still to settle in the work: where the scope is opened (the loader's
`Enact` wrapper, the `act` handoff, or a body-level form), and whether the
same holds for keyword ability bodies and helper macros.

Related: `semantics-v2-anaphor-resolution-heuristics` (how a pronoun with two
candidates resolves in card text). This ticket is narrower: a body's pronouns
should never have seen the card's candidates at all.

## What it blocks

Writing Azog, Moria's Ruin, and any card that hands a keyword action to a
player after mentioning an object of the same kind as one the action's body
reads back.
