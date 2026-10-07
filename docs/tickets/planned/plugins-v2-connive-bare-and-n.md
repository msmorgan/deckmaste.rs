---
needs: [semantics-v2-deed-is-a-name]
---
**Bare "connive" and "connive N" are two keyword action declarations, each
spelled as a card prints it.** Decided 2026-10-07 by the owner. Lands after
`semantics-v2-deed-is-a-name`, which renames the keyword action labels and
the facts keys this ticket touches. Standard constraints apply. Small;
Sonnet-mechanical once the shape below is given.

## Decided 2026-10-07

- Today `connive` (`plugins_v2/builtin/macros/keyword_actions/connive.ron`)
  takes `params: [Amount]` and its header says bare "Connive" is written
  `connive(1)`. That is a defect: "connive(1) doesn't match what's written on
  a card".
- `conniveN(n)` carries today's body, the variant's own wording
  [CR#701.50d]: draw N, discard N, a +1/+1 counter per nonland card
  discarded. `params: [Amount]`, `spelling: "connive"` until
  `builtin-v2-keyword-spelling-templates` writes it `"connive <Param(0)>"`.
- `connive` is its own declaration, `params: []`, whose body is
  `conniveN(1)`: one card drawn, one discarded, a counter if it is nonland
  [CR#701.50a].
- No positional-default feature is added. Owner: "even if we allowed trailing
  positionals to be defaulted, two macros are probably necessary because
  connive would be spelled 'connive' with no number."
- `grammar` stays one word definition per declaration: both keep
  `Verb(bare: "connive", …)`; today's `Custom(frames: [[], [Amount]])` splits
  into `connive`'s `Intransitive` and `conniveN`'s `MeasureComplement`. The
  head `connive` is shared, so both declarations are candidates for the verb
  and the frame selects between them. If the lexicon export or the
  English environment refuses two declarations sharing one verb head, STOP
  and report rather than merging them back.

## The event

A permanent "connives" after either process completes [CR#701.50f], so
there is one deed, `connive`, for both forms. The loader wraps every keyword
action body in its own deed (`keyword_action_body`,
`crates/deckmaste_semantics_v2/src/keywords.rs`), so left alone the two
declarations would name two deeds and bare connive would enact twice. The
shape: `conniveN` declares `deed: connive` (a bare declaration name, after
the deed landing) and `connive` declares `deed: None`, so `conniveN`'s
enactment is the one event either way. The facts table keeps one `connive`
row (today `"Connive"`, `lean/Semantics/Check/Words.lean:306` with
`permanentAgent`, and `keywordActionLabels` in
`lean/Semantics/Check/Facts.lean`) and gains no `conniveN` row; if the facts
generator lists declarations rather than deeds, fix it to read the deed. Say
in the landing record how `Deed` and the facts table name it.

Connive 0 makes no connive event [CR#701.50e]. Record whether `conniveN(0)`
still enacts the deed; if it does and the body cannot say otherwise, route
it to a live ticket rather than widening this one.

## The work

- Split `connive.ron` into `connive.ron` and `conniveN.ron`, headers cited
  to their own rules.
- Re-spell every canon card that writes `connive(1)` to `connive` and every
  `connive(N)` with N other than 1 to `conniveN(N)` (grep
  `plugins_v2/canon` and `plugins_v2/testing` when the landing runs; on
  2026-10-07 the only site was Unstable Experiment). The Lean pins in
  `lean/Semantics/Proofs/BindingIdentity.lean` (`okConniveOne`, the connive 2
  pin) keep their outcomes.
- Update the keyword-action pins that count or name connive's frames
  (`crates/deckmaste_construction_core/tests/builtin_v2_keyword_actions.rs`,
  the English "Connive 2." pin in
  `crates/deckmaste_english_v3/tests/amount_complements.rs`), re-spelled,
  never deleted.

## Proof

`cargo xtask lean check`: canon and testing counts unchanged or better.
`cargo xtask expansions`: the diff is limited to the two declarations and
the re-spelled canon cards. The facts check passes with one `connive` row.
`cargo xtask gate --changed --run` passes; state the command.
