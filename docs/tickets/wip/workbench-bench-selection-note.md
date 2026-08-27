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
