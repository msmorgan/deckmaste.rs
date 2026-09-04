---
needs: []
---
**Refuse moving an ability now that abilities are objects.** Residue of
`workbench-ability-kind-fold` (2026-09-04): with `Kind.Ability` folded into
`Object` + `AbilityP`, `Move (Macros.target (AbilityHead AnyActivated)) exileZ
[]` typechecks. An ability on the stack is an object that cannot change zones
[CR#113.1c] (it ceases to exist when it leaves the stack, it is never moved).
The payload gate on `Move`'s `what` was written and reverted: `what` is
abstract inside the sixteen `Macros` move wrappers, so the obligation must
thread through them to their callers, and the first attempt produced
`Multiple solutions found` in `scry`/`fateseal`/`surveil`.

Fix: a `MovablePayload` gate on `Move` (and `SetStatus`? decide from the CR)
threaded through the move macros as one erased obligation each; a pin
refusing the sentence above, probed non-vacuous; the look macros keep their
single solution.

Size: M. Done when: the pin refutes; every move macro keeps its surface; the
bench typechecks; build at its module count. Standard constraints apply.

## As landed

- **`MovablePayload` gate — landed as a payload field on the existing `Movable`
  witness, so no new threaded obligation name.** `Phrase.NotAnAbility : Bool ->
  Type` (sole constructor `PayloadIsObject : NotAnAbility False`) is now an
  erased auto field of `Movable`'s `ObjectMoves` case:
  `ObjectMoves : {auto 0 nb : NotAnAbility (nounIsAbility n)} -> Movable {k = Object} n`.
  `Effect.Move`'s `{auto 0 mk : Movable what}` is unchanged, so the gate reaches
  every `Move` through the witness the pile-kind round already installed.
- **`SetStatus` needs no gate.** Only permanents have status [CR#110.5d]; an
  ability is on the stack [CR#405.1], and `SetStatus`'s existing
  `{auto 0 ok : ZoneIs (nounZone n) Battlefield}` already collapses to
  `So False` for an ability subject. Probed:
  `SetStatus Tapped (Macros.target (AbilityHead AnyActivated))` fails with
  `Can't find an implementation for So False.` before and after this round.
  `StatusHolder` is untouched.
- **Threaded through the move macros, one erased `{auto 0 mk : Movable n}`
  each**, at 16 wrappers: `destroy`, `exileWithCounters`,
  `returnToBattlefieldWithCounters`, `putOntoBattlefield`,
  `putOntoBattlefieldTapped`, `putOntoBattlefieldTappedAttacking`,
  `putOntoBattlefieldUnderYourControl`, `sacrifice`, `discard`, `meldInto`,
  `returnTo`, `returnToBattlefieldTransformed`, `returnToBattlefield`,
  `shuffleInto`, `puts`, `exileUntil`. `move` and `exile` already carried it
  (18 wrappers carry it now). `sacrificeIt` and `mills` move a concrete noun
  (`Pro (AtSlot PermanentSlot) OneOf`, `LibrarySlice OnTop …`) and needed
  nothing. Every macro keeps its explicit surface; no bench call site changed.
- **The look macros keep their single solution.** `scryBody`, `surveilBody`,
  `fateseal`, `scry` and `surveil` are untouched: their `move` calls carry
  concrete nouns, so `Movable` resolves at the call site by search. Probed by
  building — see the STOP below for the shape that did not work.
- **Pin.** `ProofsZone.badAbilityMovedToAZone` refuses
  `Move (Macros.target (AbilityHead AnyActivated)) Macros.exileZ []` at
  `{mk = ObjectMoves {nb = ok}}`, body `PayloadIsObject impossible`; positive
  twin `ProofsZone.okObjectMovedToAZone` ("Exile target creature.") beside it.
  No vintage-legal witness: no printed card moves an ability.
- Nothing left undone.

## Landing record

Numbers before/after:

- Idris modules built: 46 → 46 (clean `rm -rf build` rebuild).
- `Phrase.Movable` constructors: 2 → 2 (`ObjectMoves` gains one erased field);
  new `Phrase.NotAnAbility`, 1 constructor.
- Move wrappers in `Macros` carrying the `Movable` obligation: 2 → 18.
- `ProofsZone` `Unspellable` pins: 81 → 82.
- Bench (`Cards.idr`, `Cards/*.idr`) call sites changed: 0.

Gate lines (all foreground, from the workspace):

- `cd idris && rm -rf build && ./scripts/build` — exit 0; last line
  `46/46: Building Cards (src/Cards.idr)`; 0 lines matching `error|warning`.
- `cargo xtask cite check --list-noncompliant` —
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` — `checked 14230 citations against cr.txt
  (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git > /tmp/round.diff && cargo xtask cite audit --diff
  < /tmp/round.diff` — `audited 4 citation site(s)`. [CR#113.1c] ("An ability
  can be an activated or triggered ability on the stack. This kind of ability
  is an object.") and [CR#608.2n] ("As the final part of an ability's
  resolution, the ability is removed from the stack and ceases to exist.") both
  support, rather than undercut, the claim that an ability leaves the stack by
  ceasing to exist and so never changes zones. No `cite bless` was needed:
  both rules were already registered in `cr-citations.lock`, which is
  unchanged by this round.

Performance advisory: clean whole-model build after, two runs, `1m29.545s` and
`2m14.229s` real; the pre-round build measured `1m49.765s` real. Machine
variance swamps the difference — read this as no measurable change.

Assurance counts — restored 0, re-spelled 0, ignored 0, added 2, removed 0.

- Added: `badAbilityMovedToAZone` (the pin) and `okObjectMovedToAZone` (its
  positive twin), both in `ProofsZone`.
- Non-vacuity probe: the pin was mis-stated once, with
  `Macros.target Macros.creature` in place of the ability noun, and the message
  changed from clean to `probeMisstated PayloadIsObject is not a valid
  impossible case.`
- No pin or witness was deleted or re-spelled; `ProofsPiles.badPlayerMovedToAZone`
  (which pattern-matches `ObjectMoves`) and `badPileFaceAsAStatus` still refute
  unchanged.

### Deviations and additions

- **`Phrase.NotAnAbility`, a Bool-indexed named witness, rather than a bare
  `So (not (nounIsAbility n))`.** The `So` form is what the reverted first
  attempt used, and it reproduces exactly: with `{auto 0 nb : So (…)}` on
  `ObjectMoves`, the goal reduces to `So True` inside `scryBody`, `fateseal`
  and `surveilBody`, where the erased `Placeable` arguments `ps` and `pr` are
  also `So True`, and the search reports `Multiple solutions found in search
  of: So True … Possible correct results: ps, pr`. A named `data` witness has
  no other inhabitant in scope, so the search is deterministic and the look
  macros need no call-site annotation — this is the idiom `VERIFY.md`
  prescribes ("prefer a named `data` witness … over a bare `So (…)`").
- **`exileUntil`'s `{auto 0 mk}` is declared before its explicit `ev`
  argument**, because `ev`'s type spells out the `Move` the obligation gates.
  The explicit surface (`exileUntil n ev`) is unchanged.

### STOP

None taken. The one shape abandoned mid-round (an explicit
`{mk = ObjectMoves}` at the five `move` sites inside `scryBody`/`fateseal`/
`surveilBody`) was a workaround for the `So` ambiguity above; it became
unnecessary once the witness was named, and no call site in the look macros
was left changed.
