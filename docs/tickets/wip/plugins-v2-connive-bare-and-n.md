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

## Landing record

Series on claim `twnmsonp`, oldest first: `lwyuqtss` (the two declarations,
the facts generator reading deeds, Unstable Experiment, the pins), then this
record with the residue routing. Stage (b) has no commit of its own: changing
`connive` to take no parameters makes `connive(1)` fail to load, so the one
canon site (Unstable Experiment, now `connive`) moved with stage (a).
`plugins_v2/testing` and every other canon card write no connive.

**Shape.** `conniveN` (`params: [Amount]`, `MeasureComplement`) carries the
body [CR#701.50d] and writes `deed: connive`. `connive` (`params: []`,
`Intransitive`) writes `deed: None` and body `conniveN(1)` [CR#701.50a]. Both
forms enact one `Deed.action "connive"` [CR#701.50f]: bare connive expands to
a single `Enact` (checked in its expansion and in Unstable Experiment's
generated Lean term). The facts table keeps one `connive` row:
`keywordActionLabels` is now built from the deed each declaration enacts
(`facts/lean.rs` `keyword_action_deeds`, reading the loaded `Enact` verb;
a declaration with no `Enact` wrapper keeps its own name), so it is byte
identical, with no `conniveN` row. `cargo xtask facts labels` reads the same
list (it had read the stub file names and failed on `conniveN`).

**Spelling, owed.** `conniveN`'s spelling is the bare head `"connive"`:
`builtin_v2_keyword_actions.rs` demands one literal spelling part. Writing
`"connive <Param(0)>"` is owed to `builtin-v2-keyword-spelling-templates`.

**Connive 0.** `conniveN(0)` still enacts the `connive` deed (the loader
wraps every amount), against [CR#701.50e]. Routed to
`semantics-v2-keyword-action-residues`, together with the `conniveN` name
read at `Deed` (an `Action("conniveN")` nothing enacts, refused only by
`knownAct`).

**PROVE** (on `lwyuqtss`, refreshed). `cargo xtask lean check`: canon 134/134,
testing 7/7. Lean build: 83 jobs, no errors. `cargo xtask lean definitions`:
553/553. `facts generate` changes nothing, and `facts check` passes. `cargo
xtask gate --changed --run` runs `cargo test -p deckmaste_construction_core
-p deckmaste_lexical_source -p deckmaste_semantics_v2 -p
deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`: 1411 passed, 0
failed, 1 ignored (pre-existing). Clippy `-D warnings` is clean on xtask,
construction_core and english_v3. Cite checks: 0 non-compliant, 0 stale, and
the audit diff was read rule by rule. `kanban check` is OK.

**DISCLOSE.** Expansions (`diff -r`): builtin adds `conniveN.expanded`;
`connive.expanded` changes only in its sample invocation and amount (3 to 1);
the index goes from 1507 to 1508 declarations. The canon and testing
expansions are identical. Lexicon export (`lexical --card-name "Black Lotus"
--export`): `lexeme:keyword_action/connive` goes from `Custom [[], [Amount]]`
to `Intransitive`; the new `lexeme:keyword_action/conniveN` (lemma `connive`)
carries the `MeasureComplement` frame (`MeasurePhrase` where it was
`Amount`). Nothing else changes, and the export accepts the shared head.
Assurance counts: 0 restored, 1 re-spelled (the connive frame pin, now an
`Intransitive` pin), 0 ignored, 0 removed. Added: the `conniveN`
`MeasureComplement` pin, `conniveN` in `EXPECTED_NAMES`, and the English
pins "Connive." and "This creature connives." No STOPs.
