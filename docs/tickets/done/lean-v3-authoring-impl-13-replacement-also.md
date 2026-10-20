---
needs: [lean-v3-authoring-impl-12-replacement-type]
---
# Add `Replacement.also` for additive entry and event modifications (`[CR#614.1c,614.1d,614.12]`)

**Add `Replacement.also (action : Action)` alongside `Replacement.instead` in
`lean/Semantics/Abilities.lean` so additive replacement effects (`[CR#614.1c,614.1d,614.12]`)
augment the replaced event without re-stating the underlying `enter` action or
faking an `"instead"` substitution.** Thirteenth implementation step of
[`lean-v3-authoring-surface`](lean-v3-authoring-surface.md). Standard
constraints apply.

## Scope

### 1. `Replacement.also`, `withRiders` & `onlyIf` (`Abilities.lean`, `Templates.lean`, `Check/`)

- `Replacement` in `lean/Semantics/Abilities.lean` becomes
  `inCaller body | instead action | also action | withRiders riders | onlyIf condition body`:
  - `Replacement.instead`: wholly replaces the `WouldClause` / `replaced` event
    with `action` (`[CR#614.1a]`).
  - `Replacement.also`: does `action` as part of the event, which still happens
    (`[CR#603.6d,614.1c]`).
  - `Replacement.withRiders (riders : List MoveRider)`: the replaced entering still happens,
    with these Entry Riders ("enters tapped / with … counters / as a copy / transformed",
    `[CR#614.1c,614.1d,614.12]`). These modify the entering event itself, so neither
    `instead` (which would restate `enter`) nor `also` (`also (tap it)` would be a separate
    "becomes tapped" event) is right. Named for the glossary's Entry Rider, which this arm alone
    carries (`with` is a Lean keyword); a Pile Placement is no rider, so it is refused here. The
    checker accepts it only where the replaced event is an entering, a move to the battlefield or
    a token's creation (refusal `riddenEntering`, e.g. riders over a draw or a dying), and only
    with Entry Riders (refusal `entryRidersOnly`).
  - `Replacement.onlyIf (condition) (body)`: one structural conditional wrapper in place of a
    per-arm `when` field; `instead`, `also` and `withRiders` take only their payload.
    `doIf c r` at expected type `Replacement` builds `onlyIf c r`, so cards still write
    `doIf c (instead a)`. `Replacement.parts?` and its fuel are retired; the checker steps
    `onlyIf` as condition, then body in a new clause, and a caller's `.inCaller` replacement
    stays one caller frame, so a Definition's condition over a caller replacement whose
    condition and action share an anaphor is accepted.
- Templates `instead (action)`, `also (action)`, `withRiders (riders)` in
  `lean/Semantics/Templates.lean`; the `enters subject riders` template builds
  `replace (would subject enter) (withRiders riders)`, so nothing restates `enter`.
- The `may` convention, per arm: under `instead`, only a wholly-`may` action with no failure arm
  leaves the event to happen when declined; under `also`, the event happens regardless, and
  declining a `may` does its failure arm if it has one (riot's haste) and otherwise nothing;
  under `withRiders` there is nothing to decline; under `onlyIf`, the event happens unmodified
  while the condition fails.
- A condition checked as the permanent would enter is the replacement's own (`doIf c
  (withRiders …)`: daybound, bloodthirst, impending, sunburst). A condition deciding whether the
  ability exists at all stays on the static ability (`onlyUnless`): the Saga's lore counter,
  "each Saga without read ahead" `[CR#714.3a]`. Turbulent Fen's "enters tapped unless …" also
  stays `onlyUnless`, because `onlyIf` has no "unless" marking for the checker's `markingOk`.
- Update `stepReplacementWith` (`Check/Phrase.lean`), `Check/AbilityRules.lean`
  (`staticTextContext` publishing text choices from `also`), `Check/CardFacts.lean`, and
  `Check/NumberSlots.lean` (`Replacement.numberSlots`: a `withRiders` replacement's riders).
  `CallerScope Replacement` and `Authoring.inspect` need no change.

### 2. Re-author Additive Entry & Event Modifications onto `also` (`Definitions/`, `Cards/`, `Proofs/`)

- Oracle phrasing sorts each entering modification: what the text states as how it enters
  ("enters tapped", "enters with N counters", "enters as a copy", "enters transformed") is a
  `withRiders` replacement; what it states as something done as it enters ("choose …",
  "you may sacrifice …", "reveal …", "put … counters on it as it enters", "you may exchange …")
  is `also`. Counters may be an `also` action where the Oracle says "put", or where a rider
  cannot carry what the text needs (a `may`, a failure arm, or a count read from an action done
  as it enters): counters given as it enters are put on it (`[CR#122.6]`). Gather Specimens
  stays `instead` (its Oracle text says "instead").
- Express additive `"As ~ enters, ..."` and entry-modification keywords/helpers
  through `replace (would ... enter) (also ...)` without repeating
  `instruct thisCreature enter` or wrapping in a redundant `mayOrElse ... (instruct thisCreature enter)`:
  - **`tribute` (`[CR#702.104a]`, `Keywords.lean`):**
    ```lean
    replace (would thisCreature enter)
      (also
        (sequentially
          [ choose anOpponent,
            instruct (the isChosenPlayer)
              (may (putCounters (.inCaller amount) p1p1Counter thisCreature)) ]))
    ```
  - **`unleash` (`[CR#702.98a]`):**
    ```lean
    replace (would thisPermanent enter)
      (also (may (putCounters (.lit 1) p1p1Counter thisPermanent)))
    ```
  - **`riot` (`[CR#702.136a]`):**
    ```lean
    replace (would thisPermanent enter)
      (also (mayOrElse (putCounters (.lit 1) p1p1Counter thisPermanent) (gainHaste thisPermanent)))
    ```
  - **`amplify` (`[CR#702.38a]`)** and **`devourQuality` (`[CR#702.82a]`)**:
    use `also (sequentially [...])` so the creature's entry is augmented by the
    reveal/sacrifice and counter placement without re-invoking `enter`.
  - **`entersChoosing` (`Templates.lean`)** and ***Deadpool, Trading Card*
    (`Cards/Keyword.lean`):** express `"As ~ enters, choose ..."` /
    `"As ~ enters, you may exchange ..."` via `replace (would ... enter) (also ...)`
    (and retire `StaticSpec.asEvent` if all remaining `"As [event], [action]"`
    sites in `Keywords.lean`, `Cards/`, and `Proofs/` are covered by
    `replace ... (also ...)`). `asEvent` stays: dash, blitz and warp act as a cost is paid,
    daybound and nightbound as it becomes night or day, and "as it becomes attached" is no
    entering; none is a replacement effect `[CR#614.1c]` covers. Enchant's Aura attachment acts
    as the Aura enters, but it is the game rule `[CR#303.4f]`, not "As this enters" text, so it
    too stays `asEvent`.

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
