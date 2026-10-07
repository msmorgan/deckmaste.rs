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
(`Amount.plur (.lit 1) = .one`: Scry, Surveil, Fateseal, Connive). Decided
2026-10-06 (below): a referent parameter is bound at its first mention, and
plurality is read off the `Amount` argument. `lean-macros-from-ron` needs the
same answer from the other side; it is recorded in `semantics-v2.md` §7
(ruling 2026-10-06) and §12.1 points there. `target-sugar-elaboration` names
Fight as its fixture for a different mechanism; related, not overlapping.


Then give Fight, Regenerate, Scry, Surveil, Fateseal, and Connive their
bodies under the chosen device, with a canon card each proving through
`lean-check`.

Work items routed from `plugins-v2-scry-surveil-fight-bodies` (which stays in
`done/`): give the `scry`, `surveil` and `fight` keyword action declarations
their bodies here.

## Also built here: keyword-body reference scope

Decided 2026-10-05 (owner: "ok"): the reference scope of
`semantics-v2-keyword-body-reference-scope` — every keyword action body read
in its own scope, seeing only its parameters and what it introduces — is
built inside this landing, not separately. That ticket holds its design and
pins and names this one as its need; this landing does its work.

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

The smallest change proposed for fight was the capture device itself (rejected
2026-10-06, below): a typed capture
parameter in the RON signature (`params: [Capture(Subject), Capture(Subject)]`
or similar) that the semantics_v2 loader turns into `WithBindings` with a
loader-allocated scope and the `NounShape` read computed by the mirror, as
`MacroCapture.read` computes it in Lean.

**Related finding.** `plugins_v2/builtin/macros/instructions/regenerationApplication.ron`
writes `Param(subject)` six times where Lean's `regenerationApplication`
captures its subject; it has no callers, so nothing depends on the
difference yet. When Regenerate gets its body here, re-spell that helper
under the binding rule below.

## Decided 2026-10-06

Recorded as rulings in `docs/decisions/semantics-v2.md` §7 (2026-10-06);
§12.1's "It computes" bucket points there.

**First mention binds, later mentions refer.** A referent parameter
(`Subject`, `NounPhrase`) of a registry macro is introduced by the body's
first mention of it; every later mention is a back-reference to that binding,
resolved by binding identity (glossary: **Binding Identity**), not by pronoun
search. Substitution by copy stays for a parameter mentioned once. The
loader generates the back-reference, which needs a binding identity to
address; `docs/tickets/maybe/lean-checker-binding-ids.md` holds the
addressing observation. Owner: "really painted myself into a corner here
didn't I. I guess B'."

- Rejected: binding at the call (the owner's first intent, "B"). It hoists
  the argument out of its clause: `act(each(opponent), discard(a(card)))`
  would choose one card for every opponent, and a choice under `may` or
  `doIf` would be made whether or not the branch runs.
- Rejected: an opt-in `Capture(Subject)` annotation (the device sketched in
  the evidence above).
- The v1 `Target(N)` device is not re-adopted (owner).

Counted 2026-10-06: 228 macros have a referent parameter (254 parameters);
223 of them expand byte-identically under the rule. Three mention one twice
or more: `instructions/regenerationApplication.ron` (`subject`, 6 mentions,
no callers), `keyword_actions/detain.ron` (parameter 0, 2) and
`keyword_actions/harness.ron` (parameter 0, 2). The fight and regenerate
bodies written here join them. `Instruction` parameters (46) and
`Predicate` parameters (35) are never mentioned twice. `Amount` parameters
are mentioned twice in `dredge`, `fabricate` and `endure`; substitution is
right for an amount.

**Plurality is read off the `Amount` argument**: scry 1 is "that card",
scry 2 "those cards". The Lean re-spelling of `lookAndSort` with a
numberless pronoun is rejected.

**Scope** (`semantics-v2-keyword-body-reference-scope`, its open items
settled): the scope opens at the loader's `Enact` wrapper, never at an `act`
handoff; keyword ability bodies get their own scope too; helper macros are
transparent and read the scope of the body that calls them.

**Retire the convenience helpers to their general form, in this landing.**
A helper that bakes a pronoun in where `verb(it)` says the same is retired; a
helper that carries its own `spelling` stays (none of those below does,
checked 2026-10-06). Owner: "those 'shorthands' were not doing anything for
the most part"; on `sacrificeIt`, it is "`add1 = (+) 1`".

| Helper | General form |
|---|---|
| `determiners/itsA.ron`, `itsACard.ron`, `itIsntA.ron`, `abilities/itIsntAnAbility.ron` | `matches(it, p)` and its negation |
| `events/thatTurns.ron` | `byTurn(thatTurn)` |
| `instructions/revealIt.ron` | `revealCards(foundCard)` |
| `zones/theirHand.ron` | `handOf(they)` |
| `instructions/searchTheirLibraryFor.ron` | `search(libraryOf(they), exactly(1), p)` |
| `instructions/foundCard.ron` | `theVerbed(search, One)` once `semantics-v2-deed-is-a-name` lands; `itVerbed(…)` until then |
| Lean `sacrificeIt` (`Macros.lean`, 3 callers) | `sacrifice(it)` with the slot selector |
| Lean `itOrThem` (`Macros.lean`, no callers) | deleted |

`predicates/comparesOwnStat.ron` bakes `Pro(Bare, One, Top(1))` inside a
larger body: re-spell it with the pronoun as an argument. The primitive
aliases `thatMuch`, `theDifference`, `itsManaCost`, `theGrantor`,
`theOutcome` and `thatAbility` stay: the name is the constructor.

The nine Lean macros that build a pronoun window (`itPrior`,
`itCondSubject`, `attachToIt`, `requireBlockIt`, `itsOther`, `ownSubject`,
`controllerSacrifices`, `lookedCards`, `agentRef`) are not conveniences and
are not touched here; they are inventory for
`semantics-v2-anaphor-resolution-heuristics`.

**Bodies to write:** fight, regenerate (`regenerationApplication.ron`
re-spelled), scry, surveil, fateseal and connive. Fight's rule text, "Each
of those creatures deals damage equal to its power to the other creature"
[CR#701.14a], is a reciprocal the model lacks; write fight with the binding
rule, and the reciprocal gap is routed to
`semantics-v2-anaphor-resolution-heuristics`.

**Burglar Rat.** Owner, 2026-10-06: "burglar rat should probably be
cardIn(hand)", that is, its discard argument reads `cardIn(hand)` rather than
`a(card)`. Settle it when that body is re-spelled.
