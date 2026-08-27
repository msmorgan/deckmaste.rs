---
needs: []
---
# State the bench's selection principle where it is defined

`Cards.idr:1` calls itself "the workbench's evidence bench" and states
nothing further. No file in `idris/` states a selection methodology for which
cards get benched — not random, not curated-for-coverage, not
corpus-representative-sample. This is a real, cheap-to-close gap: it costs a
docstring, not a mechanism.

## Why this is a documentation ticket, not a coverage-metric ticket

Per `docs/memory/rulings/measurements-live-in-pins.md`: "the workbench proves
rules text is self-consistent... it is not a census of printed phrasings."
Building a total-corpus coverage ratchet for the Idris bench would be the
wrong answer to this gap — it would import a goal (total corpus coverage)
the workbench has deliberately not adopted. The actual gap is narrower and
already true in practice: bench entries are added when a construction,
keyword, or interaction needs a witness for a round's work, not to sample the
corpus for representativeness. A card's absence from the bench is not a claim
about that card — only that no round has needed it yet. That principle has
never been written down, and a reader lacking it could conclude, wrongly,
that its absence means no principle exists.

## The ask

State the principle above (or the conductor's more precise restatement of it)
in `Cards.idr`'s module docstring. If the conductor judges the principle
durable enough to warrant a standing ruling rather than a file-local note,
record it as a short addition to (or cross-reference from)
`docs/memory/rulings/measurements-live-in-pins.md` instead — decide that at
execution time rather than presuming it here.

## Consumption boundary

`idris/src/Experimental/Cards.idr` (module docstring). Optionally
`docs/memory/rulings/measurements-live-in-pins.md`, at the executor's
discretion. No Rust crate, no other Idris source.

## Acceptance

- `Cards.idr`'s header docstring states the bench's actual selection
  principle in the terms above.
- No coverage-percentage or corpus-sampling mechanism is introduced by this
  ticket; if a future ticket proposes one, it must argue against this
  ticket's recorded rationale rather than silently reintroducing it.

Standard constraints apply.

## As landed

- `idris/src/Experimental/Cards.idr`'s module docstring now states the
  selection principle in three parts: entries are added when a construction,
  keyword, or interaction needs a witness for a round's work; the bench is
  explicitly *not* random, curated-for-coverage, or corpus-representative; and
  a card's absence is not a claim about that card, only that no round has
  needed it yet. The docstring also says outright that the corpus fraction is
  not a quantity the workbench tracks, so a future reader has to argue against
  a stated position rather than fill a silence.
- The principle was judged durable enough to record on both sides, so the
  optional half was taken as well: a paragraph in
  `docs/memory/rulings/measurements-live-in-pins.md` derives bench growth from
  the same doctrine that governs pins, and its `INDEX.md` routing entry was
  extended to match.
- The docstring deliberately does **not** cite the memory note. `Cards.idr` is
  tracked source, and `docs/memory/README.md` forbids tracked files from
  referencing local memory (the dependency is one-way). The docstring
  therefore stands on its own wording; the memory note is the side that
  carries the cross-reference.
- No coverage-percentage or corpus-sampling mechanism was introduced.
- Gate: `idris2 --build mtg-dev.ipkg` clean (15/15 modules, `Experimental.Cards`
  at 12/15); full `idris/scripts/build` run recorded on the second ticket.
