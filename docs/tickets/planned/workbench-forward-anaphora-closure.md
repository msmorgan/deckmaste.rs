---
needs: [workbench-conditional-and-coordination]
---
# Prove every anaphor resolves forward

The workbench exists to show that all of oracle text's anaphora can be authored
in a forward direction — a card is written in reading order and every
back-reference resolves from what was already bound — without old semantics'
lifting devices (the prenex `Targeted` channel and positional `Target n` reads,
the `With`/`WithChosenValue` prefix binders). Today that claim is prose in
`docs/decisions/semantics-v2.md` and `Bridge.idr`'s T-rules; no ticket
delivers it and nothing would fail if a constructor quietly broke it. This
ticket makes it falsifiable.

Law: **oracle text is strictly anaphoric.** Pronouns and demonstratives follow
their antecedents. No cataphora, no introduction channel above the clause, no
constructor that reads a mention introduced later in reading order. The
alternative — a full-endophora binder — was considered and rejected
(2026-08-21): miserable to build and true to nothing the corpus writes.

## The change

- Enumerate every anaphor constructor (`It`, `They`, `Them`, `Those`, `That`,
  `TheVerbed`, `ThoseVerbed` and any sibling in `Noun`) and state per
  constructor, as a proof or pin rather than a comment, that it resolves over
  `Bindings` by counted uniqueness to a mention introduced earlier in the
  term's reading order.
- Audit every binding-threading function (`nomIntro`, `amtIntro`, `effIntro`,
  `condIntro`/`condDelta`, `preIntro`, `annIntro`, the `Effects` telescope) for
  a site where the threaded context is not the reading-order prefix. Each hit
  is either fixed here or named with its owner.
- Standing exceptions at minting, owned by `workbench-conditional-and-
  coordination`: the `If`/`OnlyIf` split and `WhereLetter`/
  `WhereLetterStatic`'s postposed definition. After that ticket lands, the
  bench has no card authored out of reading order; confirm it.
- Promote the claim to an ADR in `docs/decisions/` — the law above, the
  retired lifting devices by name, and the forward-only binder as the contract
  every new constructor meets — linked from the decisions README.

## Consumption boundary

`idris/src/Experimental.idr`, `idris/src/Experimental/Words.idr`,
`idris/src/Experimental/Proofs*.idr`, `docs/decisions/`. No Rust crate is
touched.

## Acceptance

- Every anaphor constructor carries its forward-resolution proof or pin; no
  threading function hands a clause anything but its reading-order prefix, or
  the exception is owned by a named ticket.
- The ADR exists. `idris/scripts/build` PASS, no witness lost, no pin silently
  passing.

Standard constraints apply.
