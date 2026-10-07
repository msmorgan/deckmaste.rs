---
needs: [semantics-v2-macro-capture-and-plurality]
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

## The decision

Decided 2026-10-05: a keyword action body is read in its own reference scope,
so that its pronouns see only its parameters and what the body itself
introduces, never the calling card's mentions. [CR#701.47a] defines amass as a self-contained
procedure ("Choose an Army creature you control. Put N +1/+1 counters on that
creature."), so "that creature" there cannot mean anything the card said
before.

Decided 2026-10-06 (`docs/decisions/semantics-v2.md` §7), settling what was
left open:

- The scope opens at the loader's `Enact` wrapper, the keyword action
  boundary, never at an `act` handoff: `act(they, discard(that(Card)))` must
  see the choice made before it. A body-level form is not added.
- Every keyword ability body is read in its own scope too.
- Helper macros are transparent: a helper reads the scope of the body that
  calls it. Helpers that only bake a pronoun in are retired to the general
  form in the capture landing (its "Decided 2026-10-06" section lists them).

## Built inside the capture-and-plurality landing

Decided 2026-10-05: this scope is built inside the
`semantics-v2-macro-capture-and-plurality` landing, not separately. Owner:
"ok". Its design substance stays recorded here; that landing builds it, and
this ticket closes when that landing's pins (above, with the inferred
`literalAmass`-after-destroy pin added) hold.

Related: `semantics-v2-anaphor-resolution-heuristics` (how a pronoun with two
candidates resolves in card text). This ticket is narrower: a body's pronouns
should never have seen the card's candidates at all.

## What it blocks

Writing Azog, Moria's Ruin, and any card that hands a keyword action to a
player after mentioning an object of the same kind as one the action's body
reads back.

## Landing record

Built inside `semantics-v2-macro-capture-and-plurality` (its landing record has
the series, gate and counts); change `qotwpwwluqnq`. A keyword action body is
read in its own scope, opened at the loader's `Enact` wrapper: Lean
`Instruction.ownScope` masks the calling text's mentions except the performer
the innermost handoff names, and each parameter is read in the caller's view
(`inCaller`). A keyword ability body was already checked from an empty context.
Helper macros get no scope. `docs/decisions/semantics-v2.md` §12.2 and
`lean/CONTRACTS.md` ("Binding identity and a keyword body's own scope") record
it.

Pins (`lean/Semantics/Proofs/Actor.lean`): `okLiteralAmassBare`,
`okLiteralAmassHandedOff`, `badAmassBareItAfterDestroy` and
`okAzogControllerAmasses` keep their outcomes. Added: the inferred
`badLiteralAmassAfterDestroy` (the literal body with no scope, after "destroy
target creature": the same two `.anaphor .bare .one 2`), and its scoped twin
`okScopedAmassAfterDestroy` (checks); `okScopedAmass`,
`okScopedAmassHandedOff`; `okAzogScopedAmassX` (the amount, a parameter, reads
the destroyed creature in the caller's view); `scopedBodyCannotReadTheCaller`;
`keywordAbilityBodyCannotReadItsCard`. The testing probe "Amass After Destroy
Probe" ("Destroy target creature. Its controller amasses Goblins 2.") proves
through `lean check`. Azog itself is not written: "that creature's power" of a
destroyed creature needs last-known information the bench's `that` does not
read. No STOP.
