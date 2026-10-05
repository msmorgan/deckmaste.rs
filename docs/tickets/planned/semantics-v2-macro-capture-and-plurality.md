---
needs: []
---
**What the RON macro language can say: capture parameters and
amount-derived plurality.** Design-bearing (sol or Opus); standard
constraints apply.

Lean's
`semantic_macro` bodies use two devices with no positional-RON spelling:
`capture` parameters that re-read an argument after a binding is introduced
(Fight, Regenerate) and plurality computed from an `Amount` argument
(`Amount.plur (.lit 1) = .one`: Scry, Surveil, Fateseal, Connive). Decide
whether the RON macro language grows the device (`macros-are-declarative.md`
forbids control flow; a typed capture is not control flow) or the Lean macro
is re-spelled without it. `lean-macros-from-ron` needs the same answer from
the other side, so record it in `semantics-v2.md` §12. `target-sugar-elaboration`
names Fight as its fixture for a different mechanism; related, not overlapping.


Then give Fight, Regenerate, Scry, Surveil, Fateseal, and Connive their
bodies under the chosen device, with a canon card each proving through
`lean-check`.

## Routed here by `plugins-v2-dialect` (2026-09-07)

The helper macro port (`semantics-v2.md` §12.1) left 23 `semantic_macro`s in
Lean because they COMPUTE rather than substitute — a `let`, a `match`, a
`.map` over an argument, an anonymous constructor, a plurality read off a
subject. They are the same question this ticket asks, one device at a time,
so they arrive here by name:

`agentRef`, `amass`, `chooseModes`, `chooseSpree`, `controllerSacrifices`,
`dealDamageOwnPower`, `itCondSubject`, `itPrior`, `itsOther`, `joinedHead`,
`joinedHeadWhile`, `lookAndSort`, `lookAndSortInto`, `lookedCards`,
`lookedTop`, `loseCounters`, `modular`, `ownSubject`, `partyOf`,
`requireBlockIt`, `rollRow`, `sacrificeIt`, `scaledMana`.

Each decides one way or the other — the RON macro language grows the device,
or the Lean macro is re-spelled without it — and a macro that ports gets its
declaration under `plugins_v2/builtin/macros/<family>/` with the rest.
Twenty-eight further macros call one of these and port when their callee
does; §12.1 lists them.

## Evidence from `plugins-v2-scry-surveil-fight-bodies` (2026-10-05)

That ticket tried to give the `scry`, `surveil` and `fight` keyword action
declarations RON bodies under the implicit actor, each expanding to the term
the Lean macro builds with the actor as its agent. None was writable, for the
two reasons this ticket names. The terms below are `#eval` output of the Lean
macros on the tree of that claim.

**Scry and surveil: the plurality comes from the amount.** With the actor as
agent, `agentRef .actor` is `.actor` (it introduces nothing), so
`scry n (agent := .actor)` is

```
enact (action "Scry")
  (sequentially
    [ expose lookAt (cards (librarySlice top n actor)) actor,
      move (someOf (counted (range none none)) none
             (pro bare PL (introduced [object])))
           wherever (library (oneEnd bottom) (some anyOrder) none bare) [],
      move (theRest object many) wherever
           (library (oneEnd top) (some anyOrder) none bare) [] ])
  (some actor)
```

and `surveil` the same with `zone graveyard bare` as the first move's
destination. Everything is fixed except `PL`, `lookedCards`'s
`slice.plur = outputPlur actor.plur n.plur = Amount.plur n`: `one` for
`scry 1` / `surveil 1`, `many` for `scry 2` / `surveil 2` (both evaluated).
The `introduced [object]` pattern is fixed: the slice's own binding is the
only one it introduces, since `NounPhrase.result` does not read the amount
and `actor` introduces nothing. A template must write one plurality, so it is
right for one class of amounts and wrong for the other. There is also no
macro spelling of the windowed pronoun itself: the window has its alias
(`nouns/introduced.ron`), but every `pronouns/` macro reads `Whole` and
`NounPhrase.pro` is `internal_expansion`, with no alias (`lean/CONTRACTS.md`,
"Grammatical references and internal windows"), and keyword bodies read under the macros-only
restriction with a shrink-only `ALLOWED_RAW`
(`crates/deckmaste_semantics_v2/tests/keyword_bodies.rs`). The smallest
change that makes both writable is either device: a RON-side plurality read
off an `Amount` argument (what `lookedCards` would need as a declaration), or
a Lean re-spelling of `lookAndSort`/`lookAndSortInto` whose looked-cards
pronoun does not carry the amount's plurality (for example a model node for
"the cards looked at this way" that the checker sizes itself). Either way,
`lookedCards` (or its replacement) becomes a helper declaration whose body
may write the window, and `scry`/`surveil` call it.

**Fight: the capture is computed per argument and per depth.** `fight l r`
is

```
withBindings D [subject l, subject r]
  (doIf (and [matches L (and [creature, inZone battlefield]),
              matches R (and [creature, inZone battlefield])])
    (enact (action "Fight")
      (simultaneously [dealDamage L (statOf power L) R,
                       dealDamage R (statOf power R) L])) none)
```

where `D` is the `MacroContext` depth of the call (0 at the root, 3 when
evaluated under a context of depth 3), and each read is
`MacroCapture.read D i x`: `x` itself when `x` is `actor` or already a
parameter read, otherwise `pro (parameter SHAPE) x.plur (parameter D i)`,
with `SHAPE` a `NounShape` computed from the argument (`kindOr`, `isAbility`,
`isYou`, `selfDefinedOk`, `ascribable`, `twoPartiesOk`, `opponentOnly`,
`bareThis`). Evaluated: `target creatureYouControl` gives
`selfDefined := false`, `thisCreature` gives `selfDefined := true`, `actor`
is read as itself. A template can write none of the three: the shape and the
plurality are functions of the argument, the scope number is a function of
the expansion's nesting, and `withBindings`/`Reach.parameter` are
`internal_expansion` with no alias. Writing each parameter four times instead
would select a target four times. The deed row `("Fight", {})` gives no
player performer, so when it is writable the declaration needs
`agent: None`; `Scry` and `Surveil` are `playerAgent`, so they take the
ordinary `Some(actor)` wrapper.

The smallest change for fight is the capture device itself: a typed capture
parameter in the RON signature (`params: [Capture(Subject), Capture(Subject)]`
or similar) that the semantics_v2 loader turns into `WithBindings` with a
loader-allocated scope and the `NounShape` read computed by the mirror, as
`MacroCapture.read` computes it in Lean.

**Related finding.** `plugins_v2/builtin/macros/instructions/regenerationApplication.ron`
writes `Param(subject)` six times where Lean's `regenerationApplication`
captures its subject; it has no callers, so nothing depends on the
difference yet. When Regenerate gets its body here, re-spell that helper over
the same device.
