---
needs: []
---
**Add a `Pro` read at kind `TurnRef` so turn possessives are nouns.**
Cleanroom review 2026-09-03, R4, ruled: turn possessives become nouns. This
is the small half of the possessor fold and lands first —
`workbench-possessor-noun` needs it.

`Owner.ThatTurns` (`Words.idr:3829–3832`) is a fused word gated by
`TurnDeixis` (`Triggers.idr:588`) over a `TurnRef` binding that `ExtraTurn`
already mints (`Effect.idr:1897`). What is missing is only the *read*: `Noun
bs TurnRef` has no constructor, so "that turn's" cannot be spelled
compositionally and must stay a word.

## The ruling

The binding exists; add the read. A `Pro (Reach) (Plurality)` read at kind
`TurnRef` — the same counted-uniqueness idiom as every other pronoun read, no
recency and no ranking — makes "that turn's upkeep" a `Noun bs TurnRef` and
lets `TurnDeixis` retire. Turn possessives are nouns, not a word class.

Scope here is the read plus its gate, and re-spelling whatever bench sites
spell `ThatTurns` today. Deleting `Owner` itself, and re-typing
`HeaderPossessor` / `DurationEnd` / `TriggerWindow` / `Timing` /
`OnlyDuring`, is `workbench-possessor-noun`.

Size: S.

Done when: the build is 23/23 with 0 errors and 0 warnings; `Noun bs TurnRef`
has a `Pro` read with the `= 1` count gate; "that turn's" is spellable as that
read over the `ExtraTurn` binding and is a typechecking bench witness;
`TurnDeixis` is gone or reduced to the generic read's gate; a pin refutes the
read where no turn binding is in scope, and it is non-vacuous. Standard
constraints apply.

## As landed

- Added one `Reach` case, `ThatTurn` (`Words.idr` `data Reach`), rather than
  widening `Bare`: `Bare` is `kindLte Object b.kind` with `reachKind Bare =
  Object`, so it cannot yield a `Noun bs TurnRef`. A new `NounWord` was the
  alternative and was rejected — it would cost ~12 `wordReaches` clauses plus
  the `Eq`/index tables, and would open "that turn" as a `Described` noun.
- `reachKind ThatTurn = TurnRef` and `reaches ThatTurn pl b = kindLte TurnRef
  b.kind && isOne pl == isOne b.plur` — the same shape as `reaches Bare`, so
  the `= 1` count gate is `countReach ThatTurn OneOf bs = 1` with no recency
  and no ranking. `countReach`/`provOfReach`/`zoneOfReach`/`tyOfReach`/
  `faceOfReach`/`setZoneReach` needed no change (all generic over `Reach`),
  and every `Pro`-dispatching clause in `Phrase.idr` already had a
  `(Pro _ _)` fall-through.
- One macro, `Macros.thatTurn = Pro ThatTurn OneOf : Noun bs TurnRef`.
- Bench witness `Experimental.Cards.finalFortuneThatTurn : Noun (effIntro
  {bs = []} (ExtraTurn You (Lit 1))) TurnRef` — "that turn" of Final Fortune's
  second clause (Vintage-legal, printed "Take an extra turn after this one. At
  the beginning of that turn's end step, you lose the game."), read at exactly
  the frame that clause sees.
- Pins in `ProofsAnaphora`: `badThatTurnWithoutTurn` (no `TurnRef` binding in
  scope) and `badThatTurnAfterTwoTurns` (two, over the helper frame
  `twoExtraTurns`). Both probed non-vacuous.
- Undone, deliberately: `Owner.ThatTurns`, `TurnDeixis`, `isTurnDeictic` and
  `possessorB` are untouched, and nothing yet consumes a `Noun bs TurnRef` —
  re-typing `HeaderPossessor` and retiring the fused word is
  `workbench-possessor-noun` (F2), per this round's brief. The ticket's "`TurnDeixis`
  is gone or reduced to the generic read's gate" therefore lands with F2, not here.

## Landing record

- Construction count: `Reach` 7 → 8 cases; `Noun` constructors 34 → 34;
  `Macros` +1 (`thatTurn`); `ProofsAnaphora` `Unspellable` pins 5 → 7.
- Coverage and lock state: no bench card term changed; `cr-citations.lock`
  unchanged (the cited rule was already registered).
- Gates (all foreground): `./scripts/build` clean rebuild, last line
  `23/23: Building Cards (src/Cards.idr)`, 0 `Error`/`Warning` lines;
  `cargo xtask cite check --list-noncompliant` → `0 non-compliant
  citation-looking string(s)`; `cargo xtask cite check` → `checked 17723
  citations against cr.txt (eff. 2026-08-07); 0 stale`; `cargo xtask cite
  audit --diff` → `audited 0 citation site(s) — nothing selected`.
- Pin probes: in a scratch module, the positive twin `Noun (effIntro {bs = []}
  (ExtraTurn You (Lit 1))) TurnRef = Macros.thatTurn` typechecks, and
  `countReach ThatTurn OneOf` reduces to `0`, `1`, `2` by `Refl` over the
  zero-, one- and two-turn frames. Re-stating each pin over the one-turn frame
  turns `Refl impossible` into `Error: … Refl is not a valid impossible case`.
  Scratch module deleted.
- Assurance: restored 0; re-spelled 0; ignored 0; added 3 (1 bench witness, 2
  pins); removed 0.
- Deviations and additions: `twoExtraTurns : Bindings` in `ProofsAnaphora` as
  the two-turn pin's frame (a helper, not a claim). No CR citation was added to
  either pin: they are anaphora count gates, not term refusals — the whole
  existing anaphora pin family in `ProofsAnaphora` carries none — and the
  nearest rule, [CR#500.7], supplies a recency order ("the most recently
  created turn will be taken first") that the counted-uniqueness ruling
  deliberately refuses, so citing it would point backwards.
- STOPs: none.
