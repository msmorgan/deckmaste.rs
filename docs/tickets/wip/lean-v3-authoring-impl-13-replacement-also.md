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

### 1. `Replacement.also` Constructor & `also` Template (`Abilities.lean`, `Templates.lean`, `Check/`)

- Add `| also (action : Action) (when : Option Condition := none)` to
  `Replacement` in `lean/Semantics/Abilities.lean`:
  - `Replacement.instead`: wholly replaces the `WouldClause` / `replaced` event
    with `action` (`[CR#614.1a]`).
  - `Replacement.also`: modifies or augments the `WouldClause` event without
    cancelling the underlying event (`[CR#614.1c,614.1d,614.12]`).
- Define `semantic_template also (action : Action) (when : Option Condition := none) : Replacement := .also action when`
  in `lean/Semantics/Templates.lean`.
- Update `CallerScope Replacement`, `Authoring.inspect`, `stepReplacementWith`
  (`Check/Phrase.lean`), `Check/AbilityRules.lean` (`staticTextContext` publishing
  text choices from `.replace _ (.also action)`), `Check/CardFacts.lean`, and
  `Check/NumberSlots.lean`.

### 2. Re-author Additive Entry & Event Modifications onto `also` (`Definitions/`, `Cards/`, `Proofs/`)

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
    `replace ... (also ...)`).

## Verification

- `./lean/scripts/build` (`lake build --wfail`) passes with zero warnings and
  zero `sorry`s.
- `cargo test -p xtask -p deckmaste_semantics_v2` and
  `cargo clippy -p xtask -p deckmaste_semantics_v2 --tests -- -D warnings` pass.
- `cargo xtask cite check --list-noncompliant` is empty and
  `cargo xtask cite check` reports 0 stale citations.
