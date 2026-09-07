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
