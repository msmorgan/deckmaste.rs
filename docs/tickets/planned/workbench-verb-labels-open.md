---
needs: []
---
# Keyword-action verbs are labels over expanded bodies

**Ruling (user, 2026-08-25), reversing `workbench-effect-basis-realign`'s
verdict:** the workbench mirrors semantics_v2's structure, and v1's
keyword-action shape is the model — `Composite { name, body }` where the
MACRO expands to the keyword action's full body and the label only names
which keyword action is performed ("discard is just a named move from
hand to graveyard"). The meaning lives in the expansion; the label is
data; the vocabulary is open. Authority:
`docs/memory/rulings/workbench-mirrors-semantics-v2-structure.md`
(gitignored local ruling; quote it in the round's As-landed).

## The change

- `VerbName` stops being a closed 8-member enum with meaning-bearing
  total tables. It becomes an open label (the crate's shape: an atom
  gated by membership, not by a per-member type). Pick the Idris shape
  that keeps typo-safety at the gate without a core enum row per verb.
- The carrier is `Enact : (label) -> Effect bs -> Effect bs` (the
  de-enum'd `Composite`; `Does subj label body` stays for the agentive
  surface). **The label sits on the innermost action, not the whole
  expansion** — [CR#701.9] defines the discard AS the move; choosing is
  the instruction's business. Sketch:
  `discardN n = Repeated n (Sequentially [Choose a card from your hand,
  Enact "Discard" (Move that Hand->GY)])`. Consequences the round must
  keep: n labeled moves in one atomic batch = n occurrences
  [CR#603.2c]; the stamp lands on the move's object so the verbed
  anaphors key on exactly the acted-on mention; would-discard
  replacements and "at random" attach at the labeled move.
- `TagBody`'s per-verb typed expansions move to the MACRO layer: each
  keyword-action macro (`discards`, `sacrifices`, `mills`, `scries`, …)
  builds the full body and applies the label. A well-formedness guard on
  `Composite`/`Does` may survive if it can be stated label-generically;
  a per-label table in core may not.
- The four `VerbName`-keyed total tables (`verbAgentive`, `verbMoves`,
  `Eq VerbName`, `verbedMarkingOk`) are re-homed: whatever each row
  encoded either derives from the expanded body, moves to the macro, or
  is recorded data keyed by the label.
- `TheVerbed`/`ThoseVerbed` (the verbed anaphors) keep working, keyed on
  the label via the binding `Stamp` that `Enact`/`Does` applies to the
  bindings its body introduces.
- `Repeated n body`'s outgoing context is the body's delta PLURALIZED —
  each mention the body itself introduces becomes one `ManyOf` summary
  binding (same payload, same stamp; identity on deictics), plus a
  quantity outcome carrying `n` — so "the discarded cards"/"that many"
  read the batch, and the `ChooseQ` batch form yields the same summary.
- Every bench witness spelling is preserved (the macros' names and call
  shapes stay; their definitions change).
- The `Put` finding survives: `Put` is not a [CR#701] keyword action
  ([CR#701.1]); under labels this is unremarkable — a label needs no
  rules entry, its body speaks.
- Update the two docstrings the realign round wrote (`VerbName`,
  `Effect`) to the new verdict; the `Keyword` sibling statement stands
  (it was already the declaration candidate).

## Consumption boundary

`idris/src/Experimental/Words.idr`, `Experimental.idr`, `Macros.idr`,
`Proofs*.idr`, `Cards.idr`. No Rust crate.

## Acceptance

- No closed verb enum remains; a new keyword action is demonstrably a
  new macro + label (show one added in a test/witness without touching
  any total table).
- The ruling is recorded where `VerbName`'s replacement is defined.
- No witness lost, no pin silently passing; `idris/scripts/build` PASS.

Standard constraints apply.

## Expansion-shape ruling (2026-08-25, CR-verified, amended)

- The expansion's shape is not rules content: a resolving instruction is
  atomic (nothing interleaves mid-`Sequentially`), so the engine sees
  `Repeated n [choose, move]` and a batch `ChooseQ (Exactly n) … + move`
  as the same batch. **Canonical form, pinned (user, 2026-08-25): the
  iterated singular** — `Repeated n (Sequentially [Choose one, Move
  that])` — which handles a shortfall structurally (each iteration's
  choose finds what it finds). `ChooseQ`-style batch forms are for cards
  whose printed English carries the cardinality itself ("choose two
  colors"). No per-macro discretion.
- Per-event granularity is the ENGINE's rule, keyed by the label: draws
  are individual events [CR#121.2]; a multi-card discard is one event
  with n occurrences [CR#603.2c] ("discard a card" fires n times; "one
  or more" once). The label is what licenses the engine to apply the
  right granularity — another job the label does; the term shape does
  not encode it.
- Shortfall ("discard 3 with 2 in hand") is the engine's [CR#609.3]
  do-as-much-as-possible (discard is that rule's own example); either
  expansion is honest as written.
- No printed card writes "one at a time" for discard (corpus, 2026-08-25).
